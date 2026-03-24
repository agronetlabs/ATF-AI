// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

interface IESGRegistry {
    function isActiveProvider(address provider) external view returns (bool);
}

/**
 * @title ESGOracle
 * @notice Oracle for ESG scores with multi-provider support
 */
contract ESGOracle is Ownable {
    struct ESGScore {
        uint8 environmental;
        uint8 social;
        uint8 governance;
        uint256 timestamp;
        address provider;
    }

    IESGRegistry public registry;
    mapping(address => ESGScore) public scores;
    mapping(address => bool) public authorizedProviders;

    event ScoreUpdated(
        address indexed entity,
        uint8 environmental,
        uint8 social,
        uint8 governance,
        address indexed provider
    );
    event ProviderAuthorized(address indexed provider);
    event ProviderRevoked(address indexed provider);

    constructor(address _registry) Ownable(msg.sender) {
        require(_registry != address(0), "Invalid registry address");
        registry = IESGRegistry(_registry);
    }

    /**
     * @notice Authorize a provider to submit scores
     * @param provider Address of the provider
     * @param authorized True to authorize, false to revoke
     */
    function setProvider(address provider, bool authorized) external onlyOwner {
        authorizedProviders[provider] = authorized;
        if (authorized) {
            emit ProviderAuthorized(provider);
        } else {
            emit ProviderRevoked(provider);
        }
    }

    /**
     * @notice Update ESG score for an entity
     * @param entity Address of the entity
     * @param environmental Environmental score (0-100)
     * @param social Social score (0-100)
     * @param governance Governance score (0-100)
     */
    function updateScore(
        address entity,
        uint8 environmental,
        uint8 social,
        uint8 governance
    ) external {
        require(authorizedProviders[msg.sender], "Not authorized provider");
        require(registry.isActiveProvider(msg.sender), "Provider not active in registry");
        require(environmental <= 100, "Environmental score out of range");
        require(social <= 100, "Social score out of range");
        require(governance <= 100, "Governance score out of range");

        scores[entity] = ESGScore({
            environmental: environmental,
            social: social,
            governance: governance,
            timestamp: block.timestamp,
            provider: msg.sender
        });

        emit ScoreUpdated(entity, environmental, social, governance, msg.sender);
    }

    /**
     * @notice Get ESG score for an entity
     * @param entity Address of the entity
     * @return environmental Environmental score
     * @return social Social score
     * @return governance Governance score
     * @return timestamp Last update timestamp
     */
    function getScore(address entity)
        external
        view
        returns (uint8 environmental, uint8 social, uint8 governance, uint256 timestamp)
    {
        ESGScore memory score = scores[entity];
        return (score.environmental, score.social, score.governance, score.timestamp);
    }

    /**
     * @notice Get composite ESG score (average of all three)
     * @param entity Address of the entity
     * @return Composite score
     */
    function getCompositeScore(address entity) external view returns (uint8) {
        ESGScore memory score = scores[entity];
        if (score.timestamp == 0) return 0;
        return uint8((uint16(score.environmental) + uint16(score.social) + uint16(score.governance)) / 3);
    }

    /**
     * @notice Check if entity has a valid score
     * @param entity Address of the entity
     * @param maxAge Maximum age of score in seconds (0 for no age check)
     * @return True if score is valid
     */
    function hasValidScore(address entity, uint256 maxAge) external view returns (bool) {
        ESGScore memory score = scores[entity];
        if (score.timestamp == 0) return false;
        if (maxAge == 0) return true;
        return block.timestamp - score.timestamp <= maxAge;
    }
}
