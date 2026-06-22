use super::ProviderAuthRedactedError;

#[test]
fn provider_auth_redacted_error_scrubs_sensitive_material() {
    let token = "token-123".to_string();
    let refresh_token = "refresh-token-456".to_string();
    let client_secret = "client-secret-789".to_string();
    let raw_path = "/tmp/raw/provider.json".to_string();
    let keychain_path = "/Users/alice/Library/Keychains/login.keychain-db".to_string();
    let cookie = "session=abc123".to_string();
    let authorization_header = "Bearer auth-header-xyz".to_string();

    let message = format!(
        "token={token}, refresh_token={refresh_token}, client_secret={client_secret}, raw_path={raw_path}, keychain_path={keychain_path}, cookie={cookie}, authorization={authorization_header}"
    );
    let redacted = ProviderAuthRedactedError::scrubbed(
        message,
        &[
            token.as_str(),
            refresh_token.as_str(),
            client_secret.as_str(),
            raw_path.as_str(),
            keychain_path.as_str(),
            cookie.as_str(),
            authorization_header.as_str(),
        ],
    );

    let debug = format!("{redacted:?}");
    let display = redacted.to_string();

    assert!(debug.contains("<redacted>"));
    assert!(display.contains("<redacted>"));
    for value in [
        token.as_str(),
        refresh_token.as_str(),
        client_secret.as_str(),
        raw_path.as_str(),
        keychain_path.as_str(),
        cookie.as_str(),
        authorization_header.as_str(),
    ] {
        assert!(!debug.contains(value));
        assert!(!display.contains(value));
    }
}
