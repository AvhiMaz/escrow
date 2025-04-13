pub mod make;

pub use make::*;
use pinocchio::program_error::ProgramError;

#[repr(C)]
pub enum EscrowInstructions {
    Make,
}

impl TryFrom<&u8> for EscrowInstructions {
    type Error = ProgramError;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowInstructions::Make),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
