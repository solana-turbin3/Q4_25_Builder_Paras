use anchor_lang::prelude::*;

use crate::{
    error::TarniError,
    state::{GameType, PrizeSplit, Tournament, TournamentState},
    events::{TournamentCreated},
    utils::time::now_ts,
    constants::*,
};

#[derive(Accounts)]
#[instruction(tournament_id: u64)]
pub struct CreateTournament<'info> {
    #[account(
        init,
        payer = organizer,
        space = 8 + Tournament::INIT_SPACE,
        seeds = [TOURNAMENT_SEED, &tournament_id.to_le_bytes()],
        bump,
    )]
    pub tournament: Account<'info, Tournament>,
    
    /// CHECK: System PDA that will hold tournament funds
    #[account(
        mut,
        seeds = [ESCROW_SEED, tournament.key().as_ref()],
        bump,
    )]
    pub escrow: AccountInfo<'info>,

    #[account(mut)]
    pub organizer: Signer<'info>,

    /// CHECK: This is the backend authority account that's validated by the program logic
    pub backend: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> CreateTournament<'info> {
    pub fn process(
        &mut self,
        game_type: GameType,
        tournament_id: u64,
        entry_fee: u64,
        max_participants: u8,
        prize_split: PrizeSplit,
        rules: String,
        starts_at: i64,
        escrow_bump: u8,
    ) -> Result<()> {

    require!(
        entry_fee >= MIN_ENTRY_FEE,
        TarniError::EntryFeeTooLow
    );
    require!(
        max_participants >= MIN_PARTICIPANTS && max_participants <= MAX_PARTICIPANTS,
        TarniError::InvalidArgument
    );
    require!(
        prize_split.is_valid(),
        TarniError::InvalidPrizeSplit
    );
    require!(
        rules.len() <= MAX_RULES_LENGTH,
        TarniError::InvalidArgument
    );
    require!(
        starts_at > now_ts(),
        TarniError::InvalidArgument
    );
        let checkin_opens = starts_at
            .checked_sub(CHECKIN_WINDOW_SECONDS)
            .ok_or(TarniError::MathError)?;
        
        let checkin_closes = starts_at; // Check-in closes when tournament starts
        
        self.tournament.set_inner(Tournament {
            tournament_id,
            organizer: self.organizer.key(),
            backend: self.backend.key(),
            game_type,
            entry_fee,
            max_participants,
            current_participants: 0,
            prize_split,
            rules,
            state: TournamentState::Open,
            created_at: now_ts(),
            starts_at,
            checkin_opens,
            checkin_closes, // this is very doubtful
            started_at: 0, // Will be set when tournament actually starts
            escrow: self.escrow.key(),
            prize_pool: 0, // Will accumulate as participants register
            escrow_bump: escrow_bump,
            bump: self.tournament.bump,
        });

        emit!(TournamentCreated {
            tournament: self.tournament.key(),
            organizer: self.organizer.key(),
            backend: self.backend.key(),
            start_time: self.tournament.starts_at,
            entry_fee,
        });

        Ok(())
    }
}