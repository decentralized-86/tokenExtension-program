pub mod instructions;
pub mod state;
// use crate::instructions::{ChallengeParams, CreateChallenge};
use crate::state::ChallengeMetadata;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

    use anchor_lang::solana_program::pubkey;

declare_id!("FUkgpVESK463wEYuwfpTbXGr2YtgezdQnSDPhteNWyrN");

#[program]
pub mod p2p_challenge {
    use super::*;

    #[error_code]
    pub enum MyError {
        #[msg("Password was wrong")]
        WrongPassword,
    }

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

    pub fn finalize_challenge(
        ctx: Context<FinalizeChallenge>,
        winner: pubkey,
        password: string,
    ) -> Result<()> {

        //Check password
        if password != "secrect" {
                return err!(MyError::WrongPassword);
                //panic!("Password wrong. OMG!");
            }

        let challenge_metadata = &ctx.accounts.challenge_metadata;
        let escrow_token_account = &ctx.accounts.escrow_token_account;
        let winner_account = &ctx.accounts.winner;

        // Check if the challenge is active
        if !challenge_metadata.is_active {
            return Err(ErrorCode::ChallengeNotActive.into());
        }

        // Update winner information
        challenge_metadata.winner = winner;
        challenge_metadata.is_active = false;

        //Cpi accounts
        let cpi_accounts = Transfer {
            from: escrow_token_account.to_account_info().clone(),
            to: winner_account.to_account_info().clone(),
            authority: ctx.accounts.winner.to_account_info(),
        };

        // Transfer all funds from escrow to the winner
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.clone(),
            cpi_accounts,
            ctx.accounts.rent.to_account_info().clone(),
        );
        Token::transfer(transfer_ctx, escrow_token_account.lamports())?;

        msg!("Challenge finalized! Funds transferred to winner!");

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
        #[account(
        init_if_needed,
        seeds = [b"escrowVault"],
        bump,
        payer = signer,
        space = 8
        )]
        pub escrow_token_account: Account<'info, TokenAccount>,
        pub token_mint: Account<'info, Mint>,
        #[account(mut)]
        pub creator: Signer<'info>,
        pub system_program: Program<'info, System>,
        pub token_program: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
    }

     #[derive(Accounts)]
    pub struct FinalizeChallenge<'info> {
        #[account(mut)]
        pub challenge_metadata: Account<'info, ChallengeMetadata>,
        #[account(mut)]
        pub escrow_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub winner: Signer<'info>, // Added winner as Signer
        pub token_program: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
    }
}

// The CreateChallenge struct from instructions.rs should have all the accounts you are using here.
