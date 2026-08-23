# ATF-AI Adapter Specification: RWA Privileged Action Governance

**Status:** DRAFT v0.1  
**Adapter ID:** `atf-adapter-rwa-privileged-governance`  
**Parent Spec:** `atf-core-v1` — see [`specs/atf-core-v1.md`](../atf-core-v1.md)  
**Maintained by:** AgroNet Labs  
**Last Updated:** 2026-08-22

---

## 1. Purpose

This adapter defines how ATF-AI governs **privileged actions in Real-World Asset (RWA) systems** before cryptographic execution.

The core principle is:

> **A private key is an execution mechanism, not an authority.**

Possession of an administrator key, multisig signer, HSM credential, or MPC share MUST NOT by itself be sufficient to authorize a privileged RWA action. The action MUST first satisfy deterministic ATF-AI governance policy and produce an auditable authorization record.

This adapter is designed for RWA systems where smart contracts may be technically sound while material exposure remains in operational paths such as redemption, privileged administration, key handling, oracle replacement, upgradeability, or compliance overrides.

---

## 2. Scope

This specification covers governance for privileged actions including:

- asset issuance and minting;
- burning and redemption;
- freeze and unfreeze operations;
- pause and unpause operations;
- contract upgrades;
- oracle replacement;
- compliance-policy changes;
- signer or administrator changes;
- emergency actions;
- settlement authorization.

This specification does **not** replace:

- HSMs;
- MPC systems;
- multisig wallets;
- hardware wallets;
- smart-contract audits;
- custodian controls;
- legal or regulatory review.

Those systems remain execution and assurance mechanisms. ATF-AI supplies the deterministic governance layer that decides whether an action is eligible to reach them.

---

## 3. Architecture

```text
Privileged Action Request
          |
          v
+---------------------------+
|   ATF-AI Governance       |
|---------------------------|
| identity / role policy    |
| compliance policy         |
| asset-state validation    |
| quorum / segregation      |
| timelock                  |
| provenance / evidence     |
| action-specific rules     |
+---------------------------+
          |
          | approved attestation
          v
+---------------------------+
| Execution Security        |
| MPC / Multisig / HSM      |
+---------------------------+
          |
          v
Blockchain / Custodian / Settlement Rail
          |
          v
Execution Receipt + Provenance Record
```

The adapter MUST preserve separation between **governance authorization** and **cryptographic signing**.

---

## 4. Required Roles

Implementations MAY define additional roles, but SHOULD model at least the following responsibilities:

| Role | Responsibility |
|---|---|
| `requester` | Initiates a privileged action request. |
| `issuer` | Represents the regulated or contractual asset issuer. |
| `compliance_officer` | Confirms policy and eligibility requirements. |
| `security_officer` | Confirms operational-security requirements. |
| `settlement_operator` | Controls or confirms off-chain settlement/redemption execution. |
| `governance_validator` | Applies the deterministic ATF-AI policy. |
| `signer` | Holds a cryptographic signing capability after governance approval. |

A single person or system SHOULD NOT simultaneously satisfy every privileged role for high-impact actions.

---

## 5. Privileged Action Record

Every governed privileged action MUST include the ATF-AI Core fields and the following adapter fields:

| Field | Type | Description |
|---|---|---|
| `action.kind` | enum | Privileged operation being requested. |
| `action.asset_id` | URI/string | Asset or instrument affected. |
| `action.target` | string | Contract, wallet, custodian, or settlement target. |
| `action.value` | optional decimal/string | Economic value or token quantity affected. |
| `requester.id` | URI/string | Stable requester identity. |
| `requester.roles` | array | Roles asserted by requester. |
| `governance.policy_id` | URI/string | Immutable policy version used for evaluation. |
| `governance.required_quorum` | integer | Minimum approvals required. |
| `governance.approvals` | array | Distinct approvals and role bindings. |
| `governance.timelock_until` | optional ISO 8601 | Earliest permitted execution time. |
| `compliance.attestations` | array | Compliance attestations required by policy. |
| `provenance.hash` | SHA-256 | Hash of the normalized action payload/evidence. |
| `execution.method` | enum | `multisig`, `mpc`, `hsm`, `hardware-wallet`, or other declared method. |
| `execution.tx_ref` | optional string | Blockchain or settlement execution reference. |

The canonical machine-readable representation is defined in [`../atf-rwa-privileged-action.schema.json`](../atf-rwa-privileged-action.schema.json).

---

## 6. Deterministic Governance Requirements

Before an action can receive `approved`, the governance validator MUST verify all policy requirements applicable to that action.

At minimum:

1. **Identity** — requester and approvers are verified identities.
2. **Role authorization** — required roles are present and valid.
3. **Distinct approvals** — quorum counts distinct approving identities.
4. **Segregation of duties** — policy-defined incompatible roles are not collapsed into one authority.
5. **Compliance** — required attestations are valid and not expired or revoked.
6. **Asset state** — the asset is in a state that permits the requested operation.
7. **Timelock** — required waiting period has elapsed.
8. **Provenance** — request payload and evidence have an immutable provenance hash.
9. **Execution binding** — the approved request is cryptographically bound to the exact transaction or settlement instruction eventually signed.

An implementation MUST reject execution when the signed transaction materially differs from the approved action payload.

---

## 7. Admin-Key Governance

The adapter treats administrator-key exposure as a governance problem in addition to a cryptographic-security problem.

### 7.1 Required Principle

