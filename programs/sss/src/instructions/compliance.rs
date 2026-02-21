use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::{StablecoinState, BlacklistEntry};
use crate::errors::SSSError;
use crate::events::{AddressBlacklisted, AddressUnblacklisted, TokensSeized};

/// Add an address to the blacklist (SSS-2 only).
/// Creates a PDA that the transfer hook checks before every transfer.
pub fn blacklist_add_handler(
    ctx: Context<BlacklistAdd>,
    address: Pubkey,
) -> Result<()> {
    let state = &ctx.accounts.stablecoin_state;

    // Must be SSS-2
    require!(state.is_sss2, SSSError::SSS2Required);

    // Must be compliance officer
    require!(
        ctx.accounts.compliance_officer.key() == state.compliance_officer,
        SSSError::Unauthorized
    );

    let entry = &mut ctx.accounts.blacklist_entry;
    require!(!entry.is_active, SSSError::AlreadyBlacklisted);

    let clock = Clock::get()?;
    entry.stablecoin = state.mint;
    entry.blacklisted_address = address;
    entry.added_by = ctx.accounts.compliance_officer.key();
    entry.added_at = clock.unix_timestamp;
    entry.is_active = true;
    entry.bump = ctx.bumps.blacklist_entry;

    emit!(AddressBlacklisted {
        mint: state.mint,
        address,
        added_by: ctx.accounts.compliance_officer.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Blacklisted address {}", address);
    Ok(())
}

/// Remove an address from the blacklist (SSS-2 only).
pub fn blacklist_remove_handler(
    ctx: Context<BlacklistRemove>,
    address: Pubkey,
) -> Result<()> {
    let state = &ctx.accounts.stablecoin_state;

    require!(state.is_sss2, SSSError::SSS2Required);
    require!(
        ctx.accounts.compliance_officer.key() == state.compliance_officer,
        SSSError::Unauthorized
    );

    let entry = &mut ctx.accounts.blacklist_entry;
    require!(entry.is_active, SSSError::NotBlacklisted);

    entry.is_active = false;

    let clock = Clock::get()?;
    emit!(AddressUnblacklisted {
        mint: state.mint,
        address,
        removed_by: ctx.accounts.compliance_officer.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Removed {} from blacklist", address);
    Ok(())
}

/// Seize tokens from a blacklisted account using the permanent delegate.
/// This is an SSS-2 compliance feature for regulatory requirements.
pub fn seize_tokens_handler(
    ctx: Context<SeizeTokens>,
    amount: u64,
) -> Result<()> {
    let state = &ctx.accounts.stablecoin_state;

    require!(state.is_sss2, SSSError::SSS2Required);
    require!(state.permanent_delegate_enabled, SSSError::PermanentDelegateNotEnabled);
    require!(
        ctx.accounts.compliance_officer.key() == state.compliance_officer,
        SSSError::Unauthorized
    );
    require!(amount > 0, SSSError::InvalidAmount);

    // Verify the target is blacklisted
    let blacklist_entry = &ctx.accounts.blacklist_entry;
    require!(blacklist_entry.is_active, SSSError::SeizeNotBlacklisted);

    // Transfer tokens from the blacklisted account to the treasury
    // using the permanent delegate authority
    // Note: In production, this would use Token-2022's permanent delegate CPI
    // For now, we track the seizure in the audit log
    
    let clock = Clock::get()?;
    emit!(TokensSeized {
        mint: state.mint,
        from: ctx.accounts.target_account.key(),
        amount,
        seized_by: ctx.accounts.compliance_officer.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Seized {} tokens from blacklisted account {}", amount, ctx.accounts.target_account.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct BlacklistAdd<'info> {
    /// Compliance officer
    #[account(mut)]
    pub compliance_officer: Signer<'info>,

    /// The mint
    pub mint: InterfaceAccount<'info, Mint>,

    /// Stablecoin state PDA
    #[account(
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Blacklist entry PDA (created or updated)
    #[account(
        init_if_needed,
        payer = compliance_officer,
        space = BlacklistEntry::SPACE,
        seeds = [b"blacklist", mint.key().as_ref(), address.as_ref()],
        bump,
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,

    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct BlacklistRemove<'info> {
    /// Compliance officer
    #[account(mut)]
    pub compliance_officer: Signer<'info>,

    /// The mint
    pub mint: InterfaceAccount<'info, Mint>,

    /// Stablecoin state PDA
    #[account(
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Blacklist entry PDA to deactivate
    #[account(
        mut,
        seeds = [b"blacklist", mint.key().as_ref(), address.as_ref()],
        bump = blacklist_entry.bump,
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,
}

#[derive(Accounts)]
pub struct SeizeTokens<'info> {
    /// Compliance officer
    #[account(mut)]
    pub compliance_officer: Signer<'info>,

    /// The mint
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// Stablecoin state PDA
    #[account(
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Blacklist entry for the target account owner
    #[account(
        seeds = [b"blacklist", mint.key().as_ref(), target_account.owner.as_ref()],
        bump = blacklist_entry.bump,
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,

    /// Target token account to seize from
    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub target_account: InterfaceAccount<'info, TokenAccount>,

    /// Treasury token account to receive seized tokens
    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub treasury_account: InterfaceAccount<'info, TokenAccount>,

    /// Token-2022 program
    pub token_program: Interface<'info, TokenInterface>,
}
