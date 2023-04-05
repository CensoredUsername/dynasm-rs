//! This module provides several reusable compoments for implementing assemblers

use std::mem;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

#[cfg(feature = "std")]
use {
    std::collections::hash_map::Entry,
    std::collections::BTreeMap,
    std::io,
    std::sync::{Arc, RwLock, RwLockWriteGuard},
};

#[cfg(not(feature = "std"))]
use {
    alloc::collections::BTreeMap,
    alloc::sync::Arc,
    lock_api::{RwLock, RwLockWriteGuard},
};

use fnv::FnvHashMap;

#[cfg(feature = "mmap")]
use crate::mmap::{ExecutableBuffer, MutableBuffer};
use crate::relocations::{ImpossibleRelocation, Relocation, RelocationKind, RelocationSize};
use crate::{AssemblyOffset, DynamicLabel, DynasmError, DynasmLabelApi, LabelKind};

/// A static label represents either a local label or a global label reference.
/// Global labels are unique names, which can be referenced multiple times, but only defined once.
/// Local labels are non-unique names. They can be referenced multiple times, and any reference
/// indicates if they refer to a label after the reference, or a label before the reference.
/// A static label records how many local labels with the same name have been emitted beforehand
/// so we can treat them as local labels as well.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticLabel {
    name: &'static str,
    version: usize,
}

impl StaticLabel {
    /// Create a new static label for a global label
    pub fn global(name: &'static str) -> StaticLabel {
        StaticLabel { name, version: 0 }
    }

    /// Create a new static label for a local label, with the given version id to distinguish it.
    pub fn local(name: &'static str, version: usize) -> StaticLabel {
        StaticLabel { name, version }
    }

    /// Returns if this static label represents a global label
    pub fn is_global(&self) -> bool {
        self.version == 0
    }

    /// Returns if this static label represents a local label
    pub fn is_local(&self) -> bool {
        self.version != 0
    }

    /// Returns the static label targetting the label with the same name, after this one.
    /// if it is a global label, returns a copy of itself.
    pub fn next(mut self) -> StaticLabel {
        self.version += (self.version != 0) as usize;
        self
    }

    /// Returns the representation of the first local label used with the given name.
    pub fn first(name: &'static str) -> StaticLabel {
        StaticLabel { name, version: 1 }
    }

    /// Returns the name of this static label
    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

/// This struct implements a protection-swapping assembling buffer
#[cfg(feature = "mmap")]
#[derive(Debug)]
pub struct MemoryManager {
    // buffer where the end result is copied into
    execbuffer: Arc<RwLock<ExecutableBuffer>>,

    // size of the allocated mmap (so we don't have to go through RwLock to get it)
    execbuffer_size: usize,
    // length of the allocated mmap that has been written into
    asmoffset: usize,

    // the address that the current execbuffer starts at
    execbuffer_addr: usize,
}

#[cfg(feature = "mmap")]
impl MemoryManager {
    /// Create a new memory manager, with `initial_mmap_size` data allocated
    pub fn new(initial_mmap_size: usize) -> io::Result<Self> {
        let execbuffer = ExecutableBuffer::new(initial_mmap_size)?;
        let execbuffer_addr = execbuffer.as_ptr() as usize;

        Ok(MemoryManager {
            execbuffer: Arc::new(RwLock::new(execbuffer)),
            execbuffer_size: initial_mmap_size,
            asmoffset: 0,
            execbuffer_addr,
        })
    }

    /// Returns the amount of bytes already committed to the manager
    pub fn committed(&self) -> usize {
        self.asmoffset
    }

    /// Returns the current start address of the managed executable memory
    pub fn execbuffer_addr(&self) -> usize {
        self.execbuffer_addr
    }

