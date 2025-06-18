use crate::common::error::*;
use crate::common::checks::*;
use crate::operations::opcodes::*;
use crate::operations::instruction::*;

use crate::hardware::memory_drum::MemoryDrum;

pub struct InstructionRegister {
    pub instruction: Instruction,
}

impl InstructionRegister {
    pub fn new() -> Self {
        InstructionRegister {
            instruction: Instruction::new(Opcode::Stop, 0, 0).unwrap()
        }
    }

    pub fn update(&mut self, val: i32) -> Result<(), Error> {
        Instruction::try_from(val)
            .map_err(|e| e)
            .map(|i| {
                self.instruction = i.clone()
            })
    }

    pub fn fetch_data(self, track: u8, sector: u8, memory: &MemoryDrum) -> Result<i32, Error> {
        if !is_track_valid(track) {
            return Err(Error::MaxTrackExceeded);
        }

        if !is_sector_valid(sector) {
            return Err(Error::MaxSectorExceeded);
        }

        memory.fetch(track, sector)
            .map_err(|e| e)
            .map(|data| data)
    }

    pub fn opcode(self) -> Opcode {
        self.instruction.opcode()
    }
}