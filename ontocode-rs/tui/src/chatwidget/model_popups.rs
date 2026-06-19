//! Model, collaboration, and reasoning popups for `ChatWidget`.
//!
//! These surfaces are tightly related because changing one often redirects
//! into another, especially while Plan mode is active.

use super::*;
use ontocode_model_provider_info::GEMINI_PROVIDER_ID;
use ontocode_model_provider_info::OPENAI_PROVIDER_ID;

const DEFAULT_ROUTE_SUBTITLE: &str = "Global model/provider selection is the default route; sub-agent roles may use their own provider route.";

impl ChatWidget {
    /// Open a popup to choose a quick auto model. Selecting "All models"
    /// opens the full picker with every available preset.
    pub(crate) fn open_model_popup(&mut self) {
        if !self.is_session_configured() {
            self.add_info_message(
                "Model selection is disabled until startup completes.".to_string(),
                /*hint*/ None,
            );
            return;
        }

        let presets: Vec<ModelPreset> = match self.model_catalog.try_list_models() {
            Ok(models) => models,
            Err(_) => {
                self.add_info_message(
                    "Models are being updated; please try /model again in a moment.".to_string(),
                    /*hint*/ None,
                );
                return;
            }
        };
        if let Ok(provider_groups) = self.model_catalog.try_list_provider_groups()
            && provider_groups.len() > 1
        {
            self.open_grouped_model_popup(provider_groups);
            return;
        }
        self.open_model_popup_with_presets(presets);
    }

    fn model_menu_header(&self, title: &str, subtitle: &str) -> Box<dyn Renderable> {
        let title = title.to_string();
        let subtitle = subtitle.to_string();
        let mut header = ColumnRenderable::new();
        header.push(Line::from(title.bold()));
        header.push(Line::from(subtitle.dim()));
        if let Some(warning) = self.model_menu_warning_line() {
            header.push(warning);
        }
        Box::new(header)
    }

