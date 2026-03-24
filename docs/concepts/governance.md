# Governance

## Overview

The ATF-AI **Governance Model** defines how the protocol itself is maintained, versioned, and evolved — and how authority is distributed between the protocol steward, adapter maintainers, and compliance auditors. Governance in ATF-AI is intentionally separated from implementation: the framework defines the rules, adapters implement them.

---

## Separation of Concerns: Framework vs. Adapters

A core design principle of ATF-AI is that **governance is adapter-agnostic**:

```
┌─────────────────────────────────────────────────────┐
│                  ATF-AI Framework                   │
│   Defines: Governance rules, versioning policy,     │
│   role definitions, change processes                │
└──────────────────────┬──────────────────────────────┘
                       │ implements
         ┌─────────────┼─────────────┐
         ▼             ▼             ▼
  ┌────────────┐ ┌────────────┐ ┌────────────┐
  │  ERC-8040  │ │  Cloud     │ │  IoT       │
  │  Adapter   │ │  Adapter   │ │  Adapter   │
  └────────────┘ └────────────┘ └────────────┘
```

This means:
- The **ATF-AI Governance Model** applies universally to all adapters
- Adapters can extend governance rules for domain-specific concerns
- Adapters cannot override core ATF-AI governance constraints

---

## Roles

### Protocol Steward — AgroNet Labs (Leandro)

The Protocol Steward is responsible for:

- Maintaining the canonical ATF-AI specification (`specs/atf-core-v1.md`)
- Approving breaking changes to the core protocol
- Publishing official governance decisions via the `GOVERNANCE.md` root file
- Certifying new adapters as official ATF-AI adapters
- Managing the Semantic Versioning roadmap

**Current Protocol Steward:** AgroNet Labs  
**Contact:** See [`GOVERNANCE.md`](../../GOVERNANCE.md) in the repository root

### Adapter Maintainer

An Adapter Maintainer is responsible for:

- Implementing and maintaining a specific ATF-AI adapter
- Ensuring the adapter conforms to the ATF-AI Core Spec
- Publishing adapter-level governance documentation
- Responding to security disclosures related to their adapter

**Example:** The ERC-8040 Blockchain Adapter is maintained by AgroNet Labs. See [`specs/adapters/erc8040.md`](../../specs/adapters/erc8040.md).

### Compliance Auditor

A Compliance Auditor is an independent third party (or automated agent) that:

- Verifies that attestations produced by ATF-AI agents are valid
- Audits provenance chains for integrity
- Issues external compliance reports referencing ATF-AI governance records

Compliance Auditors interact with ATF-AI primarily through the [Attestation](./attestation.md) layer — they do not require direct access to the protocol internals.

---

## How Protocol Changes Are Governed

ATF-AI follows **Semantic Versioning** (`MAJOR.MINOR.PATCH`) for the core protocol:

| Version Type | Trigger | Approval Required |
|---|---|---|
| **PATCH** (`x.x.1`) | Bug fixes, documentation corrections | Protocol Steward review |
| **MINOR** (`x.1.x`) | New features, new adapter interfaces | Protocol Steward approval + 14-day comment period |
| **MAJOR** (`2.x.x`) | Breaking changes to core spec | Protocol Steward + community RFC process |

### RFC Process for Major Changes

1. **Proposal** — Author submits an RFC document to the repository
2. **Discussion** — 30-day public comment period
3. **Review** — Protocol Steward reviews and responds to comments
4. **Decision** — Protocol Steward publishes accept/reject decision
5. **Implementation** — If accepted, changes are scheduled for the next major release

---

## Governance Records

All governance decisions that affect the protocol produce **Governance Attestations** — a specific type of [Attestation](./attestation.md) in ATF-AI. This means the governance history of the protocol is itself subject to the same provenance and traceability guarantees as any other agent action.

Examples of governance records:
- "ATF-AI Core Spec v1.1 approved on 2024-06-01 by Protocol Steward"
- "ERC-8040 Adapter certified as official ATF-AI adapter on 2024-03-24"
- "Security disclosure ATF-SEC-2024-01 resolved in patch v1.0.3"

---

## Relationship with GOVERNANCE.md

This document describes the governance model at a conceptual level. The operational governance policies — including contact information, response SLAs, and current version status — are maintained in the [`GOVERNANCE.md`](../../GOVERNANCE.md) file at the repository root.

| Document | Purpose |
|---|---|
| `docs/concepts/governance.md` (this file) | Conceptual model: roles, principles, decision processes |
| `GOVERNANCE.md` (root) | Operational: current policies, contacts, version status |
| `specs/atf-core-v1.md` | Technical: formal protocol specification |

---

## Related Concepts

- [Provenance](./provenance.md) — governance decisions produce provenance records
- [Attestation](./attestation.md) — governance outcomes are expressed as attestations
- [Zero-Trust](./zero-trust.md) — governance enforcement relies on zero-trust agent identity
