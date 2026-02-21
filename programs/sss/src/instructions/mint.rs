use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, MintTo as SplMintTo, mint_to};

use crate::state::StablecoinState;
use crate::errors::SSSError;
use crate::events::TokensMinted;

/// Mint new tokens to a specified token account.
/// Only callable by the designated mint authority.
pub fn handler(
    ctx: Context<MintTo>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, SSSError::InvalidAmount);

    let state = &ctx.accounts.stablecoin_state;
    
    // Verify caller is the mint authority
    require!(
        ctx.accounts.mint_authority.key() == state.mint_authority,
        SSSError::Unauthorized
    );

    // Perform the mint via Token-2022
    let cpi_accounts = SplMintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    mint_to(cpi_ctx, amount)?;

    // Update audit state
    let state = &mut ctx.accounts.stablecoin_state;
    state.total_minted = state.total_minted.checked_add(amount).ok_or(SSSError::Overflow)?;
    state.updated_at = Clock::get()?.unix_timestamp;

    emit!(TokensMinted {
        mint: state.mint,
        to: ctx.accounts.token_account.key(),
        amount,
        minted_by: ctx.accounts.mint_authority.key(),
        total_minted: state.total_minted,
        timestamp: state.updated_at,
    });

    msg!("SSS: Minted {} tokens to {}", amount, ctx.accounts.token_account.key());

    Ok(())
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    /// Mint authority (must match stablecoin_state.mint_authority)
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// The mint
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// Destination token account
    #[account(
        mut,
        token::mint = mint,
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
