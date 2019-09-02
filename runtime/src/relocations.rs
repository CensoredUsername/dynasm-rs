use byteorder::{ByteOrder, LittleEndian};


/// Used to inform assemblers on how to implement relocations for each architecture.
/// When implementing a new architecture, one simply has to implement this trait for
/// the architecture's relocation definition.
pub trait Relocation {
    /// The encoded representation for this relocation that is emitted by the dynasm! macro.
    type Encoding;
    /// construct this relocation from an encoded representation.
    fn from_encoding(encoding: Self::Encoding) -> Self;
    /// construct this relocation from a simple size. This is used to implement relocations in directives and literal pools.
    fn encode_from_size(size: RelocationSize) -> Self::Encoding;
    /// Returns the offset that this relocation is relative to, backwards with respect to the definition
    /// point of this relocation. (i.e. 0 for x64 as relocations are relative to the end of the instruction, and 4 for aarch64 as they are)
    /// Defaults to the size of this relocation.
    fn start_offset(&self) -> usize {
        self.size()
    }
    /// Returns the offset of the start of the bytes containing this relocation, backwards with respect to the definition point
    /// of this relocation.
    /// Defaults to the size of this relocation.
    fn field_offset(&self) -> usize {
        self.size()
    }
    /// The size of the slice of bytes affected by this relocation
    fn size(&self) -> usize;
    /// Write a value into a buffer of size `self.size()` in the format of this relocation.
    /// Any bits not part of the relocation should be preserved.
    fn write_value(&self, buf: &mut [u8], value: isize);
    /// Read a value from a buffer of size `self.size()` in the format of this relocation.
    fn read_value(&self, buf: &[u8]) -> isize;
    /// Specifies what kind of relocation this relocation instance is.
    fn kind(&self) -> RelocationKind;
    /// Specifies the default page size on this platform.
    fn page_size() -> usize;
}


/// Specifies what kind of relocation a relocation is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelocationKind {
    /// A simple, PC-relative relocation. These can be encoded once and do not need
    /// to be adjusted when the executable buffer is moved.
    Relative = 0,
    /// An absolute relocation to a relative address,
    /// i.e. trying to put the address of a dynasm x86 function in a register
    /// This means adjustment is necessary when the executable buffer is moved
    AbsToRel = 1,
    /// A relative relocation to an absolute address,
    /// i.e. trying to call a rust function with a dynasm x86 call.
    /// This means adjustment is necessary when the executable buffer is moved
    RelToAbs = 2,
}

impl RelocationKind {
    /// Converts back from numeric value to RelocationKind
    pub fn from_encoding(encoding: u8) -> Self {
        match encoding {
            0 => Self::Relative,
            1 => Self::AbsToRel,
            2 => Self::RelToAbs,
            x => panic!("Unsupported relocation kind {}", x)
        }
    }
}


/// A descriptor for the size of a relocation. This also doubles as a relocation itself
/// for relocations in data directives. Can be converted to relocations of any kind of architecture
/// using `Relocation::from_size`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelocationSize {
    /// A byte-sized relocation
    Byte = 1,
    /// A two-byte relocation
    Word = 2,
    /// A four-byte sized relocation
    DWord = 4,
    /// An 8-byte sized relocation
    QWord = 8,
}

impl Relocation for RelocationSize {
    type Encoding = u8;
    fn from_encoding(encoding: Self::Encoding) -> Self {
        match encoding {
            0 => RelocationSize::Byte,
            1 => RelocationSize::Word,
            2 => RelocationSize::DWord,
            3 => RelocationSize::QWord,
            x => panic!("Unsupported relocation size {}", 1u32 << x)
        }
    }
    fn encode_from_size(size: RelocationSize) -> Self::Encoding {
        match size {
            RelocationSize::Byte => 0,
            RelocationSize::Word => 1,
            RelocationSize::DWord => 2,
            RelocationSize::QWord => 3,
        }
    }
    fn size(&self) -> usize {
        *self as usize
    }
    fn write_value(&self, buf: &mut [u8], value: isize) {
        match self {
            RelocationSize::Byte => buf[0] = value as u8,
            RelocationSize::Word => LittleEndian::write_i16(buf, value as i16),
            RelocationSize::DWord => LittleEndian::write_i32(buf, value as i32),
            RelocationSize::QWord => LittleEndian::write_i64(buf, value as i64),
        }
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        match self {
            RelocationSize::Byte => buf[0] as i8 as isize,
            RelocationSize::Word => LittleEndian::read_i16(buf) as isize,
            RelocationSize::DWord => LittleEndian::read_i32(buf) as isize,
            RelocationSize::QWord => LittleEndian::read_i64(buf) as isize,
        }
    }
    fn kind(&self) -> RelocationKind {
        RelocationKind::Relative
    }
    fn page_size() -> usize {
        0
    }
}


/// Relocation implementation for the x64 architecture.
pub struct X64Relocation {
    size: RelocationSize,
    offset: u8,
}

