use super::*;
use assert_matches::assert_matches;
use ontocode_config::types::ModelAvailabilityNuxConfig;
use ontocode_model_provider_info::GEMINI_CLI_PROVIDER_ID;
use ontocode_model_provider_info::GEMINI_PROVIDER_ID;
use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_model_provider_info::OPENAI_PROVIDER_ID;
use ontocode_model_provider_info::WireApi;
use ontocode_models_manager::native_provider_catalogs;
use ontocode_protocol::openai_models::ConfigShellToolType;
use ontocode_protocol::openai_models::ModelAvailabilityNux;
use ontocode_protocol::openai_models::ModelInfo;
use ontocode_protocol::openai_models::ModelVisibility;
use ontocode_protocol::openai_models::ModelsResponse;
use ontocode_protocol::openai_models::TruncationPolicyConfig;
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use tokio::sync::mpsc::unbounded_channel;
use wiremock::MockServer;

fn all_model_presets() -> Vec<ModelPreset> {
    crate::legacy_core::test_support::all_model_presets().clone()
}

fn dynamic_model(slug: &str, visibility: ModelVisibility) -> ModelInfo {
    ModelInfo {
        slug: slug.to_string(),
        display_name: slug.to_string(),
        description: None,
        default_reasoning_level: None,
        supported_reasoning_levels: Vec::new(),
        shell_type: ConfigShellToolType::Default,
        visibility,
        supported_in_api: false,
        priority: 0,
        additional_speed_tiers: Vec::new(),
        service_tiers: Vec::new(),
        default_service_tier: None,
        availability_nux: None,
        upgrade: None,
        base_instructions: String::new(),
        model_messages: None,
        supports_reasoning_summaries: false,
        default_reasoning_summary: Default::default(),
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: None,
        web_search_tool_type: Default::default(),
        truncation_policy: TruncationPolicyConfig::bytes(/*limit*/ 10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: false,
        context_window: None,
        max_context_window: None,
        auto_compact_token_limit: None,
        effective_context_window_percent: 95,
        experimental_supported_tools: Vec::new(),
        input_modalities: Default::default(),
        used_fallback_model_metadata: false,
        supports_search_tool: false,
        auto_review_model_override: None,
        tool_mode: None,
        multi_agent_version: None,
    }
}

fn model_availability_nux_config(shown_count: &[(&str, u32)]) -> ModelAvailabilityNuxConfig {
    ModelAvailabilityNuxConfig {
        shown_count: shown_count
            .iter()
            .map(|(model, count)| ((*model).to_string(), *count))
            .collect(),
    }
}

fn model_migration_copy_to_plain_text(copy: &crate::model_migration::ModelMigrationCopy) -> String {
    if let Some(markdown) = copy.markdown.as_ref() {
        return markdown.clone();
    }
    let mut s = String::new();
    for span in &copy.heading {
        s.push_str(&span.content);
    }
    s.push('\n');
    s.push('\n');
    for line in &copy.content {
        for span in &line.spans {
            s.push_str(&span.content);
        }
        s.push('\n');
    }
    s
}

#[tokio::test]
async fn build_provider_model_groups_includes_disabled_gemini_cli_group() {
    let (chat_widget, _app_event_tx, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
    let config = chat_widget.config_ref().clone();

    let groups = build_provider_model_groups(
        &config,
        &crate::legacy_core::test_support::all_model_presets().clone(),
    )
    .await;

    let gemini_cli_group = groups
        .iter()
        .find(|group| group.provider_id == GEMINI_CLI_PROVIDER_ID)
        .expect("gemini-cli provider group should be present");

    assert_eq!(
        gemini_cli_group,
        &crate::model_catalog::gemini_cli_disabled_provider_group()
    );
}

#[tokio::test]
async fn build_provider_model_groups_marks_blocked_runtime_providers() {
    let (chat_widget, _app_event_tx, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
    let mut config = chat_widget.config_ref().clone();
    config.model_providers = HashMap::from([
        (
            "kimi".to_string(),
            ModelProviderInfo {
                name: "Kimi".to_string(),
                ..ModelProviderInfo::default()
            },
        ),
        (
            "antigravity".to_string(),
            ModelProviderInfo {
                name: "Antigravity".to_string(),
                ..ModelProviderInfo::default()
            },
        ),
        (
            "claude".to_string(),
            ModelProviderInfo {
                name: "Claude".to_string(),
                ..ModelProviderInfo::default()
            },
        ),
    ]);
    config.model_provider_id = "kimi".to_string();
    config.model_provider = config
        .model_providers
        .get(&config.model_provider_id)
        .cloned()
        .expect("selected provider config should exist");

    let groups = build_provider_model_groups(&config, &all_model_presets()).await;

    let kimi_group = groups
        .iter()
        .find(|group| group.provider_id == "kimi")
        .expect("kimi provider group should be present");
    let antigravity_group = groups
        .iter()
        .find(|group| group.provider_id == "antigravity")
        .expect("antigravity provider group should be present");
    let claude_group = groups
        .iter()
        .find(|group| group.provider_id == "claude")
        .expect("claude provider group should be present");

    assert_eq!(
        kimi_group.disabled_reason.as_deref(),
        Some("Kimi runtime is not available yet.")
    );
    assert!(kimi_group.models.is_empty());
    assert_eq!(
        antigravity_group.disabled_reason.as_deref(),
        Some("Antigravity runtime is not available yet.")
    );
    assert!(antigravity_group.models.is_empty());
    assert_eq!(
        claude_group.disabled_reason.as_deref(),
        Some("Claude runtime is not available yet.")
    );
    assert!(claude_group.models.is_empty());
}

#[tokio::test]
async fn build_provider_model_groups_includes_gemini_group() {
    let (chat_widget, _app_event_tx, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
    let config = chat_widget.config_ref().clone();

    let groups = build_provider_model_groups(
        &config,
        &crate::legacy_core::test_support::all_model_presets().clone(),
    )
    .await;

    let gemini_group = groups
        .iter()
        .find(|group| group.provider_id == GEMINI_PROVIDER_ID)
        .expect("gemini provider group should be present");

    assert_eq!(gemini_group.display_name, "Gemini");
    assert_eq!(gemini_group.disabled_reason, None);
    assert!(!gemini_group.models.is_empty());

    let gemini_index = groups
        .iter()
        .position(|group| group.provider_id == GEMINI_PROVIDER_ID)
        .expect("gemini provider group should be present");
    let gemini_cli_index = groups
        .iter()
        .position(|group| group.provider_id == GEMINI_CLI_PROVIDER_ID)
        .expect("gemini-cli provider group should be present");
    assert!(gemini_index < gemini_cli_index);
}

#[tokio::test]
async fn build_provider_model_groups_includes_openai_group_when_current_provider_is_not_openai() {
    let (chat_widget, _app_event_tx, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
    let mut config = chat_widget.config_ref().clone();
    assert!(config.model_providers.contains_key(OPENAI_PROVIDER_ID));

    config.model_provider_id = GEMINI_PROVIDER_ID.to_string();
    config.model_provider = config
        .model_providers
        .get(&config.model_provider_id)
        .cloned()
        .expect("selected current provider should have configuration");

    let groups = build_provider_model_groups(
        &config,
        &crate::legacy_core::test_support::all_model_presets().clone(),
    )
    .await;

    let openai_group = groups
        .iter()
        .find(|group| group.provider_id == OPENAI_PROVIDER_ID)
        .expect("openai provider group should be present");

    assert!(!openai_group.models.is_empty());
    assert_eq!(
        groups
            .iter()
            .filter(|group| group.provider_id == OPENAI_PROVIDER_ID)
            .count(),
        1
    );
}

#[tokio::test]
async fn build_provider_model_groups_keeps_external_provider_raw_catalog() {
    let server = MockServer::start().await;
    core_test_support::responses::mount_models_once(
        &server,
        ModelsResponse {
            models: vec![
                dynamic_model("gpt-5.3-codex-spark", ModelVisibility::List),
                dynamic_model("hidden-provider-model", ModelVisibility::Hide),
            ],
        },
    )
    .await;
    let (chat_widget, _app_event_tx, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
    let mut config = chat_widget.config_ref().clone();
    config.model_provider_id = OPENAI_PROVIDER_ID.to_string();
    config.model_provider = config
        .model_providers
        .get(OPENAI_PROVIDER_ID)
        .cloned()
        .expect("openai provider should exist");
    config.model_providers.insert(
        "cliproxyapi".to_string(),
        ModelProviderInfo {
            name: "CLIProxyAPI".to_string(),
            base_url: Some(format!("{}/v1", server.uri())),
            wire_api: WireApi::Responses,
            experimental_bearer_token: Some("provider-token".to_string()),
            ..ModelProviderInfo::default()
        },
    );

    let groups = build_provider_model_groups(&config, &all_model_presets()).await;

    let provider_group = groups
        .iter()
        .find(|group| group.provider_id == "cliproxyapi")
        .expect("cliproxyapi provider group should be present");
    assert!(
        provider_group
            .models
            .iter()
            .any(|model| model.model == "gpt-5.3-codex-spark"),
        "expected dynamic CLIProxyAPI model in group: {:?}",
        provider_group.models
    );
    assert!(
        provider_group
            .models
            .iter()
            .any(|model| !model.show_in_picker)
    );
}

#[test]
fn gemini_cli_disabled_provider_group_uses_static_catalog_data() {
    let group = crate::model_catalog::gemini_cli_disabled_provider_group();
    let expected_models = native_provider_catalogs::gemini_models_response()
        .models
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();

    assert_eq!(group.provider_id, GEMINI_CLI_PROVIDER_ID);
    assert_eq!(group.display_name, "Gemini CLI");
    assert_eq!(
        group.disabled_reason.as_deref(),
        Some(crate::model_catalog::GEMINI_CLI_DISABLED_REASON)
    );
    assert_eq!(group.models, expected_models);
}

#[test]
fn gemini_provider_group_uses_static_catalog_data() {
    let group = crate::model_catalog::gemini_provider_group();
    let expected_models = native_provider_catalogs::gemini_models_response()
        .models
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();

    assert_eq!(group.provider_id, GEMINI_PROVIDER_ID);
    assert_eq!(group.display_name, "Gemini");
    assert_eq!(group.disabled_reason, None);
    assert_eq!(group.models, expected_models);
}

#[tokio::test]
async fn model_migration_prompt_only_shows_for_deprecated_models() {
    let seen = BTreeMap::new();
    assert!(should_show_model_migration_prompt(
        "gpt-5.2",
        "gpt-5.4",
        &seen,
        &all_model_presets()
    ));
    assert!(should_show_model_migration_prompt(
        "gpt-5.3-codex",
        "gpt-5.4",
        &seen,
        &all_model_presets()
    ));
    assert!(!should_show_model_migration_prompt(
        "gpt-5.3-codex",
        "gpt-5.3-codex",
        &seen,
        &all_model_presets()
    ));
}

#[test]
fn select_model_availability_nux_picks_only_eligible_model() {
    let mut presets = all_model_presets();
    presets.iter_mut().for_each(|preset| {
        preset.availability_nux = None;
    });
    let target = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("target preset present");
    target.availability_nux = Some(ModelAvailabilityNux {
        message: "gpt-5.4 is available".to_string(),
    });

    let selected = select_model_availability_nux(&presets, &model_availability_nux_config(&[]));

    assert_eq!(
        selected,
        Some(StartupTooltipOverride {
            model_slug: "gpt-5.4".to_string(),
            message: "gpt-5.4 is available".to_string(),
        })
    );
}

#[test]
fn select_model_availability_nux_skips_missing_and_exhausted_models() {
    let mut presets = all_model_presets();
    presets.iter_mut().for_each(|preset| {
        preset.availability_nux = None;
    });
    let gpt_5 = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("gpt-5.4 preset present");
    gpt_5.availability_nux = Some(ModelAvailabilityNux {
        message: "gpt-5.4 is available".to_string(),
    });
    let gpt_5_2 = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4-mini")
        .expect("gpt-5.4-mini preset present");
    gpt_5_2.availability_nux = Some(ModelAvailabilityNux {
        message: "gpt-5.4-mini is available".to_string(),
    });

    let selected = select_model_availability_nux(
        &presets,
        &model_availability_nux_config(&[("gpt-5.4", MODEL_AVAILABILITY_NUX_MAX_SHOW_COUNT)]),
    );

    assert_eq!(
        selected,
        Some(StartupTooltipOverride {
            model_slug: "gpt-5.4-mini".to_string(),
            message: "gpt-5.4-mini is available".to_string(),
        })
    );
}

#[test]
fn select_model_availability_nux_uses_existing_model_order_as_priority() {
    let mut presets = all_model_presets();
    presets.iter_mut().for_each(|preset| {
        preset.availability_nux = None;
    });
    let first = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4-mini")
        .expect("gpt-5.4-mini preset present");
    first.availability_nux = Some(ModelAvailabilityNux {
        message: "first".to_string(),
    });
    let second = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("gpt-5.4 preset present");
    second.availability_nux = Some(ModelAvailabilityNux {
        message: "second".to_string(),
    });

    let selected = select_model_availability_nux(&presets, &model_availability_nux_config(&[]));

    assert_eq!(
        selected,
        Some(StartupTooltipOverride {
            model_slug: "gpt-5.4".to_string(),
            message: "second".to_string(),
        })
    );
}

#[test]
fn select_model_availability_nux_returns_none_when_all_models_are_exhausted() {
    let mut presets = all_model_presets();
    presets.iter_mut().for_each(|preset| {
        preset.availability_nux = None;
    });
    let target = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("target preset present");
    target.availability_nux = Some(ModelAvailabilityNux {
        message: "gpt-5.4 is available".to_string(),
    });

    let selected = select_model_availability_nux(
        &presets,
        &model_availability_nux_config(&[("gpt-5.4", MODEL_AVAILABILITY_NUX_MAX_SHOW_COUNT)]),
    );

    assert_eq!(selected, None);
}

#[tokio::test]
async fn prepare_startup_tooltip_override_persists_model_availability_nux_count() {
    let codex_home = tempdir().expect("temp codex home");
    let mut config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .build()
        .await
        .expect("config");
    let mut presets = all_model_presets();
    presets.iter_mut().for_each(|preset| {
        preset.availability_nux = None;
    });
    let target = presets
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("target preset present");
    target.availability_nux = Some(ModelAvailabilityNux {
        message: "gpt-5.4 is available".to_string(),
    });

    let tooltip =
        prepare_startup_tooltip_override(&mut config, &presets, /*is_first_run*/ false).await;

    assert_eq!(tooltip.as_deref(), Some("gpt-5.4 is available"));
    assert_eq!(
        config.model_availability_nux.shown_count,
        HashMap::from([("gpt-5.4".to_string(), 1)])
    );

    let reloaded = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .build()
        .await
        .expect("reloaded config");
    assert_eq!(
        reloaded.model_availability_nux.shown_count,
        HashMap::from([("gpt-5.4".to_string(), 1)])
    );
}

#[tokio::test]
async fn accepted_model_migration_persists_target_default_reasoning_effort() {
    let codex_home = tempdir().expect("temp codex home");
    let mut config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .build()
        .await
        .expect("config");
    config.model = Some("gpt-5.2".to_string());
    config.model_reasoning_effort = Some(ReasoningEffortConfig::XHigh);

    let (tx_raw, mut rx) = unbounded_channel();
    let app_event_tx = AppEventSender::new(tx_raw);

    apply_accepted_model_migration(
        &mut config,
        &app_event_tx,
        "gpt-5.2".to_string(),
        "gpt-5.4".to_string(),
        ReasoningEffortConfig::Medium,
    );

    assert_eq!(config.model.as_deref(), Some("gpt-5.4"));
    assert_eq!(
        config.model_reasoning_effort,
        Some(ReasoningEffortConfig::Medium)
    );

    let acknowledged = rx.try_recv().expect("acknowledged event");
    assert_matches!(
        acknowledged,
        AppEvent::PersistModelMigrationPromptAcknowledged { from_model, to_model }
            if from_model == "gpt-5.2" && to_model == "gpt-5.4"
    );

    let update_model = rx.try_recv().expect("update model event");
    assert_matches!(
        update_model,
        AppEvent::UpdateModel(model) if model == "gpt-5.4"
    );

    let update_effort = rx.try_recv().expect("update effort event");
    assert_matches!(
        update_effort,
        AppEvent::UpdateReasoningEffort(Some(ReasoningEffortConfig::Medium))
    );

    let persist_selection = rx.try_recv().expect("persist model selection event");
    assert_matches!(
        persist_selection,
        AppEvent::PersistModelSelection(selection)
            if selection.model == "gpt-5.4"
                && selection.model_provider.is_none()
                && selection.effort == Some(ReasoningEffortConfig::Medium)
    );
}

#[tokio::test]
async fn model_migration_prompt_respects_hide_flag_and_self_target() {
    let mut seen = BTreeMap::new();
    seen.insert("gpt-5.2".to_string(), "gpt-5.4".to_string());
    assert!(!should_show_model_migration_prompt(
        "gpt-5.2",
        "gpt-5.4",
        &seen,
        &all_model_presets()
    ));
    assert!(!should_show_model_migration_prompt(
        "gpt-5.4",
        "gpt-5.4",
        &seen,
        &all_model_presets()
    ));
}

#[tokio::test]
async fn model_migration_prompt_skips_when_target_missing_or_hidden() {
    let mut available = all_model_presets();
    let mut current = available
        .iter()
        .find(|preset| preset.model == "gpt-5.2")
        .cloned()
        .expect("preset present");
    current.upgrade = Some(ModelUpgrade {
        id: "missing-target".to_string(),
        reasoning_effort_mapping: None,
        migration_config_key: HIDE_GPT5_1_MIGRATION_PROMPT_CONFIG.to_string(),
        model_link: None,
        upgrade_copy: None,
        migration_markdown: None,
    });
    available.retain(|preset| preset.model != "gpt-5.2");
    available.push(current.clone());

    assert!(!should_show_model_migration_prompt(
        &current.model,
        "missing-target",
        &BTreeMap::new(),
        &available,
    ));

    assert!(target_preset_for_upgrade(&available, "missing-target").is_none());

    let mut with_hidden_target = all_model_presets();
    let target = with_hidden_target
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.4")
        .expect("target preset present");
    target.show_in_picker = false;

    assert!(!should_show_model_migration_prompt(
        "gpt-5.2",
        "gpt-5.4",
        &BTreeMap::new(),
        &with_hidden_target,
    ));
    assert!(target_preset_for_upgrade(&with_hidden_target, "gpt-5.4").is_none());
}

#[tokio::test]
async fn model_migration_prompt_shows_for_hidden_model() {
    let codex_home = tempdir().expect("temp codex home");
    let config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .build()
        .await
        .expect("config");

    let mut available_models = all_model_presets();
    let current = available_models
        .iter_mut()
        .find(|preset| preset.model == "gpt-5.3-codex")
        .expect("gpt-5.3-codex preset present");
    current.show_in_picker = false;
    let current = current.clone();
    assert!(
        !current.show_in_picker,
        "expected gpt-5.3-codex to be hidden from picker for this test"
    );

    let upgrade = current.upgrade.as_ref().expect("upgrade configured");
    available_models
        .iter_mut()
        .find(|preset| preset.model == upgrade.id)
        .expect("upgrade target present")
        .show_in_picker = true;
    assert!(
        should_show_model_migration_prompt(
            &current.model,
            &upgrade.id,
            &config.notices.model_migrations,
            &available_models,
        ),
        "expected migration prompt to be eligible for hidden model"
    );

    let target =
        target_preset_for_upgrade(&available_models, &upgrade.id).expect("upgrade target present");
    let target_description = (!target.description.is_empty()).then(|| target.description.clone());
    let can_opt_out = true;
    let copy = migration_copy_for_models(
        &current.model,
        &upgrade.id,
        upgrade.model_link.clone(),
        upgrade.upgrade_copy.clone(),
        upgrade.migration_markdown.clone(),
        target.display_name.clone(),
        target_description,
        can_opt_out,
    );

    assert_snapshot!(
        "model_migration_prompt_shows_for_hidden_model",
        model_migration_copy_to_plain_text(&copy)
    );
}
