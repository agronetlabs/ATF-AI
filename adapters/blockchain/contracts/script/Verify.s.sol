// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";

/**
 * @title Verify
 * @notice Script to verify contracts on block explorers
 * @dev Usage:
 *   forge script script/Verify.s.sol:Verify \
 *     --rpc-url $RPC_URL \
 *     --verify \
 *     --etherscan-api-key $API_KEY \
 *     -vvvv
 */
contract Verify is Script {
    function run() external view {
        // Get contract addresses from environment
        address registry = vm.envAddress("REGISTRY_ADDRESS");
        address oracle = vm.envAddress("ORACLE_ADDRESS");
        address token = vm.envAddress("TOKEN_ADDRESS");
        address factory = vm.envAddress("FACTORY_ADDRESS");

        console.log("=== VERIFICATION INFO ===");
        console.log("ESGRegistry:", registry);
        console.log("ESGOracle:", oracle);
        console.log("ERC8040:", token);
        console.log("ERC8040Factory:", factory);
        console.log("\nRun the following commands to verify:");
        console.log("\n# ESGRegistry (no constructor args)");
        console.log("forge verify-contract", registry, "src/ESGRegistry.sol:ESGRegistry --chain-id", block.chainid);
        console.log("\n# ESGOracle (constructor: address _registry)");
        console.log("forge verify-contract", oracle, "src/ESGOracle.sol:ESGOracle --chain-id", block.chainid, "--constructor-args $(cast abi-encode 'constructor(address)' ", registry, ")");
        console.log("\n# ERC8040 (constructor: string name, string symbol, address _oracle)");
        console.log("forge verify-contract", token, "src/ERC8040.sol:ERC8040 --chain-id", block.chainid, "--constructor-args $(cast abi-encode 'constructor(string,string,address)' 'AgroNet ESG Token' 'AGRO' ", oracle, ")");
        console.log("\n# ERC8040Factory (constructor: address _oracle)");
        console.log("forge verify-contract", factory, "src/ERC8040Factory.sol:ERC8040Factory --chain-id", block.chainid, "--constructor-args $(cast abi-encode 'constructor(address)' ", oracle, ")");
    }
}
