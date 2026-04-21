use serde::{Deserialize, Serialize};

/// ESG Rating levels from D (lowest) to AAA (highest)
/// Note: Order matters for Ord trait - D is lowest, AAA is highest
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ESGRating {
    D,
    C,
    CC,
    CCC,
    B,
    BB,
    BBB,
    A,
    AA,
    AAA,
}

impl ESGRating {
    /// Convert a total ESG score (0-100) to a rating
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 90.0 => ESGRating::AAA,
            s if s >= 85.0 => ESGRating::AA,
            s if s >= 80.0 => ESGRating::A,
            s if s >= 70.0 => ESGRating::BBB,
            s if s >= 60.0 => ESGRating::BB,
            s if s >= 50.0 => ESGRating::B,
            s if s >= 40.0 => ESGRating::CCC,
            s if s >= 30.0 => ESGRating::CC,
            s if s >= 20.0 => ESGRating::C,
            _ => ESGRating::D,
        }
    }

    /// Check if this rating is investment grade (BBB or higher)
    pub fn is_investment_grade(&self) -> bool {
        matches!(
            self,
            ESGRating::AAA | ESGRating::AA | ESGRating::A | ESGRating::BBB
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ESGRating::AAA => "AAA",
            ESGRating::AA => "AA",
            ESGRating::A => "A",
            ESGRating::BBB => "BBB",
            ESGRating::BB => "BB",
            ESGRating::B => "B",
            ESGRating::CCC => "CCC",
            ESGRating::CC => "CC",
            ESGRating::C => "C",
            ESGRating::D => "D",
        }
    }
}

/// ESG Score breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGScore {
    pub environmental: f64,
    pub social: f64,
    pub governance: f64,
    pub total: f64,
    pub rating: ESGRating,
}

impl ESGScore {
    /// Create a new ESG score
    pub fn new(environmental: f64, social: f64, governance: f64) -> Self {
        let total = (environmental + social + governance) / 3.0;
        let rating = ESGRating::from_score(total);
        Self {
            environmental,
            social,
            governance,
            total,
            rating,
        }
    }

    /// Check if this score is investment grade
    pub fn is_investment_grade(&self) -> bool {
        self.rating.is_investment_grade()
    }
}

/// ESG Scoring calculator with configurable weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGScoring {
    pub environmental_weight: f64,
    pub social_weight: f64,
    pub governance_weight: f64,
}

impl Default for ESGScoring {
    fn default() -> Self {
        Self {
            environmental_weight: 1.0 / 3.0,
            social_weight: 1.0 / 3.0,
            governance_weight: 1.0 / 3.0,
        }
    }
}

impl ESGScoring {
    /// Create a new ESG scoring calculator with equal weights
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a custom ESG scoring calculator with specific weights
    pub fn with_weights(
        environmental_weight: f64,
        social_weight: f64,
        governance_weight: f64,
    ) -> Self {
        Self::try_with_weights(environmental_weight, social_weight, governance_weight)
            .expect("ESG weights must be non-negative and sum to > 0")
    }

    /// Create a custom ESG scoring calculator with validation
    pub fn try_with_weights(
        environmental_weight: f64,
        social_weight: f64,
        governance_weight: f64,
    ) -> Result<Self, String> {
        if environmental_weight < 0.0 || social_weight < 0.0 || governance_weight < 0.0 {
            return Err("ESG weights must be non-negative".to_string());
        }

        let total = environmental_weight + social_weight + governance_weight;
        if total <= 0.0 {
            return Err("ESG weights must sum to > 0".to_string());
        }

        Ok(Self {
            environmental_weight: environmental_weight / total,
            social_weight: social_weight / total,
            governance_weight: governance_weight / total,
        })
    }

    /// Calculate ESG score from individual component scores
    pub fn calculate(&self, environmental: f64, social: f64, governance: f64) -> ESGScore {
        let weighted_total = (environmental * self.environmental_weight)
            + (social * self.social_weight)
            + (governance * self.governance_weight);

        ESGScore {
            environmental,
            social,
            governance,
            total: weighted_total,
            rating: ESGRating::from_score(weighted_total),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esg_rating_from_score() {
        assert_eq!(ESGRating::from_score(95.0), ESGRating::AAA);
        assert_eq!(ESGRating::from_score(87.0), ESGRating::AA);
        assert_eq!(ESGRating::from_score(75.0), ESGRating::BBB);
        assert_eq!(ESGRating::from_score(65.0), ESGRating::BB);
        assert_eq!(ESGRating::from_score(55.0), ESGRating::B);
        assert_eq!(ESGRating::from_score(10.0), ESGRating::D);
    }

    #[test]
    fn test_esg_rating_investment_grade() {
        assert!(ESGRating::AAA.is_investment_grade());
        assert!(ESGRating::BBB.is_investment_grade());
        assert!(!ESGRating::BB.is_investment_grade());
        assert!(!ESGRating::D.is_investment_grade());
    }

    #[test]
    fn test_esg_score_new() {
        let score = ESGScore::new(90.0, 85.0, 80.0);
        assert_eq!(score.environmental, 90.0);
        assert_eq!(score.social, 85.0);
        assert_eq!(score.governance, 80.0);
        assert!((score.total - 85.0).abs() < 0.01);
        assert_eq!(score.rating, ESGRating::AA);
    }

    #[test]
    fn test_esg_scoring_default() {
        let scoring = ESGScoring::default();
        let score = scoring.calculate(90.0, 80.0, 70.0);
        assert!((score.total - 80.0).abs() < 0.01);
        assert_eq!(score.rating, ESGRating::A);
    }

    #[test]
    fn test_esg_scoring_custom_weights() {
        let scoring = ESGScoring::with_weights(2.0, 1.0, 1.0);
        let score = scoring.calculate(80.0, 60.0, 60.0);
        // (80*0.5 + 60*0.25 + 60*0.25) = 70
        assert!((score.total - 70.0).abs() < 0.01);
    }

    #[test]
    fn test_esg_scoring_try_with_weights_invalid() {
        assert!(ESGScoring::try_with_weights(-1.0, 1.0, 1.0).is_err());
        assert!(ESGScoring::try_with_weights(0.0, 0.0, 0.0).is_err());
    }
}
