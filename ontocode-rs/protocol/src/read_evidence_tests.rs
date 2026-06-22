use super::FileReadEvidence;
use super::MAX_EVIDENCE_BUCKET_ENTRIES;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::path::PathBuf;

#[test]
fn file_read_evidence_deserializes_legacy_payloads_with_missing_buckets() {
    let evidence: FileReadEvidence = serde_json::from_value(json!({
        "paths": {
            "/repo/read.txt": 2
        }
    }))
    .expect("legacy evidence payload should deserialize");

    assert_eq!(evidence.paths.len(), 1);
    assert!(evidence.symbol_touches.is_empty());
    assert!(evidence.tests_run.is_empty());
    assert!(evidence.policy_checks.is_empty());
    assert!(evidence.source_references.is_empty());
}

#[test]
fn file_read_evidence_records_bounded_normalized_entries() {
    let mut evidence = FileReadEvidence::default();
    let path =
        ontocode_utils_absolute_path::AbsolutePathBuf::try_from(PathBuf::from("/repo/read.txt"))
            .expect("absolute path");

    evidence.record_path(path.clone());
    evidence.record_path(path.clone());
    evidence.record_symbol_touch("  symbol\nname  ");
    evidence.record_test_run("  cargo   test -p ontocode-core  ");
    evidence.record_policy_check("  just   fmt  ");
    evidence.record_source_reference("  source\nreference\tvalue  ");

    for index in 0..(MAX_EVIDENCE_BUCKET_ENTRIES + 5) {
        evidence.record_symbol_touch(format!("symbol-{index}"));
    }

    assert_eq!(evidence.paths.get(&path), Some(&2));
    assert_eq!(evidence.symbol_touches.get("symbol name"), Some(&1));
    assert_eq!(
        evidence.tests_run.get("cargo test -p ontocode-core"),
        Some(&1)
    );
    assert_eq!(evidence.policy_checks.get("just fmt"), Some(&1));
    assert_eq!(
        evidence.source_references.get("source reference value"),
        Some(&1)
    );
    assert_eq!(evidence.symbol_touches.len(), MAX_EVIDENCE_BUCKET_ENTRIES);
}
