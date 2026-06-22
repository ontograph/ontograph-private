use std::fmt;

use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderOAuthCredential;
use thiserror::Error;

pub const GEMINI_PROVIDER_ID: &str = "gemini";
pub const GEMINI_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
pub const GEMINI_OAUTH_CLIENT_ID: &str =
    "681255809395-oo8ft2oprdrnp9e3aqf6av3hmdib135j.apps.googleusercontent.com";

#[derive(Clone, PartialEq, Eq)]
pub struct GeminiOAuthTokens {
    pub credential_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub client_id: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub account_id: Option<String>,
    pub project_id: Option<String>,
    pub provenance: GeminiOAuthProvenance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeminiOAuthProvenance {
    Browser,
    UserCode,
    Device,
}

impl GeminiOAuthProvenance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Browser => "gemini-browser-oauth",
            Self::UserCode => "gemini-user-code-oauth",
            Self::Device => "gemini-device-oauth",
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GeminiOAuthCredentialError {
    #[error(
        "Gemini OAuth credentials are missing refresh metadata. Log in again with /login gemini."
    )]
    MissingRefreshToken,
    #[error("Gemini OAuth credentials are missing a client id. Log in again with /login gemini.")]
    MissingClientId,
}

impl GeminiOAuthTokens {
    pub fn into_provider_oauth_credential(
        self,
    ) -> Result<ProviderOAuthCredential, GeminiOAuthCredentialError> {
        if self.refresh_token.is_empty() {
            return Err(GeminiOAuthCredentialError::MissingRefreshToken);
        }
        if self.client_id.is_empty() {
            return Err(GeminiOAuthCredentialError::MissingClientId);
        }

        let mut credential = ProviderOAuthCredential::new(
            GEMINI_PROVIDER_ID,
            ProviderCredentialSourceKind::FirstPartyLogin,
            self.credential_id,
            self.access_token,
        );
        credential.refresh_token = Some(self.refresh_token);
        credential.client_id = Some(self.client_id);
        credential.token_endpoint = Some(GEMINI_TOKEN_ENDPOINT.to_string());
        credential.scopes = self.scopes;
        credential.expires_at = self.expires_at;
        credential.account_id = self.account_id;
        credential.endpoint = self.project_id;
        credential.provenance = Some(self.provenance.as_str().to_string());
        Ok(credential)
    }
}

impl fmt::Debug for GeminiOAuthTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeminiOAuthTokens")
            .field("credential_id", &self.credential_id)
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .field("client_id", &self.client_id)
            .field("scopes", &self.scopes)
            .field("expires_at", &self.expires_at)
            .field("account_id", &self.account_id)
            .field("project_id", &self.project_id)
            .field("provenance", &self.provenance)
            .finish()
    }
}

#[cfg(test)]
#[path = "gemini_oauth_tests.rs"]
mod tests;
