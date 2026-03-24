#include <iostream>
#include <cassert>
#include "erc8040/compliance.hpp"

using namespace erc8040;

void test_validate_esg_compliant() {
    ComplianceValidator validator;
    ESGScore score{85, 78, 92, 85, ESGRating::AA};
    
    ComplianceResult result = validator.validate_esg(score, 70);
    assert(result.status == ComplianceStatus::Compliant);
    std::cout << "[PASS] test_validate_esg_compliant\n";
}

void test_validate_esg_non_compliant() {
    ComplianceValidator validator;
    ESGScore score{45, 50, 48, 47, ESGRating::B};
    
    ComplianceResult result = validator.validate_esg(score, 60);
    assert(result.status == ComplianceStatus::NonCompliant);
    std::cout << "[PASS] test_validate_esg_non_compliant\n";
}

void test_overall_status_compliant() {
    ComplianceValidator validator;
    std::vector<ComplianceResult> results = {
        {"rule1", ComplianceStatus::Compliant, "OK"},
        {"rule2", ComplianceStatus::Compliant, "OK"},
        {"rule3", ComplianceStatus::Compliant, "OK"}
    };
    
    assert(validator.overall_status(results) == ComplianceStatus::Compliant);
    std::cout << "[PASS] test_overall_status_compliant\n";
}

void test_overall_status_non_compliant() {
    ComplianceValidator validator;
    std::vector<ComplianceResult> results = {
        {"rule1", ComplianceStatus::Compliant, "OK"},
        {"rule2", ComplianceStatus::NonCompliant, "Failed"},
        {"rule3", ComplianceStatus::Compliant, "OK"}
    };
    
    assert(validator.overall_status(results) == ComplianceStatus::NonCompliant);
    std::cout << "[PASS] test_overall_status_non_compliant\n";
}

void test_overall_status_partial() {
    ComplianceValidator validator;
    std::vector<ComplianceResult> results = {
        {"rule1", ComplianceStatus::Compliant, "OK"},
        {"rule2", ComplianceStatus::PartiallyCompliant, "Partial"},
        {"rule3", ComplianceStatus::Compliant, "OK"}
    };
    
    assert(validator.overall_status(results) == ComplianceStatus::PartiallyCompliant);
    std::cout << "[PASS] test_overall_status_partial\n";
}

int main() {
    std::cout << "=== ERC-8040 C++ SDK - Compliance Tests ===\n\n";
    
    test_validate_esg_compliant();
    test_validate_esg_non_compliant();
    test_overall_status_compliant();
    test_overall_status_non_compliant();
    test_overall_status_partial();
    
    std::cout << "\n✅ All Compliance tests passed!\n";
    return 0;
}
