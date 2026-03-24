// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title MockUSDC
 * @notice Mock USDC token for testing
 */
contract MockUSDC is ERC20 {
    constructor() ERC20("Mock USDC", "mUSDC") {
        _mint(msg.sender, 1_000_000_000 * 10**6); // 1B USDC
    }
    
    function decimals() public pure override returns (uint8) {
        return 6;
    }
    
    /**
     * @notice Faucet for testing - mint tokens to any address
     * @dev Intentionally public and unrestricted for testnet use
     * @param to Recipient address
     * @param amount Amount to mint
     */
    function faucet(address to, uint256 amount) external {
        _mint(to, amount);
    }
}
