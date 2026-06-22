//! App-server-backed config update helpers for the TUI.
//!
//! This module centralizes the small typed update helpers the TUI uses
//! when a config mutation must be owned by the app server rather than written
//! to the local `config.toml` directly.

use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use ontocode_app_server_client::AppServerRequestHandle;
use ontocode_app_server_protocol::ClientRequest;
use ontocode_app_server_protocol::ConfigBatchWriteParams;
use ontocode_app_server_protocol::ConfigEdit;
use ontocode_app_server_protocol::ConfigReadParams;
use ontocode_app_server_protocol::ConfigReadResponse;
use ontocode_app_server_protocol::ConfigWriteResponse;
use ontocode_app_server_protocol::MergeStrategy;
use ontocode_app_server_protocol::RequestId;
use ontocode_app_server_protocol::SkillsConfigWriteParams;
use ontocode_app_server_protocol::SkillsConfigWriteResponse;
use ontocode_config::loader::project_trust_key;
use ontocode_features::FEATURES;
use ontocode_protocol::config_types::SERVICE_TIER_DEFAULT_REQUEST_VALUE;
use ontocode_protocol::config_types::TrustLevel;
use ontocode_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value as JsonValue;
use std::path::Path;
use uuid::Uuid;

pub(crate) fn replace_config_value(key_path: impl Into<String>, value: JsonValue) -> ConfigEdit {
    ConfigEdit {
        key_path: key_path.into(),
        value,
        merge_strategy: MergeStrategy::Replace,
    }
}

pub(crate) fn clear_config_value(key_path: impl Into<String>) -> ConfigEdit {
    replace_config_value(key_path, JsonValue::Null)
}

pub(crate) struct ModelSelectionConfigEdits<'a, T> {
    pub(crate) model: &'a str,
    pub(crate) model_provider: Option<&'a str>,
    pub(crate) effort: Option<T>,
}

pub(crate) fn app_scoped_key_path(app_id: &str, key_path: &str) -> String {
    let app_id = serde_json::Value::String(app_id.to_string()).to_string();
    format!("apps.{app_id}.{key_path}")
}

fn trusted_project_edit(project_path: &Path) -> ConfigEdit {
    let project_key = project_trust_key(project_path)
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    replace_config_value(
        format!("projects.\"{project_key}\".trust_level"),
        serde_json::json!(TrustLevel::Trusted.to_string()),
    )
}

pub(crate) fn build_model_selection_edits<T: ToString>(
    selection: ModelSelectionConfigEdits<'_, T>,
) -> Vec<ConfigEdit> {
    let effort_edit = selection.effort.map_or_else(
        || clear_config_value("model_reasoning_effort"),
        |effort| {
            replace_config_value(
                "model_reasoning_effort",
                serde_json::json!(effort.to_string()),
            )
        },
    );
    let mut edits = vec![replace_config_value(
        "model",
        serde_json::json!(selection.model),
    )];
    if let Some(model_provider) = selection.model_provider {
        edits.push(replace_config_value(
            "model_provider",
            serde_json::json!(model_provider),
        ));
    }
    edits.push(effort_edit);
    edits
}

pub(crate) fn build_service_tier_selection_edits(service_tier: Option<&str>) -> Vec<ConfigEdit> {
    let service_tier_edit = service_tier.map_or_else(
        || clear_config_value("service_tier"),
        |service_tier| {
            let config_value = if service_tier == SERVICE_TIER_DEFAULT_REQUEST_VALUE {
                SERVICE_TIER_DEFAULT_REQUEST_VALUE
            } else {
                match ontocode_protocol::config_types::ServiceTier::from_request_value(service_tier)
                {
                    Some(ontocode_protocol::config_types::ServiceTier::Fast) => "fast",
                    Some(ontocode_protocol::config_types::ServiceTier::Flex) => "flex",
                    None => service_tier,
                }
            };
            replace_config_value("service_tier", serde_json::json!(config_value))
        },
    );
    vec![service_tier_edit]
}

#[cfg(target_os = "windows")]
pub(crate) fn build_windows_sandbox_mode_edits(elevated_enabled: bool) -> Vec<ConfigEdit> {
    let feature_key_path = |feature: &str| format!("features.{feature}");
    vec![
        replace_config_value(
            "windows.sandbox",
            serde_json::json!(if elevated_enabled {
                "elevated"
            } else {
                "unelevated"
            }),
        ),
        clear_config_value(feature_key_path("experimental_windows_sandbox")),
        clear_config_value(feature_key_path("elevated_windows_sandbox")),
        clear_config_value(feature_key_path("enable_experimental_windows_sandbox")),
    ]
}

