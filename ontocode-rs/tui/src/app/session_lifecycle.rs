//! Session, resume, fork, and subagent selection lifecycle for the TUI app.
//!
//! This module owns the high-level transitions between app-server threads: starting fresh sessions,
//! resuming/forking saved sessions, replacing ChatWidget instances, and maintaining the agent picker
//! cache used for multi-agent navigation.

use super::*;

#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentDefinitionOptionalFields {
    model: Option<String>,
    model_reasoning_effort: Option<ontocode_protocol::openai_models::ReasoningEffort>,
    service_tier: Option<String>,
    nickname_candidates: Option<Vec<String>>,
}

fn normalize_agent_definition_optional_string(
    field_label: &str,
    value: Option<String>,
) -> color_eyre::Result<Option<String>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value.trim();
    if value.is_empty() {
        color_eyre::eyre::bail!("{field_label} cannot be blank.");
    }
    Ok(Some(value.to_string()))
}

fn normalize_agent_definition_nickname_candidates(
    nickname_candidates: Option<Vec<String>>,
) -> color_eyre::Result<Option<Vec<String>>> {
    let Some(nickname_candidates) = nickname_candidates else {
        return Ok(None);
    };

    if nickname_candidates.is_empty() {
        color_eyre::eyre::bail!("nickname_candidates must contain at least one name");
    }

    let mut normalized_candidates = Vec::with_capacity(nickname_candidates.len());
    let mut seen_candidates = std::collections::BTreeSet::new();

    for nickname in nickname_candidates {
        let normalized_nickname = nickname.trim();
        if normalized_nickname.is_empty() {
            color_eyre::eyre::bail!("nickname_candidates cannot contain blank names");
        }

        if !seen_candidates.insert(normalized_nickname.to_string()) {
            color_eyre::eyre::bail!("nickname_candidates cannot contain duplicates");
        }

        if !normalized_nickname
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_'))
        {
            color_eyre::eyre::bail!(
                "nickname_candidates may only contain ASCII letters, digits, spaces, hyphens, and underscores"
            );
        }

        normalized_candidates.push(normalized_nickname.to_string());
    }

    Ok(Some(normalized_candidates))
}

fn parse_agent_definition_optional_fields(
    raw_optional_fields: &str,
) -> color_eyre::Result<AgentDefinitionOptionalFields> {
    let mut fields: AgentDefinitionOptionalFields = toml::from_str(raw_optional_fields)
        .wrap_err("parse optional agent definition fields as TOML")?;
    fields.model = normalize_agent_definition_optional_string("model", fields.model)?;
    fields.service_tier =
        normalize_agent_definition_optional_string("service_tier", fields.service_tier)?;
    fields.nickname_candidates =
        normalize_agent_definition_nickname_candidates(fields.nickname_candidates)?;
    Ok(fields)
}

#[derive(Debug, PartialEq, Eq)]
struct AgentDefinitionProposalScaffold {
    name: String,
    description: String,
    developer_instructions: String,
}

fn parse_agent_definition_proposal(
    raw_proposal: &str,
) -> color_eyre::Result<AgentDefinitionProposalScaffold> {
    let lines = raw_proposal
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let Some(raw_name) = lines.first().copied() else {
        color_eyre::eyre::bail!("Proposal must include a role name on the first line.");
    };
    let Some(name) = crate::legacy_core::util::normalize_thread_name(raw_name) else {
        color_eyre::eyre::bail!("Proposal must include a non-empty role name on the first line.");
    };
    let body_lines = lines.iter().skip(1).copied().collect::<Vec<_>>();
    let body_lines = if body_lines.is_empty() {
        vec![raw_name]
    } else {
        body_lines
    };
    let description = body_lines[0].to_string();
    let developer_instructions = body_lines.join("\n");
    Ok(AgentDefinitionProposalScaffold {
        name,
        description,
        developer_instructions,
    })
}

