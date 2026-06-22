use super::GeminiOauthImportRejectionReason;
use super::GeminiOauthImportReport;
use super::parse_antigravity_oauth_import_sample;
use super::parse_gemini_adc_oauth_import_sample;
use super::parse_gemini_oauth_import_sample;
use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingView;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderCredentialRefreshState;
use ontocode_provider_auth::ProviderOAuthCredential;
use ontocode_provider_auth::ProviderOAuthCredentialSource;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::convert::TryFrom;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

fn antigravity_cloud_code_assist_request_fixture() -> serde_json::Value {
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
}

#[test]
fn parses_valid_donor_sample_into_provider_oauth_credential() {
    let report = parse_gemini_oauth_import_sample(
        &json!({
            "email": "alice@example.com",
            "project_id": "demo-project",
            "token": {
                "access_token": "access-secret",
                "refresh_token": "refresh-secret",
                "expiry": "3026-05-22T12:34:56Z",
                "token_uri": "https://oauth2.googleapis.com/token",
                "client_id": "client-id",
                "scopes": [
                    "https://www.googleapis.com/auth/cloud-platform",
                    "https://www.googleapis.com/auth/userinfo.email"
                ]
            },
            "client_secret": "client-secret",
            "keychain_path": "/Users/alice/Library/Keychains/login.keychain-db"
        }),
        false,
    );

    let credential = report.credential.expect("credential should import");
    assert_eq!(report.rejections, Vec::new());
    assert_eq!(credential.provider_name, "gemini-cli");

    let expected_expires_at = parse_rfc3339_millis("3026-05-22T12:34:56Z");

    let mut expected = ProviderOAuthCredential::new(
        "gemini-cli",
        ProviderCredentialSourceKind::ExternalImport,
        "gemini-cli:demo-project",
        "access-secret",
    );
    expected.account_id = Some("alice@example.com".to_string());
    expected.endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected.client_id = Some("client-id".to_string());
    expected.token_endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected.scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform".to_string(),
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ];
    expected.expires_at = Some(expected_expires_at);
    expected.provenance = Some("cli-proxy-api".to_string());
    expected.refresh_token = Some("refresh-secret".to_string());

    let mut expected_view = ProviderCredentialRoutingView::new(
        "gemini-cli".to_string(),
        ProviderCredentialSourceKind::ExternalImport,
        ProviderCredentialAuthKind::OAuthBearer,
        "gemini-cli:demo-project".to_string(),
    );
    expected_view.account_id = Some("alice@example.com".to_string());
    expected_view.endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected_view.client_id = Some("client-id".to_string());
    expected_view.scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform".to_string(),
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ];
    expected_view.expires_at = Some(expected_expires_at);
    expected_view.provenance = Some("cli-proxy-api".to_string());

    let source = credential.to_provider_oauth_credential_source();
    assert_eq!(credential.to_provider_oauth_credential(), expected);
    assert_eq!(source.current_credential(), expected);
    assert_eq!(source.current_routing_summary(), expected_view.to_summary());

    let source_debug = format!("{source:?}");
    assert!(!source_debug.contains("access-secret"));
    assert!(!source_debug.contains("refresh-secret"));
    assert!(source_debug.contains("<redacted>"));

    let refresh_descriptor = source.current_refresh_descriptor();
    let refresh_descriptor_debug = format!("{refresh_descriptor:?}");
    let summary_debug = format!("{:?}", source.current_routing_summary());
    assert!(!refresh_descriptor_debug.contains("access-secret"));
    assert!(!refresh_descriptor_debug.contains("refresh-secret"));
    assert!(!summary_debug.contains("access-secret"));
    assert!(!summary_debug.contains("refresh-secret"));

    assert_eq!(
        credential.to_provider_credential_routing_view(),
        expected_view
    );

    let oauth_debug = format!("{:?}", credential.to_provider_oauth_credential());
    assert!(!oauth_debug.contains("access-secret"));
    assert!(!oauth_debug.contains("refresh-secret"));
    assert!(oauth_debug.contains("<redacted>"));

    let view_debug = format!("{:?}", credential.to_provider_credential_routing_view());
    assert!(!view_debug.contains("access-secret"));
    assert!(!view_debug.contains("refresh-secret"));
}

