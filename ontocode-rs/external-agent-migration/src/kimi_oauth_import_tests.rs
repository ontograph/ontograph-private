use super::ImportableKimiOauthCredential;
use super::KimiOauthImportRejectionReason;
use super::KimiOauthImportReport;
use super::parse_kimi_oauth_import_sample;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderOAuthCredential;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::convert::TryFrom;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[test]
fn parses_valid_kimi_sample_into_provider_oauth_credential() {
    let report = parse_kimi_oauth_import_sample(
        &json!({
            "type": "kimi",
            "access_token": "kimi-access-token-redacted",
            "refresh_token": "kimi-refresh-token-redacted",
            "token_type": "Bearer",
            "scope": "chat:read chat:write",
            "expired": false,
            "expires_at": "3026-05-22T12:34:56Z",
            "device_id": "kimi-device-redacted"
        }),
        false,
    );

    let expected_expires_at = parse_rfc3339_millis("3026-05-22T12:34:56Z");
    let expected_credential = ImportableKimiOauthCredential {
        provider_name: "kimi".to_string(),
        device_id: Some("kimi-device-redacted".to_string()),
        token_type: Some("Bearer".to_string()),
        scopes: vec!["chat:read".to_string(), "chat:write".to_string()],
        expires_at: Some(expected_expires_at),
        access_token: "kimi-access-token-redacted".to_string(),
        refresh_token: "kimi-refresh-token-redacted".to_string(),
        expired: Some(false),
        provenance: "cli-proxy-api".to_string(),
    };
    let expected = KimiOauthImportReport {
        credential: Some(expected_credential.clone()),
        rejections: Vec::new(),
        dry_run: false,
    };

    assert_eq!(report, expected);

    let mut expected_oauth = ProviderOAuthCredential::new(
        "kimi",
        ProviderCredentialSourceKind::ExternalImport,
        "kimi:imported",
        "kimi-access-token-redacted",
    );
    expected_oauth.scopes = vec!["chat:read".to_string(), "chat:write".to_string()];
    expected_oauth.expires_at = Some(expected_expires_at);
    expected_oauth.provenance = Some("cli-proxy-api".to_string());
    expected_oauth.refresh_token = Some("kimi-refresh-token-redacted".to_string());

    let credential = report.credential.clone().expect("credential should import");
    assert_eq!(credential, expected_credential);
    let oauth = credential.to_provider_oauth_credential();
    assert_eq!(oauth, expected_oauth);

    let report_debug = format!("{report:?}");
    assert!(!report_debug.contains("kimi-access-token-redacted"));
    assert!(!report_debug.contains("kimi-refresh-token-redacted"));
    assert!(report_debug.contains("<redacted>"));

    let oauth_debug = format!("{oauth:?}");
    assert!(!oauth_debug.contains("kimi-access-token-redacted"));
    assert!(!oauth_debug.contains("kimi-refresh-token-redacted"));
    assert!(oauth_debug.contains("<redacted>"));
}

#[test]
fn kimi_import_rejects_missing_refresh_token() {
    let report = parse_kimi_oauth_import_sample(
        &json!({
            "type": "kimi",
            "access_token": "kimi-access-token-redacted",
            "token_type": "Bearer",
            "scope": "chat:read",
            "expired": false,
            "expires_at": "3026-05-22T12:34:56Z",
            "device_id": "kimi-device-redacted"
        }),
        false,
    );

    assert_eq!(
        report,
        KimiOauthImportReport {
            credential: None,
            rejections: vec![super::KimiOauthImportRejection {
                reason: KimiOauthImportRejectionReason::MissingRefreshToken,
            }],
            dry_run: false,
        }
    );

    let report_debug = format!("{report:?}");
    assert!(!report_debug.contains("kimi-access-token-redacted"));
    assert!(!report_debug.contains("kimi-device-redacted"));
}

#[test]
fn kimi_import_rejects_malformed_expiry() {
    let report = parse_kimi_oauth_import_sample(
        &json!({
            "type": "kimi",
            "access_token": "kimi-access-token-redacted",
            "refresh_token": "kimi-refresh-token-redacted",
            "token_type": "Bearer",
            "scope": "chat:read",
            "expired": false,
            "expires_at": "not-a-timestamp",
            "device_id": "kimi-device-redacted"
        }),
        false,
    );

    assert_eq!(
        report,
        KimiOauthImportReport {
            credential: None,
            rejections: vec![super::KimiOauthImportRejection {
                reason: KimiOauthImportRejectionReason::InvalidExpiry,
            }],
            dry_run: false,
        }
    );

    let report_debug = format!("{report:?}");
    assert!(!report_debug.contains("kimi-access-token-redacted"));
    assert!(!report_debug.contains("kimi-refresh-token-redacted"));
}

fn parse_rfc3339_millis(value: &str) -> u64 {
    let parsed = OffsetDateTime::parse(value, &Rfc3339).expect("valid RFC3339 timestamp");
    let millis = parsed.unix_timestamp_nanos().div_euclid(1_000_000);
    u64::try_from(millis).expect("timestamp should be after Unix epoch")
}
