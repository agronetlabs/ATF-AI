# Attestation

## Overview

An **Attestation** in ATF-AI is a verifiable, signed declaration produced by an agent asserting that a specific condition, compliance requirement, or governance rule has been evaluated and its result is certified. Attestations are the primary output interface of the ATF-AI framework — they transform raw provenance records into actionable compliance declarations.

---

## What is an Attestation?

An attestation is more than a log entry. It is a **structured, signed claim** that:

1. References a specific [Provenance Record](./provenance.md) (the evidence)
2. Declares a specific compliance outcome (pass/fail/conditional)
3. Is signed by the issuing agent with its verified identity
4. Carries an expiration policy and lifecycle metadata

```
Attestation Structure
─────────────────────
{
  "attestation_id": "atf-att-20240315-0042",
  "type": "compliance",
  "standard": "EU-SFDR-Article-9",
  "result": "PASS",
  "confidence": 0.97,
  "evidence": {
    "provenance_record": "hash:sha256:abc123...",
    "evaluated_at": "2024-03-15T14:22:00Z"
  },
  "issuer": {
    "agent_id": "atf-agent-compliance-01",
    "identity_proof": "..."
  },
  "validity": {
    "issued_at": "2024-03-15T14:22:01Z",
    "expires_at": "2024-09-15T14:22:01Z"
  },
  "signature": "..."
}
```

---

## Types of Attestation

ATF-AI defines three primary attestation types:

### 1. Compliance Attestation

Certifies that an asset, agent action, or dataset satisfies a specific regulatory or technical standard.

**Examples:**
- EU SFDR Article 6/8/9 classification
- EU Taxonomy alignment score
- MiFID II suitability declaration
- ISO 20022 message conformance

### 2. Governance Attestation

Certifies that a governance rule or policy within the ATF-AI framework has been applied correctly.

**Examples:**
- "Agent upgrade was approved by Protocol Steward"
- "Configuration change passed governance review"
- "Adapter deployment meets ATF-AI core spec v1 requirements"

### 3. Provenance Attestation

Certifies that a chain of provenance records is intact, unmodified, and produced by verified agents.

**Examples:**
- "All provenance records from agent X between T1 and T2 are valid"
- "Input data hash matches declared source"

---

## Attestation Lifecycle

```
    ┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────┐
    │  PENDING │────►│   ISSUED     │────►│   VERIFIED   │────►│ EXPIRED  │
    └──────────┘     └──────────────┘     └──────────────┘     └──────────┘
                            │                                        ▲
                            │         ┌──────────────┐              │
                            └────────►│   REVOKED    │──────────────┘
                                      └──────────────┘
```

| State | Description |
|---|---|
| **PENDING** | Attestation requested but not yet evaluated |
| **ISSUED** | Agent has evaluated and signed the attestation |
| **VERIFIED** | A second agent or external system has confirmed the attestation |
| **EXPIRED** | Validity window has passed — re-attestation required |
| **REVOKED** | Issuer or governance authority invalidated the attestation before expiry |

---

## Attestations and ESG Scoring

ATF-AI attestations integrate directly with ESG scoring pipelines. A compliance attestation for EU SFDR Article 9 requires underlying attestations for:

- Environmental score ≥ threshold
- Social score ≥ threshold  
- Governance score ≥ threshold
- No principal adverse impacts (PAI) violations

Each of these sub-attestations carries its own provenance record, creating a **composable attestation tree** that regulators or auditors can traverse.

---

## Practical Examples

### SFDR Article 9 Attestation

An asset manager operating on ATF-AI can generate an SFDR Article 9 attestation:

```
ATF-AI Agent (Compliance Evaluator)
  → Evaluates asset against SFDR Article 9 criteria
  → References environmental data provenance records
  → Produces Compliance Attestation: "SFDR-ART9-PASS"
  → Signed with agent identity key
  → Stored in attestation registry
  → (Optional) Published via ERC-8040 adapter to blockchain
```

### EU Taxonomy Attestation

```
ATF-AI Agent (Taxonomy Evaluator)
  → Evaluates economic activity alignment
  → References sector classification data provenance
  → Produces Compliance Attestation: "EU-TAXONOMY-ALIGNED-60PCT"
  → Valid for 6 months (re-evaluation required at renewal)
```

---

## Attestation Chains

Attestations can reference other attestations as evidence, forming **attestation chains**. This is the mechanism through which ATF-AI supports composable compliance:

```
Root Attestation: "ESG Portfolio Compliant"
  ├── Child: "SFDR Article 9 — Asset A"
  │     └── Evidence: Provenance Record #PR-0042
  ├── Child: "EU Taxonomy Aligned — Asset B"
  │     └── Evidence: Provenance Record #PR-0043
  └── Child: "No PAI Violations — Portfolio"
        └── Evidence: Provenance Records #PR-0044..#PR-0099
```

---

## Attestations and the Blockchain Adapter

The [ERC-8040 adapter](../integrations/blockchain/overview.md) maps ATF-AI attestations to on-chain verifiable data:

| ATF-AI Attestation | ERC-8040 Implementation |
|---|---|
| Compliance Attestation result | ESGOracle on-chain score |
| Attestation issuer identity | Authorized oracle provider address |
| Attestation validity window | Token metadata expiry field |

See [`specs/adapters/erc8040.md`](../../specs/adapters/erc8040.md) for the full mapping.

---

## Related Concepts

- [Provenance](./provenance.md) — the evidence layer underlying attestations
- [Governance](./governance.md) — how governance attestations are produced and enforced
- [Zero-Trust](./zero-trust.md) — how agent identity is verified before attestation issuance
