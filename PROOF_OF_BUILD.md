# Proof of Build — ATF-AI / ERC-8040

This document separates **current repository CI evidence** from **historical broader-ecosystem build snapshots** so test totals are not mixed across different dates or components.

---

## Current ATF-AI Repository CI

**Date:** 2026-08-23  
**Workflow:** `Spec CI`  
**Run:** `#158` / GitHub Actions run `32613970456`  
**Result:** ✅ Success

The RWA Privileged Action Governance update was validated before merge with:

| Check | Result |
|---|---:|
| Rust unit tests | **31/31 passing** |
| Rust integration tests | **6/6 passing** |
| **Current Rust total** | **37/37 passing** |
| `specs/ERC-8040-atf.schema.json` | **valid** |
| `specs/atf-rwa-privileged-action.schema.json` | **valid** |

### New RWA Governance Tests Included

The current Rust unit suite includes the RWA governance cases introduced in August 2026:

```text
rwa::governance::tests::approves_when_policy_is_satisfied ... ok
rwa::governance::tests::escalates_when_quorum_is_incomplete ... ok
rwa::governance::tests::escalates_until_timelock_expires ... ok
rwa::governance::tests::rejects_duplicate_approvers_when_distinct_required ... ok
rwa::governance::tests::rejects_missing_provenance ... ok
rwa::governance::tests::rejects_upgrade_while_redemptions_are_active ... ok
```

Current Rust result:

```text
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

Schema validation result:

```text
schema specs/ERC-8040-atf.schema.json is valid
schema specs/atf-rwa-privileged-action.schema.json is valid
```

The current CI therefore demonstrates that the canonical Rust reference implementation and both machine-readable schemas compile/validate successfully together.

---

## Historical Extended Ecosystem Snapshot

**Date:** 2026-05-18  
**Scope:** ERC-8040 Rust/Python/C++ SDKs + settlement/backend snapshot  
**Result at that date:** ✅ 81/81 tests passing

| Component | Language | Snapshot result |
|---|---|---:|
| ERC-8040 Core | Rust | 31/31 |
| Python SDK | Python 3.12 | 30/30 |
| C++ SDK | C++17 / GCC 15.2.0 | 10/10 |
| Backend (Settlement) | Rust / Axum | 10/10 |
| **Historical total** | **4 component groups** | **81/81** |

> **Important:** `81/81` is a dated ecosystem snapshot, not the current ATF-AI repository test total. The August 2026 RWA governance work added six new Rust unit tests, and the current repository CI now reports **37/37 Rust tests**. Python, C++, and external settlement/backend components were not re-run by the current `Spec CI`, so this document does not manufacture a new combined total.

---

## Historical Rust Snapshot — 2026-05-18

At the May snapshot, the Rust core recorded 25 unit tests plus 6 integration tests:

```text
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

The August 2026 RWA module increased the unit-test count from 25 to 31 while preserving the 6 integration tests.

---

## Historical Python SDK Snapshot — 2026-05-18

```text
platform win32 -- Python 3.12.10
collected 30 items
30 passed in 0.53s
```

Coverage at the snapshot included compliance rules, ESG scoring, SFDR mapping, taxonomy alignment, ISO 20022 message creation, and carbon-intensity estimation.

---

## Historical C++ SDK Snapshot — 2026-05-18

```text
RESULT: 2/2 TEST SUITES | 10/10 PASSED
C++17 | GCC 15.2.0
```

The snapshot included ESG scoring and compliance validation tests.

---

## Historical Backend Settlement Snapshot — 2026-05-18

```text
test result: ok. 10 passed; 0 failed
```

The recorded backend snapshot included settlement validation, double-entry ledger invariants, Ethereum/Tron/CCTP provider tests, liquidity behavior, and authentication flow tests.

### Settlement Evidence

![ATF-AI Settlement Live](assets/proof/atf-ai-audit-hash-live.jpg)

The referenced implementation produced audit identifiers in the form:

```text
ATF-AI-AUDIT-{SHA256}
```

### Backend Build Evidence

![Backend Build Passing](assets/proof/backend-build-passing.jpg)

---

## Evidence Policy

ATF-AI documentation should distinguish:

- **current CI evidence** — reproduced by the repository's active workflow;
- **dated external/component snapshots** — results recorded at a specific point in time;
- **planned or claimed capabilities** — which must not be represented as passing tests without evidence.

A passing CI run demonstrates that the tested artifacts passed the declared checks. It does **not** by itself imply production readiness, external security audit, regulatory approval, adapter stability, or ATF-AI certification.

---

**AgroNet Labs LLC** | [agronet.ai](https://agronet.ai) | [github.com/agronetlabs](https://github.com/agronetlabs)
