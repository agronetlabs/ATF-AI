use erc8040_core::*;

#[test]
fn test_erc8040_version() {
    assert_eq!(VERSION, "0.1.0");
    assert_eq!(STANDARD_ID, "ERC-8040");
}

#[test]
fn test_esg_scoring_workflow() {
    // Create ESG scoring calculator
    let scoring = ESGScoring::new();

    // Calculate ESG score
    let score = scoring.calculate(85.0, 80.0, 75.0);

    assert_eq!(score.rating, esg::ESGRating::A);
    assert!(score.is_investment_grade());
}

#[test]
fn test_compliance_validation_workflow() {
    use chrono::Utc;
    use compliance::*;

    // Create validator
    let mut validator = ComplianceValidator::new();

    // Add a rule
    let rule = ComplianceRule {
        id: "SFDR-001".to_string(),
        framework: RegulatoryFramework::EuSfdr,
        jurisdiction: Jurisdiction::EU,
        category: RuleCategory::EsgDisclosure,
        severity: Severity::High,
        description: "ESG disclosure requirement".to_string(),
        effective_from: Utc::now(),
        effective_until: None,
        required_esg_rating: Some("BBB".to_string()),
    };

    validator.add_rule(rule);

    // Create ESG score
    let esg_score = esg::ESGScore::new(85.0, 80.0, 75.0);

    // Validate
    let results = validator.validate_esg(
        &esg_score,
        Jurisdiction::EU,
        RegulatoryFramework::EuSfdr,
        RuleCategory::EsgDisclosure,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, ComplianceStatus::Compliant);

    // Check overall status
    let overall = validator.overall_status(&results);
    assert_eq!(overall, ComplianceStatus::Compliant);
}

#[test]
fn test_iso20022_bridge_workflow() {
    use iso20022::*;

    // Create bridge
    let bridge = ISO20022Bridge::new();

    // Create ESG score
    let esg_score = esg::ESGScore::new(90.0, 85.0, 80.0);

    // Convert to ISO 20022
    let classification = bridge.esg_to_iso(&esg_score);

    assert_eq!(classification.erc8040_rating, "AA");
    assert_eq!(classification.sfdr_article, 9);
    assert!(classification.taxonomy_alignment > 0.0);

    // Create SETR message
    let instrument =
        FinancialInstrument::new("ERC8040 Token".to_string()).with_isin("US1234567890".to_string());

    let setr = bridge.create_setr_with_esg(instrument, &esg_score, 100.0, "2024-01-01".to_string());

    assert!(setr.esg_classification.is_some());
}

#[test]
fn test_oracle_provider_workflow() {
    use oracle::*;

    // Create mock oracle
    let oracle = MockOracleProvider::new();

    // Request ESG score
    let request = OracleRequest::new(OracleDataType::ESGScore, "0x1234567890abcdef".to_string());

    let response = oracle.request(request).unwrap();

    match response.data {
        OracleData::ESGScore(score) => {
            assert!(score.total > 0.0);
        }
        _ => panic!("Expected ESGScore data"),
    }
}

#[test]
fn test_full_workflow() {
    use compliance::*;
    use iso20022::*;
    use oracle::*;

    // 1. Get ESG score from oracle
    let oracle = MockOracleProvider::new();
    let request = OracleRequest::new(OracleDataType::ESGScore, "0x1234567890abcdef".to_string());
    let response = oracle.request(request).unwrap();

    let esg_score = match response.data {
        OracleData::ESGScore(score) => score,
        _ => panic!("Expected ESGScore"),
    };

    // 2. Validate compliance
    let mut validator = ComplianceValidator::new();
    let rule = ComplianceRule {
        id: "SFDR-001".to_string(),
        framework: RegulatoryFramework::EuSfdr,
        jurisdiction: Jurisdiction::EU,
        category: RuleCategory::EsgDisclosure,
        severity: Severity::High,
        description: "ESG requirement".to_string(),
        effective_from: chrono::Utc::now(),
        effective_until: None,
        required_esg_rating: Some("B".to_string()),
    };
    validator.add_rule(rule);

    let results = validator.validate_all(
        &esg_score,
        Jurisdiction::EU,
        RegulatoryFramework::EuSfdr,
        RuleCategory::EsgDisclosure,
    );
    let overall = validator.overall_status(&results);

    assert_eq!(overall, ComplianceStatus::Compliant);

    // 3. Create ISO 20022 message
    let bridge = ISO20022Bridge::new();
    let instrument = FinancialInstrument::new("ERC8040 Token".to_string());
    let setr = bridge.create_setr_with_esg(instrument, &esg_score, 100.0, "2024-01-01".to_string());

    assert!(setr.esg_classification.is_some());
}
