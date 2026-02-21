use anchor_lang::prelude::*;

#[error_code]
pub enum SSSError {
    #[msg("Name too long (max 32 characters)")]
    NameTooLong,

    #[msg("Symbol too long (max 10 characters)")]
    SymbolTooLong,

    #[msg("Unauthorized: caller lacks the required role")]
    Unauthorized,

    #[msg("SSS-2 feature required but not enabled for this stablecoin")]
    SSS2Required,

    #[msg("Address is already blacklisted")]
    AlreadyBlacklisted,

    #[msg("Address is not blacklisted")]
    NotBlacklisted,

    #[msg("Transfer blocked: sender or recipient is blacklisted")]
    TransferBlocked,

    #[msg("Cannot seize tokens from a non-blacklisted account")]
    SeizeNotBlacklisted,

    #[msg("Insufficient balance for operation")]
    InsufficientBalance,

    #[msg("Permanent delegate not enabled for this stablecoin")]
    PermanentDelegateNotEnabled,

    #[msg("Transfer hook not enabled for this stablecoin")]
    TransferHookNotEnabled,

    #[msg("Invalid amount: must be greater than zero")]
    InvalidAmount,

    #[msg("Account is frozen")]
    AccountFrozen,

    #[msg("Arithmetic overflow")]
    Overflow,

    #[msg("Invalid role for this operation")]
    InvalidRole,
}
