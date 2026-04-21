// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/ERC8040.sol";
import "../src/ESGOracle.sol";
import "../src/ESGRegistry.sol";
import "../src/ERC8040Factory.sol";

/**
 * @title DeployTestnet
 * @notice Deployment script for testnet environments
 */
contract DeployTestnet is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        // 1. Deploy ESG Registry
        ESGRegistry registry = new ESGRegistry();
        console.log("ESGRegistry deployed:", address(registry));
        
        // 2. Deploy ESG Oracle
        ESGOracle oracle = new ESGOracle(address(registry));
        console.log("ESGOracle deployed:", address(oracle));
        
        // 3. Deploy ERC8040 Token
        ERC8040 token = new ERC8040(
            "AgroNet ESG Token",
            "AGRO",
            address(oracle)
        );
        console.log("ERC8040 deployed:", address(token));
        
        // 4. Deploy Factory
        ERC8040Factory factory = new ERC8040Factory(address(oracle));
        console.log("ERC8040Factory deployed:", address(factory));
        
        // 5. Setup initial provider (deployer)
        registry.registerProvider(msg.sender, "AgroNet Labs");
        oracle.setProvider(msg.sender, true);
        
        // 6. Set sample ESG scores for testing
        oracle.updateScore(msg.sender, 75, 80, 85);
        
        vm.stopBroadcast();
        
        // Output deployment addresses
        console.log("\n=== DEPLOYMENT COMPLETE ===");
        console.log("ESGRegistry:", address(registry));
        console.log("ESGOracle:", address(oracle));
        console.log("ERC8040:", address(token));
        console.log("ERC8040Factory:", address(factory));
    }
}
