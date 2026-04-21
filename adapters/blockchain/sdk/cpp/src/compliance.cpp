#include "erc8040/compliance.hpp"

namespace erc8040 {

bool ComplianceRule::is_effective() const {
    return true;
}

ComplianceValidator::ComplianceValidator() {}

ComplianceResult ComplianceValidator::validate_esg(const ESGScore& score, uint8_t min_score) const {
    if (score.total >= min_score) {
        return ComplianceResult{"esg_min_score", ComplianceStatus::Compliant, "ESG score meets minimum requirement"};
    }
    return ComplianceResult{"esg_min_score", ComplianceStatus::NonCompliant, "ESG score below minimum requirement"};
}

std::vector<ComplianceResult> ComplianceValidator::validate_all(
    const ESGScore& score, 
    const std::vector<ComplianceRule>& rules) const {
    std::vector<ComplianceResult> results;
    for (const auto& rule : rules) {
        if (rule.is_effective()) {
            results.push_back(ComplianceResult{rule.id, ComplianceStatus::Compliant, "Rule validated"});
        }
    }
    return results;
}

ComplianceStatus ComplianceValidator::overall_status(const std::vector<ComplianceResult>& results) const {
    for (const auto& result : results) {
        if (result.status == ComplianceStatus::NonCompliant) {
            return ComplianceStatus::NonCompliant;
        }
    }
    for (const auto& result : results) {
        if (result.status == ComplianceStatus::PartiallyCompliant) {
            return ComplianceStatus::PartiallyCompliant;
        }
    }
    return ComplianceStatus::Compliant;
}

} // namespace erc8040
