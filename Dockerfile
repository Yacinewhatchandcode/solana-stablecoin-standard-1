# ── Solana Stablecoin Standard — Development Container ──
# Provides Rust, Solana CLI, Anchor, and Node.js for building and testing

FROM rust:1.78-slim-bookworm AS base

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    curl \
    git \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 LTS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.anza.xyz/v1.18.26/install)" \
    && echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> /etc/profile.d/solana.sh
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor avm --force \
    && avm install 0.30.1 \
    && avm use 0.30.1
ENV PATH="/root/.avm/bin:${PATH}"

# Set working directory
WORKDIR /workspace

# Copy Cargo workspace files first (for caching)
COPY Cargo.toml Anchor.toml ./
COPY programs/sss/Cargo.toml programs/sss/Cargo.toml

# Copy source code
COPY programs/ programs/
COPY sdk/ sdk/
COPY cli/ cli/
COPY tests/ tests/

# Install SDK dependencies
RUN cd sdk && npm install 2>/dev/null || true

# Configure Solana for localnet
RUN solana-keygen new --no-bip39-passphrase -o /root/.config/solana/id.json 2>/dev/null || true \
    && solana config set --url localhost

# Build the program
RUN anchor build

# Expose Solana validator port
EXPOSE 8899 8900

# Default: run tests
CMD ["anchor", "test"]
