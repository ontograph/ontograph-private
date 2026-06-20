use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use ontocode_config::types::McpServerTransportConfig;
use ontocode_core::config::edit::ConfigEditsBuilder;
use ontocode_core::config::load_global_mcp_servers;
use pretty_assertions::assert_eq;
use serde_json::Value as JsonValue;
use serde_json::json;
use tempfile::TempDir;

fn codex_command(codex_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(ontocode_utils_cargo_bin::cargo_bin("ontocode")?);
    cmd.env("CODEX_HOME", codex_home);
    Ok(cmd)
}

#[test]
fn list_shows_empty_state() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd.args(["mcp", "list"]).output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("No MCP servers configured yet."));

    Ok(())
}

#[tokio::test]
async fn list_and_get_render_expected_output() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut add = codex_command(codex_home.path())?;
    add.args([
        "mcp",
        "add",
        "docs",
        "--env",
        "TOKEN=stdio-secret-value",
        "--",
        "docs-server",
        "--port",
        "4000",
    ])
    .assert()
    .success();

    let mut servers = load_global_mcp_servers(codex_home.path()).await?;
    let docs_entry = servers
        .get_mut("docs")
        .expect("docs server should exist after add");
    match &mut docs_entry.transport {
        McpServerTransportConfig::Stdio { env_vars, .. } => {
            *env_vars = vec!["APP_TOKEN".into(), "WORKSPACE_ID".into()];
        }
        other => panic!("unexpected transport: {other:?}"),
    }
    ConfigEditsBuilder::new(codex_home.path())
        .replace_mcp_servers(&servers)
        .apply_blocking()?;

    let mut list_cmd = codex_command(codex_home.path())?;
    let list_output = list_cmd.args(["mcp", "list"]).output()?;
    assert!(list_output.status.success());
    let stdout = String::from_utf8(list_output.stdout)?;
    assert!(stdout.contains("Name"));
    assert!(stdout.contains("docs"));
    assert!(stdout.contains("docs-server"));
    assert!(!stdout.contains("stdio-secret-value"));
    assert!(stdout.contains("TOKEN=*****"));
    assert!(stdout.contains("APP_TOKEN=*****"));
    assert!(stdout.contains("WORKSPACE_ID=*****"));
    assert!(stdout.contains("Status"));
    assert!(stdout.contains("Auth"));
    assert!(stdout.contains("enabled"));
    assert!(stdout.contains("Unsupported"));

    let mut list_json_cmd = codex_command(codex_home.path())?;
    let json_output = list_json_cmd.args(["mcp", "list", "--json"]).output()?;
    assert!(json_output.status.success());
    let stdout = String::from_utf8(json_output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed,
        json!([
          {
            "name": "docs",
            "enabled": true,
            "disabled_reason": null,
            "transport": {
              "type": "stdio",
              "command": "docs-server",
              "args": [
                "--port",
                "4000"
              ],
              "env": {
                "TOKEN": "*****"
              },
              "env_vars": [
                "APP_TOKEN",
                "WORKSPACE_ID"
              ],
              "cwd": null
            },
            "startup_timeout_sec": null,
            "tool_timeout_sec": null,
            "auth_status": "unsupported"
          }
        ]
        )
    );

    let mut get_cmd = codex_command(codex_home.path())?;
    let get_output = get_cmd.args(["mcp", "get", "docs"]).output()?;
    assert!(get_output.status.success());
    let stdout = String::from_utf8(get_output.stdout)?;
    assert!(stdout.contains("docs"));
    assert!(stdout.contains("transport: stdio"));
    assert!(stdout.contains("command: docs-server"));
    assert!(stdout.contains("args: --port 4000"));
    assert!(!stdout.contains("stdio-secret-value"));
    assert!(stdout.contains("env: TOKEN=*****"));
    assert!(stdout.contains("APP_TOKEN=*****"));
    assert!(stdout.contains("WORKSPACE_ID=*****"));
    assert!(stdout.contains("enabled: true"));
    assert!(stdout.contains("remove: codex mcp remove docs"));

    let mut get_json_cmd = codex_command(codex_home.path())?;
    let get_json_output = get_json_cmd
        .args(["mcp", "get", "docs", "--json"])
        .output()?;
    assert!(get_json_output.status.success());
    let stdout = String::from_utf8(get_json_output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed,
        json!({
            "name": "docs",
            "enabled": true,
            "disabled_reason": null,
            "transport": {
                "type": "stdio",
                "command": "docs-server",
                "args": ["--port", "4000"],
                "env": {
                    "TOKEN": "*****"
                },
                "env_vars": ["APP_TOKEN", "WORKSPACE_ID"],
                "cwd": null
            },
            "enabled_tools": null,
            "disabled_tools": null,
            "startup_timeout_sec": null,
            "tool_timeout_sec": null,
        })
    );

    Ok(())
}