pub(crate) fn build_feature_enabled_edit(feature_key: &str, enabled: bool) -> ConfigEdit {
    let key_path = format!("features.{feature_key}");
    let is_default_false_feature = FEATURES
        .iter()
        .find(|spec| spec.key == feature_key)
        .is_some_and(|spec| !spec.default_enabled);
    if enabled || !is_default_false_feature {
        replace_config_value(key_path, serde_json::json!(enabled))
    } else {
        clear_config_value(key_path)
    }
}

pub(crate) fn build_memory_settings_edits(
    use_memories: bool,
    generate_memories: bool,
) -> Vec<ConfigEdit> {
    vec![
        replace_config_value("memories.use_memories", serde_json::json!(use_memories)),
        replace_config_value(
            "memories.generate_memories",
            serde_json::json!(generate_memories),
        ),
    ]
}

pub(crate) fn build_oss_provider_edit(provider: &str) -> ConfigEdit {
    replace_config_value("oss_provider", serde_json::json!(provider))
}

pub(crate) async fn write_config_batch(
    request_handle: AppServerRequestHandle,
    edits: Vec<ConfigEdit>,
) -> Result<ConfigWriteResponse> {
    let request_id = RequestId::String(format!("tui-config-write-{}", Uuid::new_v4()));
    request_handle
        .request_typed(ClientRequest::ConfigBatchWrite {
            request_id,
            params: ConfigBatchWriteParams {
                edits,
                file_path: None,
                expected_version: None,
                reload_user_config: true,
            },
        })
        .await
        .wrap_err("config/batchWrite failed in TUI")
}

pub(crate) async fn write_trusted_project(
    request_handle: AppServerRequestHandle,
    project_path: &Path,
) -> Result<ConfigWriteResponse> {
    write_config_batch(request_handle, vec![trusted_project_edit(project_path)]).await
}

pub(crate) async fn read_effective_config(
    request_handle: AppServerRequestHandle,
    cwd: String,
) -> Result<ConfigReadResponse> {
    let request_id = RequestId::String(format!("tui-config-read-{}", Uuid::new_v4()));
    request_handle
        .request_typed(ClientRequest::ConfigRead {
            request_id,
            params: ConfigReadParams {
                include_layers: false,
                cwd: Some(cwd),
            },
        })
        .await
        .wrap_err("config/read failed in TUI")
}

pub(crate) async fn write_skill_enabled(
    request_handle: AppServerRequestHandle,
    path: AbsolutePathBuf,
    enabled: bool,
) -> Result<()> {
    let request_id = RequestId::String(format!("tui-skill-config-write-{}", Uuid::new_v4()));
    let _: SkillsConfigWriteResponse = request_handle
        .request_typed(ClientRequest::SkillsConfigWrite {
            request_id,
            params: SkillsConfigWriteParams {
                path: Some(path),
                name: None,
                enabled,
            },
        })
        .await
        .wrap_err("skills/config/write failed in TUI")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn app_scoped_key_path_quotes_dotted_app_ids() {
        assert_eq!(
            app_scoped_key_path("plugin.linear", "enabled"),
            "apps.\"plugin.linear\".enabled"
        );
    }

    #[test]
    fn trusted_project_edit_targets_project_trust_level() {
        assert_eq!(
            trusted_project_edit(Path::new("/workspace/team.project")),
            ConfigEdit {
                key_path: "projects.\"/workspace/team.project\".trust_level".to_string(),
                value: serde_json::json!("trusted"),
                merge_strategy: MergeStrategy::Replace,
            }
        );
    }

    #[test]
    fn model_selection_edits_preserve_legacy_model_only_selection() {
        assert_eq!(
            build_model_selection_edits(ModelSelectionConfigEdits {
                model: "gpt-5.4",
                model_provider: None,
                effort: Some("medium"),
            }),
            vec![
                replace_config_value("model", serde_json::json!("gpt-5.4")),
                replace_config_value("model_reasoning_effort", serde_json::json!("medium")),
            ]
        );
    }

    #[test]
    fn model_selection_edits_write_provider_model_and_effort_together() {
        assert_eq!(
            build_model_selection_edits(ModelSelectionConfigEdits {
                model: "claude-sonnet-4-5",
                model_provider: Some("claude"),
                effort: None::<&str>,
            }),
            vec![
                replace_config_value("model", serde_json::json!("claude-sonnet-4-5")),
                replace_config_value("model_provider", serde_json::json!("claude")),
                clear_config_value("model_reasoning_effort"),
            ]
        );
    }
}
