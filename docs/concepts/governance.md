# Governance

## Overview

The ATF-AI **Governance Model** defines how the protocol is maintained, versioned, reviewed, and evolved — and how authority is separated between the Protocol Steward, maintainers, adapter maintainers, auditors, and AI-assisted contributors.

ATF-AI governance is intentionally separated from implementation: **the framework defines trust and governance requirements; adapters implement them for specific infrastructures or domains.**

The operational source of truth is [`GOVERNANCE.md`](../../GOVERNANCE.md).

---

## Separation of Concerns: Framework vs. Adapters

```text
+-----------------------------------------------------+
|                  ATF-AI Framework                   |
| Governance rules / provenance / versioning /        |
| decision records / adapter requirements             |
+--------------------------+--------------------------+
                           |
                 implements / extends
          +----------------+----------------+
          |                |                |
          v                v                v
   +-------------+  +-------------+  +-------------+
   | ERC-8040    |  | RWA         |  | Cloud / IoT |
   | Adapter     |  | Governance  |  | Adapters    |
   +-------------+  +-------------+  +-------------+
```

This means:

- The **ATF-AI Core Governance Model** applies universally.
- Adapters MAY extend governance for domain-specific concerns.
- Adapters MUST NOT override mandatory ATF-AI Core constraints.
- Adapter maturity is independent from core maturity.

---

## Roles

### Protocol Steward — AgroNet Labs LLC

The Protocol Steward is responsible for:

- maintaining the canonical ATF-AI specification;
- approving breaking core changes;
- coordinating versioning and releases;
- recognizing official adapters;
- issuing or revoking formal ATF-AI certification;
- maintaining governance and security records.

**Current Protocol Steward:** AgroNet Labs LLC  
**Operational policy:** [`GOVERNANCE.md`](../../GOVERNANCE.md)

### Maintainer

A Maintainer reviews changes, merges Pull Requests, enforces contribution requirements, and protects the consistency of the protocol and reference artifacts.

The canonical GitHub ownership rule is maintained in `.github/CODEOWNERS`.

### Adapter Maintainer

An Adapter Maintainer is responsible for:

- implementing and maintaining a specific ATF-AI adapter;
- declaring the compatible ATF-AI Core version;
- documenting conformance and security requirements;
- maintaining tests and schemas where applicable;
- responding to adapter security disclosures.

Examples:

- ERC-8040 Blockchain Adapter: [`specs/adapters/erc8040.md`](../../specs/adapters/erc8040.md)
- RWA Privileged Action Governance Adapter: [`specs/adapters/rwa-privileged-governance.md`](../../specs/adapters/rwa-privileged-governance.md)

### Independent Auditor

An independent auditor may:

- verify ATF-AI attestations;
- audit provenance chains;
- assess security controls and conformance;
- issue independent reports referencing ATF-AI governance evidence.

Independent assessment does not require protocol governance authority.

### AI-Assisted Contributor

AI systems may materially assist architecture, code, documentation, tests, validation, or security analysis.

They are **non-authoritative contributors**. AI systems do not receive voting rights, merge authority, certification authority, or independent power to approve their own output.

Model/vendor identity is attribution only and is not a protocol dependency.

---

## How Protocol Changes Are Governed

ATF-AI follows Semantic Versioning for the core protocol:

| Version Type | Trigger | Normal approval path |
|---|---|---|
| **PATCH** (`x.x.1`) | Corrections and non-behavioral clarifications | Maintainer review + applicable CI |
| **MINOR** (`x.1.x`) | Backward-compatible features or interfaces | Maintainer/Steward approval + public review when material |
| **MAJOR** (`2.x.x`) | Breaking core changes | Protocol Steward + RFC process |
| **SECURITY** | Urgent mitigation | Expedited human approval + auditable follow-up record |

Normal review targets are:

- **14 days** for material MINOR changes;
- **30 days** for MAJOR RFCs.

These periods may be shortened for urgent security or operational reasons when the rationale is recorded.

---

## Adapter Lifecycle

Adapters use explicit maturity states:

| Status | Meaning |
|---|---|
| `DRAFT` | Design and threat model may change materially. |
| `REVIEW` | Interface is substantially defined and undergoing external/integration review. |
| `STABLE` | Versioned conformance surface with compatibility expectations. |
| `DEPRECATED` | Superseded or no longer recommended. |

Current examples as of **2026-08-23**:

- ERC-8040 Blockchain Adapter — `DRAFT v0.1`
- RWA Privileged Action Governance Adapter — `DRAFT v0.1`

Passing CI does not automatically move an adapter from DRAFT to STABLE.

---

## Human Authority and AI Assistance

ATF-AI permits AI-assisted governance workflows but does not delegate protocol authority to a model.

```text
AI / Tool Assistance
        |
        v
Proposal / Analysis / Tests
        |
        v
Human Maintainer Review
        |
        v
CI + Governance Requirements
        |
        v
Human Acceptance / Rejection
        |
        v
Governance Record / Merge
```

This separation mirrors ATF-AI's broader zero-trust principle: possessing technical capability does not, by itself, create governance authority.

---

## Governance Records

Material governance decisions should produce an auditable record through repository history and, where applicable, a Governance Attestation.

Examples include:

- approval of a new core version;
- adapter transition from `DRAFT` to `REVIEW` or `STABLE`;
- security mitigation acceptance;
- certification issuance or revocation;
- deprecation decisions.

Illustrative record:

```text
Decision: RWA Privileged Action Governance Adapter accepted as DRAFT v0.1
Authority: AgroNet Labs / Maintainer
Core compatibility: ATF-AI Core v1
Evidence: specification + schema + Rust reference validator + passing CI
Status: DRAFT (not STABLE)
```

---

## Conformance vs. Certification

**Conformance** means an implementation can demonstrate compliance with the applicable ATF-AI Core and adapter requirements.

**ATF-AI Certified** is a formal designation issued through the governance process defined by AgroNet Labs. Conformance alone does not automatically grant certification.

This distinction prevents self-certification from being confused with independent or steward-issued review.

---

## Relationship with Root Governance

| Document | Purpose |
|---|---|
| `docs/concepts/governance.md` | Conceptual model and role separation |
| [`GOVERNANCE.md`](../../GOVERNANCE.md) | Operational authority, review gates, lifecycle, certification |
| [`CONTRIBUTING.md`](../../CONTRIBUTING.md) | Contribution workflow and AI-assistance disclosure |
| [`specs/atf-core-v1.md`](../../specs/atf-core-v1.md) | Technical core protocol specification |

---

## Related Concepts

- [Provenance](./provenance.md) — evidence underlying governance records
- [Attestation](./attestation.md) — signed governance/compliance declarations
- [Zero-Trust](./zero-trust.md) — no capability is implicitly trusted
