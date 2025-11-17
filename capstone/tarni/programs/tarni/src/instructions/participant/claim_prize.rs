use anchor_lang::prelude::*;
use anchor_lang::{system_program, system_program::{transfer, Transfer}};

use crate::{
    constants::*,
    error::TarniError,
    state::{Participant, Tournament},
    events::PrizeClaimed,
};

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(
        mut,
        constraint = tournament.escrow == escrow.key() @ TarniError::InvalidEscrowAccount,
        constraint = tournament.can_distribute_prizes() @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        has_one = player @ TarniError::InvalidParticipant,
        has_one = tournament @ TarniError::InvalidParticipant,
        constraint = participant.can_claim_prize() @ TarniError::NotEligibleForPrize,
    )]
    pub participant: Account<'info, Participant>,

    #[account(mut)]
    pub player: Signer<'info>,
    
    /// CHECK: System PDA that holds tournament funds
    #[account(
        mut,
        seeds = [ESCROW_SEED, tournament.key().as_ref()],
        bump,
        constraint = escrow.owner == &system_program::ID @ TarniError::InvalidEscrowAccount,
    )]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimPrize<'info> {
    pub fn claim_prize(&mut self, escrow_bump: u8) -> Result<()> {
        let tournament = &mut self.tournament;
        let participant = &mut self.participant;

        let prize_amount = participant.prize_amount;

        let tournament_key = tournament.key();
        let signer_seeds: &[&[&[u8]]] = &[
            &[
                ESCROW_SEED,
                tournament_key.as_ref(),
                &[escrow_bump],
            ],
        ];

        // Update state before CPI to avoid borrow issues
        participant.claimed = true;
        participant.prize_amount = 0;
        
        tournament.prize_pool = tournament
            .prize_pool
            .checked_sub(prize_amount)
            .ok_or(TarniError::MathError)?;

        let cpi_accounts = Transfer {
            from: self.escrow.to_account_info(),
            to: self.player.to_account_info(),
        };
        let cpi_program = self.system_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, prize_amount)?;

        emit!(PrizeClaimed {
            tournament: tournament.key(),
            player: self.player.key(),
            amount: prize_amount,
        });

        Ok(())
    }
}