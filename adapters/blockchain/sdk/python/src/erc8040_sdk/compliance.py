"""Compliance validation and regulatory framework module for ERC-8040."""
from datetime import datetime
from enum import Enum

from pydantic import BaseModel, Field


class RegulatoryFramework(str, Enum):
    """Regulatory frameworks supported by ERC-8040."""

    EU_SFDR = "EU_SFDR"
    EU_TAXONOMY = "EU_Taxonomy"
    SEC_CLIMATE = "SEC_Climate"
    MIFID_II = "MiFID_II"
    BASEL = "Basel"

    def display_name(self) -> str:
        """Get the display name for the framework."""
        mapping = {
            "EU_SFDR": "EU SFDR",
            "EU_Taxonomy": "EU Taxonomy",
            "SEC_Climate": "SEC Climate",
            "MiFID_II": "MiFID II",
            "Basel": "Basel",
        }
        return mapping.get(self.value, self.value)


class Jurisdiction(str, Enum):
    """Jurisdiction where compliance applies."""

    EU = "EU"
    US = "US"
    UK = "UK"
    BRAZIL = "BRAZIL"
    GLOBAL = "GLOBAL"

    def code(self) -> str:
        """Get the jurisdiction code."""
        mapping = {
            "EU": "EU",
            "US": "US",
            "UK": "UK",
            "BRAZIL": "BR",
            "GLOBAL": "GLOBAL",
        }
        return mapping.get(self.value, self.value)


class Severity(str, Enum):
    """Severity level of a compliance rule."""

    LOW = "LOW"
    MEDIUM = "MEDIUM"
    HIGH = "HIGH"
    CRITICAL = "CRITICAL"


class ComplianceStatus(str, Enum):
    """Status of a compliance check."""

    COMPLIANT = "compliant"
    PARTIALLY_COMPLIANT = "partially_compliant"
    NON_COMPLIANT = "non_compliant"
    PENDING = "pending"
    NOT_APPLICABLE = "not_applicable"


class RuleCategory(str, Enum):
    """Category of compliance rule."""

    KYC_AML = "KYC_AML"
    ESG_DISCLOSURE = "ESG_Disclosure"
    INVESTMENT_RESTRICTION = "InvestmentRestriction"
    REPORTING = "Reporting"
    RISK_MANAGEMENT = "RiskManagement"


class ComplianceRule(BaseModel):
    """A compliance rule that must be satisfied."""

    id: str
    framework: RegulatoryFramework
    jurisdiction: Jurisdiction
    category: RuleCategory
    severity: Severity
    description: str
    effective_from: datetime
    effective_until: datetime | None = None
    required_esg_rating: str | None = None

    def is_effective(self, at_time: datetime | None = None) -> bool:
        """Check if the rule is currently effective.

        Args:
            at_time: Time to check effectiveness (defaults to now)

        Returns:
            True if the rule is effective at the given time
        """
        check_time = at_time or datetime.now()
        if check_time < self.effective_from:
            return False
        if self.effective_until and check_time > self.effective_until:
            return False
        return True

    def applies_to(self, jurisdiction: Jurisdiction) -> bool:
        """Check if the rule applies to a specific jurisdiction.

        Args:
            jurisdiction: Jurisdiction to check

        Returns:
            True if the rule applies to the given jurisdiction
        """
        return self.jurisdiction == jurisdiction or self.jurisdiction == Jurisdiction.GLOBAL


class ComplianceResult(BaseModel):
    """Result of a compliance validation."""

    rule_id: str
    status: ComplianceStatus
    message: str
    checked_at: datetime = Field(default_factory=datetime.now)


