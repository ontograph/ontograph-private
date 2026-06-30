use pretty_assertions::assert_eq;

use super::*;

#[test]
fn manifest_serializes_as_json_contract() {
    let manifest = sample_manifest();

    let value = serde_json::to_value(&manifest).expect("manifest should serialize");
    assert_eq!(
        value,
        serde_json::json!({
            "repository": {
                "root": "/workspace/project",
                "vcs": "git",
                "remote_url": "https://example.test/project.git"
            },
            "version": {
                "schema_version": 1,
                "implementation_version": "0.0.0-test"
            },
            "active_generation_id": "gen-1",
            "completed_generations": [{
                "metadata": {
                    "generation_id": "gen-1",
                    "target_head": "abc123",
                    "created_at_unix_seconds": 1_725_000_000,
                    "completed_at_unix_seconds": 1_725_000_010,
                    "manifest_hash": "sha256:manifest"
                },
                "counts": {
                    "files": 3,
                    "symbols": 5,
                    "relationships": 7,
                    "nodes": 8,
                    "edges": 9
                }
            }]
        })
    );

    let round_trip =
        serde_json::from_value::<OntographManifest>(value).expect("manifest should deserialize");
    assert_eq!(round_trip, manifest);
}

#[test]
fn matching_manifest_does_not_require_rebuild() {
    let manifest = sample_manifest();

    assert_eq!(manifest.rebuild_reason(&sample_expectation()), None);
}

#[test]
fn version_mismatch_requires_rebuild() {
    let mut manifest = sample_manifest();
    manifest.version.schema_version = 2;

    assert_eq!(
        manifest.rebuild_reason(&sample_expectation()),
        Some(RebuildReason::SchemaVersionMismatch)
    );

    let mut manifest = sample_manifest();
    manifest.version.implementation_version = "0.0.1-test".to_string();

    assert_eq!(
        manifest.rebuild_reason(&sample_expectation()),
        Some(RebuildReason::ImplementationVersionMismatch)
    );
}

#[test]
fn repository_or_head_mismatch_requires_rebuild() {
    let mut manifest = sample_manifest();
    manifest.repository.root = "/workspace/other".to_string();

    assert_eq!(
        manifest.rebuild_reason(&sample_expectation()),
        Some(RebuildReason::RepositoryMismatch)
    );

    let mut manifest = sample_manifest();
    manifest.completed_generations[0].metadata.target_head = "def456".to_string();

    assert_eq!(
        manifest.rebuild_reason(&sample_expectation()),
        Some(RebuildReason::TargetHeadMismatch)
    );
}

#[test]
fn missing_active_generation_requires_rebuild() {
    let mut manifest = sample_manifest();
    manifest.active_generation_id = "missing".to_string();

    assert_eq!(
        manifest.rebuild_reason(&sample_expectation()),
        Some(RebuildReason::MissingActiveGeneration)
    );
}

#[test]
fn artifact_root_uses_private_ontograph_paths_and_rejects_path_like_generation_ids() {
    let root = OntographArtifactRoot::new("/workspace/project");

    assert_eq!(
        root.root_path(),
        std::path::PathBuf::from("/workspace/project/.ontograph")
    );
    assert_eq!(
        root.generation_manifest_path("gen-1").expect("valid id"),
        std::path::PathBuf::from("/workspace/project/.ontograph/generations/gen-1/manifest.json")
    );
    assert_eq!(
        root.generation_manifest_path("../escape").unwrap_err(),
        ArtifactPublishError::InvalidGenerationId
    );
}

