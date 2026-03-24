# Security Policy

This document describes the security policy for the **ATF-AI — Autonomous Trust Framework** specification and its reference artifacts.

---

## Supported Versions

The following versions of the ATF-AI specification currently receive security updates:

| Version | Status |
|---------|--------|
| `v1.x` (current) | ✅ Actively maintained |
| Pre-release drafts | ⚠️ Best-effort only |

When a security issue is identified, a patch release will be published for all actively maintained versions.

---

## Reporting a Vulnerability

If you discover a security vulnerability in the ATF-AI specification, reference implementations, or associated tooling, please report it **responsibly** by following the process below.

### How to Report

**Do not open a public GitHub Issue for security vulnerabilities.**

Instead, send an email to:

> **admin@agronet.io**  
> Subject: `[ATF-AI SECURITY] <brief description>`

Include in your report:
- A clear description of the vulnerability.
- The affected component (specification section, reference artifact, or API definition).
- Steps to reproduce or a proof-of-concept if applicable.
- Your assessment of the potential impact.

### Response SLA

| Stage | Target Time |
|-------|-------------|
| **Acknowledgement** | Within 48 hours of receipt |
| **Initial assessment** | Within 5 business days |
| **Resolution or mitigation** | Within 30 days for critical issues; 90 days for lower severity |

We will keep you informed throughout the process and credit your responsible disclosure in the release notes (unless you prefer to remain anonymous).

---

## Security Model

ATF-AI's security model is built on the same **zero-trust principles** that the framework enforces for AI agent governance:

### Zero-Trust by Design
No component of an ATF-AI-governed system is implicitly trusted. Every agent action, governance decision, and execution event must carry verifiable attestations. This principle applies to ATF-AI's own specification artifacts and reference implementations.

### Cryptographic Provenance
All official ATF-AI specification releases and reference artifacts are signed with cryptographic provenance records (in-toto format). Consumers of the specification can verify the integrity and origin of any official artifact.

### Infrastructure Agnosticism
ATF-AI's security properties are defined at the protocol level — not tied to any specific infrastructure. This means the security model remains consistent regardless of whether ATF-AI is implemented on a blockchain network, a cloud platform, an enterprise system, or an embedded device.

### Auditability
All changes to the ATF-AI specification are tracked in this public repository with full version history. Any observer can audit the complete evolution of the protocol.

---

## Scope

### In Scope
The following are within the scope of ATF-AI's security policy:

- The ATF-AI core specification (`specs/atf-core-v1.md` and future versions).
- The ATF-AI OpenAPI definition (`api/openapi.yaml`).
- Reference artifacts published in this repository (`specs/registry.json`, schema definitions).
- The ATF-AI documentation published on GitHub Pages.

### Out of Scope
The following are **not** covered by this security policy:

- Third-party ATF-AI adapters (e.g., `erc-8040-ecosystem`). Security issues in adapters should be reported to the respective adapter maintainers.
- Implementations of ATF-AI built by external organizations. AgroNet Labs is not responsible for the security of third-party implementations.
- GitHub platform-level security issues. Report those directly to GitHub.

---

## Contact

**AgroNet Labs LLC**  
<https://agronet.ai>  
**Security E-mail:** admin@agronet.io
