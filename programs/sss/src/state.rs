use anchor_lang::prelude::*;

/// Configuration for creating a new stablecoin
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct StablecoinConfig {
    /// Human-readable name of the stablecoin (e.g., "USD Coin")
    pub name: String,
    /// Symbol (e.g., "USDC")
    pub symbol: String,
    /// Metadata URI (e.g., link to JSON with logo, description)
    pub uri: String,
    /// Decimal places (typically 6 for USD stablecoins)
    pub decimals: u8,
    // ── SSS-2 compliance flags ──
    /// Enable permanent delegate (allows seizing tokens from any account)
    pub enable_permanent_delegate: bool,
    /// Enable transfer hook (checks blacklist on every transfer)
    pub enable_transfer_hook: bool,
    /// Whether new accounts default to frozen (SSS-2: usually true)
    pub default_account_frozen: bool,
}

impl StablecoinConfig {
    /// Returns true if this is an SSS-2 compliant stablecoin
    pub fn is_sss2(&self) -> bool {
        self.enable_permanent_delegate && self.enable_transfer_hook
    }

    /// Returns the preset name
    pub fn preset_name(&self) -> &str {
        if self.is_sss2() {
            "SSS-2"
        } else {
            "SSS-1"
        }
    }
}

/// On-chain state for a deployed stablecoin
#[account]
#[derive(Debug)]
pub struct StablecoinState {
    /// The mint address for this stablecoin
    pub mint: Pubkey,
    /// Authority who deployed this stablecoin
    pub authority: Pubkey,
    /// Mint authority (can mint new tokens)
    pub mint_authority: Pubkey,
    /// Freeze authority (can freeze/thaw accounts)
    pub freeze_authority: Pubkey,
    /// Compliance officer (SSS-2: manages blacklist, can seize tokens)
    pub compliance_officer: Pubkey,
    /// Whether this is SSS-2 compliant
    pub is_sss2: bool,
    /// Permanent delegate enabled
    pub permanent_delegate_enabled: bool,
    /// Transfer hook enabled
    pub transfer_hook_enabled: bool,
    /// Default account frozen on creation
    pub default_account_frozen: bool,
    /// Total supply minted (tracked for audit)
    pub total_minted: u64,
    /// Total supply burned (tracked for audit)
    pub total_burned: u64,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
    /// Name
    pub name: String,
    /// Symbol  
    pub symbol: String,
    /// Decimals
    pub decimals: u8,
}

impl StablecoinState {
    pub const MAX_NAME_LEN: usize = 32;
    pub const MAX_SYMBOL_LEN: usize = 10;
    pub const SPACE: usize = 8  // discriminator
        + 32  // mint
        + 32  // authority
        + 32  // mint_authority
        + 32  // freeze_authority
        + 32  // compliance_officer
        + 1   // is_sss2
        + 1   // permanent_delegate_enabled
        + 1   // transfer_hook_enabled
        + 1   // default_account_frozen
        + 8   // total_minted
        + 8   // total_burned
        + 8   // created_at
        + 8   // updated_at
        + 1   // bump
        + 4 + Self::MAX_NAME_LEN   // name (string prefix + data)
        + 4 + Self::MAX_SYMBOL_LEN // symbol (string prefix + data)
        + 1;  // decimals

    pub fn net_supply(&self) -> u64 {
        self.total_minted.saturating_sub(self.total_burned)
    }
}

/// Blacklist entry — stores a blacklisted address for SSS-2 stablecoins
#[account]
#[derive(Debug)]
pub struct BlacklistEntry {
    /// The stablecoin this blacklist entry belongs to
    pub stablecoin: Pubkey,
    /// The blacklisted wallet address
    pub blacklisted_address: Pubkey,
    /// Who added this entry
    pub added_by: Pubkey,
    /// Timestamp when added
    pub added_at: i64,
    /// Whether this entry is active
    pub is_active: bool,
    /// Bump seed for PDA
    pub bump: u8,
}

impl BlacklistEntry {
    pub const SPACE: usize = 8  // discriminator
        + 32  // stablecoin
        + 32  // blacklisted_address
        + 32  // added_by
        + 8   // added_at
        + 1   // is_active
        + 1;  // bump
}

/// Role types for role management
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum Role {
    MintAuthority,
    FreezeAuthority,
    ComplianceOfficer,
}
