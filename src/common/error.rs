#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidTrack,
    InvalidSector,
    Overflow,
    DivideByZero,
    BadDataInput,
    InstructionDecodeFailed,
    MemoryFetchFailed,
    MemoryStoreFailed,
    MaxTrackExceeded,
    MaxSectorExceeded,
    InstructionOpcodeDecodeFailed,
    InstructionTrackDecodeFailed,
    InstructionSectorDecodeFailed,
    InvalidData,
    OpcodeFromU8Failed,
    OpcodeFromCharFailed,
}