No privileged key SHOULD be able to unilaterally perform a high-impact action solely because the key is valid.

For high-impact operations, implementations SHOULD require:

- role-separated approval;
- threshold signing or equivalent;
- deterministic policy validation;
- explicit transaction-intent binding;
- timelock where operationally appropriate;
- post-execution attestation.

### 7.2 Recommended High-Impact Actions

The following actions SHOULD default to enhanced governance:

- `upgrade_contract`;
- `change_signer`;
- `change_oracle`;
- `change_compliance_rule`;
- `mint` above a policy threshold;
- `burn` above a policy threshold;
- `redemption` above a policy threshold;
- `emergency_action`.

---

## 8. Redemption Governance

Redemption is treated as an end-to-end state transition across on-chain and off-chain systems.

A redemption authorization SHOULD validate:

1. holder identity and entitlement;
2. token ownership and available balance;
3. asset status and redemption eligibility;
4. jurisdiction and compliance restrictions;
5. redemption amount and limits;
6. issuer authorization;
7. settlement destination integrity;
8. required approvals/quorum;
9. burn/freeze sequencing;
10. final settlement evidence.

### 8.1 State Model

```text
REQUESTED
   |
   v
VALIDATED
   |
   v
APPROVED
   |
   +------> REJECTED
   |
   v
LOCKED / BURN-PENDING
   |
   v
SETTLEMENT-PENDING
   |
   v
SETTLED
   |
   v
ATTESTED
```

Every transition MUST be traceable through ATF-AI provenance records.

### 8.2 Atomicity Requirement

Where full technical atomicity between blockchain and off-chain settlement is impossible, the implementation MUST define explicit compensating controls and failure states. A token MUST NOT silently transition to a final redeemed state without evidence of the corresponding settlement outcome.

---

## 9. Governance Attestation

An approved privileged action MUST produce a Governance Attestation containing at least:

```json
{
  "type": "governance",
  "standard": "ATF-AI-RWA-PRIVILEGED-ACTION-v0.1",
  "action_id": "urn:uuid:...",
  "policy_id": "atf-policy:rwa:redemption:1.0.0",
  "decision": "approved",
  "provenance_hash": "<sha256>",
  "required_quorum": 3,
  "approval_count": 3,
  "execution_eligible_at": "2026-08-24T12:00:00Z",
  "validator": "did:example:governance-validator",
  "issued_at": "2026-08-22T12:00:00Z"
}
```

The attestation SHOULD be referenced by the eventual blockchain transaction, custodian instruction, or settlement receipt whenever the target infrastructure supports such binding.

---

## 10. Example Policy

```yaml
action: upgrade_contract
policy_id: atf-policy:rwa:upgrade:1.0.0
required_quorum: 3
required_roles:
  - issuer
  - compliance_officer
  - security_officer
require_distinct_approvers: true
timelock: 48h
checks:
  - implementation_hash_verified
  - audit_attestation_valid
  - no_active_redemptions
  - jurisdiction_policy_valid
provenance:
  required: true
execution:
  allowed_methods:
    - mpc
    - multisig
```

---

## 11. Relationship to ERC-8040

ERC-8040 remains the ATF-AI adapter for blockchain/ESG digital-asset workflows. This adapter is complementary: it governs **privileged lifecycle operations** around an RWA token or digital asset.

```text
ATF-AI Core
   |
   +-- ERC-8040 Adapter
   |      Token / ESG / provenance / compliance mapping
   |
   +-- RWA Privileged Governance Adapter
          admin actions / redemption / signer policy / settlement authorization
```

An ERC-8040 implementation MAY adopt this adapter for privileged operations without changing the ERC-8040 token interface itself.

---

## 12. Conformance

A conforming implementation MUST:

1. implement the ATF-AI Core action and provenance fields;
2. validate privileged actions before execution;
3. prevent an execution layer from treating key possession alone as governance authorization;
4. bind approvals to a specific action payload and policy version;
5. enforce quorum using distinct identities;
6. record post-execution outcome and reference it from provenance;
7. support deterministic rejection or escalation when policy conditions are unmet.

A conforming implementation SHOULD:

1. use MPC, multisig, HSM, or equivalent institutional key protection;
2. support timelocks for high-impact actions;
3. segregate issuer, compliance, security, and settlement roles;
4. expose governance attestations to independent auditors;
5. maintain an explicit redemption state machine.

---

## 13. Security Notes

This adapter reduces governance concentration and operational key risk; it does not claim to eliminate compromise risk. Implementers MUST separately address endpoint security, signer-device compromise, social engineering, malicious governance participants, oracle manipulation, custodian failure, and jurisdiction-specific legal requirements.

AI agents MAY analyze evidence, construct transaction intent, or perform deterministic validation. AI agents SHOULD NOT directly possess unrestricted production private keys solely by virtue of being an ATF-AI agent.

---

## 14. Versioning

This adapter begins at `DRAFT v0.1`. Findings from independent RWA security reviews, operational implementations, and external auditors are expected to refine the threat model before a stable release.

Breaking changes follow the ATF-AI Governance Model.

---

## 15. References

- [ATF-AI Core Spec v1](../atf-core-v1.md)
- [ATF-AI Governance](../../docs/concepts/governance.md)
- [ATF-AI Attestations](../../docs/concepts/attestation.md)
- [ATF-AI Zero-Trust](../../docs/concepts/zero-trust.md)
- [ERC-8040 Adapter Specification](./erc8040.md)
