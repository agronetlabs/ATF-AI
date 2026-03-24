// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/ESGRegistry.sol";
import "../src/ESGOracle.sol";
import "../src/ERC8040.sol";
import "../src/ERC8040Factory.sol";

/**
 * @title DeploymentTest
 * @notice Basic deployment tests for ERC8040 ecosystem
 */
contract DeploymentTest is Test {
    ESGRegistry public registry;
    ESGOracle public oracle;
    ERC8040 public token;
    ERC8040Factory public factory;

    address public deployer = address(this);
    address public user1 = address(0x1);
    address public user2 = address(0x2);

    function setUp() public {
        // Deploy contracts in correct order
        registry = new ESGRegistry();
        oracle = new ESGOracle(address(registry));
        token = new ERC8040("AgroNet ESG Token", "AGRO", address(oracle));
        factory = new ERC8040Factory(address(oracle));

        // Setup initial provider
        registry.registerProvider(deployer, "Test Provider");
        oracle.setProvider(deployer, true);
    }

    function testDeployment() public {
        // Verify registry deployment
        assertTrue(address(registry) != address(0), "Registry not deployed");
        assertTrue(registry.isActiveProvider(deployer), "Deployer not active provider");

        // Verify oracle deployment
        assertTrue(address(oracle) != address(0), "Oracle not deployed");
        assertEq(address(oracle.registry()), address(registry), "Oracle registry mismatch");

        // Verify token deployment
        assertTrue(address(token) != address(0), "Token not deployed");
        assertEq(token.name(), "AgroNet ESG Token", "Token name mismatch");
        assertEq(token.symbol(), "AGRO", "Token symbol mismatch");
        assertEq(address(token.oracle()), address(oracle), "Token oracle mismatch");

        // Verify factory deployment
        assertTrue(address(factory) != address(0), "Factory not deployed");
        assertEq(address(factory.oracle()), address(oracle), "Factory oracle mismatch");
    }

    function testESGScoreUpdate() public {
        // Update ESG score
        oracle.updateScore(user1, 75, 80, 85);

        // Verify score
        (uint8 env, uint8 soc, uint8 gov, uint256 timestamp) = oracle.getScore(user1);
        assertEq(env, 75, "Environmental score mismatch");
        assertEq(soc, 80, "Social score mismatch");
        assertEq(gov, 85, "Governance score mismatch");
        assertTrue(timestamp > 0, "Timestamp not set");
    }

    function testTokenMintWithESG() public {
        // Set ESG score for user1
        oracle.updateScore(user1, 75, 80, 85);

        // Mint tokens
        uint256 tokenId = token.mintWithESG(user1, 1000 * 10**18, 1, "ipfs://test");

        // Verify token balance
        assertEq(token.balanceOf(user1), 1000 * 10**18, "Balance mismatch");
        assertEq(tokenId, 1, "Token ID mismatch");

        // Verify ESG metadata
        (uint8 env, uint8 soc, uint8 gov, uint256 auditId, string memory auditHash, uint256 mintedAt) = 
            token.tokenESGData(tokenId);
        assertEq(env, 75, "Environmental score mismatch");
        assertEq(soc, 80, "Social score mismatch");
        assertEq(gov, 85, "Governance score mismatch");
        assertEq(auditId, 1, "Audit ID mismatch");
        assertEq(auditHash, "ipfs://test", "Audit hash mismatch");
        assertTrue(mintedAt > 0, "Minted timestamp not set");
    }

    function testFactoryCreateToken() public {
        // Create new token through factory
        vm.prank(user1);
        address newToken = factory.createToken("User Token", "UTK");

        // Verify token created
        assertTrue(newToken != address(0), "Token not created");
        assertEq(ERC8040(newToken).name(), "User Token", "Token name mismatch");
        assertEq(ERC8040(newToken).symbol(), "UTK", "Token symbol mismatch");
        assertEq(ERC8040(newToken).owner(), user1, "Owner mismatch");

        // Verify factory tracking
        assertEq(factory.getTokenCount(), 1, "Token count mismatch");
        address[] memory userTokens = factory.getTokensByCreator(user1);
        assertEq(userTokens.length, 1, "User token count mismatch");
        assertEq(userTokens[0], newToken, "User token address mismatch");
    }

    function testCannotMintWithoutESGScore() public {
        // Try to mint without ESG score
        vm.expectRevert("No ESG score found");
        token.mintWithESG(user2, 1000 * 10**18, 1, "ipfs://test");
    }

    function testCannotMintBelowMinimumScore() public {
        // Set low ESG score
        oracle.updateScore(user2, 30, 35, 40);

        // Try to mint with low score
        vm.expectRevert("ESG score below minimum");
        token.mintWithESG(user2, 1000 * 10**18, 1, "ipfs://test");
    }
}