#[test]
fn parses_google_adc_sample_into_gemini_provider_oauth_credential() {
    let report = parse_gemini_adc_oauth_import_sample(
        &json!({
            "client_id": "client-id",
            "client_secret": "client-secret",
            "refresh_token": "refresh-secret",
            "access_token": "access-secret",
            "expiry": "3026-05-22T12:34:56Z",
            "token_uri": "https://oauth2.googleapis.com/token",
            "quota_project_id": "demo-project",
            "scopes": "https://www.googleapis.com/auth/cloud-platform https://www.googleapis.com/auth/userinfo.email"
        }),
        false,
    );

    let credential = report.credential.expect("credential should import");
    assert_eq!(report.rejections, Vec::new());
    assert_eq!(credential.provider_name, "gemini");

    let expected_expires_at = parse_rfc3339_millis("3026-05-22T12:34:56Z");

    let mut expected = ProviderOAuthCredential::new(
        "gemini",
        ProviderCredentialSourceKind::ExternalImport,
        "gemini:demo-project",
        "access-secret",
    );
    expected.endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected.client_id = Some("client-id".to_string());
    expected.token_endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected.scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform".to_string(),
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ];
    expected.expires_at = Some(expected_expires_at);
    expected.provenance = Some("google-oauth-import".to_string());
    expected.refresh_token = Some("refresh-secret".to_string());

    let mut expected_view = ProviderCredentialRoutingView::new(
        "gemini".to_string(),
        ProviderCredentialSourceKind::ExternalImport,
        ProviderCredentialAuthKind::OAuthBearer,
        "gemini:demo-project".to_string(),
    );
    expected_view.endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected_view.client_id = Some("client-id".to_string());
    expected_view.scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform".to_string(),
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ];
    expected_view.expires_at = Some(expected_expires_at);
    expected_view.provenance = Some("google-oauth-import".to_string());

    let source = credential.to_provider_oauth_credential_source();
    assert_eq!(credential.to_provider_oauth_credential(), expected);
    assert_eq!(source.current_credential(), expected);
    assert_eq!(source.current_routing_summary(), expected_view.to_summary());
    assert_eq!(
        credential.to_provider_credential_routing_view(),
        expected_view
    );

    let oauth_debug = format!("{:?}", credential.to_provider_oauth_credential());
    assert!(!oauth_debug.contains("access-secret"));
    assert!(!oauth_debug.contains("refresh-secret"));
    assert!(!oauth_debug.contains("client-secret"));
    assert!(oauth_debug.contains("<redacted>"));
}

#[test]
fn parses_gemini_cli_token_file_without_client_id() {
    let report = parse_gemini_adc_oauth_import_sample(
        &json!({
            "access_token": "access-secret",
            "refresh_token": "refresh-secret",
            "token_type": "Bearer",
            "scope": "https://www.googleapis.com/auth/cloud-platform https://www.googleapis.com/auth/userinfo.email",
            "expiry_date": 3333333333000_u64
        }),
        false,
    );

    let credential = report.credential.expect("credential should import");
    assert_eq!(report.rejections, Vec::new());
    assert_eq!(credential.provider_name, "gemini");
    assert_eq!(
        credential.client_id,
        "681255809395-oo8ft2oprdrnp9e3aqf6av3hmdib135j.apps.googleusercontent.com"
    );
    assert_eq!(
        credential.token_endpoint,
        Some("https://oauth2.googleapis.com/token".to_string())
    );
    assert_eq!(
        credential.scopes,
        vec![
            "https://www.googleapis.com/auth/cloud-platform".to_string(),
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ]
    );
    assert_eq!(credential.expires_at, Some(3333333333000));
}

