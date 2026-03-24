use serde::{Deserialize, Serialize};

/// ESG Category types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ESGCategory {
    Environmental,
    Social,
    Governance,
}

impl ESGCategory {
    pub fn code(&self) -> &'static str {
        match self {
            ESGCategory::Environmental => "E",
            ESGCategory::Social => "S",
            ESGCategory::Governance => "G",
        }
    }
}

/// Environmental metrics for ESG scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalMetrics {
    /// Carbon footprint in metric tons CO2e
    pub carbon_footprint: f64,
    /// Percentage of renewable energy used (0-100)
    pub renewable_energy_pct: f64,
    /// Water usage in cubic meters
    pub water_usage: f64,
    /// Waste reduction percentage compared to baseline (0-100)
    pub waste_reduction_pct: f64,
}

impl Default for EnvironmentalMetrics {
    fn default() -> Self {
        Self {
            carbon_footprint: 0.0,
            renewable_energy_pct: 0.0,
            water_usage: 0.0,
            waste_reduction_pct: 0.0,
        }
    }
}

/// Social metrics for ESG scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMetrics {
    /// Labor standards score (0-100)
    pub labor_standards_score: f64,
    /// Community investment amount or score
    pub community_investment: f64,
    /// Diversity index (0-100)
    pub diversity_index: f64,
}

impl Default for SocialMetrics {
    fn default() -> Self {
        Self {
            labor_standards_score: 0.0,
            community_investment: 0.0,
            diversity_index: 0.0,
        }
    }
}

/// Governance metrics for ESG scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceMetrics {
    /// Board independence percentage (0-100)
    pub board_independence_pct: f64,
    /// Transparency score (0-100)
    pub transparency_score: f64,
    /// Anti-corruption score (0-100)
    pub anti_corruption_score: f64,
}

impl Default for GovernanceMetrics {
    fn default() -> Self {
        Self {
            board_independence_pct: 0.0,
            transparency_score: 0.0,
            anti_corruption_score: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esg_category_code() {
        assert_eq!(ESGCategory::Environmental.code(), "E");
        assert_eq!(ESGCategory::Social.code(), "S");
        assert_eq!(ESGCategory::Governance.code(), "G");
    }

    #[test]
    fn test_environmental_metrics_default() {
        let metrics = EnvironmentalMetrics::default();
        assert_eq!(metrics.carbon_footprint, 0.0);
        assert_eq!(metrics.renewable_energy_pct, 0.0);
    }
}
