extern crate memmap;

pub mod x64;

use std::ops::Deref;
use std::iter::Extend;
use std::mem;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use memmap::Mmap;

/// This macro takes a *const pointer from the source operand, and then casts it to the desired return type.
/// this allows it to be used as an easy shorthand for passing pointers as dynasm immediate arguments.
#[macro_export]
macro_rules! Pointer {
    ($e:expr) => {$e as *const _ as _};
}

/// Preforms the same action as the `Pointer!` macro, but casts to a *mut pointer.
#[macro_export]
macro_rules! MutPointer {
    ($e:expr) => {$e as *mut _ as _};
}

/// A struct representing an offset into the assembling buffer of a `DynasmLabelApi` struct.
/// The wrapped `usize` is the offset from the start of the assembling buffer in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssemblyOffset(pub usize);

/// A dynamic label
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicLabel(usize);

/// A structure holding a buffer of executable memory
#[derive(Debug)]
pub struct ExecutableBuffer {
    // length of the buffer that has actually been written to
    length: usize,
    // backing buffer
    buffer: Mmap
}

/// A structure wrapping some executable memory. It dereferences into a &[u8] slice.
impl ExecutableBuffer {
    /// Obtain a pointer into the executable memory from an offset into it.
    /// When an offset returned from `DynasmLabelApi::offset` is used, the resulting pointer
    /// will point to the start of the first instruction after the offset call,
    /// which can then be jumped or called to divert control flow into the executable
    /// buffer. Note that if this buffer is accessed through an Executor, these pointers
    /// will only be valid as long as its lock is held. When no locks are held,
    /// The assembler is free to relocate the executable buffer when it requires
    /// more memory than available.
    pub fn ptr(&self, offset: AssemblyOffset) -> *const u8 {
        &self[offset.0] as *const u8
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {&mut self.buffer.as_mut_slice()[..self.length] }
    }
}

impl Deref for ExecutableBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { &self.buffer.as_slice()[..self.length] }
    }
}

/// A read-only shared reference to the executable buffer inside an Assembler. By
/// locking it the internal `ExecutableBuffer` can be accessed and executed.
#[derive(Debug, Clone)]
pub struct Executor {
    execbuffer: Arc<RwLock<ExecutableBuffer>>
}

/// A read-only lockable reference to the internal `ExecutableBuffer` of an Assembler.
/// To gain access to this buffer, it must be locked.
impl Executor {
    /// Gain read-access to the internal `ExecutableBuffer`. While the returned guard
    /// is alive, it can be used to read and execute from the `ExecutableBuffer`.
    /// Any pointers created to the `Executablebuffer` should no longer be used when
    /// the guard is dropped.
    #[inline]
    pub fn lock(&self) -> RwLockReadGuard<ExecutableBuffer> {
        self.execbuffer.read().unwrap()
    }
}

/// This trait represents the interface that must be implemented to allow
/// the dynasm preprocessor to assemble into a datastructure.
pub trait DynasmApi<'a> : Extend<u8> + Extend<&'a u8> {
    /// Report the current offset into the assembling target
    fn offset(&self) -> AssemblyOffset;
    /// Push a byte into the assembling target
    fn push(&mut self, byte: u8);
    /// Push a signed byte into the assembling target
    #[inline]
    fn push_i8(&mut self, value: i8) {
        self.push(value as u8);
    }
    /// Push a signed word into the assembling target
    #[inline]
    fn push_i16(&mut self, value: i16) {
        self.extend(unsafe {
            mem::transmute::<_, [u8; 2]>(value.to_le())
        }.iter().cloned());
    }
    /// Push a signed doubleword into the assembling target
    #[inline]
    fn push_i32(&mut self, value: i32) {
        self.extend(unsafe {
            mem::transmute::<_, [u8; 4]>(value.to_le())
        }.iter().cloned());
    }
    /// Push a signed quadword into the assembling target
    #[inline]
    fn push_i64(&mut self, value: i64) {
        self.extend(unsafe {
            mem::transmute::<_, [u8; 8]>(value.to_le())
        }.iter().cloned());
    }
    /// This function is called in when a runtime error has to be generated. It panics.
    #[inline]
    fn runtime_error(&self, msg: &'static str) -> ! {
        panic!(msg);
    }
}

/// This trait extends DynasmApi to not only allow assembling, but also labels and various directives
pub trait DynasmLabelApi<'a> : DynasmApi<'a> {
    /// Push nops until the assembling target end is aligned to the given alignment
    fn align(&mut self, alignment: usize);
    /// Record the definition of a local label
    fn local_label(  &mut self, name: &'static str);
    /// Record the definition of a global label
    fn global_label( &mut self, name: &'static str);
    /// Record the definition of a dynamic label
    fn dynamic_label(&mut self, id: DynamicLabel);

    /// Record a relocation spot for a forward reference to a local label
    fn forward_reloc( &mut self, name: &'static str, size: u8);
    /// Record a relocation spot for a backward reference to a local label
    fn backward_reloc(&mut self, name: &'static str, size: u8);
    /// Record a relocation spot for a reference to a global label
    fn global_reloc(  &mut self, name: &'static str, size: u8);
    /// Record a relocation spot for a reference to a dynamic label
    fn dynamic_reloc( &mut self, id: DynamicLabel,   size: u8);
}
