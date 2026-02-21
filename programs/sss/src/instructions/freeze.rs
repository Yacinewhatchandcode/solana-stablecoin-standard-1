use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, FreezeAccount as SplFreeze, ThawAccount as SplThaw, freeze_account, thaw_account};

use crate::state::StablecoinState;
use crate::errors::SSSError;
use crate::events::{AccountFrozenEvent, AccountThawedEvent};

/// Freeze a token account — prevents all transfers in/out.
pub fn freeze_handler(
    ctx: Context<FreezeAccount>,
) -> Result<()> {
    let state = &ctx.accounts.stablecoin_state;

    // Verify caller is the freeze authority
    require!(
        ctx.accounts.freeze_authority.key() == state.freeze_authority,
        SSSError::Unauthorized
    );

    let cpi_accounts = SplFreeze {
        account: ctx.accounts.token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.freeze_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    freeze_account(cpi_ctx)?;

    let clock = Clock::get()?;
    emit!(AccountFrozenEvent {
        mint: state.mint,
        account: ctx.accounts.token_account.key(),
        frozen_by: ctx.accounts.freeze_authority.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Froze account {}", ctx.accounts.token_account.key());
    Ok(())
}

/// Thaw (unfreeze) a token account — re-enables transfers.
pub fn thaw_handler(
    ctx: Context<ThawAccount>,
) -> Result<()> {
    let state = &ctx.accounts.stablecoin_state;

    require!(
        ctx.accounts.freeze_authority.key() == state.freeze_authority,
        SSSError::Unauthorized
    );

    let cpi_accounts = SplThaw {
        account: ctx.accounts.token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.freeze_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    thaw_account(cpi_ctx)?;

    let clock = Clock::get()?;
    emit!(AccountThawedEvent {
        mint: state.mint,
        account: ctx.accounts.token_account.key(),
        thawed_by: ctx.accounts.freeze_authority.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Thawed account {}", ctx.accounts.token_account.key());
    Ok(())
}

#[derive(Accounts)]
pub struct FreezeAccount<'info> {
    /// Freeze authority
    #[account(mut)]
    pub freeze_authority: Signer<'info>,

    /// The mint
    pub mint: InterfaceAccount<'info, Mint>,

    /// Token account to freeze
    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    /// Stablecoin state PDA
    #[account(
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Token-2022 program
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ThawAccount<'info> {
    /// Freeze authority
    #[account(mut)]
    pub freeze_authority: Signer<'info>,

    /// The mint
    pub mint: InterfaceAccount<'info, Mint>,

    /// Token account to thaw
    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    /// Stablecoin state PDA
    #[account(
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Token-2022 program
    pub token_program: Interface<'info, TokenInterface>,
}
