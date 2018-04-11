extern crate memmap;
extern crate byteorder;
extern crate take_mut;

pub mod common;
pub mod x64;
pub mod x86;

use std::ops::{Deref, DerefMut};
use std::iter::Extend;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::io;
use std::error;
use std::fmt;

use memmap::{Mmap, MmapMut};
use byteorder::{ByteOrder, LittleEndian};

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

/// ExecutableBuffer equivalent that contains an MmapMut instead of an MMap
#[derive(Debug)]
struct MutableBuffer {
    // length of the buffer that has actually been written to
    length: usize,
    // backing buffer
    buffer: MmapMut
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

    fn new(length: usize, size: usize) -> io::Result<ExecutableBuffer> {
        Ok(ExecutableBuffer {
            length: length,
            buffer: MmapMut::map_anon(size)?.make_exec()?
        })
    }

    fn make_mut(self) -> io::Result<MutableBuffer> {
        Ok(MutableBuffer {
            length: self.length,
            buffer: self.buffer.make_mut()?
        })
    }
}

impl MutableBuffer {
    fn new(length: usize, size: usize) -> io::Result<MutableBuffer> {
        Ok(MutableBuffer {
            length: length,
            buffer: MmapMut::map_anon(size)?
        })
    }

    fn make_exec(self) -> io::Result<ExecutableBuffer> {
        Ok(ExecutableBuffer {
            length: self.length,
            buffer: self.buffer.make_exec()?
        })
    }
}

impl Deref for ExecutableBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.buffer[..self.length]
    }
}

impl Deref for MutableBuffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.buffer[..self.length]
    }
}

impl DerefMut for MutableBuffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.length]
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
pub trait DynasmApi: Extend<u8> + for<'a> Extend<&'a u8> {
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
        let mut buf = [0u8; 2];
        LittleEndian::write_i16(&mut buf, value);
        self.extend(&buf);
    }
    /// Push a signed doubleword into the assembling target
    #[inline]
    fn push_i32(&mut self, value: i32) {
        let mut buf = [0u8; 4];
        LittleEndian::write_i32(&mut buf, value);
        self.extend(&buf);
    }
    /// Push a signed quadword into the assembling target
    #[inline]
    fn push_i64(&mut self, value: i64) {
        let mut buf = [0u8; 8];
        LittleEndian::write_i64(&mut buf, value);
        self.extend(&buf);
    }
    /// Push an usigned word into the assembling target
    #[inline]
    fn push_u16(&mut self, value: u16) {
        let mut buf = [0u8; 2];
        LittleEndian::write_u16(&mut buf, value);
        self.extend(&buf);
    }
    /// Push an usigned doubleword into the assembling target
    #[inline]
    fn push_u32(&mut self, value: u32) {
        let mut buf = [0u8; 4];
        LittleEndian::write_u32(&mut buf, value);
        self.extend(&buf);
    }
    /// Push an usigned quadword into the assembling target
    #[inline]
    fn push_u64(&mut self, value: u64) {
        let mut buf = [0u8; 8];
        LittleEndian::write_u64(&mut buf, value);
        self.extend(&buf);
    }
    /// This function is called in when a runtime error has to be generated. It panics.
    #[inline]
    fn runtime_error(&self, msg: &'static str) -> ! {
        panic!(msg);

    }
}

/// This trait extends DynasmApi to not only allow assembling, but also labels and various directives
pub trait DynasmLabelApi : DynasmApi {
    type Relocation;

    /// Push nops until the assembling target end is aligned to the given alignment
    fn align(&mut self, alignment: usize);
    /// Record the definition of a local label
    fn local_label(  &mut self, name: &'static str);
    /// Record the definition of a global label
    fn global_label( &mut self, name: &'static str);
    /// Record the definition of a dynamic label
    fn dynamic_label(&mut self, id: DynamicLabel);

    /// Record a relocation spot for a forward reference to a local label
    fn forward_reloc( &mut self, name: &'static str, kind: Self::Relocation);
    /// Record a relocation spot for a backward reference to a local label
    fn backward_reloc(&mut self, name: &'static str, kind: Self::Relocation);
    /// Record a relocation spot for a reference to a global label
    fn global_reloc(  &mut self, name: &'static str, kind: Self::Relocation);
    /// Record a relocation spot for a reference to a dynamic label
    fn dynamic_reloc( &mut self, id: DynamicLabel,   kind: Self::Relocation);
}

/// A basic implementation of DynasmApi onto a simple Vec<u8> to assist debugging
impl DynasmApi for Vec<u8> {
    fn offset(&self) -> AssemblyOffset {
        AssemblyOffset(self.len())
    }

    fn push(&mut self, byte: u8) {
        Vec::push(self, byte);
    }
}

/// An error type that is returned from various check and check_exact methods

#[derive(Debug, Clone)]
pub enum DynasmError {
    CheckFailed
}

impl fmt::Display for DynasmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An assembly modification check failed")
    }
}

impl error::Error for DynasmError {
    fn description(&self) -> &str {
        match *self {
            DynasmError::CheckFailed => "An assembly modification offset check failed"
        }
    }
}
