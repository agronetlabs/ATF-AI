#pragma once
#include <string>
#include <cstdint>
#include <stdexcept>

namespace erc8040 {

enum class ESGCategory { Environmental, Social, Governance };

enum class ESGRating { AAA, AA, A, BBB, BB, B, CCC, CC, C, D };

struct ESGScore {
    uint8_t environmental;
    uint8_t social;
    uint8_t governance;
    uint8_t total;
    ESGRating rating;
    
    bool is_investment_grade() const;
};

class ESGScoring {
public:
    ESGScoring(double env_weight = 0.33, double social_weight = 0.33, double gov_weight = 0.34);
    
    ESGScore calculate(double environmental, double social, double governance) const;
    static ESGRating rating_from_score(uint8_t score);
    static std::string rating_to_string(ESGRating rating);
    
private:
    double env_weight_;
    double social_weight_;
    double gov_weight_;
};

} // namespace erc8040
