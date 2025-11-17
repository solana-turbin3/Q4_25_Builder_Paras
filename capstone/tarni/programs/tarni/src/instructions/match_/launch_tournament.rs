use anchor_lang::prelude::*;

use crate::{
    constants::MATCH_SEED,
    error::TarniError,
    events::MatchLaunched, 
    state::{Match, MatchState, Tournament, TournamentState},
    utils::time::now_ts,
};

#[derive(Accounts)]
pub struct LaunchTournament<'info> {
    #[account(
        mut,
        has_one = organizer @ TarniError::NotOrganizer, 
        constraint = tournament.can_launch() @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        init,
        payer = organizer,
        space = 8 + Match::INIT_SPACE,
        seeds = [MATCH_SEED, tournament.key().as_ref()],
        bump,
    )]
    pub match_account: Account<'info, Match>,
    
    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> LaunchTournament<'info> {
    pub fn launch_tournament(&mut self, match_id_hash: [u8; 32], bump: u8) -> Result<()> {
        let tournament = &mut self.tournament;
        let match_account = &mut self.match_account;

        match_account.set_inner(Match {
            tournament: tournament.key(),
            match_id_hash,
            match_id: 0,              
            participants: vec![],       
            state: MatchState::Pending,
            created_at: now_ts(),
            starts_at: tournament.starts_at,
            cin_deadline: tournament.starts_at + 300, // 5 minutes after match start for check-in
            participant_limit: tournament.current_participants,
            checked_in: 0,
            bump,
        });

        tournament.state = TournamentState::InProgress;

        emit!(MatchLaunched {
            tournament: tournament.key(),
            match_id_hash, 
            participants: vec![], 
            starts_at: match_account.starts_at,
        });

        Ok(())
    }
}