impl App {
    pub(super) async fn open_agent_picker(&mut self, app_server: &mut AppServerSession) {
        let mut thread_ids = self.agent_navigation.tracked_thread_ids();
        for thread_id in self.thread_event_channels.keys().copied() {
            if !thread_ids.contains(&thread_id) {
                thread_ids.push(thread_id);
            }
        }
        for thread_id in thread_ids {
            if self.side_threads.contains_key(&thread_id) {
                continue;
            }
            if !self
                .refresh_agent_picker_thread_liveness(app_server, thread_id)
                .await
            {
                continue;
            }
        }

        let has_non_primary_agent_thread = self
            .agent_navigation
            .has_non_primary_thread(self.primary_thread_id);
        if !self.config.features.enabled(Feature::Collab) && !has_non_primary_agent_thread {
            self.chat_widget.open_multi_agent_enable_prompt();
            return;
        }

        let has_agent_roles = !self.config.agent_roles.is_empty();
        let project_root = get_git_repo_root(self.config.cwd.as_path())
            .unwrap_or_else(|| self.config.cwd.to_path_buf());
        let repo_local_agents_dir = project_root.join(".codex").join("agents");

        let mut initial_selected_idx = None;
        let mut items: Vec<SelectionItem> = vec![
            SelectionItem {
                name: "Create from proposal".to_string(),
                description: Some(
                    "Write a repo-local scaffold from one freeform role proposal.".to_string(),
                ),
                actions: vec![Box::new(move |tx| {
                    tx.send(AppEvent::OpenCreateAgentDefinitionProposalPrompt);
                })],
                dismiss_on_select: false,
                dismiss_parent_on_child_accept: true,
                search_value: Some("create agent definition from proposal role prompt".to_string()),
                ..Default::default()
            },
            SelectionItem {
                name: "Create agent definition".to_string(),
                description: Some(
                    "Write a repo-local .codex/agents/<slug>.toml scaffold.".to_string(),
                ),
                actions: vec![Box::new(move |tx| {
                    tx.send(AppEvent::OpenCreateAgentDefinitionPrompt);
                })],
                dismiss_on_select: false,
                dismiss_parent_on_child_accept: true,
                search_value: Some("create agent definition role scaffold".to_string()),
                ..Default::default()
            },
        ];
        for (thread_id, entry) in self.agent_navigation.ordered_threads() {
            if self.active_thread_id == Some(thread_id) {
                initial_selected_idx = Some(items.len());
            }
            let is_primary = self.primary_thread_id == Some(thread_id);
            let name = format_agent_picker_item_name(
                entry.agent_nickname.as_deref(),
                entry.agent_role.as_deref(),
                is_primary,
            );
            let uuid = thread_id.to_string();
            items.push(SelectionItem {
                name: name.clone(),
                name_prefix_spans: agent_picker_status_dot_spans(entry.is_closed),
                description: Some(uuid.clone()),
                is_current: self.active_thread_id == Some(thread_id),
                actions: vec![Box::new(move |tx| {
                    tx.send(AppEvent::SelectAgentThread(thread_id));
                })],
                dismiss_on_select: true,
                search_value: Some(format!("{name} {uuid}")),
                ..Default::default()
            });

            if !is_primary {
                let rename_name = name.clone();
                items.push(SelectionItem {
                    name: format!("Rename {rename_name}"),
                    description: Some(
                        "Change only the visible label for this live thread.".to_string(),
                    ),
                    actions: vec![Box::new(move |tx| {
                        tx.send(AppEvent::OpenRenameAgentThreadPrompt { thread_id });
                    })],
                    dismiss_parent_on_child_accept: true,
                    search_value: Some(format!("rename {rename_name} {uuid}")),
                    ..Default::default()
                });
                let delete_name = name.clone();
                items.push(SelectionItem {
                    name: format!("Delete {delete_name}"),
                    description: Some(
                        "Remove this live thread from the current session only.".to_string(),
                    ),
                    actions: vec![Box::new(move |tx| {
                        tx.send(AppEvent::DeleteAgentThread { thread_id });
                    })],
                    dismiss_on_select: true,
                    search_value: Some(format!("delete remove hide {delete_name} {uuid}")),
                    ..Default::default()
                });
            }
        }

        if has_agent_roles {
            items.push(SelectionItem {
                name: "Available role definitions".to_string(),
                is_disabled: true,
                ..Default::default()
            });

            for (role_name, role_config) in &self.config.agent_roles {
                let mut search_value = role_name.clone();
                if let Some(description) = role_config.description.as_deref() {
                    search_value.push(' ');
                    search_value.push_str(description);
                }

                items.push(SelectionItem {
                    name: format!("{role_name} [role]"),
                    description: role_config.description.clone(),
                    is_disabled: true,
                    search_value: Some(search_value),
                    ..Default::default()
                });

                if let Some(config_file) = role_config
                    .config_file
                    .as_ref()
                    .filter(|path| path.starts_with(&repo_local_agents_dir))
                {
                    let role_name = role_name.clone();
                    let source_path = config_file.clone();
                    let search_value = format!("rename role definition {role_name}");
                    let rename_role_name = role_name.clone();
                    let rename_source_path = source_path.clone();
                    items.push(SelectionItem {
                        name: format!("Rename {role_name} [role]"),
                        description: Some(
                            "Move this repo-local role definition to a new name.".to_string(),
                        ),
                        actions: vec![Box::new(move |tx| {
                            tx.send(AppEvent::OpenRenameAgentDefinitionPrompt {
                                source_path: rename_source_path.clone(),
                                role_name: rename_role_name.clone(),
                            });
                        })],
                        dismiss_parent_on_child_accept: true,
                        search_value: Some(search_value),
                        ..Default::default()
                    });

                    let role_name = role_name.clone();
                    let source_path = config_file.clone();
                    let search_value = format!("copy duplicate role definition {role_name}");
                    let copy_role_name = role_name.clone();
                    items.push(SelectionItem {
                        name: format!("Copy {role_name} [role]"),
                        description: Some(
                            "Duplicate this repo-local role definition into a new file."
                                .to_string(),
                        ),
                        actions: vec![Box::new(move |tx| {
                            tx.send(AppEvent::OpenCopyAgentDefinitionPrompt {
                                source_path: source_path.clone(),
                                role_name: copy_role_name.clone(),
                            });
                        })],
                        dismiss_parent_on_child_accept: true,
                        search_value: Some(search_value),
                        ..Default::default()
                    });

                    let role_name = role_name.clone();
                    let source_path = config_file.clone();
                    let search_value = format!("delete remove role definition {role_name}");
                    items.push(SelectionItem {
                        name: format!("Delete {role_name} [role]"),
                        description: Some(
                            "Remove this repo-local role definition file.".to_string(),
                        ),
                        actions: vec![Box::new(move |tx| {
                            tx.send(AppEvent::OpenDeleteAgentDefinitionPrompt {
                                source_path: source_path.clone(),
                                role_name: role_name.clone(),
                            });
                        })],
                        dismiss_parent_on_child_accept: true,
                        search_value: Some(search_value),
                        ..Default::default()
                    });
                }
            }
        }

        self.chat_widget.show_selection_view(SelectionViewParams {
            title: Some("Subagents".to_string()),
            subtitle: Some(AgentNavigationState::picker_subtitle()),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            initial_selected_idx,
            ..Default::default()
        });
    }

