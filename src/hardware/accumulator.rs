//use crate::operations::
pub struct Accumulator {
    accumulated_value: i32
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { accumulated_value: 0 }
    }

    pub fn add(mut self, value: i32) -> RegisterResult {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return RegisterResult::BadDataInput;
        }

        let result = self.accumulated_value + value;

        if result > MAX_POS_DATA || value < MAX_NEG_DATA {
            return RegisterResult::Overflow;
        }

        self.accumulated_value = result;
        RegisterResult::Ok
    }

    pub fn hi_mult(mut self, value: i32) -> RegisterResult {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return false;
        }

        let result: i64 = (self.accumulated_value * value).into();
        
        // for the hi-mult, we can shift the result back down 
        // to 32-bits, downside is this mult can overflow!
        let result: i32 = result >> 32;

        // check for overflow!
        if result > MAX_POS_DATA || result < MAX_NEG_DATA {
            return false;
        }

        self.accumulated_value = result;
        true
    }

    pub fn low_mult(mut self, value: i32) -> RegisterResult {
        if value > MAX_POS_DATA || result < MAX_NEG_DATA {
            return RegisterResult::BadDataInput;
        }

        let result: i64 = (self.accumulated_value * value).into();

        // bit mask the lower 32-bits and then simply take the value
        let result: i32 = result & 0xFFFFFFFF;

        self.accumulated_value = result;
        return RegisterResult::Ok
    }

    pub fn divide(mut self, value: i32) -> RegisterResult {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return RegisterResult::BadDataInput
        }

        if value == 0 {
            return RegisterResult::DivideByZero;
        }

        let result = self.accumulated_value / value;

        // test to make sure we don't have an overflow
        if result > MAX_POS_DATA || result < MAX_NEG_DATA {
            return RegisterResult::Overflow;
        }

        // result is valid
        self.accumulated_value = result;
        RegisterResult::Ok
    }

    pub fn clear(mut self) {
        self.accumulated_value = 0;
    }

    pub fn load(mut self, value: i32) -> RegisterResult {
        if value > MAX_POS_DATA || value < MAX_NEG_DATA {
            return RegisterResult::BadDataInput;
        }

        self.accumulated_value = value;
        RegisterResult::Ok
    }

    pub fn store(self, track: u8, sector: u8, memory: &mut MemoryDrum) -> RegisterResult {
        if track > MAX_TRACK {
            return RegisterResult::InvalidTrack;
        }

        if sector > MAX_SECTOR {
            return RegisterResult::InvalidTrack
        }

        if !memory.add_word(self.accumulated_value, track, sector) {
            return RegisterResult::MemoryStoreFailed
        }

        RegisterResult::Ok
    }