//! RWA privileged-action governance primitives.
//!
//! This module implements deterministic pre-execution policy checks for
//! high-impact Real-World Asset lifecycle operations. It intentionally does
//! not manage or hold signing keys; cryptographic execution remains delegated
//! to MPC, multisig, HSM, hardware-wallet, or custodian infrastructure.

mod governance;

pub use governance::{
    Approval, GovernanceDecision, GovernanceResult, PrivilegedActionKind,
    RwaActionContext, RwaGovernancePolicy, RwaGovernanceValidator,
};
