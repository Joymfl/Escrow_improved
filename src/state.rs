use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
pub struct Escrow {
    pub sender_account: Pubkey,
    pub taker_account: Pubkey,
}
