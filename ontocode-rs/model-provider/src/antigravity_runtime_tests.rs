use pretty_assertions::assert_eq;
use serde_json::json;
use std::collections::HashSet;

use super::AntigravityRefreshDedupeKey;
use super::AntigravityRequestRefreshOwner;

#[test]
fn cloud_code_assist_request_contract_matches_donor_fixture() {
    let owner = AntigravityRequestRefreshOwner::new(
        "antigravity",
        "antigravity:workspace-1",
        Some("workspace-1"),
        "access-secret",
        Some("refresh-secret"),
    );

    let request = owner.cloud_code_assist_request_contract();

    assert_eq!(
        request,
        json!({
            "provider": "antigravity",
            "loadCodeAssist": {
                "metadata": {
                    "ideType": "ANTIGRAVITY",
                    "platform": "PLATFORM_UNSPECIFIED",
                    "pluginType": "GEMINI"
                }
            },
            "onboardUser": {
                "tierId": "legacy-tier",
                "metadata": {
                    "ideType": "ANTIGRAVITY",
                    "platform": "PLATFORM_UNSPECIFIED",
                    "pluginType": "GEMINI"
                }
            }
        })
    );

    let request_json = request.to_string();
    assert!(!request_json.contains("access-secret"));
    assert!(!request_json.contains("refresh-secret"));
}

#[test]
fn refresh_dedupe_key_uses_provider_credential_and_project_metadata_without_tokens() {
    let owner = AntigravityRequestRefreshOwner::new(
        "antigravity",
        "antigravity:workspace-1",
        Some("workspace-1"),
        "access-secret",
        Some("refresh-secret"),
    );

    let key = owner.refresh_dedupe_key();

    assert_eq!(
        key,
        AntigravityRefreshDedupeKey {
            provider_id: "antigravity".to_string(),
            credential_id: "antigravity:workspace-1".to_string(),
            project_id: Some("workspace-1".to_string()),
        }
    );
    assert_eq!(
        key.to_string(),
        "antigravity|antigravity:workspace-1|workspace-1"
    );

    let debug_output = format!("{owner:?}");
    assert!(!debug_output.contains("access-secret"));
    assert!(!debug_output.contains("refresh-secret"));
    assert!(debug_output.contains("<redacted>"));
}

#[test]
fn refresh_dedupe_key_partitions_by_provider_credential_and_project() {
    let owner = AntigravityRequestRefreshOwner::new(
        "antigravity",
        "antigravity:workspace-1",
        Some("workspace-1"),
        "access-secret-a",
        Some("refresh-secret-a"),
    );
    let same_identity_with_new_tokens = AntigravityRequestRefreshOwner::new(
        "antigravity",
        "antigravity:workspace-1",
        Some("workspace-1"),
        "access-secret-b",
        Some("refresh-secret-b"),
    );
    let different_project = AntigravityRequestRefreshOwner::new(
        "antigravity",
        "antigravity:workspace-1",
        Some("workspace-2"),
        "access-secret-a",
        Some("refresh-secret-a"),
    );
    let different_credential = AntigravityRequestRefreshOwner::new(
        "antigravity",
        "antigravity:workspace-2",
        Some("workspace-1"),
        "access-secret-a",
        Some("refresh-secret-a"),
    );

    assert_eq!(
        owner.refresh_dedupe_key(),
        same_identity_with_new_tokens.refresh_dedupe_key()
    );

    let keys = HashSet::from([
        owner.refresh_dedupe_key(),
        same_identity_with_new_tokens.refresh_dedupe_key(),
        different_project.refresh_dedupe_key(),
        different_credential.refresh_dedupe_key(),
    ]);

    assert_eq!(keys.len(), 3);
}
