use std::fmt::format;

use crate::operations::opcodes::Opcode;

const MAX_TRACK_SECTOR: u8 = 2^6;
#[derive(Debug)]
struct CommandWord {
    opcode: Opcode,
    track: u8,
    sector: u8,
}

fn track_sector_ok(val: u8) -> bool {
    val <= MAX_TRACK_SECTOR
}

impl CommandWord {
    pub fn new(opcode: Opcode, track: u8, sector: u8) -> Result<Self, String> {
        let track_ok = track_sector_ok(track);
        let sector_ok = track_sector_ok(sector);

        if !track_ok {
            return Err(
                format!("Track is too large.  Max size: {}", MAX_TRACK_SECTOR)
            );
        }

        if !sector_ok {
            return Err(
                format!("Sectors is too large. Max size {}", MAX_TRACK_SECTOR)
            );
        }

        Ok(
            CommandWord { 
                opcode: opcode, 
                track: track, 
                sector: sector 
            }
        ) 
    }

    pub fn get_opcode(self) -> Opcode {
        self.opcode
    }

    pub fn get_track(self) -> u8 {
        self.track
    }

    pub fn get_sector(self) -> u8 {
        self.sector
    }
}