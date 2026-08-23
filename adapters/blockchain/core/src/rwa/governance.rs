use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// High-impact RWA lifecycle operations subject to ATF-AI governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivilegedActionKind {
    Mint,
    Burn,
    Freeze,
    Unfreeze,
    Pause,
    Unpause,
    UpgradeContract,
    ChangeOracle,
    ChangeComplianceRule,
    ChangeSigner,
    Redemption,
    Settlement,
    EmergencyAction,
}

/// ATF-AI governance outcome aligned with the core specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    Approved,
    Rejected,
    Escalated,
}

/// One role-bound approval for a privileged action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Approval {
    pub approver_id: String,
    pub role: String,
    pub approved_at: DateTime<Utc>,
}

/// Deterministic governance policy applied before execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RwaGovernancePolicy {
    pub policy_id: String,
    pub required_quorum: usize,
    pub required_roles: Vec<String>,
    pub require_distinct_approvers: bool,
    pub timelock_seconds: i64,
    pub require_compliance_attestation: bool,
    pub require_provenance: bool,
    pub block_upgrade_with_active_redemptions: bool,
}

impl RwaGovernancePolicy {
    /// Conservative baseline for institutional privileged actions.
    pub fn institutional_default(policy_id: impl Into<String>) -> Self {
        Self {
            policy_id: policy_id.into(),
            required_quorum: 3,
            required_roles: vec![
                "issuer".to_string(),
                "compliance_officer".to_string(),
                "security_officer".to_string(),
            ],
            require_distinct_approvers: true,
            timelock_seconds: 0,
            require_compliance_attestation: true,
            require_provenance: true,
            block_upgrade_with_active_redemptions: true,
        }
    }
}

/// Runtime evidence supplied to the deterministic validator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwaActionContext {
    pub action_id: String,
    pub kind: PrivilegedActionKind,
    pub created_at: DateTime<Utc>,
    pub provenance_hash: Option<String>,
    pub compliance_attestation_valid: bool,
    pub active_redemptions: bool,
    pub approvals: Vec<Approval>,
}

/// Result of policy evaluation. Signing/execution must only consume Approved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceResult {
    pub action_id: String,
    pub policy_id: String,
    pub decision: GovernanceDecision,
    pub reasons: Vec<String>,
    pub evaluated_at: DateTime<Utc>,
    pub execution_eligible_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct RwaGovernanceValidator;

