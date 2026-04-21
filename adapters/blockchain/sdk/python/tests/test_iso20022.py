"""Tests for ISO 20022 bridge functionality."""

from erc8040_sdk import (
    ESGClassification,
    ESGRating,
    ESGScore,
    FinancialInstrument,
    ISO20022Bridge,
)


def test_map_sfdr_article_article_9():
    """Test SFDR mapping for Article 9 ratings (AAA, AA, A)."""
    bridge = ISO20022Bridge()

    assert bridge.map_sfdr_article(ESGRating.AAA) == 9
    assert bridge.map_sfdr_article(ESGRating.AA) == 9
    assert bridge.map_sfdr_article(ESGRating.A) == 9


def test_map_sfdr_article_article_8():
    """Test SFDR mapping for Article 8 ratings (BBB, BB)."""
    bridge = ISO20022Bridge()

    assert bridge.map_sfdr_article(ESGRating.BBB) == 8
    assert bridge.map_sfdr_article(ESGRating.BB) == 8


def test_map_sfdr_article_article_6():
    """Test SFDR mapping for Article 6 ratings (B and below)."""
    bridge = ISO20022Bridge()

    assert bridge.map_sfdr_article(ESGRating.B) == 6
    assert bridge.map_sfdr_article(ESGRating.CCC) == 6
    assert bridge.map_sfdr_article(ESGRating.CC) == 6
    assert bridge.map_sfdr_article(ESGRating.C) == 6
    assert bridge.map_sfdr_article(ESGRating.D) == 6


def test_calculate_taxonomy_alignment_high_score():
    """Test taxonomy alignment for high environmental scores (>= 80)."""
    bridge = ISO20022Bridge()

    score_90 = ESGScore.create(90.0, 85.0, 80.0)
    assert bridge.calculate_taxonomy_alignment(score_90) == 90.0

    score_85 = ESGScore.create(85.0, 80.0, 75.0)
    assert bridge.calculate_taxonomy_alignment(score_85) == 85.0

    score_80 = ESGScore.create(80.0, 75.0, 70.0)
    assert bridge.calculate_taxonomy_alignment(score_80) == 80.0


def test_calculate_taxonomy_alignment_medium_score():
    """Test taxonomy alignment for medium environmental scores (60-79)."""
    bridge = ISO20022Bridge()

    score_75 = ESGScore.create(75.0, 70.0, 68.0)
    alignment = bridge.calculate_taxonomy_alignment(score_75)
    assert abs(alignment - 30.0) < 0.01  # (75 - 60) * 2 = 30

    score_65 = ESGScore.create(65.0, 60.0, 58.0)
    alignment = bridge.calculate_taxonomy_alignment(score_65)
    assert abs(alignment - 10.0) < 0.01  # (65 - 60) * 2 = 10

    score_60 = ESGScore.create(60.0, 55.0, 50.0)
    alignment = bridge.calculate_taxonomy_alignment(score_60)
    assert abs(alignment - 0.0) < 0.01  # (60 - 60) * 2 = 0


def test_calculate_taxonomy_alignment_low_score():
    """Test taxonomy alignment for low environmental scores (< 60)."""
    bridge = ISO20022Bridge()

    score_50 = ESGScore.create(50.0, 45.0, 40.0)
    assert bridge.calculate_taxonomy_alignment(score_50) == 0.0

    score_30 = ESGScore.create(30.0, 25.0, 20.0)
    assert bridge.calculate_taxonomy_alignment(score_30) == 0.0


def test_esg_to_iso_high_performance():
    """Test ESG to ISO conversion for high ESG performance."""
    bridge = ISO20022Bridge()
    score = ESGScore.create(90.0, 85.0, 80.0)

    classification = bridge.esg_to_iso(score)

    assert classification.taxonomy_alignment == 90.0
    assert classification.sfdr_article == 9
    assert classification.erc8040_rating == "AA"
    assert classification.carbon_intensity is not None
    assert abs(classification.carbon_intensity - 50.0) < 0.01  # 500 * (1 - 0.9)


def test_esg_to_iso_medium_performance():
    """Test ESG to ISO conversion for medium ESG performance."""
    bridge = ISO20022Bridge()
    score = ESGScore.create(75.0, 70.0, 68.0)

    classification = bridge.esg_to_iso(score)

    assert abs(classification.taxonomy_alignment - 30.0) < 0.01
    assert classification.sfdr_article == 8
    assert classification.erc8040_rating == "BBB"
    assert abs(classification.carbon_intensity - 125.0) < 0.01  # 500 * (1 - 0.75)


