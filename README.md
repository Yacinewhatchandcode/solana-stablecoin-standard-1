# 🪙 Solana Stablecoin Standard (SSS)

> **A modular, composable framework for creating stablecoins on Solana with regulatory compliance built-in.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Solana](https://img.shields.io/badge/Solana-Token--2022-9945FF)](https://spl.solana.com/token-2022)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-FF6B35)](https://anchor-lang.com)

## Architecture

```
┌─────────────────────────────────────────────────┐
│           Layer 3 — Standard Presets            │
│  ┌──────────────┐    ┌───────────────────────┐  │
│  │    SSS-1     │    │        SSS-2          │  │
│  │   Minimal    │    │     Compliant         │  │
│  │  Stablecoin  │    │     Stablecoin        │  │
│  │              │    │                       │  │
│  │ • Mint Auth  │    │ • All SSS-1 features  │  │
│  │ • Freeze Auth│    │ • Permanent Delegate  │  │
│  │ • Metadata   │    │ • Transfer Hook       │  │
│  │              │    │ • Blacklist (PDAs)     │  │
│  └──────────────┘    └───────────────────────┘  │
├─────────────────────────────────────────────────┤
│           Layer 2 — Modules                     │
│  ┌─────────────────────┐  ┌──────────────────┐  │
│  │  Compliance Module  │  │  Privacy Module  │  │
│  │  • Transfer Hook    │  │  • Confidential  │  │
│  │  • Blacklist PDAs   │  │    Transfers     │  │
│  │  • Permanent Del.   │  │  • Allowlists    │  │
│  │  • Token Seizure    │  │                  │  │
│  └─────────────────────┘  └──────────────────┘  │
├─────────────────────────────────────────────────┤
│           Layer 1 — Base SDK                    │
│  • Token-2022 Mint (with extensions)            │
│  • Role Management (Mint/Freeze/Compliance)     │
│  • TypeScript SDK + Admin CLI                   │
│  • Event-based Audit Trail                      │
└─────────────────────────────────────────────────┘
```

## Presets

### SSS-1 — Minimal Stablecoin

For simple use cases: internal tokens, DAO treasuries, ecosystem settlement.

| Feature | Status |
|---------|--------|
| Mint authority | ✅ |
| Freeze authority | ✅ |
| Token metadata | ✅ |
| Permanent delegate | ❌ |
| Transfer hook | ❌ |
| Blacklist enforcement | ❌ |

**When to use:** Compliance is reactive (freeze accounts as needed).

### SSS-2 — Compliant Stablecoin

For regulated, institutional-grade tokens (USDC/USDT class).

| Feature | Status |
|---------|--------|
| All SSS-1 features | ✅ |
| Permanent delegate | ✅ |
| Transfer hook (blacklist check) | ✅ |
| On-chain blacklist PDAs | ✅ |
| Token seizure | ✅ |
| Default accounts frozen | ✅ |

**When to use:** Regulators expect active enforcement (no gaps).

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.75+)
- [Solana CLI](https://docs.solanalabs.com/cli/install) (1.18+)
- [Anchor](https://anchor-lang.com/docs/installation) (0.30+)
- [Node.js](https://nodejs.org) (20+)

### Build

```bash
# Clone
git clone https://github.com/Yacinewhatchandcode/solana-stablecoin-standard
cd solana-stablecoin-standard

# Build the Anchor program
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### CLI Usage

```bash
# Install CLI
npm install -g @sss/cli

# Create an SSS-1 stablecoin
sys create-token --preset sss-1 --name "My Dollar" --symbol MYD

# Create an SSS-2 compliant stablecoin
sys create-token --preset sss-2 --name "Regulated USD" --symbol RUSD

# Mint tokens
sys mint --mint <MINT_KEY> --to <TOKEN_ACCOUNT> --amount 1000000

# Blacklist an address (SSS-2)
sys blacklist add --mint <MINT_KEY> --address <WALLET>

# Seize tokens from blacklisted account (SSS-2)
sys seize --mint <MINT_KEY> --from <TOKEN_ACCOUNT> --amount 500000

# View stablecoin info
sys info --mint <MINT_KEY>
```

### SDK Usage

```typescript
import { SSSClient, createSSS1Config, createSSS2Config } from "@sss/sdk";

// Initialize client
const client = new SSSClient(provider);

// Create SSS-1 stablecoin
const config = createSSS1Config("My Dollar", "MYD");
const { mint } = await client.initialize(config);

// Mint tokens
await client.mintTo(mint, recipientTokenAccount, 1_000_000);

// For SSS-2: full compliance flow
const config2 = createSSS2Config("Regulated USD", "RUSD");
const { mint: mint2 } = await client.initialize(config2);

// Blacklist a suspicious address
await client.blacklistAdd(mint2, suspiciousAddress);

// Seize tokens via permanent delegate
await client.seizeTokens(mint2, targetAccount, treasury, targetOwner, 500_000);
```

## On-Chain Program

The program uses a single `StablecoinConfig` struct to support both presets:

```rust
pub struct StablecoinConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    // SSS-2 compliance flags
    pub enable_permanent_delegate: bool,
    pub enable_transfer_hook: bool,
    pub default_account_frozen: bool,
}
```

### Instructions

| Instruction | SSS-1 | SSS-2 | Description |
|-------------|-------|-------|-------------|
| `initialize` | ✅ | ✅ | Create a new stablecoin |
| `mint_to` | ✅ | ✅ | Mint tokens |
| `burn` | ✅ | ✅ | Burn tokens |
| `transfer` | ✅ | ✅ | Transfer tokens |
| `freeze_account` | ✅ | ✅ | Freeze a token account |
| `thaw_account` | ✅ | ✅ | Unfreeze a token account |
| `blacklist_add` | ❌ | ✅ | Add address to blacklist |
| `blacklist_remove` | ❌ | ✅ | Remove from blacklist |
| `seize_tokens` | ❌ | ✅ | Seize via permanent delegate |
| `update_role` | ✅ | ✅ | Update role assignments |
| `transfer_hook` | ❌ | ✅ | Blacklist check on every transfer |

### Role Management

| Role | Permission | SSS-1 | SSS-2 |
|------|-----------|-------|-------|
| `MINT_AUTHORITY` | Mint new tokens | ✅ | ✅ |
| `FREEZE_AUTHORITY` | Freeze/thaw accounts | ✅ | ✅ |
| `COMPLIANCE_OFFICER` | Manage blacklist, seize tokens | ❌ | ✅ |

### Events (Audit Trail)

Every operation emits a structured event for compliance auditing:

- `StablecoinInitialized` — Token creation
- `TokensMinted` / `TokensBurned` — Supply changes
- `TokensTransferred` — Transfers
- `AccountFrozenEvent` / `AccountThawedEvent` — Freeze operations
- `AddressBlacklisted` / `AddressUnblacklisted` — Blacklist changes
- `TokensSeized` — Enforcement actions
- `RoleUpdated` — Authority changes
- `TransferHookExecuted` — Per-transfer compliance checks

## Security

- ✅ Role-based access control (RBAC) for all privileged operations
- ✅ Feature gating: SSS-2 operations rejected on SSS-1 tokens
- ✅ Input validation: amount > 0, name/symbol length limits
- ✅ Arithmetic overflow protection (checked_add/checked_sub)
- ✅ PDA-based blacklist entries (cannot be forged)
- ✅ Complete audit trail via events

## Tests

```bash
# Unit + integration tests
anchor test

# Specific test suite
anchor test -- --grep "SSS-2"

# Fuzz testing (requires Trident)
trident fuzz
```

## Project Structure

```
solana-stablecoin-standard/
├── programs/sss/
│   └── src/
│       ├── lib.rs              # Program entry point
│       ├── state.rs            # Account definitions
│       ├── errors.rs           # Error codes
│       ├── events.rs           # Audit events
│       └── instructions/
│           ├── initialize.rs   # Token creation
│           ├── mint.rs         # Minting
│           ├── burn.rs         # Burning
│           ├── transfer.rs     # Transfers
│           ├── freeze.rs       # Freeze/thaw
│           ├── compliance.rs   # Blacklist + seizure
│           ├── roles.rs        # Role management
│           └── hook.rs         # Transfer hook
├── sdk/                        # TypeScript SDK
│   └── src/index.ts
├── cli/                        # Admin CLI
│   └── src/index.ts
├── tests/                      # Integration tests
│   └── sss.ts
├── docs/                       # Documentation
├── Anchor.toml
├── Cargo.toml
└── README.md
```

## Roadmap

- [x] SSS-1: Minimal Stablecoin
- [x] SSS-2: Compliant Stablecoin
- [x] TypeScript SDK
- [x] Admin CLI
- [x] Role management
- [x] Transfer hook (blacklist enforcement)
- [x] Event-based audit trail
- [ ] SSS-3: Private Stablecoin (confidential transfers)
- [ ] Oracle integration (price feeds)
- [ ] TUI (terminal UI)
- [ ] Example frontend (React)

## License

MIT — See [LICENSE](LICENSE) for details.

## Author

**Yacine Benhamou** — [Prime.AI](https://prime-ai.fr)

- Email: yacine@prime-ai.fr
- Twitter: [@yace19ai](https://twitter.com/yace19ai)
- GitHub: [Yacinewhatchandcode](https://github.com/Yacinewhatchandcode)
