use anchor_lang::prelude::*;
use anchor_lang::{system_program, system_program::{Transfer, transfer}};

use crate::{
    constants::*,
    error::TarniError,
    state::{Participant, Tournament},
    events::RefundClaimed,
};

#[derive(Accounts)]
pub struct ClaimRefund<'info>{
    #[account(
        mut,
        constraint = tournament.escrow == escrow.key() @ TarniError::InvalidEscrowAccount,
        constraint = tournament.is_cancel()@ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        has_one = player @ TarniError::InvalidParticipant,
        has_one = tournament @ TarniError::InvalidParticipant,
        constraint = participant.can_claim_refund() @ TarniError::NotEligibleForPrize,
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


impl<'info> ClaimRefund<'info> {
    pub fn claim_refund(&mut self, escrow_bump: u8) -> Result<()> {
        let tournament = &mut self.tournament;
        let participant = &mut self.participant;

        let refund_amount = participant.refund_amount;
        let tournament_key = tournament.key();
        let signer_seeds: &[&[&[u8]]] = &[
            &[
                ESCROW_SEED,
                tournament_key.as_ref(),
                &[escrow_bump],
            ],
        ];

        // Update state before CPI to avoid borrow issues
        participant.refunded = true;
        participant.refund_amount = 0;

        tournament.prize_pool = tournament
            .prize_pool
            .checked_sub(refund_amount)
            .ok_or(TarniError::MathError)?;

        let cpi_accounts = Transfer {
            from: self.escrow.to_account_info(),
            to: self.player.to_account_info(),
        };
        let cpi_program = self.system_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, refund_amount)?;

        emit !(RefundClaimed {
            tournament: tournament.key(),
            player: self.player.key(),
            amount: refund_amount,
        });

        Ok(())
    }
}