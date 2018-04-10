// some basic types that are used across multiple assemblers.
// like primitives for assembling buffers, etc

use std::io;
use std::sync::{Arc, RwLock};
use std::iter::Extend;

use take_mut;

use ::{ExecutableBuffer, MutableBuffer, AssemblyOffset};
use ::{DynasmApi};

/// This struct implements a protection-swapping assembling buffer
#[derive(Debug)]
pub struct BaseAssembler {
    // buffer where the end result is copied into
    execbuffer: Arc<RwLock<ExecutableBuffer>>,
    // instruction buffer while building the assembly
    pub ops: Vec<u8>,

    // size of the allocated mmap (so we don't have to go through RwLock to get it)
    execbuffer_size: usize,
    // length of the allocated mmap that has been written into
    asmoffset: usize,
}

impl BaseAssembler {
    pub fn new(initial_mmap_size: usize) -> io::Result<BaseAssembler> {
        Ok(BaseAssembler {
            execbuffer: Arc::new(RwLock::new(ExecutableBuffer::new(0, initial_mmap_size)?)),
            ops: Vec::new(),
            execbuffer_size: initial_mmap_size,
            asmoffset: 0,
        })
    }

    pub fn asmoffset(&self) -> usize {
        self.asmoffset
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

    pub fn commit(&mut self) {
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

            // swap the buffers
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
    /// If this is violated, it panics.
    #[inline]
    pub fn check(&mut self, offset: AssemblyOffset) {
        if self.offset > offset.0 {
            panic!("specified offset to check is smaller than the actual offset");
        }
    }

    /// Checks that the current modification offset is exactly the specified offset.
    /// If this is violated, it panics.
    #[inline]
    pub fn check_exact(&mut self, offset: AssemblyOffset) {
        if self.offset != offset.0 {
            panic!("specified offset to check is not the actual offset");
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
