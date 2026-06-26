use crate::function_tool::FunctionCallError;
use crate::safety::SafetyCheck;
use crate::safety::assess_patch_safety;
use crate::session::turn_context::TurnContext;
use crate::tools::sandboxing::ExecApprovalRequirement;
use ontocode_apply_patch::ApplyPatchAction;
use ontocode_apply_patch::ApplyPatchFileChange;
use ontocode_protocol::protocol::FileChange;
use ontocode_protocol::protocol::FileSystemSandboxPolicy;
use ontocode_utils_absolute_path::AbsolutePathBuf;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub(crate) enum InternalApplyPatchInvocation {
    /// The `apply_patch` call was handled programmatically, without any sort
    /// of sandbox, because the user explicitly approved it. This is the
    /// result to use with the `shell` function call that contained `apply_patch`.
    Output(Result<String, FunctionCallError>),

    /// The `apply_patch` call was approved, either automatically because it
    /// appears that it should be allowed based on the user's sandbox policy
    /// *or* because the user explicitly approved it. The runtime realizes the
    /// patch through the selected environment filesystem.
    DelegateToRuntime(ApplyPatchRuntimeInvocation),
}

#[derive(Debug)]
pub(crate) struct ApplyPatchRuntimeInvocation {
    pub(crate) action: ApplyPatchAction,
    pub(crate) auto_approved: bool,
    pub(crate) exec_approval_requirement: ExecApprovalRequirement,
}

pub(crate) async fn apply_patch(
    turn_context: &TurnContext,
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    action: ApplyPatchAction,
) -> InternalApplyPatchInvocation {
    let generated_file_warning_paths = generated_file_warning_paths(&action);
    match assess_patch_safety(
        &action,
        turn_context.approval_policy.value(),
        &turn_context.permission_profile(),
        file_system_sandbox_policy,
        &action.cwd,
        turn_context.windows_sandbox_level,
    ) {
        SafetyCheck::AutoApprove {
            user_explicitly_approved,
            ..
        } if generated_file_warning_paths.is_empty() => {
            InternalApplyPatchInvocation::DelegateToRuntime(ApplyPatchRuntimeInvocation {
                action,
                auto_approved: !user_explicitly_approved,
                exec_approval_requirement: ExecApprovalRequirement::Skip {
                    bypass_sandbox: false,
                    proposed_execpolicy_amendment: None,
                },
            })
        }
        SafetyCheck::AutoApprove { .. } => {
            InternalApplyPatchInvocation::DelegateToRuntime(ApplyPatchRuntimeInvocation {
                action,
                auto_approved: false,
                exec_approval_requirement: ExecApprovalRequirement::NeedsApproval {
                    reason: None,
                    proposed_execpolicy_amendment: None,
                },
            })
        }
        SafetyCheck::AskUser => {
            // Delegate the approval prompt (including cached approvals) to the
            // tool runtime, consistent with how shell/unified_exec approvals
            // are orchestrator-driven.
            InternalApplyPatchInvocation::DelegateToRuntime(ApplyPatchRuntimeInvocation {
                action,
                auto_approved: false,
                exec_approval_requirement: ExecApprovalRequirement::NeedsApproval {
                    reason: None,
                    proposed_execpolicy_amendment: None,
                },
            })
        }
        SafetyCheck::Reject { reason } => InternalApplyPatchInvocation::Output(Err(
            FunctionCallError::RespondToModel(format!("patch rejected: {reason}")),
        )),
    }
}

pub(crate) fn generated_file_warning_paths(action: &ApplyPatchAction) -> Vec<AbsolutePathBuf> {
    let mut generated_paths = BTreeSet::new();
    for (path, change) in action.changes() {
        if is_generated_file_path(path)
            && let Ok(path) = AbsolutePathBuf::from_absolute_path(path)
        {
            generated_paths.insert(path);
        }
        if let ApplyPatchFileChange::Update {
            move_path: Some(move_path),
            ..
        } = change
            && is_generated_file_path(move_path)
            && let Ok(move_path) = AbsolutePathBuf::from_absolute_path(move_path)
        {
            generated_paths.insert(move_path);
        }
    }

    generated_paths.into_iter().collect()
}

pub(crate) fn is_generated_file_path(path: &Path) -> bool {
    const GENERATED_FILENAMES: &[&str] = &[
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "bun.lockb",
        "bun.lock",
        "composer.lock",
        "gemfile.lock",
        "cargo.lock",
        "poetry.lock",
        "pipfile.lock",
        "shrinkwrap.json",
        "npm-shrinkwrap.json",
        ".terraform.lock.hcl",
    ];
    const GENERATED_EXTENSIONS: &[&str] = &[
        ".min.js",
        ".min.css",
        ".min.html",
        ".bundle.js",
        ".bundle.css",
        ".generated.ts",
        ".generated.js",
    ];
    const GENERATED_SEGMENTS: &[&str] = &[
        "dist",
        "build",
        "out",
        "output",
        "node_modules",
        "vendor",
        "vendored",
        "third_party",
        "third-party",
        "external",
        ".next",
        ".nuxt",
        ".svelte-kit",
        "coverage",
        "__pycache__",
        ".tox",
        "venv",
        ".venv",
    ];
    const GENERATED_SUFFIXES: &[&str] = &["target/release", "target/debug"];
    const GENERATED_PATTERNS: &[&str] = &[
        ".generated.",
        ".gen.",
        ".auto.",
        "_generated.",
        "_gen.",
        ".pb.",
        ".grpc.",
        ".swagger.",
        ".openapi.",
    ];

    let normalized = path.to_string_lossy().replace('\\', "/").to_lowercase();
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let file_name = file_name.to_lowercase();

    if GENERATED_FILENAMES.contains(&file_name.as_str()) {
        return true;
    }

    if GENERATED_EXTENSIONS
        .iter()
        .any(|suffix| file_name.ends_with(suffix))
    {
        return true;
    }

    if file_name.split('.').count() > 2 {
        let mut parts = file_name.rsplit('.');
        if let (Some(ext), Some(prev)) = (parts.next(), parts.next()) {
            let compound = format!(".{prev}.{ext}");
            if GENERATED_EXTENSIONS.contains(&compound.as_str()) {
                return true;
            }
        }
    }

    for segment in normalized.split('/').filter(|segment| !segment.is_empty()) {
        if GENERATED_SEGMENTS.contains(&segment) {
            return true;
        }
    }

    if GENERATED_SUFFIXES
        .iter()
        .any(|suffix| normalized.contains(&format!("/{suffix}/")))
    {
        return true;
    }

    GENERATED_PATTERNS
        .iter()
        .any(|pattern| file_name.contains(pattern))
}

pub(crate) fn convert_apply_patch_to_protocol(
    action: &ApplyPatchAction,
) -> HashMap<PathBuf, FileChange> {
    let mut result = HashMap::with_capacity(action.changes().len());
    for (path, change) in action.changes() {
        let protocol_change = match change {
            ApplyPatchFileChange::Add { content, .. } => FileChange::Add {
                content: content.clone(),
            },
            ApplyPatchFileChange::Delete { content } => FileChange::Delete {
                content: content.clone(),
            },
            ApplyPatchFileChange::Update {
                unified_diff,
                move_path,
                new_content: _new_content,
            } => FileChange::Update {
                unified_diff: unified_diff.clone(),
                move_path: move_path.clone(),
            },
        };
        result.insert(path.to_path_buf(), protocol_change);
    }
    result
}

#[cfg(test)]
#[path = "apply_patch_tests.rs"]
mod tests;
