use anchor_lang::prelude::*;

use crate::{
    error::TarniError,
    events::TournamentCancelled,
    state::{Tournament, TournamentState},
    utils::time::now_ts,
};

#[derive(Accounts)]
pub struct CancelTournament<'info> {
    #[account(
        mut,
        has_one = organizer @ TarniError::NotOrganizer,
        constraint = tournament.can_cancel() @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    pub organizer: Signer<'info>,
}

impl<'info> CancelTournament<'info> {
    pub fn cancel(&mut self) -> Result<()>{
        let tournament = &mut self.tournament;

        tournament.state = TournamentState::Cancelled;

        emit!(TournamentCancelled {
            tournament: tournament.key(),
            cancelled_at: now_ts(),
        });
        Ok(())
}
}