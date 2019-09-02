// some basic types that are used across multiple assemblers.
// like primitives for assembling buffers, etc

use std::io;
use std::collections::hash_map::{HashMap, Entry};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::mem;

use crate::{DynamicLabel, AssemblyOffset, DynasmError, DynasmLabelApi};
use crate::mmap::{ExecutableBuffer, MutableBuffer};
use crate::relocations::{Relocation, RelocationKind, RelocationSize};


/// This struct implements a protection-swapping assembling buffer
#[derive(Debug)]
pub struct MemoryManager {
    // buffer where the end result is copied into
    execbuffer: Arc<RwLock<ExecutableBuffer>>,

    // size of the allocated mmap (so we don't have to go through RwLock to get it)
    execbuffer_size: usize,
    // length of the allocated mmap that has been written into
    asmoffset: usize,

    // the address that the current execbuffer starts at
    execbuffer_addr: usize
}

impl MemoryManager {
    /// Create a new memory manager, with `initial_mmap_size` data allocated
    pub fn new(initial_mmap_size: usize) -> io::Result<Self> {
        let execbuffer = ExecutableBuffer::new(initial_mmap_size)?;
        let execbuffer_addr = execbuffer.as_ptr() as usize;

        Ok(MemoryManager {
            execbuffer: Arc::new(RwLock::new(execbuffer)),
            execbuffer_size: initial_mmap_size,
            asmoffset: 0,
            execbuffer_addr: execbuffer_addr
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
    pub fn commit<F>(&mut self, new: &mut Vec<u8>, f: F) where F: FnOnce(&mut [u8], usize, usize) {
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
            let mut new_buffer = MutableBuffer::new(self.execbuffer_size).expect("Could not allocate a larger buffer");
            new_buffer.set_len(new_asmoffset);

            // copy over the data
            new_buffer[.. old_asmoffset].copy_from_slice(&self.execbuffer.read().unwrap());
            new_buffer[old_asmoffset..].copy_from_slice(&new);
            let new_buffer_addr = new_buffer.as_ptr() as usize;

            // allow modifications to be made
            f(&mut new_buffer, self.execbuffer_addr, new_buffer_addr);

            // swap the buffers
            self.execbuffer_addr = new_buffer_addr;
            *self.execbuffer.write().unwrap() = new_buffer.make_exec().expect("Could not swap buffer protection modes")

        } else {

            // temporarily change the buffer protection modes and copy in new data
            let mut lock = self.write();
            let buffer = mem::replace(&mut *lock, ExecutableBuffer::default());
            let mut buffer = buffer.make_mut().expect("Could not swap buffer protection modes");

            // update buffer and length
            buffer.set_len(new_asmoffset);
            buffer[old_asmoffset..].copy_from_slice(&new);

            // repack the buffer
            let buffer = buffer.make_exec().expect("Could not swap buffer protection modes");
            mem::replace(&mut *lock, buffer);
        }

        new.clear();
        self.asmoffset = new_asmoffset;
    }

    /// Borrow the internal memory buffer mutably
    pub fn write(&self) -> RwLockWriteGuard<ExecutableBuffer> {
        self.execbuffer.write().unwrap()
    }

    /// finalizes the currently committed part of the buffer.
    pub fn finalize(self) -> Result<ExecutableBuffer, Self> {
        match Arc::try_unwrap(self.execbuffer) {
            Ok(execbuffer) => Ok(execbuffer.into_inner().unwrap()),
            Err(arc) => Err(Self {
                execbuffer: arc,
                ..self
            })
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
#[derive(Debug, Clone)]
pub struct LabelRegistry {
    // mapping of global labels to offsets
    global_labels: HashMap<&'static str, AssemblyOffset>,
    // mapping of local labels to offsets
    local_labels: HashMap<&'static str, AssemblyOffset>,
    // mapping of dynamic label ids to offsets
    dynamic_labels: Vec<Option<AssemblyOffset>>,
}

impl LabelRegistry {
    /// Create a new, empty label registry
    pub fn new() -> LabelRegistry {
        LabelRegistry {
            global_labels: HashMap::new(),
            local_labels: HashMap::new(),
            dynamic_labels: Vec::new(),
        }
    }

    /// Create a new dynamic label id
    pub fn new_dynamic_label(&mut self) -> DynamicLabel {
        let id = self.dynamic_labels.len();
        self.dynamic_labels.push(None);
        DynamicLabel(id)
    }

    /// Define a the dynamic label `id` to be located at `offset`.
    pub fn define_dynamic(&mut self, id: DynamicLabel, offset: AssemblyOffset) -> Result<(), DynasmError> {
        let entry = &mut self.dynamic_labels[id.0];
        *entry.as_mut().ok_or(DynasmError::DuplicateLabel)? = offset;
        Ok(())
    }

    /// Define a the global label `name` to be located at `offset`.
    pub fn define_global(&mut self, name: &'static str, offset: AssemblyOffset) -> Result<(), DynasmError> {
        match self.global_labels.entry(name) {
            Entry::Occupied(_) => Err(DynasmError::DuplicateLabel),
            Entry::Vacant(v) => {
                v.insert(offset);
                Ok(())
            }
        }
    }

    /// Define a the local label `name` to be located at `offset`.
    pub fn define_local(&mut self, name: &'static str, offset: AssemblyOffset) {
        self.local_labels.insert(name, offset);
    }

    /// Returns the offset at which the dynamic label `id` was defined, if one was defined.
    pub fn resolve_dynamic(&self, id: DynamicLabel) -> Option<AssemblyOffset> {
        self.dynamic_labels.get(id.0).and_then(|&e| e)
    }

    /// Returns the offset at which the global label `name` was defined, if one was defined.
    pub fn resolve_global(&self, name: &'static str) -> Option<AssemblyOffset> {
        self.global_labels.get(&name).cloned()
    }

    /// Returns the offset at which the last local label named `id` was defined, if one was defined.
    pub fn resolve_local(&self, name: &'static str) -> Option<AssemblyOffset> {
        self.local_labels.get(&name).cloned()
    }
}


/// An abstraction of a relocation of type `R` that was defined to exist at offset `offset`
#[derive(Clone, Debug)]
pub struct PatchLoc<R: Relocation> {
    pub offset: AssemblyOffset,
    pub relocation: R,
}

impl<R: Relocation> PatchLoc<R> {
    /// create a new `PatchLoc`
    pub fn new(offset: AssemblyOffset, relocation: R) -> PatchLoc<R> {
        PatchLoc {
            offset,
            relocation
        }
    }

    // Slice out the relevant part of an assembling buffer
    fn slice<'a>(&self, buf_offset: usize, buffer: &'a mut [u8]) -> &'a mut [u8] {
        let field_offset = self.offset.0 - buf_offset - self.relocation.field_offset();
        &mut buffer[field_offset .. field_offset + self.relocation.size()]
    }

    fn value(&self, target: usize, buf_addr: usize) -> isize {
        (match self.relocation.kind() {
            RelocationKind::Relative => target.wrapping_sub(self.offset.0 - self.relocation.start_offset()),
            RelocationKind::RelToAbs => target.wrapping_sub(self.offset.0 - self.relocation.start_offset() + buf_addr),
            RelocationKind::AbsToRel => target + buf_addr
        }) as isize
    }

    /// Patch `buffer` so that this relocation patch will point to `target`.
    /// `buffer` is a subsection of a larger buffer, located at offset `buf_offset` in this larger buffer.
    /// `buf_addr` is the address that this larger buffer will come to reside at when it is assembled.
    pub fn patch(&self, buf_offset: usize, buf_addr: usize, buffer: &mut [u8], target: usize) {
        let buf = self.slice(buf_offset, buffer);
        let value = self.value(target, buf_addr);
        self.relocation.write_value(buf, value)
    }

    /// Patch `buffer` so that this relocation will still point to the right location due to a change in the address of the containing buffer.
    /// `buffer` is a subsection of a larger buffer, located at offset `buf_offset` in this larger buffer.
    /// `adjustment` is `new_buf_addr - old_buf_addr`.
    pub fn adjust(&self, buf_offset: usize, buffer: &mut [u8], adjustment: isize) {
        let buf = self.slice(buf_offset, buffer);
        let value = self.relocation.read_value(buf);
        let value = match self.relocation.kind() {
            RelocationKind::Relative => value,
            RelocationKind::RelToAbs => value.wrapping_sub(adjustment),
            RelocationKind::AbsToRel => value.wrapping_add(adjustment),
        };
        self.relocation.write_value(buf, value);
    }

    /// Returns if this patch requires adjustment when the address of the buffer it resides in is altered.
    pub fn needs_adjustment(&self) -> bool {
        match self.relocation.kind() {
            RelocationKind::Relative => false,
            RelocationKind::RelToAbs
            | RelocationKind::AbsToRel => true,
        }
    }
}


/// A registry of relocations and the respective labels they point towards.
pub struct RelocRegistry<R: Relocation> {
    global: Vec<(PatchLoc<R>, &'static str)>,
    dynamic: Vec<(PatchLoc<R>, DynamicLabel)>,
    local: HashMap<&'static str, Vec<PatchLoc<R>>>
}

impl<R: Relocation> RelocRegistry<R> {
    /// Create a new, empty relocation registry.
    pub fn new() -> RelocRegistry<R> {
        RelocRegistry {
            global: Vec::new(),
            dynamic: Vec::new(),
            local: HashMap::new()
        }
    }

    /// Add a new patch targetting the global label `name`.
    pub fn add_global(&mut self, name: &'static str, patchloc: PatchLoc<R>) {
        self.global.push((patchloc, name));
    }

    /// Add a new patch targetting the dynamic label `id`.
    pub fn add_dynamic(&mut self, id: DynamicLabel, patchloc: PatchLoc<R>) {
        self.dynamic.push((patchloc, id))
    }

    /// Add a new patch targetting the next local label `name`.
    /// As any relocation targetting a previous local label can be immediately resolved these should not be recorded.
    pub fn add_local(&mut self, name: &'static str, patchloc: PatchLoc<R>) {
        match self.local.entry(name) {
            Entry::Occupied(mut o) => o.get_mut().push(patchloc),
            Entry::Vacant(v) => {
                v.insert(vec![patchloc]);
            }
        }
    }

    /// Return an iterator through all defined relocations targetting local label `name`.
    /// These relocations are removed from the registry.
    pub fn take_locals_named<'a>(&'a mut self, name: &'static str) -> impl Iterator<Item=PatchLoc<R>> + 'a {
        self.local.get_mut(&name).into_iter().flat_map(|v| v.drain(..))
    }

    /// Return an iterator through all defined relocations targeting global labels and the labels they target.
    /// These relocations are removed from the registry.
    pub fn take_globals<'a>(&'a mut self) -> impl Iterator<Item=(PatchLoc<R>, &'static str)> + 'a {
        self.global.drain(..)
    }

    /// Return an iterator through all defined relocations targeting dynamic labels and the labels they target.
    /// These relocations are removed from the registry.
    pub fn take_dynamics<'a>(&'a mut self) -> impl Iterator<Item=(PatchLoc<R>, DynamicLabel)> + 'a {
        self.dynamic.drain(..)
    }

    /// Return an iterator through all defined relocations targeting local labels and the labels they target.
    /// These relocations are removed from the registry.
    pub fn take_locals<'a>(&'a mut self) -> impl Iterator<Item=(PatchLoc<R>, &'static str)> + 'a {
        self.local.iter_mut().flat_map(|(&k, v)| v.drain(..).map(move |p| (p, k)))
    }
}


/// A registry of relocations that have been encoded previously, but need to be adjusted when the address of the buffer they
/// reside in changes.
pub struct ManagedRelocs<R: Relocation> {
    managed: BTreeMap<usize, PatchLoc<R>>
}

impl<R: Relocation> ManagedRelocs<R> {
    /// Create a new, empty managed relocation registry.
    pub fn new() -> Self {
        Self {
            managed: BTreeMap::new()
        }
    }

    /// Add a relocation to this registry.
    pub fn add(&mut self, patchloc: PatchLoc<R>) {
        self.managed.insert(patchloc.offset.0 - patchloc.relocation.field_offset(), patchloc);
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

        let keys: Vec<_> = self.managed.range(start .. end).map(|(&k, _)| k).collect();
        for k in keys {
            self.managed.remove(&k);
        }
    }

    /// Iterate through all defined managed relocations.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a PatchLoc<R>> + 'a {
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
    Align(usize),
}

/// Literal pool implementation. One can programmatically push items in this literal pool and retrieve offsets to them in the pool.
/// Then later, the pool will be encoded into the instruction stream and items can be retrieved using the address of the literal pool.
/// and the previously emitted offsets. Values are always at least aligned to their size.
#[derive(Clone, Debug)]
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
    fn bump_offset(&mut self, size: RelocationSize) -> usize {
        // Correct for alignment
        self.align(size as usize);
        let offset = self.offset;
        self.offset += size as usize;
        offset
    }

    /// Add extra alignment for the next value in the literal pool
    pub fn align(&mut self, size: usize) {
        let misalign = self.offset % (size as usize);
        if misalign == 0 {
            return;
        }

        self.entries.push(LitPoolEntry::Align(size));
        self.offset += misalign;
    }

    /// Encode `value` into the literal pool.
    pub fn push_u8(&mut self, value: u8) -> usize {
        let offset = self.bump_offset(RelocationSize::Byte);
        self.entries.push(LitPoolEntry::U8(value));
        offset
    }

    /// Encode `value` into the literal pool.
    pub fn push_u16(&mut self, value: u16) -> usize {
        let offset = self.bump_offset(RelocationSize::Word);
        self.entries.push(LitPoolEntry::U16(value));
        offset
    }

    /// Encode `value` into the literal pool.
    pub fn push_u32(&mut self, value: u32) -> usize {
        let offset = self.bump_offset(RelocationSize::DWord);
        self.entries.push(LitPoolEntry::U32(value));
        offset
    }

    /// Encode `value` into the literal pool.
    pub fn push_u64(&mut self, value: u64) -> usize {
        let offset = self.bump_offset(RelocationSize::QWord);
        self.entries.push(LitPoolEntry::U64(value));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_dynamic(&mut self, id: DynamicLabel, size: RelocationSize) -> usize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Dynamic(size, id));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_global(&mut self, name: &'static str, size: RelocationSize) -> usize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Global(size, name));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_forward(&mut self, name: &'static str, size: RelocationSize) -> usize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Forward(size, name));
        offset
    }

    /// Encode the relative address of a label into the literal pool (relative to the location in the pool)
    pub fn push_backward(&mut self, name: &'static str, size: RelocationSize) -> usize {
        let offset = self.bump_offset(size);
        self.entries.push(LitPoolEntry::Backward(size, name));
        offset
    }

    /// Emit this literal pool into the specified assembler
    pub fn emit<R, D>(self, assembler: &mut D)
    where R: Relocation, D: DynasmLabelApi<Relocation=R> {
        for entry in self.entries {
            match entry {
                LitPoolEntry::U8(value) => assembler.push(value),
                LitPoolEntry::U16(value) => assembler.push_u16(value),
                LitPoolEntry::U32(value) => assembler.push_u32(value),
                LitPoolEntry::U64(value) => assembler.push_u64(value),
                LitPoolEntry::Dynamic(size, id) => assembler.dynamic_reloc(id, R::encode_from_size(size)),
                LitPoolEntry::Global(size, name) => assembler.global_reloc(name, R::encode_from_size(size)),
                LitPoolEntry::Forward(size, name) => assembler.forward_reloc(name, R::encode_from_size(size)),
                LitPoolEntry::Backward(size, name) => assembler.backward_reloc(name, R::encode_from_size(size)),
                LitPoolEntry::Align(alignment) => assembler.align(alignment),
            }
        }
    }
}
