use super::final_answer_verification_warning;
use ontocode_protocol::read_evidence::FileReadEvidence;

#[test]
fn warns_when_final_answer_claims_tests_without_evidence() {
    let warning = final_answer_verification_warning("Tests passed.", &FileReadEvidence::default())
        .expect("missing test evidence should warn");

    assert!(warning.contains("tests"), "{warning}");
}

#[test]
fn accepts_test_claim_with_recorded_evidence() {
    let mut evidence = FileReadEvidence::default();
    evidence.record_test_run("just test -p ontocode-core");

    assert_eq!(
        final_answer_verification_warning("Tests passed.", &evidence),
        None
    );
}

#[test]
fn does_not_warn_for_explicitly_unrun_tests() {
    assert_eq!(
        final_answer_verification_warning(
            "Tests not run because this was a docs-only review.",
            &FileReadEvidence::default(),
        ),
        None
    );
}