    pub(super) fn create_agent_definition_from_proposal_scaffold(
        &mut self,
        raw_proposal: &str,
    ) -> color_eyre::Result<PathBuf> {
        let scaffold = parse_agent_definition_proposal(raw_proposal)?;
        let project_root = get_git_repo_root(self.config.cwd.as_path())
            .unwrap_or_else(|| self.config.cwd.to_path_buf());
        let slug = slugify_agent_definition_name(&scaffold.name);
        let agents_dir = project_root.join(".codex").join("agents");
        let role_path = agents_dir.join(format!("{slug}.toml"));
        if role_path.exists() {
            color_eyre::eyre::bail!("Agent definition already exists: {}", role_path.display());
        }
        std::fs::create_dir_all(&agents_dir)
            .wrap_err_with(|| format!("create {}", agents_dir.display()))?;
        let scaffold = format!(
            "name = {:?}\n\
description = {:?}\n\
developer_instructions = \"\"\"\n\
{}\n\
\"\"\"\n",
            scaffold.name, scaffold.description, scaffold.developer_instructions
        );
        std::fs::write(&role_path, scaffold)
            .wrap_err_with(|| format!("write {}", role_path.display()))?;
        Ok(role_path)
    }

    pub(super) fn create_agent_definition_scaffold(
        &mut self,
        raw_name: &str,
        raw_optional_fields: &str,
    ) -> color_eyre::Result<PathBuf> {
        let Some(name) = crate::legacy_core::util::normalize_thread_name(raw_name) else {
            color_eyre::eyre::bail!("Agent definition name cannot be empty.");
        };
        let optional_fields = parse_agent_definition_optional_fields(raw_optional_fields)?;
        let project_root = get_git_repo_root(self.config.cwd.as_path())
            .unwrap_or_else(|| self.config.cwd.to_path_buf());
        let slug = slugify_agent_definition_name(&name);
        let agents_dir = project_root.join(".codex").join("agents");
        let role_path = agents_dir.join(format!("{slug}.toml"));
        if role_path.exists() {
            color_eyre::eyre::bail!("Agent definition already exists: {}", role_path.display());
        }
        std::fs::create_dir_all(&agents_dir)
            .wrap_err_with(|| format!("create {}", agents_dir.display()))?;
        let mut scaffold_lines = vec![
            format!("name = {name:?}"),
            format!("description = {name:?}"),
        ];
        if let Some(model) = optional_fields.model {
            scaffold_lines.push(format!("model = {}", toml::Value::String(model)));
        }
        if let Some(model_reasoning_effort) = optional_fields.model_reasoning_effort {
            scaffold_lines.push(format!(
                "model_reasoning_effort = {}",
                toml::Value::String(model_reasoning_effort.to_string())
            ));
        }
        if let Some(service_tier) = optional_fields.service_tier {
            scaffold_lines.push(format!(
                "service_tier = {}",
                toml::Value::String(service_tier)
            ));
        }
        if let Some(nickname_candidates) = optional_fields.nickname_candidates {
            let nickname_candidates = toml::Value::Array(
                nickname_candidates
                    .into_iter()
                    .map(toml::Value::String)
                    .collect(),
            );
            scaffold_lines.push(format!("nickname_candidates = {nickname_candidates}"));
        }
        scaffold_lines.push("developer_instructions = \"\"\"".to_string());
        scaffold_lines.push("Fill in the instructions for this role.".to_string());
        scaffold_lines.push("\"\"\"".to_string());
        let scaffold = format!("{}\n", scaffold_lines.join("\n"));
        std::fs::write(&role_path, scaffold)
            .wrap_err_with(|| format!("write {}", role_path.display()))?;
        Ok(role_path)
    }

