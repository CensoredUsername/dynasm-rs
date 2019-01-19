// some basic types that are used across multiple assemblers.
// like primitives for assembling buffers, etc

use std::io;
use std::sync::{Arc, RwLock};
use std::iter::Extend;
use std::collections::HashMap;

use take_mut;

use ::{ExecutableBuffer, MutableBuffer, AssemblyOffset, DynamicLabel};
use ::{DynasmApi, DynasmError};

/// This struct implements a protection-swapping assembling buffer
#[derive(Debug)]
pub(crate) struct BaseAssembler {
    // buffer where the end result is copied into
    execbuffer: Arc<RwLock<ExecutableBuffer>>,
    // instruction buffer while building the assembly
    pub ops: Vec<u8>,

    // size of the allocated mmap (so we don't have to go through RwLock to get it)
    execbuffer_size: usize,
    // length of the allocated mmap that has been written into
    asmoffset: usize,

    // the address that the current execbuffer starts at
    execbuffer_addr: usize
}

impl BaseAssembler {
    pub fn new(initial_mmap_size: usize) -> io::Result<BaseAssembler> {
        let execbuffer = ExecutableBuffer::new(0, initial_mmap_size)?;
        let execbuffer_addr = execbuffer.as_ptr() as usize;

        Ok(BaseAssembler {
            execbuffer: Arc::new(RwLock::new(execbuffer)),
            ops: Vec::new(),
            execbuffer_size: initial_mmap_size,
            asmoffset: 0,
            execbuffer_addr: execbuffer_addr
        })
    }

    pub fn asmoffset(&self) -> usize {
        self.asmoffset
    }

    pub fn execbuffer_addr(&self) -> usize {
        self.execbuffer_addr
    }

    pub fn offset(&self) -> usize {
        self.asmoffset + self.ops.len()
    }

    pub fn push(&mut self, value: u8) {
        self.ops.push(value);
    }

    pub fn align(&mut self, alignment: usize, with: u8) {
        let offset = self.offset() % alignment;
        if offset != 0 {
            for _ in 0 .. (alignment - offset) {
                self.push(with);
            }
        }
    }

    pub fn commit<F>(&mut self, f: F) where F: FnOnce(&mut [u8], usize, usize) {
        let old_asmoffset = self.asmoffset;
        let new_asmoffset = self.offset();

        if old_asmoffset >= new_asmoffset {
            return;
        }

        // see if we need to request a new buffer
        if new_asmoffset > self.execbuffer_size {
            while self.execbuffer_size <= new_asmoffset {
                self.execbuffer_size *= 2;
            }

            // create a larger writable buffer
            let mut new_buffer = MutableBuffer::new(new_asmoffset, self.execbuffer_size).expect("Could not allocate a larger buffer");

            // copy over the data
            new_buffer[.. old_asmoffset].copy_from_slice(&self.execbuffer.read().unwrap());
            new_buffer[old_asmoffset..].copy_from_slice(&self.ops);
            let new_buffer_addr = new_buffer.as_ptr() as usize;

            // allow modifications to be made
            f(&mut new_buffer, self.execbuffer_addr, new_buffer_addr);

            // swap the buffers
            self.execbuffer_addr = new_buffer_addr;
            *self.execbuffer.write().unwrap() = new_buffer.make_exec().expect("Could not swap buffer protection modes")

        } else {

            // temporarily change the buffer protection modes and copy in new data
            let mut lock = self.execbuffer.write().unwrap();
            take_mut::take_or_recover(&mut *lock, || ExecutableBuffer::new(0, 1).unwrap(), |buffer| {
                let mut buffer = buffer.make_mut().expect("Could not allocate a larger buffer");

                // update buffer and length
                buffer.length = new_asmoffset;
                buffer[old_asmoffset..].copy_from_slice(&self.ops);

                // and repack
                buffer.make_exec().expect("Could not swap buffer protection modes")
            });
        }

        self.ops.clear();
        self.asmoffset = new_asmoffset;
    }

    // finalizes the currently committed part of the buffer.
    pub fn finalize(self) -> Result<ExecutableBuffer, BaseAssembler> {
        match Arc::try_unwrap(self.execbuffer) {
            Ok(execbuffer) => Ok(execbuffer.into_inner().unwrap()),
            Err(arc) => Err(BaseAssembler {
                execbuffer: arc,
                ..self
            })
        }
    }

    pub fn reader(&self) -> Arc<RwLock<ExecutableBuffer>> {
        self.execbuffer.clone()
    }

    pub fn alter_uncommitted(&mut self) -> UncommittedModifier {
        UncommittedModifier::new(&mut self.ops, AssemblyOffset(self.asmoffset))
    }
}

impl Extend<u8> for BaseAssembler {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        self.ops.extend(iter)
    }
}

