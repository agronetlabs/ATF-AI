#include "erc8040/esg.hpp"
#include <algorithm>
#include <cmath>

namespace erc8040 {

namespace {
uint8_t clamp_score(double value) {
    double clamped = std::clamp(value, 0.0, 100.0);
    return static_cast<uint8_t>(std::lround(clamped));
}
}  // namespace
bool ESGScore::is_investment_grade() const {
    return rating == ESGRating::AAA || 
           rating == ESGRating::AA || 
           rating == ESGRating::A || 
           rating == ESGRating::BBB;
}

ESGScoring::ESGScoring(double env_weight, double social_weight, double gov_weight)
    : env_weight_(env_weight), social_weight_(social_weight), gov_weight_(gov_weight) {
    if (env_weight < 0.0 || social_weight < 0.0 || gov_weight < 0.0) {
        throw std::invalid_argument("ESG weights must be non-negative");
    }

    double total = env_weight + social_weight + gov_weight;
    if (total <= 0.0) {
        throw std::invalid_argument("ESG weights must sum to > 0");
    }

    env_weight_ /= total;
    social_weight_ /= total;
    gov_weight_ /= total;
}

ESGScore ESGScoring::calculate(double environmental, double social, double governance) const {
    uint8_t environmental_score = clamp_score(environmental);
    uint8_t social_score = clamp_score(social);
    uint8_t governance_score = clamp_score(governance);

    double weighted = environmental_score * env_weight_ + 
                      social_score * social_weight_ + 
                      governance_score * gov_weight_;
    uint8_t total = clamp_score(weighted);
    ESGRating rating = rating_from_score(total);
    
    return ESGScore{
        environmental_score,
        social_score,
        governance_score,
        total,
        rating
    };
}

ESGRating ESGScoring::rating_from_score(uint8_t score) {
    if (score >= 90) return ESGRating::AAA;
    if (score >= 85) return ESGRating::AA;
    if (score >= 80) return ESGRating::A;
    if (score >= 70) return ESGRating::BBB;
    if (score >= 60) return ESGRating::BB;
    if (score >= 50) return ESGRating::B;
    if (score >= 40) return ESGRating::CCC;
    if (score >= 30) return ESGRating::CC;
    if (score >= 20) return ESGRating::C;
    return ESGRating::D;
}

std::string ESGScoring::rating_to_string(ESGRating rating) {
    switch (rating) {
        case ESGRating::AAA: return "AAA";
        case ESGRating::AA: return "AA";
        case ESGRating::A: return "A";
        case ESGRating::BBB: return "BBB";
        case ESGRating::BB: return "BB";
        case ESGRating::B: return "B";
        case ESGRating::CCC: return "CCC";
        case ESGRating::CC: return "CC";
        case ESGRating::C: return "C";
        case ESGRating::D: return "D";
        default: return "Unknown";
    }
}

} // namespace erc8040