    /// Commits the data from `new` into the managed memory, calling `f` when the buffer is moved to fix anything
    /// that relies on the address of the buffer
    pub fn commit<F>(&mut self, new: &mut Vec<u8>, f: F)
    where
        F: FnOnce(&mut [u8], usize, usize),
    {
        let old_asmoffset = self.asmoffset;
        let new_asmoffset = self.asmoffset + new.len();

        if old_asmoffset >= new_asmoffset {
            return;
        }

        // see if we need to request a new buffer
        if new_asmoffset > self.execbuffer_size {
            while self.execbuffer_size <= new_asmoffset {
                self.execbuffer_size *= 2;
            }

            // create a larger writable buffer
            let mut new_buffer = MutableBuffer::new(self.execbuffer_size)
                .expect("Could not allocate a larger buffer");
            new_buffer.set_len(new_asmoffset);

            // copy over the data
            new_buffer[..old_asmoffset].copy_from_slice(&self.execbuffer.read().unwrap());
            new_buffer[old_asmoffset..].copy_from_slice(&new);
            let new_buffer_addr = new_buffer.as_ptr() as usize;

            // allow modifications to be made
            f(&mut new_buffer, self.execbuffer_addr, new_buffer_addr);

            // swap the buffers
            self.execbuffer_addr = new_buffer_addr;
            *self.execbuffer.write().unwrap() = new_buffer
                .make_exec()
                .expect("Could not swap buffer protection modes")
        } else {
            // temporarily change the buffer protection modes and copy in new data
            let mut lock = self.write();
            let buffer = mem::replace(&mut *lock, ExecutableBuffer::default());
            let mut buffer = buffer
                .make_mut()
                .expect("Could not swap buffer protection modes");

            // update buffer and length
            buffer.set_len(new_asmoffset);
            buffer[old_asmoffset..].copy_from_slice(&new);

            // repack the buffer
            let buffer = buffer
                .make_exec()
                .expect("Could not swap buffer protection modes");
            *lock = buffer;
        }

        new.clear();
        self.asmoffset = new_asmoffset;
    }

    /// Borrow the internal memory buffer mutably
    pub fn write(&self) -> RwLockWriteGuard<ExecutableBuffer> {
        self.execbuffer.write().unwrap()
    }

    /// Finalizes the currently committed part of the buffer.
    pub fn finalize(self) -> Result<ExecutableBuffer, Self> {
        match Arc::try_unwrap(self.execbuffer) {
            Ok(execbuffer) => Ok(execbuffer.into_inner().unwrap()),
            Err(arc) => Err(Self {
                execbuffer: arc,
                ..self
            }),
        }
    }

    /// Create an atomically refcounted reference to the internal executable buffer
    pub fn reader(&self) -> Arc<RwLock<ExecutableBuffer>> {
        self.execbuffer.clone()
    }
}

/// A registry of labels. Contains all necessessities for keeping track of dynasm labels.
/// This is useful when implementing your own assembler and can also be used to query
/// assemblers for the offsets of labels.
#[derive(Debug, Clone, Default)]
pub struct LabelRegistry {
    // mapping of local + global labels to offsets
    static_labels: FnvHashMap<StaticLabel, AssemblyOffset>,
    // mapping of dynamic label ids to offsets
    dynamic_labels: Vec<Option<AssemblyOffset>>,
    // mapping of local label -> current generation. Generation starts at 1.
    local_versions: FnvHashMap<&'static str, usize>,
}

impl LabelRegistry {
    /// Create a new, empty label registry
    pub fn new() -> LabelRegistry {
        LabelRegistry {
            static_labels: FnvHashMap::default(),
            dynamic_labels: Vec::new(),
            local_versions: FnvHashMap::default(),
        }
    }

    /// Create a new, empty label registry with `capacity` space for each different label type.
    pub fn with_capacity(locals: usize, globals: usize, dynamics: usize) -> LabelRegistry {
        LabelRegistry {
            static_labels: FnvHashMap::with_capacity_and_hasher(
                locals + globals,
                Default::default(),
            ),
            dynamic_labels: Vec::with_capacity(dynamics),
            local_versions: FnvHashMap::with_capacity_and_hasher(locals, Default::default()),
        }
    }

    /// Clears the internal contents of this label registry, while maintaining the backing allocations.
    pub fn clear(&mut self) {
        self.static_labels.clear();
        self.dynamic_labels.clear();
        self.local_versions.clear();
    }

    /// Create a new dynamic label id
    pub fn new_dynamic_label(&mut self) -> DynamicLabel {
        let id = self.dynamic_labels.len();
        self.dynamic_labels.push(None);
        DynamicLabel(id)
    }

