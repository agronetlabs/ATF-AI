# ATF-AI Governance

This document defines the operational governance model for the **Autonomous Trust Framework for Artificial Intelligence (ATF-AI)**.

ATF-AI is an open, infrastructure-agnostic protocol coordinated by **AgroNet Labs LLC**. The protocol may be developed with substantial AI assistance, but **governance authority remains human and organizational**.

---

## Governance Model

ATF-AI operates as an **open protocol with coordinated stewardship**:

- The specification and core documentation are publicly available and freely adoptable.
- **AgroNet Labs LLC** acts as Protocol Steward and coordinating body.
- Core rules remain infrastructure- and vendor-neutral.
- Blockchain, cloud, enterprise, IoT, financial, and other adapters consume the protocol without becoming privileged dependencies.
- AI systems may assist analysis, implementation, testing, and documentation, but do not independently approve protocol changes or certifications.

---

## Authority and Roles

### Protocol Steward

**AgroNet Labs LLC** is the Protocol Steward and is responsible for:

- maintaining the canonical ATF-AI specification;
- approving breaking changes to the core protocol;
- coordinating versioning and release policy;
- recognizing official ATF-AI adapters;
- issuing or revoking formal ATF-AI certification;
- maintaining governance and security records.

### Maintainers

Maintainers review proposals, merge Pull Requests, maintain reference implementations, and enforce governance requirements.

**Current Maintainer:**
- Leandro Lemos — Founder & Lead Engineer, AgroNet Labs LLC

The repository's `CODEOWNERS` file defines the canonical GitHub review owner.

### Contributors

Contributors may submit issues, Pull Requests, documentation, tests, schemas, adapter implementations, security analysis, and governance proposals. Contributors must follow [`CONTRIBUTING.md`](./CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md), and [`SECURITY.md`](./SECURITY.md).

### Adapter Maintainers

Adapter Maintainers are responsible for:

- keeping an adapter compatible with its declared ATF-AI Core version;
- publishing adapter-level conformance and security requirements;
- maintaining tests and machine-readable schemas where applicable;
- responding to security disclosures affecting the adapter.

### Independent Auditors

Independent auditors may validate attestations, provenance chains, implementation behavior, security controls, and conformance claims. They do not need governance authority to issue an independent assessment.

---

## AI-Assisted Governance Boundary

ATF-AI permits AI-assisted development and review, but **AI systems are non-authoritative participants**.

An AI model or agent may:

- propose architecture or specification changes;
- generate or review code and documentation;
- perform deterministic validation or test generation;
- analyze security and compliance evidence;
- assist CI/CD and provenance workflows.

An AI model or agent may **not**, by itself:

- approve a breaking core-spec change;
- issue ATF-AI certification;
- self-approve or self-merge a Pull Request;
- override a human maintainer decision;
- convert repository, wallet, signer, API, or infrastructure credentials into governance authority.

Model/vendor attribution is informational only. ATF-AI is not governed by, dependent on, or controlled by any specific AI provider.

---

## Decision-Making Process

### Proposal

A material change should be proposed through a GitHub Issue or Pull Request and include:

- the problem being solved;
- whether the change affects the **core protocol** or an **adapter**;
- technical rationale;
- backward-compatibility impact;
- security implications;
- test/schema/documentation impact.

### Review Criteria

Maintainers review proposals for:

1. **Agnosticism** — core changes must remain infrastructure-neutral.
2. **Correctness** — requirements must be technically sound and unambiguous.
3. **Determinism** — governance behavior must be reproducible and auditable.
4. **Backward compatibility** — breaking changes require explicit versioning.
5. **Security** — privileged operations and trust boundaries must be explicit.
6. **Provenance** — accepted changes must remain attributable and traceable.
7. **Validation** — relevant CI, tests, and schemas must pass before merge.

### Change Classes

