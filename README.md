[![ATF-AI Verified](https://img.shields.io/badge/ATF--AI-VERIFIED-2ea44f?style=for-the-badge&logo=vercel)](https://github.com/agronetlabs/AgroPay/blob/main/docs/agropay-core-attestation.md)
[![Provenance Traceable](https://img.shields.io/badge/PROVENANCE-SIGNED-0f9d58?style=for-the-badge&logo=oci)](https://github.com/agronetlabs/AgroPay/blob/main/docs/agropay-core-attestation.md)
[![Pull Shark](https://img.shields.io/badge/PULL--SHARK-ACTIVE-0066ff?style=for-the-badge&logo=github)](https://github.com/agronetlabs/AgroPay)

[![Crates.io](https://img.shields.io/crates/v/agrocrypto-core.svg)](https://crates.io/crates/agrocrypto-core)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/agrocrypto-core)](https://opensource.org/licenses)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Status](https://img.shields.io/badge/project-Verified%20Blockchain%20Infra-orange)
![Deployed](https://img.shields.io/badge/deployed-AWS-blue)
![Deployed](https://img.shields.io/badge/deployed-Cloudflare-orange)
![Deployed](https://img.shields.io/badge/deployed-OpenAI-black)

---

# ATF-AI — Open Compliance Protocol (EIP-8040)

**Purpose.** Digitalização de ativos financeiros com IA, compliance e interoperabilidade, alinhado ao EIP-8040.  
**Model.** Protocolo **gratuito** para adoção; **implementações certificadas** coordenadas pela **AgroNet Labs**.

## Components
- **Specs**: `/specs` — EIP-8040-ATF (v1.0), schema JSON, registry
- **API**: `/api` — OpenAPI `/verify` + exemplos
- **Docs**: `/docs` — manifesto, arquitetura, governança

> “A roda já existe. Estamos colocando **eixo, direção e freio** — com IA, compliance e interoperabilidade.”

![Spec CI](https://github.com/agronetlabs/atf-ai/actions/workflows/ci.yml/badge.svg)
**Purpose.** AI-driven digitalization of financial assets with built-in compliance, auditability, and interoperability — fully aligned to **EIP-8040**.  
**Model.** A **free and open protocol** for adoption by enterprises, governments, and technology providers.  
Certified implementations are **coordinated by AgroNet Labs** under the **AgroCrypto Trust Framework (ATF-AI)**.

---

## 🌍 Vision

ATF-AI establishes a **machine-readable layer of financial compliance** — allowing any asset, registry, or institution to prove legitimacy and origin on-chain or off-chain.  
It provides the missing bridge between **digital asset representation (EIP-8040)** and **institutional governance**.

> “The wheel already exists.  
> We’re adding the **axle, steering, and brakes** — with AI, compliance, and interoperability.”

---

## 🧩 Components

| Module | Description |
|---------|--------------|
| **/specs** | Core specification — EIP-8040-ATF schema (v1.0), registry, and compliance data model. |
| **/api** | RESTful API — OpenAPI `/verify` endpoint for validation and attestation. |
| **/docs** | Manifesto, governance policy, and architectural diagrams. |
| **/examples** | Reference JSON assets and verification scripts. |

---

## ⚙️ Architecture

The ATF-AI protocol operates in **three layers**:

1. **Descriptor Layer (EIP-8040)** — Defines digital asset metadata.  
2. **Compliance Layer (ATF-AI)** — Adds attestations, provenance, and jurisdiction context.  
3. **Verification Layer (ATF-AI Verify API)** — Performs validation and digital signature (JWS).

Each implementation can be extended with **AI validators** (e.g. GPT-5, Gemma, LLaMA) for semantic and jurisdictional compliance checks.

---

## 🔒 Governance

- Open, AI-assisted **DAO governance** for validation and certification.  
- Coordinated through **AgroNet Labs**, following the **AgroCrypto Trust Framework (ATF)**.  
- Institutional integration available via **ISIN**, **LEI**, and **OIDC/JWT** standards.

---

## 🧪 Development Status

| Stage | Component | Status |
|--------|------------|--------|
| Spec v1.0 | `specs/eip-8040-atf.schema.json` | ✅ Stable |
| API Verify | `api/openapi.yaml` | ✅ Available |
| CI Validation | `.github/workflows/ci.yml` | ✅ Passing |
| Pages Docs | `/docs` | 🚀 Published |

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
<https://agrocrypto.network>  
**E-mail:** support@agronet.io
