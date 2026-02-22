#!/bin/bash
# ── Deploy SSS Program to Solana Devnet ──
# Prerequisites: solana-cli, anchor-cli installed

set -e

echo "🌐 Solana Stablecoin Standard — Devnet Deployment"
echo "================================================="

# Configure for Devnet
solana config set --url devnet
echo "✅ Configured for Devnet"

# Generate keypair if needed
if [ ! -f ~/.config/solana/id.json ]; then
    solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
    echo "🔑 Generated new deployment keypair"
fi

DEPLOYER=$(solana address)
echo "📍 Deployer: $DEPLOYER"

# Airdrop SOL for deployment
echo "💰 Requesting airdrop..."
solana airdrop 2 || echo "⚠️  Airdrop failed — ensure devnet has capacity or fund manually"
sleep 2
solana airdrop 2 || true
sleep 2

BALANCE=$(solana balance | awk '{print $1}')
echo "💰 Balance: $BALANCE SOL"

# Build
echo "🔨 Building program..."
anchor build

# Deploy
echo "🚀 Deploying to Devnet..."
anchor deploy --provider.cluster devnet

# Get program ID
PROGRAM_ID=$(solana program show --programs | grep sss | awk '{print $1}')
echo ""
echo "================================================="
echo "✅ DEPLOYMENT SUCCESSFUL"
echo "================================================="
echo "Program ID: $PROGRAM_ID"
echo "Explorer:   https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
echo "Deployer:   $DEPLOYER"
echo ""

# Run a smoke test
echo "🧪 Running smoke test..."
anchor test --skip-local-validator --provider.cluster devnet 2>&1 | head -20

echo ""
echo "📋 Example usage:"
echo "  solana config set --url devnet"
echo "  # Use the SDK or CLI to interact with the deployed program"
echo "  # See README.md for full instructions"
