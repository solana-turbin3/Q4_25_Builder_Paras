use anchor_lang::prelude::*;

declare_id!("6ZDRzAxyRYS5GsZKEm4BWjEty3NaBuvRg8GvzDKck27c");

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

#[program]
pub mod tarni {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_tournament(
        ctx: Context<CreateTournament>,
        tournament_id: u64,
        game_type: GameType,
        entry_fee: u64,
        max_participants: u8,
        prize_split: PrizeSplit,
        rules: String,
        starts_at: i64,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.process(
            game_type,
            tournament_id,
            entry_fee,
            max_participants,
            prize_split,
            rules,
            starts_at,
            ctx.bumps.escrow,
        )
    }

    pub fn lock_registration(
        ctx: Context<LockRegistration>
    ) -> anchor_lang::Result<()> {
        ctx.accounts.process()
    }

    pub fn update_tournament(
        ctx: Context<UpdateTournament>,
        entry_fee: Option<u64>,
        max_participants: Option<u8>,
        prize_splits: Option<PrizeSplit>,
        rules: Option<String>,
        starts_at: Option<i64>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.update(entry_fee, max_participants, prize_splits, rules, starts_at)
    }

    pub fn cancel_tournament(
        ctx: Context<CancelTournament>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.cancel()
    }

    pub fn register_participant(
        ctx: Context<RegisterParticipant>,
        game_account: Pubkey,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.register(game_account, ctx.bumps.participant)
    }

    pub fn checkin(
        ctx: Context<Checkin>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.checkin()
    }

    pub fn claim_prize(
        ctx: Context<ClaimPrize>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.claim_prize(ctx.bumps.escrow)
    }

    pub fn claim_refund(
        ctx: Context<ClaimRefund>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.claim_refund(ctx.bumps.escrow)
    }

    pub fn launch_tournament(
        ctx: Context<LaunchTournament>,
        match_id_hash: [u8; 32],
    ) -> anchor_lang::Result<()> {
        ctx.accounts.launch_tournament(match_id_hash, ctx.bumps.match_account)
    }

    pub fn start_match(
        ctx: Context<StartMatch>,
        match_id: u64,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.start_match(match_id)
    }

    pub fn submit_results(
        ctx: Context<SubmitResults>,
        ipfs_cid: String,
        signature: [u8; 64],
        placements: Vec<state::PlayerResult>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.submit_results(ipfs_cid, signature, placements, ctx.bumps.result)
    }

    pub fn auto_disqualify(
        ctx: Context<AutoDisqualify>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.auto_disqualify()
    }

    pub fn distribute_prizes(
        ctx: Context<Prizes>,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.calculate_prizes()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
