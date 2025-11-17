use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Result {
    pub match_: Pubkey,
    pub tournament: Pubkey,
    #[max_len(120)]
    pub ipfs_cid: String,
    pub submitted_at: i64,
    pub signature: [u8; 64],
    #[max_len(64)]
    pub placements: Vec<PlayerResult>,
    pub verified: bool,
    pub distributed: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct PlayerResult {
    #[max_len(44)]  // Pubkey as base58 string
    pub player: String,
    pub placement: u8,
    pub kills: u8,
}


