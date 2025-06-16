use crate::hardware::memory_drum::{MAX_TRACK, MAX_SECTOR, MAX_POS_DATA, MAX_NEG_DATA};
use crate::operations::command_word::{CommandWord};

pub struct CounterRegister {
    track: u8,
    sector: u8,
}

pub enum RegisterError {
    InvalidTrack,
    InvalidSector,
    Overflow,
    DivideByZero,
}

impl CounterRegister {
    pub fn new_at_start() -> Self {
        CounterRegister { track: 0, sector: 0 }
    }

    pub fn new(track: u8, sector: u8) -> Result<Self, RegisterError> {
        if track > MAX_TRACK {
            return Err(RegisterError::InvalidTrack);
        }

        if sector > MAX_SECTOR {
            return Err(RegisterError::InvalidSector);
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

    pub fn update(mut self, track: u8, sector: u8) -> bool {
        if track > MAX_TRACK {
            return false;
        }

        if sector > MAX_SECTOR {
            return false;
        }

        self.track = track;
        self.sector = sector;
        true
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
}

pub struct Accumulator {
    accumulated_value: i32
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { accumulated_value: 0 }
    }

    pub fn add(mut self, value: i32) -> bool {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return false;
        }

        self.accumulated_value += value;
        true
    }

    pub fn hi_mult(mut self, value: i32) -> bool {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return false;
        }

        let result: i64 = (self.accumulated_value * value).into();
        let result: i32 = ((result & 0xEFFFFFFF00000000) >> 32).try_into().unwrap();


        self.accumulated_value *= value;
        true
    }

    pub fn divide(mut self, value: i32) -> Result<bool, RegisterError> {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return Err(RegisterError::Overflow);
        }

        if value == 0 {
            return Err(RegisterError::DivideByZero);
        }

        let result = self.accumulated_value / value;

        // test to make sure we don't have an overflow
        if result > MAX_POS_DATA || result < MAX_NEG_DATA {
            return Err(RegisterError::Overflow)
        }

        // result is valid
        self.accumulated_value = result;
        Ok(true)
    }

    pub fn clear(mut self) {
        self.accumulated_value = 0;
    }

    pub fn load(mut self, value: i32) -> bool {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return false;
        }

        self.accumulated_value = value;
        true
    }

    pub fn store(self) -> i32 {
        self.accumulated_value
    }
}