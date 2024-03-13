use crate::state::{ChallengeMetadata, EscrowAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ChallengeParams {
    pub goal: u64,
    pub challenge_type: u8,
    pub start_time: i64,
    pub end_time: i64,
    pub stake_amount: u64,
}

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(init, payer = creator, space = ChallengeMetadata::LEN)]
    pub challenge_metadata: Account<'info, ChallengeMetadata>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}
