use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use ontocode_utils_path::write_atomically;
use serde::Deserialize;
use serde::Serialize;

pub(crate) const ONTOGRAPH_SCHEMA_VERSION: u32 = 1;
const ONTOGRAPH_DIR: &str = ".ontograph";
const GENERATIONS_DIR: &str = "generations";
const CURRENT_GENERATION_FILE: &str = "current-generation";
const MANIFEST_FILE: &str = "manifest.json";
pub(crate) const SOURCE_INDEX_FILE: &str = "source-index.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RepositoryIdentity {
    pub root: String,
    pub vcs: String,
    pub remote_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ArtifactVersion {
    pub schema_version: u32,
    pub implementation_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GenerationMetadata {
    pub generation_id: String,
    pub target_head: String,
    pub created_at_unix_seconds: i64,
    pub completed_at_unix_seconds: i64,
    pub manifest_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GraphCountSummary {
    pub files: u64,
    pub symbols: u64,
    pub relationships: u64,
    pub nodes: u64,
    pub edges: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ArtifactGeneration {
    pub metadata: GenerationMetadata,
    pub counts: GraphCountSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct OntographManifest {
    pub repository: RepositoryIdentity,
    pub version: ArtifactVersion,
    pub active_generation_id: String,
    pub completed_generations: Vec<ArtifactGeneration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestExpectation {
    pub repository: RepositoryIdentity,
    pub version: ArtifactVersion,
    pub target_head: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OntographArtifactRoot {
    repo_root: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RebuildReason {
    SchemaVersionMismatch,
    ImplementationVersionMismatch,
    RepositoryMismatch,
    MissingActiveGeneration,
    TargetHeadMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactReadiness {
    Ready,
    RebuildRequired(ArtifactRebuildReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactRebuildReason {
    MissingCurrentPointer,
    CorruptCurrentPointer,
    MissingManifest,
    UnreadableManifest,
    InvalidManifest,
    ManifestMismatch(RebuildReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactPublishError {
    MissingActiveGeneration,
    InvalidGenerationId,
    SerializeManifest,
    WriteManifest,
    WriteCurrentPointer,
}

impl OntographManifest {
    pub(crate) fn rebuild_reason(
        &self,
        expectation: &ManifestExpectation,
    ) -> Option<RebuildReason> {
        if self.version.schema_version != expectation.version.schema_version {
            return Some(RebuildReason::SchemaVersionMismatch);
        }
        if self.version.implementation_version != expectation.version.implementation_version {
            return Some(RebuildReason::ImplementationVersionMismatch);
        }
        if self.repository != expectation.repository {
            return Some(RebuildReason::RepositoryMismatch);
        }

        let Some(active_generation) = self.active_generation() else {
            return Some(RebuildReason::MissingActiveGeneration);
        };
        if active_generation.metadata.target_head != expectation.target_head {
            return Some(RebuildReason::TargetHeadMismatch);
        }

        None
    }

    fn active_generation(&self) -> Option<&ArtifactGeneration> {
        self.completed_generations
            .iter()
            .find(|generation| generation.metadata.generation_id == self.active_generation_id)
    }
}

impl OntographArtifactRoot {
    pub(crate) fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }

    pub(crate) fn root_path(&self) -> PathBuf {
        self.repo_root.join(ONTOGRAPH_DIR)
    }

    pub(crate) fn generations_root(&self) -> PathBuf {
        self.root_path().join(GENERATIONS_DIR)
    }

    pub(crate) fn current_pointer_path(&self) -> PathBuf {
        self.root_path().join(CURRENT_GENERATION_FILE)
    }

    pub(crate) fn generation_dir(
        &self,
        generation_id: &str,
    ) -> Result<PathBuf, ArtifactPublishError> {
        validate_generation_id(generation_id)?;
        Ok(self.generations_root().join(generation_id))
    }

    pub(crate) fn generation_manifest_path(
        &self,
        generation_id: &str,
    ) -> Result<PathBuf, ArtifactPublishError> {
        self.generation_file_path(generation_id, MANIFEST_FILE)
    }

    pub(crate) fn generation_file_path(
        &self,
        generation_id: &str,
        file_name: &str,
    ) -> Result<PathBuf, ArtifactPublishError> {
        validate_generation_id(file_name)?;
        Ok(self.generation_dir(generation_id)?.join(file_name))
    }

    pub(crate) fn publish_manifest(
        &self,
        manifest: &OntographManifest,
    ) -> Result<(), ArtifactPublishError> {
        let active_generation = manifest
            .active_generation()
            .ok_or(ArtifactPublishError::MissingActiveGeneration)?;
        let manifest_path =
            self.generation_manifest_path(&active_generation.metadata.generation_id)?;
        let contents = serde_json::to_string_pretty(manifest)
            .map_err(|_| ArtifactPublishError::SerializeManifest)?;
        write_atomically(&manifest_path, &format!("{contents}\n"))
            .map_err(|_| ArtifactPublishError::WriteManifest)?;
        write_atomically(
            &self.current_pointer_path(),
            &format!("{}\n", active_generation.metadata.generation_id),
        )
        .map_err(|_| ArtifactPublishError::WriteCurrentPointer)
    }

    pub(crate) fn current_manifest_readiness(
        &self,
        expectation: &ManifestExpectation,
    ) -> ArtifactReadiness {
        let generation_id = match read_current_generation_id(&self.current_pointer_path()) {
            Ok(generation_id) => generation_id,
            Err(reason) => return ArtifactReadiness::RebuildRequired(reason),
        };
        let Ok(manifest_path) = self.generation_manifest_path(&generation_id) else {
            return ArtifactReadiness::RebuildRequired(
                ArtifactRebuildReason::CorruptCurrentPointer,
            );
        };
        let contents = match fs::read_to_string(manifest_path) {
            Ok(contents) => contents,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::MissingManifest);
            }
            Err(_) => {
                return ArtifactReadiness::RebuildRequired(
                    ArtifactRebuildReason::UnreadableManifest,
                );
            }
        };
        let manifest = match serde_json::from_str::<OntographManifest>(&contents) {
            Ok(manifest) => manifest,
            Err(_) => {
                return ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::InvalidManifest);
            }
        };

        match manifest.rebuild_reason(expectation) {
            Some(reason) => {
                ArtifactReadiness::RebuildRequired(ArtifactRebuildReason::ManifestMismatch(reason))
            }
            None => ArtifactReadiness::Ready,
        }
    }
}

fn read_current_generation_id(path: &Path) -> Result<String, ArtifactRebuildReason> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(ArtifactRebuildReason::MissingCurrentPointer);
        }
        Err(_) => return Err(ArtifactRebuildReason::CorruptCurrentPointer),
    };
    let generation_id = contents.trim();
    validate_generation_id(generation_id)
        .map_err(|_| ArtifactRebuildReason::CorruptCurrentPointer)?;
    Ok(generation_id.to_string())
}

fn validate_generation_id(generation_id: &str) -> Result<(), ArtifactPublishError> {
    let is_valid = !generation_id.is_empty()
        && generation_id != "."
        && generation_id != ".."
        && generation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'));
    if is_valid {
        Ok(())
    } else {
        Err(ArtifactPublishError::InvalidGenerationId)
    }
}

#[cfg(test)]
#[path = "manifest_tests.rs"]
mod tests;
