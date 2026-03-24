#include "erc8040/iso20022.hpp"
#include <sstream>

namespace erc8040 {

ISO20022Bridge::ISO20022Bridge() {}

ESGClassification ISO20022Bridge::esg_to_iso(const ESGScore& score) const {
    return ESGClassification{
        calculate_taxonomy_alignment(score),
        map_sfdr_article(score.rating),
        ESGScoring::rating_to_string(score.rating),
        std::nullopt
    };
}

uint8_t ISO20022Bridge::map_sfdr_article(ESGRating rating) const {
    switch (rating) {
        case ESGRating::AAA:
        case ESGRating::AA:
        case ESGRating::A:
            return 9;
        case ESGRating::BBB:
        case ESGRating::BB:
            return 8;
        default:
            return 6;
    }
}

double ISO20022Bridge::calculate_taxonomy_alignment(const ESGScore& score) const {
    return static_cast<double>(score.environmental) / 100.0;
}

std::string ISO20022Bridge::create_setr_message(
    const FinancialInstrument& instrument,
    const ESGClassification& esg) const {
    std::ostringstream xml;
    xml << "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    xml << "<Document xmlns=\"urn:iso:std:iso:20022:tech:xsd:setr.010.001.04\">\n";
    xml << "  <SctiesTradConf>\n";
    xml << "    <FinInstrmId>\n";
    xml << "      <ISIN>" << instrument.isin << "</ISIN>\n";
    xml << "      <LEI>" << instrument.lei << "</LEI>\n";
    xml << "      <Nm>" << instrument.name << "</Nm>\n";
    xml << "    </FinInstrmId>\n";
    xml << "    <ESGClssfctn>\n";
    xml << "      <TaxnmyAlgnmt>" << esg.taxonomy_alignment << "</TaxnmyAlgnmt>\n";
    xml << "      <SFDRArtcl>" << static_cast<int>(esg.sfdr_article) << "</SFDRArtcl>\n";
    xml << "      <ERC8040Rtg>" << esg.erc8040_rating << "</ERC8040Rtg>\n";
    xml << "    </ESGClssfctn>\n";
    xml << "  </SctiesTradConf>\n";
    xml << "</Document>";
    return xml.str();
}

} // namespace erc8040
