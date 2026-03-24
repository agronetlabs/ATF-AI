-include .env

.PHONY: all test deploy-sepolia deploy-base-sepolia verify clean

# Build
build:
forge build

# Test
test:
forge test -vvv

# Deploy to Sepolia
deploy-sepolia:
forge script script/DeployTestnet.s.sol:DeployTestnet \
--rpc-url $(SEPOLIA_RPC_URL) \
--broadcast \
--verify \
-vvvv

# Deploy to Base Sepolia
deploy-base-sepolia:
forge script script/DeployTestnet.s.sol:DeployTestnet \
--rpc-url $(BASE_SEPOLIA_RPC_URL) \
--broadcast \
--verify \
-vvvv

# Verify contracts on Sepolia
verify-sepolia:
@echo "Verifying contract at $(CONTRACT_ADDRESS)"
forge verify-contract $(CONTRACT_ADDRESS) $(CONTRACT_PATH) \
--chain sepolia \
--etherscan-api-key $(ETHERSCAN_API_KEY)

# Verify contracts on Base Sepolia
verify-base-sepolia:
@echo "Verifying contract at $(CONTRACT_ADDRESS)"
forge verify-contract $(CONTRACT_ADDRESS) $(CONTRACT_PATH) \
--chain base-sepolia \
--etherscan-api-key $(BASESCAN_API_KEY)

# Gas report
gas:
forge test --gas-report

# Coverage
coverage:
forge coverage

# Format code
format:
forge fmt

# Clean build artifacts
clean:
forge clean

# Install dependencies
install:
forge install OpenZeppelin/openzeppelin-contracts --no-commit

# Update dependencies
update:
forge update

# Snapshot gas usage
snapshot:
forge snapshot

# Help
help:
@echo "Available targets:"
@echo "  build              - Build contracts"
@echo "  test               - Run tests"
@echo "  deploy-sepolia     - Deploy to Sepolia testnet"
@echo "  deploy-base-sepolia - Deploy to Base Sepolia testnet"
@echo "  verify-sepolia     - Verify contract on Sepolia"
@echo "  verify-base-sepolia - Verify contract on Base Sepolia"
@echo "  gas                - Generate gas report"
@echo "  coverage           - Generate coverage report"
@echo "  format             - Format Solidity code"
@echo "  clean              - Clean build artifacts"
@echo "  install            - Install dependencies"
@echo "  update             - Update dependencies"
@echo "  snapshot           - Snapshot gas usage"
