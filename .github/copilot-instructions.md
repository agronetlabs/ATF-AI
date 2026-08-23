# ATF-AI Repository Instructions for GitHub Copilot

ATF-AI is an infrastructure-agnostic trust and governance protocol coordinated by AgroNet Labs LLC.

## Governance Boundary

- Treat the repository as **human-governed and AI-assisted**.
- Do not describe AI-generated changes as approved, certified, or authoritative unless an authorized human maintainer has explicitly accepted them.
- Do not self-approve or self-merge material protocol changes.
- CI success is validation evidence, not protocol approval.
- Follow `GOVERNANCE.md`, `CONTRIBUTING.md`, `SECURITY.md`, and `.github/CODEOWNERS`.

## Core vs. Adapter Changes

- Keep `specs/atf-core-v1.md` infrastructure-agnostic.
- Put blockchain-, RWA-, cloud-, IoT-, or other domain-specific behavior in adapters unless it is genuinely universal.
- Adapters must declare parent core compatibility and maturity (`DRAFT`, `REVIEW`, `STABLE`, or `DEPRECATED`).
- Do not represent a `DRAFT` adapter as stable solely because reference code or CI exists.

## RWA Privileged Actions

For admin, redemption, signer, oracle, compliance, upgrade, mint/burn, or settlement operations:

- preserve separation between **governance authorization** and **cryptographic execution**;
- never treat possession of a private key, signer credential, MPC share, or HSM capability as sufficient governance authority by itself;
- preserve quorum, distinct approvers, role separation, timelocks, provenance, compliance evidence, and transaction-intent binding where the applicable policy requires them;
- record deterministic rejection/escalation outcomes;
- maintain rollback or compensating-control paths when atomic execution is impossible.

Reference: `specs/adapters/rwa-privileged-governance.md`.

## Security

Never introduce production:

- private keys;
- seed phrases;
- wallet recovery material;
- MPC shares;
- HSM secrets;
- API secrets or bearer tokens;
- production signing credentials.

Use clearly non-production fixtures in tests and examples.

## Validation

For changes affecting Rust or schemas, run the applicable checks before proposing merge:

```bash
cargo test --workspace
ajv compile -s specs/ERC-8040-atf.schema.json -c ajv-formats
ajv compile -s specs/atf-rwa-privileged-action.schema.json -c ajv-formats
```

Update tests, schemas, documentation, and version metadata together when an interface changes.

## Provenance and Documentation

- Keep governance decisions, action provenance, and security assumptions explicit.
- Prefer deterministic, auditable behavior over hidden heuristics for governance decisions.
- Distinguish current CI evidence from historical build snapshots.
- Distinguish conformance from formal `ATF-AI Certified` status.
- Do not imply external regulatory approval merely because a compliance mapping exists.

## AI Attribution

AI/tool attribution is historical contribution metadata, not a protocol dependency. Do not hard-code a model vendor into ATF-AI Core requirements.
