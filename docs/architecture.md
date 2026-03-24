# ATF-AI Architecture

**ATF-AI** provides a universal trust and governance layer for autonomous AI agents. Its architecture is designed to be infrastructure-agnostic: the three core layers can be implemented on any runtime environment — cloud, on-premise, decentralized networks, or embedded systems.

---

## Three-Layer Architecture

### 1. Agent Layer

The Agent Layer is where autonomous AI agents operate. Agents in this layer are responsible for executing tasks, synthesizing information, generating outputs, and coordinating with other agents.

**ATF-AI requirements at this layer:**
- Every agent action must emit a verifiable provenance record (in-toto attestation).
- Every agent interaction must carry a distributed trace identifier (OpenTelemetry-compatible).
- Agents must declare their capabilities and governance scope before execution.

*This layer is agnostic to the agent runtime — it applies equally to LLM-based agents, rule-based automation, robotic process automation, and multi-agent orchestration frameworks.*

---

### 2. Governance Layer

The Governance Layer is the core of ATF-AI. It enforces the rules that determine whether an agent action is valid, auditable, and compliant with the declared governance policy.

**ATF-AI requirements at this layer:**
- Validation rules must be deterministic and version-controlled.
- Every governance decision must produce an auditable record.
- The governance policy must be explicitly declared and independently verifiable.
- Human oversight integration points must be defined for high-impact decisions.

*This layer is agnostic to the governance technology — it can be implemented using smart contracts, traditional databases, policy-as-code frameworks (e.g., OPA), or any auditable rule engine.*

---

### 3. Execution Layer

The Execution Layer is where validated agent workflows are carried out. By the time a workflow reaches this layer, it has been approved by the Governance Layer and carries a complete provenance trail from the Agent Layer.

**ATF-AI requirements at this layer:**
- Only governance-approved workflows may execute.
- Execution results must be written back to the provenance record.
- Failures must trigger auditable rollback or escalation paths.

*This layer is agnostic to the execution infrastructure — cloud functions, blockchain transactions, database writes, API calls, or physical actuators all qualify as execution targets.*

---

## Core Specifications

| Artifact | Description |
|----------|-------------|
| `specs/atf-core-v1.md` | The ATF-AI core protocol specification — infrastructure-agnostic. |
| `api/openapi.yaml` | OpenAPI definition for the ATF-AI verification endpoint. |
| `specs/registry.json` | Sample agent registry demonstrating ATF-AI metadata structure. |
| `docs/manifesto.md` | Governance principles and core values. |

---

## Adapter Model

ATF-AI defines the trust protocol. Specific technology integrations are built as **pluggable adapters** that implement the ATF-AI specification on top of a particular infrastructure.

An ATF-AI adapter must:
1. Implement the Agent Layer interface (provenance emission, trace propagation).
2. Connect to a Governance Layer that satisfies ATF-AI's determinism and auditability requirements.
3. Report execution results back through the provenance chain.

**Example adapters (not part of ATF-AI core):**

| Adapter | Infrastructure | Repository |
|---------|---------------|------------|
| erc-8040-ecosystem | Blockchain / ESG digital assets | [github.com/agronetlabs/erc-8040-ecosystem](https://github.com/agronetlabs/erc-8040-ecosystem) |

Any team may build and publish an ATF-AI adapter for their infrastructure domain. See [CONTRIBUTING.md](../CONTRIBUTING.md) for adapter contribution guidelines.

---

## Governance

The protocol is governed under AgroNet Labs, with AI-assisted coordination by Copilot and Codex GPT. See [GOVERNANCE.md](../GOVERNANCE.md) for the full governance model.
