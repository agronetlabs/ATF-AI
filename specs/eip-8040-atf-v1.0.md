# EIP-8040-ATF v1.0 — Compliance Layer
**Scope.** Auditable digital representation of financial assets, validated through **ATF-AI** and anchored per **EIP-8040**.

## Required Fields
- `asset_id` (string, URI)
- `issuer` (DID or LEI)
- `jurisdiction` (ISO-3166-1 alpha-2)
- `compliance.attestations[]` (OIDC/JWT or CCF)
- `provenance.hash` (SHA-256)
- `ledger.ref` (chainId, tx, block)
