use anchor_lang::prelude::*;
use solana_program::hash::hash;


use crate::{
    error::TarniError,
    events::MatchStarted,
    state::{Match, MatchState, Tournament},
    utils::time::now_ts,
};

#[derive(Accounts)]
pub struct StartMatch<'info> {
    #[account(
        mut,
        has_one = backend @ TarniError::NotBackendAuthority,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        has_one = tournament @ TarniError::InvalidMatch,
        constraint = match_account.can_start() @ TarniError::InvalidMatchState,
    )]
    pub match_account: Account<'info, Match>,

    /// CHECK: Backend authority keypair
    pub backend: Signer<'info>,
}

impl<'info> StartMatch<'info> {
    pub fn start_match(&mut self, match_id: u64) -> Result<()> {
        let match_account = &mut self.match_account;
        let now = now_ts();

        let provided_hash = hash(&match_id.to_le_bytes());
        require!(
            provided_hash.to_bytes() == match_account.match_id_hash,
            TarniError::InvalidArgument
        );

        match_account.match_id = match_id;

        match_account.state = MatchState::InProgress;
        match_account.starts_at = now;

        emit!(MatchStarted {
            tournament: self.tournament.key(),
            match_: match_account.key(),
            match_id: match_account.match_id,
            started_at: now,
        });

        Ok(())
    }
}
