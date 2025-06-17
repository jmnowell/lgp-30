use crate::hardware::memory_drum::*;
use crate::operations::instruction::{CommandWord};
use crate::operations::opcodes::Opcode;

pub struct CounterRegister {
    track: u8,
    sector: u8,
}

pub enum RegisterResult {
    
}

impl CounterRegister {
    pub fn new_at_start() -> Self {
        CounterRegister { track: 0, sector: 0 }
    }

    pub fn new(track: u8, sector: u8) -> Result<Self, RegisterResult> {
        if track > MAX_TRACK {
            return Err(RegisterResult::InvalidTrack);
        }

        if sector > MAX_SECTOR {
            return Err(RegisterResult::InvalidSector);
        }

        Ok(
            CounterRegister { track: track, sector: sector }
        )
    }

    pub fn get_track(self) -> u8 {
        self.track
    }

    pub fn get_sector(self) -> u8 {
        self.sector
    }

    pub fn update(mut self, track: u8, sector: u8) -> RegisterResult {
        if track > MAX_TRACK {
            return RegisterResult::InvalidTrack;
        }

        if sector > MAX_SECTOR {
            return RegisterResult::InvalidSector;
        }

        self.track = track;
        self.sector = sector;
        RegisterResult::Ok
    }

    pub fn reset(mut self) {
        self.track = 0;
        self.sector = 0;
    }
}

pub struct InstructionRegister {
    pub instruction: CommandWord,
}

impl InstructionRegister {
    pub fn new() -> Self {
        InstructionRegister { 
            instruction: CommandWord::new(
                crate::operations::opcodes::Opcode::Stop, 
                0, 
                0
            ).unwrap() 
        }
    }

    pub fn update(mut self, val: i32) -> RegisterResult {
        let candidate_inst = CommandWord::try_from(value);

        match candidate_inst {
            Ok(ci) => {
                self.instruction = ci;
                return RegisterResult::Ok
            },
            Err(e) => {
                return RegisterResult::InstructionDecodeFailed
            }
        }
    }

    pub fn fetch_data(mut self, track: u8, sector: u8, memory: &MemoryDrum) -> Result<i32, RegisterResult> {
        if track > MAX_TRACK {
            return RegisterResult::InvalidTrack;
        }

        if sector > MAX_SECTOR {
            return RegisterResult::InvalidSector;
        }

        let word = memory.fetch_word(track, sector);

        match word {
            Ok(val) => return Ok(val),
            Err(e) => return Err(RegisterResult::MemoryFetchFailed),
        }
    }

    pub fn opcode(self) -> Opcode {
        self.instruction.get_opcode()
    }
}