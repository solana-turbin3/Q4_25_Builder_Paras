use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Match {
    pub tournament: Pubkey,
    pub match_id_hash: [u8; 32],
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
    pub fn can_checkin(&self, current_time: i64) -> bool {
        matches!(self.state, MatchState::Pending)
            && current_time >= self.starts_at - 600 // Check-in opens 10 minutes before start
            && current_time <= self.cin_deadline
    }

    pub fn all_checked_in(&self) -> bool {
        self.checked_in as u8 == self.participant_limit
    }

    pub fn checkin_limit_reached(&self, current_time: i64) -> bool {
        current_time >= self.cin_deadline
    }

    pub fn can_start(&self) -> bool {
        matches!(self.state,MatchState::CheckedIn)
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum MatchState {
    Pending,
    CheckedIn,
    InProgress,
    Complete,
}