    /// Define a the dynamic label `id` to be located at `offset`.
    pub fn define_dynamic(
        &mut self,
        id: DynamicLabel,
        offset: AssemblyOffset,
    ) -> Result<(), DynasmError> {
        match self.dynamic_labels.get_mut(id.0) {
            Some(Some(_)) => return Err(DynasmError::DuplicateLabel(LabelKind::Dynamic(id))),
            Some(e) => *e = Some(offset),
            None => return Err(DynasmError::UnknownLabel(LabelKind::Dynamic(id))),
        }
        Ok(())
    }

    /// Define a the global label `name` to be located at `offset`.
    pub fn define_global(
        &mut self,
        name: &'static str,
        offset: AssemblyOffset,
    ) -> Result<(), DynasmError> {
        match self.static_labels.entry(StaticLabel::global(name)) {
            Entry::Occupied(_) => Err(DynasmError::DuplicateLabel(LabelKind::Global(name))),
            Entry::Vacant(v) => {
                v.insert(offset);
                Ok(())
            }
        }
    }

    /// Define a the local label `name` to be located at `offset`.
    pub fn define_local(&mut self, name: &'static str, offset: AssemblyOffset) {
        let generation = match self.local_versions.entry(name) {
            Entry::Occupied(mut o) => {
                *o.get_mut() += 1;
                *o.get()
            }
            Entry::Vacant(v) => {
                v.insert(1);
                1
            }
        };
        self.static_labels
            .insert(StaticLabel::local(name, generation), offset);
    }

    /// Turns a local label into a static label, by adding some extra information to it
    /// so we know what local label it is even after another has been defined
    pub fn place_local_reference(&self, name: &'static str) -> Option<StaticLabel> {
        self.local_versions
            .get(name)
            .map(|&version| StaticLabel::local(name, version))
    }

    /// Returns the offset at which the dynamic label `id` was defined, if one was defined.
    pub fn resolve_dynamic(&self, id: DynamicLabel) -> Result<AssemblyOffset, DynasmError> {
        self.dynamic_labels
            .get(id.0)
            .and_then(|&e| e)
            .ok_or_else(|| DynasmError::UnknownLabel(LabelKind::Dynamic(id)))
    }

    /// Returns the offset at which the global label `name` was defined, if one was defined.
    pub fn resolve_static(&self, label: &StaticLabel) -> Result<AssemblyOffset, DynasmError> {
        self.static_labels.get(label).cloned().ok_or_else(|| {
            DynasmError::UnknownLabel(if label.is_global() {
                LabelKind::Global(label.name)
            } else {
                LabelKind::Local(label.name)
            })
        })
    }
}

/// An abstraction of a relocation of type `R`, located at `location`.
#[derive(Clone, Debug)]
pub struct PatchLoc<R: Relocation> {
    /// The AssemblyOffset at which this relocation was emitted
    pub location: AssemblyOffset,
    /// The offset, backwards, from location that the actual field to be modified starts at
    pub field_offset: u8,
    /// The offset, backwards, to be subtracted from location to get the address that the relocation should be calculated relative to.
    pub ref_offset: u8,
    /// The type of relocation to be emitted.
    pub relocation: R,
    /// A constant offset added to the destination address of this relocation when it is calculated.
    pub target_offset: isize,
}

impl<R: Relocation> PatchLoc<R> {
    /// create a new `PatchLoc`
    pub fn new(
        location: AssemblyOffset,
        target_offset: isize,
        field_offset: u8,
        ref_offset: u8,
        relocation: R,
    ) -> PatchLoc<R> {
        PatchLoc {
            location,
            field_offset,
            ref_offset,
            relocation,
            target_offset,
        }
    }

    /// Returns a range that covers the entire relocation in its assembling buffer
    /// `buf_offset` is a value that is subtracted from this range when the buffer you want to slice
    /// with this range is only a part of a bigger buffer.
    pub fn range(&self, buf_offset: usize) -> std::ops::Range<usize> {
        let field_offset = self.location.0 - buf_offset - self.field_offset as usize;
        field_offset..field_offset + self.relocation.size()
    }

