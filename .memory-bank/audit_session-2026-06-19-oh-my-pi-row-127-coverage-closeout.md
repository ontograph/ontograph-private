# Oh My Pi Row 127 Coverage Closeout

Date: 2026-06-19

Outcome:
- ADR row 127 was closed as already covered by the existing redaction test in `ontocode-rs/rmcp-client/src/oauth.rs`.
- No Rust source changes were required.

Evidence:
- `save_oauth_tokens_rejects_malformed_record_without_leaking_tokens` calls `assert_error_redacts_oauth_tokens(&error, &["access-token", "refresh-token"])`.
- The same test asserts the returned error contains `invalid OAuth tokens`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client` passed in this session.

OntoIndex:
- `inspect({action: "context", target: "save_oauth_tokens_rejects_malformed_record_without_leaking_tokens"})` resolved the owner symbol in `ontocode-rs/rmcp-client/src/oauth.rs` and showed the redaction assertion callsite.