impl<'a> Extend<&'a u8> for BaseAssembler {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'a u8> {
        self.ops.extend(iter.into_iter().cloned())
    }
}


/// This struct is a wrapper around an `Assembler` normally created using the
/// `Assembler.alter_uncommitted` method. It allows the user to edit parts
/// of the assembling buffer that cannot be determined easily or efficiently
/// in advance. Due to limitations of the label resolution algorithms, this
/// assembler does not allow labels to be used.
pub struct UncommittedModifier<'a> {
    buffer: &'a mut Vec<u8>,
    base_offset: usize,
    offset: usize
}

impl<'a> UncommittedModifier<'a> {
    /// create a new uncommittedmodifier
    pub fn new(buffer: &mut Vec<u8>, base_offset: AssemblyOffset) -> UncommittedModifier {
        UncommittedModifier {
            buffer: buffer,
            base_offset: base_offset.0,
            offset: base_offset.0
        }
    }

    /// Sets the current modification offset to the given value
    #[inline]
    pub fn goto(&mut self, offset: AssemblyOffset) {
        self.offset = offset.0;
    }

    /// Checks that the current modification offset is not larger than the specified offset.
    #[inline]
    pub fn check(&mut self, offset: AssemblyOffset) -> Result<(), DynasmError> {
        if self.offset > offset.0 {
            Err(DynasmError::CheckFailed)
        } else {
            Ok(())
        }
    }

    /// Checks that the current modification offset is exactly the specified offset.
    #[inline]
    pub fn check_exact(&mut self, offset: AssemblyOffset) -> Result<(), DynasmError> {
        if self.offset != offset.0 {
            Err(DynasmError::CheckFailed)
        } else {
            Ok(())
        }
    }
}

impl<'a> DynasmApi for UncommittedModifier<'a> {
    #[inline]
    fn offset(&self) -> AssemblyOffset {
        AssemblyOffset(self.offset)
    }

    #[inline]
    fn push(&mut self, value: u8) {
        self.buffer[self.offset - self.base_offset] = value;
        self.offset += 1;
    }
}

impl<'a> Extend<u8> for UncommittedModifier<'a> {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        for i in iter {
            self.push(i)
        }
    }
}

impl<'a, 'b> Extend<&'b u8> for UncommittedModifier<'a> {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'b u8> {
        self.extend(iter.into_iter().cloned())
    }
}


/// This struct contains implementations for common parts of label handling between
/// Assemblers
#[derive(Debug, Clone)]
pub(crate) struct LabelRegistry {
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

    /// API for the user to create a new dynamic label
    pub fn new_dynamic_label(&mut self) -> DynamicLabel {
        let id = self.dynamic_labels.len();
        self.dynamic_labels.push(None);
        DynamicLabel(id)
    }

    /// API for the user to get the definition offset of a dynamic label so they don't have to
    /// keep both the dynamic label id and the offset stored when we already do so as well
    pub fn try_resolve_dynamic_label(&self, id: DynamicLabel) -> Option<AssemblyOffset> {
        self.dynamic_labels.get(id.0).and_then(|&x| x)
    }

    /// API for dynasm! to create a new dynamic label target
    pub fn dynamic_label(&mut self, id: DynamicLabel, offset: AssemblyOffset)  {
        let entry = &mut self.dynamic_labels[id.0];
        if entry.is_some() {
            panic!("Duplicate dynamic label '{}'", id.0);
        }
        *entry = Some(offset);
    }

    /// API for dynasm! to create a new global label target
    pub fn global_label(&mut self, name: &'static str, offset: AssemblyOffset) {
        if let Some(_) = self.global_labels.insert(name, offset) {
            panic!("Duplicate global label '{}'", name);
        }
    }

    /// API for dynasm! to create a new local label target
    pub fn local_label(&mut self, name: &'static str, offset: AssemblyOffset) {
        self.local_labels.insert(name, offset);
    }

    /// API for dynasm! to resolve a dynamic label reference
    pub fn resolve_dynamic_label(&self, id: DynamicLabel) -> AssemblyOffset {
        if let Some(target) = self.try_resolve_dynamic_label(id) {
            target
        } else {
            panic!("Unknown dynamic label '{}'", id.0);
        }
    }

    /// API for dynasm! to resolve a global label reference
    pub fn resolve_global_label(&self, name: &'static str) -> AssemblyOffset {
        if let Some(&target) = self.global_labels.get(&name) {
            target
        } else {
            panic!("Unknown global label '{}'", name);
        }
    }

    /// API for dynasm! to resolve a dynamic label reference
    pub fn resolve_local_label(&self, name: &'static str) -> AssemblyOffset {
        if let Some(&target) = self.local_labels.get(&name) {
            target
        } else {
            panic!("Unknown local label '{}'", name);
        }
    }
}
