use std::time::Duration;
use std::time::SystemTime;

use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingView;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use pretty_assertions::assert_eq;

use crate::ProviderCredentialRefreshState;
use crate::ProviderOAuthCredential;
use crate::ProviderOAuthCredentialRecord;
use crate::ProviderOAuthCredentialSourceKindRecord;

#[test]
fn provider_oauth_credential_projects_to_redacted_routing_view() {
    let mut actual = ProviderOAuthCredential::new(
        "GitHub Copilot",
        ProviderCredentialSourceKind::ExternalImport,
        "copilot-account-1",
        "secret-access-token",
    );
    actual.account_id = Some("user-123".to_string());
    actual.endpoint = Some("https://api.github.com/copilot_internal/v2/token".to_string());
    actual.client_id = Some("client-123".to_string());
    actual.token_endpoint = Some("https://github.com/login/oauth/access_token".to_string());
    actual.scopes = vec!["read:user".to_string(), "copilot".to_string()];
    actual.expires_at = Some(1234);
    actual.provenance = Some("copilot-import".to_string());
    actual.refresh_token = Some("secret-refresh-token".to_string());

    let mut expected = ProviderCredentialRoutingView::new(
        "GitHub Copilot",
        ProviderCredentialSourceKind::ExternalImport,
        ProviderCredentialAuthKind::OAuthBearer,
        "copilot-account-1",
    );
    expected.account_id = Some("user-123".to_string());
    expected.endpoint = Some("https://api.github.com/copilot_internal/v2/token".to_string());
    expected.client_id = Some("client-123".to_string());
    expected.scopes = vec!["read:user".to_string(), "copilot".to_string()];
    expected.expires_at = Some(1234);
    expected.provenance = Some("copilot-import".to_string());

    assert_eq!(actual.to_routing_view(), expected);
    assert!(actual.is_refreshable());
}

#[test]
fn provider_oauth_credential_debug_redacts_secret_material() {
    let mut credential = ProviderOAuthCredential::new(
        "OpenAI",
        ProviderCredentialSourceKind::FirstPartyLogin,
        "chatgpt-account-1",
        "super-secret-access-token",
    );
    credential.refresh_token = Some("super-secret-refresh-token".to_string());

    let debug = format!("{credential:?}");

    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("super-secret-access-token"));
    assert!(!debug.contains("super-secret-refresh-token"));
}

#[test]
fn provider_oauth_credential_record_round_trips_and_redacts() {
    let mut credential = ProviderOAuthCredential::new(
        "OpenAI",
        ProviderCredentialSourceKind::ExternalImport,
        "record-account",
        "secret-access-token",
    );
    credential.endpoint = Some("https://example.invalid".to_string());
    credential.refresh_token = Some("secret-refresh-token".to_string());
    credential.scopes = vec!["scope-a".to_string()];

    let record = ProviderOAuthCredentialRecord::from(&credential);
    let debug = format!("{record:?}");

    assert!(record.matches_provider_credential("OpenAI", "record-account"));
    assert_eq!(ProviderOAuthCredential::from(record.clone()), credential);
    assert_eq!(
        record.source_kind,
        ProviderOAuthCredentialSourceKindRecord::ExternalImport
    );
    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains("secret-access-token"));
    assert!(!debug.contains("secret-refresh-token"));
}

#[test]
fn provider_oauth_credential_projects_to_bounded_redacted_summary() {
    let mut credential = ProviderOAuthCredential::new(
        "OpenAI",
        ProviderCredentialSourceKind::FirstPartyLogin,
        "chatgpt-account-1",
        "super-secret-access-token",
    );
    credential.account_id = Some("user-123".to_string());
    credential.endpoint = Some("https://chatgpt.com/backend-api/conversation".to_string());
    credential.client_id = Some("client-123".to_string());
    credential.token_endpoint = Some("https://auth.openai.com/oauth/token".to_string());
    credential.scopes = vec![
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ];
    credential.expires_at = Some(42);
    credential.provenance = Some("login-state".to_string());
    credential.refresh_token = Some("super-secret-refresh-token".to_string());

    let expected = ProviderCredentialRoutingSummary {
        provider_name: "OpenAI".to_string(),
        source_kind: ProviderCredentialSourceKind::FirstPartyLogin,
        auth_kind: ProviderCredentialAuthKind::OAuthBearer,
        has_account_id: true,
        has_endpoint: true,
        has_client_id: true,
        scope_count: 3,
        expires_at: Some(42),
        provenance: Some("login-state".to_string()),
    };

    let actual = credential.to_routing_summary();
    let debug = format!("{actual:?}");

    assert_eq!(actual, expected);
    assert!(!debug.contains("super-secret-access-token"));
    assert!(!debug.contains("super-secret-refresh-token"));
    assert!(!debug.contains("chatgpt-account-1"));
    assert!(!debug.contains("user-123"));
    assert!(!debug.contains("client-123"));
    assert!(!debug.contains("https://auth.openai.com/oauth/token"));
    assert!(!debug.contains("openid"));
}

#[test]
fn provider_oauth_credential_refresh_state_tracks_refresh_token_and_expiry() {
    let non_refreshable = ProviderOAuthCredential::new(
        "Claude",
        ProviderCredentialSourceKind::ExternalImport,
        "claude-connector-1",
        "secret-access-token",
    );
    assert_eq!(
        non_refreshable.refresh_state(),
        ProviderCredentialRefreshState::NonRefreshable
    );

    let expired_at = now_millis().saturating_sub(1_000);
    let mut expired_refreshable = ProviderOAuthCredential::new(
        "OpenAI",
        ProviderCredentialSourceKind::FirstPartyLogin,
        "chatgpt-account-1",
        "secret-access-token",
    );
    expired_refreshable.expires_at = Some(expired_at);
    expired_refreshable.refresh_token = Some("secret-refresh-token".to_string());
    assert_eq!(
        expired_refreshable.refresh_state(),
        ProviderCredentialRefreshState::RefreshEligible
    );

    let future_at = now_millis().saturating_add(Duration::from_secs(120).as_millis() as u64);
    let mut healthy_refreshable = ProviderOAuthCredential::new(
        "OpenAI",
        ProviderCredentialSourceKind::FirstPartyLogin,
        "chatgpt-account-2",
        "secret-access-token",
    );
    healthy_refreshable.expires_at = Some(future_at);
    healthy_refreshable.refresh_token = Some("secret-refresh-token".to_string());
    assert_eq!(
        healthy_refreshable.refresh_state(),
        ProviderCredentialRefreshState::RefreshHealthy
    );
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis() as u64
}
