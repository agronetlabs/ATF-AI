#include <iostream>
#include <cassert>
#include <stdexcept>
#include "erc8040/esg.hpp"

using namespace erc8040;

void test_esg_scoring() {
    ESGScoring scorer;
    ESGScore score = scorer.calculate(85.0, 78.0, 92.0);
    
    assert(score.environmental == 85);
    assert(score.social == 78);
    assert(score.governance == 92);
    assert(score.total >= 80 && score.total <= 90);
    std::cout << "[PASS] test_esg_scoring\n";
}

void test_rating_from_score() {
    assert(ESGScoring::rating_from_score(95) == ESGRating::AAA);
    assert(ESGScoring::rating_from_score(85) == ESGRating::AA);
    assert(ESGScoring::rating_from_score(80) == ESGRating::A);
    assert(ESGScoring::rating_from_score(70) == ESGRating::BBB);
    assert(ESGScoring::rating_from_score(60) == ESGRating::BB);
    assert(ESGScoring::rating_from_score(50) == ESGRating::B);
    assert(ESGScoring::rating_from_score(40) == ESGRating::CCC);
    assert(ESGScoring::rating_from_score(35) == ESGRating::CC);
    assert(ESGScoring::rating_from_score(30) == ESGRating::CC);
    assert(ESGScoring::rating_from_score(20) == ESGRating::C);
    assert(ESGScoring::rating_from_score(5) == ESGRating::D);
    std::cout << "[PASS] test_rating_from_score\n";
}

void test_investment_grade() {
    ESGScoring scorer;
    
    ESGScore high_score = scorer.calculate(90.0, 85.0, 88.0);
    assert(high_score.is_investment_grade() == true);
    
    ESGScore low_score = scorer.calculate(30.0, 25.0, 28.0);
    assert(low_score.is_investment_grade() == false);
    
    std::cout << "[PASS] test_investment_grade\n";
}

void test_rating_to_string() {
    assert(ESGScoring::rating_to_string(ESGRating::AAA) == "AAA");
    assert(ESGScoring::rating_to_string(ESGRating::BB) == "BB");
    assert(ESGScoring::rating_to_string(ESGRating::D) == "D");
    std::cout << "[PASS] test_rating_to_string\n";
}

void test_invalid_weights() {
    try {
        ESGScoring scorer(-1.0, 1.0, 1.0);
        (void)scorer;
        assert(false && "Expected invalid_argument for negative weight");
    } catch (const std::invalid_argument&) {
    }

    try {
        ESGScoring scorer(0.0, 0.0, 0.0);
        (void)scorer;
        assert(false && "Expected invalid_argument for zero weights");
    } catch (const std::invalid_argument&) {
    }

    std::cout << "[PASS] test_invalid_weights\n";
}

int main() {
    std::cout << "=== ERC-8040 C++ SDK - ESG Tests ===\n\n";
    
    test_esg_scoring();
    test_rating_from_score();
    test_investment_grade();
    test_rating_to_string();
    test_invalid_weights();
    
    std::cout << "\n✅ All ESG tests passed!\n";
    return 0;
}
