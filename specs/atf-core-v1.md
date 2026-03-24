# ATF-AI Core Specification — v1

**Version:** 1.0  
**Status:** Stable  
**Coordinated by:** AgroNet Labs LLC

---

## Overview

This document defines the **ATF-AI Core Protocol** — the infrastructure-agnostic specification for establishing trust, provenance, and governance in autonomous AI agent systems.

Any system that implements the requirements defined in this specification may claim ATF-AI compliance, regardless of the underlying infrastructure (cloud, blockchain, on-premise, IoT, embedded, etc.).

---

## Required Fields

Every ATF-AI-governed agent action record must include the following fields:

### Provenance

| Field | Type | Description |
|-------|------|-------------|
| `agent_id` | URI | Unique, stable identifier for the agent that produced the action. |
| `action_id` | URI | Unique identifier for this specific action instance. |
| `timestamp` | ISO 8601 datetime | When the action was initiated. |
| `provenance.attestation` | in-toto statement (JSON) | Cryptographic attestation of the action's origin and integrity. |
| `provenance.hash` | SHA-256 hex string | Hash of the action payload, used for tamper detection. |

### Traceability

| Field | Type | Description |
|-------|------|-------------|
| `trace.id` | OpenTelemetry Trace ID | Distributed trace identifier for cross-system observability. |
| `trace.span_id` | OpenTelemetry Span ID | Span identifier for this specific action within the trace. |

### Governance

| Field | Type | Description |
|-------|------|-------------|
| `governance.policy_id` | URI | Reference to the governance policy version that validated this action. |
| `governance.decision` | enum: `approved`, `rejected`, `escalated` | The outcome of governance validation. |
| `governance.validator` | URI | Identifier of the governance validator that made the decision. |

---

## Layer Requirements

### Agent Layer
- Agents MUST emit a compliant provenance record for every action before execution.
- Agents MUST propagate trace context across all downstream calls.
- Agents MUST declare their `agent_id` using a stable, resolvable URI.

### Governance Layer
- Governance validators MUST apply deterministic, version-controlled rules.
- Governance validators MUST produce an auditable decision record for every evaluated action.
- The `governance.policy_id` MUST reference a specific, immutable version of the policy.

### Execution Layer
- Execution MUST NOT proceed without a `governance.decision` of `approved`.
- Execution results MUST be appended to the provenance record after completion.
- Failures MUST trigger an auditable escalation or rollback record.

---

## Adapter Compatibility

This specification defines the protocol requirements. Infrastructure-specific adapters may extend this specification with additional fields relevant to their domain (e.g., blockchain transaction references, cloud resource IDs, IoT device identifiers), provided that all required fields above are present and valid.

Adapters must document any extensions in their own adapter specification, referencing this document as the base.

---

## Versioning

This is version `1.0` of the ATF-AI Core Specification. Future versions will follow [Semantic Versioning](https://semver.org/). See [GOVERNANCE.md](../GOVERNANCE.md) for the versioning policy.
