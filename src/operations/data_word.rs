use crate::common::constants::{MAX_POS_DATA, MAX_NEG_DATA};

#[derive(Debug, Clone, Copy)]
pub struct DataWord {
    data: i32,
    pub is_negative: bool,
}

fn check_data(data: &i32) -> bool {
    if *data > MAX_POS_DATA || *data < MAX_NEG_DATA {
        return false;
    }

    true
}

impl DataWord {
    pub fn new(data: i32) -> Result<Self, String> {
        if !check_data(&data) {
            return Err(
                format!(
                    "{} does not fit into memory. Max size: {}, Min size: {}",
                    data, 
                    MAX_POS_DATA, 
                    MAX_NEG_DATA)
            );
        }

        Ok(DataWord {
            data: data,
            is_negative: data & 0x8000 == 1
        })
    }

    pub fn set_data(&mut self, data: i32) -> bool {
        // Check if the data is between the values 
        // of 2^30 and (2^30)-1
        if data > MAX_POS_DATA || data < MAX_NEG_DATA {
            return false;
        }

        // we have valid data, so 
        // we can store it
        self.data = data;

        // check if it's negative,
        // bit test the MSB and set the is_negative
        // flag
        self.is_negative = data & 0x8000 == 1;
        return true;
    }

    pub fn get_data(self) -> i32 {
        self.data
    }
}