// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ESGRegistry
 * @notice Registry for ESG data providers
 */
contract ESGRegistry is Ownable {
    struct Provider {
        string name;
        bool isActive;
        uint256 registeredAt;
    }

    mapping(address => Provider) public providers;
    address[] public providerList;

    event ProviderRegistered(address indexed provider, string name);
    event ProviderDeactivated(address indexed provider);
    event ProviderActivated(address indexed provider);

    constructor() Ownable(msg.sender) {}

    /**
     * @notice Register a new ESG data provider
     * @param provider Address of the provider
     * @param name Name of the provider
     */
    function registerProvider(address provider, string memory name) external onlyOwner {
        require(provider != address(0), "Invalid provider address");
        require(bytes(name).length > 0, "Provider name required");
        require(providers[provider].registeredAt == 0, "Provider already registered");

        providers[provider] = Provider({
            name: name,
            isActive: true,
            registeredAt: block.timestamp
        });

        providerList.push(provider);
        emit ProviderRegistered(provider, name);
    }

    /**
     * @notice Deactivate a provider
     * @param provider Address of the provider
     */
    function deactivateProvider(address provider) external onlyOwner {
        require(providers[provider].isActive, "Provider not active");
        providers[provider].isActive = false;
        emit ProviderDeactivated(provider);
    }

    /**
     * @notice Activate a provider
     * @param provider Address of the provider
     */
    function activateProvider(address provider) external onlyOwner {
        require(!providers[provider].isActive, "Provider already active");
        require(providers[provider].registeredAt > 0, "Provider not registered");
        providers[provider].isActive = true;
        emit ProviderActivated(provider);
    }

    /**
     * @notice Check if an address is a registered and active provider
     * @param provider Address to check
     * @return True if provider is active
     */
    function isActiveProvider(address provider) external view returns (bool) {
        return providers[provider].isActive;
    }

    /**
     * @notice Get total number of registered providers
     * @return Number of providers
     */
    function getProviderCount() external view returns (uint256) {
        return providerList.length;
    }
}
