use std::fmt;

use serde_json::Value as JsonValue;
use serde_json::json;

const CLOUD_CODE_ASSIST_IDE_TYPE: &str = "ANTIGRAVITY";
const CLOUD_CODE_ASSIST_PLATFORM: &str = "PLATFORM_UNSPECIFIED";
const CLOUD_CODE_ASSIST_PLUGIN_TYPE: &str = "GEMINI";
const CLOUD_CODE_ASSIST_TIER_ID: &str = "legacy-tier";

/// Small private owner used by tests to cover the donor-shaped request contract
/// and the refresh dedupe identity without wiring runtime behavior.
#[derive(Clone, PartialEq, Eq)]
struct AntigravityRequestRefreshOwner {
    provider_id: String,
    credential_id: String,
    project_id: Option<String>,
    access_token: String,
    refresh_token: Option<String>,
}

impl fmt::Debug for AntigravityRequestRefreshOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AntigravityRequestRefreshOwner")
            .field("provider_id", &self.provider_id)
            .field("credential_id", &self.credential_id)
            .field("project_id", &self.project_id)
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl AntigravityRequestRefreshOwner {
    fn new(
        provider_id: impl Into<String>,
        credential_id: impl Into<String>,
        project_id: Option<impl Into<String>>,
        access_token: impl Into<String>,
        refresh_token: Option<impl Into<String>>,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            credential_id: credential_id.into(),
            project_id: project_id.map(Into::into),
            access_token: access_token.into(),
            refresh_token: refresh_token.map(Into::into),
        }
    }

    fn cloud_code_assist_request_contract(&self) -> JsonValue {
        json!({
            "provider": self.provider_id.clone(),
            "loadCodeAssist": {
                "metadata": {
                    "ideType": CLOUD_CODE_ASSIST_IDE_TYPE,
                    "platform": CLOUD_CODE_ASSIST_PLATFORM,
                    "pluginType": CLOUD_CODE_ASSIST_PLUGIN_TYPE,
                },
            },
            "onboardUser": {
                "tierId": CLOUD_CODE_ASSIST_TIER_ID,
                "metadata": {
                    "ideType": CLOUD_CODE_ASSIST_IDE_TYPE,
                    "platform": CLOUD_CODE_ASSIST_PLATFORM,
                    "pluginType": CLOUD_CODE_ASSIST_PLUGIN_TYPE,
                },
            },
        })
    }

    fn refresh_dedupe_key(&self) -> AntigravityRefreshDedupeKey {
        AntigravityRefreshDedupeKey {
            provider_id: self.provider_id.clone(),
            credential_id: self.credential_id.clone(),
            project_id: self.project_id.clone(),
        }
    }
}

/// Dedupe identity for Antigravity refresh work.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct AntigravityRefreshDedupeKey {
    provider_id: String,
    credential_id: String,
    project_id: Option<String>,
}

impl fmt::Display for AntigravityRefreshDedupeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.project_id.as_deref() {
            Some(project_id) => write!(
                f,
                "{}|{}|{}",
                self.provider_id, self.credential_id, project_id
            ),
            None => write!(f, "{}|{}", self.provider_id, self.credential_id),
        }
    }
}

#[cfg(test)]
#[path = "antigravity_runtime_tests.rs"]
mod tests;
