use crate::common::constants::{MAX_NEG_DATA, MAX_POS_DATA, MAX_SECTOR, MAX_TRACK};
use crate::common::error::*;

pub fn is_track_valid(track: u8) -> bool {
    track <= MAX_TRACK
}

pub fn is_sector_valid(sector: u8) -> bool {
    sector <= MAX_SECTOR
}

pub fn is_data_valid(val: i32) -> bool {
    val >= MAX_NEG_DATA && val <= MAX_POS_DATA
}

pub fn check_memory_loc(track: u8, sector: u8) -> Result<(), Error> {
    if !is_track_valid(track) {
        return Err(Error::MaxTrackExceeded);
    }

    if !is_sector_valid(sector) {
        return Err(Error::MaxSectorExceeded);
    }

    Ok(())
}