    fn model_menu_warning_line(&self) -> Option<Line<'static>> {
        let base_url = self.custom_openai_base_url()?;
        let warning = format!(
            "Warning: OpenAI base URL is overridden to {base_url}. Selecting models may not be supported or work properly."
        );
        Some(Line::from(warning.red()))
    }

    fn custom_openai_base_url(&self) -> Option<String> {
        if !self.config.model_provider.is_openai() {
            return None;
        }

        let base_url = self.config.model_provider.base_url.as_ref()?;
        let trimmed = base_url.trim();
        if trimmed.is_empty() {
            return None;
        }

        let normalized = trimmed.trim_end_matches('/');
        if normalized == DEFAULT_OPENAI_BASE_URL {
            return None;
        }

        Some(trimmed.to_string())
    }

    pub(crate) fn open_model_popup_with_presets(&mut self, presets: Vec<ModelPreset>) {
        let presets: Vec<ModelPreset> = presets
            .into_iter()
            .filter(|preset| {
                preset.show_in_picker && self.model_provider_supports_model(&preset.model)
            })
            .collect();

        let current_model = self.current_model();
        let current_label = presets
            .iter()
            .find(|preset| preset.model.as_str() == current_model)
            .map(|preset| preset.model.to_string())
            .unwrap_or_else(|| self.model_display_name().to_string());

        let (mut auto_presets, other_presets): (Vec<ModelPreset>, Vec<ModelPreset>) = presets
            .into_iter()
            .partition(|preset| Self::is_auto_model(&preset.model));

        if auto_presets.is_empty() {
            self.open_all_models_popup(other_presets);
            return;
        }

        auto_presets.sort_by_key(|preset| Self::auto_model_order(&preset.model));
        let mut items: Vec<SelectionItem> = auto_presets
            .into_iter()
            .map(|preset| {
                let description =
                    (!preset.description.is_empty()).then_some(preset.description.clone());
                let model = preset.model.clone();
                let should_prompt_plan_mode_scope = self.should_prompt_plan_mode_reasoning_scope(
                    model.as_str(),
                    Some(preset.default_reasoning_effort),
                );
                let actions = Self::model_selection_actions(
                    model.clone(),
                    None,
                    Some(preset.default_reasoning_effort),
                    should_prompt_plan_mode_scope,
                );
                SelectionItem {
                    name: model.clone(),
                    description,
                    is_current: model.as_str() == current_model,
                    is_default: preset.is_default,
                    actions,
                    dismiss_on_select: true,
                    ..Default::default()
                }
            })
            .collect();

        if !other_presets.is_empty() {
            let all_models = other_presets;
            let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                tx.send(AppEvent::OpenAllModelsPopup {
                    models: all_models.clone(),
                });
            })];

            let is_current = !items.iter().any(|item| item.is_current);
            let description = Some(format!(
                "Choose a specific model and reasoning level (current: {current_label})"
            ));

            items.push(SelectionItem {
                name: "All models".to_string(),
                description,
                is_current,
                actions,
                dismiss_on_select: true,
                ..Default::default()
            });
        }

        let header = self.model_menu_header(
            "Select Model",
            &format!("Pick a quick auto mode or browse all models. {DEFAULT_ROUTE_SUBTITLE}"),
        );
        self.bottom_pane.show_selection_view(SelectionViewParams {
            footer_hint: Some(standard_popup_hint_line()),
            items,
            header,
            ..Default::default()
        });
    }

    fn is_auto_model(model: &str) -> bool {
        model.starts_with("codex-auto-")
    }

    fn auto_model_order(model: &str) -> usize {
        match model {
            "codex-auto-fast" => 0,
            "codex-auto-balanced" => 1,
            "codex-auto-thorough" => 2,
            _ => 3,
        }
    }

    pub(crate) fn open_all_models_popup(&mut self, presets: Vec<ModelPreset>) {
        if presets.is_empty() {
            self.add_info_message(
                "No additional models are available right now.".to_string(),
                /*hint*/ None,
            );
            return;
        }

        let mut items: Vec<SelectionItem> = Vec::new();
        for preset in presets.into_iter() {
            let description =
                (!preset.description.is_empty()).then_some(preset.description.to_string());
            let is_current = preset.model.as_str() == self.current_model();
            let single_supported_effort = preset.supported_reasoning_efforts.len() == 1;
            let preset_for_action = preset.clone();
            let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                let preset_for_event = preset_for_action.clone();
                tx.send(AppEvent::OpenReasoningPopup {
                    model: preset_for_event,
                    model_provider: None,
                });
            })];
            items.push(SelectionItem {
                name: preset.model.clone(),
                description,
                is_current,
                is_default: preset.is_default,
                actions,
                dismiss_on_select: single_supported_effort,
                dismiss_parent_on_child_accept: !single_supported_effort,
                ..Default::default()
            });
        }

        let header = self.model_menu_header(
            "Select Model and Effort",
            "Access legacy models by running codex -m <model_name> or in your config.toml",
        );
        self.bottom_pane.show_selection_view(SelectionViewParams {
            footer_hint: Some(self.bottom_pane.standard_popup_hint_line()),
            items,
            header,
            ..Default::default()
        });
    }

    fn open_grouped_model_popup(
        &mut self,
        provider_groups: Vec<crate::model_catalog::ProviderModelGroup>,
    ) {
        let current_model = self.current_model().to_string();
        let current_provider = self.config.model_provider_id.clone();
        let mut items: Vec<SelectionItem> = Vec::new();

        for group in provider_groups {
            let visible_models = group
                .models
                .into_iter()
                .filter(|preset| preset.show_in_picker)
                .collect::<Vec<_>>();
            if visible_models.is_empty() && group.disabled_reason.is_none() {
                continue;
            }

            items.push(SelectionItem {
                name: group.display_name.clone(),
                description: Some(group.provider_id.clone()),
                is_disabled: true,
                disabled_reason: group.disabled_reason.clone(),
                search_value: Some(format!("{} {}", group.display_name, group.provider_id)),
                ..Default::default()
            });

            let group_disabled_reason = group.disabled_reason.clone();
            for preset in visible_models {
                let description =
                    (!preset.description.is_empty()).then_some(preset.description.to_string());
                let is_current =
                    group.provider_id == current_provider && preset.model == current_model;
                let single_supported_effort = preset.supported_reasoning_efforts.len() == 1;
                let preset_for_action = preset.clone();
                let model_provider =
                    (group.provider_id != current_provider).then_some(group.provider_id.clone());
                let actions: Vec<SelectionAction> = if group_disabled_reason.is_some() {
                    Vec::new()
                } else {
                    vec![Box::new(move |tx| {
                        tx.send(AppEvent::OpenReasoningPopup {
                            model: preset_for_action.clone(),
                            model_provider: model_provider.clone(),
                        });
                    })]
                };

                items.push(SelectionItem {
                    name: preset.model.clone(),
                    description,
                    is_current,
                    is_default: preset.is_default,
                    is_disabled: group_disabled_reason.is_some(),
                    disabled_reason: group_disabled_reason.clone(),
                    actions,
                    dismiss_on_select: single_supported_effort && group_disabled_reason.is_none(),
                    dismiss_parent_on_child_accept: !single_supported_effort
                        && group_disabled_reason.is_none(),
                    ..Default::default()
                });
            }
        }

        if items.is_empty() {
            self.open_model_popup_with_presets(Vec::new());
            return;
        }

        let header = self.model_menu_header(
            "Select Model",
            &format!("Choose a model from a configured provider. {DEFAULT_ROUTE_SUBTITLE}"),
        );
        self.bottom_pane.show_selection_view(SelectionViewParams {
            footer_hint: Some(self.bottom_pane.standard_popup_hint_line()),
            items,
            header,
            is_searchable: true,
            ..Default::default()
        });
    }

    fn model_selection_actions(
        model_for_action: String,
        model_provider_for_action: Option<String>,
        effort_for_action: Option<ReasoningEffortConfig>,
        should_prompt_plan_mode_scope: bool,
    ) -> Vec<SelectionAction> {
        vec![Box::new(move |tx| {
            if should_prompt_plan_mode_scope {
                tx.send(AppEvent::OpenPlanReasoningScopePrompt {
                    model: model_for_action.clone(),
                    effort: effort_for_action,
                });
                return;
            }

            tx.send(AppEvent::UpdateModel(model_for_action.clone()));
            tx.send(AppEvent::UpdateReasoningEffort(effort_for_action));
            tx.send(AppEvent::PersistModelSelection(
                crate::app_event::ModelSelection {
                    model: model_for_action.clone(),
                    model_provider: model_provider_for_action.clone(),
                    effort: effort_for_action,
                },
            ));
        })]
    }

    fn should_prompt_plan_mode_reasoning_scope(
        &self,
        selected_model: &str,
        selected_effort: Option<ReasoningEffortConfig>,
    ) -> bool {
        if !self.collaboration_modes_enabled()
            || self.active_mode_kind() != ModeKind::Plan
            || selected_model != self.current_model()
        {
            return false;
        }

        // Prompt whenever the selection is not a true no-op for both:
        // 1) the active Plan-mode effective reasoning, and
        // 2) the stored global defaults that would be updated by the fallback path.
        selected_effort != self.effective_reasoning_effort()
            || selected_model != self.current_collaboration_mode.model()
            || selected_effort != self.current_collaboration_mode.reasoning_effort()
    }

    pub(crate) fn open_plan_reasoning_scope_prompt(
        &mut self,
        model: String,
        effort: Option<ReasoningEffortConfig>,
    ) {
        let reasoning_phrase = match effort {
            Some(ReasoningEffortConfig::None) => "no reasoning".to_string(),
            Some(selected_effort) => {
                format!(
                    "{} reasoning",
                    Self::reasoning_effort_label(selected_effort).to_lowercase()
                )
            }
            None => "the selected reasoning".to_string(),
        };
        let plan_only_description = format!("Always use {reasoning_phrase} in Plan mode.");
        let plan_reasoning_source = if let Some(plan_override) =
            self.config.plan_mode_reasoning_effort
        {
            format!(
                "user-chosen Plan override ({})",
                Self::reasoning_effort_label(plan_override).to_lowercase()
            )
        } else if let Some(plan_mask) = collaboration_modes::plan_mask(self.model_catalog.as_ref())
        {
            match plan_mask.reasoning_effort.flatten() {
                Some(plan_effort) => format!(
                    "built-in Plan default ({})",
                    Self::reasoning_effort_label(plan_effort).to_lowercase()
                ),
                None => "built-in Plan default (no reasoning)".to_string(),
            }
        } else {
            "built-in Plan default".to_string()
        };
        let all_modes_description = format!(
            "Set the global default reasoning level and the Plan mode override. This replaces the current {plan_reasoning_source}."
        );
        let subtitle = format!("Choose where to apply {reasoning_phrase}.");

        let plan_only_actions: Vec<SelectionAction> = vec![Box::new({
            let model = model.clone();
            move |tx| {
                tx.send(AppEvent::UpdateModel(model.clone()));
                tx.send(AppEvent::UpdatePlanModeReasoningEffort(effort));
                tx.send(AppEvent::PersistPlanModeReasoningEffort(effort));
            }
        })];
        let all_modes_actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
            tx.send(AppEvent::UpdateModel(model.clone()));
            tx.send(AppEvent::UpdateReasoningEffort(effort));
            tx.send(AppEvent::UpdatePlanModeReasoningEffort(effort));
            tx.send(AppEvent::PersistPlanModeReasoningEffort(effort));
            tx.send(AppEvent::PersistModelSelection(
                crate::app_event::ModelSelection {
                    model: model.clone(),
                    model_provider: None,
                    effort,
                },
            ));
        })];

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(PLAN_MODE_REASONING_SCOPE_TITLE.to_string()),
            subtitle: Some(subtitle),
            footer_hint: Some(standard_popup_hint_line()),
            items: vec![
                SelectionItem {
                    name: PLAN_MODE_REASONING_SCOPE_PLAN_ONLY.to_string(),
                    description: Some(plan_only_description),
                    actions: plan_only_actions,
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: PLAN_MODE_REASONING_SCOPE_ALL_MODES.to_string(),
                    description: Some(all_modes_description),
                    actions: all_modes_actions,
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        self.notify(Notification::PlanModePrompt {
            title: PLAN_MODE_REASONING_SCOPE_TITLE.to_string(),
        });
    }

    /// Open a popup to choose the reasoning effort (stage 2) for the given model.
    #[cfg(test)]
    pub(crate) fn open_reasoning_popup(&mut self, preset: ModelPreset) {
        self.open_reasoning_popup_for_provider(preset, None);
    }

    pub(crate) fn open_reasoning_popup_for_provider(
        &mut self,
        preset: ModelPreset,
        model_provider: Option<String>,
    ) {
        let default_effort: ReasoningEffortConfig = preset.default_reasoning_effort;
        let supported = preset.supported_reasoning_efforts;
        let in_plan_mode =
            self.collaboration_modes_enabled() && self.active_mode_kind() == ModeKind::Plan;

        let warn_effort = if supported
            .iter()
            .any(|option| option.effort == ReasoningEffortConfig::XHigh)
        {
            Some(ReasoningEffortConfig::XHigh)
        } else if supported
            .iter()
            .any(|option| option.effort == ReasoningEffortConfig::High)
        {
            Some(ReasoningEffortConfig::High)
        } else {
            None
        };
        let warning_text = warn_effort.map(|effort| {
            let effort_label = Self::reasoning_effort_label(effort);
            format!("⚠ {effort_label} reasoning effort can quickly consume Plus plan rate limits.")
        });
        let warn_for_model = preset.model.starts_with("gpt-5.1-codex")
            || preset.model.starts_with("gpt-5.1-codex-max")
            || preset.model.starts_with("gpt-5.2");

        struct EffortChoice {
            stored: Option<ReasoningEffortConfig>,
            display: ReasoningEffortConfig,
        }
        let mut choices: Vec<EffortChoice> = Vec::new();
        for effort in ReasoningEffortConfig::iter() {
            if supported.iter().any(|option| option.effort == effort) {
                choices.push(EffortChoice {
                    stored: Some(effort),
                    display: effort,
                });
            }
        }
        if choices.is_empty() {
            choices.push(EffortChoice {
                stored: Some(default_effort),
                display: default_effort,
            });
        }

        if choices.len() == 1 {
            let selected_effort = choices.first().and_then(|c| c.stored);
            let selected_model = preset.model;
            if model_provider.is_none()
                && self.should_prompt_plan_mode_reasoning_scope(&selected_model, selected_effort)
            {
                self.app_event_tx
                    .send(AppEvent::OpenPlanReasoningScopePrompt {
                        model: selected_model,
                        effort: selected_effort,
                    });
            } else {
                self.apply_model_and_effort_for_provider(
                    selected_model,
                    model_provider,
                    selected_effort,
                );
            }
            return;
        }

        let default_choice: Option<ReasoningEffortConfig> = choices
            .iter()
            .any(|choice| choice.stored == Some(default_effort))
            .then_some(Some(default_effort))
            .flatten()
            .or_else(|| choices.iter().find_map(|choice| choice.stored))
            .or(Some(default_effort));

        let model_slug = preset.model.to_string();
        let is_current_model = self.current_model() == preset.model.as_str();
        let highlight_choice = if is_current_model {
            if in_plan_mode {
                self.config
                    .plan_mode_reasoning_effort
                    .or(self.effective_reasoning_effort())
            } else {
                self.effective_reasoning_effort()
            }
        } else {
            default_choice
        };
        let selection_choice = highlight_choice.or(default_choice);
        let initial_selected_idx = choices
            .iter()
            .position(|choice| choice.stored == selection_choice)
            .or_else(|| {
                selection_choice
                    .and_then(|effort| choices.iter().position(|choice| choice.display == effort))
            });
        let mut items: Vec<SelectionItem> = Vec::new();
        for choice in choices.iter() {
            let effort = choice.display;
            let mut effort_label = Self::reasoning_effort_label(effort).to_string();
            if choice.stored == default_choice {
                effort_label.push_str(" (default)");
            }

            let description = choice
                .stored
                .and_then(|effort| {
                    supported
                        .iter()
                        .find(|option| option.effort == effort)
                        .map(|option| option.description.to_string())
                })
                .filter(|text| !text.is_empty());

            let show_warning = warn_for_model && warn_effort == Some(effort);
            let selected_description = if show_warning {
                warning_text.as_ref().map(|warning_message| {
                    description.as_ref().map_or_else(
                        || warning_message.clone(),
                        |d| format!("{d}\n{warning_message}"),
                    )
                })
            } else {
                None
            };

            let model_for_action = model_slug.clone();
            let model_provider_for_action = model_provider.clone();
            let choice_effort = choice.stored;
            let should_prompt_plan_mode_scope = model_provider_for_action.is_none()
                && self.should_prompt_plan_mode_reasoning_scope(model_slug.as_str(), choice_effort);
            let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                if should_prompt_plan_mode_scope {
                    tx.send(AppEvent::OpenPlanReasoningScopePrompt {
                        model: model_for_action.clone(),
                        effort: choice_effort,
                    });
                } else {
                    tx.send(AppEvent::UpdateModel(model_for_action.clone()));
                    tx.send(AppEvent::UpdateReasoningEffort(choice_effort));
                    tx.send(AppEvent::PersistModelSelection(
                        crate::app_event::ModelSelection {
                            model: model_for_action.clone(),
                            model_provider: model_provider_for_action.clone(),
                            effort: choice_effort,
                        },
                    ));
                }
            })];

            items.push(SelectionItem {
                name: effort_label,
                description,
                selected_description,
                is_current: is_current_model && choice.stored == highlight_choice,
                actions,
                dismiss_on_select: true,
                ..Default::default()
            });
        }

        let mut header = ColumnRenderable::new();
        header.push(Line::from(
            format!("Select Reasoning Level for {model_slug}").bold(),
        ));

        self.bottom_pane.show_selection_view(SelectionViewParams {
            header: Box::new(header),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            initial_selected_idx,
            ..Default::default()
        });
    }

    pub(super) fn reasoning_effort_label(effort: ReasoningEffortConfig) -> &'static str {
        match effort {
            ReasoningEffortConfig::None => "None",
            ReasoningEffortConfig::Minimal => "Minimal",
            ReasoningEffortConfig::Low => "Low",
            ReasoningEffortConfig::Medium => "Medium",
            ReasoningEffortConfig::High => "High",
            ReasoningEffortConfig::XHigh => "Extra high",
        }
    }

    pub(super) fn apply_model_and_effort_without_persist(
        &self,
        model: String,
        effort: Option<ReasoningEffortConfig>,
    ) {
        self.app_event_tx.send(AppEvent::UpdateModel(model));
        self.app_event_tx
            .send(AppEvent::UpdateReasoningEffort(effort));
    }

    fn apply_model_and_effort_for_provider(
        &self,
        model: String,
        model_provider: Option<String>,
        effort: Option<ReasoningEffortConfig>,
    ) {
        let model_provider =
            model_provider.or_else(|| self.inferred_model_provider_for_model(&model));
        self.app_event_tx
            .send(AppEvent::UpdateReasoningEffort(effort));
        self.app_event_tx.send(AppEvent::PersistModelSelection(
            crate::app_event::ModelSelection {
                model,
                model_provider,
                effort,
            },
        ));
    }

    pub(super) fn unsupported_model_provider_message(&self, model: &str) -> Option<String> {
        if self.model_provider_supports_model(model) {
            return None;
        }

        let required_provider = if model.starts_with("gemini-") {
            "Gemini"
        } else {
            "OpenAI"
        };
        Some(format!(
            "Model {model} requires the {required_provider} provider. Select the matching provider/model pair from /model or start a new thread after saving that selection."
        ))
    }

    fn model_provider_supports_model(&self, model: &str) -> bool {
        if model.starts_with("gemini-") {
            return self.config.model_provider_id == GEMINI_PROVIDER_ID;
        }
        self.config.model_provider_id != GEMINI_PROVIDER_ID
    }

    fn inferred_model_provider_for_model(&self, model: &str) -> Option<String> {
        if model.starts_with("gemini-") {
            return Some(GEMINI_PROVIDER_ID.to_string());
        }
        if model.starts_with("gpt-") {
            return Some(OPENAI_PROVIDER_ID.to_string());
        }

        if self.config.model_provider_id != GEMINI_PROVIDER_ID {
            return None;
        }

        let groups = self.model_catalog.try_list_provider_groups().ok()?;
        let provider_id = groups
            .into_iter()
            .filter(|group| group.disabled_reason.is_none())
            .find(|group| {
                group.provider_id != self.config.model_provider_id
                    && group
                        .models
                        .iter()
                        .any(|preset| preset.show_in_picker && preset.model == model)
            })
            .map(|group| group.provider_id);
        provider_id.or_else(|| {
            model
                .starts_with("gpt-")
                .then(|| OPENAI_PROVIDER_ID.to_string())
        })
    }
}
