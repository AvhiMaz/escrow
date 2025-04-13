pub mod make;
pub mod take;

pub use make::*;
pub use take::*;

use pinocchio::program_error::ProgramError;

#[repr(C)]
pub enum EscrowInstructions {
    Make,
    Take,
}

impl TryFrom<&u8> for EscrowInstructions {
    type Error = ProgramError;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowInstructions::Make),
            1 => Ok(EscrowInstructions::Take),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
