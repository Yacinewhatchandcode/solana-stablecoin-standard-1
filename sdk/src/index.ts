/**
 * Solana Stablecoin Standard (SSS) SDK
 *
 * TypeScript client for creating and managing SSS-1 and SSS-2 stablecoins.
 *
 * @module @sss/sdk
 * @author Yacine Benhamou <yacine@prime-ai.fr>
 * @license MIT
 */

import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Connection, Transaction } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction } from "@solana/spl-token";

// ──────────────────────────────────────────────────────
// Types
// ──────────────────────────────────────────────────────

/** SSS preset type */
export type SSSPreset = "SSS-1" | "SSS-2";

/** Configuration for creating a new stablecoin */
export interface StablecoinConfig {
  name: string;
  symbol: string;
  uri: string;
  decimals: number;
  enablePermanentDelegate: boolean;
  enableTransferHook: boolean;
  defaultAccountFrozen: boolean;
}

/** Role types for role management */
export enum Role {
  MintAuthority = "MintAuthority",
  FreezeAuthority = "FreezeAuthority",
  ComplianceOfficer = "ComplianceOfficer",
}

/** On-chain stablecoin state */
export interface StablecoinState {
  mint: PublicKey;
  authority: PublicKey;
  mintAuthority: PublicKey;
  freezeAuthority: PublicKey;
  complianceOfficer: PublicKey;
  isSss2: boolean;
  permanentDelegateEnabled: boolean;
  transferHookEnabled: boolean;
  defaultAccountFrozen: boolean;
  totalMinted: BN;
  totalBurned: BN;
  createdAt: BN;
  updatedAt: BN;
  name: string;
  symbol: string;
  decimals: number;
}

// ──────────────────────────────────────────────────────
// Presets
// ──────────────────────────────────────────────────────

/** SSS-1: Minimal Stablecoin preset */
export function createSSS1Config(
  name: string,
  symbol: string,
  uri: string = "",
  decimals: number = 6
): StablecoinConfig {
  return {
    name,
    symbol,
    uri,
    decimals,
    enablePermanentDelegate: false,
    enableTransferHook: false,
    defaultAccountFrozen: false,
  };
}

/** SSS-2: Compliant Stablecoin preset */
export function createSSS2Config(
  name: string,
  symbol: string,
  uri: string = "",
  decimals: number = 6
): StablecoinConfig {
  return {
    name,
    symbol,
    uri,
    decimals,
    enablePermanentDelegate: true,
    enableTransferHook: true,
    defaultAccountFrozen: true,
  };
}

// ──────────────────────────────────────────────────────
// PDA Derivation
// ──────────────────────────────────────────────────────

const PROGRAM_ID = new PublicKey("SSS111111111111111111111111111111111111111");

/** Derive the stablecoin state PDA */
export function findStablecoinStatePDA(
  mint: PublicKey,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("stablecoin"), mint.toBuffer()],
    programId
  );
}

/** Derive a blacklist entry PDA */
export function findBlacklistEntryPDA(
  mint: PublicKey,
  address: PublicKey,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("blacklist"), mint.toBuffer(), address.toBuffer()],
    programId
  );
}

// ──────────────────────────────────────────────────────
// Client
// ──────────────────────────────────────────────────────

/**
 * SSS Client — main entry point for interacting with the Solana Stablecoin Standard.
 *
 * @example
 * ```typescript
 * const client = new SSSClient(provider);
 *
 * // Create an SSS-1 stablecoin
 * const config = createSSS1Config("My Dollar", "MYD");
 * const { mint, stablecoinState } = await client.initialize(config);
 *
 * // Mint tokens
 * await client.mintTo(mint, recipientTokenAccount, 1_000_000);
 *
 * // For SSS-2: blacklist and seize
 * const config2 = createSSS2Config("Regulated USD", "RUSD");
 * const { mint: mint2 } = await client.initialize(config2);
 * await client.blacklistAdd(mint2, suspiciousAddress);
 * await client.seizeTokens(mint2, targetAccount, treasuryAccount, 500_000);
 * ```
 */
export class SSSClient {
  private program: Program;
  private provider: AnchorProvider;

  constructor(provider: AnchorProvider, programId: PublicKey = PROGRAM_ID) {
    this.provider = provider;
    // In production, load the IDL from the chain or from a file
    this.program = {} as Program; // Placeholder — requires IDL
  }

  /** Get the current connection */
  get connection(): Connection {
    return this.provider.connection;
  }

  /** Get the wallet public key */
  get wallet(): PublicKey {
    return this.provider.wallet.publicKey;
  }

  // ── Token Lifecycle ──

  /** Initialize a new stablecoin with the given configuration */
  async initialize(config: StablecoinConfig): Promise<{
    mint: PublicKey;
    stablecoinState: PublicKey;
    txSignature: string;
  }> {
    const mint = Keypair.generate();
    const [stablecoinState] = findStablecoinStatePDA(mint.publicKey);

    const txSignature = await this.program.methods
      .initialize({
        name: config.name,
        symbol: config.symbol,
        uri: config.uri,
        decimals: config.decimals,
        enablePermanentDelegate: config.enablePermanentDelegate,
        enableTransferHook: config.enableTransferHook,
        defaultAccountFrozen: config.defaultAccountFrozen,
      })
      .accounts({
        authority: this.wallet,
        mint: mint.publicKey,
        stablecoinState,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mint])
      .rpc();

    return { mint: mint.publicKey, stablecoinState, txSignature };
  }

