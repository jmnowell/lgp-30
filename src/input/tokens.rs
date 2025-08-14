#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ASMToken {
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
    EndOfLine,
}

impl TryFrom<char> for ASMToken {
    type Error = crate::common::error::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'B' => Ok(ASMToken::Bring),
            'A' => Ok(ASMToken::Add),
            'S' => Ok(ASMToken::Subtract),
            'H' => Ok(ASMToken::Hold),
            'C' => Ok(ASMToken::Clear),
            'E' => Ok(ASMToken::Extract),
            'D' => Ok(ASMToken::Divide),
            'T' => Ok(ASMToken::Test),
            'U' => Ok(ASMToken::UncondTransfer),
            'M' => Ok(ASMToken::MultTopHalf), // TODO: M is ambiguous
            'N' => Ok(ASMToken::MultLowHalf),
            'R' => Ok(ASMToken::ReturnAddress),
            'Y' => Ok(ASMToken::StoreAddress),
            'P' => Ok(ASMToken::Print),
            'I' => Ok(ASMToken::Input),
            'Z' => Ok(ASMToken::Stop),
            '0' => Ok(ASMToken::Zero),
            '1' => Ok(ASMToken::One),
            '2' => Ok(ASMToken::Two),
            '3' => Ok(ASMToken::Three),
            '4' => Ok(ASMToken::Four),
            '5' => Ok(ASMToken::Five),
            '6' => Ok(ASMToken::Six),
            '7' => Ok(ASMToken::Seven),
            '8' => Ok(ASMToken::Eight),
            '9' => Ok(ASMToken::Nine),
            '\n' => Ok(ASMToken::EndOfLine),
            _ => Err(crate::common::error::Error::OpcodeFromCharFailed),
        }
    }
}
