# Blockchain Integration — Technical Architecture

## Overview

This document provides the technical details of how the ATF-AI framework maps to the ERC-8040 blockchain adapter. It covers the data flow, smart contract roles, SDK support, and the formal concept-to-implementation mapping table.

For a high-level introduction, see [overview.md](./overview.md).  
For the formal adapter specification, see [`specs/adapters/erc8040.md`](../../../specs/adapters/erc8040.md).

---

## Smart Contract Roles

The ERC-8040 adapter consists of four primary smart contracts, each mapping to a specific ATF-AI layer:

### `ERC8040.sol` — Provenance and Zero-Trust Token

The core ERC-20 compatible token that carries ATF-AI provenance data on-chain.

**ATF-AI mapping:**
- Stores provenance record hashes in token metadata
- Enforces zero-trust validation through score verification before transfers
- Exposes the ESG score as a first-class token attribute

```
Token Metadata (ERC8040.sol)
──────────────────────────────
{
  "token_id": "0x...",
  "holder": "0x...",         ← Agent Identity (wallet address)
  "lei": "549300...",        ← Legal Entity Identifier
  "esg_score": {
    "environmental": 92.5,
    "social": 87.0,
    "governance": 90.0,
    "composite": 89.8        ← ATF-AI Provenance Record hash embedded
  },
  "provenance_hash": "sha256:abc123...",
  "updated_at": 1710000000
}
```

### `ESGOracle.sol` — Attestation Engine

The oracle contract that receives ATF-AI attestation results from authorized agents and publishes them on-chain.

**ATF-AI mapping:**
- Receives attestation results from ATF-AI compliance agents
- Authorized agent list mirrors the ATF-AI Zero-Trust Identity Registry
- Each oracle update = one ATF-AI Attestation published on-chain

```
ESGOracle Flow
──────────────
ATF-AI Agent → Produces Attestation → ESGOracle.updateScore(address, score)
                                              │
                                              ▼
                                    On-chain verification
                                    (checks authorized provider)
                                              │
                                              ▼
                                    ERC8040.sol metadata updated
```

### `ESGRegistry.sol` — Governance and Zero-Trust Registry

The registry of authorized ESG data providers, mapping directly to the ATF-AI Governance Layer.

**ATF-AI mapping:**
- Only addresses registered in `ESGRegistry` can update oracle scores
- Registration requires governance approval (Protocol Steward authorization)
- Acts as the on-chain implementation of ATF-AI's Zero-Trust Identity Registry

### `ERC8040Factory.sol` — Adapter Interface

The factory contract that deploys new ERC-8040 token instances, serving as the adapter's entry point.

**ATF-AI mapping:**
- Each `factory.deploy()` call = one ATF-AI Adapter Interface instantiation
- Encodes governance parameters (registry address, oracle address) at deploy time

---

## Data Flow: ATF-AI Agent → On-Chain

```
1. ATF-AI Agent receives input data
       │
       ▼
2. Zero-Trust check: agent identity verified against ATF-AI registry
       │
       ▼
3. ATF-AI processes data → produces Provenance Record (PR-XXXX)
       │
       ▼
4. ATF-AI issues Compliance Attestation referencing PR-XXXX
       │
       ▼
5. ERC-8040 Adapter receives attestation result
       │
       ▼
6. ESGOracle.updateScore() called with attestation result
   (caller verified against ESGRegistry)
       │
       ▼
7. ERC8040 token metadata updated with new ESG score + provenance hash
       │
       ▼
8. On-chain event emitted → third parties can verify
```

---

## Multi-Language SDK Support

The ERC-8040 adapter provides SDKs for three languages, all sharing the same core logic via the Rust engine:

### Rust Core (`core/`)

The foundational engine for ESG scoring and attestation logic. All other SDKs call into or replicate the Rust core algorithms.

```rust
use erc8040_core::{ESGScore, ProvenanceRecord, AttestationBuilder};

let score = ESGScore::new(92.5, 87.0, 90.0);
let record = ProvenanceRecord::create(&score, agent_key)?;
let attestation = AttestationBuilder::new()
    .standard("EU-SFDR-Article-9")
    .evidence(record)
    .build_and_sign(&agent_key)?;
```

### Python SDK (`python-sdk/`)

High-level Python interface for integration with data pipelines, ML models, and regulatory reporting tools.

```python
from erc8040_sdk import ESGScore, AttestationBuilder, ComplianceStandard

score = ESGScore.create(environmental=92.5, social=87.0, governance=90.0)
attestation = (
    AttestationBuilder()
    .standard(ComplianceStandard.SFDR_ARTICLE_9)
    .evidence(score)
    .build_and_sign(agent_key)
)
```

### C++ SDK (`cpp-sdk/`)

High-performance C++17 SDK for latency-sensitive environments.

```cpp
#include <erc8040/esg.hpp>
#include <erc8040/attestation.hpp>

auto score = erc8040::ESGScore{92.5, 87.0, 90.0};
auto attestation = erc8040::AttestationBuilder{}
    .standard(erc8040::Standard::SFDR_ARTICLE_9)
    .evidence(score)
    .build_and_sign(agent_key);
```

---

## ATF-AI Concept → ERC-8040 Implementation Mapping

| ATF-AI Concept | ERC-8040 Implementation | File |
|---|---|---|
| Provenance Record | ESG Score + Token metadata | `ERC8040.sol` |
| Attestation | ESGOracle verification result | `ESGOracle.sol` |
| Governance Layer | ESGRegistry authorized providers | `ESGRegistry.sol` |
| Zero-Trust Validation | On-chain score verification | `ERC8040.sol` |
| Agent Identity | Wallet address + LEI | `ESGRegistry.sol` |
| Adapter Interface | ERC8040Factory deployment | `ERC8040Factory.sol` |
| Attestation Chain | Oracle update history + events | `ESGOracle.sol` |
| Provenance Hash | `provenance_hash` token field | `ERC8040.sol` |

---

## Deployment Targets

| Network | Type | Status |
|---|---|---|
| Ethereum Sepolia | Testnet | Active |
| Base Sepolia | Testnet | Active |
| Ethereum Mainnet | Production | Planned |
| Base Mainnet | Production | Planned |

---

## Related Documentation

- [Overview](./overview.md) — Introduction and architecture diagram
- [SWIFT/ISO 20022 Bridge](./swift-bridge.md) — Traditional finance interoperability
- [Formal Adapter Spec](../../../specs/adapters/erc8040.md)
- [ERC-8040 Repository](https://github.com/agronetlabs/erc-8040-ecosystem)
