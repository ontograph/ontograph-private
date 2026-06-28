use super::ToolOrchestrator;
use super::unattended_read_only_permission_profile;
use core_test_support::PathBufExt;
use core_test_support::test_path_buf;
use ontocode_protocol::models::PermissionProfile;
use ontocode_protocol::permissions::FileSystemAccessMode;
use ontocode_protocol::permissions::FileSystemPath;
use ontocode_protocol::permissions::FileSystemSandboxEntry;
use ontocode_protocol::permissions::FileSystemSandboxPolicy;
use ontocode_protocol::permissions::NetworkSandboxPolicy;
use pretty_assertions::assert_eq;

#[test]
fn clamp_file_system_policy_to_read_only_downgrades_writes_and_preserves_denies() {
    let readable_root = test_path_buf("/tmp/orchestrator-read-only").abs();
    let denied_root = readable_root.join(".git");
    let policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Path {
                path: readable_root.clone(),
            },
            access: FileSystemAccessMode::Write,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path {
                path: denied_root.clone(),
            },
            access: FileSystemAccessMode::Deny,
        },
    ]);

    let clamped = ToolOrchestrator::clamp_file_system_policy_to_read_only(&policy);

    assert_eq!(policy.kind, clamped.kind);
    assert_eq!(policy.glob_scan_max_depth, clamped.glob_scan_max_depth);
    assert_eq!(
        vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: readable_root.clone(),
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path { path: denied_root },
                access: FileSystemAccessMode::Deny,
            },
        ],
        clamped.entries
    );
}

#[test]
fn unattended_read_only_permission_profile_preserves_network_policy() {
    let writable_root = test_path_buf("/tmp/orchestrator-permissions").abs();
    let permission_profile = PermissionProfile::workspace_write_with(
        std::slice::from_ref(&writable_root),
        NetworkSandboxPolicy::Enabled,
        /*exclude_tmpdir_env_var*/ false,
        /*exclude_slash_tmp*/ false,
    );

    let clamped = unattended_read_only_permission_profile(&permission_profile);

    assert_eq!(
        NetworkSandboxPolicy::Enabled,
        clamped.network_sandbox_policy()
    );
    assert!(
        clamped
            .file_system_sandbox_policy()
            .can_read_path_with_cwd(writable_root.as_path(), writable_root.as_path())
    );
    assert!(
        !clamped
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(writable_root.as_path(), writable_root.as_path())
    );
}
