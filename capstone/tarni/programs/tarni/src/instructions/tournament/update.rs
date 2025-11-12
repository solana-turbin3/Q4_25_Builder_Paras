use anchor_lang::prelude::*;

use crate::{
    constants::*,
    error::TarniError,
    state::{PrizeSplit, Tournament},
    utils::{time::now_ts},
};

#[derive(Accounts)]
pub struct UpdateTournament<'info> {
    #[account(
        mut,
        has_one = organizer @ TarniError::NotOrganizer,
        constraint = tournament.is_mut() @ TarniError::InvalidTournamentState,
    )]
    pub tournament: Account<'info, Tournament>,

    pub organizer: Signer<'info>,
}

impl <'info> UpdateTournament<'info> {
    pub fn update(
        &mut self,
        entry_fee: Option<u64>,
        max_participants: Option<u8>,
        prize_splits: Option<PrizeSplit>,
        rules: Option<String>,
        starts_at: Option<i64>,
    ) -> Result<()> {
        let tournament = &mut self.tournament;

        if let Some(entry_fee) = entry_fee {
            require!(
                entry_fee >= MIN_ENTRY_FEE,
                TarniError::EntryFeeTooLow
            );
            tournament.entry_fee = entry_fee;
        }

        if let Some(max) = max_participants {
            require!(
                max >= MIN_PARTICIPANTS && max <= MAX_PARTICIPANTS,
                TarniError::InvalidArgument
            );
            require!(
                max >= tournament.current_participants,
                TarniError::InvalidArgument
            );
            tournament.max_participants = max;
        }

        if let Some(split) = prize_splits  {
            require!(
                split.is_valid(),
                TarniError::InvalidPrizeSplit
            );
            tournament.prize_split = split;
        }
        
        if let Some(new_rules) = rules {
            require!(
                new_rules.len() <= MAX_RULES_LENGTH,
                TarniError::InvalidArgument
            );
            tournament.rules = new_rules;
        }

        if let Some(new_start) = starts_at {
            require!(
                new_start > now_ts(),
                TarniError::InvalidArgument
            );
            tournament.starts_at = new_start;
            tournament.checkin_opens = new_start
                .checked_sub(CHECKIN_WINDOW_SECONDS)
                .ok_or(TarniError::MathError)?;
        }       
        Ok(())
    }
}