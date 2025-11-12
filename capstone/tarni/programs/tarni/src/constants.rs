pub const TOURNAMENT_SEED: &[u8] = b"tournament";
pub const PARTICIPANT_SEED: &[u8] = b"participant";
pub const MATCH_SEED: &[u8] = b"match";
pub const RESULT_SEED: &[u8] = b"result";
pub const ESCROW_SEED: &[u8] = b"escrow";

pub const MAX_PARTICIPANTS: u8 = 64;
pub const MIN_PARTICIPANTS: u8 = 2;

pub const MIN_ENTRY_FEE: u64 = 50_000_000;

pub const MAX_RULES_LENGTH: usize = 200;
pub const MAX_CID_LENGTH: usize = 120;
pub const MAX_MATCH_DATA_LENGTH: usize = 200;
pub const MAX_DQ_REASON_LENGTH: usize = 32;

pub const CHECKIN_WINDOW_SECONDS: i64 = 6 * 60 * 60; // 6 hours

pub const MAX_WINNERS: usize = 5;
