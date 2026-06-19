use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingView;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderOAuthCredential;
use ontocode_provider_auth::StaticProviderOAuthCredentialSource;
use serde_json::Value as JsonValue;
use std::convert::TryFrom;
use std::fmt;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

const KIMI_PROVIDER_NAME: &str = "kimi";
const KIMI_IMPORT_PROVENANCE: &str = "cli-proxy-api";

/// Kimi OAuth credential data that is safe to hand to Codex OAuth storage.
#[derive(Clone, PartialEq, Eq)]
pub struct ImportableKimiOauthCredential {
    pub provider_name: String,
    pub device_id: Option<String>,
    pub token_type: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub access_token: String,
    pub refresh_token: String,
    pub expired: Option<bool>,
    pub provenance: String,
}

impl fmt::Debug for ImportableKimiOauthCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportableKimiOauthCredential")
            .field("provider_name", &self.provider_name)
            .field("device_id", &self.device_id)
            .field("token_type", &self.token_type)
            .field("scopes", &self.scopes)
            .field("expires_at", &self.expires_at)
            .field("expired", &self.expired)
            .field("provenance", &self.provenance)
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .finish()
    }
}

impl ImportableKimiOauthCredential {
    pub fn to_provider_oauth_credential(&self) -> ProviderOAuthCredential {
        let provider_name = self.provider_name.clone();
        let credential_id = format!("{provider_name}:imported");
        let mut credential = ProviderOAuthCredential::new(
            provider_name,
            ProviderCredentialSourceKind::ExternalImport,
            credential_id,
            self.access_token.clone(),
        );
        credential.scopes = self.scopes.clone();
        credential.expires_at = self.expires_at;
        credential.provenance = Some(self.provenance.clone());
        credential.refresh_token = Some(self.refresh_token.clone());
        credential
    }

    pub fn to_provider_oauth_credential_source(&self) -> StaticProviderOAuthCredentialSource {
        StaticProviderOAuthCredentialSource::new(self.to_provider_oauth_credential())
    }

    pub fn to_provider_credential_routing_view(&self) -> ProviderCredentialRoutingView {
        let credential = self.to_provider_oauth_credential();
        let mut view = ProviderCredentialRoutingView::new(
            credential.provider_name.clone(),
            credential.source_kind,
            ProviderCredentialAuthKind::OAuthBearer,
            credential.credential_id.clone(),
        );
        view.scopes = credential.scopes;
        view.expires_at = credential.expires_at;
        view.provenance = credential.provenance;
        view
    }
}

/// Parse result for a Kimi OAuth credential export sample.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KimiOauthImportReport {
    pub credential: Option<ImportableKimiOauthCredential>,
    pub rejections: Vec<KimiOauthImportRejection>,
    pub dry_run: bool,
}

/// A token record that was not safe to import.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KimiOauthImportRejection {
    pub reason: KimiOauthImportRejectionReason,
}

/// Reason a Kimi OAuth token record was not importable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KimiOauthImportRejectionReason {
    MissingTokenObject,
    InvalidTokenShape,
    InvalidType,
    MissingAccessToken,
    MissingRefreshToken,
    InvalidScopes,
    InvalidExpiry,
}

/// Parses the donor / CLIProxyAPI Kimi OAuth token sample shape.
pub fn parse_kimi_oauth_import_sample(value: &JsonValue, dry_run: bool) -> KimiOauthImportReport {
    match parse_credential(value) {
        Ok(credential) => KimiOauthImportReport {
            credential: Some(credential),
            rejections: Vec::new(),
            dry_run,
        },
        Err(reason) => KimiOauthImportReport {
            credential: None,
            rejections: vec![KimiOauthImportRejection { reason }],
            dry_run,
        },
    }
}

fn parse_credential(
    value: &JsonValue,
) -> Result<ImportableKimiOauthCredential, KimiOauthImportRejectionReason> {
    let Some(root_object) = value.as_object() else {
        return Err(KimiOauthImportRejectionReason::MissingTokenObject);
    };

    let provider_type = required_string(root_object, "type")?;
    if provider_type != KIMI_PROVIDER_NAME {
        return Err(KimiOauthImportRejectionReason::InvalidType);
    }

    let access_token = required_string(root_object, "access_token")?;
    let refresh_token = required_string(root_object, "refresh_token")?;
    let token_type = optional_string(root_object, "token_type")?;
    let device_id = optional_string(root_object, "device_id")?;
    let scopes = parse_scopes(root_object)?;
    let expires_at = parse_expires_at(root_object)?;
    let expired = optional_bool(root_object, "expired")?;

    Ok(ImportableKimiOauthCredential {
        provider_name: KIMI_PROVIDER_NAME.to_string(),
        device_id,
        token_type,
        scopes,
        expires_at,
        access_token,
        refresh_token,
        expired,
        provenance: KIMI_IMPORT_PROVENANCE.to_string(),
    })
}

