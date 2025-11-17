use anchor_lang::prelude::*;

use crate::{
    error::TarniError,
    events::TournamentCancelled,
    state::{Participant, Tournament, TournamentState},
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

    #[account(mut,
        has_one = tournament @ TarniError::InvalidParticipant,
    )]
    pub participant: Account<'info, Participant>,

    pub organizer: Signer<'info>,
}

impl<'info> CancelTournament<'info> {
    pub fn cancel(&mut self) -> Result<()>{
        let tournament = &mut self.tournament;

        require!(
            tournament.can_cancel(),
            TarniError::InvalidTournamentState
        );

        tournament.state = TournamentState::Cancelled;

        self.participant.refund_amount = self.participant.entry_paid;
        self.participant.refunded = false;

        emit!(TournamentCancelled {
            tournament: tournament.key(),
            cancelled_at: now_ts(),
        });
        Ok(())
}
}