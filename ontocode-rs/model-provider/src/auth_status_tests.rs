use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

use ontocode_login::AuthManager;
use ontocode_model_provider_info::GEMINI_CLI_PROVIDER_ID;
use ontocode_model_provider_info::GEMINI_PROVIDER_ID;
use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderAuthRedactedError;
use pretty_assertions::assert_eq;

use super::ProviderAuthStatusRow;
use super::ProviderAuthStatusRowBuilder;
use super::ProviderAuthStatusState;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn unique_temp_home() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "codex-provider-auth-status-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos()
    ))
}

async fn gemini_auth_manager() -> Arc<AuthManager> {
    let codex_home = unique_temp_home();
    std::fs::create_dir_all(&codex_home).expect("temp auth home should be created");

    let mut credential = ontocode_provider_auth::ProviderOAuthCredential::new(
        "gemini",
        ProviderCredentialSourceKind::ExternalImport,
        "gemini:workspace-1",
        "oauth-access-token",
    );
    credential.account_id = Some("workspace-1".to_string());

    ontocode_login::auth::upsert_provider_oauth_credential(
        codex_home.as_path(),
        ontocode_login::AuthCredentialsStoreMode::File,
        credential,
    )
    .expect("provider oauth credential should persist");

    AuthManager::shared(
        codex_home,
        /*enable_codex_api_key_env*/ false,
        ontocode_login::AuthCredentialsStoreMode::File,
        /*chatgpt_base_url*/ None,
    )
    .await
}

#[tokio::test]
async fn gemini_status_prefers_api_key_over_oauth_fallback() {
    let auth_manager = gemini_auth_manager().await;
    let _lock = env_lock().lock().expect("env lock");
    let key = "GEMINI_API_KEY";
    let previous = env::var(key).ok();
    unsafe {
        env::set_var(key, "gemini-test-key");
    }

    let row = ProviderAuthStatusRow::from_provider(
        GEMINI_PROVIDER_ID,
        &ModelProviderInfo::create_gemini_provider(),
        Some(auth_manager.as_ref()),
    );

    match previous {
        Some(value) => unsafe {
            env::set_var(key, value);
        },
        None => unsafe {
            env::remove_var(key);
        },
    }

    assert_eq!(
        row,
        ProviderAuthStatusRow {
            provider_id: GEMINI_PROVIDER_ID.to_string(),
            display_label: "Gemini".to_string(),
            state: ProviderAuthStatusState::ApiKeyActive,
            active_source_label: Some("API key".to_string()),
            redacted_account_label: None,
            disabled_reason: None,
        }
    );
}

#[tokio::test]
async fn gemini_status_ignores_oauth_fallback_when_api_key_is_missing() {
    let auth_manager = gemini_auth_manager().await;
    let _lock = env_lock().lock().expect("env lock");
    let key = "GEMINI_API_KEY";
    let previous = env::var(key).ok();
    unsafe {
        env::remove_var(key);
    }

    let row = ProviderAuthStatusRow::from_provider(
        GEMINI_PROVIDER_ID,
        &ModelProviderInfo::create_gemini_provider(),
        Some(auth_manager.as_ref()),
    );

    match previous {
        Some(value) => unsafe {
            env::set_var(key, value);
        },
        None => unsafe {
            env::remove_var(key);
        },
    }

    assert_eq!(
        row,
        ProviderAuthStatusRow {
            provider_id: GEMINI_PROVIDER_ID.to_string(),
            display_label: "Gemini".to_string(),
            state: ProviderAuthStatusState::NotConfigured,
            active_source_label: None,
            redacted_account_label: None,
            disabled_reason: Some("Gemini auth is not configured".to_string()),
        }
    );
}

#[test]
fn gemini_cli_status_is_blocked() {
    let row = ProviderAuthStatusRow::from_provider(
        GEMINI_CLI_PROVIDER_ID,
        &ModelProviderInfo::create_gemini_cli_provider(),
        None,
    );

    assert_eq!(
        row,
        ProviderAuthStatusRow {
            provider_id: GEMINI_CLI_PROVIDER_ID.to_string(),
            display_label: "Gemini CLI".to_string(),
            state: ProviderAuthStatusState::Blocked,
            active_source_label: None,
            redacted_account_label: None,
            disabled_reason: Some("Gemini CLI runtime is not available yet.".to_string()),
        }
    );
}

#[test]
fn builder_supports_ready_and_not_configured_states() {
    let ready = ProviderAuthStatusRowBuilder::new("custom", "Custom")
        .ready()
        .build();
    let not_configured = ProviderAuthStatusRowBuilder::new("custom", "Custom")
        .not_configured("Custom auth is not configured")
        .build();

    assert_eq!(
        ready,
        ProviderAuthStatusRow {
            provider_id: "custom".to_string(),
            display_label: "Custom".to_string(),
            state: ProviderAuthStatusState::Ready,
            active_source_label: None,
            redacted_account_label: None,
            disabled_reason: None,
        }
    );
    assert_eq!(
        not_configured,
        ProviderAuthStatusRow {
            provider_id: "custom".to_string(),
            display_label: "Custom".to_string(),
            state: ProviderAuthStatusState::NotConfigured,
            active_source_label: None,
            redacted_account_label: None,
            disabled_reason: Some("Custom auth is not configured".to_string()),
        }
    );
}

#[test]
fn error_redacted_state_hides_sensitive_material() {
    let token = "token-123".to_string();
    let refresh_token = "refresh-token-456".to_string();
    let client_secret = "client-secret-789".to_string();
    let raw_path = "/tmp/raw/provider.json".to_string();
    let keychain_path = "/Users/alice/Library/Keychains/login.keychain-db".to_string();
    let cookie = "session=abc123".to_string();
    let authorization_header = "Bearer auth-header-xyz".to_string();

    let redacted = ProviderAuthRedactedError::scrubbed(
        format!(
            "token={token}, refresh_token={refresh_token}, client_secret={client_secret}, raw_path={raw_path}, keychain_path={keychain_path}, cookie={cookie}, authorization={authorization_header}"
        ),
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

    let row = ProviderAuthStatusRowBuilder::new("gemini", "Gemini")
        .error_redacted(redacted)
        .build();
    let rendered = format!("{row:?}");

    for value in [
        token.as_str(),
        refresh_token.as_str(),
        client_secret.as_str(),
        raw_path.as_str(),
        keychain_path.as_str(),
        cookie.as_str(),
        authorization_header.as_str(),
    ] {
        assert!(!rendered.contains(value));
    }
    assert!(rendered.contains("<redacted>"));
    assert_eq!(row.state, ProviderAuthStatusState::ErrorRedacted);
}
