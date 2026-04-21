//! # ERC-8040 Core Library
//! Implementation of the ERC-8040 Compliance Token Standard.

pub mod compliance;
pub mod esg;
pub mod iso20022;
pub mod oracle;

pub use compliance::{ComplianceRule, ComplianceStatus, ComplianceValidator};
pub use esg::{ESGCategory, ESGScore, ESGScoring};
pub use iso20022::ISO20022Bridge;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const STANDARD_ID: &str = "ERC-8040";
