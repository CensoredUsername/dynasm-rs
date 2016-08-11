extern crate memmap;

use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::ops::Deref;
use std::iter::Extend;
use std::mem;
use std::cmp;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use memmap::{Mmap, Protection};

#[derive(Debug)]
struct PatchLoc(usize, u8);

#[derive(Debug)]
pub struct Assembler {
    // buffer where the end result is copied into
    execbuffer: Arc<RwLock<Mmap>>,

    // offset of the current assembly buffer w.r.t. the start of the execbuffer
    asmoffset: usize,
    // instruction buffer while building the assembly
    ops: Vec<u8>,

    // label name -> target loc
    global_labels: HashMap<&'static str, usize>,
    // end of patch location -> name
    global_relocs: Vec<(PatchLoc, &'static str)>, 

    // label id -> target loc
    dynamic_labels: HashMap<usize, usize>,
    // location to be resolved, loc, label id
    dynamic_relocs: Vec<(PatchLoc, usize)>,

    // labelname -> most recent patch location
    local_labels: HashMap<&'static str, usize>,
    // locations to be patched once this label gets seen. name -> Vec<locs>
    local_relocs: HashMap<&'static str, Vec<PatchLoc>>
}

#[derive(Debug, Clone)]
pub struct Executor {
    execbuffer: Arc<RwLock<Mmap>>
}

pub struct ExecutionGuard<'a> {
    guard: RwLockReadGuard<'a, Mmap>
}

pub struct ExecutableBuffer {
    backing: Mmap
}

impl Extend<u8> for Assembler {
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        self.ops.extend(iter)
    }
}

impl Assembler {
    pub fn new() -> Assembler {
        let size = 256;
        Assembler {
            execbuffer: Arc::new(RwLock::new(Mmap::anonymous(size, Protection::ReadExecute).unwrap())),
            asmoffset: 0,
            ops: Vec::new(),
            global_labels: HashMap::new(),
            dynamic_labels: HashMap::new(),
            local_labels: HashMap::new(),
            global_relocs: Vec::new(),
            dynamic_relocs: Vec::new(),
            local_relocs: HashMap::new()
        }
    }

    pub fn offset(&self) -> usize {
        self.ops.len() + self.asmoffset
    }

    #[inline]
    pub fn push(&mut self, value: u8) {
        self.ops.push(value);
    }

    #[inline]
    pub fn push_8(&mut self, value: i8) {
        self.ops.push(value as u8);
    }

    #[inline]
    pub fn push_16(&mut self, value: i16) {
        self.ops.extend( unsafe {
            mem::transmute::<_, [u8; 2]>(value.to_le())
        }.into_iter());
    }

    #[inline]
    pub fn push_32(&mut self, value: i32) {
        self.ops.extend( unsafe {
            mem::transmute::<_, [u8; 4]>(value.to_le())
        }.into_iter());
    }

    #[inline]
    pub fn push_64(&mut self, value: i64) {
        self.ops.extend( unsafe {
            mem::transmute::<_, [u8; 8]>(value.to_le())
        }.into_iter());
    }

    #[inline]
    pub fn align(&mut self, to: usize) {
        if self.ops.len() % to != 0 {
            for _ in 0..(to - self.ops.len() % to) {
                self.ops.push(0x90)
            }
        }
    }

    #[inline]
    fn patch_loc(&mut self, loc: PatchLoc, target: usize) {
        let buf_loc = loc.0 - self.asmoffset;
        let buf = &mut self.ops[buf_loc - loc.1 as usize .. buf_loc];
        let target = target as isize - loc.0 as isize;

        unsafe { match loc.1 {
            1 => buf.copy_from_slice(&mem::transmute::<_, [u8; 1]>( (target as i8 ).to_le() )),
            2 => buf.copy_from_slice(&mem::transmute::<_, [u8; 2]>( (target as i16).to_le() )),
            4 => buf.copy_from_slice(&mem::transmute::<_, [u8; 4]>( (target as i32).to_le() )),
            8 => buf.copy_from_slice(&mem::transmute::<_, [u8; 8]>( (target as i64).to_le() )),
            _ => panic!("invalid patch size")
        } }
    }

    #[inline]
    pub fn global_label(&mut self, name: &'static str) {
        let offset = self.offset();
        if let Some(name) = self.global_labels.insert(name, offset) {
            panic!("Duplicate global label '{}'", name);
        }
    }

    #[inline]
    pub fn global_reloc(&mut self, name: &'static str, size: u8) {
        let offset = self.offset();
        self.global_relocs.push((PatchLoc(offset, size), name));
    }

    #[inline]
    pub fn dynamic_label(&mut self, id: usize) {
        let offset = self.offset();
        if let Some(id) = self.dynamic_labels.insert(id, offset) {
            panic!("Duplicate label '{}'", id);
        }
    }

