use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::ops::Deref;
use std::mem;

#[derive(Debug)]
struct PatchLoc(usize, u8);

#[derive(Debug)]
pub struct AssemblingBuffer {
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

impl AssemblingBuffer {
    pub fn new() -> AssemblingBuffer {
        AssemblingBuffer {
            ops: Vec::new(),
            global_labels: HashMap::new(),
            dynamic_labels: HashMap::new(),
            local_labels: HashMap::new(),
            global_relocs: Vec::new(),
            dynamic_relocs: Vec::new(),
            local_relocs: HashMap::new()
        }
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
        self.ops.extend( unsafe { ::std::mem::transmute::<_, [u8; 2]>(value.to_le()) }.into_iter() );
    }

    #[inline]
    pub fn push_32(&mut self, value: i32) {
        self.ops.extend( unsafe { ::std::mem::transmute::<_, [u8; 4]>(value.to_le()) }.into_iter() );
    }

    #[inline]
    pub fn push_64(&mut self, value: i64) {
        self.ops.extend( unsafe { ::std::mem::transmute::<_, [u8; 8]>(value.to_le()) }.into_iter() );
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
    fn patch_loc(ops: &mut [u8], loc: PatchLoc, target: usize) {
        let buf = &mut ops[loc.0 - loc.1 as usize .. loc.0];

        unsafe { match loc.1 {
            1 => buf.copy_from_slice(&mem::transmute::<_, [u8; 1]>( ((target as isize - loc.0 as isize) as i8 ).to_le() )),
            2 => buf.copy_from_slice(&mem::transmute::<_, [u8; 2]>( ((target as isize - loc.0 as isize) as i16).to_le() )),
            4 => buf.copy_from_slice(&mem::transmute::<_, [u8; 4]>( ((target as isize - loc.0 as isize) as i32).to_le() )),
            8 => buf.copy_from_slice(&mem::transmute::<_, [u8; 8]>( ((target as isize - loc.0 as isize) as i64).to_le() )),
            _ => panic!("invalid patch size")
        } }
    }

    #[inline]
    pub fn global_label(&mut self, name: &'static str) {
        if let Some(name) = self.global_labels.insert(name, self.ops.len()) {
            panic!("Duplicate global label '{}'", name);
        }
    }

    #[inline]
    pub fn global_reloc(&mut self, name: &'static str, size: u8) {
        self.global_relocs.push((PatchLoc(self.ops.len(), size), name));
    }

    #[inline]
    pub fn dynamic_label(&mut self, id: usize) {
        if let Some(id) = self.dynamic_labels.insert(id, self.ops.len()) {
            panic!("Duplicate label '{}'", id);
        }
    }

    #[inline]
    pub fn dynamic_reloc(&mut self, id: usize, size: u8) {
        self.dynamic_relocs.push((PatchLoc(self.ops.len(), size), id));
    }

    #[inline]
    pub fn local_label(&mut self, name: &'static str) {
        if let Some(relocs) = self.local_relocs.remove(&name) {
            for loc in relocs {
                let len = self.ops.len();
                Self::patch_loc(&mut self.ops, loc, len);
            }
        }
        self.local_labels.insert(name, self.ops.len());
    }

    #[inline]
    pub fn forward_reloc(&mut self, name: &'static str, size: u8) {
        match self.local_relocs.entry(name) {
            Occupied(mut o) => {
                o.get_mut().push(PatchLoc(self.ops.len(), size));
            },
            Vacant(v) => {
                v.insert(vec![PatchLoc(self.ops.len(), size)]);
            }
        }
    }

    #[inline]
    pub fn backward_reloc(&mut self, name: &'static str, size: u8) {
        if let Some(&target) = self.local_labels.get(&name) {
            let len = self.ops.len();
            Self::patch_loc(&mut self.ops, PatchLoc(len, size), target)
        } else {
            panic!("Unknown local label '{}'", name);
        }
    }

    pub fn encode_relocs(&mut self) {
        for (loc, name) in self.global_relocs.drain(..) {
            if let Some(&target) = self.global_labels.get(&name) {
                Self::patch_loc(&mut self.ops, loc, target)
            } else {
                panic!("Unkonwn global label '{}'", name);
            }
        }

        for (loc, id) in self.dynamic_relocs.drain(..) {
            if let Some(&target) = self.dynamic_labels.get(&id) {
                Self::patch_loc(&mut self.ops, loc, target)
            } else {
                panic!("Unkonwn dynamic label '{}'", id);
            }
        }

        if let Some(name) = self.local_relocs.keys().next() {
            panic!("Unknown local label '{}'", name);
        }
    }
}

impl Deref for AssemblingBuffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.ops
    }
}
