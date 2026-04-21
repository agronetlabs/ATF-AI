// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./ERC8040.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ERC8040Factory
 * @notice Factory for creating ERC8040 tokens
 */
contract ERC8040Factory is Ownable {
    address public oracle;
    
    struct TokenInfo {
        address tokenAddress;
        string name;
        string symbol;
        address creator;
        uint256 createdAt;
    }

    TokenInfo[] public tokens;
    mapping(address => address[]) public tokensByCreator;

    event TokenCreated(
        address indexed tokenAddress,
        string name,
        string symbol,
        address indexed creator
    );

    constructor(address _oracle) Ownable(msg.sender) {
        require(_oracle != address(0), "Invalid oracle address");
        oracle = _oracle;
    }

    /**
     * @notice Create a new ERC8040 token
     * @param name Token name
     * @param symbol Token symbol
     * @return address of the created token
     */
    function createToken(
        string memory name,
        string memory symbol
    ) external returns (address) {
        require(bytes(name).length > 0, "Name required");
        require(bytes(symbol).length > 0, "Symbol required");

        // Deploy new token
        ERC8040 token = new ERC8040(name, symbol, oracle);
        
        // Transfer ownership to creator
        token.transferOwnership(msg.sender);

        // Store token info
        tokens.push(TokenInfo({
            tokenAddress: address(token),
            name: name,
            symbol: symbol,
            creator: msg.sender,
            createdAt: block.timestamp
        }));

        tokensByCreator[msg.sender].push(address(token));

        emit TokenCreated(address(token), name, symbol, msg.sender);
        return address(token);
    }

    /**
     * @notice Update oracle address for new deployments
     * @param newOracle New oracle address
     */
    function setOracle(address newOracle) external onlyOwner {
        require(newOracle != address(0), "Invalid oracle address");
        oracle = newOracle;
    }

    /**
     * @notice Get total number of tokens created
     * @return Number of tokens
     */
    function getTokenCount() external view returns (uint256) {
        return tokens.length;
    }

    /**
     * @notice Get tokens created by an address
     * @param creator Creator address
     * @return Array of token addresses
     */
    function getTokensByCreator(address creator) external view returns (address[] memory) {
        return tokensByCreator[creator];
    }

    /**
     * @notice Get token info by index
     * @param index Token index
     * @return Token information
     */
    function getTokenInfo(uint256 index) external view returns (TokenInfo memory) {
        require(index < tokens.length, "Index out of bounds");
        return tokens[index];
    }
}
