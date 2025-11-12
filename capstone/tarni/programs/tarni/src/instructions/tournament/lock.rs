use anchor_lang::prelude::*;

use crate::{
    error::TarniError,
    state::{Tournament, TournamentState},
    utils::time::now_ts,
    events::RegistrationClosed,
};

#[derive(Accounts)]
pub struct LockRegistration<'info> {
    #[account(
        mut,
        has_one = organizer @ TarniError::NotOrganizer,
        constraint = tournament.is_mut() @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    pub organizer: Signer<'info>,
}

impl<'info> LockRegistration<'info> {
    pub fn process(&mut self) -> Result<()> {
        let tournament = &mut self.tournament;

        require!(
            tournament.current_participants >= crate::constants::MIN_PARTICIPANTS,
            TarniError::NotEnoughParticipants
        );

        tournament.state = TournamentState::Locked;

        emit!(RegistrationClosed {
            tournament: tournament.key(),
            closed_at: now_ts(),
        });

        Ok(())
    }
}