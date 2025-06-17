use crate::common::constants::*;
use crate::common::checks::*;
use crate::common::error::*;

pub struct MemoryDrum {
    memory: [[i32; MAX_TRACK as usize]; MAX_SECTOR as usize],
    head_loc: (u8, u8),
}

impl MemoryDrum {
    pub fn new() -> Self {
        MemoryDrum { 
            memory: [[i32::MIN; MAX_TRACK as usize]; MAX_SECTOR as usize],
            head_loc: (0, 0),
        }
    }

    pub fn is_memory_null(self, track: u8, sector: u8) -> Result<bool, Error> {
        match check_memory_loc(track, sector) {
            Ok(_) => return Ok(self.memory[track as usize][sector as usize] == i32::MIN),
            Err(e) => return Err(e),
        }
    }

    // TODO: Add in timing here to simulate the access speed of the LGP-30.  Since 
    //       the memory drum rotates, and the read head steps linearly, this isn't 
    //       anywhere near as fast as modern computers.
    pub fn fetch(self, track: u8, sector: u8) -> Result<i32, Error> {
        match check_memory_loc(track, sector) {
            Ok(_) => return Ok(self.memory[track as usize][sector as usize]),
            Err(e) => return Err(e),
        }
    }

    pub fn store(mut self, word: i32, track: u8, sector: u8) -> Result<bool, Error> {
        match check_memory_loc(track, sector) {
            Ok(_) => {
                self.head_loc = (track, sector);
                self.memory[track as usize][sector as usize] = word;
                return Ok(true);
            },
            Err(e) => return Err(e),
        }
    }
}