use anchor_lang::prelude::*;

declare_id!("6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c");

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

#[program]
pub mod tarni {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_tournament(
        ctx: Context<CreateTournament>,
        tournament_id: u64,
        game_type: GameType,
        entry_fee: u64,
        max_participants: u8,
        prize_split: PrizeSplit,
        rules: String,
        starts_at: i64,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.process(
            game_type,
            tournament_id,
            entry_fee,
            max_participants,
            prize_split,
            rules,
            starts_at,
        )
    }

    pub fn lock_registration(
        ctx: Context<LockRegistration>
    ) -> anchor_lang::Result<()> {
        ctx.accounts.process()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
