extern crate memmap;
extern crate take_mut;
extern crate byteorder;

pub mod common;
pub mod x64;
pub mod x86;
pub mod aarch64;

use std::ops::{Deref, DerefMut};
use std::iter::Extend;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::io;
use std::error;
use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use memmap::{Mmap, MmapMut};


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


/// This struct contains implementations for common parts of label handling between
/// Assemblers
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

    /// API for the user to create a new dynamic label
    pub fn new_dynamic_label(&mut self) -> DynamicLabel {
        let id = self.dynamic_labels.len();
        self.dynamic_labels.push(None);
        DynamicLabel(id)
    }

    /// API for dynasm! to create a new dynamic label target
    pub fn define_dynamic(&mut self, id: DynamicLabel, offset: AssemblyOffset) -> Result<(), DynasmError> {
        let entry = &mut self.dynamic_labels[id.0];
        *entry.as_mut().ok_or(DynasmError::DuplicateLabel)? = offset;
        Ok(())
    }

    /// API for dynasm! to create a new global label target
    pub fn define_global(&mut self, name: &'static str, offset: AssemblyOffset) -> Result<(), DynasmError> {
        match self.global_labels.entry(name) {
            Entry::Occupied(_) => Err(DynasmError::DuplicateLabel),
            Entry::Vacant(v) => {
                v.insert(offset);
                Ok(())
            }
        }
    }

    /// API for dynasm! to create a new local label target
    pub fn define_local(&mut self, name: &'static str, offset: AssemblyOffset) {
        self.local_labels.insert(name, offset);
    }

    /// API for dynasm! to resolve a dynamic label reference
    pub fn resolve_dynamic(&self, id: DynamicLabel) -> Option<AssemblyOffset> {
        self.dynamic_labels.get(id.0).and_then(|&e| e)
    }

    /// API for dynasm! to resolve a global label reference
    pub fn resolve_global(&self, name: &'static str) -> Option<AssemblyOffset> {
        self.global_labels.get(&name).cloned()
    }

    /// API for dynasm! to resolve a dynamic label reference
    pub fn resolve_local(&self, name: &'static str) -> Option<AssemblyOffset> {
        self.local_labels.get(&name).cloned()
    }
}


/// This trait represents the interface that must be implemented to allow
/// the dynasm preprocessor to assemble into a datastructure.
pub trait DynasmApi: Extend<u8> + for<'a> Extend<&'a u8> {
    /// Report the current offset into the assembling target
    fn offset(&self) -> AssemblyOffset;
    /// Push a byte into the assembling target
    fn push(&mut self, byte: u8);
    /// Push filler until the assembling target end is aligned to the given alignment.
    fn align(&mut self, alignment: usize);

    #[inline]
    /// Push a signed byte into the assembling target
    fn push_i8(&mut self, value: i8) {
        self.push(value as u8);
    }
    /// Push a signed word into the assembling target
    #[inline]
    fn push_i16(&mut self, value: i16) {
        self.extend(&value.to_le_bytes());
    }
    /// Push a signed doubleword into the assembling target
    #[inline]
    fn push_i32(&mut self, value: i32) {
        self.extend(&value.to_le_bytes());
    }
    /// Push a signed quadword into the assembling target
    #[inline]
    fn push_i64(&mut self, value: i64) {
        self.extend(&value.to_le_bytes());
    }
    /// Push an usigned word into the assembling target
    #[inline]
    fn push_u16(&mut self, value: u16) {
        self.extend(&value.to_le_bytes());
    }
    /// Push an usigned doubleword into the assembling target
    #[inline]
    fn push_u32(&mut self, value: u32) {
        self.extend(&value.to_le_bytes());
    }
    /// Push an usigned quadword into the assembling target
    #[inline]
    fn push_u64(&mut self, value: u64) {
        self.extend(&value.to_le_bytes());
    }
    /// This function is called in when a runtime error has to be generated. It panics.
    #[inline]
    fn runtime_error(&self, msg: &'static str) -> ! {
        panic!(msg);

    }
}

/// This trait extends DynasmApi to not only allow assembling, but also labels and various directives
pub trait DynasmLabelApi : DynasmApi {
    /// The relocation info type this assembler uses. 
    type Relocation;

    fn registry(&self) -> &LabelRegistry;
    fn registry_mut(&mut self) -> &mut LabelRegistry;

    /// Record the definition of a local label
    fn local_label(  &mut self, name: &'static str);
    /// Record the definition of a global label
    fn global_label( &mut self, name: &'static str) {
        let offset = self.offset();
        self.registry_mut().define_global(name, offset).unwrap();
    }
    /// Record the definition of a dynamic label
    fn dynamic_label(&mut self, id: DynamicLabel) {
        let offset = self.offset();
        self.registry_mut().define_dynamic(id, offset).unwrap();
    }

    /// Record a relocation spot for a forward reference to a local label
    fn forward_reloc( &mut self, name: &'static str, kind: Self::Relocation);
    /// Record a relocation spot for a backward reference to a local label
    fn backward_reloc(&mut self, name: &'static str, kind: Self::Relocation);
    /// Record a relocation spot for a reference to a global label
    fn global_reloc(  &mut self, name: &'static str, kind: Self::Relocation);
    /// Record a relocation spot for a reference to a dynamic label
    fn dynamic_reloc( &mut self, id: DynamicLabel,   kind: Self::Relocation);
    /// Record a relocation spot to an arbitrary target.
    fn bare_reloc(    &mut self, target: usize,      kind: Self::Relocation);
}


/// An assembler that is purely a Vec<u8>. It doesn't support labels, but can be used to easily inspect generated code.
pub struct VecAssembler(Vec<u8>);

impl Extend<u8> for VecAssembler {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        self.0.extend(iter)
    }
}

impl<'a> Extend<&'a u8> for VecAssembler {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'a u8> {
        self.0.extend(iter)
    }
}

impl DynasmApi for VecAssembler {
    #[inline]
    fn offset(&self) -> AssemblyOffset {
        AssemblyOffset(self.0.len())
    }

    #[inline]
    fn push(&mut self, byte: u8) {
        self.0.push(byte);
    }

    #[inline]
    fn align(&mut self, alignment: usize) {
        let offset = self.offset().0 % alignment;
        if offset != 0 {
            for _ in offset .. alignment {
                self.push(0);
            }
        }
    }
}


/// An error type that is returned from various check and check_exact methods
#[derive(Debug, Clone)]
pub enum DynasmError {
    CheckFailed,
    DuplicateLabel
}

impl fmt::Display for DynasmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DynasmError::CheckFailed => write!(f, "An assembly modification check failed"),
            DynasmError::DuplicateLabel => write!(f, "Duplicate label defined"),
        }
    }
}

impl error::Error for DynasmError {
    fn description(&self) -> &str {
        match *self {
            DynasmError::CheckFailed => "An assembly modification offset check failed",
            DynasmError::DuplicateLabel => "Duplicate label defined",
        }
    }
}
