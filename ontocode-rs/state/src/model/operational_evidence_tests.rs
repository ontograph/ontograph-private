use super::EvidenceDomain;
use super::EvidenceRisk;
use super::EvidenceStatus;
use super::RedactionStatus;
use pretty_assertions::assert_eq;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::FromStr;

fn assert_snake_case_enum<T>(value: T, snake_case: &str)
where
    T: Serialize + DeserializeOwned + FromStr + PartialEq + Debug,
    <T as FromStr>::Err: Debug,
{
    assert_eq!(
        serde_json::to_string(&value).expect("enum should serialize"),
        format!("\"{snake_case}\"")
    );
    assert_eq!(
        serde_json::from_str::<T>(&format!("\"{snake_case}\"")).expect("enum should deserialize"),
        value
    );
    assert_eq!(T::from_str(snake_case).expect("enum should parse"), value);
}

#[test]
fn evidence_domain_uses_snake_case_strings() {
    assert_snake_case_enum(EvidenceDomain::RuntimeTopology, "runtime_topology");
}

#[test]
fn evidence_status_uses_snake_case_strings() {
    assert_snake_case_enum(EvidenceStatus::Dispatched, "dispatched");
}

#[test]
fn evidence_risk_uses_snake_case_strings() {
    assert_snake_case_enum(EvidenceRisk::Critical, "critical");
}

#[test]
fn redaction_status_uses_snake_case_strings() {
    assert_snake_case_enum(RedactionStatus::Redacted, "redacted");
}
