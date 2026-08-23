# ATF-AI Architecture

**ATF-AI** provides a universal trust and governance layer for autonomous AI agents and governed machine actions. Its architecture is infrastructure-agnostic: the three core layers can be implemented across cloud, on-premise, decentralized networks, embedded systems, financial infrastructure, or other execution environments.

---

## Three-Layer Architecture

### 1. Agent Layer

The Agent Layer is where autonomous AI agents or other governed actors perform logic, synthesis, validation, orchestration, or action preparation.

**ATF-AI requirements at this layer:**

- Every governed action must emit a verifiable provenance record.
- Trace context must propagate across downstream calls where supported.
- Agents/actors must declare identity, capabilities, and governance scope before execution.

This layer is runtime-agnostic: LLM agents, deterministic automation, RPA, multi-agent frameworks, and non-AI system actors can participate when they satisfy the same governance contract.

---

### 2. Governance Layer

The Governance Layer is the core of ATF-AI. It determines whether an action is valid, authorized, auditable, and compliant with the declared policy.

**ATF-AI requirements at this layer:**

- Validation rules must be deterministic and version-controlled.
- Every governance decision must produce an auditable record.
- The policy version must be explicitly declared and independently verifiable.
- High-impact actions must define human or institutional oversight points where required.
- Technical capability or credential possession must not automatically equal governance authority.

The Governance Layer can be implemented with policy-as-code engines, databases, smart contracts, rule engines, institutional approval systems, or other auditable mechanisms.

---

### 3. Execution Layer

The Execution Layer performs workflows only after governance approval.

**ATF-AI requirements at this layer:**

- Only governance-approved workflows may execute.
- Execution must be bound to the approved action intent where material.
- Execution results must be written back to the provenance record.
- Failures must trigger auditable rollback, compensation, or escalation paths.

Execution targets may include cloud functions, blockchain transactions, database writes, APIs, custodians, settlement rails, HSM/MPC signing systems, or physical actuators.

---

## Core Specifications

| Artifact | Description |
|---|---|
| [`specs/atf-core-v1.md`](../specs/atf-core-v1.md) | Infrastructure-agnostic ATF-AI core protocol. |
| [`api/openapi.yaml`](../api/openapi.yaml) | OpenAPI definition for ATF-AI verification interfaces. |
| [`specs/registry.json`](../specs/registry.json) | Example registry / ATF-AI metadata structure. |
| [`docs/manifesto.md`](./manifesto.md) | Governance principles and core values. |
| [`GOVERNANCE.md`](../GOVERNANCE.md) | Operational governance authority and lifecycle rules. |

---

## Adapter Model

ATF-AI defines the trust protocol. Infrastructure- or domain-specific behavior is implemented through **pluggable adapters**.

An ATF-AI adapter must:

1. preserve required ATF-AI provenance and action identity;
2. connect to a deterministic Governance Layer;
3. enforce the declared governance decision before execution;
4. report execution results back through the provenance chain;
5. declare its compatible ATF-AI Core version and maturity status.

### Current Adapter Examples

| Adapter | Scope | Status | Specification |
|---|---|---|---|
| **ERC-8040** | Blockchain / ESG digital assets | DRAFT v0.1 | [`specs/adapters/erc8040.md`](../specs/adapters/erc8040.md) |
| **RWA Privileged Action Governance** | Admin actions, redemption, signer policy, settlement authorization | DRAFT v0.1 | [`specs/adapters/rwa-privileged-governance.md`](../specs/adapters/rwa-privileged-governance.md) |

The ERC-8040 implementation ecosystem is maintained at [agronetlabs/erc-8040-ecosystem](https://github.com/agronetlabs/erc-8040-ecosystem).

---

## RWA Privileged-Action Pattern

For institutional digital assets, ATF-AI separates governance authorization from cryptographic execution:

```text
Action Intent
    |
    v
ATF-AI Policy Validation
    |
    v
Governance Attestation
    |
    v
MPC / Multisig / HSM / Institutional Signer
    |
    v
Blockchain / Custodian / Settlement Rail
    |
    v
Execution Receipt + Provenance
```

A valid private key, signer, MPC share, or administrator credential is treated as an **execution capability**, not sufficient governance authority by itself.

---

## Governance and AI Assistance

ATF-AI is **human-governed and AI-assisted**.

Protocol authority is coordinated by AgroNet Labs according to [`GOVERNANCE.md`](../GOVERNANCE.md). AI systems may assist architecture, implementation, testing, documentation, or security analysis, but they do not independently approve protocol changes, issue certification, or self-merge contributions.

As of the **2026-08-23 governance review**, the current OpenAI assistance context includes **GPT-5.6 Sol**. GitHub Copilot and other AI tools may also assist development. These model/tool identities are historical contribution metadata only — **ATF-AI has no dependency on any specific AI vendor or model**.

See [`CONTRIBUTING.md`](../CONTRIBUTING.md) for AI-assistance disclosure guidance.
