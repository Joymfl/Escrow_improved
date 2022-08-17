use crate::errors::EscrowErrors::InvalidInstruction;
use solana_program::program_error::ProgramError;
use std::convert::TryInto;
pub enum EscrowInstruction {
    /// [0] Sender Authority and Signer (Alice)
    /// [1] Temp Token Account by Sender to be transferred over to EscrowAccount
    /// [2] Sender's Token Account to receive token from trade
    /// [3] Escrow Account
    /// [4] Rent Sysvar
    /// [5] Token Program
    InitEscrow { amount: u64 },

    /// Accepts a trade
    /// 0. [signer] The account of the person taking the trade
    /// 1. [writable] The takers token account for the token they send
    /// 2. [writable] The taker's token account for the token they will receive if successful
    /// 3. [writable] The PDA's temp token account to get tokens from
    /// 4. [writable] The initializer's main account to send rent fees to
    /// 5. [writable] The initializer's token account to receive tokens from the trade
    /// 6. [writable] The escrow account holding information about the trade
    /// 7. [] The token program
    /// 8. [] The PDA account
    Exchange {
        /// The amount the taker expects to be paid in the other token
        amount: u64,
    },
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
