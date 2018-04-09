// some basic types that are used across multiple assemblers.
// like primitives for assembling buffers, etc

use std::io;
use std::sync::{Arc, RwLock};

use take_mut;

use ::{ExecutableBuffer, MutableBuffer, DynamicLabel};

/// This struct implements a protection-swapping assembling buffer
pub struct BaseAssembler {
    // buffer where the end result is copied into
    execbuffer: Arc<RwLock<ExecutableBuffer>>,
    // instruction buffer while building the assembly
    ops: Vec<u8>,

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
}
