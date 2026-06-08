use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;

const COMMAND_NAME_OVERRIDE_ENV_VAR: &str = "ONTOCODE_CLI_COMMAND_NAME";
const PRIMARY_COMMAND_NAME: &str = "codex";

fn resolve_codex_path() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_codex").map(PathBuf::from) {
        let path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir()
                .context("failed to resolve current directory for codex binary")?
                .join(path)
        };
        if path.exists() {
            return Ok(path);
        }
    }

    let current_exe = std::env::current_exe().context("failed to locate ontocode binary")?;
    let extension = current_exe
        .extension()
        .map(|ext| format!(".{}", ext.to_string_lossy()))
        .unwrap_or_default();
    Ok(current_exe.with_file_name(format!("{PRIMARY_COMMAND_NAME}{extension}")))
}

fn main() -> anyhow::Result<()> {
    let codex_path = resolve_codex_path()?;

    let status = Command::new(&codex_path)
        .env(COMMAND_NAME_OVERRIDE_ENV_VAR, "ontocode")
        .args(std::env::args_os().skip(1))
        .status()
        .with_context(|| format!("failed to launch {}", codex_path.display()))?;

    std::process::exit(status.code().unwrap_or(1));
}
