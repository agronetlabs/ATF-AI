# ERC-8040 Python SDK

Python SDK for the ERC-8040 Compliance Token Standard, providing ESG scoring and regulatory compliance validation.

## Features

- **ESG Scoring**: Calculate and validate ESG (Environmental, Social, Governance) scores
- **Rating System**: Convert scores to standard ratings (AAA to D)
- **Compliance Validation**: Validate against regulatory frameworks (EU SFDR, SEC Climate, etc.)
- **Type Safety**: Built with Pydantic v2 for robust data validation

## Installation

```bash
pip install erc8040-sdk
```

Or with development dependencies:

```bash
pip install erc8040-sdk[dev]
```

## Quick Start

### ESG Scoring

```python
from erc8040_sdk import ESGScore, ESGScoring, ESGRating

# Create an ESG score
score = ESGScore.create(
    environmental=85.0,
    social=80.0,
    governance=75.0
)

print(f"Total Score: {score.total}")  # 80.0
print(f"Rating: {score.rating}")      # ESGRating.A
print(f"Investment Grade: {score.is_investment_grade()}")  # True

# Custom weighted scoring
scoring = ESGScoring(
    environmental_weight=2.0,
    social_weight=1.0,
    governance_weight=1.0
)

weighted_score = scoring.calculate(80.0, 70.0, 90.0)
print(f"Weighted Total: {weighted_score.total}")
```

Note: ESG weights must be non-negative and sum to > 0. Invalid weights raise `ValueError`.

### Compliance Validation

```python
from datetime import datetime
from erc8040_sdk import (
    ComplianceValidator,
    ComplianceRule,
    RegulatoryFramework,
    Jurisdiction,
    RuleCategory,
    Severity,
    ESGScore
)

# Create a compliance validator
validator = ComplianceValidator()

# Add a compliance rule
rule = ComplianceRule(
    id="SFDR-001",
    framework=RegulatoryFramework.EU_SFDR,
    jurisdiction=Jurisdiction.EU,
    category=RuleCategory.ESG_DISCLOSURE,
    severity=Severity.HIGH,
    description="Minimum ESG rating required for EU SFDR compliance",
    effective_from=datetime(2023, 1, 1),
    required_esg_rating="BBB"
)

validator.add_rule(rule)

# Validate an ESG score
esg_score = ESGScore.create(85.0, 80.0, 75.0)
results = validator.validate_esg(
    esg_score,
    Jurisdiction.EU,
    RegulatoryFramework.EU_SFDR,
    RuleCategory.ESG_DISCLOSURE,
)

for result in results:
    print(f"Rule: {result.rule_id}")
    print(f"Status: {result.status}")
    print(f"Message: {result.message}")

# Get overall status
overall = validator.overall_status(results)
print(f"Overall Status: {overall}")
```

## Development

### Setup

```bash
# Install uv (recommended)
pip install uv

# Install dependencies
uv sync --all-extras

# Run tests
uv run pytest tests/ -v

# Run linter
uv run ruff check .
```

### Running Tests

```bash
pytest tests/ -v --cov=erc8040_sdk
```

## API Reference

### ESG Module

- `ESGCategory`: Enum for ESG categories (ENVIRONMENTAL, SOCIAL, GOVERNANCE)
- `ESGRating`: Enum for ratings (AAA, AA, A, BBB, BB, B, CCC, CC, C, D)
- `ESGScore`: Pydantic model for ESG scores with validation
- `ESGScoring`: Calculator for weighted ESG scores

### Compliance Module

- `RegulatoryFramework`: Enum for supported frameworks (EU_SFDR, SEC_CLIMATE, etc.)
- `Jurisdiction`: Enum for jurisdictions (EU, US, UK, BRAZIL, GLOBAL)
- `Severity`: Enum for rule severity levels
- `ComplianceStatus`: Enum for compliance statuses
- `ComplianceRule`: Pydantic model for compliance rules
- `ComplianceValidator`: Validator for compliance checks

## License

MIT - See LICENSE file for details

## Links

- [GitHub Repository](https://github.com/agronetlabs/erc-8040-ecosystem)
- [ERC-8040 Specification](../../docs/whitepaper.md)
- [Core Rust Library](../../core/)
