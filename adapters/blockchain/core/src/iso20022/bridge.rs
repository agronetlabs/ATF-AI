use crate::compliance::{ComplianceResult, ComplianceStatus};
use crate::esg::{ESGRating, ESGScore};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::{ESGClassification, FinancialInstrument, SetrMessage};

/// Bridge between ERC-8040 and ISO 20022
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISO20022Bridge;

impl ISO20022Bridge {
    /// Create a new ISO 20022 bridge
    pub fn new() -> Self {
        Self
    }

    /// Convert ESG Score to ISO 20022 ESG Classification
    pub fn esg_to_iso(&self, esg_score: &ESGScore) -> ESGClassification {
        let taxonomy_alignment = self.calculate_taxonomy_alignment(esg_score);
        let sfdr_article = self.determine_sfdr_article(esg_score);
        let carbon_intensity = self.estimate_carbon_intensity(esg_score);

        ESGClassification::new(
            taxonomy_alignment,
            sfdr_article,
            esg_score.rating.as_str().to_string(),
            carbon_intensity,
        )
    }

    /// Calculate EU Taxonomy alignment percentage
    fn calculate_taxonomy_alignment(&self, esg_score: &ESGScore) -> f64 {
        // Simplified calculation based on environmental score
        // Real implementation would use detailed taxonomy criteria
        if esg_score.environmental >= 80.0 {
            esg_score.environmental.min(100.0)
        } else if esg_score.environmental >= 60.0 {
            (esg_score.environmental - 60.0) * 2.0
        } else {
            0.0
        }
    }

    /// Determine SFDR Article classification (6, 8, or 9)
    fn determine_sfdr_article(&self, esg_score: &ESGScore) -> u8 {
        match esg_score.rating {
            ESGRating::AAA | ESGRating::AA | ESGRating::A => 9, // Article 9: Sustainable investment
            ESGRating::BBB | ESGRating::BB => 8, // Article 8: Promotes ESG
            _ => 6,                              // Article 6: No sustainability objective
        }
    }

    /// Estimate carbon intensity based on environmental score
    fn estimate_carbon_intensity(&self, esg_score: &ESGScore) -> f64 {
        // Inverse relationship: higher environmental score = lower carbon intensity
        // Returns tCO2e per $M revenue
        let max_intensity = 500.0;
        max_intensity * (1.0 - esg_score.environmental / 100.0)
    }

    /// Convert compliance results to ISO 20022 format
    pub fn compliance_to_iso(&self, results: &[ComplianceResult]) -> Vec<String> {
        results
            .iter()
            .filter(|r| r.status == ComplianceStatus::Compliant)
            .map(|r| r.rule_id.clone())
            .collect()
    }

    /// Create a SETR message with ESG classification
    pub fn create_setr_with_esg(
        &self,
        instrument: FinancialInstrument,
        esg_score: &ESGScore,
        quantity: f64,
        trade_date: String,
    ) -> SetrMessage {
        let esg_classification = self.esg_to_iso(esg_score);

        SetrMessage {
            message_id: Uuid::new_v4().to_string(),
            instrument,
            esg_classification: Some(esg_classification),
            quantity,
            trade_date,
        }
    }
}

impl Default for ISO20022Bridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esg_to_iso() {
        let bridge = ISO20022Bridge::new();
        let esg_score = ESGScore::new(90.0, 85.0, 80.0);

        let classification = bridge.esg_to_iso(&esg_score);

        assert_eq!(classification.erc8040_rating, "AA");
        assert_eq!(classification.sfdr_article, 9);
        assert!(classification.taxonomy_alignment > 0.0);
    }

    #[test]
    fn test_sfdr_article_mapping() {
        let bridge = ISO20022Bridge::new();

        let aaa_score = ESGScore::new(95.0, 95.0, 95.0);
        let classification = bridge.esg_to_iso(&aaa_score);
        assert_eq!(classification.sfdr_article, 9);

        let bbb_score = ESGScore::new(75.0, 75.0, 75.0);
        let classification = bridge.esg_to_iso(&bbb_score);
        assert_eq!(classification.sfdr_article, 8);

        let low_score = ESGScore::new(50.0, 50.0, 50.0);
        let classification = bridge.esg_to_iso(&low_score);
        assert_eq!(classification.sfdr_article, 6);
    }

    #[test]
    fn test_create_setr_with_esg() {
        let bridge = ISO20022Bridge::new();
        let instrument = FinancialInstrument::new("ERC8040 Token".to_string())
            .with_isin("US1234567890".to_string());
        let esg_score = ESGScore::new(85.0, 80.0, 75.0);

        let setr =
            bridge.create_setr_with_esg(instrument, &esg_score, 100.0, "2024-01-01".to_string());

        assert_eq!(setr.quantity, 100.0);
        assert!(setr.esg_classification.is_some());
        assert_eq!(setr.esg_classification.unwrap().erc8040_rating, "A");
    }
}
