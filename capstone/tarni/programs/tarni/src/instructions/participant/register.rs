use::anchor_lang::prelude::*;
use::anchor_lang::system_program::{
    Transfer, transfer };

use crate::{
    constants::*,
    error::TarniError,
    state::{Participant, Tournament},
    events::ParticipantRegistered,
    utils::time::now_ts,
};


#[derive(Accounts)]
pub struct RegisterParticipant<'info> {
    #[account(
        mut,
        has_one = escrow @ TarniError::InvalidEscrowAccount,
        constraint = tournament.can_register() @ TarniError::InvalidTournamentState
    )]
    pub tournament: Account<'info, Tournament>,


    #[account(
        init,
        payer = player,
        space = 8 + Participant::INIT_SPACE,
        seeds = [PARTICIPANT_SEED, tournament.key().as_ref(), player.key.as_ref()],
        bump,
    )]
    pub participant: Account<'info, Participant>,

    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut
    )]
    pub escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


impl<'info> RegisterParticipant<'info> {
    pub fn register(
        &mut self,
        game_account: Pubkey,
    ) -> Result<()> {
        let tournament = &mut self.tournament;


        require!(
            tournament.current_participants < tournament.max_participants,
            TarniError::TournamentFull
        );

        let entry_fee = tournament.entry_fee;

        let accounts = Transfer {
                    from: self.player.to_account_info(),
                    to: self.escrow.to_account_info(),
                };

        let cpi_ctx =   CpiContext::new(
                self.system_program.to_account_info(),
                accounts,
            );

        transfer(cpi_ctx, entry_fee)?;

        tournament.current_participants = tournament
            .current_participants
            .checked_add(1)
            .ok_or(TarniError::MathError)?;

        tournament.prize_pool = tournament
            .prize_pool
            .checked_add(entry_fee)
            .ok_or(TarniError::MathError)?;

        self.participant.set_inner(
            Participant {
                player: self.player.key(),
                tournament: tournament.key(),
                game_account,
                registered_at: now_ts(),
                entry_paid: entry_fee,
                checked_in: false,
                checkin_time: 0,
                disqualified: false,
                dq_reason: String::new(),
                prize_amount: 0,
                claimed: false,
                refunded: false,
                refund_amount: 0,
                bump: self.participant.bump,
            }
        );

        emit!(ParticipantRegistered {
            tournament: tournament.key(),
            player: self.player.key(),
            slot: tournament.current_participants,
        });

        Ok(())
    }
}