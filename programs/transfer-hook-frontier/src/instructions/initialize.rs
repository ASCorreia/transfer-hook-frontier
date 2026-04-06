use anchor_lang::prelude::*;
use anchor_spl::{token_2022, token_interface::Mint};

use crate::{ANCHOR_DISCRIMINATOR_SIZE, RateLimit, error::ErrorCode};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = user,
        seeds = [b"rate_limit", mint.key().as_ref(), user.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR_SIZE + RateLimit::INIT_SPACE,
    )]
    pub rate_limit: Account<'info, RateLimit>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // Ensure the mint is a token-2022 mint by checking its owner
    require!(ctx.accounts.mint.to_account_info().owner == &token_2022::ID, ErrorCode::InvalidMint);

    // Initialize the rate limit account with the user, mint, max amount, and last updated timestamp
    ctx.accounts.rate_limit.set_inner(RateLimit { 
        user: ctx.accounts.user.key(), 
        mint: ctx.accounts.mint.key(), 
        max_amount: RateLimit::MAX_AMOUNT, 
        last_updated: Clock::get()?.unix_timestamp, 
        amount_transferred: 0 
    });

    Ok(())
}