def test_esg_to_iso_low_performance():
    """Test ESG to ISO conversion for low ESG performance."""
    bridge = ISO20022Bridge()
    score = ESGScore.create(45.0, 50.0, 48.0)

    classification = bridge.esg_to_iso(score)

    assert classification.taxonomy_alignment == 0.0
    assert classification.sfdr_article == 6
    assert classification.erc8040_rating == "CCC"
    assert abs(classification.carbon_intensity - 275.0) < 0.01  # 500 * (1 - 0.45)


def test_create_setr_message():
    """Test SETR message XML generation."""
    bridge = ISO20022Bridge()

    instrument = FinancialInstrument(
        isin="US46434G1031",
        lei="549300PZDW6EBUUJ8G35",
        name="ERC8040 Green Bond"
    )

    classification = ESGClassification(
        taxonomy_alignment=92.0,
        sfdr_article=9,
        erc8040_rating="AA",
        carbon_intensity=40.0
    )

    xml = bridge.create_setr_message(instrument, classification)

    # Verify XML structure and content
    assert '<?xml version="1.0" encoding="UTF-8"?>' in xml
    assert 'urn:iso:std:iso:20022:tech:xsd:setr.010.001.04' in xml
    assert '<ISIN>US46434G1031</ISIN>' in xml
    assert '<LEI>549300PZDW6EBUUJ8G35</LEI>' in xml
    assert '<Nm>ERC8040 Green Bond</Nm>' in xml
    assert '<TaxnmyAlgnmt>92.0</TaxnmyAlgnmt>' in xml
    assert '<SFDRArtcl>9</SFDRArtcl>' in xml
    assert '<ERC8040Rtg>AA</ERC8040Rtg>' in xml


def test_create_setr_message_integration():
    """Test complete integration from ESG score to SETR message."""
    bridge = ISO20022Bridge()

    # Create ESG score
    score = ESGScore.create(
        environmental=92.0,
        social=88.0,
        governance=85.0
    )

    # Convert to ISO classification
    classification = bridge.esg_to_iso(score)

    # Create financial instrument
    instrument = FinancialInstrument(
        isin="US1234567890",
        lei="123456789012345678XX",
        name="ERC8040 Sustainable Energy Token"
    )

    # Generate SETR message
    xml = bridge.create_setr_message(instrument, classification)

    # Verify complete workflow
    assert '<ISIN>US1234567890</ISIN>' in xml
    assert '<TaxnmyAlgnmt>92.0</TaxnmyAlgnmt>' in xml
    assert '<SFDRArtcl>9</SFDRArtcl>' in xml
    assert '<ERC8040Rtg>AA</ERC8040Rtg>' in xml


def test_financial_instrument_creation():
    """Test financial instrument dataclass."""
    instrument = FinancialInstrument(
        isin="US46434G1031",
        lei="549300PZDW6EBUUJ8G35",
        name="Test Instrument"
    )

    assert instrument.isin == "US46434G1031"
    assert instrument.lei == "549300PZDW6EBUUJ8G35"
    assert instrument.name == "Test Instrument"


def test_esg_classification_creation():
    """Test ESG classification dataclass."""
    classification = ESGClassification(
        taxonomy_alignment=85.0,
        sfdr_article=9,
        erc8040_rating="AA",
        carbon_intensity=75.0
    )

    assert classification.taxonomy_alignment == 85.0
    assert classification.sfdr_article == 9
    assert classification.erc8040_rating == "AA"
    assert classification.carbon_intensity == 75.0


def test_esg_classification_without_carbon_intensity():
    """Test ESG classification with optional carbon intensity."""
    classification = ESGClassification(
        taxonomy_alignment=85.0,
        sfdr_article=9,
        erc8040_rating="AA"
    )

    assert classification.taxonomy_alignment == 85.0
    assert classification.carbon_intensity is None


def test_carbon_intensity_estimation():
    """Test carbon intensity estimation across score range."""
    bridge = ISO20022Bridge()

    # High environmental score = low carbon intensity
    score_100 = ESGScore.create(100.0, 100.0, 100.0)
    classification = bridge.esg_to_iso(score_100)
    assert abs(classification.carbon_intensity - 0.0) < 0.01

    # Medium environmental score = medium carbon intensity
    score_50 = ESGScore.create(50.0, 50.0, 50.0)
    classification = bridge.esg_to_iso(score_50)
    assert abs(classification.carbon_intensity - 250.0) < 0.01

    # Low environmental score = high carbon intensity
    score_0 = ESGScore.create(0.0, 50.0, 50.0)
    classification = bridge.esg_to_iso(score_0)
    assert abs(classification.carbon_intensity - 500.0) < 0.01
