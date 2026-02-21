use anchor_lang::prelude::*;

use crate::state::{StablecoinState, BlacklistEntry};
use crate::errors::SSSError;
use crate::events::TransferHookExecuted;

/// Transfer hook handler — executed automatically by Token-2022 before every transfer.
/// For SSS-2 tokens, this checks that neither the sender nor recipient is blacklisted.
pub fn transfer_hook_handler(
    ctx: Context<TransferHook>,
    amount: u64,
) -> Result<()> {
    let state = &ctx.accounts.stablecoin_state;
    
    // Only enforce for SSS-2 tokens with transfer hook enabled
    if !state.transfer_hook_enabled {
        return Ok(());
    }

    let clock = Clock::get()?;

    // Check sender blacklist
    if let Some(sender_bl) = &ctx.accounts.sender_blacklist {
        if sender_bl.is_active {
            emit!(TransferHookExecuted {
                mint: state.mint,
                source: ctx.accounts.source.key(),
                destination: ctx.accounts.destination.key(),
                amount,
                allowed: false,
                timestamp: clock.unix_timestamp,
            });
            return Err(SSSError::TransferBlocked.into());
        }
    }

    // Check recipient blacklist
    if let Some(recipient_bl) = &ctx.accounts.recipient_blacklist {
        if recipient_bl.is_active {
            emit!(TransferHookExecuted {
                mint: state.mint,
                source: ctx.accounts.source.key(),
                destination: ctx.accounts.destination.key(),
                amount,
                allowed: false,
                timestamp: clock.unix_timestamp,
            });
            return Err(SSSError::TransferBlocked.into());
        }
    }

    emit!(TransferHookExecuted {
        mint: state.mint,
        source: ctx.accounts.source.key(),
        destination: ctx.accounts.destination.key(),
        amount,
        allowed: true,
        timestamp: clock.unix_timestamp,
    });

    msg!("SSS: Transfer hook passed — {} tokens allowed", amount);
    Ok(())
}

/// Fallback handler for the transfer hook interface.
/// Routes SPI transfer-hook-interface instructions to our handler.
pub fn fallback_handler<'info>(
    _program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> Result<()> {
    // Check if this is the Execute instruction from the transfer hook interface
    let instruction_discriminator = &data[..8];
    
    // SPL Transfer Hook Interface "Execute" instruction discriminator
    let execute_discriminator: [u8; 8] = spl_transfer_hook_interface::instruction::ExecuteInstruction::SPL_DISCRIMINATOR_SLICE
        .try_into()
        .unwrap_or([0u8; 8]);

    if instruction_discriminator == execute_discriminator {
        msg!("SSS: Transfer hook execute called via fallback");
        // In a full implementation, deserialize accounts and call transfer_hook_handler
    }

    Ok(())
}

#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// Source token account (sender)
    /// CHECK: Validated by Token-2022 program
    pub source: AccountInfo<'info>,

    /// The mint
    /// CHECK: Validated by Token-2022 program
    pub mint: AccountInfo<'info>,

    /// Destination token account (recipient)
    /// CHECK: Validated by Token-2022 program
    pub destination: AccountInfo<'info>,

    /// Owner of the source account
    /// CHECK: Validated by Token-2022 program
    pub owner: AccountInfo<'info>,

    /// Extra account: stablecoin state
    #[account(
        seeds = [b"stablecoin", mint.key().as_ref()],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    /// Extra account: sender blacklist entry (optional — may not exist)
    pub sender_blacklist: Option<Account<'info, BlacklistEntry>>,

    /// Extra account: recipient blacklist entry (optional — may not exist)
    pub recipient_blacklist: Option<Account<'info, BlacklistEntry>>,
}
