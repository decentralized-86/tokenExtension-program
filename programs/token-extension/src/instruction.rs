use crate::state::{ChallengeMetadata, EscrowAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(init, payer = creator, space = ChallengeMetadata::LEN)]
    pub challenge_metadata: Account<'info, ChallengeMetadata>,
    #[account(init, payer = creator, space = EscrowAccount::LEN, seeds = [b"escrow", challenge_metadata.key().as_ref()], bump)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ChallengeParams {
    pub goal: u64,
    pub challenge_type: u8,
    pub start_time: i64,
    pub end_time: i64,
    pub stake_amount: u64,
}
