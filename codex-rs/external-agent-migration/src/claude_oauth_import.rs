use serde_json::Value as JsonValue;
use std::collections::HashSet;
use std::fmt;

const KIND_MCP_OAUTH: &str = "mcp_oauth";
const EXPIRES_AT_UNIT_MILLISECONDS: &str = "milliseconds_unix_epoch";

/// MCP OAuth credential data that is safe to hand to Codex OAuth storage.
#[derive(Clone, PartialEq, Eq)]
pub struct ImportableMcpOAuthCredential {
    pub connector_name: String,
    pub server_url: String,
    pub client_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub provenance: String,
}

impl fmt::Debug for ImportableMcpOAuthCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportableMcpOAuthCredential")
            .field("connector_name", &self.connector_name)
            .field("server_url", &self.server_url)
            .field("client_id", &self.client_id)
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .field("scopes", &self.scopes)
            .field("expires_at", &self.expires_at)
            .field("provenance", &self.provenance)
            .finish()
    }
}

/// Parse result for a Claude OAuth credential export sample.
#[derive(Clone, PartialEq, Eq)]
pub struct ClaudeOauthImportReport {
    pub credentials: Vec<ImportableMcpOAuthCredential>,
    pub rejections: Vec<ClaudeOauthImportRejection>,
    pub dry_run: bool,
}

impl fmt::Debug for ClaudeOauthImportReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClaudeOauthImportReport")
            .field("credentials", &self.credentials)
            .field("rejections", &self.rejections)
            .field("dry_run", &self.dry_run)
            .finish()
    }
}

impl ClaudeOauthImportReport {
    pub fn status(&self) -> ClaudeOauthImportStatus {
        match (self.credentials.is_empty(), self.rejections.is_empty()) {
            (false, true) => ClaudeOauthImportStatus::Complete,
            (false, false) => ClaudeOauthImportStatus::Partial,
            (true, false) => {
                if self
                    .rejections
                    .iter()
                    .any(|r| r.reason == ClaudeOauthImportRejectionReason::LockedKeychain)
                {
                    ClaudeOauthImportStatus::LockedKeychain
                } else if self
                    .rejections
                    .iter()
                    .any(|r| r.reason == ClaudeOauthImportRejectionReason::ConsentRequired)
                {
                    ClaudeOauthImportStatus::ConsentRequired
                } else {
                    ClaudeOauthImportStatus::NonImportable
                }
            }
            (true, true) => ClaudeOauthImportStatus::Empty,
        }
    }
}

/// High-level outcome for a Claude OAuth import attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaudeOauthImportStatus {
    Complete,
    Partial,
    NonImportable,
    Empty,
    ConsentRequired,
    LockedKeychain,
}

/// A credential record that was not safe to import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaudeOauthImportRejection {
    pub index: Option<usize>,
    pub reason: ClaudeOauthImportRejectionReason,
}

/// Reason a Claude OAuth credential record was not importable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaudeOauthImportRejectionReason {
    MissingCredentialsArray,
    UnsupportedKind,
    MissingConnectorName,
    MissingServerUrl,
    InvalidServerUrl,
    MissingClientId,
    MissingAccessToken,
    InvalidScopes,
    InvalidExpiresAt,
    UnknownExpiresAtUnit,
    DuplicateCredentialKey,
    ConsentRequired,
    LockedKeychain,
}

/// Parses the sanitized fixture contract defined in `.memory-bank/CLAUDE_OAUTH_IMPORT_SAMPLE_CONTRACT.md`.
pub fn parse_claude_oauth_import_sample(
    value: &JsonValue,
    dry_run: bool,
    user_consented: bool,
) -> ClaudeOauthImportReport {
    if !user_consented {
        return ClaudeOauthImportReport {
            credentials: Vec::new(),
            rejections: vec![ClaudeOauthImportRejection {
                index: None,
                reason: ClaudeOauthImportRejectionReason::ConsentRequired,
            }],
            dry_run,
        };
    }

    let Some(records) = value.get("credentials").and_then(JsonValue::as_array) else {
        return ClaudeOauthImportReport {
            credentials: Vec::new(),
            rejections: vec![ClaudeOauthImportRejection {
                index: None,
                reason: ClaudeOauthImportRejectionReason::MissingCredentialsArray,
            }],
            dry_run,
        };
    };

    let mut credentials = Vec::new();
    let mut rejections = Vec::new();
    let mut seen_keys = HashSet::new();

    for (index, record) in records.iter().enumerate() {
        match parse_credential(record) {
            Ok(credential) => {
                let key = (
                    credential.connector_name.clone(),
                    credential.server_url.clone(),
                );
                if !seen_keys.insert(key) {
                    rejections.push(ClaudeOauthImportRejection {
                        index: Some(index),
                        reason: ClaudeOauthImportRejectionReason::DuplicateCredentialKey,
                    });
                    continue;
                }
                credentials.push(credential);
            }
            Err(reason) => rejections.push(ClaudeOauthImportRejection {
                index: Some(index),
                reason,
            }),
        }
    }

    ClaudeOauthImportReport {
        credentials,
        rejections,
        dry_run,
    }
}