    /// Returns the actual value that should be inserted at the relocation site.
    pub fn value(&self, target: usize, buf_addr: usize) -> isize {
        (match self.relocation.kind() {
            RelocationKind::Relative => {
                target.wrapping_sub(self.location.0 - self.ref_offset as usize)
            }
            RelocationKind::RelToAbs => {
                target.wrapping_sub(self.location.0 - self.ref_offset as usize + buf_addr)
            }
            RelocationKind::AbsToRel => target + buf_addr,
        }) as isize
            + self.target_offset
    }

    /// Patch `buffer` so that this relocation patch will point to `target`.
    /// `buf_addr` is the address that the assembling buffer will come to reside at when it is assembled.
    /// `target` is the offset that this relocation will be targetting.
    pub fn patch(
        &self,
        buffer: &mut [u8],
        buf_addr: usize,
        target: usize,
    ) -> Result<(), ImpossibleRelocation> {
        let value = self.value(target, buf_addr);
        self.relocation.write_value(buffer, value)
    }

    /// Patch `buffer` so that this relocation will still point to the right location due to a change in the address of the containing buffer.
    /// `buffer` is a subsection of a larger buffer, located at offset `buf_offset` in this larger buffer.
    /// `adjustment` is `new_buf_addr - old_buf_addr`.
    pub fn adjust(&self, buffer: &mut [u8], adjustment: isize) -> Result<(), ImpossibleRelocation> {
        let value = self.relocation.read_value(buffer);
        let value = match self.relocation.kind() {
            RelocationKind::Relative => value,
            RelocationKind::RelToAbs => value.wrapping_sub(adjustment),
            RelocationKind::AbsToRel => value.wrapping_add(adjustment),
        };
        self.relocation.write_value(buffer, value)
    }

    /// Returns if this patch requires adjustment when the address of the buffer it resides in is altered.
    pub fn needs_adjustment(&self) -> bool {
        match self.relocation.kind() {
            RelocationKind::Relative => false,
            RelocationKind::RelToAbs | RelocationKind::AbsToRel => true,
        }
    }
}

/// A registry of relocations and the respective labels they point towards.
#[derive(Debug, Default)]
pub struct RelocRegistry<R: Relocation> {
    static_targets: Vec<(PatchLoc<R>, StaticLabel)>,
    dynamic_targets: Vec<(PatchLoc<R>, DynamicLabel)>,
}

impl<R: Relocation> RelocRegistry<R> {
    /// Create a new, empty relocation registry.
    pub fn new() -> RelocRegistry<R> {
        RelocRegistry {
            static_targets: Vec::new(),
            dynamic_targets: Vec::new(),
        }
    }

    /// Create a new, empty relocation registry with reserved space for the specified amount of static and dynamic references
    pub fn with_capacity(static_references: usize, dynamic_references: usize) -> RelocRegistry<R> {
        RelocRegistry {
            static_targets: Vec::with_capacity(static_references),
            dynamic_targets: Vec::with_capacity(dynamic_references),
        }
    }

    /// Add a new patch targetting a static label (either global or local).
    pub fn add_static(&mut self, label: StaticLabel, patchloc: PatchLoc<R>) {
        self.static_targets.push((patchloc, label));
    }

    /// Add a new patch targetting the dynamic label `id`.
    pub fn add_dynamic(&mut self, id: DynamicLabel, patchloc: PatchLoc<R>) {
        self.dynamic_targets.push((patchloc, id))
    }

    /// Return an iterator through all defined relocations targeting global labels and the labels they target.
    /// These relocations are removed from the registry.
    pub fn take_statics<'a>(&'a mut self) -> impl Iterator<Item = (PatchLoc<R>, StaticLabel)> + 'a {
        self.static_targets.drain(..)
    }

    /// Return an iterator through all defined relocations targeting dynamic labels and the labels they target.
    /// These relocations are removed from the registry.
    pub fn take_dynamics<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (PatchLoc<R>, DynamicLabel)> + 'a {
        self.dynamic_targets.drain(..)
    }
}

/// A registry of relocations that have been encoded previously, but need to be adjusted when the address of the buffer they
/// reside in changes.
#[derive(Debug, Default)]
pub struct ManagedRelocs<R: Relocation> {
    managed: BTreeMap<usize, PatchLoc<R>>,
}

impl<R: Relocation> ManagedRelocs<R> {
    /// Create a new, empty managed relocation registry.
    pub fn new() -> Self {
        Self {
            managed: BTreeMap::new(),
        }
    }

