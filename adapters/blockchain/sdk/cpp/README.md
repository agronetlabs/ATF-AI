# ERC-8040 C++ SDK

High-performance C++ SDK for institutional-grade ESG token operations with ISO 20022 SWIFT integration.

## 🎯 Features

- **ESG Scoring Engine** - AAA to D rating system with weighted calculations
- **Compliance Validation** - EU SFDR, Taxonomy, SEC, MiFID II, Basel support
- **ISO 20022 Bridge** - Native SWIFT message generation for institutional settlement
- **High Performance** - Zero-copy operations, optimized for HFT environments
- **Modern C++17** - RAII, constexpr, structured bindings

## 📁 Structure

```
cpp-sdk/
├── include/erc8040/
│   ├── esg.hpp          # ESG scoring and ratings
│   ├── compliance.hpp   # Regulatory compliance validation
│   └── iso20022.hpp     # SWIFT ISO 20022 bridge
├── src/
│   ├── esg.cpp
│   ├── compliance.cpp
│   └── iso20022.cpp
├── tests/
│   ├── test_esg.cpp
│   └── test_compliance.cpp
└── CMakeLists.txt
```

## 🚀 Quick Start

### Build

```bash
mkdir build && cd build
cmake ..
make
```

### Usage

```cpp
#include <erc8040/esg.hpp>
#include <erc8040/compliance.hpp>
#include <erc8040/iso20022.hpp>

using namespace erc8040;

int main() {
    // Calculate ESG Score
    ESGScoring scorer(0.40, 0.30, 0.30); // weights must be non-negative and sum > 0
    ESGScore score = scorer.calculate(85.0, 78.0, 92.0);
    
    std::cout << "Rating: " << ESGScoring::rating_to_string(score.rating) << "\n";
    
    // Validate Compliance
    ComplianceValidator validator;
    auto result = validator.validate_esg(score, 70);
    
    // Generate ISO 20022 SWIFT Message
    ISO20022Bridge bridge;
    FinancialInstrument instrument{"US0378331005", "HWUPKR0MPOU8FGXBT394", "Apple Inc."};
    ESGClassification esg_class = bridge.esg_to_iso(score);
    std::string swift_xml = bridge.create_setr_message(instrument, esg_class);
    
    return 0;
}
```

## 📊 ESG Rating Scale

| Score | Rating | Investment Grade |
|-------|--------|------------------|
| 90-100 | AAA | ✅ Yes |
| 85-89 | AA | ✅ Yes |
| 80-84 | A | ✅ Yes |
| 70-79 | BBB | ✅ Yes |
| 60-69 | BB | ❌ No |
| 50-59 | B | ❌ No |
| 40-49 | CCC | ❌ No |
| 30-39 | CC | ❌ No |
| 20-29 | C | ❌ No |
| 0-19 | D | ❌ No |

## 🏛️ ATF-AI Compliance

This SDK is part of the ERC-8040 ecosystem, an official ATF-AI adapter.  
Trust model, provenance standards, and governance rules are defined by the [ATF-AI framework](https://github.com/agronetlabs/ATF-AI).

## 📄 License

MIT License - Part of the ERC-8040 Ecosystem