#[test]
fn parses_antigravity_sample_into_provider_oauth_credential() {
    let report = parse_antigravity_oauth_import_sample(
        &json!({
            "provider": "antigravity",
            "email": "alice@example.com",
            "project_id": "antigravity-project",
            "token": {
                "access_token": "access-secret",
                "refresh_token": "refresh-secret",
                "expiry": "3026-05-22T12:34:56Z",
                "token_uri": "https://oauth2.googleapis.com/token",
                "client_id": "client-id",
                "scopes": [
                    "https://www.googleapis.com/auth/cloud-platform",
                    "https://www.googleapis.com/auth/userinfo.email"
                ]
            },
            "client_secret": "client-secret",
            "keychain_path": "/Users/alice/Library/Keychains/login.keychain-db"
        }),
        false,
    );

    let credential = report.credential.clone().expect("credential should import");
    assert_eq!(report.rejections, Vec::new());
    assert_eq!(credential.provider_name, "antigravity");

    let expected_expires_at = parse_rfc3339_millis("3026-05-22T12:34:56Z");

    let mut expected = ProviderOAuthCredential::new(
        "antigravity",
        ProviderCredentialSourceKind::ExternalImport,
        "antigravity:antigravity-project",
        "access-secret",
    );
    expected.account_id = Some("alice@example.com".to_string());
    expected.endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected.client_id = Some("client-id".to_string());
    expected.token_endpoint = Some("https://oauth2.googleapis.com/token".to_string());
    expected.scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform".to_string(),
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
    ];
    expected.expires_at = Some(expected_expires_at);
    expected.provenance = Some("cliproxyapi-antigravity-import".to_string());
    expected.refresh_token = Some("refresh-secret".to_string());

    assert_eq!(credential.to_provider_oauth_credential(), expected);

    let debug_output = format!("{report:?}");
    assert!(!debug_output.contains("access-secret"));
    assert!(!debug_output.contains("refresh-secret"));
    assert!(!debug_output.contains("client-secret"));
    assert!(!debug_output.contains("/Users/alice/Library/Keychains/login.keychain-db"));
    assert!(debug_output.contains("<redacted>"));
}

#[test]
fn antigravity_import_rejects_missing_refresh_token() {
    let report = parse_antigravity_oauth_import_sample(
        &json!({
            "provider": "antigravity",
            "email": "alice@example.com",
            "project_id": "antigravity-project",
            "token": {
                "access_token": "access-secret",
                "expiry": "3026-05-22T12:34:56Z",
                "token_uri": "https://oauth2.googleapis.com/token",
                "client_id": "client-id",
                "scopes": ["https://www.googleapis.com/auth/cloud-platform"]
            },
            "client_secret": "client-secret"
        }),
        false,
    );

    assert_eq!(report.credential, None);
    assert_eq!(
        report.rejections,
        vec![super::GeminiOauthImportRejection {
            reason: GeminiOauthImportRejectionReason::MissingRefreshToken,
        }]
    );

    let debug_output = format!("{report:?}");
    assert!(!debug_output.contains("access-secret"));
    assert!(!debug_output.contains("client-secret"));
}

#[test]
fn missing_refresh_token_is_accepted_as_non_refreshable() {
    let report = parse_gemini_oauth_import_sample(
        &json!({
            "email": "alice@example.com",
            "project_id": "demo-project",
            "token": {
                "access_token": "access-secret",
                "token_uri": "https://oauth2.googleapis.com/token",
                "client_id": "client-id",
                "scopes": ["https://www.googleapis.com/auth/cloud-platform"]
            }
        }),
        false,
    );

    let credential = report.credential.expect("credential should import");
    assert_eq!(report.rejections, Vec::new());
    assert_eq!(credential.refresh_token, None);
    assert_eq!(credential.provider_name, "gemini-cli");

    let oauth = credential.to_provider_oauth_credential();
    let source = credential.to_provider_oauth_credential_source();
    assert!(!oauth.is_refreshable());
    assert_eq!(oauth.refresh_token, None);
    assert_eq!(
        source.current_refresh_descriptor().state,
        ProviderCredentialRefreshState::NonRefreshable
    );
}

