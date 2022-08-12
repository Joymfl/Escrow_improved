use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EscrowErrors {
    #[error("Instruction Invalid")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<EscrowErrors> for ProgramError {
    fn from(e: EscrowErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