    pub(super) fn copy_agent_definition_scaffold(
        &mut self,
        source_path: &Path,
        raw_name: &str,
    ) -> color_eyre::Result<PathBuf> {
        let Some(name) = crate::legacy_core::util::normalize_thread_name(raw_name) else {
            color_eyre::eyre::bail!("Agent definition name cannot be empty.");
        };
        let project_root = get_git_repo_root(self.config.cwd.as_path())
            .unwrap_or_else(|| self.config.cwd.to_path_buf());
        let agents_dir = project_root.join(".codex").join("agents");
        let canonical_source = source_path
            .canonicalize()
            .wrap_err_with(|| format!("read {}", source_path.display()))?;
        let canonical_agents_dir = agents_dir
            .canonicalize()
            .unwrap_or_else(|_| agents_dir.clone());
        if !canonical_source.starts_with(&canonical_agents_dir) {
            color_eyre::eyre::bail!(
                "Only repo-local agent definitions under {} can be copied.",
                agents_dir.display()
            );
        }

        let slug = slugify_agent_definition_name(&name);
        let role_path = agents_dir.join(format!("{slug}.toml"));
        if role_path.exists() {
            color_eyre::eyre::bail!("Agent definition already exists: {}", role_path.display());
        }

        let source = std::fs::read_to_string(&canonical_source)
            .wrap_err_with(|| format!("read {}", canonical_source.display()))?;
        let mut wrote_name = false;
        let mut copied = source
            .lines()
            .map(|line| {
                if !wrote_name && line.trim_start().starts_with("name =") {
                    wrote_name = true;
                    format!("name = {name:?}")
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>();
        if !wrote_name {
            copied.insert(0, format!("name = {name:?}"));
        }

        std::fs::write(&role_path, format!("{}\n", copied.join("\n")))
            .wrap_err_with(|| format!("write {}", role_path.display()))?;
        Ok(role_path)
    }

    pub(super) fn rename_agent_definition_scaffold(
        &mut self,
        source_path: &Path,
        raw_name: &str,
    ) -> color_eyre::Result<PathBuf> {
        let Some(name) = crate::legacy_core::util::normalize_thread_name(raw_name) else {
            color_eyre::eyre::bail!("Agent definition name cannot be empty.");
        };
        let project_root = get_git_repo_root(self.config.cwd.as_path())
            .unwrap_or_else(|| self.config.cwd.to_path_buf());
        let agents_dir = project_root.join(".codex").join("agents");
        let canonical_source = source_path
            .canonicalize()
            .wrap_err_with(|| format!("read {}", source_path.display()))?;
        let canonical_agents_dir = agents_dir
            .canonicalize()
            .unwrap_or_else(|_| agents_dir.clone());
        if !canonical_source.starts_with(&canonical_agents_dir) {
            color_eyre::eyre::bail!(
                "Only repo-local agent definitions under {} can be renamed.",
                agents_dir.display()
            );
        }

        let slug = slugify_agent_definition_name(&name);
        let role_path = agents_dir.join(format!("{slug}.toml"));
        let target_is_source = role_path
            .canonicalize()
            .map(|path| path == canonical_source)
            .unwrap_or(false);
        if role_path.exists() && !target_is_source {
            color_eyre::eyre::bail!("Agent definition already exists: {}", role_path.display());
        }

        let source = std::fs::read_to_string(&canonical_source)
            .wrap_err_with(|| format!("read {}", canonical_source.display()))?;
        let mut wrote_name = false;
        let mut renamed = source
            .lines()
            .map(|line| {
                if !wrote_name && line.trim_start().starts_with("name =") {
                    wrote_name = true;
                    format!("name = {name:?}")
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>();
        if !wrote_name {
            renamed.insert(0, format!("name = {name:?}"));
        }

        std::fs::write(&role_path, format!("{}\n", renamed.join("\n")))
            .wrap_err_with(|| format!("write {}", role_path.display()))?;
        if !target_is_source {
            std::fs::remove_file(&canonical_source)
                .wrap_err_with(|| format!("remove {}", canonical_source.display()))?;
        }
        Ok(role_path)
    }

    pub(super) fn delete_agent_definition_scaffold(
        &mut self,
        source_path: &Path,
        confirmation: &str,
    ) -> color_eyre::Result<PathBuf> {
        if confirmation.trim() != "DELETE" {
            color_eyre::eyre::bail!("Type DELETE to confirm agent definition removal.");
        }

        let project_root = get_git_repo_root(self.config.cwd.as_path())
            .unwrap_or_else(|| self.config.cwd.to_path_buf());
        let agents_dir = project_root.join(".codex").join("agents");
        let canonical_source = source_path
            .canonicalize()
            .wrap_err_with(|| format!("read {}", source_path.display()))?;
        let canonical_agents_dir = agents_dir
            .canonicalize()
            .unwrap_or_else(|_| agents_dir.clone());
        if !canonical_source.starts_with(&canonical_agents_dir) {
            color_eyre::eyre::bail!(
                "Only repo-local agent definitions under {} can be deleted.",
                agents_dir.display()
            );
        }

        std::fs::remove_file(&canonical_source)
            .wrap_err_with(|| format!("remove {}", canonical_source.display()))?;
        Ok(canonical_source)
    }

    pub(super) fn is_terminal_thread_read_error(err: &color_eyre::Report) -> bool {
        err.chain()
            .any(|cause| cause.to_string().contains("thread not loaded:"))
    }

    pub(super) fn closed_state_for_thread_read_error(
        err: &color_eyre::Report,
        existing_is_closed: Option<bool>,
    ) -> bool {
        Self::is_terminal_thread_read_error(err) || existing_is_closed.unwrap_or(false)
    }

    pub(super) fn can_fallback_from_include_turns_error(err: &color_eyre::Report) -> bool {
        err.chain().any(|cause| {
            let message = cause.to_string();
            message.contains("includeTurns is unavailable before first user message")
                || message.contains("ephemeral threads do not support includeTurns")
        })
    }

    /// Updates cached picker metadata and then mirrors any visible-label change into the footer.
    ///
    /// These two writes stay paired so the picker rows and contextual footer continue to describe
    /// the same displayed thread after nickname or role updates.
    pub(super) fn upsert_agent_picker_thread(
        &mut self,
        thread_id: ThreadId,
        agent_nickname: Option<String>,
        agent_role: Option<String>,
        is_closed: bool,
    ) {
        self.chat_widget.set_collab_agent_metadata(
            thread_id,
            agent_nickname.clone(),
            agent_role.clone(),
        );
        self.agent_navigation
            .upsert(thread_id, agent_nickname, agent_role, is_closed);
        self.sync_active_agent_label();
    }

    /// Marks a cached picker thread closed and recomputes the contextual footer label.
    ///
    /// Closing a thread is not the same as removing it: users can still inspect finished agent
    /// transcripts, and the stable next/previous traversal order should not collapse around them.
    pub(super) fn mark_agent_picker_thread_closed(&mut self, thread_id: ThreadId) {
        self.agent_navigation.mark_closed(thread_id);
        self.sync_active_agent_label();
    }

    pub(super) fn rename_agent_picker_thread_label(
        &mut self,
        thread_id: ThreadId,
        raw_name: &str,
    ) -> color_eyre::Result<()> {
        if self.primary_thread_id == Some(thread_id) {
            color_eyre::eyre::bail!("The main thread cannot be renamed from /agent.");
        }
        if self.agent_navigation.get(&thread_id).is_none() {
            color_eyre::eyre::bail!("Unknown agent thread: {thread_id}");
        }
        let Some(name) = crate::legacy_core::util::normalize_thread_name(raw_name) else {
            color_eyre::eyre::bail!("Agent label cannot be empty.");
        };
        let Some(existing_entry) = self.agent_navigation.get(&thread_id).cloned() else {
            color_eyre::eyre::bail!("Unknown agent thread: {thread_id}");
        };
        self.upsert_agent_picker_thread(
            thread_id,
            Some(name),
            existing_entry.agent_role,
            existing_entry.is_closed,
        );
        Ok(())
    }

    pub(super) fn can_manage_agent_picker_thread(
        &self,
        thread_id: ThreadId,
    ) -> color_eyre::Result<()> {
        if self.primary_thread_id == Some(thread_id) {
            color_eyre::eyre::bail!("The main thread cannot be managed from /agent.");
        }
        if self.agent_navigation.get(&thread_id).is_none() {
            color_eyre::eyre::bail!("Unknown agent thread: {thread_id}");
        }
        Ok(())
    }

    pub(super) async fn refresh_agent_picker_thread_liveness(
        &mut self,
        app_server: &mut AppServerSession,
        thread_id: ThreadId,
    ) -> bool {
        let existing_entry = self.agent_navigation.get(&thread_id).cloned();
        let has_replay_channel = self.thread_event_channels.contains_key(&thread_id);
        match app_server
            .thread_read(thread_id, /*include_turns*/ false)
            .await
        {
            Ok(thread) => {
                self.upsert_agent_picker_thread(
                    thread_id,
                    thread.agent_nickname.or_else(|| {
                        existing_entry
                            .as_ref()
                            .and_then(|entry| entry.agent_nickname.clone())
                    }),
                    thread.agent_role.or_else(|| {
                        existing_entry
                            .as_ref()
                            .and_then(|entry| entry.agent_role.clone())
                    }),
                    matches!(
                        thread.status,
                        ontocode_app_server_protocol::ThreadStatus::NotLoaded
                    ),
                );
                true
            }
            Err(err) => {
                if Self::is_terminal_thread_read_error(&err) && !has_replay_channel {
                    self.agent_navigation.remove(thread_id);
                    return false;
                }
                let is_closed = Self::closed_state_for_thread_read_error(
                    &err,
                    existing_entry.as_ref().map(|entry| entry.is_closed),
                );
                if let Some(entry) = existing_entry {
                    self.upsert_agent_picker_thread(
                        thread_id,
                        entry.agent_nickname,
                        entry.agent_role,
                        is_closed,
                    );
                } else {
                    self.upsert_agent_picker_thread(
                        thread_id, /*agent_nickname*/ None, /*agent_role*/ None,
                        is_closed,
                    );
                }
                true
            }
        }
    }

    /// Materializes a live thread into local replay state when the picker knows about it but the
    /// TUI has not cached a local event channel yet.
    ///
    /// Resume-time backfill intentionally avoids creating empty placeholder channels, because those
    /// placeholders make stale `/agent` entries open blank transcripts. When a user later selects a
    /// still-live discovered thread, attach it on demand with a real resumed snapshot.
    pub(super) async fn attach_live_thread_for_selection(
        &mut self,
        app_server: &mut AppServerSession,
        thread_id: ThreadId,
    ) -> Result<bool> {
        if self.thread_event_channels.contains_key(&thread_id) {
            return Ok(true);
        }

        let (session, turns, live_attached) = match app_server
            .resume_thread(self.config.clone(), thread_id)
            .await
        {
            Ok(started) => (started.session, started.turns, true),
            Err(resume_err) => {
                tracing::warn!(
                    thread_id = %thread_id,
                    error = %resume_err,
                    "failed to resume live thread for selection; falling back to thread/read"
                );
                let (thread, turns) = match app_server
                    .thread_read(thread_id, /*include_turns*/ true)
                    .await
                {
                    Ok(thread) => {
                        let turns = thread.turns.clone();
                        (thread, turns)
                    }
                    Err(err) if Self::can_fallback_from_include_turns_error(&err) => {
                        let thread = app_server
                            .thread_read(thread_id, /*include_turns*/ false)
                            .await?;
                        (thread, Vec::new())
                    }
                    Err(err) => return Err(err),
                };
                if turns.is_empty() {
                    // A `thread/read` fallback without turns would create a blank local replay
                    // channel with no live listener attached, which blocks later real re-attach.
                    return Err(color_eyre::eyre::eyre!(
                        "Agent thread {thread_id} is not yet available for replay or live attach."
                    ));
                }
                let mut session = self.session_state_for_thread_read(thread_id, &thread).await;
                // `thread/read` can seed replay state, but it does not attach the app-server
                // listener that `thread/resume` establishes, so treat this path as replay-only.
                session.model.clear();
                (session, turns, false)
            }
        };
        let channel = self.ensure_thread_channel(thread_id);
        let mut store = channel.store.lock().await;
        store.set_session(session, turns);
        Ok(live_attached)
    }

    /// Replaces the chat widget and re-seeds the new widget's collab metadata from the navigation
    /// cache.
    ///
    /// Thread switches reconstruct the `ChatWidget`, which loses the `collab_agent_metadata` map.
    /// This helper copies every known nickname/role from `AgentNavigationState` into the
    /// replacement widget so that replayed collab items render agent names immediately.
    pub(super) fn replace_chat_widget(&mut self, mut chat_widget: ChatWidget) {
        // Transfer the last-written terminal title to the replacement widget
        // so it knows what OSC title is currently displayed. Without this, the
        // new widget would redundantly clear and rewrite the same title, causing
        // a visible flicker in some terminals.
        let previous_terminal_title = self.chat_widget.last_terminal_title.take();
        if chat_widget.last_terminal_title.is_none() {
            chat_widget.last_terminal_title = previous_terminal_title;
        }
        chat_widget.remote_connection = self.chat_widget.remote_connection.clone();
        for (thread_id, entry) in self.agent_navigation.ordered_threads() {
            chat_widget.set_collab_agent_metadata(
                thread_id,
                entry.agent_nickname.clone(),
                entry.agent_role.clone(),
            );
        }
        self.chat_widget = chat_widget;
        self.sync_active_agent_label();
    }

    pub(super) async fn select_agent_thread(
        &mut self,
        tui: &mut tui::Tui,
        app_server: &mut AppServerSession,
        thread_id: ThreadId,
    ) -> Result<()> {
        if self.active_thread_id == Some(thread_id) {
            return Ok(());
        }

        if !self
            .refresh_agent_picker_thread_liveness(app_server, thread_id)
            .await
        {
            self.chat_widget
                .add_error_message(format!("Agent thread {thread_id} is no longer available."));
            return Ok(());
        }

        let mut is_replay_only = self
            .agent_navigation
            .get(&thread_id)
            .is_some_and(|entry| entry.is_closed);
        let mut attached_replay_only = false;
        if self.should_attach_live_thread_for_selection(thread_id) {
            match self
                .attach_live_thread_for_selection(app_server, thread_id)
                .await
            {
                Ok(live_attached) => {
                    attached_replay_only = !live_attached;
                    if attached_replay_only {
                        is_replay_only = true;
                    }
                }
                Err(err) => {
                    self.chat_widget.add_error_message(format!(
                        "Failed to attach to agent thread {thread_id}: {err}"
                    ));
                    return Ok(());
                }
            }
        } else if !self.thread_event_channels.contains_key(&thread_id) && is_replay_only {
            self.chat_widget
                .add_error_message(format!("Agent thread {thread_id} is no longer available."));
            return Ok(());
        }

        let previous_thread_id = self.active_thread_id;
        self.store_active_thread_receiver().await;
        self.active_thread_id = None;
        let Some((receiver, mut snapshot)) = self.activate_thread_for_replay(thread_id).await
        else {
            self.chat_widget
                .add_error_message(format!("Agent thread {thread_id} is already active."));
            if let Some(previous_thread_id) = previous_thread_id {
                self.activate_thread_channel(previous_thread_id).await;
            }
            return Ok(());
        };

        self.refresh_snapshot_session_if_needed(
            app_server,
            thread_id,
            is_replay_only,
            &mut snapshot,
        )
        .await;

        self.active_thread_id = Some(thread_id);
        self.active_thread_rx = Some(receiver);

        let init = self.chatwidget_init_for_forked_or_resumed_thread(
            tui,
            self.config.clone(),
            /*initial_user_message*/ None,
        );
        self.replace_chat_widget(ChatWidget::new_with_app_event(init));

        self.reset_for_thread_switch(tui)?;
        self.replay_thread_snapshot(snapshot, !is_replay_only);
        if is_replay_only {
            let message = if attached_replay_only {
                format!(
                    "Agent thread {thread_id} could not be resumed live. Replaying saved transcript."
                )
            } else {
                format!("Agent thread {thread_id} is closed. Replaying saved transcript.")
            };
            self.chat_widget.add_info_message(message, /*hint*/ None);
        }
        self.drain_active_thread_events(tui).await?;
        self.refresh_pending_thread_approvals().await;

        Ok(())
    }

    pub(super) fn should_attach_live_thread_for_selection(&self, thread_id: ThreadId) -> bool {
        !self.thread_event_channels.contains_key(&thread_id)
            && self
                .agent_navigation
                .get(&thread_id)
                .is_none_or(|entry| !entry.is_closed)
    }

    pub(super) fn reset_for_thread_switch(&mut self, tui: &mut tui::Tui) -> Result<()> {
        self.reset_transcript_state_after_clear();
        tui.clear_pending_history_lines();
        Self::clear_terminal_for_thread_switch(&mut tui.terminal)?;
        Ok(())
    }

    pub(super) fn clear_terminal_for_thread_switch<B>(
        terminal: &mut crate::custom_terminal::Terminal<B>,
    ) -> Result<()>
    where
        B: Backend + Write,
    {
        terminal.clear_scrollback_and_visible_screen_ansi()?;
        let mut area = terminal.viewport_area;
        if area.y > 0 {
            area.y = 0;
            terminal.set_viewport_area(area);
        }
        Ok(())
    }

    pub(super) fn reset_thread_event_state(&mut self) {
        self.abort_all_thread_event_listeners();
        self.thread_event_channels.clear();
        self.agent_navigation.clear();
        self.side_threads.clear();
        self.active_thread_id = None;
        self.active_thread_rx = None;
        self.primary_thread_id = None;
        self.last_subagent_backfill_attempt = None;
        self.primary_session_configured = None;
        self.pending_primary_events.clear();
        self.pending_app_server_requests.clear();
        self.pending_startup_thread_start = false;
        self.chat_widget.set_pending_thread_approvals(Vec::new());
        self.sync_active_agent_label();
    }

    pub(super) async fn handle_startup_thread_started(
        &mut self,
        app_server: &mut AppServerSession,
        result: Result<AppServerStartedThread, String>,
    ) -> Result<()> {
        if !self.pending_startup_thread_start {
            if let Ok(started) = result {
                let thread_id = started.session.thread_id;
                if let Err(err) = app_server.thread_unsubscribe(thread_id).await {
                    tracing::warn!(
                        thread_id = %thread_id,
                        "failed to unsubscribe stale startup thread: {err}"
                    );
                }
                self.discard_thread_local_state(thread_id).await;
            }
            return Ok(());
        }

        self.pending_startup_thread_start = false;
        self.chat_widget
            .set_queue_submissions_until_session_configured(/*queue*/ false);
        match result {
            Ok(started) => {
                self.enqueue_primary_thread_session(started.session, started.turns)
                    .await?;
                self.chat_widget.maybe_send_next_queued_input();
            }
            Err(err) => {
                return Err(color_eyre::eyre::eyre!(
                    "Failed to start a fresh session through the app server: {err}"
                ));
            }
        }
        Ok(())
    }

    pub(super) async fn start_fresh_session_with_summary_hint(
        &mut self,
        tui: &mut tui::Tui,
        app_server: &mut AppServerSession,
        session_start_source: Option<ThreadStartSource>,
        initial_user_message: Option<crate::chatwidget::UserMessage>,
    ) {
        // Start a fresh in-memory session while preserving resumability via persisted rollout
        // history. If an initial message is provided, `enqueue_primary_thread_session` suppresses it
        // until the new session is configured and any replayed turns have been rendered.
        self.refresh_in_memory_config_from_disk_best_effort("starting a new thread")
            .await;
        let model = self.chat_widget.current_model().to_string();
        let config = self.fresh_session_config();
        let summary = session_summary(
            self.chat_widget.token_usage(),
            self.chat_widget.thread_id(),
            self.chat_widget.thread_name(),
            self.chat_widget.rollout_path().as_deref(),
        );
        self.shutdown_current_thread(app_server).await;
        let tracked_thread_ids: Vec<ThreadId> =
            self.thread_event_channels.keys().copied().collect();
        for thread_id in tracked_thread_ids {
            if let Err(err) = app_server.thread_unsubscribe(thread_id).await {
                tracing::warn!("failed to unsubscribe tracked thread {thread_id}: {err}");
            }
        }
        self.config = config.clone();
        match app_server
            .start_thread_with_session_start_source(&config, session_start_source)
            .await
        {
            Ok(started) => {
                if let Err(err) = self
                    .replace_chat_widget_with_app_server_thread(
                        tui,
                        app_server,
                        started,
                        initial_user_message,
                    )
                    .await
                {
                    self.chat_widget.add_error_message(format!(
                        "Failed to attach to fresh app-server thread: {err}"
                    ));
                } else if let Some(summary) = summary {
                    let mut lines: Vec<Line<'static>> = Vec::new();
                    if let Some(usage_line) = summary.usage_line {
                        lines.push(usage_line.into());
                    }
                    if let Some(command) = summary.resume_hint {
                        let spans = vec!["To continue this session, run ".into(), command.cyan()];
                        lines.push(spans.into());
                    }
                    self.chat_widget.add_plain_history_lines(lines);
                }
            }
            Err(err) => {
                self.chat_widget.add_error_message(format!(
                    "Failed to start a fresh session through the app server: {err}"
                ));
                self.config.model = Some(model);
            }
        }
        tui.frame_requester().schedule_frame();
    }

    pub(super) async fn replace_chat_widget_with_app_server_thread(
        &mut self,
        tui: &mut tui::Tui,
        app_server: &mut AppServerSession,
        started: AppServerStartedThread,
        initial_user_message: Option<crate::chatwidget::UserMessage>,
    ) -> Result<()> {
        // Initial messages are for freshly attached primary threads only. Thread switches and
        // resume/fork flows pass `None` so they cannot replay old history and then auto-submit a new
        // user turn by accident.
        self.reset_thread_event_state();
        let init = self.chatwidget_init_for_forked_or_resumed_thread(
            tui,
            self.config.clone(),
            initial_user_message,
        );
        self.replace_chat_widget(ChatWidget::new_with_app_event(init));
        self.enqueue_primary_thread_session(started.session, started.turns)
            .await?;
        self.backfill_loaded_subagent_threads(app_server).await;
        Ok(())
    }

    /// Fetches all loaded threads from the app server and registers descendants of the primary
    /// thread in the navigation cache and chat widget metadata.
    ///
    /// Called after `replace_chat_widget_with_app_server_thread` during resume, fork, and new
    /// thread creation so that the `/agent` picker and keyboard navigation are pre-populated even
    /// if the TUI did not witness the original spawn events.
    ///
    /// The loaded-thread list is fetched in full (no pagination) and the spawn tree is walked
    /// by `find_loaded_subagent_threads_for_primary`. Each discovered subagent is registered via
    /// `upsert_agent_picker_thread`, which writes to both `AgentNavigationState` and the
    /// `ChatWidget` metadata map.
    pub(super) async fn backfill_loaded_subagent_threads(
        &mut self,
        app_server: &mut AppServerSession,
    ) -> bool {
        let Some(primary_thread_id) = self.primary_thread_id else {
            return false;
        };

        let loaded_thread_ids = match app_server
            .thread_loaded_list(ThreadLoadedListParams {
                cursor: None,
                limit: None,
            })
            .await
        {
            Ok(response) => response.data,
            Err(err) => {
                tracing::warn!(%err, "failed to list loaded threads for subagent backfill");
                return false;
            }
        };

        let mut threads = Vec::new();
        let mut had_read_error = false;
        for thread_id in loaded_thread_ids {
            let Ok(thread_id) = ThreadId::from_string(&thread_id) else {
                tracing::warn!("ignoring loaded thread with invalid id during subagent backfill");
                continue;
            };

            if thread_id == primary_thread_id {
                continue;
            }

            match app_server
                .thread_read(thread_id, /*include_turns*/ false)
                .await
            {
                Ok(thread) => threads.push(thread),
                Err(err) => {
                    had_read_error = true;
                    tracing::warn!(thread_id = %thread_id, %err, "failed to read loaded thread");
                }
            }
        }

        for thread in find_loaded_subagent_threads_for_primary(threads, primary_thread_id) {
            self.upsert_agent_picker_thread(
                thread.thread_id,
                thread.agent_nickname,
                thread.agent_role,
                /*is_closed*/ false,
            );
        }

        !had_read_error
    }

    /// Returns the adjacent thread id for keyboard navigation, backfilling from the server if the
    /// local cache has no neighbor.
    ///
    /// Tries the fast path first: ask `AgentNavigationState` directly. If it returns `None` (no
    /// adjacent entry exists, typically because the cache was never populated with remote
    /// subagents), performs a full `backfill_loaded_subagent_threads` and retries. This ensures the
    /// first next/previous keypress in a resumed remote session discovers subagents on demand
    /// without requiring the user to wait for a proactive fetch.
    pub(super) async fn adjacent_thread_id_with_backfill(
        &mut self,
        app_server: &mut AppServerSession,
        direction: AgentNavigationDirection,
    ) -> Option<ThreadId> {
        let current_thread = self.current_displayed_thread_id();
        if let Some(thread_id) = self
            .agent_navigation
            .adjacent_thread_id(current_thread, direction)
        {
            return Some(thread_id);
        }

        let primary_thread_id = self.primary_thread_id?;
        if self.last_subagent_backfill_attempt == Some(primary_thread_id) {
            return None;
        }

        if self.backfill_loaded_subagent_threads(app_server).await {
            self.last_subagent_backfill_attempt = Some(primary_thread_id);
        }
        self.agent_navigation
            .adjacent_thread_id(self.current_displayed_thread_id(), direction)
    }

    pub(super) fn fresh_session_config(&self) -> Config {
        let mut config = self.config.clone();
        config.service_tier = self.chat_widget.configured_service_tier();
        config
    }
    pub(super) async fn resume_target_session(
        &mut self,
        tui: &mut tui::Tui,
        app_server: &mut AppServerSession,
        target_session: SessionTarget,
    ) -> Result<AppRunControl> {
        if self.ignore_same_thread_resume(&target_session) {
            tui.frame_requester().schedule_frame();
            return Ok(AppRunControl::Continue);
        }

        let current_cwd = self.config.cwd.to_path_buf();
        let resume_cwd = if self.app_server_target.uses_remote_workspace() {
            current_cwd.clone()
        } else {
            match crate::session_resume::resolve_cwd_for_resume_or_fork(
                tui,
                self.state_db.as_deref(),
                &current_cwd,
                target_session.thread_id,
                target_session.path.as_deref(),
                CwdPromptAction::Resume,
                /*allow_prompt*/ true,
            )
            .await?
            {
                crate::session_resume::ResolveCwdOutcome::Continue(Some(cwd)) => cwd,
                crate::session_resume::ResolveCwdOutcome::Continue(None) => current_cwd.clone(),
                crate::session_resume::ResolveCwdOutcome::Exit => {
                    return Ok(AppRunControl::Exit(ExitReason::UserRequested));
                }
            }
        };

        let mut resume_config = match self
            .rebuild_config_for_resume_or_fallback(&current_cwd, resume_cwd)
            .await
        {
            Ok(cfg) => cfg,
            Err(err) => {
                self.chat_widget.add_error_message(format!(
                    "Failed to rebuild configuration for resume: {err}"
                ));
                return Ok(AppRunControl::Continue);
            }
        };
        self.apply_runtime_policy_overrides(&mut resume_config);

        let summary = session_summary(
            self.chat_widget.token_usage(),
            self.chat_widget.thread_id(),
            self.chat_widget.thread_name(),
            self.chat_widget.rollout_path().as_deref(),
        );
        match app_server
            .resume_thread(resume_config.clone(), target_session.thread_id)
            .await
        {
            Ok(resumed) => {
                let resumed_thread_id = resumed.session.thread_id;
                self.shutdown_current_thread(app_server).await;
                self.config = resume_config;
                tui.set_notification_settings(
                    self.config.tui_notifications.method,
                    self.config.tui_notifications.condition,
                );
                self.file_search
                    .update_search_dir(self.config.cwd.to_path_buf());
                match self
                    .replace_chat_widget_with_app_server_thread(
                        tui, app_server, resumed, /*initial_user_message*/ None,
                    )
                    .await
                {
                    Ok(()) => {
                        if let Some(summary) = summary {
                            let mut lines: Vec<Line<'static>> = Vec::new();
                            if let Some(usage_line) = summary.usage_line {
                                lines.push(usage_line.into());
                            }
                            if let Some(command) = summary.resume_hint {
                                let spans =
                                    vec!["To continue this session, run ".into(), command.cyan()];
                                lines.push(spans.into());
                            }
                            self.chat_widget.add_plain_history_lines(lines);
                        }
                        self.maybe_prompt_resume_paused_goal_after_resume(
                            app_server,
                            resumed_thread_id,
                        )
                        .await;
                    }
                    Err(err) => {
                        self.chat_widget.add_error_message(format!(
                            "Failed to attach to resumed app-server thread: {err}"
                        ));
                    }
                }
            }
            Err(err) => {
                let path_display = target_session.display_label();
                self.chat_widget.add_error_message(format!(
                    "Failed to resume session from {path_display}: {err}"
                ));
            }
        }

        Ok(AppRunControl::Continue)
    }
}

fn slugify_agent_definition_name(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "agent".to_string()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_thread_read_error_detection_matches_not_loaded_errors() {
        let err = color_eyre::eyre::eyre!(
            "thread/read failed during TUI session lookup: thread/read failed: thread not loaded: thr_123"
        );

        assert!(App::is_terminal_thread_read_error(&err));
    }

    #[test]
    fn terminal_thread_read_error_detection_ignores_transient_failures() {
        let err = color_eyre::eyre::eyre!(
            "thread/read failed during TUI session lookup: thread/read transport error: broken pipe"
        );

        assert!(!App::is_terminal_thread_read_error(&err));
    }

    #[test]
    fn closed_state_for_thread_read_error_preserves_live_state_without_cache_on_transient_error() {
        let err = color_eyre::eyre::eyre!(
            "thread/read failed during TUI session lookup: thread/read transport error: broken pipe"
        );

        assert!(!App::closed_state_for_thread_read_error(
            &err, /*existing_is_closed*/ None
        ));
    }

    #[test]
    fn closed_state_for_thread_read_error_marks_terminal_uncached_threads_closed() {
        let err = color_eyre::eyre::eyre!(
            "thread/read failed during TUI session lookup: thread/read failed: thread not loaded: thr_123"
        );

        assert!(App::closed_state_for_thread_read_error(
            &err, /*existing_is_closed*/ None
        ));
    }

    #[test]
    fn include_turns_fallback_detection_handles_unmaterialized_and_ephemeral_threads() {
        let unmaterialized = color_eyre::eyre::eyre!(
            "thread/read failed during TUI session lookup: thread/read failed: thread thr_123 is not materialized yet; includeTurns is unavailable before first user message"
        );
        let ephemeral = color_eyre::eyre::eyre!(
            "thread/read failed during TUI session lookup: thread/read failed: ephemeral threads do not support includeTurns"
        );

        assert!(App::can_fallback_from_include_turns_error(&unmaterialized));
        assert!(App::can_fallback_from_include_turns_error(&ephemeral));
    }
}
