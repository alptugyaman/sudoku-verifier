# Environment Setup for Succinct Prover Network

## Required Environment Variables

Create a `.env` file in the `sudoku-verifier` directory with the following configuration:

```env
# SP1 Configuration
# Set to "network" to use Succinct Prover Network, "local" for local proving
SP1_PROVER=network

# Succinct Prover Network Configuration (Sepolia Testnet)
# Replace with your actual private key that has been whitelisted for the network
NETWORK_PRIVATE_KEY=your_sepolia_private_key_here

# Prover Network RPC URL (Sepolia)
NETWORK_RPC_URL=https://rpc-production.succinct.xyz

# Logging level
RUST_LOG=info

# API Configuration
API_PORT=3000
```

## Setup Steps

### 1. Get Invite Code
- Visit [https://testnet.succinct.xyz](https://testnet.succinct.xyz)
- Get an invite code through X (Twitter)
- Only 1,000 spots available for Level 1: Crisis of Trust

### 2. Setup Wallet
- Create a fresh Ethereum wallet for Sepolia testnet
- Get some Sepolia ETH from faucets
- This wallet's private key will be used for the `NETWORK_PRIVATE_KEY`

### 3. Deposit USDC
- Deposit $10 USDC to the Succinct Prover Network for proof generation costs
- This covers multiple proof generations (~$0.01-0.10 per proof)

### 4. Configure Private Key
Replace `your_sepolia_private_key_here` in the `.env` file with your actual private key:

```env
NETWORK_PRIVATE_KEY=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
```

## Usage

### Start with Network Prover
```bash
SP1_PROVER=network NETWORK_PRIVATE_KEY=0x... cargo run --release -p sudoku-script
```

### Start with Local Prover (Fallback)
```bash
SP1_PROVER=local cargo run --release -p sudoku-script
```

## Frontend Configuration

In the `succinct-sudoku` project, update the private key in `src/lib/prover-network.ts`:

```typescript
const PROVER_NETWORK_CONFIG = {
    // Replace with your actual private key
    privateKey: 'your_sepolia_private_key_here',
    // ... other config
};
```

## Important Notes

1. **Security**: In production, never hardcode private keys. Use proper environment variable management.
2. **Testnet Only**: This configuration is for Sepolia testnet only.
3. **Costs**: Each proof generation costs a small amount of USDC (~$0.01-0.10).
4. **Invite Required**: You need an invite code to access the Succinct Prover Network testnet. 