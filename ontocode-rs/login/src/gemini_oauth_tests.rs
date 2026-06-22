use pretty_assertions::assert_eq;

use super::*;

fn valid_tokens() -> GeminiOAuthTokens {
    GeminiOAuthTokens {
        credential_id: "gemini-account-1".to_string(),
        access_token: "secret-access-token".to_string(),
        refresh_token: "secret-refresh-token".to_string(),
        client_id: "gemini-client-id".to_string(),
        scopes: vec!["https://www.googleapis.com/auth/cloud-platform".to_string()],
        expires_at: Some(1_725_000_000_000),
        account_id: Some("user@example.com".to_string()),
        project_id: Some("project-1".to_string()),
        provenance: GeminiOAuthProvenance::Browser,
    }
}

#[test]
fn gemini_oauth_tokens_project_to_provider_oauth_credential() {
    let credential = valid_tokens()
        .into_provider_oauth_credential()
        .expect("credential should project");

    assert_eq!(credential.provider_id(), GEMINI_PROVIDER_ID);
    assert_eq!(credential.credential_id, "gemini-account-1");
    assert_eq!(credential.access_token, "secret-access-token");
    assert_eq!(
        credential.refresh_token.as_deref(),
        Some("secret-refresh-token")
    );
    assert_eq!(credential.client_id.as_deref(), Some("gemini-client-id"));
    assert_eq!(
        credential.token_endpoint.as_deref(),
        Some(GEMINI_TOKEN_ENDPOINT)
    );
    assert_eq!(
        credential.scopes,
        vec!["https://www.googleapis.com/auth/cloud-platform".to_string()]
    );
    assert_eq!(credential.expires_at, Some(1_725_000_000_000));
    assert_eq!(credential.account_id.as_deref(), Some("user@example.com"));
    assert_eq!(credential.endpoint.as_deref(), Some("project-1"));
    assert_eq!(
        credential.provenance.as_deref(),
        Some("gemini-browser-oauth")
    );
}

#[test]
fn gemini_oauth_tokens_reject_missing_refresh_metadata() {
    let mut tokens = valid_tokens();
    tokens.refresh_token.clear();

    assert_eq!(
        tokens.into_provider_oauth_credential(),
        Err(GeminiOAuthCredentialError::MissingRefreshToken)
    );
}

#[test]
fn gemini_oauth_tokens_reject_missing_client_id() {
    let mut tokens = valid_tokens();
    tokens.client_id.clear();

    assert_eq!(
        tokens.into_provider_oauth_credential(),
        Err(GeminiOAuthCredentialError::MissingClientId)
    );
}

#[test]
fn gemini_oauth_tokens_debug_redacts_tokens() {
    let rendered = format!("{:?}", valid_tokens());

    assert!(!rendered.contains("secret-access-token"));
    assert!(!rendered.contains("secret-refresh-token"));
    assert!(rendered.contains("<redacted>"));
}

#[test]
fn gemini_oauth_provenance_uses_adr_values() {
    assert_eq!(
        GeminiOAuthProvenance::Browser.as_str(),
        "gemini-browser-oauth"
    );
    assert_eq!(
        GeminiOAuthProvenance::UserCode.as_str(),
        "gemini-user-code-oauth"
    );
    assert_eq!(
        GeminiOAuthProvenance::Device.as_str(),
        "gemini-device-oauth"
    );
}