#[test]
fn malformed_token_shape_returns_redacted_rejection() {
    let report = parse_gemini_oauth_import_sample(
        &json!({
            "email": "alice@example.com",
            "project_id": "demo-project",
            "token": {
                "access_token": 123,
                "refresh_token": "refresh-secret",
                "client_id": "client-id",
                "scopes": "cloud-platform"
            },
            "client_secret": "client-secret",
            "keychain_path": "/Users/alice/Library/Keychains/login.keychain-db"
        }),
        false,
    );

    assert_eq!(report.credential, None);
    assert_eq!(
        report.rejections,
        vec![super::GeminiOauthImportRejection {
            reason: GeminiOauthImportRejectionReason::InvalidTokenShape,
        }]
    );

    let debug_output = format!("{report:?}");
    assert!(!debug_output.contains("access-secret"));
    assert!(!debug_output.contains("refresh-secret"));
    assert!(!debug_output.contains("client-secret"));
    assert!(!debug_output.contains("/Users/alice/Library/Keychains/login.keychain-db"));
}

#[test]
fn missing_access_token_is_rejected_for_adc_import() {
    let report = parse_gemini_adc_oauth_import_sample(
        &json!({
            "client_id": "client-id",
            "client_secret": "client-secret",
            "refresh_token": "refresh-secret",
            "token_uri": "https://oauth2.googleapis.com/token",
            "quota_project_id": "demo-project"
        }),
        false,
    );

    assert_eq!(report.credential, None);
    assert_eq!(
        report.rejections,
        vec![super::GeminiOauthImportRejection {
            reason: GeminiOauthImportRejectionReason::MissingAccessToken,
        }]
    );

    let debug_output = format!("{report:?}");
    assert!(!debug_output.contains("refresh-secret"));
    assert!(!debug_output.contains("client-secret"));
    assert!(!debug_output.contains("/Users/alice/Library/Keychains/login.keychain-db"));
}

#[test]
fn debug_output_redacts_imported_secret_material() {
    let report = parse_gemini_oauth_import_sample(
        &json!({
            "email": "alice@example.com",
            "project_id": "demo-project",
            "token": {
                "access_token": "access-secret",
                "refresh_token": "refresh-secret",
                "expiry": "3026-05-22T12:34:56Z",
                "client_id": "client-id",
                "scopes": ["scope-a"]
            },
            "client_secret": "client-secret",
            "keychain_path": "/Users/alice/Library/Keychains/login.keychain-db"
        }),
        true,
    );

    let credential = report.credential.expect("credential should import");
    let oauth = credential.to_provider_oauth_credential();

    let report_debug = format!(
        "{:?}",
        GeminiOauthImportReport {
            credential: Some(credential),
            rejections: vec![],
            dry_run: true,
        }
    );
    assert!(!report_debug.contains("access-secret"));
    assert!(!report_debug.contains("refresh-secret"));
    assert!(!report_debug.contains("client-secret"));
    assert!(!report_debug.contains("/Users/alice/Library/Keychains/login.keychain-db"));
    assert!(report_debug.contains("<redacted>"));

    let oauth_debug = format!("{oauth:?}");
    assert!(!oauth_debug.contains("access-secret"));
    assert!(!oauth_debug.contains("refresh-secret"));
    assert!(oauth_debug.contains("<redacted>"));
}

#[test]
fn antigravity_cloud_code_assist_fixture_preserves_donor_metadata_contract() {
    let fixture = antigravity_cloud_code_assist_request_fixture();

    assert_eq!(
        fixture,
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
}

fn parse_rfc3339_millis(value: &str) -> u64 {
    let parsed = OffsetDateTime::parse(value, &Rfc3339).expect("valid RFC3339 timestamp");
    let millis = parsed.unix_timestamp_nanos().div_euclid(1_000_000);
    u64::try_from(millis).expect("timestamp should be after Unix epoch")
}