    /// Add a relocation to this registry.
    pub fn add(&mut self, patchloc: PatchLoc<R>) {
        self.managed.insert(
            patchloc.location.0 - patchloc.field_offset as usize,
            patchloc,
        );
    }

    /// Take all items from another registry and add them to this registry
    pub fn append(&mut self, other: &mut ManagedRelocs<R>) {
        self.managed.append(&mut other.managed);
    }

    /// Remove all managed relocations whose byte fields start in the range start .. end.
    /// This is useful when implementing an `Assembler::alter` API, as any managed relocations
    /// that were overwritten should be removed from the registry, otherwise the replacement code
    /// would be corrupted when managed relocations are updated.
    pub fn remove_between(&mut self, start: usize, end: usize) {
        if start == end {
            return;
        }

        let keys: Vec<_> = self.managed.range(start..end).map(|(&k, _)| k).collect();
        for k in keys {
            self.managed.remove(&k);
        }
    }

    /// Iterate through all defined managed relocations.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a PatchLoc<R>> + 'a {
        self.managed.values()
    }
}

#[derive(Clone, Debug)]
enum LitPoolEntry {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Dynamic(RelocationSize, DynamicLabel),
    Global(RelocationSize, &'static str),
    Forward(RelocationSize, &'static str),
    Backward(RelocationSize, &'static str),
    Align(u8, usize),
}

/// Literal pool implementation. One can programmatically push items in this literal pool and retrieve offsets to them in the pool.
/// Then later, the pool will be encoded into the instruction stream and items can be retrieved using the address of the literal pool.
/// and the previously emitted offsets. Values are always at least aligned to their size.
#[derive(Clone, Debug, Default)]
pub struct LitPool {
    offset: usize,
    entries: Vec<LitPoolEntry>,
}

impl LitPool {
    /// Create a new, empty literal pool
    pub fn new() -> Self {
        LitPool {
            offset: 0,
            entries: Vec::new(),
        }
    }

    // align the pool to the specified size, record the offset, and bump the offset
    fn bump_offset(&mut self, size: RelocationSize) -> isize {
        // Correct for alignment
        self.align(size as usize, 0);
        let offset = self.offset;
        self.offset += size as usize;
        offset as isize
    }

    /// Add extra alignment for the next value in the literal pool
    pub fn align(&mut self, size: usize, with: u8) {
        let misalign = self.offset % (size as usize);
        if misalign == 0 {
            return;
        }

        self.entries.push(LitPoolEntry::Align(with, size));
        self.offset += size as usize - misalign;
    }

    /// Encode `value` into the literal pool.
    pub fn push_u8(&mut self, value: u8) -> isize {
        let offset = self.bump_offset(RelocationSize::Byte);
        self.entries.push(LitPoolEntry::U8(value));
        offset
    }

    /// Encode `value` into the literal pool.
    pub fn push_u16(&mut self, value: u16) -> isize {
        let offset = self.bump_offset(RelocationSize::Word);
        self.entries.push(LitPoolEntry::U16(value));
        offset
    }

    /// Encode `value` into the literal pool.
    pub fn push_u32(&mut self, value: u32) -> isize {
        let offset = self.bump_offset(RelocationSize::DWord);
        self.entries.push(LitPoolEntry::U32(value));
        offset
    }

    /// Encode `value` into the literal pool.
    pub fn push_u64(&mut self, value: u64) -> isize {
        let offset = self.bump_offset(RelocationSize::QWord);
        self.entries.push(LitPoolEntry::U64(value));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_dynamic(&mut self, id: DynamicLabel, size: RelocationSize) -> isize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Dynamic(size, id));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_global(&mut self, name: &'static str, size: RelocationSize) -> isize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Global(size, name));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_forward(&mut self, name: &'static str, size: RelocationSize) -> isize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Forward(size, name));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_backward(&mut self, name: &'static str, size: RelocationSize) -> isize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Backward(size, name));
        offset
    }

    fn pad_sized<D: DynasmLabelApi>(size: RelocationSize, assembler: &mut D) {
        match size {
            RelocationSize::Byte => assembler.push(0),
            RelocationSize::Word => assembler.push_u16(0),
            RelocationSize::DWord => assembler.push_u32(0),
            RelocationSize::QWord => assembler.push_u64(0),
        }
    }

    /// Emit this literal pool into the specified assembler
    pub fn emit<D: DynasmLabelApi>(self, assembler: &mut D) {
        for entry in self.entries {
            match entry {
                LitPoolEntry::U8(value) => assembler.push(value),
                LitPoolEntry::U16(value) => assembler.push_u16(value),
                LitPoolEntry::U32(value) => assembler.push_u32(value),
                LitPoolEntry::U64(value) => assembler.push_u64(value),
                LitPoolEntry::Dynamic(size, id) => {
                    Self::pad_sized(size, assembler);
                    assembler.dynamic_relocation(
                        id,
                        0,
                        size as u8,
                        size as u8,
                        D::Relocation::from_size(size),
                    );
                }
                LitPoolEntry::Global(size, name) => {
                    Self::pad_sized(size, assembler);
                    assembler.global_relocation(
                        name,
                        0,
                        size as u8,
                        size as u8,
                        D::Relocation::from_size(size),
                    );
                }
                LitPoolEntry::Forward(size, name) => {
                    Self::pad_sized(size, assembler);
                    assembler.forward_relocation(
                        name,
                        0,
                        size as u8,
                        size as u8,
                        D::Relocation::from_size(size),
                    );
                }
                LitPoolEntry::Backward(size, name) => {
                    Self::pad_sized(size, assembler);
                    assembler.backward_relocation(
                        name,
                        0,
                        size as u8,
                        size as u8,
                        D::Relocation::from_size(size),
                    );
                }
                LitPoolEntry::Align(with, alignment) => assembler.align(alignment, with),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use relocations::{Relocation, RelocationSize};
    use std::fmt::Debug;

    #[test]
    #[cfg(feature = "mmap")]
    fn test_litpool_size() {
        test_litpool::<RelocationSize>();
    }

    #[test]
    #[cfg(feature = "mmap")]
    fn test_litpool_x64() {
        test_litpool::<x64::X64Relocation>();
    }

    #[test]
    #[cfg(feature = "mmap")]
    fn test_litpool_x86() {
        test_litpool::<x86::X86Relocation>();
    }

    #[cfg(feature = "mmap")]
    #[test]
    fn test_litpool_aarch64() {
        test_litpool::<aarch64::Aarch64Relocation>();
    }

    #[cfg(feature = "mmap")]
    fn test_litpool<R: Relocation + Debug>() {
        let mut ops = Assembler::<R>::new().unwrap();
        let dynamic1 = ops.new_dynamic_label();

        let mut pool = components::LitPool::new();

        ops.local_label("backward1");

        assert_eq!(pool.push_u8(0x12), 0);
        assert_eq!(pool.push_u8(0x34), 1);
        assert_eq!(pool.push_u8(0x56), 2);

        assert_eq!(pool.push_u16(0x789A), 4);

        assert_eq!(pool.push_u32(0xBCDE_F012), 8);

        assert_eq!(pool.push_u64(0x3456_789A_BCDE_F012), 16);

        assert_eq!(pool.push_forward("forward1", RelocationSize::Byte), 24);

        pool.align(4, 0xCC);

        assert_eq!(pool.push_global("global1", RelocationSize::Word), 28);

        assert_eq!(pool.push_dynamic(dynamic1, RelocationSize::DWord), 32);

        assert_eq!(pool.push_backward("backward1", RelocationSize::QWord), 40);

        pool.emit(&mut ops);

        assert_eq!(ops.offset().0, 48);

        ops.local_label("forward1");
        ops.global_label("global1");
        ops.dynamic_label(dynamic1);

        assert_eq!(ops.commit(), Ok(()));
        let buf = ops.finalize().unwrap();

        assert_eq!(
            &*buf,
            &[
                0x12, 0x34, 0x56, 0x00, 0x9A, 0x78, 0x00, 0x00, 0x12, 0xF0, 0xDE, 0xBC, 0x00, 0x00,
                0x00, 0x00, 0x12, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 24, 0xCC, 0xCC, 0xCC,
                20, 0, 0x00, 0x00, 16, 0, 0, 0, 0x00, 0x00, 0x00, 0x00, 0xD8, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFFu8,
            ] as &[u8]
        );
    }
}