| Change | Minimum governance path |
|---|---|
| **PATCH** — typo, clarification, non-behavioral fix | Maintainer review + passing applicable CI |
| **MINOR** — backward-compatible feature or adapter interface | Maintainer approval + public review period when material |
| **MAJOR** — breaking core change | Protocol Steward approval + RFC process + major version bump |
| **SECURITY** — urgent mitigation | Expedited Maintainer/Steward approval with post-merge governance record |

For planned material MINOR changes, the normal target is a **14-day public comment period**. For MAJOR core changes, the normal RFC discussion period is **30 days**. The Protocol Steward may shorten these periods for urgent security or operational reasons, but the rationale must be documented.

---

## Adapter Lifecycle

Official ATF-AI adapters use the following maturity states:

| Status | Meaning |
|---|---|
| `DRAFT` | Design is open to material change; implementation and threat model are still evolving. |
| `REVIEW` | Interface is substantially defined and undergoing external or integration review. |
| `STABLE` | Conformance surface is versioned and changes follow compatibility rules. |
| `DEPRECATED` | Superseded or no longer recommended for new implementations. |

Current examples:

- `specs/adapters/erc8040.md` — **DRAFT v0.1**
- `specs/adapters/rwa-privileged-governance.md` — **DRAFT v0.1**

A DRAFT adapter must not be represented as a stable standard solely because reference code exists or CI passes.

---

## Pull Request Acceptance Gates

Before a material Pull Request is merged, maintainers should confirm the applicable gates:

- scope is correctly classified as core, adapter, reference implementation, documentation, or security;
- schemas compile when changed;
- executable tests pass;
- privileged operations preserve separation between governance authorization and cryptographic execution;
- no secrets, private keys, seed phrases, MPC shares, or sensitive credentials are committed;
- documentation and version metadata are consistent;
- AI-assisted work has human review when material;
- the final merge decision is made by an authorized human maintainer.

CI success is necessary evidence where applicable, but **CI success is not protocol approval by itself**.

---

## Conformance vs. Certification

**Conformance** and **certification** are different claims.

An implementation may claim **ATF-AI conformance** when it can demonstrate compliance with the applicable core specification and declared adapter requirements.

The designation **ATF-AI Certified** is reserved for implementations that have completed an explicit certification review and received a certification decision issued under AgroNet Labs governance.

A certification review should verify, at minimum:

| Requirement | Evidence expectation |
|---|---|
| **Agent Layer** | Verifiable provenance and trace context for governed actions. |
| **Governance Layer** | Deterministic, versioned, auditable policy decisions. |
| **Execution Layer** | Execution gated on approved governance decisions with recorded outcomes. |
| **Security** | Relevant threat model, key/privileged-action controls, and disclosure process. |
| **Adapter conformance** | Declared adapter requirements and version compatibility. |

Certification is not tied to a specific blockchain, cloud provider, AI model, wallet vendor, or execution technology.

To request a certification review, contact **admin@agronet.io**.

---

## Governance Records and Provenance

Material governance decisions should be traceable through repository history and, where applicable, ATF-AI Governance Attestations.

Governance records may include:

- core or adapter version approvals;
- adapter status transitions (`DRAFT` → `REVIEW` → `STABLE`);
- security mitigations;
- certification issuance or revocation;
- deprecation decisions.

The governance record should identify the decision, responsible human authority, policy/version context, timestamp, and relevant evidence.

---

## Versioning

ATF-AI follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (`v2.0`, `v3.0`) — breaking core changes.
- **MINOR** (`v1.1`, `v1.2`) — backward-compatible additions.
- **PATCH** (`v1.0.1`) — corrections and clarifications.

The current stable core specification is [`specs/atf-core-v1.md`](./specs/atf-core-v1.md).

Adapters must declare their compatible ATF-AI Core version and their own maturity/version status.

---

## Contact

**AgroNet Labs LLC**  
<https://agronet.ai>  
**E-mail:** admin@agronet.io  
Telegram: @agronetlabs
