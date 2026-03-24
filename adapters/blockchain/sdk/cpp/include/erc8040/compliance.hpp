#pragma once
#include <string>
#include <vector>
#include "esg.hpp"

namespace erc8040 {

enum class RegulatoryFramework {
    EU_SFDR, EU_Taxonomy, SEC_Climate, MiFID_II, Basel, Custom
};

enum class Jurisdiction { EU, US, UK, Brazil, Global, Custom };

enum class Severity { Low, Medium, High, Critical };

enum class ComplianceStatus {
    Compliant, PartiallyCompliant, NonCompliant, Pending, NotApplicable
};

struct ComplianceRule {
    std::string id;
    std::string name;
    RegulatoryFramework framework;
    Jurisdiction jurisdiction;
    Severity severity;
    
    bool is_effective() const;
};

struct ComplianceResult {
    std::string rule_id;
    ComplianceStatus status;
    std::string message;
};

class ComplianceValidator {
public:
    ComplianceValidator();
    
    ComplianceResult validate_esg(const ESGScore& score, uint8_t min_score) const;
    std::vector<ComplianceResult> validate_all(const ESGScore& score, 
        const std::vector<ComplianceRule>& rules) const;
    ComplianceStatus overall_status(const std::vector<ComplianceResult>& results) const;
};

} // namespace erc8040
