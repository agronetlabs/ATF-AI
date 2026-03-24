"""Tests for compliance validation functionality."""
from datetime import datetime, timedelta

from erc8040_sdk import (
    ComplianceRule,
    ComplianceStatus,
    ComplianceValidator,
    ESGScore,
    Jurisdiction,
    RegulatoryFramework,
    RuleCategory,
    Severity,
)


def test_compliance_validator():
    """Test compliance validator creation."""
    validator = ComplianceValidator()
    assert validator is not None
    assert isinstance(validator.rules, list)
    assert len(validator.rules) == 0


def test_compliance_status():
    """Test compliance status enum values."""
    assert ComplianceStatus.COMPLIANT.value == "compliant"
    assert ComplianceStatus.NON_COMPLIANT.value == "non_compliant"
    assert ComplianceStatus.PENDING.value == "pending"


def test_compliance_rule_is_effective():
    """Test compliance rule effectiveness check."""
    now = datetime.now()
    rule = ComplianceRule(
        id="TEST-001",
        framework=RegulatoryFramework.EU_SFDR,
        jurisdiction=Jurisdiction.EU,
        category=RuleCategory.ESG_DISCLOSURE,
        severity=Severity.HIGH,
        description="Test rule",
        effective_from=now - timedelta(days=10),
        effective_until=now + timedelta(days=10),
    )

    assert rule.is_effective(now)
    assert not rule.is_effective(now - timedelta(days=20))
    assert not rule.is_effective(now + timedelta(days=20))


def test_compliance_rule_applies_to():
    """Test jurisdiction applicability."""
    rule = ComplianceRule(
        id="TEST-001",
        framework=RegulatoryFramework.EU_SFDR,
        jurisdiction=Jurisdiction.EU,
        category=RuleCategory.ESG_DISCLOSURE,
        severity=Severity.HIGH,
        description="Test rule",
        effective_from=datetime.now(),
    )

    assert rule.applies_to(Jurisdiction.EU)
    assert not rule.applies_to(Jurisdiction.US)

    global_rule = ComplianceRule(
        id="TEST-002",
        framework=RegulatoryFramework.EU_SFDR,
        jurisdiction=Jurisdiction.GLOBAL,
        category=RuleCategory.ESG_DISCLOSURE,
        severity=Severity.HIGH,
        description="Global rule",
        effective_from=datetime.now(),
    )

    assert global_rule.applies_to(Jurisdiction.EU)
    assert global_rule.applies_to(Jurisdiction.US)


def test_compliance_validator_add_rule():
    """Test adding rules to validator."""
    validator = ComplianceValidator()
    rule = ComplianceRule(
        id="SFDR-001",
        framework=RegulatoryFramework.EU_SFDR,
        jurisdiction=Jurisdiction.EU,
        category=RuleCategory.ESG_DISCLOSURE,
        severity=Severity.HIGH,
        description="Minimum ESG rating required",
        effective_from=datetime.now(),
        required_esg_rating="BBB",
    )

    validator.add_rule(rule)
    assert len(validator.rules) == 1


def test_compliance_validator_validate_esg():
    """Test ESG validation against compliance rules."""
    validator = ComplianceValidator()
    rule = ComplianceRule(
        id="SFDR-001",
        framework=RegulatoryFramework.EU_SFDR,
        jurisdiction=Jurisdiction.EU,
        category=RuleCategory.ESG_DISCLOSURE,
        severity=Severity.HIGH,
        description="Minimum ESG rating required",
        effective_from=datetime.now(),
        required_esg_rating="BBB",
    )

    validator.add_rule(rule)

    esg_score = ESGScore.create(85.0, 80.0, 75.0)
    results = validator.validate_esg(
        esg_score,
        Jurisdiction.EU,
        RegulatoryFramework.EU_SFDR,
        RuleCategory.ESG_DISCLOSURE,
    )

    assert len(results) == 1
    assert results[0].status == ComplianceStatus.COMPLIANT


def test_compliance_validator_overall_status():
    """Test overall status calculation."""
    validator = ComplianceValidator()

    from erc8040_sdk.compliance import ComplianceResult

    results = [
        ComplianceResult(
            rule_id="R1",
            status=ComplianceStatus.COMPLIANT,
            message="OK",
        ),
        ComplianceResult(
            rule_id="R2",
            status=ComplianceStatus.COMPLIANT,
            message="OK",
        ),
    ]

    assert validator.overall_status(results) == ComplianceStatus.COMPLIANT

    results_non_compliant = [
        ComplianceResult(
            rule_id="R1",
            status=ComplianceStatus.COMPLIANT,
            message="OK",
        ),
        ComplianceResult(
            rule_id="R2",
            status=ComplianceStatus.NON_COMPLIANT,
            message="Failed",
        ),
    ]

    assert validator.overall_status(results_non_compliant) == ComplianceStatus.NON_COMPLIANT


def test_compliance_validator_invalid_required_rating():
    """Test invalid required ESG rating handling."""
    validator = ComplianceValidator()
    rule = ComplianceRule(
        id="SFDR-INVALID",
        framework=RegulatoryFramework.EU_SFDR,
        jurisdiction=Jurisdiction.EU,
        category=RuleCategory.ESG_DISCLOSURE,
        severity=Severity.HIGH,
        description="Invalid ESG rating",
        effective_from=datetime.now(),
        required_esg_rating="INVALID",
    )

    validator.add_rule(rule)

    esg_score = ESGScore.create(85.0, 80.0, 75.0)
    results = validator.validate_esg(
        esg_score,
        Jurisdiction.EU,
        RegulatoryFramework.EU_SFDR,
        RuleCategory.ESG_DISCLOSURE,
    )

    assert len(results) == 1
    assert results[0].status == ComplianceStatus.NON_COMPLIANT
