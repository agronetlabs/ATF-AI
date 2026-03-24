// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

interface IESGOracle {
    function getScore(address entity)
        external
        view
        returns (uint8 environmental, uint8 social, uint8 governance, uint256 timestamp);
    
    function getCompositeScore(address entity) external view returns (uint8);
    
    function hasValidScore(address entity, uint256 maxAge) external view returns (bool);
}

/**
 * @title ERC8040
 * @notice ERC-8040 Compliance Token with ESG scoring
 */
contract ERC8040 is ERC20, ERC20Burnable, Ownable {
    IESGOracle public oracle;
    
    struct ESGMetadata {
        uint8 environmental;
        uint8 social;
        uint8 governance;
        uint256 auditId;
        string auditHash;
        uint256 mintedAt;
    }

    // Token ID to ESG metadata
    mapping(uint256 => ESGMetadata) public tokenESGData;
    
    // Counter for unique token batches
    uint256 public nextTokenId;
    
    // Minimum ESG score required (0-100)
    uint8 public minESGScore = 50;

    event ESGTokenMinted(
        address indexed to,
        uint256 indexed tokenId,
        uint256 amount,
        uint8 environmental,
        uint8 social,
        uint8 governance
    );
    
    event MinESGScoreUpdated(uint8 newMinScore);

    constructor(
        string memory name,
        string memory symbol,
        address _oracle
    ) ERC20(name, symbol) Ownable(msg.sender) {
        require(_oracle != address(0), "Invalid oracle address");
        oracle = IESGOracle(_oracle);
        nextTokenId = 1;
    }

    /**
     * @notice Mint tokens with ESG verification
     * @param to Recipient address
     * @param amount Amount to mint
     * @param auditId Audit identifier
     * @param auditHash IPFS or other hash of audit documentation
     */
    function mintWithESG(
        address to,
        uint256 amount,
        uint256 auditId,
        string memory auditHash
    ) external onlyOwner returns (uint256) {
        require(to != address(0), "Invalid recipient");
        require(amount > 0, "Amount must be greater than 0");
        require(bytes(auditHash).length > 0, "Audit hash required");

        // Get ESG score from oracle
        (uint8 env, uint8 soc, uint8 gov, uint256 timestamp) = oracle.getScore(to);
        require(timestamp > 0, "No ESG score found");
        
        // Validate minimum score requirement using oracle's composite score
        uint8 compositeScore = oracle.getCompositeScore(to);
        require(compositeScore >= minESGScore, "ESG score below minimum");

        // Store ESG metadata
        uint256 tokenId = nextTokenId++;
        tokenESGData[tokenId] = ESGMetadata({
            environmental: env,
            social: soc,
            governance: gov,
            auditId: auditId,
            auditHash: auditHash,
            mintedAt: block.timestamp
        });

        // Mint tokens
        _mint(to, amount);

        emit ESGTokenMinted(to, tokenId, amount, env, soc, gov);
        return tokenId;
    }

    /**
     * @notice Update minimum ESG score requirement
     * @param newMinScore New minimum score (0-100)
     */
    function setMinESGScore(uint8 newMinScore) external onlyOwner {
        require(newMinScore <= 100, "Score must be 0-100");
        minESGScore = newMinScore;
        emit MinESGScoreUpdated(newMinScore);
    }

    /**
     * @notice Update oracle address
     * @param newOracle New oracle address
     */
    function setOracle(address newOracle) external onlyOwner {
        require(newOracle != address(0), "Invalid oracle address");
        oracle = IESGOracle(newOracle);
    }

    /**
     * @notice Get ESG metadata for a token batch
     * @param tokenId Token batch ID
     * @return ESG metadata
     */
    function getESGMetadata(uint256 tokenId) external view returns (ESGMetadata memory) {
        return tokenESGData[tokenId];
    }
}