  /** Mint tokens to a token account */
  async mintTo(
    mint: PublicKey,
    tokenAccount: PublicKey,
    amount: number | BN
  ): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const amountBN = typeof amount === "number" ? new BN(amount) : amount;

    return this.program.methods
      .mintTo(amountBN)
      .accounts({
        mintAuthority: this.wallet,
        mint,
        tokenAccount,
        stablecoinState,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  }

  /** Burn tokens from a token account */
  async burn(
    mint: PublicKey,
    tokenAccount: PublicKey,
    amount: number | BN
  ): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const amountBN = typeof amount === "number" ? new BN(amount) : amount;

    return this.program.methods
      .burn(amountBN)
      .accounts({
        owner: this.wallet,
        mint,
        tokenAccount,
        stablecoinState,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  }

  /** Transfer tokens between accounts */
  async transfer(
    mint: PublicKey,
    from: PublicKey,
    to: PublicKey,
    amount: number | BN
  ): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const amountBN = typeof amount === "number" ? new BN(amount) : amount;

    return this.program.methods
      .transfer(amountBN)
      .accounts({
        owner: this.wallet,
        mint,
        from,
        to,
        stablecoinState,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  }

  // ── Freeze Operations ──

  /** Freeze a token account */
  async freezeAccount(mint: PublicKey, tokenAccount: PublicKey): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);

    return this.program.methods
      .freezeAccount()
      .accounts({
        freezeAuthority: this.wallet,
        mint,
        tokenAccount,
        stablecoinState,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  }

  /** Thaw (unfreeze) a token account */
  async thawAccount(mint: PublicKey, tokenAccount: PublicKey): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);

    return this.program.methods
      .thawAccount()
      .accounts({
        freezeAuthority: this.wallet,
        mint,
        tokenAccount,
        stablecoinState,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  }

  // ── SSS-2: Compliance Operations ──

  /** Add an address to the blacklist (SSS-2 only) */
  async blacklistAdd(mint: PublicKey, address: PublicKey): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const [blacklistEntry] = findBlacklistEntryPDA(mint, address);

    return this.program.methods
      .blacklistAdd(address)
      .accounts({
        complianceOfficer: this.wallet,
        mint,
        stablecoinState,
        blacklistEntry,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  /** Remove an address from the blacklist (SSS-2 only) */
  async blacklistRemove(mint: PublicKey, address: PublicKey): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const [blacklistEntry] = findBlacklistEntryPDA(mint, address);

    return this.program.methods
      .blacklistRemove(address)
      .accounts({
        complianceOfficer: this.wallet,
        mint,
        stablecoinState,
        blacklistEntry,
      })
      .rpc();
  }

  /** Seize tokens from a blacklisted account (SSS-2 only) */
  async seizeTokens(
    mint: PublicKey,
    targetAccount: PublicKey,
    treasuryAccount: PublicKey,
    targetOwner: PublicKey,
    amount: number | BN
  ): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const [blacklistEntry] = findBlacklistEntryPDA(mint, targetOwner);
    const amountBN = typeof amount === "number" ? new BN(amount) : amount;

    return this.program.methods
      .seizeTokens(amountBN)
      .accounts({
        complianceOfficer: this.wallet,
        mint,
        stablecoinState,
        blacklistEntry,
        targetAccount,
        treasuryAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  }

  // ── Role Management ──

  /** Update a role assignment */
  async updateRole(
    mint: PublicKey,
    role: Role,
    newAuthority: PublicKey
  ): Promise<string> {
    const [stablecoinState] = findStablecoinStatePDA(mint);
    const roleArg = { [role.toLowerCase()]: {} };

    return this.program.methods
      .updateRole(roleArg, newAuthority)
      .accounts({
        authority: this.wallet,
        mint,
        stablecoinState,
      })
      .rpc();
  }

  // ── Query Methods ──

  /** Fetch the on-chain stablecoin state */
  async getStablecoinState(mint: PublicKey): Promise<StablecoinState> {
    const [stablecoinStatePDA] = findStablecoinStatePDA(mint);
    return this.program.account.stablecoinState.fetch(stablecoinStatePDA) as Promise<StablecoinState>;
  }

  /** Check if an address is blacklisted */
  async isBlacklisted(mint: PublicKey, address: PublicKey): Promise<boolean> {
    const [blacklistEntryPDA] = findBlacklistEntryPDA(mint, address);
    try {
      const entry = await this.program.account.blacklistEntry.fetch(blacklistEntryPDA);
      return (entry as any).isActive;
    } catch {
      return false;
    }
  }

  /** Get the net supply (minted - burned) */
  async getNetSupply(mint: PublicKey): Promise<BN> {
    const state = await this.getStablecoinState(mint);
    return state.totalMinted.sub(state.totalBurned);
  }
}

// ──────────────────────────────────────────────────────
// Exports
// ──────────────────────────────────────────────────────

export {
  SSSClient,
  createSSS1Config,
  createSSS2Config,
  findStablecoinStatePDA,
  findBlacklistEntryPDA,
  PROGRAM_ID,
};
