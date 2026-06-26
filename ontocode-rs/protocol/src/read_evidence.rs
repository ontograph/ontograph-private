use ontocode_utils_absolute_path::AbsolutePathBuf;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use ts_rs::TS;

const MAX_EVIDENCE_BUCKET_ENTRIES: usize = 32;
const MAX_EVIDENCE_ENTRY_CHARS: usize = 160;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
#[serde(default)]
pub struct FileReadEvidence {
    pub paths: BTreeMap<AbsolutePathBuf, usize>,
    pub symbol_touches: BTreeMap<String, usize>,
    pub tests_run: BTreeMap<String, usize>,
    pub policy_checks: BTreeMap<String, usize>,
    pub source_references: BTreeMap<String, usize>,
}

impl Default for FileReadEvidence {
    fn default() -> Self {
        Self {
            paths: BTreeMap::new(),
            symbol_touches: BTreeMap::new(),
            tests_run: BTreeMap::new(),
            policy_checks: BTreeMap::new(),
            source_references: BTreeMap::new(),
        }
    }
}

impl FileReadEvidence {
    pub fn record_path(&mut self, path: AbsolutePathBuf) {
        record_bounded_entry(&mut self.paths, path);
    }

    pub fn record_symbol_touch(&mut self, symbol: impl AsRef<str>) {
        if let Some(symbol) = normalize_evidence_entry(symbol.as_ref()) {
            record_bounded_entry(&mut self.symbol_touches, symbol);
        }
    }

    pub fn record_test_run(&mut self, test: impl AsRef<str>) {
        if let Some(test) = normalize_evidence_entry(test.as_ref()) {
            record_bounded_entry(&mut self.tests_run, test);
        }
    }

    pub fn record_policy_check(&mut self, check: impl AsRef<str>) {
        if let Some(check) = normalize_evidence_entry(check.as_ref()) {
            record_bounded_entry(&mut self.policy_checks, check);
        }
    }

    pub fn record_source_reference(&mut self, source_ref: impl AsRef<str>) {
        if let Some(source_ref) = normalize_evidence_entry(source_ref.as_ref()) {
            record_bounded_entry(&mut self.source_references, source_ref);
        }
    }
}

fn record_bounded_entry<T>(entries: &mut BTreeMap<T, usize>, entry: T)
where
    T: Ord + Clone,
{
    if entries.len() >= MAX_EVIDENCE_BUCKET_ENTRIES && !entries.contains_key(&entry) {
        return;
    }

    *entries.entry(entry).or_insert(0) += 1;
}

fn normalize_evidence_entry(value: &str) -> Option<String> {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return None;
    }

    Some(truncate_to_chars(&normalized, MAX_EVIDENCE_ENTRY_CHARS))
}

fn truncate_to_chars(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let mut truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        truncated.push_str("...");
    }
    truncated
}

#[cfg(test)]
#[path = "read_evidence_tests.rs"]
mod tests;
