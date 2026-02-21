use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::{StablecoinConfig, StablecoinState};
use crate::errors::SSSError;
use crate::events::StablecoinInitialized;

/// Initialize a new stablecoin with either SSS-1 or SSS-2 preset.
/// 
/// SSS-1 (Minimal): Mint authority + freeze authority + metadata
/// SSS-2 (Compliant): SSS-1 + permanent delegate + transfer hook + blacklist
pub fn handler(
    ctx: Context<Initialize>,
    config: StablecoinConfig,
) -> Result<()> {
    // Validate config
    require!(config.name.len() <= StablecoinState::MAX_NAME_LEN, SSSError::NameTooLong);
    require!(config.symbol.len() <= StablecoinState::MAX_SYMBOL_LEN, SSSError::SymbolTooLong);

    let clock = Clock::get()?;
    let state = &mut ctx.accounts.stablecoin_state;

    // Store stablecoin state
    state.mint = ctx.accounts.mint.key();
    state.authority = ctx.accounts.authority.key();
    state.mint_authority = ctx.accounts.authority.key();
    state.freeze_authority = ctx.accounts.authority.key();
    state.compliance_officer = ctx.accounts.authority.key();
    state.is_sss2 = config.is_sss2();
    state.permanent_delegate_enabled = config.enable_permanent_delegate;
    state.transfer_hook_enabled = config.enable_transfer_hook;
    state.default_account_frozen = config.default_account_frozen;
    state.total_minted = 0;
    state.total_burned = 0;
    state.created_at = clock.unix_timestamp;
    state.updated_at = clock.unix_timestamp;
    state.bump = ctx.bumps.stablecoin_state;
    state.name = config.name.clone();
    state.symbol = config.symbol.clone();
    state.decimals = config.decimals;

    // Emit initialization event for audit trail
    emit!(StablecoinInitialized {
        mint: state.mint,
        authority: state.authority,
        name: config.name,
        symbol: config.symbol,
        decimals: config.decimals,
        preset: state.preset_label(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Initialized {} stablecoin ({})", state.preset_label(), state.symbol);

    Ok(())
}

impl StablecoinState {
    pub fn preset_label(&self) -> String {
        if self.is_sss2 {
            "SSS-2".to_string()
        } else {
            "SSS-1".to_string()
        }
    }
}

#[derive(Accounts)]
#[instruction(config: StablecoinConfig)]
pub struct Initialize<'info> {
    /// The authority (deployer) who owns this stablecoin
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The mint account for the new stablecoin token
    /// Created via Token-2022 with appropriate extensions
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// PDA storing the stablecoin's configuration and state
    #[account(
        init,
        payer = authority,
        space = StablecoinState::SPACE,
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Token-2022 program (required for extensions)
    pub token_program: Interface<'info, TokenInterface>,

    /// System program
    pub system_program: Program<'info, System>,

    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}
