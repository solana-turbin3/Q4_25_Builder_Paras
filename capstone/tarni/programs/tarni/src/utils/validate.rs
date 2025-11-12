use anchor_lang::prelude::*;
use crate::state::{Tournament, Participant};
use crate::error::TarniError;

pub fn assert_registration_open(tournament: &Tournament) -> Result<()> {
    require!(tournament.can_register(), TarniError::RegistrationClosed);
    Ok(())
}

pub fn assert_participant_not_disqualified(participant: &Participant) -> Result<()> {
    require!(!participant.disqualified, TarniError::AlreadyCheckedInOrDQ);
    Ok(())
}
