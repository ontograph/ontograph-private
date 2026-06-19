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

const GEMINI_DONOR_PROVIDER_NAME: &str = "gemini-cli";
const GEMINI_PROVIDER_NAME: &str = "gemini";
pub const ANTIGRAVITY_PROVIDER_NAME: &str = "antigravity";
const GEMINI_IMPORT_PROVENANCE: &str = "cli-proxy-api";
const GEMINI_ADC_IMPORT_PROVENANCE: &str = "google-oauth-import";
const ANTIGRAVITY_IMPORT_PROVENANCE: &str = "cliproxyapi-antigravity-import";
const DEFAULT_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const GEMINI_CLI_OAUTH_CLIENT_ID: &str =
    "681255809395-oo8ft2oprdrnp9e3aqf6av3hmdib135j.apps.googleusercontent.com";

/// Gemini OAuth credential data that is safe to hand to Codex OAuth storage.
#[derive(Clone, PartialEq, Eq)]
pub struct ImportableGeminiOauthCredential {
    pub provider_name: String,
    pub email: Option<String>,
    pub project_id: Option<String>,
    pub client_id: String,
    pub token_endpoint: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub provenance: String,
}

impl fmt::Debug for ImportableGeminiOauthCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportableGeminiOauthCredential")
            .field("provider_name", &self.provider_name)
            .field("email", &self.email)
            .field("project_id", &self.project_id)
            .field("client_id", &self.client_id)
            .field("token_endpoint", &self.token_endpoint)
            .field("scopes", &self.scopes)
            .field("expires_at", &self.expires_at)
            .field("provenance", &self.provenance)
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl ImportableGeminiOauthCredential {
    pub fn to_provider_credential_routing_view(&self) -> ProviderCredentialRoutingView {
        let mut view = ProviderCredentialRoutingView::new(
            self.provider_name.clone(),
            ProviderCredentialSourceKind::ExternalImport,
            ProviderCredentialAuthKind::OAuthBearer,
            self.credential_id(),
        );
        view.account_id = self.email.clone();
        view.endpoint = self.token_endpoint.clone();
        view.client_id = Some(self.client_id.clone());
        view.scopes = self.scopes.clone();
        view.expires_at = self.expires_at;
        view.provenance = Some(self.provenance.clone());
        view
    }

    pub fn to_provider_oauth_credential(&self) -> ProviderOAuthCredential {
        let mut credential = ProviderOAuthCredential::new(
            self.provider_name.clone(),
            ProviderCredentialSourceKind::ExternalImport,
            self.credential_id(),
            self.access_token.clone(),
        );
        credential.account_id = self.email.clone();
        credential.endpoint = self.token_endpoint.clone();
        credential.client_id = Some(self.client_id.clone());
        credential.token_endpoint = self.token_endpoint.clone();
        credential.scopes = self.scopes.clone();
        credential.expires_at = self.expires_at;
        credential.provenance = Some(self.provenance.clone());
        credential.refresh_token = self.refresh_token.clone();
        credential
    }

    pub fn to_provider_oauth_credential_source(&self) -> StaticProviderOAuthCredentialSource {
        StaticProviderOAuthCredentialSource::new(self.to_provider_oauth_credential())
    }

    fn credential_id(&self) -> String {
        if let Some(project_id) = self.project_id.as_deref().filter(|value| !value.is_empty()) {
            format!("{}:{project_id}", self.provider_name)
        } else if let Some(email) = self.email.as_deref().filter(|value| !value.is_empty()) {
            format!("{}:{email}", self.provider_name)
        } else {
            format!("{}:imported", self.provider_name)
        }
    }
}

/// Parse result for a Gemini OAuth credential export sample.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GeminiOauthImportReport {
    pub credential: Option<ImportableGeminiOauthCredential>,
    pub rejections: Vec<GeminiOauthImportRejection>,
    pub dry_run: bool,
}

/// A token record that was not safe to import.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GeminiOauthImportRejection {
    pub reason: GeminiOauthImportRejectionReason,
}

