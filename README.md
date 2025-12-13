[![ISO 20022 Compatible](https://img.shields.io/badge/ISO%2020022-Compatible-00a651?style=for-the-badge&logo=swift&logoColor=white)](https://www.iso20022.org/)
[![SWIFT Ready](https://img.shields.io/badge/SWIFT-Ready-ff6600?style=for-the-badge&logo=swift&logoColor=white)](https://www.swift.com/)
[![ATF-AI Verified](https://img.shields.io/badge/ATF--AI-VERIFIED-2ea44f?style=for-the-badge&logo=vercel)](https://github.com/agronetlabs/atf-ai)
[![Provenance Traceable](https://img.shields.io/badge/PROVENANCE-SIGNED-0f9d58?style=for-the-badge&logo=oci)](https://github.com/agronetlabs/atf-ai)
[![Copilot](https://img.shields.io/badge/GitHub%20Copilot-Active-0066ff?style=for-the-badge&logo=githubcopilot)](https://github.com/features/copilot)
[![OpenAI Codex](https://img.shields.io/badge/OpenAI%20Codex-Active-ff6600?style=for-the-badge&logo=openai&logoColor=white)](https://github.com/features/copilot)
[![Pull Shark](https://img.shields.io/badge/PULL--SHARK-ACTIVE-0066ff?style=for-the-badge&logo=github)](https://github.com/agronetlabs/AgroPay)

[![Crates.io](https://img.shields.io/crates/v/agrocrypto-core.svg)](https://crates.io/crates/agrocrypto-core)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/agrocrypto-core)](https://opensource.org/licenses)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Status](https://img.shields.io/badge/project-Verified%20Blockchain%20Infra-orange)
![Deployed](https://img.shields.io/badge/deployed-AWS-blue)
![Deployed](https://img.shields.io/badge/deployed-Cloudflare-orange)
![Deployed](https://img.shields.io/badge/deployed-OpenAI-black)

---

# ATF-AI — Open Compliance Protocol (ERC-8040)

> **Compliance-as-a-Service Protocol.**  
> Fully aligned with ERC-8040 and governed by a certified AI-based DAO for institutional-grade digital compliance, auditability, and asset legitimacy.

**Purpose.** AI-driven digitalization of financial assets with built-in compliance, auditability, and interoperability — fully aligned to **ERC-8040**.  
**Model.** A **free and open protocol** for adoption by enterprises, governments, and technology providers.  
Certified implementations are **coordinated by AgroNet Labs** under the **(ATF-AI: Autonomus Trust Framework) (ATF-AI)**.


---

## 🌍 Vision

ATF-AI establishes a **machine-readable layer of financial compliance** — allowing any asset, registry, or institution to prove legitimacy and origin on-chain or off-chain.  
It provides the missing bridge between **digital asset representation (ERC-8040)** and **institutional governance**.

> “The wheel already exists.  
> We’re adding the **axle, steering, and brakes** — with AI, compliance, and interoperability.”

---

## 🏦 ISO 20022 & SWIFT Integration

On **November 22, 2025**, SWIFT completed its historic "banking reset" — the definitive transition to the **ISO 20022** standard. This transformation integrates blockchain-based ledger technology into the global financial infrastructure, enabling tokenized asset movement across **11,000+ financial institutions worldwide**.

**ATF-AI is perfectly positioned for this new era:**

| Capability | ATF-AI Alignment |
|------------|------------------|
| **Rich Data Messaging** | ATF-AI's compliance metadata schema aligns with ISO 20022's structured data requirements |
| **Tokenized Assets** | Full support for institutional-grade digital asset verification |
| **Cross-Border Compliance** | Jurisdiction-aware attestations compatible with international regulatory frameworks |
| **Interoperability** | Seamless integration with SWIFT's new blockchain-enabled infrastructure |

> "The SWIFT ISO 20022 transition marks the beginning of institutional blockchain adoption. ATF-AI provides the compliance layer that makes this integration trustworthy and auditable."

---

## 🧩 Components

| Module | Description |
|---------|--------------|
| **/specs** | Core specification — ERC-8040-ATF schema (v1.0), registry, and compliance data model. |
| **/api** | RESTful API — OpenAPI `/verify` endpoint for validation and attestation. |
| **/docs** | Manifesto, governance policy, and architectural diagrams. |
| **/examples** | Reference JSON assets and verification scripts. |

---

## 🔗 Ecosystem & Reference Implementations

| Project | Description | Link |
|---------|-------------|------|
| **esg-tokenization-protocol** | Reference implementation in Rust for ERC-8040 | [github.com/agronetlabs/esg-tokenization-protocol](https://github.com/agronetlabs/esg-tokenization-protocol) |
| **Crates.io Package** | Production-ready Rust crate | [crates.io/crates/esg-tokenization-protocol](https://crates.io/crates/esg-tokenization-protocol) |
| **ERC-8040 Discussion** | Ethereum Magicians Forum | [ethereum-magicians.org](https://ethereum-magicians.org/t/erc-8040-esg-tokenization-protocol/25846) |
| **Documentation** | Live docs on GitHub Pages | [agronetlabs.github.io/atf-ai](https://agronetlabs.github.io/atf-ai/) |

---

## ⚙️ Architecture

The ATF-AI protocol operates in **three layers**:

1. **Descriptor Layer (EIP-8040)** — Defines digital asset metadata.  
2. **Compliance Layer (ATF-AI)** — Adds attestations, provenance, and jurisdiction context.  
3. **Verification Layer (ATF-AI Verify API)** — Performs validation and digital signature (JWS).

Each implementation can be extended with **AI validators** (e.g. GPT-5, Copilot, Gemma, LLama, Deep Seek, Qwen, models) for semantic and jurisdictional compliance checks.

---

## 🔒 Governance

- Open, AI-assisted **DAO governance** for validation and certification.  
- Coordinated through **AgroNet Labs**, following the **(ATF-AI: Autonomus Trust Framework (ATF)**.  
- Institutional integration available via **ISIN**, **LEI**, and **OIDC/JWT** standards.

---

## 🧪 Development Status

| Stage | Component | Status |
|--------|------------|--------|
| Spec v1.0 | `specs/erc-8040-atf.schema.json` | ✅ Stable |
| API Verify | `api/openapi.yaml` | ✅ Available |
| CI Validation | `.github/workflows/ci.yml` | ✅ Passing |
| Pages Docs | `/docs` | 🚀 Published |
| ISO 20022 Alignment | Schema compatibility | ✅ Ready |

![Spec CI](https://github.com/agronetlabs/atf-ai/actions/workflows/ci.yml/badge.svg)
[![GitHub Pages](https://img.shields.io/badge/docs-live-brightgreen)](https://agronetlabs.github.io/atf-ai/)

---

## ⚖️ License

Openly distributed under **MIT License**.  
Implementation and certification trademarks remain under **AgroNet Labs** governance.

---

## 📬 Contact

**AgroNet Labs LLC**  
<https://agronet.ai>  
<https://agronet.network>  
**E-mail:** admin@agronet.io
Telegram @agronetlabs
