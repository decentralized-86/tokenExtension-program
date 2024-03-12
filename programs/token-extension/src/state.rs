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
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 8 + 8 + 1 + 32 + 32; // Updated size
}

#[account]
pub struct EscrowAccount {
    // Challenge ID remains the same, linking the escrow to its challenge
    pub challenge_id: Pubkey,
    // This account holds the escrowed tokens
    pub escrow_token_account: Pubkey,
    // Tracks total amount staked in the escrow
    pub total_stake: u64,
    // Program-derived address that acts as the escrow authority, capable of moving tokens
    pub escrow_authority: Pubkey,
    // Indicator of escrow's active status
    pub is_active: bool,
}

impl EscrowAccount {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 32 + 1; // Adjusted for the new structure
}