/// Reason a Gemini OAuth token record was not importable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GeminiOauthImportRejectionReason {
    MissingTokenObject,
    InvalidTokenShape,
    MissingAccessToken,
    MissingClientId,
    MissingRefreshToken,
    InvalidScopes,
    InvalidExpiry,
}

/// Parses the donor / CLIProxyAPI Gemini OAuth token sample shape.
pub fn parse_gemini_oauth_import_sample(
    value: &JsonValue,
    dry_run: bool,
) -> GeminiOauthImportReport {
    parse_gemini_oauth_import_sample_for_provider(
        value,
        dry_run,
        GEMINI_DONOR_PROVIDER_NAME,
        GEMINI_IMPORT_PROVENANCE,
    )
}

/// Parses a user-supplied Google ADC or desktop OAuth credential sample shape.
pub fn parse_gemini_adc_oauth_import_sample(
    value: &JsonValue,
    dry_run: bool,
) -> GeminiOauthImportReport {
    parse_gemini_oauth_import_sample_for_provider(
        value,
        dry_run,
        GEMINI_PROVIDER_NAME,
        GEMINI_ADC_IMPORT_PROVENANCE,
    )
}

/// Parses a user-supplied Antigravity OAuth credential sample shape.
pub fn parse_antigravity_oauth_import_sample(
    value: &JsonValue,
    dry_run: bool,
) -> GeminiOauthImportReport {
    let mut report = parse_gemini_oauth_import_sample_for_provider(
        value,
        dry_run,
        ANTIGRAVITY_PROVIDER_NAME,
        ANTIGRAVITY_IMPORT_PROVENANCE,
    );
    if report
        .credential
        .as_ref()
        .is_some_and(|credential| credential.refresh_token.is_none())
    {
        report.credential = None;
        report.rejections = vec![GeminiOauthImportRejection {
            reason: GeminiOauthImportRejectionReason::MissingRefreshToken,
        }];
    }
    report
}

fn parse_gemini_oauth_import_sample_for_provider(
    value: &JsonValue,
    dry_run: bool,
    provider_name: &str,
    provenance: &str,
) -> GeminiOauthImportReport {
    match parse_credential(value, provider_name, provenance) {
        Ok(credential) => GeminiOauthImportReport {
            credential: Some(credential),
            rejections: Vec::new(),
            dry_run,
        },
        Err(reason) => GeminiOauthImportReport {
            credential: None,
            rejections: vec![GeminiOauthImportRejection { reason }],
            dry_run,
        },
    }
}

fn parse_credential(
    value: &JsonValue,
    provider_name: &str,
    provenance: &str,
) -> Result<ImportableGeminiOauthCredential, GeminiOauthImportRejectionReason> {
    let Some(root_object) = value.as_object() else {
        return Err(GeminiOauthImportRejectionReason::MissingTokenObject);
    };

    let token = match root_object.get("token") {
        Some(token) if token.is_object() => token,
        Some(_) => return Err(GeminiOauthImportRejectionReason::InvalidTokenShape),
        None => value,
    };

    let email = optional_string(token, value, "email")?;
    let project_id = optional_string(token, value, "project_id")?.or(optional_string(
        token,
        value,
        "quota_project_id",
    )?);
    let access_token = required_string(token, value, "access_token")?;
    let client_id = optional_string(token, value, "client_id")?
        .or_else(|| {
            is_gemini_cli_token_file(token, value).then_some(GEMINI_CLI_OAUTH_CLIENT_ID.to_string())
        })
        .ok_or(GeminiOauthImportRejectionReason::MissingClientId)?;
    let refresh_token = optional_string(token, value, "refresh_token")?;
    let token_endpoint = optional_string(token, value, "token_uri")?
        .or_else(|| Some(DEFAULT_TOKEN_ENDPOINT.to_string()));
    let scopes = parse_scopes(token, value)?;
    let expires_at = parse_expires_at(token, value)?;

    Ok(ImportableGeminiOauthCredential {
        provider_name: provider_name.to_string(),
        email,
        project_id,
        client_id,
        token_endpoint,
        scopes,
        expires_at,
        access_token,
        refresh_token,
        provenance: provenance.to_string(),
    })
}