#[test]
fn publish_manifest_writes_current_pointer_and_ready_manifest() {
    let dir = tempfile::tempdir().expect("temp dir");
    let root = OntographArtifactRoot::new(dir.path());
    let manifest = sample_manifest();

    root.publish_manifest(&manifest).expect("publish manifest");

    assert_eq!(
        std::fs::read_to_string(root.current_pointer_path()).expect("current pointer"),
        "gen-1\n"
    );
    let written = std::fs::read_to_string(
        root.generation_manifest_path("gen-1")
            .expect("manifest path"),
    )
    .expect("manifest file");
    assert_eq!(
        serde_json::from_str::<OntographManifest>(&written).expect("manifest json"),
        manifest
    );
    assert_eq!(
        root.current_manifest_readiness(&sample_expectation()),
        ArtifactReadiness::Ready
    );
}

#[test]
fn current_manifest_readiness_fails_closed_for_missing_or_corrupt_pointer() {
    let dir = tempfile::tempdir().expect("temp dir");
    let root = OntographArtifactRoot::new(dir.path());

    assert_eq!(
        root.current_manifest_readiness(&sample_expectation()),
        ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::MissingCurrentPointer)
    );

    std::fs::create_dir_all(root.root_path()).expect("create root");
    std::fs::write(root.current_pointer_path(), "../escape\n").expect("write corrupt pointer");

    assert_eq!(
        root.current_manifest_readiness(&sample_expectation()),
        ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::CorruptCurrentPointer)
    );
}

#[test]
fn current_manifest_readiness_fails_closed_for_missing_or_invalid_manifest() {
    let dir = tempfile::tempdir().expect("temp dir");
    let root = OntographArtifactRoot::new(dir.path());

    std::fs::create_dir_all(root.root_path()).expect("create root");
    std::fs::write(root.current_pointer_path(), "gen-1\n").expect("write pointer");

    assert_eq!(
        root.current_manifest_readiness(&sample_expectation()),
        ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::MissingManifest)
    );

    let manifest_path = root
        .generation_manifest_path("gen-1")
        .expect("manifest path");
    std::fs::create_dir_all(manifest_path.parent().expect("manifest parent"))
        .expect("create generation");
    std::fs::write(manifest_path, "{not json").expect("write invalid manifest");

    assert_eq!(
        root.current_manifest_readiness(&sample_expectation()),
        ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::InvalidManifest)
    );
}

#[test]
fn current_manifest_readiness_reports_manifest_mismatch() {
    let dir = tempfile::tempdir().expect("temp dir");
    let root = OntographArtifactRoot::new(dir.path());
    let mut manifest = sample_manifest();
    manifest.completed_generations[0].metadata.target_head = "def456".to_string();

    root.publish_manifest(&manifest).expect("publish manifest");

    assert_eq!(
        root.current_manifest_readiness(&sample_expectation()),
        ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::ManifestMismatch(
            RebuildReason::TargetHeadMismatch
        ))
    );
}

fn sample_expectation() -> ManifestExpectation {
    ManifestExpectation {
        repository: sample_repository(),
        version: sample_version(),
        target_head: "abc123".to_string(),
    }
}

fn sample_manifest() -> OntographManifest {
    OntographManifest {
        repository: sample_repository(),
        version: sample_version(),
        active_generation_id: "gen-1".to_string(),
        completed_generations: vec![ArtifactGeneration {
            metadata: GenerationMetadata {
                generation_id: "gen-1".to_string(),
                target_head: "abc123".to_string(),
                created_at_unix_seconds: 1_725_000_000,
                completed_at_unix_seconds: 1_725_000_010,
                manifest_hash: "sha256:manifest".to_string(),
            },
            counts: GraphCountSummary {
                files: 3,
                symbols: 5,
                relationships: 7,
                nodes: 8,
                edges: 9,
            },
        }],
    }
}

fn sample_repository() -> RepositoryIdentity {
    RepositoryIdentity {
        root: "/workspace/project".to_string(),
        vcs: "git".to_string(),
        remote_url: Some("https://example.test/project.git".to_string()),
    }
}

fn sample_version() -> ArtifactVersion {
    ArtifactVersion {
        schema_version: ONTOGRAPH_SCHEMA_VERSION,
        implementation_version: "0.0.0-test".to_string(),
    }
}
