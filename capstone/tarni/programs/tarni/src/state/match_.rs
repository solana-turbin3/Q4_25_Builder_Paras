use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Match {
    pub tournament: Pubkey,
    pub match_id: u64,
    #[max_len(64)]
    pub participants: Vec<Pubkey>,
    pub state: MatchState,
    pub created_at: i64,
    pub starts_at: i64,
    pub cin_deadline: i64,
    pub participant_limit: u8,
    pub checked_in: u8,
    pub bump: u8,
}


impl Match {
    pub fn all_checked_in(&self) -> bool {
        self.checked_in as u8 == self.participant_limit
    }

    pub fn checkin_limit_reached(&self, current_time: i64) -> bool {
        current_time >= self.cin_deadline
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum MatchState {
    Pending,
    CheckedIn,
    InProgress,
    Complete,
}