class ComplianceValidator:
    """Validator for compliance rules."""

    def __init__(self):
        """Initialize a new compliance validator."""
        self.rules: list[ComplianceRule] = []

    def add_rule(self, rule: ComplianceRule) -> None:
        """Add a compliance rule.

        Args:
            rule: Compliance rule to add
        """
        self.rules.append(rule)

    def add_rules(self, rules: list[ComplianceRule]) -> None:
        """Add multiple compliance rules.

        Args:
            rules: List of compliance rules to add
        """
        self.rules.extend(rules)

    def validate_esg(
        self,
        esg_score,
        jurisdiction: Jurisdiction,
        framework: RegulatoryFramework,
        category: RuleCategory,
        at_time: datetime | None = None,
    ) -> list[ComplianceResult]:
        """Validate an ESG score against ESG-related rules.

        Args:
            esg_score: ESG score to validate (ESGScore object)
            jurisdiction: Jurisdiction to validate against
            framework: Regulatory framework to validate against
            category: Rule category to validate against
            at_time: Time to check rules (defaults to now)

        Returns:
            List of compliance results
        """
        check_time = at_time or datetime.now()
        results = []

        for rule in self.rules:
            if not rule.is_effective(check_time):
                results.append(
                    ComplianceResult(
                        rule_id=rule.id,
                        status=ComplianceStatus.NOT_APPLICABLE,
                        message="Rule not currently effective",
                    )
                )
                continue

            if (
                not rule.applies_to(jurisdiction)
                or rule.framework != framework
                or rule.category != category
            ):
                results.append(
                    ComplianceResult(
                        rule_id=rule.id,
                        status=ComplianceStatus.NOT_APPLICABLE,
                        message="Rule not applicable for given filters",
                    )
                )
                continue

            if rule.required_esg_rating is None:
                results.append(
                    ComplianceResult(
                        rule_id=rule.id,
                        status=ComplianceStatus.NOT_APPLICABLE,
                        message="No ESG rating requirement",
                    )
                )
                continue

            # Import ESGRating here to avoid circular import
            from erc8040_sdk.esg import ESGRating

            try:
                required_rating = ESGRating(rule.required_esg_rating)
            except ValueError:
                results.append(
                    ComplianceResult(
                        rule_id=rule.id,
                        status=ComplianceStatus.NON_COMPLIANT,
                        message="Invalid required ESG rating",
                    )
                )
                continue

            rating_order = list(ESGRating)

            if rating_order.index(esg_score.rating) >= rating_order.index(required_rating):
                results.append(
                    ComplianceResult(
                        rule_id=rule.id,
                        status=ComplianceStatus.COMPLIANT,
                        message=f"ESG rating {esg_score.rating.value} meets requirement",
                    )
                )
            else:
                message = (
                    f"ESG rating {esg_score.rating.value} does not meet "
                    f"requirement of {rule.required_esg_rating}"
                )
                results.append(
                    ComplianceResult(
                        rule_id=rule.id,
                        status=ComplianceStatus.NON_COMPLIANT,
                        message=message,
                    )
                )

        return results

    def overall_status(self, results: list[ComplianceResult]) -> ComplianceStatus:
        """Get overall compliance status from a list of results.

        Args:
            results: List of compliance results

        Returns:
            Overall compliance status
        """
        has_non_compliant = False
        has_partially_compliant = False
        has_compliant = False
        has_pending = False

        for result in results:
            if result.status == ComplianceStatus.NON_COMPLIANT:
                has_non_compliant = True
            elif result.status == ComplianceStatus.PARTIALLY_COMPLIANT:
                has_partially_compliant = True
            elif result.status == ComplianceStatus.COMPLIANT:
                has_compliant = True
            elif result.status == ComplianceStatus.PENDING:
                has_pending = True

        if has_non_compliant:
            return ComplianceStatus.NON_COMPLIANT
        elif has_pending:
            return ComplianceStatus.PENDING
        elif has_partially_compliant:
            return ComplianceStatus.PARTIALLY_COMPLIANT
        elif has_compliant:
            return ComplianceStatus.COMPLIANT
        else:
            return ComplianceStatus.NOT_APPLICABLE