impl RwaGovernanceValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate a privileged action before any cryptographic signing layer.
    ///
    /// `Rejected` means a hard policy violation. `Escalated` means the request
    /// is not yet execution-eligible (for example, quorum, roles, or timelock
    /// are incomplete) but may become eligible without changing the policy.
    pub fn validate(
        &self,
        policy: &RwaGovernancePolicy,
        context: &RwaActionContext,
        now: DateTime<Utc>,
    ) -> GovernanceResult {
        let mut reasons = Vec::new();
        let mut hard_failure = false;
        let mut incomplete = false;

        if policy.policy_id.trim().is_empty() {
            reasons.push("policy_id must be immutable and non-empty".to_string());
            hard_failure = true;
        }

        if context.action_id.trim().is_empty() {
            reasons.push("action_id must be non-empty".to_string());
            hard_failure = true;
        }

        if policy.require_provenance
            && context
                .provenance_hash
                .as_ref()
                .map(|hash| hash.trim().is_empty())
                .unwrap_or(true)
        {
            reasons.push("required provenance hash is missing".to_string());
            hard_failure = true;
        }

        if policy.require_compliance_attestation && !context.compliance_attestation_valid {
            reasons.push("required compliance attestation is not valid".to_string());
            hard_failure = true;
        }

        if policy.block_upgrade_with_active_redemptions
            && context.kind == PrivilegedActionKind::UpgradeContract
            && context.active_redemptions
        {
            reasons.push("contract upgrade blocked while redemptions are active".to_string());
            hard_failure = true;
        }

        let distinct_approvers: HashSet<&str> = context
            .approvals
            .iter()
            .map(|approval| approval.approver_id.as_str())
            .collect();

        if policy.require_distinct_approvers
            && distinct_approvers.len() != context.approvals.len()
        {
            reasons.push("duplicate approver identity detected".to_string());
            hard_failure = true;
        }

        if distinct_approvers.len() < policy.required_quorum {
            reasons.push(format!(
                "approval quorum incomplete: {} of {} distinct approvals",
                distinct_approvers.len(),
                policy.required_quorum
            ));
            incomplete = true;
        }

        let approved_roles: HashSet<&str> = context
            .approvals
            .iter()
            .map(|approval| approval.role.as_str())
            .collect();

        let missing_roles: Vec<&str> = policy
            .required_roles
            .iter()
            .map(String::as_str)
            .filter(|role| !approved_roles.contains(role))
            .collect();

        if !missing_roles.is_empty() {
            reasons.push(format!(
                "required approval roles missing: {}",
                missing_roles.join(", ")
            ));
            incomplete = true;
        }

        let timelock_seconds = policy.timelock_seconds.max(0);
        let execution_eligible_at = context.created_at + Duration::seconds(timelock_seconds);

        if now < execution_eligible_at {
            reasons.push(format!(
                "timelock active until {}",
                execution_eligible_at.to_rfc3339()
            ));
            incomplete = true;
        }

        let decision = if hard_failure {
            GovernanceDecision::Rejected
        } else if incomplete {
            GovernanceDecision::Escalated
        } else {
            reasons.push("all deterministic governance requirements satisfied".to_string());
            GovernanceDecision::Approved
        };

        GovernanceResult {
            action_id: context.action_id.clone(),
            policy_id: policy.policy_id.clone(),
            decision,
            reasons,
            evaluated_at: now,
            execution_eligible_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ts(hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 8, 22, hour, 0, 0)
            .single()
            .unwrap()
    }

    fn approval(id: &str, role: &str) -> Approval {
        Approval {
            approver_id: id.to_string(),
            role: role.to_string(),
            approved_at: ts(10),
        }
    }

    fn valid_context(kind: PrivilegedActionKind) -> RwaActionContext {
        RwaActionContext {
            action_id: "urn:uuid:action-001".to_string(),
            kind,
            created_at: ts(9),
            provenance_hash: Some("a".repeat(64)),
            compliance_attestation_valid: true,
            active_redemptions: false,
            approvals: vec![
                approval("did:issuer:1", "issuer"),
                approval("did:compliance:1", "compliance_officer"),
                approval("did:security:1", "security_officer"),
            ],
        }
    }

    #[test]
    fn approves_when_policy_is_satisfied() {
        let validator = RwaGovernanceValidator::new();
        let policy = RwaGovernancePolicy::institutional_default("atf-policy:rwa:1.0.0");
        let result = validator.validate(
            &policy,
            &valid_context(PrivilegedActionKind::Redemption),
            ts(11),
        );

        assert_eq!(result.decision, GovernanceDecision::Approved);
    }

    #[test]
    fn escalates_when_quorum_is_incomplete() {
        let validator = RwaGovernanceValidator::new();
        let policy = RwaGovernancePolicy::institutional_default("atf-policy:rwa:1.0.0");
        let mut context = valid_context(PrivilegedActionKind::Redemption);
        context.approvals.pop();

        let result = validator.validate(&policy, &context, ts(11));
        assert_eq!(result.decision, GovernanceDecision::Escalated);
    }

    #[test]
    fn rejects_missing_provenance() {
        let validator = RwaGovernanceValidator::new();
        let policy = RwaGovernancePolicy::institutional_default("atf-policy:rwa:1.0.0");
        let mut context = valid_context(PrivilegedActionKind::Redemption);
        context.provenance_hash = None;

        let result = validator.validate(&policy, &context, ts(11));
        assert_eq!(result.decision, GovernanceDecision::Rejected);
    }

    #[test]
    fn rejects_duplicate_approvers_when_distinct_required() {
        let validator = RwaGovernanceValidator::new();
        let policy = RwaGovernancePolicy::institutional_default("atf-policy:rwa:1.0.0");
        let mut context = valid_context(PrivilegedActionKind::Redemption);
        context.approvals[2].approver_id = context.approvals[1].approver_id.clone();

        let result = validator.validate(&policy, &context, ts(11));
        assert_eq!(result.decision, GovernanceDecision::Rejected);
    }

    #[test]
    fn rejects_upgrade_while_redemptions_are_active() {
        let validator = RwaGovernanceValidator::new();
        let policy = RwaGovernancePolicy::institutional_default("atf-policy:rwa:1.0.0");
        let mut context = valid_context(PrivilegedActionKind::UpgradeContract);
        context.active_redemptions = true;

        let result = validator.validate(&policy, &context, ts(11));
        assert_eq!(result.decision, GovernanceDecision::Rejected);
    }

    #[test]
    fn escalates_until_timelock_expires() {
        let validator = RwaGovernanceValidator::new();
        let mut policy = RwaGovernancePolicy::institutional_default("atf-policy:rwa:1.0.0");
        policy.timelock_seconds = 4 * 60 * 60;
        let context = valid_context(PrivilegedActionKind::ChangeSigner);

        let result = validator.validate(&policy, &context, ts(11));
        assert_eq!(result.decision, GovernanceDecision::Escalated);

        let later = validator.validate(&policy, &context, ts(14));
        assert_eq!(later.decision, GovernanceDecision::Approved);
    }
}
