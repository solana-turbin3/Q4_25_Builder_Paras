use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub tournament: Pubkey,
    pub total_pool: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 8 + 32 + 8 + 1; // discriminator + pubkey + u64 + u8
}