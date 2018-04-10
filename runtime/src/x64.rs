use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::iter::Extend;
use std::mem;
use std::io;

use byteorder::{ByteOrder, LittleEndian};
use take_mut;

use ::{DynasmApi, DynasmLabelApi, DynasmError};
use ::common::{BaseAssembler, LabelRegistry, UncommittedModifier};
use ::{ExecutableBuffer, MutableBuffer, Executor, DynamicLabel, AssemblyOffset};

// the argument to each relocation is the amount of bytes between the end
// of the actual relocation and the moment the relocation got emitted
// This has to be this way due to the insanity that is x64 encoding
#[derive(Debug, Clone, Copy)]
pub enum RelocationType {
    Byte,
    Word,
    DWord,
    QWord
}

impl RelocationType {
    fn size(&self) -> usize {
        match *self {
            RelocationType::Byte  => 1,
            RelocationType::Word  => 2,
            RelocationType::DWord => 4,
            RelocationType::QWord => 8,
        }
    }

    fn from_size(size: u8) -> Self {
        match size {
            1 => RelocationType::Byte,
            2 => RelocationType::Word,
            4 => RelocationType::DWord,
            8 => RelocationType::QWord,
            x => panic!("Unsupported relocation size: {}", x)
        }
    }
}

#[derive(Debug)]
struct PatchLoc(usize, u8, RelocationType);

/// This struct is an implementation of a dynasm runtime. It supports incremental
/// compilation as well as multithreaded execution with simultaneous compilation.
/// Its implementation ensures that no memory is writeable and executable at the
/// same time.
#[derive(Debug)]
pub struct Assembler {
    // protection swapping executable buffer
    base: BaseAssembler,

    // label data storage
    labels: LabelRegistry,

    // end of patch location -> name
    global_relocs: Vec<(PatchLoc, &'static str)>,
    // location to be resolved, loc, label id
    dynamic_relocs: Vec<(PatchLoc, DynamicLabel)>,
    // locations to be patched once this label gets seen. name -> Vec<locs>
    local_relocs: HashMap<&'static str, Vec<PatchLoc>>
}

/// the default starting size for an allocation by this assembler.
/// This is the page size on x64 platforms.
const MMAP_INIT_SIZE: usize = 4096;

impl Assembler {
    /// Create a new `Assembler` instance
    /// This function will return an error if it was not
    /// able to map the required executable memory. However, further methods
    /// on the `Assembler` will simply panic if an error occurs during memory
    /// remapping as otherwise it would violate the invariants of the assembler.
    /// This behaviour could be improved but currently the underlying memmap crate
    /// does not return the original mappings if a call to mprotect/VirtualProtect
    /// fails so there is no reliable way to error out if a call fails while leaving
    /// the logic of the `Assembler` intact.
    pub fn new() -> io::Result<Assembler> {
        Ok(Assembler {
            base: BaseAssembler::new(MMAP_INIT_SIZE)?,
            labels: LabelRegistry::new(),
            global_relocs: Vec::new(),
            dynamic_relocs: Vec::new(),
            local_relocs: HashMap::new()
        })
    }

    /// Create a new dynamic label that can be referenced and defined.
    pub fn new_dynamic_label(&mut self) -> DynamicLabel {
        self.labels.new_dynamic_label()
    }

