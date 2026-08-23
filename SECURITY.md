# Security Policy

This document describes the security policy for the **Autonomous Trust Framework for Artificial Intelligence (ATF-AI)** specification and canonical reference artifacts maintained in this repository.

---

## Supported Versions

| Version / artifact | Status |
|---|---|
| `ATF-AI Core v1.x` | ✅ Actively maintained |
| Canonical reference code in this repository | ✅ Maintained with repository CI |
| Pre-release / DRAFT adapters | ⚠️ Best-effort security support; interfaces may change |
| External forks and third-party deployments | ❌ Maintained by their respective operators |

Security fixes may be published as specification patches, reference-code changes, adapter revisions, or documented mitigations depending on the affected layer.

---

## Reporting a Vulnerability

**Do not open a public GitHub Issue for an undisclosed security vulnerability.**

Send a private report to:

> **admin@agronet.io**  
> Subject: `[ATF-AI SECURITY] <brief description>`

Include, when available:

- affected component and version;
- vulnerability description;
- reproduction steps or proof-of-concept;
- potential impact;
- suggested mitigation;
- whether exploitation may expose keys, funds, privileged actions, or sensitive data.

### Response Targets

| Stage | Target time |
|---|---|
| **Acknowledgement** | Within 48 hours |
| **Initial assessment** | Within 5 business days |
| **Critical mitigation / resolution target** | Within 30 days |
| **Lower-severity resolution target** | Within 90 days |

These are response targets, not guarantees. Coordinated disclosure timing may be adjusted when user funds, signing infrastructure, or production systems are at risk.

---

## Security Model

### Zero-Trust by Design

No agent, system, credential, signer, or infrastructure component is implicitly trusted. Governed actions must be evaluated against explicit policy before execution.

### Cryptographic Provenance

ATF-AI uses cryptographic provenance and attestations to make actions, decisions, and evidence traceable and tamper-evident.

### Deterministic Governance

Security-sensitive governance decisions must be based on explicit, version-controlled rules rather than hidden or non-reproducible logic.

### Separation of Governance and Execution

For privileged actions, possession of a valid credential is not sufficient governance authority.

```text
Action Intent -> Governance Validation -> Approved Attestation -> Signing / Execution
```

MPC, multisig, HSM, hardware-wallet, or custodian controls remain execution-security mechanisms. ATF-AI governs whether an action is eligible to reach those mechanisms.

### Auditability

Material specification, implementation, and governance changes are tracked through repository history and CI evidence.

---

## RWA and Privileged-Action Threat Surface

The RWA Privileged Action Governance adapter explicitly treats the following as security-sensitive:

- redemption workflows and state transitions;
- admin and signer authority;
- contract upgrades;
- oracle replacement;
- compliance-policy changes;
- mint/burn authority;
- freeze/pause actions;
- settlement destination integrity;
- quorum and segregation of duties;
- timelocks;
- transaction-intent binding;
- rollback / compensation paths;
- post-execution evidence.

Implementations must separately account for endpoint compromise, social engineering, malicious approvers, oracle manipulation, custodian failure, compromised signing devices, and jurisdiction-specific operational requirements.

See [`specs/adapters/rwa-privileged-governance.md`](./specs/adapters/rwa-privileged-governance.md).

---

## Secrets and Key Material

The repository MUST NOT contain production:

- private keys;
- seed phrases;
- wallet recovery material;
- MPC shares;
- HSM secrets;
- API secrets or bearer tokens;
- production signing credentials;
- confidential custodian recovery procedures.

Examples and test fixtures must use obviously non-production values.

AI agents SHOULD NOT directly possess unrestricted production private keys solely because they participate in an ATF-AI workflow.

---

## Scope

### In Scope

- ATF-AI core specifications under `specs/`;
- canonical schemas maintained in this repository;
- reference implementation code under `adapters/`;
- the RWA Privileged Action Governance reference implementation;
- ATF-AI OpenAPI artifacts;
- governance, provenance, and security documentation maintained here;
- CI logic that validates canonical ATF-AI artifacts.

### Out of Scope

- external forks of ATF-AI;
- production deployments operated by third parties;
- third-party wallet, HSM, MPC, custodian, cloud, or blockchain infrastructure;
- independent implementations not maintained by AgroNet Labs;
- GitHub platform vulnerabilities.

The separate [`agronetlabs/erc-8040-ecosystem`](https://github.com/agronetlabs/erc-8040-ecosystem) repository maintains its own implementation/security lifecycle; canonical adapter artifacts copied or maintained inside this repository remain in scope here.

---

## Security and Governance

Security changes may use an expedited governance path when delay would materially increase risk. Human Maintainer/Protocol Steward approval remains required, and the decision should produce an auditable post-merge governance record.

AI-generated security analysis is evidence/input only and MUST NOT be treated as independent approval.

See [`GOVERNANCE.md`](./GOVERNANCE.md).

---

## Contact

**AgroNet Labs LLC**  
<https://agronet.ai>  
**Security E-mail:** admin@agronet.io
