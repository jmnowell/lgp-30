use crate::common::error::*;
use crate::common::checks::is_data_valid;

#[derive(Debug, Clone, Copy)]
pub struct DataWord {
    data: i32,
}

impl DataWord {
    pub fn new(data: i32) -> Result<Self, Error> {
        if !is_data_valid(data) {
            return Err(Error::InvalidData);
        }

        Ok(DataWord { data: data })
    }

    pub fn set_data(&mut self, data: i32) -> Result<(), Error> {
        if !is_data_valid(data) {
            return Err(Error::InvalidData);
        }

        self.data = data;
        Ok(())
    }

    pub fn get_data(self) -> i32 {
        self.data
    }
}