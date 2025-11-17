use anchor_lang::prelude::*;

#[event]
pub struct TournamentCreated {
    pub tournament: Pubkey,
    pub organizer: Pubkey,
    pub backend: Pubkey,
    pub start_time: i64,
    pub entry_fee: u64,
}

#[event]
pub struct PlayerRegistered {
    pub tournament: Pubkey,
    pub player: Pubkey,
    pub slot: u8, // Participant slot number
}

#[event]
pub struct RegistrationClosed {
    pub tournament: Pubkey,
    pub closed_at: i64,
}

#[event]
pub struct MatchLaunched {
    pub tournament: Pubkey,
    pub match_id_hash: [u8; 32],
    pub participants: Vec<Pubkey>,
    pub starts_at: i64,
}

#[event]
pub struct MatchStarted {
    pub tournament: Pubkey,
    pub match_: Pubkey,
    pub match_id: u64,
    pub started_at: i64,
}

#[event]
pub struct PlayerCheckedIn {
    pub match_: Pubkey,
    pub tournament: Pubkey,
    pub player: Pubkey,
    pub checkin_timestamp: i64,
}

#[event]
pub struct PlayerDisqualified {
    pub tournament: Pubkey,
    pub player: Pubkey,
    pub reason: String,
}

#[event]
pub struct ResultsSubmitted {
    pub tournament: Pubkey,
    pub match_id: u64,
    pub winners: Vec<Pubkey>,
    pub match_: Pubkey,
}

#[event]
pub struct PrizeClaimed {
    pub tournament: Pubkey,
    pub player: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RefundClaimed {
    pub tournament: Pubkey,
    pub player: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TournamentCancelled {
    pub tournament: Pubkey,
    pub cancelled_at: i64,
}
