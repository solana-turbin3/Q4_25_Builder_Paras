use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Participant {
    pub player: Pubkey,
    pub tournament: Pubkey,
    pub game_account: Pubkey,
    pub registered_at: i64,
    pub entry_paid: u64,
    pub checked_in: bool,
    pub checkin_time: i64,
    pub disqualified: bool,
    #[max_len(32)]
    pub dq_reason: String,
    pub prize_amount: u64,
    pub claimed: bool,
    pub refunded: bool,
    pub refund_amount: u64,
    pub bump: u8,
}

impl Participant {
    pub fn can_checkin(&self, current_time: i64, checkin_opens: i64, match_start_time: i64) -> bool {
        !self.checked_in && !self.disqualified && current_time >= checkin_opens && current_time < match_start_time
    }

    pub fn can_claim_prize(&self) -> bool {
        !self.claimed && self.prize_amount > 0 && !self.disqualified
    }

    pub fn can_claim_refund(&self) -> bool {
        !self.refunded && self.refund_amount > 0 && !self.disqualified
    }
}