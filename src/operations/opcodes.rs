use crate::common::error::{Error};
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Bring, Add, Subtract, Hold, Clear,              // Arthimetic Opcodes
    Extract, Divide, MultTopHalf, MultLowHalf,      // 
    StoreAddress, ReturnAddress, UncondTransfer,    // Logical Opcodes
    Test, Stop,                                     //
    Print, Input                                    // I/O Opcodes
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Stop),
            1 => Ok(Self::Bring),
            2 => Ok(Self::StoreAddress),
            3 => Ok(Self::ReturnAddress),
            4 => Ok(Self::Input),
            5 => Ok(Self::Divide),
            6 => Ok(Self::MultLowHalf),
            7 => Ok(Self::MultTopHalf),
            8 => Ok(Self::Print),
            9 => Ok(Self::Extract),
            10 => Ok(Self::UncondTransfer),
            11 => Ok(Self::Test),
            12 => Ok(Self::Hold),
            13 => Ok(Self::Clear),
            14 => Ok(Self::Add),
            15 => Ok(Self::Subtract),
            _ => Err(Error::OpcodeFromU8Failed),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::Stop => 0,
            Opcode::Bring => 1,
            Opcode::StoreAddress => 2,
            Opcode::ReturnAddress => 3,
            Opcode::Input => 4,
            Opcode::Divide => 5,
            Opcode::MultLowHalf => 6,
            Opcode::MultTopHalf => 7,
            Opcode::Print => 8,
            Opcode::Extract => 9,
            Opcode::UncondTransfer => 10,
            Opcode::Test => 11,
            Opcode::Hold => 12,
            Opcode::Clear => 13,
            Opcode::Add => 14,
            Opcode::Subtract => 15,
        }
    }
}

impl TryFrom<char> for Opcode {
    type Error = Error;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        let upper = val.to_uppercase().next().unwrap();

        match upper {
            'B' => Ok(Self::Bring),
            'A' => Ok(Self::Add),
            'S' => Ok(Self::Subtract),
            'H' => Ok(Self::Hold),
            'C' => Ok(Self::Clear),
            'E' => Ok(Self::Extract),
            'D' => Ok(Self::Divide),
            'M' => Ok(Self::MultTopHalf),
            'N' => Ok(Self::MultLowHalf),
            'Y' => Ok(Self::StoreAddress),
            'R' => Ok(Self::ReturnAddress),
            'U' => Ok(Self::UncondTransfer),
            'T' => Ok(Self::Test),
            'Z' => Ok(Self::Stop),
            'P' => Ok(Self::Print),
            'I' => Ok(Self::Input),
            _ => Err(Error::OpcodeFromCharFailed),
        }
    }
}

impl Into<char> for Opcode {
    fn into(self) -> char {
        match self {
            Opcode::Bring => 'B',
            Opcode::Add => 'A',
            Opcode::Subtract => 'S',
            Opcode::Hold => 'H',
            Opcode::Clear => 'C',
            Opcode::Extract => 'E',
            Opcode::Divide => 'D',
            Opcode::MultTopHalf => 'M',
            Opcode::MultLowHalf => 'N',
            Opcode::StoreAddress => 'Y',
            Opcode::ReturnAddress => 'R',
            Opcode::UncondTransfer => 'U',
            Opcode::Test => 'T',
            Opcode::Stop => 'Z',
            Opcode::Print => 'P',
            Opcode::Input => 'I'
        }
    }
}