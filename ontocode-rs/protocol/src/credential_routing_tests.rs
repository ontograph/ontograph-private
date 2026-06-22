use pretty_assertions::assert_eq;

use super::ProviderCredentialAuthKind;
use super::ProviderCredentialRoutingSummary;
use super::ProviderCredentialRoutingView;
use super::ProviderCredentialSourceKind;

#[test]
fn provider_credential_routing_view_defaults_optional_fields() {
    assert_eq!(
        ProviderCredentialRoutingView::new(
            "OpenAI",
            ProviderCredentialSourceKind::FirstPartyLogin,
            ProviderCredentialAuthKind::AccessToken,
            "openai-access-token",
        ),
        ProviderCredentialRoutingView {
            provider_name: "OpenAI".to_string(),
            source_kind: ProviderCredentialSourceKind::FirstPartyLogin,
            auth_kind: ProviderCredentialAuthKind::AccessToken,
            credential_id: "openai-access-token".to_string(),
            account_id: None,
            endpoint: None,
            client_id: None,
            scopes: Vec::new(),
            expires_at: None,
            provenance: None,
        }
    );
}

#[test]
fn provider_credential_routing_summary_redacts_optional_identifiers_to_booleans() {
    let mut view = ProviderCredentialRoutingView::new(
        "claude",
        ProviderCredentialSourceKind::ExternalImport,
        ProviderCredentialAuthKind::OAuthBearer,
        "claude:https://mcp.example.test",
    );
    view.account_id = Some("acct_123".to_string());
    view.endpoint = Some("https://mcp.example.test".to_string());
    view.client_id = Some("client-123".to_string());
    view.scopes = vec!["scope-a".to_string(), "scope-b".to_string()];
    view.expires_at = Some(1234);
    view.provenance = Some("claude-code".to_string());

    assert_eq!(
        view.to_summary(),
        ProviderCredentialRoutingSummary {
            provider_name: "claude".to_string(),
            source_kind: ProviderCredentialSourceKind::ExternalImport,
            auth_kind: ProviderCredentialAuthKind::OAuthBearer,
            has_account_id: true,
            has_endpoint: true,
            has_client_id: true,
            scope_count: 2,
            expires_at: Some(1234),
            provenance: Some("claude-code".to_string()),
        }
    );
}
