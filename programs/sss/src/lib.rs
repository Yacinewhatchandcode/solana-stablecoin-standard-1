use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod events;

use instructions::*;

declare_id!("SSS111111111111111111111111111111111111111");

/// Solana Stablecoin Standard (SSS) Program
/// 
/// A single configurable program supporting two presets:
/// - SSS-1 (Minimal Stablecoin): Mint authority + freeze authority + metadata
/// - SSS-2 (Compliant Stablecoin): SSS-1 + permanent delegate + transfer hook + blacklist
#[program]
pub mod sss {
    use super::*;

    // ─────────────────────────────────────────────────
    // Token Lifecycle
    // ─────────────────────────────────────────────────

    /// Initialize a new stablecoin with configurable preset (SSS-1 or SSS-2)
    pub fn initialize(
        ctx: Context<Initialize>,
        config: StablecoinConfig,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, config)
    }

    /// Mint tokens to a specified account
    pub fn mint_to(
        ctx: Context<MintTo>,
        amount: u64,
    ) -> Result<()> {
        instructions::mint::handler(ctx, amount)
    }

    /// Burn tokens from a specified account  
    pub fn burn(
        ctx: Context<Burn>,
        amount: u64,
    ) -> Result<()> {
        instructions::burn::handler(ctx, amount)
    }

    /// Transfer tokens between accounts
    pub fn transfer(
        ctx: Context<Transfer>,
        amount: u64,
    ) -> Result<()> {
        instructions::transfer::handler(ctx, amount)
    }

    // ─────────────────────────────────────────────────
    // Freeze Operations
    // ─────────────────────────────────────────────────

    /// Freeze a token account (prevents all transfers)
    pub fn freeze_account(
        ctx: Context<FreezeAccount>,
    ) -> Result<()> {
        instructions::freeze::freeze_handler(ctx)
    }

    /// Thaw (unfreeze) a token account
    pub fn thaw_account(
        ctx: Context<ThawAccount>,
    ) -> Result<()> {
        instructions::freeze::thaw_handler(ctx)
    }

    // ─────────────────────────────────────────────────
    // SSS-2: Compliance Operations
    // ─────────────────────────────────────────────────

    /// Add an address to the blacklist (SSS-2 only)
    pub fn blacklist_add(
        ctx: Context<BlacklistAdd>,
        address: Pubkey,
    ) -> Result<()> {
        instructions::compliance::blacklist_add_handler(ctx, address)
    }

    /// Remove an address from the blacklist (SSS-2 only)
    pub fn blacklist_remove(
        ctx: Context<BlacklistRemove>,
        address: Pubkey,
    ) -> Result<()> {
        instructions::compliance::blacklist_remove_handler(ctx, address)
    }

    /// Seize tokens from a blacklisted account via permanent delegate (SSS-2 only)
    pub fn seize_tokens(
        ctx: Context<SeizeTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::compliance::seize_tokens_handler(ctx, amount)
    }

    // ─────────────────────────────────────────────────
    // Role Management
    // ─────────────────────────────────────────────────

    /// Update role assignment (MINT_AUTHORITY, FREEZE_AUTHORITY, COMPLIANCE_OFFICER)
    pub fn update_role(
        ctx: Context<UpdateRole>,
        role: Role,
        new_authority: Pubkey,
    ) -> Result<()> {
        instructions::roles::update_role_handler(ctx, role, new_authority)
    }

    // ─────────────────────────────────────────────────
    // Transfer Hook (SSS-2)
    // ─────────────────────────────────────────────────

    /// Execute the transfer hook — checks blacklist before every transfer
    pub fn transfer_hook(
        ctx: Context<TransferHook>,
        amount: u64,
    ) -> Result<()> {
        instructions::hook::transfer_hook_handler(ctx, amount)
    }

    /// Fallback for the transfer hook interface
    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        instructions::hook::fallback_handler(program_id, accounts, data)
    }
}
