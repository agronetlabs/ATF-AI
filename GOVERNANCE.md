# ATF-AI Governance

This document describes the governance model for the **ATF-AI — Autonomous Trust Framework**. ATF-AI is an open protocol coordinated by **AgroNet Labs**, but the framework itself is infrastructure-neutral and independent of any specific technology stack.

---

## Governance Model

ATF-AI operates as an **open protocol with coordinated stewardship**. This means:

- The specification and core documentation are publicly available and freely adoptable by any individual, team, or organization.
- **AgroNet Labs** acts as the coordinating body — responsible for versioning the spec, reviewing proposals, and maintaining the integrity of the ATF-AI certification process.
- No single infrastructure, vendor, or technology is privileged within the framework. Blockchain, cloud, enterprise, IoT, and other adapters are all equal consumers of the protocol.

---

## Decision Making

Changes to the ATF-AI specification follow a transparent, structured process:

### Proposal
Any contributor may propose a change by opening a GitHub Issue or Pull Request in this repository. The proposal must include:
- A clear description of the problem being solved.
- The proposed change to the specification or documentation.
- Rationale for why the change is infrastructure-agnostic (or, if adapter-specific, why it belongs in the core spec).

### Review
All proposals are reviewed by the **Maintainers** team. The review criteria are:
1. **Agnosticism** — Does the change apply regardless of the underlying infrastructure?
2. **Correctness** — Is the change technically sound and unambiguous?
3. **Backward compatibility** — Does the change maintain compatibility with existing ATF-AI implementations, or is a version bump required?

### Acceptance
- Minor changes (documentation clarifications, non-breaking additions) require approval from **at least one Maintainer**.
- Breaking changes to the core specification require approval from **AgroNet Labs** and a version bump following the versioning policy below.
- All accepted changes must include updated provenance metadata before merging.

---

## Roles

### Maintainers
Maintainers are responsible for reviewing proposals, merging pull requests, and maintaining the quality of the ATF-AI specification. Maintainers are designated by AgroNet Labs.

**Current Maintainers:**
- Leandro Lemos — Founder & Lead Engineer, AgroNet Labs LLC

### Contributors
Contributors are any individuals or organizations that submit issues, pull requests, documentation improvements, or adapter implementations. All contributors must follow the [Code of Conduct](./CODE_OF_CONDUCT.md) and [Contributing Guidelines](./CONTRIBUTING.md).

### Adapter Authors
Adapter Authors are teams or individuals who build ATF-AI-compatible integrations for specific infrastructures (e.g., blockchain networks, cloud platforms, IoT systems). Adapter Authors:
- Are responsible for maintaining their own adapter repositories.
- May request official ATF-AI Adapter recognition by submitting a proposal to this repository.
- Must not modify or claim ownership of the ATF-AI core specification.

---

## ATF-AI Certification

A system is considered **ATF-AI Certified** when it demonstrably implements all three layers of the ATF-AI specification:

| Requirement | Description |
|-------------|-------------|
| **Agent Layer compliance** | All agent actions emit verifiable in-toto provenance attestations. |
| **Governance Layer compliance** | All validation rules are deterministic, version-controlled, and auditable. |
| **Execution Layer compliance** | Only governance-approved workflows execute, and results are recorded in the provenance chain. |

ATF-AI Certification is **not** tied to any specific blockchain standard, cloud provider, or technology platform. Any compliant implementation — regardless of underlying infrastructure — may apply for certification.

To apply for ATF-AI Certification, contact **AgroNet Labs** at admin@agronet.io.

---

## Versioning

The ATF-AI specification follows [Semantic Versioning](https://semver.org/):

- **MAJOR** versions (`v2.0`, `v3.0`) indicate breaking changes to the core specification.
- **MINOR** versions (`v1.1`, `v1.2`) indicate backward-compatible additions.
- **PATCH** versions (`v1.0.1`) indicate documentation corrections and clarifications.

All versions are published in this repository under `specs/`. The current stable specification is `specs/atf-core-v1.md`.

Adapters are expected to declare which version(s) of the ATF-AI specification they implement.

---

## Contact

**AgroNet Labs LLC**  
<https://agronet.ai>  
**E-mail:** admin@agronet.io  
Telegram: @agronetlabs