fn parse_credential(
    record: &JsonValue,
) -> Result<ImportableMcpOAuthCredential, ClaudeOauthImportRejectionReason> {
    if optional_string(record, "kind").as_deref() != Some(KIND_MCP_OAUTH) {
        return Err(ClaudeOauthImportRejectionReason::UnsupportedKind);
    }

    let connector_name = required_string(
        record,
        "connector_name",
        ClaudeOauthImportRejectionReason::MissingConnectorName,
    )?;
    let server_url = required_string(
        record,
        "server_url",
        ClaudeOauthImportRejectionReason::MissingServerUrl,
    )?;
    if !has_supported_url_shape(&server_url) {
        return Err(ClaudeOauthImportRejectionReason::InvalidServerUrl);
    }

    let client_id = required_string(
        record,
        "client_id",
        ClaudeOauthImportRejectionReason::MissingClientId,
    )?;
    let access_token = required_string(
        record,
        "access_token",
        ClaudeOauthImportRejectionReason::MissingAccessToken,
    )?;
    let refresh_token = optional_string(record, "refresh_token");
    let scopes = parse_scopes(record)?;
    let expires_at = parse_expires_at(record)?;
    let provenance =
        optional_string(record, "provenance").unwrap_or_else(|| "claude-code".to_string());

    Ok(ImportableMcpOAuthCredential {
        connector_name,
        server_url,
        client_id,
        access_token,
        refresh_token,
        scopes,
        expires_at,
        provenance,
    })
}

fn required_string(
    record: &JsonValue,
    key: &str,
    reason: ClaudeOauthImportRejectionReason,
) -> Result<String, ClaudeOauthImportRejectionReason> {
    optional_string(record, key).ok_or(reason)
}

