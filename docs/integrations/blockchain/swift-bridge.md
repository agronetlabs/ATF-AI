# SWIFT / ISO 20022 Bridge

## Overview

The ERC-8040 adapter includes a **SWIFT/ISO 20022 bridge** that enables ESG tokens governed by ATF-AI attestations to communicate with traditional financial infrastructure. This bridge is critical for institutional adoption: fund managers, banks, and regulators operate in ISO 20022 / SWIFT environments and require ESG data in standardized financial message formats.

> **ATF-AI Principle:** The bridge is an adapter-level concern. ATF-AI Core produces attestations; the ERC-8040 adapter translates them into ISO 20022 messages. The ATF-AI layer remains agnostic of SWIFT protocols.

---

## Why ESG Tokens Need to Speak ISO 20022

Traditional financial institutions cannot directly consume on-chain data. The gap between blockchain-native ESG scoring and regulatory reporting systems requires a translation layer:

| Blockchain World | Traditional Finance World |
|---|---|
| On-chain ESG score (0–100 float) | SFDR Article classification (6/8/9) |
| Wallet address | LEI (Legal Entity Identifier) |
| Token transfer event | ISO 20022 SETR message |
| Smart contract state | SWIFT MT/MX message |
| Block timestamp | ISO 8601 settlement date |

The ISO 20022 bridge performs this translation, enabling a seamless flow from ATF-AI attestation → ERC-8040 token → ISO 20022 message → SWIFT network.

---

## Bridge Architecture

```
┌─────────────────┐      ┌──────────────────┐      ┌─────────────────┐
│  ERC-8040 Token │─────►│  ISO 20022       │─────►│  SWIFT Network  │
│  ESG Scores     │      │  Bridge          │      │  Financial Inst │
└─────────────────┘      └──────────────────┘      └─────────────────┘
     Blockchain              Transformation           Traditional Finance
```

### Bridge Components

1. **ESG Score Reader**: Reads the current ESG score from the ERC-8040 token metadata (which carries ATF-AI provenance hashes)
2. **SFDR Classifier**: Maps the composite ESG score to an SFDR Article classification
3. **ISO 20022 Encoder**: Generates a compliant ISO 20022 XML message (SETR.044 or SETR.045)
4. **SWIFT Dispatcher**: Transmits the message to the SWIFT network via BIC routing

---

## ESG Rating → SFDR Article Mapping

ATF-AI attestation results are mapped to SFDR regulatory classifications through the following table:

| ESG Rating | Score Range | SFDR Classification | Description |
|---|---|---|---|
| **AAA** | 95–100 | Article 9 | Sustainable investment — highest standard |
| **AA** | 90–94 | Article 9 | Sustainable investment |
| **A** | 80–89 | Article 9 | Sustainable investment — broad alignment |
| **BBB** | 70–79 | Article 8 | ESG promoted — not exclusively sustainable |
| **BB** | 60–69 | Article 8 | ESG characteristics promoted |
| **B** | 50–59 | Article 6 | No ESG integration claimed |
| **CCC** | 40–49 | Article 6 | Minimal ESG consideration |
| **CC** | 30–39 | Article 6 | Non-compliant with ESG criteria |
| **C** | 20–29 | Below Article 6 | Significant ESG risks identified |
| **D** | 0–19 | Below Article 6 | Does not meet minimum ESG standards |

---

## Python SDK Usage Example

The following example demonstrates the full flow from ATF-AI-produced ESG score to ISO 20022 message generation:

```python
from erc8040_sdk import ESGScore, ISO20022Bridge, FinancialInstrument

# Step 1: Create ESG score (would normally come from ATF-AI attestation)
score = ESGScore.create(environmental=90.0, social=85.0, governance=80.0)

# Step 2: Initialize the ISO 20022 bridge
bridge = ISO20022Bridge()

# Step 3: Map ESG score to SFDR classification
classification = bridge.esg_to_iso(score)
# classification.sfdr_article == "Article 9"
# classification.esg_rating == "AA"

# Step 4: Define the financial instrument
instrument = FinancialInstrument(
    isin="US46434G1031",
    lei="549300PZDW6EBUUJ8G35",
    name="ERC8040 Green Bond"
)

# Step 5: Generate ISO 20022 XML message (SETR.044)
xml_message = bridge.create_setr_message(instrument, classification)

# Step 6: Dispatch to SWIFT network
# bridge.dispatch(xml_message, bic="DEUTDEDB")
```

---

## ISO 20022 Message Structure

The bridge generates `SETR.044` (Order Instruction) and `SETR.045` (Order Instruction Confirmation) messages. The ESG classification is embedded in the `OthrBizPrcss` extension field:

```xml
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:setr.044.001.04">
  <OrdrInstr>
    <MsgId>
      <Id>ERC8040-2024-0042</Id>
      <CreDtTm>2024-03-15T14:22:00Z</CreDtTm>
    </MsgId>
    <FinInstrm>
      <ISIN>US46434G1031</ISIN>
    </FinInstrm>
    <OthrBizPrcss>
      <Tp>ESG-CLASSIFICATION</Tp>
      <Ref>ATF-AI-ATT-2024-0042</Ref>  <!-- ATF-AI Attestation ID -->
      <Desc>SFDR-Article-9;ESG-Rating:AA;Score:85.0</Desc>
    </OthrBizPrcss>
  </OrdrInstr>
</Document>
```

The `Ref` field carries the **ATF-AI Attestation ID**, ensuring traceability back to the original ATF-AI provenance chain.

---

## Provenance Traceability Through the Bridge

A key design requirement of the ISO 20022 bridge is that **ATF-AI provenance is not lost in translation**. Every ISO 20022 message generated by the bridge includes:

1. The ATF-AI Attestation ID in the `OthrBizPrcss.Ref` field
2. The ERC-8040 token ID as a supplementary reference
3. The on-chain provenance hash in the message remarks

This means a compliance auditor can trace an ISO 20022 message back to:
- The ERC-8040 token → the ESG score
- The ESGOracle attestation → the ATF-AI Compliance Attestation
- The ATF-AI Attestation → the Provenance Record chain

---

## Related Documentation

- [Overview](./overview.md) — Introduction to the blockchain adapter
- [Architecture](./architecture.md) — Smart contract mapping and data flow
- [Formal Adapter Spec](../../../specs/adapters/erc8040.md)
- [ERC-8040 Repository](https://github.com/agronetlabs/erc-8040-ecosystem)
