use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::{StablecoinState, Role};
use crate::errors::SSSError;
use crate::events::RoleUpdated;

/// Update role assignment for the stablecoin.
/// Only the current authority can reassign roles.
pub fn update_role_handler(
    ctx: Context<UpdateRole>,
    role: Role,
    new_authority: Pubkey,
) -> Result<()> {
    let state = &mut ctx.accounts.stablecoin_state;

    // Only the original authority can update roles
    require!(
        ctx.accounts.authority.key() == state.authority,
        SSSError::Unauthorized
    );

    let clock = Clock::get()?;
    let old_authority;
    let role_name;

    match role {
        Role::MintAuthority => {
            old_authority = state.mint_authority;
            state.mint_authority = new_authority;
            role_name = "MINT_AUTHORITY".to_string();
        },
        Role::FreezeAuthority => {
            old_authority = state.freeze_authority;
            state.freeze_authority = new_authority;
            role_name = "FREEZE_AUTHORITY".to_string();
        },
        Role::ComplianceOfficer => {
            require!(state.is_sss2, SSSError::SSS2Required);
            old_authority = state.compliance_officer;
            state.compliance_officer = new_authority;
            role_name = "COMPLIANCE_OFFICER".to_string();
        },
    }

    state.updated_at = clock.unix_timestamp;

    emit!(RoleUpdated {
        mint: state.mint,
        role: role_name.clone(),
        old_authority,
        new_authority,
        updated_by: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Updated {} to {}", role_name, new_authority);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateRole<'info> {
    /// Stablecoin authority (owner)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The mint
    pub mint: InterfaceAccount<'info, Mint>,

    /// Stablecoin state PDA
    #[account(
        mut,
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,
}