fn optional_string(record: &JsonValue, key: &str) -> Option<String> {
    record
        .get(key)
        .and_then(JsonValue::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn parse_scopes(record: &JsonValue) -> Result<Vec<String>, ClaudeOauthImportRejectionReason> {
    let Some(scopes) = record.get("scopes") else {
        return Ok(Vec::new());
    };
    let Some(scopes) = scopes.as_array() else {
        return Err(ClaudeOauthImportRejectionReason::InvalidScopes);
    };

    let mut parsed = Vec::with_capacity(scopes.len());
    for scope in scopes {
        let Some(scope) = scope
            .as_str()
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
        else {
            return Err(ClaudeOauthImportRejectionReason::InvalidScopes);
        };
        parsed.push(scope.to_string());
    }
    Ok(parsed)
}

fn parse_expires_at(record: &JsonValue) -> Result<Option<u64>, ClaudeOauthImportRejectionReason> {
    let Some(expires_at) = record.get("expires_at") else {
        return Ok(None);
    };

    if optional_string(record, "expires_at_unit").as_deref() != Some(EXPIRES_AT_UNIT_MILLISECONDS) {
        return Err(ClaudeOauthImportRejectionReason::UnknownExpiresAtUnit);
    }

    expires_at
        .as_u64()
        .map(Some)
        .ok_or(ClaudeOauthImportRejectionReason::InvalidExpiresAt)
}

fn has_supported_url_shape(server_url: &str) -> bool {
    let Some(rest) = server_url
        .strip_prefix("https://")
        .or_else(|| server_url.strip_prefix("http://"))
    else {
        return false;
    };

    !rest.is_empty() && !rest.starts_with('/') && !rest.chars().any(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn parses_importable_mcp_oauth_credentials() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted",
                        "refresh_token": "refresh-redacted",
                        "scopes": ["read", "write"],
                        "expires_at": 1893456000000u64,
                        "expires_at_unit": "milliseconds_unix_epoch",
                        "provenance": "claude-code"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: vec![ImportableMcpOAuthCredential {
                    connector_name: "linear".to_string(),
                    server_url: "https://mcp.linear.app/sse".to_string(),
                    client_id: "client-redacted".to_string(),
                    access_token: "access-redacted".to_string(),
                    refresh_token: Some("refresh-redacted".to_string()),
                    scopes: vec!["read".to_string(), "write".to_string()],
                    expires_at: Some(1893456000000),
                    provenance: "claude-code".to_string(),
                }],
                rejections: Vec::new(),
                dry_run: false,
            }
        );
    }

    #[test]
    fn requires_user_consent_before_parsing() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted"
                    }
                ]
            }),
            false,
            false,
        );

        assert_eq!(report.status(), ClaudeOauthImportStatus::ConsentRequired);
        assert!(report.credentials.is_empty());
    }

    #[test]
    fn reports_locked_keychain_status() {
        let report = ClaudeOauthImportReport {
            credentials: Vec::new(),
            rejections: vec![ClaudeOauthImportRejection {
                index: None,
                reason: ClaudeOauthImportRejectionReason::LockedKeychain,
            }],
            dry_run: false,
        };
        assert_eq!(report.status(), ClaudeOauthImportStatus::LockedKeychain);
    }

    #[test]
    fn rejects_missing_required_connector_fields() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted"
                    },
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "notion",
                        "server_url": "https://mcp.notion.com",
                        "access_token": "access-redacted"
                    },
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "github",
                        "server_url": "https://mcp.github.com",
                        "client_id": "client-redacted"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: vec![
                    ClaudeOauthImportRejection {
                        index: Some(0),
                        reason: ClaudeOauthImportRejectionReason::MissingServerUrl,
                    },
                    ClaudeOauthImportRejection {
                        index: Some(1),
                        reason: ClaudeOauthImportRejectionReason::MissingClientId,
                    },
                    ClaudeOauthImportRejection {
                        index: Some(2),
                        reason: ClaudeOauthImportRejectionReason::MissingAccessToken,
                    },
                ],
                dry_run: false,
            }
        );
    }

    #[test]
    fn rejects_non_mcp_oauth_records() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "account_oauth",
                        "account_id": "account-redacted",
                        "access_token": "access-redacted"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: vec![ClaudeOauthImportRejection {
                    index: Some(0),
                    reason: ClaudeOauthImportRejectionReason::UnsupportedKind,
                }],
                dry_run: false,
            }
        );
    }

    #[test]
    fn debug_output_redacts_importable_oauth_tokens() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "client-id",
                        "access_token": "super-secret-access-token",
                        "refresh_token": "super-secret-refresh-token"
                    }
                ]
            }),
            false,
            true,
        );

        let debug_output = format!("{report:?}");
        assert!(debug_output.contains("<redacted>"));
        assert!(!debug_output.contains("super-secret-access-token"));
        assert!(!debug_output.contains("super-secret-refresh-token"));
    }

    #[test]
    fn rejects_invalid_structural_fields() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "mcp.linear.app/sse",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted"
                    },
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "notion",
                        "server_url": "https://mcp.notion.com",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted",
                        "scopes": "read"
                    },
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "github",
                        "server_url": "https://mcp.github.com",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted",
                        "expires_at": 1893456000,
                        "expires_at_unit": "seconds_unix_epoch"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: vec![
                    ClaudeOauthImportRejection {
                        index: Some(0),
                        reason: ClaudeOauthImportRejectionReason::InvalidServerUrl,
                    },
                    ClaudeOauthImportRejection {
                        index: Some(1),
                        reason: ClaudeOauthImportRejectionReason::InvalidScopes,
                    },
                    ClaudeOauthImportRejection {
                        index: Some(2),
                        reason: ClaudeOauthImportRejectionReason::UnknownExpiresAtUnit,
                    },
                ],
                dry_run: false,
            }
        );
    }

    #[test]
    fn rejects_invalid_expires_at_value_with_known_unit() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted",
                        "expires_at": "1893456000000",
                        "expires_at_unit": "milliseconds_unix_epoch"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: vec![ClaudeOauthImportRejection {
                    index: Some(0),
                    reason: ClaudeOauthImportRejectionReason::InvalidExpiresAt,
                }],
                dry_run: false,
            }
        );
    }

    #[test]
    fn reports_partial_status_for_mixed_importable_and_non_importable_records() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted"
                    },
                    {
                        "kind": "account_oauth",
                        "account_id": "account-redacted",
                        "access_token": "access-redacted"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(report.status(), ClaudeOauthImportStatus::Partial);
        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: vec![ImportableMcpOAuthCredential {
                    connector_name: "linear".to_string(),
                    server_url: "https://mcp.linear.app/sse".to_string(),
                    client_id: "client-redacted".to_string(),
                    access_token: "access-redacted".to_string(),
                    refresh_token: None,
                    scopes: Vec::new(),
                    expires_at: None,
                    provenance: "claude-code".to_string(),
                }],
                rejections: vec![ClaudeOauthImportRejection {
                    index: Some(1),
                    reason: ClaudeOauthImportRejectionReason::UnsupportedKind,
                }],
                dry_run: false,
            }
        );
    }

    #[test]
    fn rejects_duplicate_connector_server_keys() {
        let report = parse_claude_oauth_import_sample(
            &json!({
                "credentials": [
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "client-redacted",
                        "access_token": "access-redacted"
                    },
                    {
                        "kind": "mcp_oauth",
                        "connector_name": "linear",
                        "server_url": "https://mcp.linear.app/sse",
                        "client_id": "other-client-redacted",
                        "access_token": "other-access-redacted"
                    }
                ]
            }),
            false,
            true,
        );

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: vec![ImportableMcpOAuthCredential {
                    connector_name: "linear".to_string(),
                    server_url: "https://mcp.linear.app/sse".to_string(),
                    client_id: "client-redacted".to_string(),
                    access_token: "access-redacted".to_string(),
                    refresh_token: None,
                    scopes: Vec::new(),
                    expires_at: None,
                    provenance: "claude-code".to_string(),
                }],
                rejections: vec![ClaudeOauthImportRejection {
                    index: Some(1),
                    reason: ClaudeOauthImportRejectionReason::DuplicateCredentialKey,
                }],
                dry_run: false,
            }
        );
    }

    #[test]
    fn rejects_missing_credentials_array() {
        let report = parse_claude_oauth_import_sample(&json!({}), false, true);

        assert_eq!(
            report,
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: vec![ClaudeOauthImportRejection {
                    index: None,
                    reason: ClaudeOauthImportRejectionReason::MissingCredentialsArray,
                }],
                dry_run: false,
            }
        );
    }

    #[test]
    fn classifies_import_status() {
        assert_eq!(
            ClaudeOauthImportReport {
                credentials: vec![ImportableMcpOAuthCredential {
                    connector_name: "linear".to_string(),
                    server_url: "https://mcp.linear.app/sse".to_string(),
                    client_id: "client-redacted".to_string(),
                    access_token: "access-redacted".to_string(),
                    refresh_token: None,
                    scopes: Vec::new(),
                    expires_at: None,
                    provenance: "claude-code".to_string(),
                }],
                rejections: Vec::new(),
                dry_run: false,
            }
            .status(),
            ClaudeOauthImportStatus::Complete
        );
        assert_eq!(
            ClaudeOauthImportReport {
                credentials: vec![ImportableMcpOAuthCredential {
                    connector_name: "linear".to_string(),
                    server_url: "https://mcp.linear.app/sse".to_string(),
                    client_id: "client-redacted".to_string(),
                    access_token: "access-redacted".to_string(),
                    refresh_token: None,
                    scopes: Vec::new(),
                    expires_at: None,
                    provenance: "claude-code".to_string(),
                }],
                rejections: vec![ClaudeOauthImportRejection {
                    index: Some(1),
                    reason: ClaudeOauthImportRejectionReason::UnsupportedKind,
                }],
                dry_run: false,
            }
            .status(),
            ClaudeOauthImportStatus::Partial
        );
        assert_eq!(
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: vec![ClaudeOauthImportRejection {
                    index: Some(0),
                    reason: ClaudeOauthImportRejectionReason::UnsupportedKind,
                }],
                dry_run: false,
            }
            .status(),
            ClaudeOauthImportStatus::NonImportable
        );
        assert_eq!(
            ClaudeOauthImportReport {
                credentials: Vec::new(),
                rejections: Vec::new(),
                dry_run: false,
            }
            .status(),
            ClaudeOauthImportStatus::Empty
        );
    }

    #[test]
    #[ignore = "requires CLAUDE_OAUTH_REDACTED_SAMPLE pointing to a sanitized real Claude sample"]
    fn validates_redacted_live_sample_from_env() {
        let path = std::env::var_os("CLAUDE_OAUTH_REDACTED_SAMPLE")
            .expect("CLAUDE_OAUTH_REDACTED_SAMPLE must point to a redacted sample JSON file");
        let sample = std::fs::read_to_string(&path)
            .expect("redacted Claude OAuth sample should be readable");
        let value: JsonValue =
            serde_json::from_str(&sample).expect("redacted Claude OAuth sample must be valid JSON");
        let report = parse_claude_oauth_import_sample(&value, false, true);

        eprintln!(
            "status={:?} importable_credentials={} rejections={:?} refreshable_credentials={}",
            report.status(),
            report.credentials.len(),
            report.rejections,
            report
                .credentials
                .iter()
                .filter(|credential| credential.refresh_token.is_some())
                .count()
        );

        assert!(
            matches!(
                report.status(),
                ClaudeOauthImportStatus::Complete | ClaudeOauthImportStatus::Partial
            ),
            "redacted sample did not contain importable MCP OAuth credentials: {report:?}"
        );
    }
}
