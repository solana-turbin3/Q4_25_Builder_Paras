use anchor_lang::prelude::*;

use crate::instructions::tournament;

#[account]
#[derive(InitSpace)]
pub struct Tournament {
    pub tournament_id: u64,
    pub organizer: Pubkey,
    pub backend: Pubkey,
    pub game_type: GameType,
    pub entry_fee: u64,
    pub max_participants: u8,
    pub current_participants: u8,
    pub prize_split: PrizeSplit,
    #[max_len(200)]
    pub rules: String,
    pub state: TournamentState,
    pub created_at: i64,
    pub starts_at: i64,
    pub checkin_opens: i64,
    pub checkin_closes: i64,
    pub started_at: i64,
    pub escrow: Pubkey,
    pub prize_pool: u64,
    pub bump: u8,
}

impl Tournament {

        pub fn can_checkin(&self) -> bool {
        matches!(self.state, TournamentState::InProgress)
        }
        // tournament start state is doubtful
        pub fn can_register(&self) -> bool {
        matches!(self.state, TournamentState::Open) && self.current_participants < self.max_participants
        }
        // 
        pub fn can_cancel(&self) -> bool {
        matches!(self.state, TournamentState::Open | TournamentState::Locked)
        }
        // we should handle it in match
        // pub fn can_checkin(&self, current_time: i64) -> bool {
        
        pub fn is_mut(&self) -> bool {
            if matches!(self.state, TournamentState::Open) && matches!(self.current_participants, 0) {
                true
            } else {
                false
            }
        
        }

        pub fn can_launch(&self) -> bool {
        matches!(self.state, TournamentState::Locked)
        }

        pub fn can_distribute_prizes(&self) -> bool {
        matches!(self.state, TournamentState::Complete)
        }
   }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
   pub enum GameType {
    SingleMatch,
   }
   
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
   pub enum TournamentState {
    Open,
    Locked,
    Cancelled,
    InProgress,
    Complete,
    Distributed,
   }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
   pub enum PrizeSplit {
    WinnerTakesAll,
    TopThree { first: u8, second: u8, third: u8 },
    TopFive { first: u8, second: u8, third: u8, fourth: u8, fifth: u8 },
   }

   impl PrizeSplit {
    pub const LEN: usize = 1 + 5; // 1 byte for variant + up to 5 bytes for percentages

    pub fn is_valid(&self) -> bool {
        match self {
            PrizeSplit::WinnerTakesAll => true,
            PrizeSplit::TopThree { first, second, third } => {
                *first as u16 + *second as u16 + *third as u16 == 100
            }
            PrizeSplit::TopFive { first, second, third, fourth, fifth } => {
                *first as u16 + *second as u16 + *third as u16 + *fourth as u16 + *fifth as u16 == 100
            }
        }
    }
    pub fn winner_count(&self) -> u8 {
        match self {
            PrizeSplit::WinnerTakesAll => 1,
            PrizeSplit::TopThree { .. } => 3,
            PrizeSplit::TopFive { .. } => 5,
        }
    }
   }