impl Relocation for X64Relocation {
    type Encoding = (u8, u8);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        Self {
            offset: encoding.0,
            size: RelocationSize::from_encoding(encoding.1)
        }
    }
    fn encode_from_size(size: RelocationSize) -> Self::Encoding {
        (0, RelocationSize::encode_from_size(size))
    }
    fn start_offset(&self) -> usize {
        0
    }
    fn field_offset(&self) -> usize{
        self.size.size() + self.offset as usize
    }
    fn size(&self) -> usize {
        self.size.size()
    }
    fn write_value(&self, buf: &mut [u8], value: isize) {
        self.size.write_value(buf, value)
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        self.size.read_value(buf)
    }
    fn kind(&self) -> RelocationKind {
        RelocationKind::Relative
    }
    fn page_size() -> usize {
        4096
    }
}


/// Relocation implementation for the aarch64 architecture.
pub enum Aarch64Relocation {
    // b, bl 26 bits, dword aligned
    B,
    // b.cond, cbnz, cbz, ldr, ldrsw, prfm: 19 bits, dword aligned
    BCOND,
    // adr split 21 bit, byte aligned
    ADR,
    // adrp split 21 bit, 4096-byte aligned
    ADRP,
    // tbnz, tbz: 14 bits, dword aligned
    TBZ,
    // Anything in directives
    Plain(RelocationSize),
}

impl Aarch64Relocation {
    fn op_mask(&self) -> u32 {
        match self {
            Self::B => 0xFC000000,
            Self::BCOND => 0xFF00001F,
            Self::ADR => 0x9F00001F,
            Self::ADRP => 0x9F00001F,
            Self::TBZ => 0xFFF8001F,
            Self::Plain(_) => 0
        }
    }
}

impl Relocation for Aarch64Relocation {
    type Encoding = (u8,);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        match encoding.0 {
            0 => Self::B,
            1 => Self::BCOND,
            2 => Self::ADR,
            3 => Self::ADRP,
            4 => Self::TBZ,
            x  => Self::Plain(RelocationSize::from_encoding(x - 5))
        }
    }
    fn encode_from_size(size: RelocationSize) -> Self::Encoding {
        (RelocationSize::encode_from_size(size) + 5,)
    }
    fn size(&self) -> usize {
        match self {
            Self::Plain(s) => s.size(),
            _ => RelocationSize::DWord.size(),
        }
    }
    fn write_value(&self, buf: &mut [u8], value: isize) {
        if let Self::Plain(s) = self {
            return s.write_value(buf, value);
        };

        let mask = self.op_mask();
        let template = LittleEndian::read_u32(buf) & mask;
        let value = value as u32;
        let packed = match self {
            Self::B => value >> 2,
            Self::BCOND => (value >> 2) << 5,
            Self::ADR  => (((value >> 2 ) & 0x7FFFF) << 5) | (( value        & 3) << 29),
            Self::ADRP => (((value >> 14) & 0x7FFFF) << 5) | (((value >> 12) & 3) << 29),
            Self::TBZ => (value >> 5) << 2,
            Self::Plain(_) => unreachable!()
        };

        LittleEndian::write_u32(buf, template | (packed & !mask));
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        if let Self::Plain(s) = self {
            return s.read_value(buf);
        };

        let mask = !self.op_mask();
        let value = LittleEndian::read_u32(buf) & mask;
        let unpacked = match self {
            Self::B => value << 2,
            Self::BCOND => (value >> 5) << 2,
            Self::ADR  =>  (((value >> 5) & 0x7FFFF) << 2) | ((value >> 29) & 3),
            Self::ADRP => ((((value >> 5) & 0x7FFFF) << 2) | ((value >> 29) & 3)) << 12,
            Self::TBZ => (value >> 5) << 2,
            Self::Plain(_) => unreachable!()
        };

        // Sign extend.
        let bits = match self {
            Self::B => 26,
            Self::BCOND => 19,
            Self::ADR => 21,
            Self::ADRP => 21,
            Self::TBZ => 14,
            Self::Plain(_) => unreachable!()
        };
        let offset = 1u32 << (bits - 1);
        let value: u32 = (unpacked ^ offset) - offset;

        value as i32 as isize
    }
    fn kind(&self) -> RelocationKind {
        RelocationKind::Relative
    }
    fn page_size() -> usize {
        4096
    }
}


/// Relocation implementation for the x86 architecture.
pub struct X86Relocation {
    size: RelocationSize,
    kind: RelocationKind,
    offset: u8,
}

impl Relocation for X86Relocation {
    type Encoding = (u8, u8, u8);
    fn from_encoding(encoding: Self::Encoding) -> Self {
        Self {
            offset: encoding.0,
            size: RelocationSize::from_encoding(encoding.1),
            kind: RelocationKind::from_encoding(encoding.2),
        }
    }
    fn encode_from_size(size: RelocationSize) -> Self::Encoding {
        (0, RelocationSize::encode_from_size(size), RelocationKind::Relative as _)
    }
    fn start_offset(&self) -> usize {
        0
    }
    fn field_offset(&self) -> usize{
        self.size.size() + self.offset as usize
    }
    fn size(&self) -> usize {
        self.size.size()
    }
    fn write_value(&self, buf: &mut [u8], value: isize) {
        self.size.write_value(buf, value)
    }
    fn read_value(&self, buf: &[u8]) -> isize {
        self.size.read_value(buf)
    }
    fn kind(&self) -> RelocationKind {
        self.kind
    }
    fn page_size() -> usize {
        4096
    }
}
