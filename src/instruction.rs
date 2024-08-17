use crate::Type;

#[derive(Copy, Clone, PartialEq)]
pub enum BlockType {
    Void,
    ResolvingTo(Type),
    Functional(u32),
}

unsafe impl bytemuck::NoUninit for BlockType {}
unsafe impl bytemuck::Zeroable for BlockType {}

#[derive(Copy, Clone, PartialEq)]
pub struct BlockHeader {
    pub ty: BlockType,
    pub length: u32,
}

unsafe impl bytemuck::Zeroable for BlockHeader {}
unsafe impl bytemuck::AnyBitPattern for BlockHeader {}
unsafe impl bytemuck::NoUninit for BlockHeader {}

#[derive(Copy, Clone)]
pub struct BlindBlockHeader {
    /// number of values 'consumed' from stack
    pub consume_count: u16,
    /// number of values 'outputted' into stack
    pub output_count: u16,
    /// block code length
    pub length: u32,
}

unsafe impl bytemuck::Zeroable for BlindBlockHeader {}
unsafe impl bytemuck::AnyBitPattern for BlindBlockHeader {}
unsafe impl bytemuck::NoUninit for BlindBlockHeader {}

#[derive(Copy, Clone)]
pub struct BlindBranchHeader {
    /// number of values 'consumed' from stack
    pub consume_count: u16,
    /// number of values 'outputted' into stack
    pub output_count: u16,
    /// then case length
    pub then_length: u32,
    /// else case length
    pub else_length: u32,
}

unsafe impl bytemuck::Zeroable for BlindBranchHeader {}
unsafe impl bytemuck::AnyBitPattern for BlindBranchHeader {}
unsafe impl bytemuck::NoUninit for BlindBranchHeader {}

#[derive(Copy, Clone)]
pub struct TableBranchHeader {
    /// number of variants of branch header
    pub variant_count: u32,
}

unsafe impl bytemuck::Zeroable for TableBranchHeader {}
unsafe impl bytemuck::AnyBitPattern for TableBranchHeader {}
unsafe impl bytemuck::NoUninit for TableBranchHeader {}

unsafe impl bytemuck::NoUninit for Instruction {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    Block,
    Loop,
    If,
    Else,
    Br,
    BrIf,
    Return,
    BrTable,
    Call,
    CallIndirect,
    Drop,
    Select,
    SelectTyped,
    LocalGet,
    LocalSet,
    LocalTee,
    GlobalGet,
    GlobalSet,
    TableGet,
    TableSet,
    I32Load,
    I64Load,
    F32Load,
    F64Load,
    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,
    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
    I32Store,
    I64Store,
    F32Store,
    F64Store,
    I32Store8,
    I32Store16,
    I64Store8,
    I64Store16,
    I64Store32,
    MemorySize,
    MemoryGrow,
    I32Const,
    I64Const,
    F32Const,
    F64Const,
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    RefNull,
    RefIsNull,
    RefFunc,
    System,
    Vector,
    Unreachable,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Instruction::Unreachable as u8 {
            Ok(unsafe { std::mem::transmute::<u8, Instruction>(value) })
        } else {
            Err(())
        }
    }
}
