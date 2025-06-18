use crate::common::constants::*;
use crate::common::checks::{is_sector_valid, is_track_valid};
use crate::common::error::*;
use crate::operations::opcodes::Opcode;


#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    opcode: Opcode,
    data_track: u8,
    data_sector: u8,
}

impl TryFrom<i32> for Instruction {
    type Error = Error;

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
        //
        //  Note: Due to the word layout, we'll have to bitshift
        //        things around.
        let opcode = val & opcode_mask >> 16;
        let track = val & track_mask >> 9;
        let sector = val & sector_mask >> 2;

        if track > u8::MAX as i32 && track > MAX_TRACK as i32 {
            return Err(Error::InstructionTrackDecodeFailed);
        }

        if sector > u8::MAX as i32 && sector > MAX_SECTOR as i32 {
            return Err(Error::InstructionSectorDecodeFailed);
        }

        if opcode > u8::MAX as i32 {
            return Err(Error::InstructionOpcodeDecodeFailed);
        }

        Opcode::try_from(opcode as u8)
            .map_err(|_e| Error::InstructionOpcodeDecodeFailed)
            .map(|o| Instruction::new(o, track as u8, sector as u8).unwrap())
    }
}

impl Into<i32> for Instruction {
    fn into(self) -> i32 {
        let opcode:u8 = self.opcode.into();
        let mut result = 0;
        result = result | opcode as i32;
        result = result << 16;

        let track = (self.data_track as i32) << 9;
        let sector = (self.data_sector as i32) << 2;

        result = result | track;
        result = result | sector;
        result
    }
}

impl Instruction {
    pub fn new(opcode: Opcode, track: u8, sector: u8) -> Result<Self, Error> {
        if !is_track_valid(track) {
            return Err(Error::MaxTrackExceeded);
        }

        if !is_sector_valid(sector) {
            return Err(Error::MaxSectorExceeded);
        }

        Ok(
            Instruction{ 
                opcode: opcode, 
                data_track: track, 
                data_sector: sector 
            }
        ) 
    }

    pub fn opcode(self) -> Opcode {
        self.opcode
    }

    pub fn data_track(self) -> u8 {
        self.data_track
    }

    pub fn data_sector(self) -> u8 {
        self.data_sector
    }
}