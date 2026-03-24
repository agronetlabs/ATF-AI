#pragma once
#include <string>
#include <optional>
#include "esg.hpp"

namespace erc8040 {

struct ESGClassification {
    double taxonomy_alignment;
    uint8_t sfdr_article;
    std::string erc8040_rating;
    std::optional<double> carbon_intensity;
};

struct FinancialInstrument {
    std::string isin;
    std::string lei;
    std::string name;
};

class ISO20022Bridge {
public:
    ISO20022Bridge();
    
    ESGClassification esg_to_iso(const ESGScore& score) const;
    uint8_t map_sfdr_article(ESGRating rating) const;
    double calculate_taxonomy_alignment(const ESGScore& score) const;
    
    /// Generate an ISO 20022 SETR (Securities Trade) XML message with ESG classification.
    /// @param instrument Financial instrument identification (ISIN, LEI, name)
    /// @param esg ESG classification data to embed in the message
    /// @return ISO 20022 setr.010.001.04 XML message as a string
    std::string create_setr_message(
        const FinancialInstrument& instrument,
        const ESGClassification& esg
    ) const;
};

} // namespace erc8040
