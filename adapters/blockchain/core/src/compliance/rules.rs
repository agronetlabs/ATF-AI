use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Regulatory frameworks supported by ERC-8040
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatoryFramework {
    #[serde(rename = "EU_SFDR")]
    EuSfdr,
    #[serde(rename = "EU_Taxonomy")]
    EuTaxonomy,
    #[serde(rename = "SEC_Climate")]
    SecClimate,
    #[serde(rename = "MiFID_II")]
    MifidIi,
    Basel,
    Custom(u32),
}

impl RegulatoryFramework {
    pub fn name(&self) -> String {
        match self {
            RegulatoryFramework::EuSfdr => "EU SFDR".to_string(),
            RegulatoryFramework::EuTaxonomy => "EU Taxonomy".to_string(),
            RegulatoryFramework::SecClimate => "SEC Climate".to_string(),
            RegulatoryFramework::MifidIi => "MiFID II".to_string(),
            RegulatoryFramework::Basel => "Basel".to_string(),
            RegulatoryFramework::Custom(id) => format!("Custom-{}", id),
        }
    }
}

/// Jurisdiction where compliance applies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Jurisdiction {
    EU,
    US,
    UK,
    Brazil,
    Global,
    Custom(u32),
}

impl Jurisdiction {
    pub fn code(&self) -> String {
        match self {
            Jurisdiction::EU => "EU".to_string(),
            Jurisdiction::US => "US".to_string(),
            Jurisdiction::UK => "UK".to_string(),
            Jurisdiction::Brazil => "BR".to_string(),
            Jurisdiction::Global => "GLOBAL".to_string(),
            Jurisdiction::Custom(id) => format!("CUSTOM-{}", id),
        }
    }
}

/// Category of compliance rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleCategory {
    #[serde(rename = "KYC_AML")]
    KycAml,
    #[serde(rename = "ESG_Disclosure")]
    EsgDisclosure,
    InvestmentRestriction,
    Reporting,
    RiskManagement,
}

/// Severity level of a compliance rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// A compliance rule that must be satisfied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub id: String,
    pub framework: RegulatoryFramework,
    pub jurisdiction: Jurisdiction,
    pub category: RuleCategory,
    pub severity: Severity,
    pub description: String,
    pub effective_from: DateTime<Utc>,
    pub effective_until: Option<DateTime<Utc>>,
    pub required_esg_rating: Option<String>,
}

impl ComplianceRule {
    /// Check if the rule is currently effective
    pub fn is_effective(&self, at_time: DateTime<Utc>) -> bool {
        at_time >= self.effective_from
            && self
                .effective_until
                .map(|until| at_time <= until)
                .unwrap_or(true)
    }

    /// Check if the rule applies to a specific jurisdiction
    pub fn applies_to(&self, jurisdiction: Jurisdiction) -> bool {
        self.jurisdiction == jurisdiction || self.jurisdiction == Jurisdiction::Global
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_regulatory_framework_name() {
        assert_eq!(RegulatoryFramework::EuSfdr.name(), "EU SFDR");
        assert_eq!(RegulatoryFramework::Basel.name(), "Basel");
    }

    #[test]
    fn test_jurisdiction_code() {
        assert_eq!(Jurisdiction::EU.code(), "EU");
        assert_eq!(Jurisdiction::US.code(), "US");
        assert_eq!(Jurisdiction::Global.code(), "GLOBAL");
    }

    #[test]
    fn test_compliance_rule_is_effective() {
        let now = Utc::now();
        let rule = ComplianceRule {
            id: "TEST-001".to_string(),
            framework: RegulatoryFramework::EuSfdr,
            jurisdiction: Jurisdiction::EU,
            category: RuleCategory::EsgDisclosure,
            severity: Severity::High,
            description: "Test rule".to_string(),
            effective_from: now - Duration::days(10),
            effective_until: Some(now + Duration::days(10)),
            required_esg_rating: None,
        };

        assert!(rule.is_effective(now));
        assert!(!rule.is_effective(now - Duration::days(20)));
        assert!(!rule.is_effective(now + Duration::days(20)));
    }

    #[test]
    fn test_compliance_rule_applies_to() {
        let rule = ComplianceRule {
            id: "TEST-001".to_string(),
            framework: RegulatoryFramework::EuSfdr,
            jurisdiction: Jurisdiction::EU,
            category: RuleCategory::EsgDisclosure,
            severity: Severity::High,
            description: "Test rule".to_string(),
            effective_from: Utc::now(),
            effective_until: None,
            required_esg_rating: None,
        };

        assert!(rule.applies_to(Jurisdiction::EU));
        assert!(!rule.applies_to(Jurisdiction::US));

        let global_rule = ComplianceRule {
            jurisdiction: Jurisdiction::Global,
            ..rule
        };
        assert!(global_rule.applies_to(Jurisdiction::EU));
        assert!(global_rule.applies_to(Jurisdiction::US));
    }
}
