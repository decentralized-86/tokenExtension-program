pub mod instructions;
pub mod state;
// use crate::instructions::{ChallengeParams, CreateChallenge};
use crate::state::ChallengeMetadata;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("FUkgpVESK463wEYuwfpTbXGr2YtgezdQnSDPhteNWyrN");

#[program]
pub mod p2p_challenge {
    use super::*;

    pub fn create_challenge(ctx: Context<CreateChallenge>, params: ChallengeParams) -> Result<()> {
        let challenge_metadata = &mut ctx.accounts.challenge_metadata;
        let escrow_token_account = &mut ctx.accounts.escrow_token_account;

        // Setup challenge metadata
        challenge_metadata.creator = *ctx.accounts.creator.key;
        challenge_metadata.goal = params.goal;
        challenge_metadata.challenge_type = params.challenge_type;
        challenge_metadata.start_time = params.start_time;
        challenge_metadata.end_time = params.end_time;
        challenge_metadata.stake_amount = params.stake_amount;
        challenge_metadata.is_active = true;
        challenge_metadata.winner = Pubkey::default();
        challenge_metadata.escrow_account = escrow_token_account.key();

        // Calculate the escrow authority PDA
        let (escrow_authority, bump_seed) = Pubkey::find_program_address(
            &[b"escrow", challenge_metadata.to_account_info().key.as_ref()],
            ctx.program_id,
        );

        // Derive the seeds for the escrow authority
        let seeds = &[
            b"escrow",
            challenge_metadata.to_account_info().key.as_ref(),
            &[bump_seed],
        ];

        let signer_seeds: &[&[&[u8]]] = &[seeds];

        // Fund the escrow token account and delegate it to the escrow authority PDA
        let cpi_accounts = Transfer {
            from: ctx.accounts.creator.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.creator.to_account_info(),
        };
        let token_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, params.stake_amount)?;

        Ok(())
    }

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
}

// The CreateChallenge struct from instructions.rs should have all the accounts you are using here.
