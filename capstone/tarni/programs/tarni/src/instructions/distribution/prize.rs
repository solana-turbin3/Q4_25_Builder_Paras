use anchor_lang::prelude::*;

use crate::{
    error::TarniError,
    state::{Match, Participant, Result as MatchResult, Tournament, TournamentState},
    utils::prize::calculate_prizes,
};

#[derive(Accounts)]
pub struct Prizes<'info> {
    #[account(
        mut,
        constraint = tournament.state == TournamentState::Complete @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        has_one = tournament @ TarniError::InvalidMatch,
    )]
    pub match_account: Account<'info, Match>,

    #[account(
        mut,
        constraint = result.match_ == match_account.key() @ TarniError::InvalidResult,
        constraint = !result.distributed @ TarniError::AlreadyDistributed,
    )]
    pub result: Account<'info, MatchResult>,

    #[account(
        mut,
        has_one = tournament @ TarniError::InvalidParticipant,
    )]
    pub participant: Account<'info, Participant>,
}

impl<'info> Prizes<'info> {
    pub fn calculate_prizes(&mut self) -> Result<()> {
        let tournament = &self.tournament;
        let result = &self.result;
        let participant = &mut self.participant;

        let participant_key_str = participant.player.to_string();
        let placement = result
            .placements
            .iter()
            .find(|p| p.player == participant_key_str)
            .ok_or(TarniError::PlayerNotInResults)?;

        // Calculate prize amounts based on placement
        let prize_amounts = calculate_prizes(
            &tournament.prize_split,
            tournament.prize_pool,
        );


        let placement_index = (placement.placement as usize).saturating_sub(1);
        
        require!(
            placement_index < prize_amounts.len(),
            TarniError::InvalidPlacement
        );

        let prize = prize_amounts[placement_index];

        // Set prize amount for participant
        participant.prize_amount = prize;

        Ok(())
    }
}