#[tokio::test]
async fn list_and_get_json_redact_streamable_http_credentials() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut add = codex_command(codex_home.path())?;
    add.args([
        "mcp",
        "add",
        "github",
        "--url",
        "https://example.com/mcp",
        "--bearer-token-env-var",
        "GITHUB_TOKEN",
    ])
    .assert()
    .success();

    let mut servers = load_global_mcp_servers(codex_home.path()).await?;
    let github = servers
        .get_mut("github")
        .expect("github server should exist after add");
    match &mut github.transport {
        McpServerTransportConfig::StreamableHttp {
            http_headers,
            env_http_headers,
            ..
        } => {
            *http_headers = Some(HashMap::from([
                (
                    "Authorization".to_string(),
                    "Bearer raw-bearer-token".to_string(),
                ),
                ("Cookie".to_string(), "raw-cookie-value".to_string()),
            ]));
            *env_http_headers = Some(HashMap::from([
                (
                    "Authorization".to_string(),
                    "GITHUB_AUTHORIZATION_HEADER".to_string(),
                ),
                ("Cookie".to_string(), "GITHUB_COOKIE_HEADER".to_string()),
            ]));
        }
        other => panic!("unexpected transport: {other:?}"),
    }
    ConfigEditsBuilder::new(codex_home.path())
        .replace_mcp_servers(&servers)
        .apply_blocking()?;

    let mut list_cmd = codex_command(codex_home.path())?;
    let list_output = list_cmd.args(["mcp", "list", "--json"]).output()?;
    assert!(list_output.status.success());
    let stdout = String::from_utf8(list_output.stdout)?;
    assert!(!stdout.contains("raw-bearer-token"));
    assert!(!stdout.contains("raw-cookie-value"));
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed,
        json!([
          {
            "name": "github",
            "enabled": true,
            "disabled_reason": null,
            "transport": {
              "type": "streamable_http",
              "url": "https://example.com/mcp",
              "bearer_token_env_var": "GITHUB_TOKEN",
              "http_headers": {
                "Authorization": "*****",
                "Cookie": "*****"
              },
              "env_http_headers": {
                "Authorization": "GITHUB_AUTHORIZATION_HEADER",
                "Cookie": "GITHUB_COOKIE_HEADER"
              }
            },
            "startup_timeout_sec": null,
            "tool_timeout_sec": null,
            "auth_status": "bearer_token"
          }
        ])
    );

    let mut get_cmd = codex_command(codex_home.path())?;
    let get_output = get_cmd.args(["mcp", "get", "github", "--json"]).output()?;
    assert!(get_output.status.success());
    let stdout = String::from_utf8(get_output.stdout)?;
    assert!(!stdout.contains("raw-bearer-token"));
    assert!(!stdout.contains("raw-cookie-value"));
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed,
        json!({
            "name": "github",
            "enabled": true,
            "disabled_reason": null,
            "transport": {
                "type": "streamable_http",
                "url": "https://example.com/mcp",
                "bearer_token_env_var": "GITHUB_TOKEN",
                "http_headers": {
                    "Authorization": "*****",
                    "Cookie": "*****"
                },
                "env_http_headers": {
                    "Authorization": "GITHUB_AUTHORIZATION_HEADER",
                    "Cookie": "GITHUB_COOKIE_HEADER"
                }
            },
            "enabled_tools": null,
            "disabled_tools": null,
            "startup_timeout_sec": null,
            "tool_timeout_sec": null,
        })
    );

    Ok(())
}

#[tokio::test]
async fn get_disabled_server_shows_single_line() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut add = codex_command(codex_home.path())?;
    add.args(["mcp", "add", "docs", "--", "docs-server"])
        .assert()
        .success();

    let mut servers = load_global_mcp_servers(codex_home.path()).await?;
    let docs = servers
        .get_mut("docs")
        .expect("docs server should exist after add");
    docs.enabled = false;
    ConfigEditsBuilder::new(codex_home.path())
        .replace_mcp_servers(&servers)
        .apply_blocking()?;

    let mut get_cmd = codex_command(codex_home.path())?;
    let get_output = get_cmd.args(["mcp", "get", "docs"]).output()?;
    assert!(get_output.status.success());
    let stdout = String::from_utf8(get_output.stdout)?;
    assert_eq!(stdout.trim_end(), "docs (disabled)");

    Ok(())
}
