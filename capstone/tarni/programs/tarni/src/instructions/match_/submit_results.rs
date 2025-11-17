use anchor_lang::prelude::*;

use crate::{
    constants::*,
    error::TarniError,
    events::ResultsSubmitted,
    state::{Match, MatchState, Participant, PrizeSplit, Result as MatchResult, Tournament, TournamentState, PlayerResult},
    utils::prize::calculate_prizes,
    utils::time::now_ts,
};

#[derive(Accounts)]
pub struct SubmitResults<'info> {

    #[account(
        mut,
        has_one = backend @ TarniError::NotBackendAuthority,
        constraint = tournament.state == TournamentState::InProgress @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        has_one = tournament @ TarniError::InvalidMatch,
        constraint = match_account.state == MatchState::InProgress @ TarniError::InvalidMatchState,
    )]
    pub match_account: Account<'info, Match>,

    #[account(
        init,
        payer = backend,
        space = 8 + MatchResult::INIT_SPACE,
        seeds = [RESULT_SEED, match_account.key().as_ref()],
        bump,
    )]
    pub result: Account<'info, MatchResult>,

    #[account(mut)]
    pub backend: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> SubmitResults<'info> {
    pub fn submit_results(
        &mut self,
        ipfs_cid: String,
        signature: [u8; 64],
        placements: Vec<PlayerResult>,
        bump: u8,
    ) -> Result<()> {
        let tournament = &mut self.tournament;
        let match_account = &mut self.match_account;
        let result = &mut self.result;

        require!(
            placements.len() <= MAX_WINNERS,
            TarniError::InvalidArgument
        );

        require!(
            placements.len() >= tournament.prize_split.winner_count() as usize,
            TarniError::InvalidArgument
        );

        result.set_inner(MatchResult {
            match_: match_account.key(),
            tournament: tournament.key(),
            ipfs_cid,
            submitted_at: now_ts(),
            signature,
            placements: placements.clone(),
            verified: true, 
            distributed: false,
            bump,
        });

        match_account.state = MatchState::Complete;
        tournament.state = TournamentState::Complete;


        let winners: Vec<Pubkey> = placements
            .iter()
            .map(|p| p.player.parse().unwrap_or_default())
            .collect();
        emit!(ResultsSubmitted {
            tournament: tournament.key(),
            match_id: match_account.match_id,
            winners,
            match_: match_account.key(),
        });

        Ok(())
    }
}