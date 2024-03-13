use anchor_lang::prelude::*;

#[account]
pub struct ChallengeMetadata {
    pub creator: Pubkey,
    pub goal: u64,
    pub challenge_type: u8,
    pub start_time: i64,
    pub end_time: i64,
    pub stake_amount: u64,
    pub is_active: bool,
    pub winner: Pubkey,
    pub escrow_account: Pubkey,
}

impl ChallengeMetadata {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 8 + 8 + 1 + 32 + 32;
}

#[account]
pub struct EscrowAccount {
    pub challenge_id: Pubkey,
    pub escrow_token_account: Pubkey,
    pub total_stake: u64,
    pub escrow_authority: Pubkey,
    pub is_active: bool,
}

impl EscrowAccount {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 32 + 1;
}
