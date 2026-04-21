# ERC-8040 Testnet Deployment Guide

This guide covers deploying the ERC-8040 ecosystem to Sepolia and Base Sepolia testnets.

## Prerequisites

Before deploying, ensure you have:

- **Foundry** installed ([Installation Guide](https://book.getfoundry.sh/getting-started/installation))
- **Testnet ETH** for gas fees:
  - Sepolia: [Sepolia Faucet](https://sepoliafaucet.com/)
  - Base Sepolia: [Base Sepolia Faucet](https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet)
- **RPC URL** from a provider like:
  - [Alchemy](https://www.alchemy.com/)
  - [Infura](https://www.infura.io/)
  - Public RPCs (slower, less reliable)
- **Block Explorer API Keys** (for contract verification):
  - Etherscan: [Get API Key](https://etherscan.io/myapikey)
  - Basescan: [Get API Key](https://basescan.org/myapikey)

## Quick Start

### 1. Setup Environment

Navigate to the contracts directory:
```bash
cd contracts
```

Copy the environment template:
```bash
cp .env.example .env
```

Edit `.env` and fill in your values:
```bash
# Your wallet private key (must have testnet ETH)
PRIVATE_KEY=0x...

# RPC endpoints
SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
BASE_SEPOLIA_RPC_URL=https://sepolia.base.org

# API keys for verification
ETHERSCAN_API_KEY=your_key_here
BASESCAN_API_KEY=your_key_here
```

⚠️ **Security Warning**: Never commit your actual private key! The `.env` file is gitignored by default.

### 2. Install Dependencies

Install OpenZeppelin contracts:
```bash
make install
```

### 3. Build Contracts

Compile the smart contracts:
```bash
make build
```

### 4. Deploy to Testnet

#### Deploy to Sepolia:
```bash
make deploy-sepolia
```

#### Deploy to Base Sepolia:
```bash
make deploy-base-sepolia
```

## Security Considerations

1. **Private Keys**: Never commit private keys to git
2. **Testnet Only**: These scripts are for testnet deployment only
3. **Ownership**: Transfer ownership to a multisig for production
4. **Audits**: Get professional audits before mainnet deployment

## License

MIT License - see [LICENSE](../LICENSE) for details
