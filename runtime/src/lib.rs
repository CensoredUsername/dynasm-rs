#[derive(Debug)]
pub struct AssemblingBuffer(Vec<u8>);

use std::ops::{Deref, DerefMut};

impl AssemblingBuffer {
    pub fn new() -> AssemblingBuffer {
        AssemblingBuffer(Vec::new())
    }

    pub fn push(&mut self, value: u8) {
        self.0.push(value);
    }

    pub fn push_8(&mut self, value: i8) {
        self.0.push(value as u8);
    }

    pub fn push_16(&mut self, value: i16) {
        self.0.extend( unsafe { ::std::mem::transmute::<_, [u8; 2]>(value.to_le())}.into_iter() );
    }

    pub fn push_32(&mut self, value: i32) {
        self.0.extend( unsafe { ::std::mem::transmute::<_, [u8; 4]>(value.to_le())}.into_iter() );
    }

    pub fn push_64(&mut self, value: i64) {
        self.0.extend( unsafe { ::std::mem::transmute::<_, [u8; 8]>(value.to_le())}.into_iter() );
    }
}

impl Deref for AssemblingBuffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl DerefMut for AssemblingBuffer {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}
