use crate::operations::opcodes::Opcode;


const MAX_TRACK_SECTOR: u8 = 2^6;
struct CommandWord {
    pub Opcode: Opcode,
    pub Track: u8,
    pub Sector: u8,
}