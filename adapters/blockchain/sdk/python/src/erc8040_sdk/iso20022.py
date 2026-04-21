"""ISO 20022 bridge for ERC-8040 ESG compliance integration."""

from dataclasses import dataclass

from erc8040_sdk.esg import ESGRating, ESGScore


@dataclass
class FinancialInstrument:
    """Financial instrument identification for ISO 20022.

    Attributes:
        isin: International Securities Identification Number (12 characters)
        lei: Legal Entity Identifier (20 characters)
        name: Human-readable instrument name
    """

    isin: str
    lei: str
    name: str


@dataclass
class ESGClassification:
    """ESG classification for ISO 20022 messaging.

    Attributes:
        taxonomy_alignment: EU Taxonomy alignment percentage (0-100)
        sfdr_article: SFDR Article classification (6, 8, or 9)
        erc8040_rating: ERC-8040 letter rating (AAA to D)
        carbon_intensity: Optional carbon intensity in tCO2e/$M revenue
    """

    taxonomy_alignment: float
    sfdr_article: int
    erc8040_rating: str
    carbon_intensity: float | None = None


class ISO20022Bridge:
    """Bridge between ERC-8040 ESG scores and ISO 20022 format.

    This bridge provides seamless conversion between blockchain-based ESG
    compliance tokens and traditional financial messaging standards.

    Example:
        >>> score = ESGScore.create(90.0, 85.0, 80.0)
        >>> bridge = ISO20022Bridge()
        >>> classification = bridge.esg_to_iso(score)
        >>> print(classification.sfdr_article)
        9
    """

    def esg_to_iso(self, score: ESGScore) -> ESGClassification:
        """Convert ESG score to ISO 20022 ESG classification.

        Args:
            score: ERC-8040 ESG score

        Returns:
            ESG classification suitable for ISO 20022 messages
        """
        return ESGClassification(
            taxonomy_alignment=self.calculate_taxonomy_alignment(score),
            sfdr_article=self.map_sfdr_article(score.rating),
            erc8040_rating=score.rating.value,
            carbon_intensity=self._estimate_carbon_intensity(score),
        )

    def map_sfdr_article(self, rating: ESGRating) -> int:
        """Map ERC-8040 rating to SFDR article classification.

        SFDR (Sustainable Finance Disclosure Regulation) articles:
        - Article 9: Sustainable investment objective (AAA, AA, A)
        - Article 8: Promotes ESG characteristics (BBB, BB)
        - Article 6: No sustainability objective (B, CCC, CC, C, D)

        Args:
            rating: ERC-8040 ESG rating

        Returns:
            SFDR article number (6, 8, or 9)
        """
        if rating in (ESGRating.AAA, ESGRating.AA, ESGRating.A):
            return 9  # Sustainable investment objective
        elif rating in (ESGRating.BBB, ESGRating.BB):
            return 8  # Promotes ESG characteristics
        else:
            return 6  # No sustainability objective

    def calculate_taxonomy_alignment(self, score: ESGScore) -> float:
        """Calculate EU Taxonomy alignment percentage.

        The alignment is calculated based on the environmental score:
        - Score >= 80: Direct alignment (score%)
        - Score 60-79: Scaled alignment ((score - 60) * 2)
        - Score < 60: No alignment (0%)

        Args:
            score: ERC-8040 ESG score

        Returns:
            EU Taxonomy alignment percentage (0-100)
        """
        env = score.environmental
        if env >= 80.0:
            return min(env, 100.0)
        elif env >= 60.0:
            return (env - 60.0) * 2.0
        else:
            return 0.0

    def create_setr_message(
        self,
        instrument: FinancialInstrument,
        esg: ESGClassification,
    ) -> str:
        """Create ISO 20022 SETR (Securities Trade) message with ESG data.

        Generates a setr.010.001.04 Securities Trade Confirmation message
        with embedded ESG classification.

        Args:
            instrument: Financial instrument identification
            esg: ESG classification

        Returns:
            ISO 20022 XML message as string
        """
        xml_parts = [
            '<?xml version="1.0" encoding="UTF-8"?>',
            '<Document xmlns="urn:iso:std:iso:20022:tech:xsd:setr.010.001.04">',
            '  <SctiesTradConf>',
            '    <FinInstrmId>',
            f'      <ISIN>{instrument.isin}</ISIN>',
            f'      <LEI>{instrument.lei}</LEI>',
            f'      <Nm>{instrument.name}</Nm>',
            '    </FinInstrmId>',
            '    <ESGClssfctn>',
            f'      <TaxnmyAlgnmt>{esg.taxonomy_alignment}</TaxnmyAlgnmt>',
            f'      <SFDRArtcl>{esg.sfdr_article}</SFDRArtcl>',
            f'      <ERC8040Rtg>{esg.erc8040_rating}</ERC8040Rtg>',
            '    </ESGClssfctn>',
            '  </SctiesTradConf>',
            '</Document>',
        ]
        return '\n'.join(xml_parts)

    def _estimate_carbon_intensity(self, score: ESGScore) -> float:
        """Estimate carbon intensity based on environmental score.

        Carbon intensity is inversely related to environmental performance.
        Returns tCO2e per $M revenue.

        Args:
            score: ERC-8040 ESG score

        Returns:
            Estimated carbon intensity in tCO2e/$M revenue
        """
        max_intensity = 500.0
        return max_intensity * (1.0 - score.environmental / 100.0)
