# 📋 Superteam Earn Submission — Solana Stablecoin Standard

## Submission Link
PR to: https://github.com/solanabr/solana-stablecoin-standard

## Repository
https://github.com/Yacinewhatchandcode/solana-stablecoin-standard

---

## Submission Description

### What I Built

A complete, modular Solana stablecoin framework implementing both SSS-1 (Minimal Stablecoin) and SSS-2 (Compliant Stablecoin) presets as defined in the bounty specification.

### Architecture

**Three-layer design:**

1. **Layer 1 — Base SDK**: Token-2022 mint with extensions, role management system, TypeScript SDK
2. **Layer 2 — Compliance Module**: Transfer hook (blacklist enforcement), permanent delegate (token seizure), blacklist PDAs
3. **Layer 3 — Standard Presets**: SSS-1 and SSS-2 configurable via a single `StablecoinConfig` struct

### On-Chain Program (Anchor)

Single configurable Anchor program supporting both presets via initialization parameters:

```rust
pub struct StablecoinConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub enable_permanent_delegate: bool,  // SSS-2
    pub enable_transfer_hook: bool,       // SSS-2
    pub default_account_frozen: bool,     // SSS-2
}
```

**Instructions implemented:**
- `initialize` — Create SSS-1 or SSS-2 stablecoin
- `mint_to` — Mint tokens (role-checked)
- `burn` — Burn tokens
- `transfer` — Transfer with automatic hook enforcement
- `freeze_account` / `thaw_account` — Freeze operations
- `blacklist_add` / `blacklist_remove` — PDA-based blacklist (SSS-2)
- `seize_tokens` — Via permanent delegate (SSS-2)  
- `update_role` — MINT_AUTHORITY, FREEZE_AUTHORITY, COMPLIANCE_OFFICER
- `transfer_hook` — Automatic blacklist check on every transfer (SSS-2)

### TypeScript SDK

```typescript
import { SSSClient, createSSS1Config, createSSS2Config } from "@sss/sdk";

const client = new SSSClient(provider);

// SSS-1: Minimal
const { mint } = await client.initialize(createSSS1Config("My Dollar", "MYD"));
await client.mintTo(mint, recipient, 1_000_000);

// SSS-2: Compliant (full flow)
const { mint: mint2 } = await client.initialize(createSSS2Config("Regulated USD", "RUSD"));
await client.blacklistAdd(mint2, suspiciousAddress);
await client.seizeTokens(mint2, targetAccount, treasury, targetOwner, 500_000);
```

### Admin CLI

```bash
sys create-token --preset sss-2 --name "Regulated USD" --symbol RUSD
sys mint --mint <KEY> --to <ACCOUNT> --amount 1000000
sys blacklist add --mint <KEY> --address <WALLET>
sys seize --mint <KEY> --from <ACCOUNT> --amount 500000
sys info --mint <KEY>
```

### Security Features

- ✅ Role-based access control (RBAC) for all privileged operations
- ✅ Feature gating: SSS-2 ops rejected on SSS-1 tokens
- ✅ Input validation: amount > 0, name/symbol length limits
- ✅ Arithmetic overflow protection (checked_add/checked_sub)
- ✅ PDA-based blacklist entries (cannot be forged)
- ✅ Complete audit trail via 11 event types

### Event-Based Audit Trail

Every operation emits structured events for compliance:
- `StablecoinInitialized`, `TokensMinted`, `TokensBurned`, `TokensTransferred`
- `AccountFrozenEvent`, `AccountThawedEvent`
- `AddressBlacklisted`, `AddressUnblacklisted`, `TokensSeized`
- `RoleUpdated`, `TransferHookExecuted`

### Tests

Integration test suite covering:
1. SSS-1 lifecycle: Create → Mint → Transfer → Burn
2. SSS-2 compliance: Create → Mint → Blacklist → Transfer (blocked) → Seize
3. Role management: Authority updates for all 3 roles
4. Edge cases: Invalid amounts, unauthorized callers, double blacklisting

### Evaluation Criteria Coverage

| Criteria | Weight | Implementation |
|----------|--------|---------------|
| SDK Design & Modularity | 20% | 3-layer architecture, clean separation |
| Completeness | 20% | All SSS-1 + SSS-2 instructions |
| Code Quality | 20% | Anchor best practices, documented |
| Security | 15% | RBAC, feature gating, overflow protection |
| Authority | 20% | 21 repos deployed, multi-agent AI background |
| Usability | 5% | Intuitive CLI + TypeScript SDK |

---

## Author

**Yacine Benhamou** — [Prime.AI](https://prime-ai.fr)
- GitHub: [Yacinewhatchandcode](https://github.com/Yacinewhatchandcode)
- Email: yacine@prime-ai.fr
- Twitter: [@yace19ai](https://twitter.com/yace19ai)
