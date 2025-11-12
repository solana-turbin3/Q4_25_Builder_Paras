use anchor_lang::prelude::*;

#[error_code]
pub enum TarniError {
    #[msg("Tournament registration is closed.")]
    RegistrationClosed,
    #[msg("Tournament is already full.")]
    TournamentFull,
    #[msg("entry fee less than minimum required.")]
    EntryFeeTooLow,
    #[msg("Entry fee does not match required amount.")]
    IncorrectEntryFee,
    #[msg("Not enough participants registered.")]
    NotEnoughParticipants,
    #[msg("Cannot check in outside check-in window.")]
    CheckinNotOpen,
    #[msg("Player has already checked in or been disqualified.")]
    AlreadyCheckedInOrDQ,
    #[msg("Tournament state does not allow this action.")]
    InvalidTournamentState,
    #[msg("Prize split percentages do not sum to 100.")]
    InvalidPrizeSplit,
    #[msg("Only the organizer can perform this action.")]
    NotOrganizer,
    #[msg("Only backend authority can perform this action.")]
    NotBackendAuthority,
    #[msg("Participant not found in tournament.")]
    ParticipantNotFound,
    #[msg("You are not eligible to claim a prize.")]
    NotEligibleForPrize,
    #[msg("Refund not available or already claimed.")]
    RefundNotAvailable,
    #[msg("Too many participants provided.")]
    TooManyParticipants,
    #[msg("Invalid argument provided.")]
    InvalidArgument,
    #[msg("Math operation error.")]
    MathError,
}
