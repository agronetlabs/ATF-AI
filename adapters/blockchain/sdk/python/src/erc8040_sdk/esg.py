"""ESG scoring and rating module for ERC-8040."""
from enum import Enum

from pydantic import BaseModel, Field


class ESGCategory(str, Enum):
    """ESG Category types."""

    ENVIRONMENTAL = "ENVIRONMENTAL"
    SOCIAL = "SOCIAL"
    GOVERNANCE = "GOVERNANCE"


class ESGRating(str, Enum):
    """ESG Rating levels from D (lowest) to AAA (highest)."""

    D = "D"
    C = "C"
    CC = "CC"
    CCC = "CCC"
    B = "B"
    BB = "BB"
    BBB = "BBB"
    A = "A"
    AA = "AA"
    AAA = "AAA"

    @classmethod
    def from_score(cls, score: float) -> "ESGRating":
        """Convert a total ESG score (0-100) to a rating."""
        if score >= 90.0:
            return cls.AAA
        elif score >= 85.0:
            return cls.AA
        elif score >= 80.0:
            return cls.A
        elif score >= 70.0:
            return cls.BBB
        elif score >= 60.0:
            return cls.BB
        elif score >= 50.0:
            return cls.B
        elif score >= 40.0:
            return cls.CCC
        elif score >= 30.0:
            return cls.CC
        elif score >= 20.0:
            return cls.C
        else:
            return cls.D

    @property
    def is_investment_grade(self) -> bool:
        """Check if this rating is investment grade (BBB or higher)."""
        return self in (ESGRating.AAA, ESGRating.AA, ESGRating.A, ESGRating.BBB)


class ESGScore(BaseModel):
    """ESG Score breakdown."""

    environmental: float = Field(ge=0.0, le=100.0)
    social: float = Field(ge=0.0, le=100.0)
    governance: float = Field(ge=0.0, le=100.0)
    total: float = Field(ge=0.0, le=100.0)
    rating: ESGRating

    @classmethod
    def create(cls, environmental: float, social: float, governance: float) -> "ESGScore":
        """Create a new ESG score with calculated total and rating."""
        total = (environmental + social + governance) / 3.0
        rating = ESGRating.from_score(total)
        return cls(
            environmental=environmental,
            social=social,
            governance=governance,
            total=total,
            rating=rating,
        )

    def is_investment_grade(self) -> bool:
        """Check if this score is investment grade."""
        return self.rating.is_investment_grade


class ESGScoring:
    """ESG Scoring calculator with configurable weights."""

    def __init__(
        self,
        environmental_weight: float = 1.0 / 3.0,
        social_weight: float = 1.0 / 3.0,
        governance_weight: float = 1.0 / 3.0,
    ):
        """Initialize ESG scoring with custom weights.

        Args:
            environmental_weight: Weight for environmental score (default: 1/3)
            social_weight: Weight for social score (default: 1/3)
            governance_weight: Weight for governance score (default: 1/3)
        """
        if environmental_weight < 0 or social_weight < 0 or governance_weight < 0:
            raise ValueError("ESG weights must be non-negative")

        total = environmental_weight + social_weight + governance_weight
        if total <= 0:
            raise ValueError("ESG weights must sum to > 0")

        self.environmental_weight = environmental_weight / total
        self.social_weight = social_weight / total
        self.governance_weight = governance_weight / total

    def calculate(self, environmental: float, social: float, governance: float) -> ESGScore:
        """Calculate ESG score from individual component scores.

        Args:
            environmental: Environmental score (0-100)
            social: Social score (0-100)
            governance: Governance score (0-100)

        Returns:
            ESGScore with weighted total and rating
        """
        weighted_total = (
            environmental * self.environmental_weight
            + social * self.social_weight
            + governance * self.governance_weight
        )

        rating = ESGRating.from_score(weighted_total)

        return ESGScore(
            environmental=environmental,
            social=social,
            governance=governance,
            total=weighted_total,
            rating=rating,
        )
