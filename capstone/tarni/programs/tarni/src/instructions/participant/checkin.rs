use anchor_lang::prelude::*;
use crate::{
    error::TarniError,
    events::PlayerCheckedIn,
    state::{Match, MatchState, Participant, Tournament},
    utils::time::now_ts,
};

#[derive(Accounts)]
pub struct Checkin<'info> {
    #[account(
        mut,
        constraint = tournament.can_checkin() @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        has_one = tournament @ TarniError::InvalidMatch,
        constraint = match_account.can_checkin(now_ts()) @ TarniError::InvalidMatchState,
    )]
    pub match_account: Account<'info, Match>,

    #[account(
        mut,
        has_one = player @ TarniError::InvalidParticipant,
        has_one = tournament @ TarniError::InvalidParticipant,
    )]
    pub participant: Account<'info, Participant>,

    pub player: Signer<'info>,
}

impl<'info> Checkin<'info> {
    pub fn checkin(&mut self) -> Result<()> {
        let now = now_ts();
        let match_account = &mut self.match_account;
        let participant = &mut self.participant;

        require!(
            match_account.can_checkin(now_ts()),
            TarniError::CheckinNotOpen
        );

        require!(!participant.checked_in, TarniError::AlreadyCheckedIn);

        participant.checked_in = true;
        participant.checkin_time = now;

        match_account.checked_in = match_account
            .checked_in
            .checked_add(1)
            .ok_or(TarniError::MathError)?;

        if match_account.all_checked_in() {
            match_account.state = MatchState::CheckedIn;
        }

        emit!(PlayerCheckedIn {
            tournament: self.tournament.key(),
            player: self.player.key(),
            checkin_timestamp: now,
        });

        Ok(())
    }
}
