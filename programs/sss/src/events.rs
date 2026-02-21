use anchor_lang::prelude::*;

#[event]
pub struct StablecoinInitialized {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub preset: String,
    pub timestamp: i64,
}

#[event]
pub struct TokensMinted {
    pub mint: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub minted_by: Pubkey,
    pub total_minted: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensBurned {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub amount: u64,
    pub burned_by: Pubkey,
    pub total_burned: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensTransferred {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct AccountFrozenEvent {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub frozen_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AccountThawedEvent {
    pub mint: Pubkey,
    pub account: Pubkey,
    pub thawed_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AddressBlacklisted {
    pub mint: Pubkey,
    pub address: Pubkey,
    pub added_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AddressUnblacklisted {
    pub mint: Pubkey,
    pub address: Pubkey,
    pub removed_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensSeized {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub amount: u64,
    pub seized_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct RoleUpdated {
    pub mint: Pubkey,
    pub role: String,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TransferHookExecuted {
    pub mint: Pubkey,
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
    pub allowed: bool,
    pub timestamp: i64,
}
