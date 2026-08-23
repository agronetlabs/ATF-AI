# RWA Privileged Action Governance

## Overview

The **RWA Privileged Action Governance Adapter** applies ATF-AI deterministic governance to high-impact Real-World Asset lifecycle operations before they reach a signing or settlement layer.

Its central rule is simple:

> **Key possession is not governance authorization.**

A multisig signer, MPC participant, HSM credential, custodian operator, or administrator account may be technically able to execute an action. ATF-AI requires the action itself to be independently validated against an immutable policy before execution becomes eligible.

See the formal specification: [`specs/adapters/rwa-privileged-governance.md`](../../../specs/adapters/rwa-privileged-governance.md).

---

## Why this matters for RWA

RWA systems cross several trust boundaries at once:

- smart contracts;
- issuers;
- compliance providers;
- custodians;
- settlement systems;
- off-chain asset records;
- privileged signing infrastructure.

A contract can be technically correct while the overall system still carries material exposure in:

- administrator permissions;
- signer concentration;
- redemption workflows;
- upgradeability;
- oracle replacement;
- settlement destination changes;
- compliance overrides.

ATF-AI addresses this class of risk by separating **authorization** from **execution**.

---

## Reference Flow

```text
RWA Action Request
       |
       v
ATF-AI Policy Evaluation
       |
       +-- identity verified?
       +-- roles authorized?
       +-- compliance valid?
       +-- asset state valid?
       +-- quorum satisfied?
       +-- timelock elapsed?
       +-- evidence/provenance valid?
       +-- execution payload bound?
       |
       v
Governance Attestation
       |
       v
MPC / Multisig / HSM / Custodian
       |
       v
Blockchain or Settlement Rail
       |
       v
Execution Receipt
       |
       v
ATF-AI Provenance Update
```

The execution layer MUST NOT interpret a valid signing credential as sufficient evidence that governance conditions were satisfied.

---

## Admin-Key Pattern

Traditional privileged flow:

```text
Admin credential -> privileged method -> execution
```

ATF-AI-governed flow:

```text
Action intent
   -> deterministic policy validation
   -> governance attestation
   -> threshold / protected signing
   -> exact-payload execution
   -> post-execution provenance
```

This permits security controls such as:

- 3-of-5 or policy-defined quorum;
- role separation;
- transaction-intent binding;
- approval expiry;
- timelocks;
- value thresholds;
- jurisdiction-specific policy;
- independent audit evidence.

---

## Redemption Pattern

Redemption must be treated as an end-to-end state transition rather than a single smart-contract function.

```text
Holder Request
      |
      v
Eligibility + Ownership + Compliance
      |
      v
Issuer / Governance Approval
      |
      v
Token Lock or Burn-Pending
      |
      v
Off-chain Settlement
      |
      +-- success -> finalized redemption + attestation
      |
      +-- failure -> defined rollback / compensation path
```

A production implementation should preserve evidence for both sides of the transition: the blockchain state change and the corresponding off-chain settlement outcome.

---

## Relationship to ERC-8040

ERC-8040 and the RWA Privileged Governance Adapter serve different purposes:

| Component | Responsibility |
|---|---|
| **ATF-AI Core** | Provenance, deterministic governance, zero-trust validation |
| **ERC-8040 Adapter** | ESG/compliance-aware digital asset implementation on EVM-compatible networks |
| **RWA Privileged Governance Adapter** | Admin operations, redemption, signer policy, settlement authorization |
| **MPC / Multisig / HSM** | Cryptographic execution security |
| **Custodian / Settlement Rail** | Off-chain asset or financial settlement |

The adapters can be composed without making the RWA governance layer dependent on ERC-8040.

---

## Recommended First Integration Targets

The first implementation phase should prioritize:

1. `redemption`;
2. `upgrade_contract`;
3. `change_signer`;
4. `change_oracle`;
5. `change_compliance_rule`;
6. high-value `mint` / `burn`;
7. emergency actions.

These operations create the clearest security value because they concentrate operational authority or bridge on-chain state with off-chain consequences.

---

## Machine-Readable Record

The draft JSON Schema for privileged action records is available at:

[`specs/atf-rwa-privileged-action.schema.json`](../../../specs/atf-rwa-privileged-action.schema.json)

The schema binds the governance decision to an exact `execution.payload_hash`, preventing an approved intent from being silently replaced by a materially different transaction.

---

## Status

**Draft v0.1 — threat model and integration interface.**

The adapter is intentionally being introduced as a draft so that real RWA security findings, independent audit feedback, and production integration experience can refine the policy model before a stable release.
