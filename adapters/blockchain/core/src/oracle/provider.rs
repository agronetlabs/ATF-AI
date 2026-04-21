use crate::esg::ESGScore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of data that can be requested from oracles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleDataType {
    ESGScore,
    CarbonEmissions,
    RegulatoryStatus,
    SanctionsCheck,
    CreditRating,
}

/// Request to an oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleRequest {
    pub id: String,
    pub data_type: OracleDataType,
    pub entity_id: String,
    pub requested_at: DateTime<Utc>,
}

impl OracleRequest {
    pub fn new(data_type: OracleDataType, entity_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            data_type,
            entity_id,
            requested_at: Utc::now(),
        }
    }
}

/// Response from an oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResponse {
    pub request_id: String,
    pub data: OracleData,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>,
}

impl OracleResponse {
    pub fn new(request_id: String, data: OracleData) -> Self {
        Self {
            request_id,
            data,
            timestamp: Utc::now(),
            signature: None,
        }
    }
}

/// Data returned by an oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OracleData {
    ESGScore(ESGScore),
    CarbonEmissions(f64),
    RegulatoryStatus(bool),
    SanctionsCheck(bool),
    CreditRating(String),
}

/// Oracle provider trait
pub trait OracleProvider {
    /// Request data from the oracle
    fn request(&self, request: OracleRequest) -> Result<OracleResponse, String>;

    /// Check if the oracle supports a specific data type
    fn supports(&self, data_type: OracleDataType) -> bool;
}

/// Mock oracle provider for testing
#[derive(Debug, Default)]
pub struct MockOracleProvider {
    default_esg_score: Option<ESGScore>,
}

impl MockOracleProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_esg_score(mut self, score: ESGScore) -> Self {
        self.default_esg_score = Some(score);
        self
    }
}

impl OracleProvider for MockOracleProvider {
    fn request(&self, request: OracleRequest) -> Result<OracleResponse, String> {
        let data = match request.data_type {
            OracleDataType::ESGScore => {
                let score = self
                    .default_esg_score
                    .clone()
                    .unwrap_or_else(|| ESGScore::new(80.0, 75.0, 70.0));
                OracleData::ESGScore(score)
            }
            OracleDataType::CarbonEmissions => OracleData::CarbonEmissions(1000.0),
            OracleDataType::RegulatoryStatus => OracleData::RegulatoryStatus(true),
            OracleDataType::SanctionsCheck => OracleData::SanctionsCheck(false),
            OracleDataType::CreditRating => OracleData::CreditRating("A".to_string()),
        };

        Ok(OracleResponse::new(request.id, data))
    }

    fn supports(&self, _data_type: OracleDataType) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_request() {
        let request =
            OracleRequest::new(OracleDataType::ESGScore, "0x1234567890abcdef".to_string());

        assert_eq!(request.data_type, OracleDataType::ESGScore);
        assert_eq!(request.entity_id, "0x1234567890abcdef");
    }

    #[test]
    fn test_mock_oracle_provider() {
        let oracle = MockOracleProvider::new();
        let request =
            OracleRequest::new(OracleDataType::ESGScore, "0x1234567890abcdef".to_string());

        let response = oracle.request(request).unwrap();

        match response.data {
            OracleData::ESGScore(score) => {
                assert!(score.total > 0.0);
            }
            _ => panic!("Expected ESGScore data"),
        }
    }

    #[test]
    fn test_mock_oracle_with_custom_score() {
        let custom_score = ESGScore::new(95.0, 90.0, 85.0);
        let oracle = MockOracleProvider::new().with_esg_score(custom_score.clone());

        let request =
            OracleRequest::new(OracleDataType::ESGScore, "0x1234567890abcdef".to_string());

        let response = oracle.request(request).unwrap();

        match response.data {
            OracleData::ESGScore(score) => {
                assert_eq!(score.environmental, 95.0);
            }
            _ => panic!("Expected ESGScore data"),
        }
    }
}
