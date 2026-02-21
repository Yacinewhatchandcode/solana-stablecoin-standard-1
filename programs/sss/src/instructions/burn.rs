use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, Burn as SplBurn, burn};

use crate::state::StablecoinState;
use crate::errors::SSSError;
use crate::events::TokensBurned;

/// Burn tokens from a specified token account.
/// Only callable by the token owner or an authorized authority.
pub fn handler(
    ctx: Context<Burn>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, SSSError::InvalidAmount);

    // Perform the burn via Token-2022
    let cpi_accounts = SplBurn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    burn(cpi_ctx, amount)?;

    // Update audit state
    let state = &mut ctx.accounts.stablecoin_state;
    state.total_burned = state.total_burned.checked_add(amount).ok_or(SSSError::Overflow)?;
    state.updated_at = Clock::get()?.unix_timestamp;

    emit!(TokensBurned {
        mint: state.mint,
        from: ctx.accounts.token_account.key(),
        amount,
        burned_by: ctx.accounts.owner.key(),
        total_burned: state.total_burned,
        timestamp: state.updated_at,
    });

    msg!("SSS: Burned {} tokens from {}", amount, ctx.accounts.token_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct Burn<'info> {
    /// Owner of the token account (or authorized delegate)
    #[account(mut)]
    pub owner: Signer<'info>,

    /// The mint
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// Source token account to burn from
    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
        token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    /// Stablecoin state PDA
    #[account(
        mut,
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Token-2022 program
    pub token_program: Interface<'info, TokenInterface>,
}
