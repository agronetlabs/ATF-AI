# Contributing to ATF-AI

[![ATF-AI Verified](https://img.shields.io/badge/ATF--AI-VERIFIED-2ea44f?style=for-the-badge)](https://github.com/agronetlabs/ATF-AI)
[![Provenance Traceable](https://img.shields.io/badge/PROVENANCE-SIGNED-0f9d58?style=for-the-badge)](https://github.com/agronetlabs/ATF-AI)
[![OpenAI GPT-5.6 Sol](https://img.shields.io/badge/OpenAI-GPT--5.6%20Sol-000000?style=for-the-badge&logo=openai&logoColor=white)](https://openai.com/)
[![GitHub Copilot](https://img.shields.io/badge/GitHub%20Copilot-Active-0066ff?style=for-the-badge&logo=githubcopilot)](https://github.com/features/copilot)

ATF-AI is an open protocol coordinated by **AgroNet Labs LLC**. Contributions are welcome across the core specification, adapters, reference implementations, schemas, tests, documentation, and security review.

The protocol is **human-governed and AI-assisted**. AI systems may materially assist engineering work, but they do not possess protocol authority, voting rights, certification authority, or independent merge authority.

---

## Contribution Scope

Contributions generally fall into one of these categories:

1. **Core protocol** — changes to `specs/atf-core-v1.md` or future core versions.
2. **Adapter specifications** — infrastructure- or domain-specific extensions under `specs/adapters/`.
3. **Reference implementations** — code under `adapters/`, SDKs, schemas, and test suites.
4. **Governance and security** — policy, threat-model, provenance, and review improvements.
5. **Documentation** — architecture, concepts, integration guides, and examples.

If a proposal is domain-specific, prefer an adapter over changing the infrastructure-agnostic core.

---

## Required Contribution Workflow

A contribution should:

- describe the problem being solved;
- identify whether the change affects the **core protocol** or an **adapter**;
- explain backward-compatibility and versioning impact;
- include or update tests where executable behavior changes;
- update JSON Schemas where machine-readable contracts change;
- document security implications for privileged or high-impact operations;
- preserve deterministic governance and provenance requirements;
- pass repository CI before merge.

For material protocol changes, use a Pull Request so the proposal, review, CI evidence, and final decision remain auditable.

---

## AI-Assisted Contributions

AI systems may participate as:

- architecture and specification assistants;
- documentation synthesis agents;
- code generation and refactoring assistants;
- test and validation assistants;
- security and compliance analysis assistants;
- infrastructure and CI/CD assistants.

### Governance Boundary

AI assistance MUST NOT be treated as protocol approval.

An AI system MUST NOT, by itself:

- approve a breaking core-spec change;
- issue ATF-AI certification;
- self-approve its own Pull Request;
- bypass required CI or human review;
- treat possession of repository credentials as governance authority.

Final acceptance remains with the human maintainer(s) defined in [`GOVERNANCE.md`](./GOVERNANCE.md).

### Current OpenAI Contribution Context

As of **2026-08-23**, the OpenAI model used in the current architecture/governance review is **GPT-5.6 Sol**. Its assistance includes specification review, RWA governance architecture, documentation synthesis, reference implementation review, and CI validation support.

This attribution is informational and time-bound. **ATF-AI does not depend on GPT-5.6 Sol, OpenAI, or any other specific model/vendor.** Future contributions may use different AI systems without changing the protocol's governance model.

When AI materially contributes to a change, maintainers are encouraged to disclose that assistance in the Pull Request description or commit metadata, for example:

```text
AI-Assisted-By: OpenAI GPT-5.6 Sol
Human-Reviewed-By: <maintainer>
ATF-AI-Scope: core | adapter | docs | security
```

---

## RWA / Privileged-Action Contributions

Changes affecting RWA lifecycle or privileged operations should explicitly review:

- redemption state transitions;
- admin and signer authority;
- quorum and segregation of duties;
- oracle and compliance-rule changes;
- upgradeability;
- settlement authorization;
- timelocks;
- transaction-intent binding;
- MPC / multisig / HSM execution boundaries;
- rollback and compensating controls;
- provenance and post-execution attestations.

A valid signer credential is an **execution capability**, not sufficient governance authority by itself. See [`specs/adapters/rwa-privileged-governance.md`](./specs/adapters/rwa-privileged-governance.md).

Never commit production private keys, seed phrases, MPC shares, HSM secrets, credentials, or sensitive recovery material.

---

## Adapter Contributions

An adapter proposal should include:

- an adapter ID;
- parent ATF-AI core version;
- purpose and scope;
- conceptual mapping to ATF-AI primitives;
- conformance requirements;
- threat model / security notes;
- version and maturity status (`DRAFT`, `REVIEW`, `STABLE`, or `DEPRECATED`);
- maintainer and disclosure contact.

Official ATF-AI adapter recognition is governed by [`GOVERNANCE.md`](./GOVERNANCE.md).

---

## Contributors Recognition

### Primary Author and Protocol Steward

**Leandro Lemos** — Founder & Lead Engineer, AgroNet Labs LLC  
Architect of the ATF-AI Framework — Autonomous Trust Framework for Artificial Intelligence.

### AI and Tooling Assistance

AI tools and models may be acknowledged when they materially contribute, but contributor recognition does not grant governance authority. Model/vendor names are historical attribution only and must not be interpreted as protocol dependencies or endorsements.

---

## Code of Conduct and Security

All contributors must follow:

- [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md)
- [`GOVERNANCE.md`](./GOVERNANCE.md)
- [`SECURITY.md`](./SECURITY.md)

Security vulnerabilities should be reported privately according to `SECURITY.md`, not through a public issue.
