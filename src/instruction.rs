use crate::errors::EscrowErrors::InvalidInstruction;
use solana_program::program_error::ProgramError;
use std::convert::TryInto;
pub enum EscrowInstruction {
    InitEscrow { amount: u64 },
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<u64, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    }
}
