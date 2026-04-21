use crate::esg::{ESGRating, ESGScore};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::rules::ComplianceRule;

/// Status of a compliance check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    Pending,
    NotApplicable,
}

impl ComplianceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "Compliant",
            ComplianceStatus::PartiallyCompliant => "Partially Compliant",
            ComplianceStatus::NonCompliant => "Non-Compliant",
            ComplianceStatus::Pending => "Pending",
            ComplianceStatus::NotApplicable => "Not Applicable",
        }
    }
}

/// Result of a compliance validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub rule_id: String,
    pub status: ComplianceStatus,
    pub message: String,
    pub checked_at: DateTime<Utc>,
}

impl ComplianceResult {
    pub fn new(rule_id: String, status: ComplianceStatus, message: String) -> Self {
        Self {
            rule_id,
            status,
            message,
            checked_at: Utc::now(),
        }
    }
}

/// Validator for compliance rules
#[derive(Debug, Default)]
pub struct ComplianceValidator {
    rules: Vec<ComplianceRule>,
}

impl ComplianceValidator {
    /// Create a new compliance validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a compliance rule
    pub fn add_rule(&mut self, rule: ComplianceRule) {
        self.rules.push(rule);
    }

    /// Add multiple compliance rules
    pub fn add_rules(&mut self, rules: Vec<ComplianceRule>) {
        self.rules.extend(rules);
    }

    /// Validate an ESG score against ESG-related rules
    pub fn validate_esg(
        &self,
        esg_score: &ESGScore,
        jurisdiction: crate::compliance::rules::Jurisdiction,
        framework: crate::compliance::rules::RegulatoryFramework,
        category: crate::compliance::rules::RuleCategory,
    ) -> Vec<ComplianceResult> {
        let now = Utc::now();
        let mut results = Vec::new();

        for rule in &self.rules {
            if !rule.is_effective(now) {
                results.push(ComplianceResult::new(
                    rule.id.clone(),
                    ComplianceStatus::NotApplicable,
                    "Rule not currently effective".to_string(),
                ));
                continue;
            }

            if !rule.applies_to(jurisdiction)
                || rule.framework != framework
                || rule.category != category
            {
                results.push(ComplianceResult::new(
                    rule.id.clone(),
                    ComplianceStatus::NotApplicable,
                    "Rule not applicable for given filters".to_string(),
                ));
                continue;
            }

            let status = if let Some(ref required_rating) = rule.required_esg_rating {
                let required = match Self::parse_esg_rating(required_rating) {
                    Some(rating) => rating,
                    None => {
                        results.push(ComplianceResult::new(
                            rule.id.clone(),
                            ComplianceStatus::NonCompliant,
                            "Invalid required ESG rating".to_string(),
                        ));
                        continue;
                    }
                };

                if esg_score.rating >= required {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::NonCompliant
                }
            } else {
                ComplianceStatus::NotApplicable
            };

            let message = match status {
                ComplianceStatus::Compliant => {
                    format!("ESG rating {} meets requirement", esg_score.rating.as_str())
                }
                ComplianceStatus::NonCompliant => format!(
                    "ESG rating {} does not meet requirement of {}",
                    esg_score.rating.as_str(),
                    rule.required_esg_rating.as_ref().unwrap()
                ),
                _ => "No ESG rating requirement".to_string(),
            };

            results.push(ComplianceResult::new(rule.id.clone(), status, message));
        }

        results
    }

    /// Validate all rules
    pub fn validate_all(
        &self,
        esg_score: &ESGScore,
        jurisdiction: crate::compliance::rules::Jurisdiction,
        framework: crate::compliance::rules::RegulatoryFramework,
        category: crate::compliance::rules::RuleCategory,
    ) -> Vec<ComplianceResult> {
        self.validate_esg(esg_score, jurisdiction, framework, category)
    }

