# Blockchain Integration — Overview

## Overview

This document describes the official **blockchain adapter** for ATF-AI: the **ERC-8040 Ecosystem**. The ERC-8040 adapter extends the ATF-AI framework with blockchain-native ESG capabilities, enabling on-chain provenance records, verifiable attestations, and compliant ESG token infrastructure — all governed by the ATF-AI Core Spec.

> **ATF-AI is blockchain-agnostic.** Blockchain is one possible deployment context — not a requirement. The ERC-8040 adapter is an optional extension, not part of ATF-AI Core.

---

## What is the ERC-8040 Adapter?

The ERC-8040 adapter is the **official ATF-AI adapter for blockchain/ESG**. It translates ATF-AI's governance, provenance, and attestation primitives into:

- **Smart contracts** (Solidity): on-chain ESG scoring, oracle, and registry
- **Multi-language SDKs**: Rust core engine, Python SDK, C++ SDK
- **Compliance bridges**: ISO 20022 / SWIFT integration for traditional finance interoperability

**Repository:** [https://github.com/agronetlabs/erc-8040-ecosystem](https://github.com/agronetlabs/erc-8040-ecosystem)  
**Formal Spec:** [`specs/adapters/erc8040.md`](../../../specs/adapters/erc8040.md)

---

## Architecture

The integration follows a **3-layer architecture**, where each layer has a clear responsibility boundary:

```
┌─────────────────────────────────────────┐
│           ATF-AI Framework              │
│  (Governance · Provenance · Zero-Trust) │
└────────────────┬────────────────────────┘
                 │ adapter interface
┌────────────────▼────────────────────────┐
│        ERC-8040 Blockchain Adapter      │
│   (ESG Scoring · Oracle · Registry)     │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│      Ethereum / EVM Networks            │
│  (Sepolia · Base · Mainnet)             │
└─────────────────────────────────────────┘
```

### Layer 1 — ATF-AI Framework

The ATF-AI Core layer is responsible for:
- **Governance**: defining which agents are authorized and what actions they can take
- **Provenance**: generating and chaining immutable records of every agent action
- **Zero-Trust Validation**: verifying agent identity before any action is executed
- **Attestation issuance**: producing signed compliance declarations

The ATF-AI layer knows nothing about blockchain. It operates on abstract provenance records and attestations.

### Layer 2 — ERC-8040 Adapter

The adapter layer is responsible for:
- **Translating** ATF-AI provenance records into on-chain token metadata
- **Publishing** attestation results via the `ESGOracle` smart contract
- **Enforcing** ATF-AI zero-trust through the `ESGRegistry` authorized provider list
- **Bridging** to traditional finance via ISO 20022 / SWIFT (see [swift-bridge.md](./swift-bridge.md))
- **Providing** multi-language developer SDKs

### Layer 3 — Ethereum / EVM Networks

The network layer handles:
- On-chain storage of ESG scores and provenance hashes
- Decentralized verification by third parties
- Token lifecycle management (mint, update, transfer)
- Multi-chain deployment (Sepolia testnet, Base Sepolia, future mainnet)

---

## Key Concepts Mapping

| ATF-AI Concept | ERC-8040 Implementation |
|---|---|
| Provenance Record | ESG Score + Token metadata in `ERC8040.sol` |
| Attestation | `ESGOracle` verification result |
| Governance Layer | `ESGRegistry` authorized providers |
| Zero-Trust Validation | On-chain score verification in `ERC8040.sol` |
| Agent Identity | Wallet address + LEI (Legal Entity Identifier) |

For the full mapping, see [`specs/adapters/erc8040.md`](../../../specs/adapters/erc8040.md).

---

## Compliance Standards Supported

The ERC-8040 adapter enables ATF-AI attestations that satisfy:

| Standard | Description |
|---|---|
| **EU SFDR** | Articles 6, 8, and 9 (sustainable finance disclosure) |
| **EU Taxonomy** | Environmental activity alignment |
| **ISO 20022** | Universal financial messaging standard |
| **MiFID II** | Investment suitability and disclosure |

---

## Next Steps

- [Architecture — Detailed Technical Integration](./architecture.md)
- [SWIFT/ISO 20022 Bridge](./swift-bridge.md)
- [Formal Adapter Spec](../../../specs/adapters/erc8040.md)
