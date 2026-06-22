use ontocode_utils_absolute_path::AbsolutePathBuf;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, TS)]
pub struct FileReadEvidence {
    pub paths: BTreeMap<AbsolutePathBuf, usize>,
}

impl Default for FileReadEvidence {
    fn default() -> Self {
        Self {
            paths: BTreeMap::new(),
        }
    }
}
