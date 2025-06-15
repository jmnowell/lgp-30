
use std::process::Command;

use crate::operations::opcodes::Opcode;
use crate::hardware::memory_drum::MAX_SECTOR as MAX_TRACK_SECTOR;

#[derive(Debug)]
struct CommandWord {
    opcode: Opcode,
    track: u8,
    sector: u8,
}

enum CommandWordError {
    OpcodeDecodeFailed,
    TrackDecodeFailed,
    SectorDecodeFailed,
    EncodeFailed,
}

fn track_sector_ok(val: u8) -> bool {
    val <= MAX_TRACK_SECTOR
}

impl TryFrom<i32> for CommandWord {
    type Error = CommandWordError;

    fn try_from(val: i32) -> Result<Self, Self::Error> {
        let opcode_mask = 0x000F0000;
        let track_mask = 0x00003F000;
        let sector_mask = 0x000000FC;

        // Taking an i32 and decoding it into a 
        // valid command word is a 3-step process
        // 
        //  1) Apply the opcode mask and check that 
        //     we can get a valid opcode.
        //
        //  2) Mask out the track and verify that it's within
        //     track bound.
        //
        //  3) Repeat (2), but for the sector.
        //
        //  If all three are valid, we have a valid Command Word
        //  and can decode it.
        let opcode = val & opcode_mask; 
        let track = val & track_mask;
        let sector = val & sector_mask;
        
    }
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