use std::fmt;

#[derive(Debug)]
pub enum Opcode {
    Bring, Add, Subtract, Hold, Clear,              // Arthimetic Opcodes
    Extract, Divide, MultTopHalf, MultLowHalf,      // 
    StoreAddress, ReturnAddress, UncondTransfer,    // Logical Opcodes
    Test, Stop,                                     //
    Print, Input                                    // I/O Opcodes
}

#[derive(Debug)]
pub enum OpcodeError {
    ToU8Failed,
    FromU8Failed,
    ToCharFailed,
    FromCharFailed,
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Self::Bring),
            12 => Ok(Self::Hold),
            13 => Ok(Self::Clear),
            2 => Ok(Self::StoreAddress),
            10 => Ok(Self::UncondTransfer),
            3 => Ok(Self::ReturnAddress),
            11 => Ok(Self::Test),
            0 => Ok(Self::Stop),
            8 => Ok(Self::Print),
            4 => Ok(Self::Input),
            14 => Ok(Self::Add),
            15 => Ok(Self::Subtract),
            7 => Ok(Self::MultTopHalf),
            6 => Ok(Self::MultLowHalf),
            5 => Ok(Self::Divide),
            9 => Ok(Self::Extract),
            _ => Err(OpcodeError::FromU8Failed),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::Bring => 1,
            Opcode::Hold => 12,
            Opcode::Clear => 13,
            Opcode::StoreAddress => 2,
            Opcode::UncondTransfer => 10,
            Opcode::ReturnAddress => 3,
            Opcode::Test => 11,
            Opcode::Stop => 0,
            Opcode::Print => 8,
            Opcode::Input => 4,
            Opcode::Add => 14,
            Opcode::Subtract => 15,
            Opcode::MultTopHalf => 7,
            Opcode::MultLowHalf => 6,
            Opcode::Divide => 5,
            Opcode::Extract => 9,
        }
    }
}

impl TryFrom<char> for Opcode {
    type Error = OpcodeError;

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
            _ => Err(OpcodeError::FromCharFailed),
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