    /// To allow already committed code to be altered, this method allows modification
    /// of the internal ExecutableBuffer directly. When this method is called, all
    /// data will be committed and access to the internal `ExecutableBuffer` will be locked.
    /// The passed function will then be called with an `AssemblyModifier` as argument.
    /// Using this `AssemblyModifier` changes can be made to the committed code.
    /// After this function returns, any labels in these changes will be resolved
    /// and the `ExecutableBuffer` will be unlocked again.
    pub fn alter<F>(&mut self, f: F) where F: FnOnce(&mut AssemblyModifier) -> () {
        self.commit();

        let cloned = self.base.reader();
        let mut lock = cloned.write().unwrap();

        // move the buffer out of the assembler for a bit
        take_mut::take_or_recover(&mut *lock, || ExecutableBuffer::new(0, MMAP_INIT_SIZE).unwrap(), |buf| {
            let mut buf = buf.make_mut().unwrap();

            {
                let mut m = AssemblyModifier {
                    asmoffset: 0,
                    assembler: self,
                    buffer: &mut buf
                };
                f(&mut m);
                m.encode_relocs();
            }

            // and stuff it back in
            buf.make_exec().unwrap()
        });

        // no commit is required as we directly modified the buffer.
    }

    /// Similar to `Assembler::alter`, this method allows modification of the yet to be
    /// committed assembing buffer. Note that it is not possible to use labels in this
    /// context, and overriding labels will cause corruption when the assembler tries to
    /// resolve the labels at commit time.
    pub fn alter_uncommitted(&mut self) -> UncommittedModifier {
        self.base.alter_uncommitted()
    }

    #[inline]
    fn patch_loc(&mut self, loc: PatchLoc, target: AssemblyOffset) {
        // the value that the relocation will have
        let target = target.0 as isize - loc.0 as isize;

        // slice out the part of the buffer to be overwritten with said value
        let offset = loc.0 - self.base.asmoffset() - loc.1 as usize;
        let buf = &mut self.base.ops[offset - loc.2.size() .. offset];

        match loc.2 {
            RelocationType::Byte  => buf[0] = target as i8 as u8,
            RelocationType::Word  => LittleEndian::write_i16(buf, target as i16),
            RelocationType::DWord => LittleEndian::write_i32(buf, target as i32),
            RelocationType::QWord => LittleEndian::write_i64(buf, target as i64)
        }
    }

    fn encode_relocs(&mut self) {
        let mut relocs = Vec::new();
        mem::swap(&mut relocs, &mut self.global_relocs);
        for (loc, name) in relocs {
            let target = self.labels.resolve_global_label(name);
            self.patch_loc(loc, target);
        }

        let mut relocs = Vec::new();
        mem::swap(&mut relocs, &mut self.dynamic_relocs);
        for (loc, id) in relocs {
            let target = self.labels.resolve_dynamic_label(id);
            self.patch_loc(loc, target);
        }

        if let Some(name) = self.local_relocs.keys().next() {
            panic!("Unknown local label '{}'", name);
        }
    }

    /// Commit the assembled code from a temporary buffer to the executable buffer.
    /// This method requires write access to the execution buffer and therefore
    /// has to obtain a lock on the datastructure. When this method is called, all
    /// labels will be resolved, and the result can no longer be changed.
    pub fn commit(&mut self) {
        // finalize all relocs in the newest part.
        self.encode_relocs();

        // update the executable buffer
        self.base.commit();
    }

    /// Consumes the assembler to return the internal ExecutableBuffer. This
    /// method will only fail if an `Executor` currently holds a lock on the datastructure,
    /// in which case it will return itself.
    pub fn finalize(mut self) -> Result<ExecutableBuffer, Assembler> {
        self.commit();
        match self.base.finalize() {
            Ok(execbuffer) => Ok(execbuffer),
            Err(base) => Err(Assembler {
                base: base,
                ..self
            })
        }
    }

    /// Creates a read-only reference to the internal `ExecutableBuffer` that must
    /// be locked to access it. Multiple of such read-only locks can be obtained
    /// at the same time, but as long as they are alive they will block any `self.commit()`
    /// calls.
    pub fn reader(&self) -> Executor {
        Executor {
            execbuffer: self.base.reader()
        }
    }
}

impl DynasmApi for Assembler {
    #[inline]
    fn offset(&self) -> AssemblyOffset {
        AssemblyOffset(self.base.offset())
    }