    /// Get overall compliance status
    pub fn overall_status(&self, results: &[ComplianceResult]) -> ComplianceStatus {
        let mut has_non_compliant = false;
        let mut has_partially_compliant = false;
        let mut has_compliant = false;
        let mut has_pending = false;

        for result in results {
            match result.status {
                ComplianceStatus::NonCompliant => has_non_compliant = true,
                ComplianceStatus::PartiallyCompliant => has_partially_compliant = true,
                ComplianceStatus::Compliant => has_compliant = true,
                ComplianceStatus::Pending => has_pending = true,
                ComplianceStatus::NotApplicable => {}
            }
        }

        if has_non_compliant {
            ComplianceStatus::NonCompliant
        } else if has_pending {
            ComplianceStatus::Pending
        } else if has_partially_compliant {
            ComplianceStatus::PartiallyCompliant
        } else if has_compliant {
            ComplianceStatus::Compliant
        } else {
            ComplianceStatus::NotApplicable
        }
    }

    fn parse_esg_rating(value: &str) -> Option<ESGRating> {
        match value {
            "AAA" => Some(ESGRating::AAA),
            "AA" => Some(ESGRating::AA),
            "A" => Some(ESGRating::A),
            "BBB" => Some(ESGRating::BBB),
            "BB" => Some(ESGRating::BB),
            "B" => Some(ESGRating::B),
            "CCC" => Some(ESGRating::CCC),
            "CC" => Some(ESGRating::CC),
            "C" => Some(ESGRating::C),
            "D" => Some(ESGRating::D),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compliance::rules::{Jurisdiction, RegulatoryFramework, RuleCategory, Severity};

    #[test]
    fn test_compliance_status_as_str() {
        assert_eq!(ComplianceStatus::Compliant.as_str(), "Compliant");
        assert_eq!(ComplianceStatus::NonCompliant.as_str(), "Non-Compliant");
    }

    #[test]
    fn test_compliance_validator() {
        let mut validator = ComplianceValidator::new();

        let rule = ComplianceRule {
            id: "SFDR-001".to_string(),
            framework: RegulatoryFramework::EuSfdr,
            jurisdiction: Jurisdiction::EU,
            category: RuleCategory::EsgDisclosure,
            severity: Severity::High,
            description: "Minimum ESG rating required".to_string(),
            effective_from: Utc::now(),
            effective_until: None,
            required_esg_rating: Some("BBB".to_string()),
        };

        validator.add_rule(rule);

        let esg_score = ESGScore::new(85.0, 80.0, 75.0);
        let results = validator.validate_esg(
            &esg_score,
            Jurisdiction::EU,
            RegulatoryFramework::EuSfdr,
            RuleCategory::EsgDisclosure,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_overall_status() {
        let validator = ComplianceValidator::new();

        let results = vec![
            ComplianceResult::new(
                "R1".to_string(),
                ComplianceStatus::Compliant,
                "OK".to_string(),
            ),
            ComplianceResult::new(
                "R2".to_string(),
                ComplianceStatus::Compliant,
                "OK".to_string(),
            ),
        ];

        assert_eq!(
            validator.overall_status(&results),
            ComplianceStatus::Compliant
        );

        let results_non_compliant = vec![
            ComplianceResult::new(
                "R1".to_string(),
                ComplianceStatus::Compliant,
                "OK".to_string(),
            ),
            ComplianceResult::new(
                "R2".to_string(),
                ComplianceStatus::NonCompliant,
                "Failed".to_string(),
            ),
        ];

        assert_eq!(
            validator.overall_status(&results_non_compliant),
            ComplianceStatus::NonCompliant
        );
    }

    #[test]
    fn test_invalid_required_esg_rating() {
        let mut validator = ComplianceValidator::new();

        let rule = ComplianceRule {
            id: "SFDR-INVALID".to_string(),
            framework: RegulatoryFramework::EuSfdr,
            jurisdiction: Jurisdiction::EU,
            category: RuleCategory::EsgDisclosure,
            severity: Severity::High,
            description: "Invalid ESG rating".to_string(),
            effective_from: Utc::now(),
            effective_until: None,
            required_esg_rating: Some("INVALID".to_string()),
        };

        validator.add_rule(rule);

        let esg_score = ESGScore::new(85.0, 80.0, 75.0);
        let results = validator.validate_esg(
            &esg_score,
            Jurisdiction::EU,
            RegulatoryFramework::EuSfdr,
            RuleCategory::EsgDisclosure,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ComplianceStatus::NonCompliant);
    }
}
