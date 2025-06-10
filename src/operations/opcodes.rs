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
pub struct OpcodeFromCharError {
    pub input: char,
    pub message: String,
}

impl fmt::Display for OpcodeFromCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid opcode as char '{}': {}", self.input, self.message)
    }
}

impl TryFrom<char> for Opcode {
    type Error = OpcodeFromCharError;

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
            _ => Err(OpcodeFromCharError { 
                input: val, 
                message: "not a valid opcode".into()})
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