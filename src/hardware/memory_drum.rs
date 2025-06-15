// From the LGP-30 manual
// There are 64 tracks, which are comprised of 64 sectors,
// giving us 4096 words of instructions/data we can use
pub const MAX_TRACK: u8 = 64;
pub const MAX_SECTOR: u8 = 64;
pub const DRUM_SIZE: u16 = (MAX_TRACK as u16) * (MAX_SECTOR as u16);

// The LGP had a funky memory architecure, were the leading bit
// was a sign bit, followed by the 30 bits available to it, but the 
// last bit was a spacer bit to denote a space between memory words.
pub const MAX_POS_DATA: i32 = 2^30 - 1;
pub const MAX_NEG_DATA: i32 = (2^30) * -1;

pub struct MemoryDrum {
    memory: [[i32; MAX_TRACK as usize]; MAX_SECTOR as usize],
    head_loc: (u8, u8),
}

// MEMORY_NULL is defined as the Maxmium of i32
// the maximum value of a memory 
pub enum MemoryError {
    MaxTrackExceeded,
    MaxSectorExceeded,
    UninitializedMemory
}

impl MemoryDrum {
    pub fn new() -> Self {
        MemoryDrum { 
            memory: [[i32::MIN; MAX_TRACK as usize]; MAX_SECTOR as usize],
            head_loc: (0, 0),
        }
    }

    pub fn add_word(mut self, word: i32, track: u8, sector: u8) -> bool {
        if track > MAX_TRACK || sector > MAX_SECTOR {
            return false;
        }

        self.head_loc = (track, sector);
        self.memory[track as usize][sector as usize] = word;
        true
    }

    pub fn is_memory_null(self, track: u8, sector: u8) -> Result<bool, MemoryError> {
        if track > MAX_TRACK {
            return Err(MemoryError::MaxTrackExceeded);
        }

        if sector > MAX_SECTOR {
            return Err(MemoryError::MaxTrackExceeded);
        }

        // check to see if the memory is equal to i32::MIN
        Ok(self.memory[track as usize][sector as usize] == i32::MIN)
    }

    pub fn fetch_word(mut self, track: u8, sector: u8) -> Result<i32, MemoryError> {
        if track > MAX_TRACK {
            return Err(MemoryError::MaxTrackExceeded);
        }

        if sector > MAX_SECTOR {
            return Err(MemoryError::MaxSectorExceeded);
        }

        self.head_loc = (track, sector);

        Ok(
            self.memory[track as usize][sector as usize]
        )
    }
}