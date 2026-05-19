[![Rust Tests](https://img.shields.io/badge/Rust%20Tests-31%2F31%20Passing-brightgreen?style=for-the-badge&logo=rust)](https://github.com/agronetlabs/erc-8040-ecosystem)
[![Python Tests](https://img.shields.io/badge/Python%20Tests-30%2F30%20Passing-brightgreen?style=for-the-badge&logo=python)](https://github.com/agronetlabs/erc-8040-ecosystem)
[![C++ Tests](https://img.shields.io/badge/C%2B%2B%20Tests-2%2F2%20Passing-brightgreen?style=for-the-badge&logo=cplusplus)](https://github.com/agronetlabs/erc-8040-ecosystem)
[![Settlement Live](https://img.shields.io/badge/Settlement-Live%20%E2%9C%85-brightgreen?style=for-the-badge)](https://github.com/agronetlabs/backend)
[![ATF-AI Verified](https://img.shields.io/badge/ATF--AI-VERIFIED-2ea44f?style=for-the-badge)](https://github.com/agronetlabs/ATF-AI)
[![SWIFT ISO 20022](https://img.shields.io/badge/SWIFT-ISO%2020022-orange?style=for-the-badge)]()
![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)

---

# ATF-AI — Autonomous Trust Framework

> *Verifiable provenance, deterministic governance, and zero-trust validation — for any agent, on any infrastructure.*

**Purpose.** ATF-AI is a universal, infrastructure-agnostic framework that answers the question:
> *"How do AI agents prove they are trustworthy, traceable, and auditable — regardless of the underlying infrastructure?"*

**Model.** A **free and open protocol** coordinated by **AgroNet Labs**. Blockchain, cloud, IoT, enterprise, and any other execution environment can implement ATF-AI as a trust layer — independently, without coupling to any specific technology stack.

---

## 🌍 Vision

ATF-AI establishes a governance and trust layer for autonomous agents — decoupled from any specific infrastructure. Any system that needs to prove the legitimacy, provenance, and operational integrity of AI-driven actions can implement ATF-AI.

> "The wheel already exists.  
> We're adding **autonomous navigation, verifiable provenance, and deterministic governance**."

---

## ⚙️ Core Architecture

The ATF-AI protocol operates through three infrastructure-agnostic layers:

1. **Agent Layer** — Autonomous AI agents performing logic, synthesis, validation, and orchestration tasks.
2. **Governance Layer** — Deterministic validation, cryptographic provenance, and zero-trust verification of every agent action.
3. **Execution Layer** — Protocol-agnostic infrastructure executing validated workflows across any runtime environment.

*Any system — cloud, on-premise, decentralized, or embedded — can implement these three layers using ATF-AI's open specification.*

---

## 🔑 Core Pillars

| Pillar | Description |
|--------|-------------|
| **Verifiable Provenance** | Every agent action is cryptographically signed and traceable via in-toto attestations and OpenTelemetry traces. |
| **Deterministic Governance** | Validation rules are explicit, reproducible, and auditable — no hidden logic, no opaque decisions. |
| **Zero-Trust Validation** | No agent or system is implicitly trusted. Every interaction is verified before execution. |

---

## 🔌 Integrations & Adapters

ATF-AI is the framework. Specific technology integrations are **optional downstream adapters** — not core dependencies.

| Adapter | Description | Link |
|---------|-------------|------|
| **erc-8040-ecosystem** | ATF-AI adapter for blockchain/ESG digital asset workflows | [github.com/agronetlabs/erc-8040-ecosystem](https://github.com/agronetlabs/erc-8040-ecosystem) |
| **Documentation** | Live docs on GitHub Pages | [agronetlabs.github.io/atf-ai](https://agronetlabs.github.io/atf-ai/) |

> Want to build an ATF-AI adapter for your infrastructure (cloud, IoT, health, fintech, agro)? See [CONTRIBUTING.md](./CONTRIBUTING.md).

---

## 🔒 Governance & Certification

- Open, AI-assisted governance for validation and certification.
- Coordinated through **AgroNet Labs**, strictly following the **Autonomous Trust Framework (ATF)** specification.
- See [GOVERNANCE.md](./GOVERNANCE.md) for full governance model.

<br>
<div align="center">
  <img src="assets/certificates/imagem.png" alt="ATF Governance Certificate" width="80%">  
  <br><br>
  <img src="assets/certificates/imagem2.png" alt="ATF Compliance Certificate" width="80%">  
</div>
<br>

---

## ⚖️ License

Openly distributed under **MIT License**.  
Implementation and certification trademarks remain under **AgroNet Labs** governance.

---

## ✅ Proof of Build

**73 tests passing across 4 languages. Zero failures.**

| Component | Language | Tests | Status |
|-----------|----------|-------|--------|
| ERC-8040 Core | Rust | 31/31 | ✅ Passing |
| Python SDK | Python 3.12 | 30/30 | ✅ Passing |
| C++ SDK | C++17 | 2/2 | ✅ Passing |
| Backend (Settlement) | Rust/Axum | 10/10 | ✅ Passing |
| **Total** | | **73/73** | **✅ Zero failures** |

### ATF-AI Audit Hash — Live Settlement

![ATF-AI Settlement Live](assets/proof/atf-ai-audit-hash-live.jpg)

`ATF-AI-AUDIT-{SHA256}` generated automatically on every settlement operation.

### Backend Build — All Tests Passing

![Backend Build Passing](assets/proof/backend-build-passing.jpg)

Clean Rust build, 10/10 unit tests passing, server live.

---

## 📬 Contact

**AgroNet Labs LLC**  
<https://agronet.ai>  
**E-mail:** admin@agronet.io  
Telegram: @agronetlabs
