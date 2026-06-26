use std::collections::HashMap;
use std::path::PathBuf;

pub const FEEDBACK_DIAGNOSTICS_ATTACHMENT_FILENAME: &str = "codex-connectivity-diagnostics.txt";

const PROXY_ENV_VARS: &[&str] = &[
    "HTTP_PROXY",
    "http_proxy",
    "HTTPS_PROXY",
    "https_proxy",
    "ALL_PROXY",
    "all_proxy",
];

const ENV_DIAGNOSTIC_VARS: &[&str] = &[
    "PATH",
    "SHELL",
    "TERM",
    "LANG",
    "LC_ALL",
    "LC_CTYPE",
    "CODEX_HOME",
    "ONTOCODE_HOME",
];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeedbackDiagnostics {
    diagnostics: Vec<FeedbackDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackDiagnostic {
    pub headline: String,
    pub details: Vec<String>,
}

fn get_codex_home() -> PathBuf {
    if let Some(val) = std::env::var_os("CODEX_HOME") {
        PathBuf::from(val)
    } else if let Some(val) = std::env::var_os("ONTOCODE_HOME") {
        PathBuf::from(val)
    } else {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".codex")
    }
}

impl FeedbackDiagnostics {
    pub fn new(diagnostics: Vec<FeedbackDiagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn collect_from_env() -> Self {
        Self::collect_from_pairs(std::env::vars())
    }

    fn collect_from_pairs<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let env = pairs
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect::<HashMap<_, _>>();
        let mut diagnostics = Vec::new();

        // 1. Proxy environment variables
        let proxy_details = PROXY_ENV_VARS
            .iter()
            .filter_map(|key| {
                let value = env.get(*key)?;
                let redacted_value = ontocode_secrets::redact_secrets(value.clone());
                Some(format!("{key} = {redacted_value}"))
            })
            .collect::<Vec<_>>();
        if !proxy_details.is_empty() {
            diagnostics.push(FeedbackDiagnostic {
                headline: "Proxy environment variables are set and may affect connectivity."
                    .to_string(),
                details: proxy_details,
            });
        }

        // 2. Env diagnostics (redacted support-bundle facts)
        let mut env_details = Vec::new();
        for key in ENV_DIAGNOSTIC_VARS {
            if let Some(value) = env.get(*key) {
                let redacted_value = ontocode_secrets::redact_secrets(value.clone());
                env_details.push(format!("{key} = {redacted_value}"));
            }
        }
        if !env_details.is_empty() {
            diagnostics.push(FeedbackDiagnostic {
                headline: "Environment variables context (redacted)".to_string(),
                details: env_details,
            });
        }

        // 3. Perf diagnostics (CPU cores, OS architecture)
        let num_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(0);
        let mut perf_details = vec![
            format!("OS = {}", std::env::consts::OS),
            format!("Architecture = {}", std::env::consts::ARCH),
        ];
        if num_cpus > 0 {
            perf_details.push(format!("Logical CPUs = {num_cpus}"));
        }
        diagnostics.push(FeedbackDiagnostic {
            headline: "Performance and system diagnostics".to_string(),
            details: perf_details,
        });

        // 4. OAuth and Auth diagnostics (redacted)
        let codex_home = get_codex_home();
        let mut auth_details = Vec::new();
        match ontocode_login::load_auth_dot_json(
            &codex_home,
            ontocode_login::AuthCredentialsStoreMode::File,
        ) {
            Ok(Some(auth)) => {
                auth_details.push(format!("stored auth mode: {:?}", auth.auth_mode));
                auth_details.push(format!(
                    "stored API key present: {}",
                    auth.openai_api_key.is_some()
                ));
                auth_details.push(format!(
                    "stored ChatGPT tokens present: {}",
                    auth.tokens.is_some()
                ));
                auth_details.push(format!(
                    "stored agent identity present: {}",
                    auth.agent_identity.is_some()
                ));
                auth_details.push(format!(
                    "provider OAuth credentials count: {}",
                    auth.provider_oauth_credentials.len()
                ));
                for (idx, cred) in auth.provider_oauth_credentials.iter().enumerate() {
                    let redacted_provider_id =
                        ontocode_secrets::redact_secrets(cred.provider_id().to_string());
                    auth_details.push(format!("  [{idx}] provider_id: {redacted_provider_id}"));
                }
            }
            Ok(None) => {
                auth_details.push("No stored auth found".to_string());
            }
            Err(err) => {
                auth_details.push(format!("Error loading stored auth: {err}"));
            }
        }
        diagnostics.push(FeedbackDiagnostic {
            headline: "Authentication and OAuth diagnostics (redacted)".to_string(),
            details: auth_details,
        });

        Self { diagnostics }
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn diagnostics(&self) -> &[FeedbackDiagnostic] {
        &self.diagnostics
    }

    pub fn attachment_text(&self) -> Option<String> {
        if self.diagnostics.is_empty() {
            return None;
        }

        let mut lines = vec!["Connectivity diagnostics".to_string(), String::new()];
        for diagnostic in &self.diagnostics {
            lines.push(format!("- {}", diagnostic.headline));
            lines.extend(
                diagnostic
                    .details
                    .iter()
                    .map(|detail| format!("  - {detail}")),
            );
        }

        Some(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::FeedbackDiagnostics;

    #[test]
    fn collect_from_pairs_reports_raw_values_and_attachment() {
        let diagnostics = FeedbackDiagnostics::collect_from_pairs([
            (
                "HTTPS_PROXY",
                "https://user:password@secure-proxy.example.com:443?secret=secret12345",
            ),
            ("http_proxy", "proxy.example.com:8080"),
            ("all_proxy", "socks5h://all-proxy.example.com:1080"),
        ]);

        let proxy_diag = diagnostics
            .diagnostics()
            .iter()
            .find(|d| d.headline.contains("Proxy"))
            .unwrap();
        assert_eq!(
            proxy_diag.headline,
            "Proxy environment variables are set and may affect connectivity."
        );
        assert_eq!(
            proxy_diag.details,
            vec![
                "http_proxy = proxy.example.com:8080".to_string(),
                "HTTPS_PROXY = https://user:password@secure-proxy.example.com:443?secret=[REDACTED_SECRET]".to_string(),
                "all_proxy = socks5h://all-proxy.example.com:1080".to_string(),
            ]
        );

        assert!(
            diagnostics.attachment_text().unwrap().contains("HTTPS_PROXY = https://user:password@secure-proxy.example.com:443?secret=[REDACTED_SECRET]"),
            "Expected redacted proxy URL in attachment text, got: {:?}",
            diagnostics.attachment_text()
        );
    }

    #[test]
    fn collect_from_pairs_ignores_absent_values() {
        let diagnostics = FeedbackDiagnostics::collect_from_pairs(Vec::<(String, String)>::new());
        assert!(
            diagnostics
                .diagnostics()
                .iter()
                .all(|d| !d.headline.contains("Proxy"))
        );
    }

    #[test]
    fn collect_from_pairs_preserves_whitespace_and_empty_values() {
        let diagnostics =
            FeedbackDiagnostics::collect_from_pairs([("HTTP_PROXY", "  proxy with spaces  ")]);

        let proxy_diag = diagnostics
            .diagnostics()
            .iter()
            .find(|d| d.headline.contains("Proxy"))
            .unwrap();
        assert_eq!(
            proxy_diag.details,
            vec!["HTTP_PROXY =   proxy with spaces  ".to_string()]
        );
    }

    #[test]
    fn collect_from_pairs_reports_values_verbatim_when_clean() {
        let proxy_value = "not a valid proxy";
        let diagnostics = FeedbackDiagnostics::collect_from_pairs([("HTTP_PROXY", proxy_value)]);

        let proxy_diag = diagnostics
            .diagnostics()
            .iter()
            .find(|d| d.headline.contains("Proxy"))
            .unwrap();
        assert_eq!(
            proxy_diag.details,
            vec!["HTTP_PROXY = not a valid proxy".to_string()]
        );
    }

    #[test]
    fn collect_from_pairs_redacts_oauth_keys_and_cookies() {
        let diagnostics = FeedbackDiagnostics::collect_from_pairs([
            ("PATH", "/usr/bin:/bin"),
            ("HTTP_PROXY", "Bearer abc123xyz78901234"),
            ("ALL_PROXY", "sk-12345678901234567890"),
        ]);

        let proxy_diag = diagnostics
            .diagnostics()
            .iter()
            .find(|d| d.headline.contains("Proxy"))
            .unwrap();
        assert!(
            proxy_diag
                .details
                .contains(&"HTTP_PROXY = Bearer [REDACTED_SECRET]".to_string())
        );
        assert!(
            proxy_diag
                .details
                .contains(&"ALL_PROXY = [REDACTED_SECRET]".to_string())
        );
    }
}