    #[inline]
    pub fn dynamic_reloc(&mut self, id: usize, size: u8) {
        let offset = self.offset();
        self.dynamic_relocs.push((PatchLoc(offset, size), id));
    }

    #[inline]
    pub fn local_label(&mut self, name: &'static str) {
        let offset = self.offset();
        if let Some(relocs) = self.local_relocs.remove(&name) {
            for loc in relocs {
                self.patch_loc(loc, offset);
            }
        }
        self.local_labels.insert(name, offset);
    }

    #[inline]
    pub fn forward_reloc(&mut self, name: &'static str, size: u8) {
        let offset = self.offset();
        match self.local_relocs.entry(name) {
            Occupied(mut o) => {
                o.get_mut().push(PatchLoc(offset, size));
            },
            Vacant(v) => {
                v.insert(vec![PatchLoc(offset, size)]);
            }
        }
    }

    #[inline]
    pub fn backward_reloc(&mut self, name: &'static str, size: u8) {
        if let Some(&target) = self.local_labels.get(&name) {
            let len = self.offset();
            self.patch_loc(PatchLoc(len, size), target)
        } else {
            panic!("Unknown local label '{}'", name);
        }
    }

    fn encode_relocs(&mut self) {
        let mut relocs = Vec::new();
        mem::swap(&mut relocs, &mut self.global_relocs);
        for (loc, name) in relocs {
            if let Some(&target) = self.global_labels.get(&name) {
                self.patch_loc(loc, target)
            } else {
                panic!("Unkonwn global label '{}'", name);
            }
        }

        let mut relocs = Vec::new();
        mem::swap(&mut relocs, &mut self.dynamic_relocs);
        for (loc, id) in relocs {
            if let Some(&target) = self.dynamic_labels.get(&id) {
                self.patch_loc(loc, target)
            } else {
                panic!("Unkonwn dynamic label '{}'", id);
            }
        }

        if let Some(name) = self.local_relocs.keys().next() {
            panic!("Unknown local label '{}'", name);
        }
    }

    pub fn commit(&mut self) {
        // finalize all relocs in the newest part.
        self.encode_relocs();
        let old_len = self.asmoffset;
        let new_len = self.offset();

        let buffer_len = self.execbuffer.read().unwrap().len();

        if new_len > buffer_len {
            // resize buffer
            let mut new_buf = Mmap::anonymous(cmp::max(new_len, buffer_len * 2), Protection::ReadWrite).unwrap();
            // copy over from the old buffer and the asm buffer (unsafe is completely safe due to use of anonymous mappings)
            unsafe {
                new_buf.as_mut_slice()[..old_len].copy_from_slice(&self.execbuffer.read().unwrap().as_slice()[..old_len]);
                new_buf.as_mut_slice()[old_len..new_len].copy_from_slice(&self.ops[..]);
            }
            new_buf.set_protection(Protection::ReadExecute).unwrap();
            // swap the buffers.
            mem::swap(&mut new_buf, &mut self.execbuffer.write().unwrap());
            // and the old buffer is dropped.
        } else {
            // make the buffer writeable and copy things over.
            let mut buf = self.execbuffer.write().unwrap();
            buf.set_protection(Protection::ReadWrite).unwrap();
            unsafe {
                buf.as_mut_slice()[old_len..new_len].copy_from_slice(&self.ops[..]);
            }
            buf.set_protection(Protection::ReadExecute).unwrap();
        }
        self.ops.clear();
        self.asmoffset = new_len;
    }

    pub fn finalize(mut self) -> Result<ExecutableBuffer, Assembler> {
        self.commit();
        match Arc::try_unwrap(self.execbuffer) {
            Ok(map) => Ok(ExecutableBuffer {
                backing: map.into_inner().unwrap()
            }),
            Err(arc) => Err(Assembler {
                execbuffer: arc,
                ..self
            })
        }
    }

    pub fn reader(&self) -> Executor {
        Executor {
            execbuffer: self.execbuffer.clone()
        }
    }
}

impl Executor {
    pub fn lock(&self) -> ExecutionGuard {
        ExecutionGuard {
            guard: self.execbuffer.read().unwrap()
        }
    }
}

impl<'a> ExecutionGuard<'a> {
    pub fn ptr(&self, idx: usize) -> *const u8 {
        &self[idx] as *const u8
    }
}

impl<'a> Deref for ExecutionGuard<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { &self.guard.as_slice() }
    }
}

impl ExecutableBuffer {
    pub fn into_intter(self) -> Mmap {
        self.backing
    }

    pub fn ptr(&self, idx: usize) -> *const u8 {
        &self[idx] as *const u8
    }
}

impl Deref for ExecutableBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { self.backing.as_slice() }
    }
}