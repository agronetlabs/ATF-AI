"""Tests for ESG scoring and rating functionality."""

from erc8040_sdk import ESGRating, ESGScore, ESGScoring


def test_esg_rating_from_score():
    """Test ESG rating calculation from score."""
    assert ESGRating.from_score(95) == ESGRating.AAA
    assert ESGRating.from_score(75) == ESGRating.BBB
    assert ESGRating.from_score(45) == ESGRating.CCC


def test_esg_rating_investment_grade():
    """Test investment grade classification."""
    assert ESGRating.AAA.is_investment_grade
    assert ESGRating.BBB.is_investment_grade
    assert not ESGRating.BB.is_investment_grade


def test_esg_scoring():
    """Test ESG scoring calculation."""
    scoring = ESGScoring()
    score = scoring.calculate(80.0, 70.0, 90.0)
    assert score.total > 0
    assert score.rating in list(ESGRating)


def test_esg_score_create():
    """Test ESG score creation."""
    score = ESGScore.create(90.0, 85.0, 80.0)
    assert score.environmental == 90.0
    assert score.social == 85.0
    assert score.governance == 80.0
    assert abs(score.total - 85.0) < 0.01
    assert score.rating == ESGRating.AA


def test_esg_scoring_custom_weights():
    """Test ESG scoring with custom weights."""
    scoring = ESGScoring(environmental_weight=2.0, social_weight=1.0, governance_weight=1.0)
    score = scoring.calculate(80.0, 60.0, 60.0)
    # (80*0.5 + 60*0.25 + 60*0.25) = 70
    assert abs(score.total - 70.0) < 0.01


def test_esg_score_is_investment_grade():
    """Test investment grade check on ESG score."""
    score = ESGScore.create(90.0, 85.0, 80.0)
    assert score.is_investment_grade()

    low_score = ESGScore.create(60.0, 55.0, 50.0)
    assert not low_score.is_investment_grade()


def test_esg_scoring_invalid_weights():
    """Test ESG scoring validation for invalid weights."""
    import pytest

    with pytest.raises(ValueError):
        ESGScoring(environmental_weight=-1.0, social_weight=1.0, governance_weight=1.0)

    with pytest.raises(ValueError):
        ESGScoring(environmental_weight=0.0, social_weight=0.0, governance_weight=0.0)