    #[inline]
    fn push(&mut self, value: u8) {
        self.base.push(value);
    }
}

impl DynasmLabelApi for Assembler {
    // would really like to have a stronger typed one here, but
    // currently it is impossible to construct an associated type just
    // from the instance name and we can't find out what assembler
    // is used in generated code.
    type Relocation = (u8, u8);

    #[inline]
    fn align(&mut self, alignment: usize) {
        self.base.align(alignment, 0x90);
    }

    #[inline]
    fn global_label(&mut self, name: &'static str) {
        let offset = self.offset();
        self.labels.global_label(name, offset);
    }

    #[inline]
    fn global_reloc(&mut self, name: &'static str, kind: Self::Relocation) {
        let offset = self.offset().0;
        self.global_relocs.push((PatchLoc(offset, kind.0, RelocationType::from_size(kind.1)), name));
    }

    #[inline]
    fn dynamic_label(&mut self, id: DynamicLabel) {
        let offset = self.offset();
        self.labels.dynamic_label(id, offset)
    }

    #[inline]
    fn dynamic_reloc(&mut self, id: DynamicLabel, kind: Self::Relocation) {
        let offset = self.offset().0;
        self.dynamic_relocs.push((PatchLoc(offset, kind.0, RelocationType::from_size(kind.1)), id));
    }

    #[inline]
    fn local_label(&mut self, name: &'static str) {
        let offset = self.offset();
        if let Some(relocs) = self.local_relocs.remove(&name) {
            for loc in relocs {
                self.patch_loc(loc, offset);
            }
        }
        self.labels.local_label(name, offset);
    }

    #[inline]
    fn forward_reloc(&mut self, name: &'static str, kind: Self::Relocation) {
        let offset = self.offset().0;
        match self.local_relocs.entry(name) {
            Occupied(mut o) => {
                o.get_mut().push(PatchLoc(offset, kind.0, RelocationType::from_size(kind.1)));
            },
            Vacant(v) => {
                v.insert(vec![PatchLoc(offset, kind.0, RelocationType::from_size(kind.1))]);
            }
        }
    }

    #[inline]
    fn backward_reloc(&mut self, name: &'static str, kind: Self::Relocation) {
        let target = self.labels.resolve_local_label(name);
        let offset = self.offset().0;
        self.patch_loc(PatchLoc(
            offset,
            kind.0,
            RelocationType::from_size(kind.1)
        ), target)
    }
}

impl Extend<u8> for Assembler {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        self.base.extend(iter)
    }
}

impl<'a> Extend<&'a u8> for Assembler {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'a u8> {
        self.base.extend(iter)
    }
}


/// This struct is a wrapper around an `Assembler` normally created using the
/// `Assembler.alter` method. Instead of writing to a temporary assembling buffer,
/// this struct assembles directly into an executable buffer. The `goto` method can
/// be used to set the assembling offset in the `ExecutableBuffer` of the assembler
/// (this offset is initialized to 0) after which the data at this location can be
/// overwritten by assembling into this struct.
pub struct AssemblyModifier<'a: 'b, 'b> {
    assembler: &'a mut Assembler,
    buffer: &'b mut MutableBuffer,
    asmoffset: usize
}

impl<'a, 'b> AssemblyModifier<'a, 'b> {
    /// Sets the current modification offset to the given value
    #[inline]
    pub fn goto(&mut self, offset: AssemblyOffset) {
        self.asmoffset = offset.0;
    }

    /// Checks that the current modification offset is not larger than the specified offset.
    #[inline]
    pub fn check(&mut self, offset: AssemblyOffset) -> Result<(), DynasmError> {
        if self.asmoffset > offset.0 {
            Err(DynasmError::CheckFailed)
        } else {
            Ok(())
        }
    }