fn required_string(
    record: &serde_json::Map<String, JsonValue>,
    key: &str,
) -> Result<String, KimiOauthImportRejectionReason> {
    let Some(value) = record.get(key) else {
        return Err(match key {
            "access_token" => KimiOauthImportRejectionReason::MissingAccessToken,
            "refresh_token" => KimiOauthImportRejectionReason::MissingRefreshToken,
            "type" => KimiOauthImportRejectionReason::InvalidType,
            _ => KimiOauthImportRejectionReason::InvalidTokenShape,
        });
    };
    let Some(value) = value.as_str() else {
        return Err(KimiOauthImportRejectionReason::InvalidTokenShape);
    };
    let value = value.trim();
    if value.is_empty() {
        return Err(match key {
            "access_token" => KimiOauthImportRejectionReason::MissingAccessToken,
            "refresh_token" => KimiOauthImportRejectionReason::MissingRefreshToken,
            "type" => KimiOauthImportRejectionReason::InvalidType,
            _ => KimiOauthImportRejectionReason::InvalidTokenShape,
        });
    }
    Ok(value.to_string())
}

fn optional_string(
    record: &serde_json::Map<String, JsonValue>,
    key: &str,
) -> Result<Option<String>, KimiOauthImportRejectionReason> {
    let Some(value) = record.get(key) else {
        return Ok(None);
    };
    let Some(value) = value.as_str() else {
        return Err(KimiOauthImportRejectionReason::InvalidTokenShape);
    };
    let value = value.trim();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_string()))
    }
}

fn optional_bool(
    record: &serde_json::Map<String, JsonValue>,
    key: &str,
) -> Result<Option<bool>, KimiOauthImportRejectionReason> {
    let Some(value) = record.get(key) else {
        return Ok(None);
    };
    value
        .as_bool()
        .map(Some)
        .ok_or(KimiOauthImportRejectionReason::InvalidTokenShape)
}

fn parse_scopes(
    record: &serde_json::Map<String, JsonValue>,
) -> Result<Vec<String>, KimiOauthImportRejectionReason> {
    let Some(scopes) = record.get("scope") else {
        return Ok(Vec::new());
    };

    match scopes {
        JsonValue::Array(values) => {
            let mut parsed = Vec::with_capacity(values.len());
            for scope in values {
                let Some(scope) = scope
                    .as_str()
                    .map(str::trim)
                    .filter(|scope| !scope.is_empty())
                else {
                    return Err(KimiOauthImportRejectionReason::InvalidScopes);
                };
                parsed.push(scope.to_string());
            }
            Ok(parsed)
        }
        JsonValue::String(scopes) => Ok(scopes
            .split_whitespace()
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
            .map(str::to_string)
            .collect()),
        _ => Err(KimiOauthImportRejectionReason::InvalidScopes),
    }
}

fn parse_expires_at(
    record: &serde_json::Map<String, JsonValue>,
) -> Result<Option<u64>, KimiOauthImportRejectionReason> {
    let Some(expires_at) = record.get("expires_at") else {
        return Ok(None);
    };

    parse_expiration_value(expires_at).map(Some)
}

fn parse_expiration_value(value: &JsonValue) -> Result<u64, KimiOauthImportRejectionReason> {
    if let Some(expires_at) = value.as_u64() {
        return Ok(expires_at);
    }

    let Some(expiry) = value
        .as_str()
        .map(str::trim)
        .filter(|expiry| !expiry.is_empty())
    else {
        return Err(KimiOauthImportRejectionReason::InvalidExpiry);
    };

    if let Ok(expiry) = expiry.parse::<u64>() {
        return Ok(expiry);
    }

    let parsed = OffsetDateTime::parse(expiry, &Rfc3339)
        .map_err(|_| KimiOauthImportRejectionReason::InvalidExpiry)?;
    let millis = parsed.unix_timestamp_nanos().div_euclid(1_000_000);
    u64::try_from(millis).map_err(|_| KimiOauthImportRejectionReason::InvalidExpiry)
}

#[cfg(test)]
#[path = "kimi_oauth_import_tests.rs"]
mod tests;
