use serde::{Deserialize, Serialize};

/// ISO 20022 Message Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ISO20022MessageType {
    SecuritiesTrade,
    PaymentInitiation,
    AccountStatement,
}

/// Securities Trade Related (SETR) Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetrMessage {
    pub message_id: String,
    pub instrument: FinancialInstrument,
    pub esg_classification: Option<ESGClassification>,
    pub quantity: f64,
    pub trade_date: String,
}

/// Payment Initiation (PAIN) Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainMessage {
    pub message_id: String,
    pub debtor: String,
    pub creditor: String,
    pub amount: f64,
    pub currency: String,
}

/// Cash Management (CAMT) Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamtMessage {
    pub message_id: String,
    pub account_id: String,
    pub balance: f64,
    pub currency: String,
}

/// Financial Instrument Identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument {
    /// International Securities Identification Number
    pub isin: Option<String>,
    /// Committee on Uniform Securities Identification Procedures
    pub cusip: Option<String>,
    /// Legal Entity Identifier
    pub lei: Option<String>,
    /// Instrument name
    pub name: String,
}

impl FinancialInstrument {
    pub fn new(name: String) -> Self {
        Self {
            isin: None,
            cusip: None,
            lei: None,
            name,
        }
    }

    pub fn with_isin(mut self, isin: String) -> Self {
        self.isin = Some(isin);
        self
    }

    pub fn with_lei(mut self, lei: String) -> Self {
        self.lei = Some(lei);
        self
    }
}

/// ESG Classification for ISO 20022
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGClassification {
    /// EU Taxonomy alignment percentage (0-100)
    pub taxonomy_alignment: f64,
    /// SFDR Article classification (6, 8, or 9)
    pub sfdr_article: u8,
    /// ERC-8040 rating (AAA to D)
    pub erc8040_rating: String,
    /// Carbon intensity (tCO2e/$M revenue)
    pub carbon_intensity: f64,
}

impl ESGClassification {
    pub fn new(
        taxonomy_alignment: f64,
        sfdr_article: u8,
        erc8040_rating: String,
        carbon_intensity: f64,
    ) -> Self {
        Self {
            taxonomy_alignment,
            sfdr_article,
            erc8040_rating,
            carbon_intensity,
        }
    }
}

/// ESG Purpose classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ESGPurpose {
    GreenBond,
    SocialBond,
    SustainabilityBond,
    SustainabilityLinkedBond,
    TransitionBond,
    Other,
}

impl ESGPurpose {
    /// Get ISO 20022 purpose code
    pub fn iso_code(&self) -> &'static str {
        match self {
            ESGPurpose::GreenBond => "GRBN",
            ESGPurpose::SocialBond => "SOCB",
            ESGPurpose::SustainabilityBond => "SUSB",
            ESGPurpose::SustainabilityLinkedBond => "SUSL",
            ESGPurpose::TransitionBond => "TRBN",
            ESGPurpose::Other => "OTHR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_instrument() {
        let instrument = FinancialInstrument::new("ERC8040 Token".to_string())
            .with_isin("US1234567890".to_string())
            .with_lei("123456789012345678XX".to_string());

        assert_eq!(instrument.name, "ERC8040 Token");
        assert_eq!(instrument.isin, Some("US1234567890".to_string()));
    }

    #[test]
    fn test_esg_classification() {
        let classification = ESGClassification::new(80.0, 9, "AAA".to_string(), 50.0);

        assert_eq!(classification.taxonomy_alignment, 80.0);
        assert_eq!(classification.sfdr_article, 9);
        assert_eq!(classification.erc8040_rating, "AAA");
    }

    #[test]
    fn test_esg_purpose_iso_code() {
        assert_eq!(ESGPurpose::GreenBond.iso_code(), "GRBN");
        assert_eq!(ESGPurpose::SocialBond.iso_code(), "SOCB");
        assert_eq!(ESGPurpose::SustainabilityBond.iso_code(), "SUSB");
    }
}
