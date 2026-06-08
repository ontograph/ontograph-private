use anyhow::Result;
use predicates::str::contains;

#[test]
fn codex_binary_keeps_codex_help_name() -> Result<()> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("codex")?);
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(contains("Codex CLI"))
        .stdout(contains("Usage: codex [OPTIONS] [PROMPT]"));
    Ok(())
}

#[test]
fn ontocode_binary_uses_ontocode_help_name() -> Result<()> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("ontocode")?);
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(contains("Ontocode CLI"))
        .stdout(contains("Usage: ontocode [OPTIONS] [PROMPT]"));
    Ok(())
}

#[test]
fn ontocode_plugin_marketplace_help_uses_alias_name() -> Result<()> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("ontocode")?);
    cmd.args(["plugin", "marketplace", "--help"]);
    cmd.assert().success().stdout(contains(
        "Usage: ontocode plugin marketplace [OPTIONS] <COMMAND>",
    ));
    Ok(())
}
