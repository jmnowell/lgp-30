use crate::common::error::*;
use crate::common::checks::*;
use crate::hardware::memory_drum::*;

pub struct Accumulator {
    accumulated_value: i32
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { accumulated_value: 0 }
    }

    pub fn add(&mut self, value: i32) -> Result<(), Error> {
        if !is_data_valid(value) {
            return Err(Error::BadDataInput);
        }
        
        let result = self.accumulated_value + value;

        if !is_data_valid(result) {
            return Err(Error::Overflow);
        }

        self.accumulated_value = result;
        Ok(())
    }

    pub fn hi_mult(&mut self, value: i32) -> Result<(), Error> {
        if !is_data_valid(value) {
            return Err(Error::BadDataInput);
        }

        let result: i64 = (self.accumulated_value * value).into();
        
        // for the hi-mult, we can shift the result back down 
        // to 32-bits, downside is this mult can overflow!
        let result: i32 = (result >> 32) as i32;

        // check for overflow!
        if !is_data_valid(result) {
            return Err(Error::Overflow);
        }

        self.accumulated_value = result;
        Ok(())
    }

    pub fn low_mult(&mut self, value: i32) -> Result<(), Error> {
        if !is_data_valid(value) {
            return Err(Error::BadDataInput);
        }

        let result: i64 = (self.accumulated_value * value).into();

        // bit mask the lower 32-bits and then simply take the value
        let result: i32 = (result & 0xFFFFFFFF) as i32;

        self.accumulated_value = result;
        Ok(())
    }

    pub fn divide(&mut self, value: i32) -> Result<(), Error> {
        if !is_data_valid(value) {
            return Err(Error::BadDataInput);
        }

        if value == 0 {
            return Err(Error::DivideByZero);
        }

        let result = self.accumulated_value / value;

        // test to make sure we don't have an overflow
        if !is_data_valid(result) {
            return Err(Error::Overflow);
        }

        // result is valid
        self.accumulated_value = result;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.accumulated_value = 0;
    }

    pub fn load(&mut self, value: i32) -> Result<(), Error> {
        if !is_data_valid(value) {
            return Err(Error::BadDataInput);
        }

        self.accumulated_value = value;
        Ok(())
    }

    pub fn store(self, track: u8, sector: u8, memory: &mut MemoryDrum) -> Result<(), Error> {
        if !is_track_valid(track) {
            return Err(Error::MaxTrackExceeded);
        }

        if !is_sector_valid(sector) {
            return Err(Error::InvalidSector);
        }

        let result = match memory.store(self.accumulated_value, track, sector) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };

        result
    }
}