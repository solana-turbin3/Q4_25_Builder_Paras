use anchor_lang::prelude::*;

use crate::{
    error::TarniError,
    events::PlayerDisqualified,
    state::{Match, Participant, Tournament},
    utils::time::now_ts,
};

#[derive(Accounts)]
pub struct AutoDisqualify<'info> {
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        has_one = tournament @ TarniError::InvalidMatch,
        constraint = match_account.can_checkin(now_ts())@ TarniError::InvalidMatchState,
    )]
    pub match_account: Account<'info, Match>,

    #[account(
        mut,
        has_one = tournament @ TarniError::InvalidParticipant,
        constraint = !participant.checked_in @ TarniError::AlreadyCheckedIn,
        constraint = !participant.disqualified @ TarniError::AlreadyDQ,
    )]
    pub participant: Account<'info, Participant>,
}

impl<'info> AutoDisqualify<'info> {
    pub fn auto_disqualify(&mut self) -> Result<()> {
        let _now = now_ts();

        let participant = &mut self.participant;
        participant.disqualified = true;
        participant.dq_reason = "No-show: Failed to check in".to_string();

        participant.refund_amount = participant.entry_paid;

        emit!(PlayerDisqualified {
            tournament: self.tournament.key(),
            player: participant.player,
            reason: participant.dq_reason.clone(),
        });

        Ok(())
    }
}
