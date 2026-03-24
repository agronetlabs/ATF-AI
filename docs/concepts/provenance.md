# Verifiable Provenance

## Overview

**Verifiable Provenance** is one of the three foundational pillars of the ATF-AI framework. It ensures that every action taken by an autonomous agent is cryptographically traceable, tamper-evident, and auditable — without requiring manual intervention.

---

## What is Provenance in the ATF-AI Context?

In traditional software systems, "provenance" refers to the origin or chain of custody of data. In ATF-AI, provenance extends this concept to cover **every decision and action produced by an autonomous agent**, including:

- Financial asset evaluations
- Compliance scoring
- Governance rule enforcement
- Data transformations and pipeline outputs

A **Provenance Record** in ATF-AI is an immutable, signed artifact that encodes:

| Field | Description |
|---|---|
| `agent_id` | Unique identity of the agent that produced the action |
| `action_type` | Classification of the action (evaluation, attestation, governance, etc.) |
| `input_hash` | Cryptographic hash of the input data |
| `output_hash` | Cryptographic hash of the output/result |
| `timestamp` | ISO 8601 UTC timestamp |
| `signature` | Cryptographic signature from the agent's identity key |
| `parent_record` | Reference to the preceding provenance record (chain) |

---

## Cryptographic Traceability Model

Each agent action in ATF-AI produces a provenance record that is **chained** to prior records, forming an append-only audit trail. This chain is enforced at the framework level — not at the adapter level.

```
Agent Action N-1       Agent Action N        Agent Action N+1
┌─────────────┐        ┌─────────────┐        ┌─────────────┐
│  Record     │        │  Record     │        │  Record     │
│  hash: H1   │◄───────│  parent: H1 │◄───────│  parent: H2 │
│  sig: S1    │        │  hash: H2   │        │  hash: H3   │
└─────────────┘        │  sig: S2    │        │  sig: S3    │
                       └─────────────┘        └─────────────┘
```

The chain property means:
- **Retroactive tampering is detectable** — modifying any historical record breaks the chain
- **Agent accountability is non-repudiable** — each record is signed by the producing agent
- **Cross-agent traces are possible** — when one agent consumes another's output, it references that agent's provenance record as parent

---

## Traditional Audit vs. ATF-AI Provenance

| Dimension | Traditional Audit | ATF-AI Verifiable Provenance |
|---|---|---|
| **Trigger** | Periodic (quarterly, annual) | Continuous — every action |
| **Coverage** | Sampled | 100% of agent actions |
| **Integrity** | Manual sign-off | Cryptographic signature |
| **Tamper detection** | Post-hoc investigation | Real-time chain validation |
| **Auditability** | Requires audit firm | Self-evident from records |
| **Latency** | Days to weeks | Milliseconds |
| **Cost** | High (human hours) | Near-zero marginal cost |

---

## Provenance and Agent Identity

ATF-AI enforces a **Zero-Trust** identity model: no agent is trusted by default. Provenance records are only valid if they carry a signature from an agent whose identity has been verified through the ATF-AI [Zero-Trust Validation Layer](./zero-trust.md).

This means:
- Unsigned records are rejected
- Records signed by unregistered agents are flagged
- Records with broken chain references trigger alerts

---

## Extending Provenance with Adapters

The ATF-AI core provenance model is **adapter-agnostic**. Adapters can extend the provenance model to persist records on external systems, without altering the core semantics.

### Example: ERC-8040 Blockchain Adapter

The [ERC-8040 adapter](../integrations/blockchain/overview.md) extends ATF-AI provenance by:

1. Embedding ATF-AI provenance record hashes into ERC-8040 token metadata
2. Publishing attestation results on-chain via the `ESGOracle` smart contract
3. Enabling third-party on-chain verification of any provenance record

This means an external party can verify an ATF-AI provenance record without running the ATF-AI framework itself — the blockchain acts as a **public notary** for the provenance chain.

See [`specs/adapters/erc8040.md`](../../specs/adapters/erc8040.md) for the formal specification of this mapping.

---

## Related Concepts

- [Attestation](./attestation.md) — how provenance records are transformed into verifiable compliance declarations
- [Zero-Trust](./zero-trust.md) — the identity model that underpins agent signing
- [Governance](./governance.md) — how governance rules produce provenance records
