// From the LGP-30 Manual
//  There are 64 tracks (0-63) and each track 
//  is comprised of 64 sectors, giving 4096 of storage space
//  on the memory drum
pub const MAX_TRACK: u8 = 63;
pub const MAX_SECTOR: u8 = 63;
pub const MAX_MEMORY: u16 = ((MAX_TRACK + 1) as u16) * ((MAX_SECTOR + 1) as u16);

// The LGP had a funky memory architecture.  Each "word" was 32-bit, but the 
// last bit was a "spacer bit" and was unused, this the LGP-30 had a memory 
// width of 31 bits, with the MSB as the sign bit.
pub const MAX_POS_DATA: i32 = 2^31 - 1;
pub const MAX_NEG_DATA: i32 = (2^31) * -1;