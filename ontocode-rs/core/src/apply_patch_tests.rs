use super::*;
use core_test_support::PathBufExt;
use core_test_support::PathExt;
use ontocode_protocol::models::PermissionProfile;
use ontocode_protocol::permissions::NetworkSandboxPolicy;
use ontocode_protocol::protocol::AskForApproval;
use ontocode_protocol::protocol::FileSystemSandboxPolicy;
use pretty_assertions::assert_eq;

use tempfile::tempdir;

#[test]
fn convert_apply_patch_maps_add_variant() {
    let tmp = tempdir().expect("tmp");
    let p = tmp.path().join("a.txt").abs();
    // Create an action with a single Add change
    let action = ApplyPatchAction::new_add_for_test(&p, "hello".to_string());

    let got = convert_apply_patch_to_protocol(&action);

    assert_eq!(
        got.get(p.as_path()),
        Some(&FileChange::Add {
            content: "hello".to_string()
        })
    );
}

#[test]
fn yolo_full_access_does_not_prompt_for_generated_file_warning() {
    let tmp = tempdir().expect("tmp");
    let cwd = tmp.path().abs();
    let generated_path = cwd.join("third_party/generated.txt");
    let action = ApplyPatchAction::new_add_for_test(&generated_path, "hello".to_string());
    let generated_paths = generated_file_warning_paths(&action);

    assert_eq!(generated_paths, vec![generated_path]);
    assert!(!generated_file_warning_requires_approval(
        &generated_paths,
        AskForApproval::Never,
        &PermissionProfile::Disabled,
        &FileSystemSandboxPolicy::unrestricted(),
    ));
    assert!(generated_file_warning_requires_approval(
        &generated_paths,
        AskForApproval::OnRequest,
        &PermissionProfile::External {
            network: NetworkSandboxPolicy::Enabled,
        },
        &FileSystemSandboxPolicy::external_sandbox(),
    ));
}