    /// Checks that the current modification offset is exactly the specified offset.
    #[inline]
    pub fn check_exact(&mut self, offset: AssemblyOffset) -> Result<(), DynasmError> {
        if self.asmoffset != offset.0 {
            Err(DynasmError::CheckFailed)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn patch_loc(&mut self, loc: PatchLoc, target: AssemblyOffset) {
        // the value that the relocation will have
        let target = target.0 as isize - loc.0 as isize;

        // slice out the part of the buffer to be overwritten with said value
        let offset = loc.0 - loc.1 as usize;
        let buf = &mut self.buffer[offset - loc.2.size() .. offset];

        match loc.2 {
            RelocationType::Byte  => buf[0] = target as i8 as u8,
            RelocationType::Word  => LittleEndian::write_i16(buf, target as i16),
            RelocationType::DWord => LittleEndian::write_i32(buf, target as i32),
            RelocationType::QWord => LittleEndian::write_i64(buf, target as i64)
        }
    }

    fn encode_relocs(&mut self) {
        let mut relocs = Vec::new();
        mem::swap(&mut relocs, &mut self.assembler.global_relocs);
        for (loc, name) in relocs {
            let target = self.assembler.labels.resolve_global_label(name);
            self.patch_loc(loc, target);
        }

        let mut relocs = Vec::new();
        mem::swap(&mut relocs, &mut self.assembler.dynamic_relocs);
        for (loc, id) in relocs {
            let target = self.assembler.labels.resolve_dynamic_label(id);
            self.patch_loc(loc, target);
        }

        if let Some(name) = self.assembler.local_relocs.keys().next() {
            panic!("Unknown local label '{}'", name);
        }
    }
}

impl<'a, 'b> DynasmApi for AssemblyModifier<'a, 'b> {
    #[inline]
    fn offset(&self) -> AssemblyOffset {
        AssemblyOffset(self.asmoffset)
    }

    #[inline]
    fn push(&mut self, value: u8) {
        self.buffer[self.asmoffset] = value;
        self.asmoffset += 1;
    }
}

impl<'a, 'b> DynasmLabelApi for AssemblyModifier<'a, 'b> {
    type Relocation = (u8, u8);

    #[inline]
    fn align(&mut self, alignment: usize) {
        self.assembler.align(alignment);
    }

    #[inline]
    fn global_label(&mut self, name: &'static str) {
        self.assembler.global_label(name);
    }

    #[inline]
    fn global_reloc(&mut self, name: &'static str, kind: Self::Relocation) {
        self.assembler.global_reloc(name, kind);
    }

    #[inline]
    fn dynamic_label(&mut self, id: DynamicLabel) {
        self.assembler.dynamic_label(id);
    }

    #[inline]
    fn dynamic_reloc(&mut self, id: DynamicLabel, kind: Self::Relocation) {
        self.assembler.dynamic_reloc(id, kind);
    }

    #[inline]
    fn local_label(&mut self, name: &'static str) {
        let offset = self.offset();
        if let Some(relocs) = self.assembler.local_relocs.remove(&name) {
            for loc in relocs {
                self.patch_loc(loc, offset);
            }
        }
        self.assembler.labels.local_label(name, offset);
    }

    #[inline]
    fn forward_reloc(&mut self, name: &'static str, kind: Self::Relocation) {
        self.assembler.forward_reloc(name, kind);
    }

    #[inline]
    fn backward_reloc(&mut self, name: &'static str, kind: Self::Relocation) {
        let target = self.assembler.labels.resolve_local_label(name);
        let offset = self.offset();
        self.patch_loc(PatchLoc(
            offset.0,
            kind.0,
            RelocationType::from_size(kind.1)
        ), target)
    }
}

impl<'a, 'b> Extend<u8> for AssemblyModifier<'a, 'b> {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        for i in iter {
            self.push(i)
        }
    }
}

impl<'a, 'b, 'c> Extend<&'c u8> for AssemblyModifier<'a, 'b> {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'c u8> {
        self.extend(iter.into_iter().cloned())
    }
}
