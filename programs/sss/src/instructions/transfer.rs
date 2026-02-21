use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

use crate::state::StablecoinState;
use crate::errors::SSSError;
use crate::events::TokensTransferred;

/// Transfer tokens between accounts.
/// For SSS-2, the transfer hook will automatically check blacklists.
pub fn handler(
    ctx: Context<Transfer>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, SSSError::InvalidAmount);

    let decimals = ctx.accounts.mint.decimals;

    // Perform transfer via Token-2022 (transfer_checked for safety)
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.from.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    transfer_checked(cpi_ctx, amount, decimals)?;

    // Update timestamp
    let state = &mut ctx.accounts.stablecoin_state;
    state.updated_at = Clock::get()?.unix_timestamp;

    emit!(TokensTransferred {
        mint: state.mint,
        from: ctx.accounts.from.key(),
        to: ctx.accounts.to.key(),
        amount,
        timestamp: state.updated_at,
    });

    msg!("SSS: Transferred {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    /// Owner of the source token account
    #[account(mut)]
    pub owner: Signer<'info>,

    /// The mint
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// Source token account
    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
        token::token_program = token_program,
    )]
    pub from: InterfaceAccount<'info, TokenAccount>,

    /// Destination token account
    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub to: InterfaceAccount<'info, TokenAccount>,

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
