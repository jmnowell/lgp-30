use crate::common::checks::{is_sector_valid, is_track_valid};
use crate::common::error::Error;

pub struct ProgramCounter {
    track: u8,
    sector: u8,
}

impl ProgramCounter {
    pub fn new_at_start() -> Self {
        ProgramCounter {
            track: 0,
            sector: 0,
        }
    }

    pub fn new(track: u8, sector: u8) -> Result<Self, Error> {
        if !is_track_valid(track) {
            return Err(Error::MaxTrackExceeded);
        }

        if !is_sector_valid(sector) {
            return Err(Error::MaxSectorExceeded);
        }

        Ok(ProgramCounter {
            track: track,
            sector: sector,
        })
    }

    pub fn track(self) -> u8 {
        self.track
    }

    pub fn sector(self) -> u8 {
        self.sector
    }

    pub fn update(&mut self, track: u8, sector: u8) -> Result<(), Error> {
        if !is_track_valid(track) {
            return Err(Error::MaxTrackExceeded);
        }

        if !is_sector_valid(sector) {
            return Err(Error::MaxSectorExceeded);
        }

        self.track = track;
        self.sector = sector;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.track = 0;
        self.sector = 0;
    }
}
