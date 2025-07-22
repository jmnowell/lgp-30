#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ASMTokens {
    Bring, 
    Add, 
    Subtract, 
    Hold, 
    Clear,          
    Extract, 
    Divide, 
    MultTopHalf, 
    MultLowHalf,  
    StoreAddress, 
    ReturnAddress, 
    UncondTransfer,
    Test, 
    Stop,                                 
    Print, 
    Input,                               
    One, 
    Two, 
    Three, 
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
    Zero,
}

impl TryFrom<char> for ASMTokens {
    type Error = crate::common::error::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'B' => Ok(ASMTokens::Bring),
            'A' => Ok(ASMTokens::Add),
            'S' => Ok(ASMTokens::Subtract),
            'H' => Ok(ASMTokens::Hold),
            'C' => Ok(ASMTokens::Clear),
            'E' => Ok(ASMTokens::Extract),
            'D' => Ok(ASMTokens::Divide),
            'T' => Ok(ASMTokens::Test),
            'U' => Ok(ASMTokens::UncondTransfer),
            'M' => Ok(ASMTokens::MultTopHalf), // TODO: M is ambiguous
            'N' => Ok(ASMTokens::MultLowHalf),
            'R' => Ok(ASMTokens::ReturnAddress),
            'Y' => Ok(ASMTokens::StoreAddress),
            'P' => Ok(ASMTokens::Print),
            'I' => Ok(ASMTokens::Input),
            'Z' => Ok(ASMTokens::Stop),
            '0' => Ok(ASMTokens::Zero),
            '1' => Ok(ASMTokens::One),
            '2' => Ok(ASMTokens::Two),
            '3' => Ok(ASMTokens::Three),
            '4' => Ok(ASMTokens::Four),
            '5' => Ok(ASMTokens::Five),
            '6' => Ok(ASMTokens::Six),
            '7' => Ok(ASMTokens::Seven),
            '8' => Ok(ASMTokens::Eight),
            '9' => Ok(ASMTokens::Nine),
            _ => Err(crate::common::error::Error::OpcodeFromCharFailed)
        }
    }
}

pub struct LexerLine {
    pub line_number: usize,
    pub tokens: Vec<ASMTokens>,
}