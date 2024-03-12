use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided account did not match the expected account.")]
    AccountMismatch,
    #[msg("The challenge cannot be created because it already exists.")]
    ChallengeAlreadyExists,
    #[msg("The participant is already registered for this challenge.")]
    ParticipantAlreadyRegistered,
    #[msg("The challenge is not active or does not exist.")]
    ChallengeNotFound,
    #[msg("The result submission is invalid or not within the challenge parameters.")]
    InvalidResultSubmission,
}