fn required_string(
    record: &JsonValue,
    root: &JsonValue,
    key: &str,
) -> Result<String, GeminiOauthImportRejectionReason> {
    let Some(value) = field(record, root, key) else {
        return Err(match key {
            "access_token" => GeminiOauthImportRejectionReason::MissingAccessToken,
            "client_id" => GeminiOauthImportRejectionReason::MissingClientId,
            _ => GeminiOauthImportRejectionReason::InvalidTokenShape,
        });
    };
    let Some(value) = value.as_str() else {
        return Err(GeminiOauthImportRejectionReason::InvalidTokenShape);
    };
    let value = value.trim();
    if value.is_empty() {
        return Err(match key {
            "access_token" => GeminiOauthImportRejectionReason::MissingAccessToken,
            "client_id" => GeminiOauthImportRejectionReason::MissingClientId,
            _ => GeminiOauthImportRejectionReason::InvalidTokenShape,
        });
    }
    Ok(value.to_string())
}

fn optional_string(
    record: &JsonValue,
    root: &JsonValue,
    key: &str,
) -> Result<Option<String>, GeminiOauthImportRejectionReason> {
    let Some(value) = field(record, root, key) else {
        return Ok(None);
    };
    let Some(value) = value.as_str() else {
        return Err(GeminiOauthImportRejectionReason::InvalidTokenShape);
    };
    let value = value.trim();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_string()))
    }
}

fn parse_scopes(
    record: &JsonValue,
    root: &JsonValue,
) -> Result<Vec<String>, GeminiOauthImportRejectionReason> {
    let Some(scopes) = field(record, root, "scopes").or_else(|| field(record, root, "scope"))
    else {
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
                    return Err(GeminiOauthImportRejectionReason::InvalidScopes);
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
        _ => Err(GeminiOauthImportRejectionReason::InvalidScopes),
    }
}

fn parse_expires_at(
    record: &JsonValue,
    root: &JsonValue,
) -> Result<Option<u64>, GeminiOauthImportRejectionReason> {
    if let Some(expires_at) = field(record, root, "expires_at") {
        return parse_expiration_value(expires_at).map(Some);
    }

    if let Some(expiry) = field(record, root, "expiry") {
        return parse_expiration_value(expiry).map(Some);
    }

    if let Some(expiry_date) = field(record, root, "expiry_date") {
        return parse_expiration_value(expiry_date).map(Some);
    }

    Ok(None)
}

fn is_gemini_cli_token_file(record: &JsonValue, root: &JsonValue) -> bool {
    field(record, root, "refresh_token").is_some()
        && field(record, root, "token_type").is_some()
        && (field(record, root, "scope").is_some() || field(record, root, "expiry_date").is_some())
}

fn parse_expiration_value(value: &JsonValue) -> Result<u64, GeminiOauthImportRejectionReason> {
    if let Some(expires_at) = value.as_u64() {
        return Ok(expires_at);
    }

    let Some(expiry) = value
        .as_str()
        .map(str::trim)
        .filter(|expiry| !expiry.is_empty())
    else {
        return Err(GeminiOauthImportRejectionReason::InvalidExpiry);
    };

    if let Ok(expiry) = expiry.parse::<u64>() {
        return Ok(expiry);
    }

    let parsed = OffsetDateTime::parse(expiry, &Rfc3339)
        .map_err(|_| GeminiOauthImportRejectionReason::InvalidExpiry)?;
    let millis = parsed.unix_timestamp_nanos().div_euclid(1_000_000);
    u64::try_from(millis).map_err(|_| GeminiOauthImportRejectionReason::InvalidExpiry)
}

fn field<'a>(record: &'a JsonValue, root: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    record.get(key).or_else(|| root.get(key))
}

#[cfg(test)]
#[path = "gemini_oauth_import_tests.rs"]
mod tests;
