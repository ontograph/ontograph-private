//! Local OAuth callback server for CLI login.
//!
//! This module runs the short-lived localhost server used by interactive sign-in.
//!
//! The callback flow has two competing responsibilities:
//!
//! - preserve enough backend and transport detail for developers, sysadmins, and support
//!   engineers to diagnose failed sign-ins
//! - avoid persisting secrets or sensitive URL/query data into normal application logs
//!
//! This module therefore keeps the user-facing error path and the structured-log path separate.
//! Returned `io::Error` values still carry the detail needed by CLI/browser callers, while
//! structured logs only emit explicitly reviewed fields plus redacted URL/error values.
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::io::{self};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use crate::auth::AuthDotJson;
use crate::auth::load_auth_dot_json;
use crate::auth::revoke_auth_tokens;
use crate::auth::save_auth;
use crate::auth::should_revoke_auth_tokens;
use crate::auth::upsert_provider_oauth_credential;
use crate::default_client::originator;
use crate::gemini_oauth::GEMINI_TOKEN_ENDPOINT;
use crate::gemini_oauth::GeminiOAuthProvenance;
use crate::gemini_oauth::GeminiOAuthTokens;
use crate::pkce::PkceCodes;
use crate::pkce::generate_pkce;
use crate::token_data::TokenData;
use crate::token_data::parse_chatgpt_jwt_claims;
use base64::Engine;
use chrono::Utc;
use ontocode_app_server_protocol::AuthMode;
use ontocode_client::build_reqwest_client_with_custom_ca;
use ontocode_config::types::AuthCredentialsStoreMode;
use ontocode_utils_template::Template;
use rand::RngCore;
use serde_json::Value as JsonValue;
use tiny_http::Header;
use tiny_http::Request;
use tiny_http::Response;
use tiny_http::Server;
use tiny_http::StatusCode;
use tracing::error;
use tracing::info;
use tracing::warn;

const DEFAULT_ISSUER: &str = "https://auth.openai.com";
const DEFAULT_PORT: u16 = 1455;
const GEMINI_AUTHORIZE_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
#[allow(dead_code)]
const GEMINI_AUTHORIZATION_REDIRECT_URI: &str = "https://codeassist.google.com/authcode";
const GEMINI_AUTHORIZATION_SCOPE: &str =
    "openid email profile https://www.googleapis.com/auth/cloud-platform";
// Keep in sync with the Codex CLI Hydra redirect URI allow-list.
const FALLBACK_PORT: u16 = 1457;
static LOGIN_ERROR_PAGE_TEMPLATE: LazyLock<Template> = LazyLock::new(|| {
    Template::parse(include_str!("assets/error.html"))
        .unwrap_or_else(|err| panic!("login error page template must parse: {err}"))
});

/// Options for launching the local login callback server.
#[derive(Debug, Clone)]
pub struct ServerOptions {
    pub codex_home: PathBuf,
    pub client_id: String,
    pub issuer: String,
    pub port: u16,
    pub open_browser: bool,
    pub force_state: Option<String>,
    pub forced_chatgpt_workspace_id: Option<Vec<String>>,
    pub codex_streamlined_login: bool,
    pub cli_auth_credentials_store_mode: AuthCredentialsStoreMode,
}

impl ServerOptions {
    /// Creates a server configuration with the default issuer and port.
    pub fn new(
        codex_home: PathBuf,
        client_id: String,
        forced_chatgpt_workspace_id: Option<Vec<String>>,
        cli_auth_credentials_store_mode: AuthCredentialsStoreMode,
    ) -> Self {
        Self {
            codex_home,
            client_id,
            issuer: DEFAULT_ISSUER.to_string(),
            port: DEFAULT_PORT,
            open_browser: true,
            force_state: None,
            forced_chatgpt_workspace_id,
            codex_streamlined_login: false,
            cli_auth_credentials_store_mode,
        }
    }
}

/// Handle for a running login callback server.
pub struct LoginServer {
    pub auth_url: String,
    pub actual_port: u16,
    server_handle: tokio::task::JoinHandle<io::Result<()>>,
    shutdown_handle: ShutdownHandle,
}

/// Handle for a running Gemini browser OAuth callback server.
pub struct GeminiLoginServer {
    pub auth_url: String,
    pub actual_port: u16,
    server_handle: tokio::task::JoinHandle<io::Result<GeminiOAuthTokens>>,
    shutdown_handle: ShutdownHandle,
}

/// Options for running a local Gemini OAuth callback server.
#[derive(Debug, Clone)]
pub struct GeminiLoginServerOptions {
    pub codex_home: PathBuf,
    pub client_id: String,
    pub credential_id: String,
    pub scopes: Vec<String>,
    pub account_id: Option<String>,
    pub project_id: Option<String>,
    pub token_endpoint: String,
    pub port: u16,
    pub open_browser: bool,
    pub force_state: Option<String>,
    pub cli_auth_credentials_store_mode: AuthCredentialsStoreMode,
}

impl Default for GeminiLoginServerOptions {
    fn default() -> Self {
        Self {
            codex_home: PathBuf::new(),
            client_id: String::new(),
            credential_id: String::new(),
            scopes: vec![GEMINI_AUTHORIZATION_SCOPE.to_string()],
            account_id: None,
            project_id: None,
            token_endpoint: GEMINI_TOKEN_ENDPOINT.to_string(),
            port: DEFAULT_PORT,
            open_browser: true,
            force_state: None,
            cli_auth_credentials_store_mode: AuthCredentialsStoreMode::File,
        }
    }
}

impl GeminiLoginServer {
    /// Waits for the Gemini callback flow to finish.
    pub async fn block_until_done(self) -> io::Result<GeminiOAuthTokens> {
        self.server_handle.await.map_err(|err| {
            io::Error::other(format!("gemini login server task panicked: {err:?}"))
        })?
    }

    /// Requests shutdown of the callback server.
    pub fn cancel(&self) {
        self.shutdown_handle.shutdown();
    }

    /// Returns a cloneable cancel handle for the running server.
    pub fn cancel_handle(&self) -> ShutdownHandle {
        self.shutdown_handle.clone()
    }
}

/// Starts a local Gemini OAuth callback server and returns a browser auth URL.
pub fn run_gemini_login_server(opts: GeminiLoginServerOptions) -> io::Result<GeminiLoginServer> {
    run_gemini_login_server_internal(opts)
}

fn run_gemini_login_server_internal(
    opts: GeminiLoginServerOptions,
) -> io::Result<GeminiLoginServer> {
    let pkce = generate_pkce();
    let force_state = opts.force_state.clone();
    let open_browser = opts.open_browser;
    let token_endpoint = opts.token_endpoint.clone();
    let scopes = opts.scopes.clone();
    let account_id = opts.account_id.clone();
    let project_id = opts.project_id.clone();
    let client_id = opts.client_id.clone();
    let credential_id = opts.credential_id.clone();
    let port = opts.port;
    let state = force_state.unwrap_or_else(generate_state);

    let server = bind_server(port)?;
    let actual_port = match server.server_addr().to_ip() {
        Some(addr) => addr.port(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::AddrInUse,
                "Unable to determine the server port",
            ));
        }
    };

    let server = Arc::new(server);
    let redirect_uri = format!("http://localhost:{actual_port}/auth/callback");
    let auth_url = build_gemini_authorize_url(&client_id, &redirect_uri, &pkce, &state);

    if open_browser && webbrowser::open(&auth_url).is_err() {
        return Err(io::Error::other("failed to open browser for Gemini login"));
    }

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Request>(16);
    let _server_handle = {
        let server = server.clone();
        thread::spawn(move || -> io::Result<()> {
            while let Ok(request) = server.recv() {
                match tx.blocking_send(request) {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Failed to send request to channel: {error}");
                        return Err(io::Error::other("Failed to send request to channel"));
                    }
                }
            }
            Ok(())
        })
    };

    let shutdown_notify = Arc::new(tokio::sync::Notify::new());
    let server_handle = {
        let shutdown_notify = shutdown_notify.clone();
        let codex_home = opts.codex_home.clone();
        let auth_credentials_store_mode = opts.cli_auth_credentials_store_mode;
        tokio::spawn(async move {
            let result = loop {
                tokio::select! {
                    _ = shutdown_notify.notified() => {
                        break Err(io::Error::other("Login was not completed"));
                    }
                    maybe_req = rx.recv() => {
                        let Some(req) = maybe_req else {
                            break Err(io::Error::other("Login was not completed"));
                        };

                        let maybe_exit = {
                            let parsed_path = url::Url::parse(&format!("http://localhost{}", req.url()))
                                .ok()
                                .map(|url| url.path().to_string())
                                .unwrap_or_default();

                            match parsed_path.as_str() {
                                "/auth/callback" | "/success" => {
                                    let url_raw = req.url().to_string();
                                    match process_gemini_callback_request(
                                        &url_raw,
                                        &client_id,
                                        &credential_id,
                                        &redirect_uri,
                                        &pkce,
                                        &state,
                                        &token_endpoint,
                                        &codex_home,
                                        auth_credentials_store_mode,
                                        scopes.clone(),
                                        account_id.clone(),
                                        project_id.clone(),
                                    )
                                    .await
                                    {
                                        GeminiRequestResult::Continue(response) => {
                                            let _ = tokio::task::spawn_blocking(move || {
                                                req.respond(response)
                                            })
                                            .await;
                                            None
                                        }
                                        GeminiRequestResult::Exit {
                                            response,
                                            result,
                                        } => {
                                            let exit_response = match response {
                                                HandledRequest::Response(response) => {
                                                    tokio::task::spawn_blocking(move || {
                                                        req.respond(response)
                                                    })
                                                    .await
                                                }
                                                HandledRequest::ResponseAndExit {
                                                    headers,
                                                    body,
                                                    result: _,
                                                } => tokio::task::spawn_blocking(move || {
                                                    send_response_with_disconnect(req, headers, body)
                                                })
                                                .await,
                                                HandledRequest::RedirectWithHeader(header) => {
                                                    let redirect = Response::empty(302)
                                                        .with_header(header);
                                                    tokio::task::spawn_blocking(move || {
                                                        req.respond(redirect)
                                                    })
                                                    .await
                                                }
                                            };
                                            if let Ok(Err(err)) = exit_response {
                                                Some(Err(err))
                                            } else {
                                                Some(result)
                                            }
                                        }
                                    }
                                }
                                "/cancel" => {
                                    let _ = tokio::task::spawn_blocking(move || {
                                        send_response_with_disconnect(
                                            req,
                                            vec![],
                                            b"Login cancelled".to_vec(),
                                        )
                                    })
                                    .await;
                                    Some(Err(io::Error::new(
                                        io::ErrorKind::Interrupted,
                                        "Login cancelled",
                                    )))
                                }
                                _ => {
                                    let _ = tokio::task::spawn_blocking(move || {
                                        req.respond(
                                            Response::from_string("Not Found").with_status_code(404),
                                        )
                                    })
                                    .await;
                                    None
                                }
                            }
                        };

                        if let Some(result) = maybe_exit {
                            break result;
                        }
                    }
                }
            };
            server.unblock();
            result
        })
    };

    Ok(GeminiLoginServer {
        auth_url,
        actual_port,
        server_handle,
        shutdown_handle: ShutdownHandle { shutdown_notify },
    })
}

enum GeminiRequestResult {
    Continue(Response<Cursor<Vec<u8>>>),
    Exit {
        response: HandledRequest,
        result: io::Result<GeminiOAuthTokens>,
    },
}

async fn process_gemini_callback_request(
    url_raw: &str,
    client_id: &str,
    credential_id: &str,
    redirect_uri: &str,
    pkce: &PkceCodes,
    expected_state: &str,
    token_endpoint: &str,
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    scopes: Vec<String>,
    account_id: Option<String>,
    project_id: Option<String>,
) -> GeminiRequestResult {
    let parsed = match parse_callback_query(url_raw) {
        Ok(parsed) => parsed,
        Err(err) => {
            return GeminiRequestResult::Exit {
                response: login_error_response(
                    &err.to_string(),
                    io::ErrorKind::InvalidData,
                    Some("invalid_callback_url"),
                    None,
                ),
                result: Err(err),
            };
        }
    };

    if parsed.path != "/auth/callback" {
        return GeminiRequestResult::Continue(
            Response::from_string("Not Found").with_status_code(404),
        );
    }

    if !callback_state_matches(&parsed, expected_state) {
        let result = io::Error::new(io::ErrorKind::PermissionDenied, "State mismatch");
        return GeminiRequestResult::Exit {
            response: login_error_response(
                &result.to_string(),
                io::ErrorKind::PermissionDenied,
                Some("state_mismatch"),
                None,
            ),
            result: Err(result),
        };
    }

    if let Some(error_code) = parsed.error {
        let message =
            oauth_callback_error_message(&error_code, parsed.error_description.as_deref());
        let result = io::Error::new(io::ErrorKind::PermissionDenied, message.clone());
        return GeminiRequestResult::Exit {
            response: login_error_response(
                &message,
                io::ErrorKind::PermissionDenied,
                Some(&error_code),
                parsed.error_description.as_deref(),
            ),
            result: Err(result),
        };
    }

    let _code = match parsed.code {
        Some(code) => code,
        None => {
            let result = io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing authorization code. Sign-in could not be completed.",
            );
            return GeminiRequestResult::Exit {
                response: login_error_response(
                    &result.to_string(),
                    io::ErrorKind::InvalidData,
                    Some("missing_authorization_code"),
                    None,
                ),
                result: Err(result),
            };
        }
    };

    let url_for_exchange = url_raw.to_string();
    let exchanged_tokens = match exchange_gemini_callback_to_provider_oauth_with_token_endpoint(
        codex_home,
        auth_credentials_store_mode,
        &url_for_exchange,
        expected_state,
        pkce,
        client_id,
        redirect_uri,
        credential_id,
        &scopes,
        account_id,
        project_id,
        token_endpoint,
    )
    .await
    {
        Ok(tokens) => tokens,
        Err(err) => {
            return GeminiRequestResult::Exit {
                response: login_error_response(
                    &format!("Token exchange failed: {err}"),
                    io::ErrorKind::Other,
                    Some("token_exchange_failed"),
                    None,
                ),
                result: Err(err),
            };
        }
    };

    GeminiRequestResult::Exit {
        response: HandledRequest::ResponseAndExit {
            headers: Vec::new(),
            body: b"Gemini login completed. You may close this tab.".to_vec(),
            result: Ok(()),
        },
        result: Ok(exchanged_tokens),
    }
}

impl LoginServer {
    /// Waits for the login callback loop to finish.
    pub async fn block_until_done(self) -> io::Result<()> {
        self.server_handle
            .await
            .map_err(|err| io::Error::other(format!("login server thread panicked: {err:?}")))?
    }

    /// Requests shutdown of the callback server.
    pub fn cancel(&self) {
        self.shutdown_handle.shutdown();
    }

    /// Returns a cloneable cancel handle for the running server.
    pub fn cancel_handle(&self) -> ShutdownHandle {
        self.shutdown_handle.clone()
    }
}

/// Handle used to signal the login server loop to exit.
#[derive(Clone, Debug)]
pub struct ShutdownHandle {
    shutdown_notify: Arc<tokio::sync::Notify>,
}

impl ShutdownHandle {
    /// Signals the login loop to terminate.
    pub fn shutdown(&self) {
        self.shutdown_notify.notify_one();
    }
}

/// Starts a local callback server and returns the browser auth URL.
pub fn run_login_server(opts: ServerOptions) -> io::Result<LoginServer> {
    let pkce = generate_pkce();
    let state = opts.force_state.clone().unwrap_or_else(generate_state);

    let server = bind_server(opts.port)?;
    let actual_port = match server.server_addr().to_ip() {
        Some(addr) => addr.port(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::AddrInUse,
                "Unable to determine the server port",
            ));
        }
    };
    let server = Arc::new(server);

    let redirect_uri = format!("http://localhost:{actual_port}/auth/callback");
    let auth_url = build_authorize_url(
        &opts.issuer,
        &opts.client_id,
        &redirect_uri,
        &pkce,
        &state,
        opts.forced_chatgpt_workspace_id.as_deref(),
    );

    if opts.open_browser {
        let _ = webbrowser::open(&auth_url);
    }

    // Map blocking reads from server.recv() to an async channel.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Request>(16);
    let _server_handle = {
        let server = server.clone();
        thread::spawn(move || -> io::Result<()> {
            while let Ok(request) = server.recv() {
                match tx.blocking_send(request) {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Failed to send request to channel: {error}");
                        return Err(io::Error::other("Failed to send request to channel"));
                    }
                }
            }
            Ok(())
        })
    };

    let shutdown_notify = Arc::new(tokio::sync::Notify::new());
    let server_handle = {
        let shutdown_notify = shutdown_notify.clone();
        let server = server;
        tokio::spawn(async move {
            let result = loop {
                tokio::select! {
                    _ = shutdown_notify.notified() => {
                        break Err(io::Error::other("Login was not completed"));
                    }
                    maybe_req = rx.recv() => {
                        let Some(req) = maybe_req else {
                            break Err(io::Error::other("Login was not completed"));
                        };

                        let url_raw = req.url().to_string();
                        let response =
                            process_request(&url_raw, &opts, &redirect_uri, &pkce, actual_port, &state).await;

                        let exit_result = match response {
                            HandledRequest::Response(response) => {
                                let _ = tokio::task::spawn_blocking(move || req.respond(response)).await;
                                None
                            }
                            HandledRequest::ResponseAndExit {
                                headers,
                                body,
                                result,
                            } => {
                                let _ = tokio::task::spawn_blocking(move || {
                                    send_response_with_disconnect(req, headers, body)
                                })
                                .await;
                                Some(result)
                            }
                            HandledRequest::RedirectWithHeader(header) => {
                                let redirect = Response::empty(302).with_header(header);
                                let _ = tokio::task::spawn_blocking(move || req.respond(redirect)).await;
                                None
                            }
                        };

                        if let Some(result) = exit_result {
                            break result;
                        }
                    }
                }
            };

            // Ensure that the server is unblocked so the thread dedicated to
            // running `server.recv()` in a loop exits cleanly.
            server.unblock();
            result
        })
    };

    Ok(LoginServer {
        auth_url,
        actual_port,
        server_handle,
        shutdown_handle: ShutdownHandle { shutdown_notify },
    })
}

/// Internal callback handling outcome.
enum HandledRequest {
    Response(Response<Cursor<Vec<u8>>>),
    RedirectWithHeader(Header),
    ResponseAndExit {
        headers: Vec<Header>,
        body: Vec<u8>,
        result: io::Result<()>,
    },
}

async fn process_request(
    url_raw: &str,
    opts: &ServerOptions,
    redirect_uri: &str,
    pkce: &PkceCodes,
    actual_port: u16,
    state: &str,
) -> HandledRequest {
    let parsed_url = match url::Url::parse(&format!("http://localhost{url_raw}")) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("URL parse error: {e}");
            return HandledRequest::Response(
                Response::from_string("Bad Request").with_status_code(400),
            );
        }
    };
    let path = parsed_url.path().to_string();

    match path.as_str() {
        "/auth/callback" => {
            let params: std::collections::HashMap<String, String> =
                parsed_url.query_pairs().into_owned().collect();
            let has_code = params.get("code").is_some_and(|code| !code.is_empty());
            let has_state = params.get("state").is_some_and(|state| !state.is_empty());
            let has_error = params.get("error").is_some_and(|error| !error.is_empty());
            let state_valid = params.get("state").map(String::as_str) == Some(state);
            info!(
                path = %path,
                has_code,
                has_state,
                has_error,
                state_valid,
                "received login callback"
            );
            if !state_valid {
                warn!(
                    path = %path,
                    has_code,
                    has_state,
                    has_error,
                    "login callback state mismatch"
                );
                return HandledRequest::Response(
                    Response::from_string("State mismatch").with_status_code(400),
                );
            }
            if let Some(error_code) = params.get("error") {
                let error_description = params.get("error_description").map(String::as_str);
                let message = oauth_callback_error_message(error_code, error_description);
                eprintln!("OAuth callback error: {message}");
                warn!(
                    error_code,
                    has_error_description = error_description.is_some_and(|s| !s.trim().is_empty()),
                    "oauth callback returned error"
                );
                return login_error_response(
                    &message,
                    io::ErrorKind::PermissionDenied,
                    Some(error_code),
                    error_description,
                );
            }
            let code = match params.get("code") {
                Some(c) if !c.is_empty() => c.clone(),
                _ => {
                    return login_error_response(
                        "Missing authorization code. Sign-in could not be completed.",
                        io::ErrorKind::InvalidData,
                        Some("missing_authorization_code"),
                        /*error_description*/ None,
                    );
                }
            };

            match exchange_code_for_tokens(&opts.issuer, &opts.client_id, redirect_uri, pkce, &code)
                .await
            {
                Ok(tokens) => {
                    if let Err(message) = ensure_workspace_allowed(
                        opts.forced_chatgpt_workspace_id.as_deref(),
                        &tokens.id_token,
                    ) {
                        eprintln!("Workspace restriction error: {message}");
                        return login_error_response(
                            &message,
                            io::ErrorKind::PermissionDenied,
                            Some("workspace_restriction"),
                            /*error_description*/ None,
                        );
                    }
                    // Obtain API key via token-exchange and persist
                    let api_key = obtain_api_key(&opts.issuer, &opts.client_id, &tokens.id_token)
                        .await
                        .ok();
                    if let Err(err) = persist_tokens_async(
                        &opts.codex_home,
                        api_key.clone(),
                        tokens.id_token.clone(),
                        tokens.access_token.clone(),
                        tokens.refresh_token.clone(),
                        opts.cli_auth_credentials_store_mode,
                    )
                    .await
                    {
                        eprintln!("Persist error: {err}");
                        return login_error_response(
                            "Sign-in completed but credentials could not be saved locally.",
                            io::ErrorKind::Other,
                            Some("persist_failed"),
                            Some(&err.to_string()),
                        );
                    }

                    let success_url = compose_success_url(
                        actual_port,
                        &opts.issuer,
                        &tokens.id_token,
                        &tokens.access_token,
                        opts.codex_streamlined_login,
                    );
                    match tiny_http::Header::from_bytes(&b"Location"[..], success_url.as_bytes()) {
                        Ok(header) => HandledRequest::RedirectWithHeader(header),
                        Err(_) => login_error_response(
                            "Sign-in completed but redirecting back to Codex failed.",
                            io::ErrorKind::Other,
                            Some("redirect_failed"),
                            /*error_description*/ None,
                        ),
                    }
                }
                Err(err) => {
                    eprintln!("Token exchange error: {err}");
                    error!("login callback token exchange failed");
                    login_error_response(
                        &format!("Token exchange failed: {err}"),
                        io::ErrorKind::Other,
                        Some("token_exchange_failed"),
                        /*error_description*/ None,
                    )
                }
            }
        }
        "/success" => {
            let use_streamlined_success = parsed_url
                .query_pairs()
                .any(|(key, value)| key == "codex_streamlined_login" && value == "true");
            let body = if use_streamlined_success {
                include_str!("assets/success.html")
            } else {
                include_str!("assets/success_legacy.html")
            };
            HandledRequest::ResponseAndExit {
                headers: match Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"text/html; charset=utf-8"[..],
                ) {
                    Ok(header) => vec![header],
                    Err(_) => Vec::new(),
                },
                body: body.as_bytes().to_vec(),
                result: Ok(()),
            }
        }
        "/cancel" => HandledRequest::ResponseAndExit {
            headers: Vec::new(),
            body: b"Login cancelled".to_vec(),
            result: Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Login cancelled",
            )),
        },
        _ => HandledRequest::Response(Response::from_string("Not Found").with_status_code(404)),
    }
}

/// tiny_http filters `Connection` headers out of `Response` objects, so using
/// `req.respond` never informs the client (or the library) that a keep-alive
/// socket should be closed. That leaves the per-connection worker parked in a
/// loop waiting for more requests, which in turn causes the next login attempt
/// to hang on the old connection. This helper bypasses tiny_http’s response
/// machinery: it extracts the raw writer, prints the HTTP response manually,
/// and always appends `Connection: close`, ensuring the socket is closed from
/// the server side. Ideally, tiny_http would provide an API to control
/// server-side connection persistence, but it does not.
fn send_response_with_disconnect(
    req: Request,
    mut headers: Vec<Header>,
    body: Vec<u8>,
) -> io::Result<()> {
    let status = StatusCode(200);
    let mut writer = req.into_writer();
    let reason = status.default_reason_phrase();
    write!(writer, "HTTP/1.1 {} {}\r\n", status.0, reason)?;
    headers.retain(|h| !h.field.equiv("Connection"));
    if let Ok(close_header) = Header::from_bytes(&b"Connection"[..], &b"close"[..]) {
        headers.push(close_header);
    }

    let content_length_value = format!("{}", body.len());
    if let Ok(content_length_header) =
        Header::from_bytes(&b"Content-Length"[..], content_length_value.as_bytes())
    {
        headers.push(content_length_header);
    }

    for header in headers {
        write!(
            writer,
            "{}: {}\r\n",
            header.field.as_str(),
            header.value.as_str()
        )?;
    }

    writer.write_all(b"\r\n")?;
    writer.write_all(&body)?;
    writer.flush()
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct ParsedCallbackQuery {
    path: String,
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[allow(dead_code)]
pub(crate) fn parse_callback_query(url_raw: &str) -> io::Result<ParsedCallbackQuery> {
    let parsed_url = url::Url::parse(&format!("http://localhost{url_raw}")).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid callback URL: {error}"),
        )
    })?;
    let params: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    Ok(ParsedCallbackQuery {
        path: parsed_url.path().to_string(),
        code: params
            .get("code")
            .filter(|value| !value.is_empty())
            .cloned(),
        state: params
            .get("state")
            .filter(|value| !value.is_empty())
            .cloned(),
        error: params
            .get("error")
            .filter(|value| !value.is_empty())
            .cloned(),
        error_description: params
            .get("error_description")
            .filter(|value| !value.is_empty())
            .cloned(),
    })
}

#[allow(dead_code)]
pub(crate) fn callback_state_matches(query: &ParsedCallbackQuery, expected_state: &str) -> bool {
    query.state.as_deref() == Some(expected_state)
}

fn build_authorize_url(
    issuer: &str,
    client_id: &str,
    redirect_uri: &str,
    pkce: &PkceCodes,
    state: &str,
    forced_chatgpt_workspace_ids: Option<&[String]>,
) -> String {
    let mut query = vec![
        ("response_type".to_string(), "code".to_string()),
        ("client_id".to_string(), client_id.to_string()),
        ("redirect_uri".to_string(), redirect_uri.to_string()),
        (
            "scope".to_string(),
            "openid profile email offline_access api.connectors.read api.connectors.invoke"
                .to_string(),
        ),
        (
            "code_challenge".to_string(),
            pkce.code_challenge.to_string(),
        ),
        ("code_challenge_method".to_string(), "S256".to_string()),
        ("id_token_add_organizations".to_string(), "true".to_string()),
        ("codex_cli_simplified_flow".to_string(), "true".to_string()),
        ("state".to_string(), state.to_string()),
        ("originator".to_string(), originator().value),
    ];
    if let Some(workspace_ids) = forced_chatgpt_workspace_ids {
        query.push(("allowed_workspace_id".to_string(), workspace_ids.join(",")));
    }
    let qs = query
        .into_iter()
        .map(|(k, v)| format!("{k}={}", urlencoding::encode(&v)))
        .collect::<Vec<_>>()
        .join("&");
    format!("{issuer}/oauth/authorize?{qs}")
}

#[allow(dead_code)]
pub(crate) fn build_authorize_url_with_extras(
    authorize_endpoint: &str,
    client_id: &str,
    redirect_uri: &str,
    pkce: &PkceCodes,
    state: &str,
    scope: &str,
    include_originator: bool,
    extra_query: &[(&str, &str)],
) -> String {
    let mut query = vec![
        ("response_type".to_string(), "code".to_string()),
        ("client_id".to_string(), client_id.to_string()),
        ("redirect_uri".to_string(), redirect_uri.to_string()),
        ("scope".to_string(), scope.to_string()),
        (
            "code_challenge".to_string(),
            pkce.code_challenge.to_string(),
        ),
        ("code_challenge_method".to_string(), "S256".to_string()),
        ("state".to_string(), state.to_string()),
    ];
    if include_originator {
        query.push(("originator".to_string(), originator().value));
    }
    query.extend(
        extra_query
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string())),
    );
    let qs = query
        .into_iter()
        .map(|(k, v)| format!("{k}={}", urlencoding::encode(&v)))
        .collect::<Vec<_>>()
        .join("&");
    format!("{authorize_endpoint}?{qs}")
}

/// Builds a Gemini OAuth browser authorization URL with offline refresh defaults.
pub(crate) fn build_gemini_authorize_url(
    client_id: &str,
    redirect_uri: &str,
    pkce: &PkceCodes,
    state: &str,
) -> String {
    build_authorize_url_with_extras(
        GEMINI_AUTHORIZE_URL,
        client_id,
        redirect_uri,
        pkce,
        state,
        GEMINI_AUTHORIZATION_SCOPE,
        false,
        &[
            ("access_type", "offline"),
            ("prompt", "consent"),
            ("include_granted_scopes", "true"),
        ],
    )
}

/// Builds the manual Gemini authorization URL shown when browser flow is unavailable.
#[allow(dead_code)]
pub(crate) fn build_gemini_manual_authorize_url(
    client_id: &str,
    pkce: &PkceCodes,
    state: &str,
) -> String {
    build_gemini_authorize_url(client_id, GEMINI_AUTHORIZATION_REDIRECT_URI, pkce, state)
}

fn generate_state() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn send_cancel_request(port: u16) -> io::Result<()> {
    let addr: SocketAddr = format!("127.0.0.1:{port}")
        .parse()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(2))?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;

    stream.write_all(b"GET /cancel HTTP/1.1\r\n")?;
    stream.write_all(format!("Host: 127.0.0.1:{port}\r\n").as_bytes())?;
    stream.write_all(b"Connection: close\r\n\r\n")?;

    let mut buf = [0u8; 64];
    let _ = stream.read(&mut buf);
    Ok(())
}

fn bind_server(port: u16) -> io::Result<Server> {
    let preferred_bind_address = format!("127.0.0.1:{port}");
    let fallback_bind_address = format!("127.0.0.1:{FALLBACK_PORT}");
    let mut bind_address = preferred_bind_address.clone();
    let mut cancel_attempted = false;
    let mut attempts = 0;
    let mut using_fallback_port = false;
    const MAX_ATTEMPTS: u32 = 10;
    const RETRY_DELAY: Duration = Duration::from_millis(200);

    loop {
        match Server::http(&bind_address) {
            Ok(server) => return Ok(server),
            Err(err) => {
                attempts += 1;
                let is_addr_in_use = err
                    .downcast_ref::<io::Error>()
                    .map(|io_err| io_err.kind() == io::ErrorKind::AddrInUse)
                    .unwrap_or(false);

                // If the address is in use, there may be another instance of the login server
                // running. Attempt to cancel it and retry before falling back.
                if is_addr_in_use {
                    if !cancel_attempted && !using_fallback_port {
                        cancel_attempted = true;
                        if let Err(cancel_err) = send_cancel_request(port) {
                            eprintln!("Failed to cancel previous login server: {cancel_err}");
                        }
                    }

                    thread::sleep(RETRY_DELAY);

                    if attempts >= MAX_ATTEMPTS {
                        if port == DEFAULT_PORT && !using_fallback_port {
                            warn!(
                                %preferred_bind_address,
                                %fallback_bind_address,
                                "default login callback port is unavailable; falling back to the registered fallback port"
                            );
                            bind_address = fallback_bind_address.clone();
                            attempts = 0;
                            using_fallback_port = true;
                            continue;
                        }

                        return Err(io::Error::new(
                            io::ErrorKind::AddrInUse,
                            format!("Port {bind_address} is already in use"),
                        ));
                    }

                    continue;
                }

                return Err(io::Error::other(err));
            }
        }
    }
}

/// Tokens returned by the OAuth authorization-code exchange.
pub(crate) struct ExchangedTokens {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenEndpointErrorDetail {
    error_code: Option<String>,
    error_message: Option<String>,
    display_message: String,
}

impl std::fmt::Display for TokenEndpointErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_message.fmt(f)
    }
}

const REDACTED_URL_VALUE: &str = "<redacted>";
const SENSITIVE_URL_QUERY_KEYS: &[&str] = &[
    "access_token",
    "api_key",
    "authorization",
    "client_secret",
    "code",
    "code_verifier",
    "cookie",
    "id_token",
    "key",
    "refresh_token",
    "requested_token",
    "set-cookie",
    "state",
    "subject_token",
    "token",
];

fn redact_sensitive_query_value(key: &str, value: &str) -> String {
    if SENSITIVE_URL_QUERY_KEYS
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(key))
    {
        REDACTED_URL_VALUE.to_string()
    } else {
        value.to_string()
    }
}

/// Redacts URL components that commonly carry auth secrets while preserving the host/path shape.
///
/// This keeps developer-facing logs useful for debugging transport failures without persisting
/// tokens, callback codes, fragments, or embedded credentials.
fn redact_sensitive_url_parts(url: &mut url::Url) {
    let _ = url.set_username("");
    let _ = url.set_password(None);
    url.set_fragment(None);

    let query_pairs = url
        .query_pairs()
        .map(|(key, value)| {
            let key = key.into_owned();
            let value = value.into_owned();
            (key.clone(), redact_sensitive_query_value(&key, &value))
        })
        .collect::<Vec<_>>();

    if query_pairs.is_empty() {
        url.set_query(None);
        return;
    }

    let redacted_query = query_pairs
        .into_iter()
        .fold(
            url::form_urlencoded::Serializer::new(String::new()),
            |mut serializer, (key, value)| {
                serializer.append_pair(&key, &value);
                serializer
            },
        )
        .finish();
    url.set_query(Some(&redacted_query));
}

/// Redacts any URL attached to a reqwest transport error before it is logged or returned.
fn redact_sensitive_error_url(mut err: reqwest::Error) -> reqwest::Error {
    if let Some(url) = err.url_mut() {
        redact_sensitive_url_parts(url);
    }
    err
}

/// Sanitizes a free-form URL string for structured logging.
///
/// This is used for caller-supplied issuer values, which may contain credentials or query
/// parameters on non-default deployments.
fn sanitize_url_for_logging(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(mut url) => {
            redact_sensitive_url_parts(&mut url);
            url.to_string()
        }
        Err(_) => "<invalid-url>".to_string(),
    }
}

async fn exchange_code_for_tokens_with_token_endpoint(
    token_endpoint: &str,
    client_id: &str,
    redirect_uri: &str,
    pkce: &PkceCodes,
    code: &str,
) -> io::Result<ExchangedTokens> {
    #[derive(serde::Deserialize)]
    struct TokenResponse {
        id_token: String,
        access_token: String,
        refresh_token: String,
        expires_in: Option<u64>,
    }

    let client = build_reqwest_client_with_custom_ca(reqwest::Client::builder())?;
    let token_endpoint = token_endpoint.trim();
    info!(
        token_endpoint = %sanitize_url_for_logging(token_endpoint),
        redirect_uri = %redirect_uri,
        "starting oauth token exchange"
    );
    let resp = client
        .post(token_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&code_verifier={}",
            urlencoding::encode(code),
            urlencoding::encode(redirect_uri),
            urlencoding::encode(client_id),
            urlencoding::encode(&pkce.code_verifier)
        ))
        .send()
        .await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(error) => {
            let error = redact_sensitive_error_url(error);
            error!(
                is_timeout = error.is_timeout(),
                is_connect = error.is_connect(),
                is_request = error.is_request(),
                error = %error,
                "oauth token exchange transport failure"
            );
            return Err(io::Error::other(error));
        }
    };

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.map_err(io::Error::other)?;
        let detail = parse_token_endpoint_error(&body);
        warn!(
            %status,
            error_code = detail.error_code.as_deref().unwrap_or("unknown"),
            error_message = detail.error_message.as_deref().unwrap_or("unknown"),
            "oauth token exchange returned non-success status"
        );
        return Err(io::Error::other(format!(
            "token endpoint returned status {status}: {detail}"
        )));
    }

    let tokens: TokenResponse = resp.json().await.map_err(io::Error::other)?;
    let expires_at = tokens
        .expires_in
        .and_then(|seconds| i64::try_from(seconds).ok())
        .and_then(|seconds| Utc::now().checked_add_signed(chrono::Duration::seconds(seconds)))
        .map(|timestamp| timestamp.timestamp() as u64);
    info!(%status, "oauth token exchange succeeded");
    Ok(ExchangedTokens {
        id_token: tokens.id_token,
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at,
    })
}

/// Exchanges an authorization code for tokens.
///
/// The returned error remains suitable for user-facing CLI/browser surfaces, so backend-provided
/// non-JSON error text is preserved there. Structured logging stays narrower: it logs reviewed
/// fields from parsed token responses and redacted transport errors, but does not log the final
/// callback-layer `%err` string.
pub(crate) async fn exchange_code_for_tokens(
    issuer: &str,
    client_id: &str,
    redirect_uri: &str,
    pkce: &PkceCodes,
    code: &str,
) -> io::Result<ExchangedTokens> {
    let token_endpoint = format!("{}/oauth/token", issuer.trim_end_matches('/'));
    info!(issuer = %sanitize_url_for_logging(issuer), token_endpoint = %sanitize_url_for_logging(&token_endpoint), "starting oauth token exchange");
    exchange_code_for_tokens_with_token_endpoint(
        &token_endpoint,
        client_id,
        redirect_uri,
        pkce,
        code,
    )
    .await
}

#[allow(dead_code)]
pub(crate) async fn exchange_gemini_code_to_provider_oauth_with_token_endpoint(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    code: &str,
    client_id: &str,
    redirect_uri: &str,
    credential_id: &str,
    scopes: &[String],
    account_id: Option<String>,
    project_id: Option<String>,
    token_endpoint: &str,
    pkce: &PkceCodes,
) -> io::Result<GeminiOAuthTokens> {
    exchange_gemini_code_to_provider_oauth_with_token_endpoint_for_provenance(
        codex_home,
        auth_credentials_store_mode,
        code,
        client_id,
        redirect_uri,
        credential_id,
        scopes,
        account_id,
        project_id,
        token_endpoint,
        pkce,
        GeminiOAuthProvenance::UserCode,
    )
    .await
}

pub(crate) async fn exchange_gemini_callback_to_provider_oauth_with_token_endpoint(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    callback_url: &str,
    expected_state: &str,
    pkce: &PkceCodes,
    client_id: &str,
    redirect_uri: &str,
    credential_id: &str,
    scopes: &[String],
    account_id: Option<String>,
    project_id: Option<String>,
    token_endpoint: &str,
) -> io::Result<GeminiOAuthTokens> {
    let parsed = parse_callback_query(callback_url)?;

    let state_matches = callback_state_matches(&parsed, expected_state);
    if !state_matches {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "State mismatch",
        ));
    }

    if let Some(error_code) = parsed.error {
        let error_message =
            oauth_callback_error_message(&error_code, parsed.error_description.as_deref());
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            error_message,
        ));
    }

    let code = parsed.code.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing authorization code. Sign-in could not be completed.",
        )
    })?;
    exchange_gemini_code_to_provider_oauth_with_token_endpoint_for_provenance(
        codex_home,
        auth_credentials_store_mode,
        &code,
        client_id,
        redirect_uri,
        credential_id,
        scopes,
        account_id,
        project_id,
        token_endpoint,
        pkce,
        GeminiOAuthProvenance::Browser,
    )
    .await
}

async fn exchange_gemini_code_to_provider_oauth_with_token_endpoint_for_provenance(
    codex_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    code: &str,
    client_id: &str,
    redirect_uri: &str,
    credential_id: &str,
    scopes: &[String],
    account_id: Option<String>,
    project_id: Option<String>,
    token_endpoint: &str,
    pkce: &PkceCodes,
    provenance: GeminiOAuthProvenance,
) -> io::Result<GeminiOAuthTokens> {
    let exchanged_tokens = exchange_code_for_tokens_with_token_endpoint(
        token_endpoint,
        client_id,
        redirect_uri,
        pkce,
        code,
    )
    .await?;
    let credential = GeminiOAuthTokens {
        credential_id: credential_id.to_string(),
        access_token: exchanged_tokens.access_token,
        refresh_token: exchanged_tokens.refresh_token,
        client_id: client_id.to_string(),
        scopes: scopes.to_vec(),
        expires_at: exchanged_tokens.expires_at,
        account_id,
        project_id,
        provenance,
    };

    upsert_provider_oauth_credential(
        codex_home,
        auth_credentials_store_mode,
        credential
            .clone()
            .into_provider_oauth_credential()
            .map_err(io::Error::other)?,
    )?;

    Ok(credential)
}

/// Persists exchanged credentials using the configured local auth store, then
/// best-effort revokes any superseded managed ChatGPT tokens.
pub(crate) async fn persist_tokens_async(
    codex_home: &Path,
    api_key: Option<String>,
    id_token: String,
    access_token: String,
    refresh_token: String,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> io::Result<()> {
    // Reuse existing synchronous logic but run it off the async runtime.
    let codex_home = codex_home.to_path_buf();
    let (previous_auth, auth) = tokio::task::spawn_blocking(move || {
        let previous_auth = match load_auth_dot_json(&codex_home, auth_credentials_store_mode) {
            Ok(auth) => auth,
            Err(err) => {
                warn!("failed to load previous auth before saving new login: {err}");
                None
            }
        };
        let mut tokens = TokenData {
            id_token: parse_chatgpt_jwt_claims(&id_token).map_err(io::Error::other)?,
            access_token,
            refresh_token,
            account_id: None,
        };
        if let Some(acc) = jwt_auth_claims(&id_token)
            .get("chatgpt_account_id")
            .and_then(|v| v.as_str())
        {
            tokens.account_id = Some(acc.to_string());
        }
        let auth = AuthDotJson {
            auth_mode: Some(AuthMode::Chatgpt),
            openai_api_key: api_key,
            tokens: Some(tokens),
            last_refresh: Some(Utc::now()),
            agent_identity: None,
            provider_oauth_credentials: Vec::new(),
        };
        save_auth(&codex_home, &auth, auth_credentials_store_mode)?;
        Ok::<_, io::Error>((previous_auth, auth))
    })
    .await
    .map_err(|e| io::Error::other(format!("persist task failed: {e}")))??;

    if should_revoke_auth_tokens(previous_auth.as_ref(), &auth)
        && let Err(err) = revoke_auth_tokens(previous_auth.as_ref()).await
    {
        warn!("failed to revoke superseded auth tokens after login: {err}");
    }

    Ok(())
}

fn compose_success_url(
    port: u16,
    issuer: &str,
    id_token: &str,
    access_token: &str,
    codex_streamlined_login: bool,
) -> String {
    let token_claims = jwt_auth_claims(id_token);
    let access_claims = jwt_auth_claims(access_token);

    let org_id = token_claims
        .get("organization_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let project_id = token_claims
        .get("project_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let completed_onboarding = token_claims
        .get("completed_platform_onboarding")
        .and_then(JsonValue::as_bool)
        .unwrap_or(false);
    let is_org_owner = token_claims
        .get("is_org_owner")
        .and_then(JsonValue::as_bool)
        .unwrap_or(false);
    let needs_setup = (!completed_onboarding) && is_org_owner;
    let plan_type = access_claims
        .get("chatgpt_plan_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let platform_url = if issuer == DEFAULT_ISSUER {
        "https://platform.openai.com"
    } else {
        "https://platform.api.openai.org"
    };

    let mut params = vec![
        ("id_token", id_token.to_string()),
        ("needs_setup", needs_setup.to_string()),
        ("org_id", org_id.to_string()),
        ("project_id", project_id.to_string()),
        ("plan_type", plan_type.to_string()),
        ("platform_url", platform_url.to_string()),
    ];
    if codex_streamlined_login {
        params.push(("codex_streamlined_login", "true".to_string()));
    }
    let qs = params
        .drain(..)
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(&v)))
        .collect::<Vec<_>>()
        .join("&");
    format!("http://localhost:{port}/success?{qs}")
}

fn jwt_auth_claims(jwt: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut parts = jwt.split('.');
    let (_h, payload_b64, _s) = match (parts.next(), parts.next(), parts.next()) {
        (Some(h), Some(p), Some(s)) if !h.is_empty() && !p.is_empty() && !s.is_empty() => (h, p, s),
        _ => {
            eprintln!("Invalid JWT format while extracting claims");
            return serde_json::Map::new();
        }
    };
    match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload_b64) {
        Ok(bytes) => match serde_json::from_slice::<serde_json::Value>(&bytes) {
            Ok(mut v) => {
                if let Some(obj) = v
                    .get_mut("https://api.openai.com/auth")
                    .and_then(|x| x.as_object_mut())
                {
                    return obj.clone();
                }
                eprintln!("JWT payload missing expected 'https://api.openai.com/auth' object");
            }
            Err(e) => {
                eprintln!("Failed to parse JWT JSON payload: {e}");
            }
        },
        Err(e) => {
            eprintln!("Failed to base64url-decode JWT payload: {e}");
        }
    }
    serde_json::Map::new()
}

/// Validates the ID token against an optional workspace restriction.
pub(crate) fn ensure_workspace_allowed(
    expected: Option<&[String]>,
    id_token: &str,
) -> Result<(), String> {
    let Some(expected) = expected else {
        return Ok(());
    };

    let claims = jwt_auth_claims(id_token);
    let Some(actual) = claims.get("chatgpt_account_id").and_then(JsonValue::as_str) else {
        return Err("Login is restricted to a specific workspace, but the token did not include an chatgpt_account_id claim.".to_string());
    };

    if expected.iter().any(|workspace_id| workspace_id == actual) {
        Ok(())
    } else {
        Err(format!(
            "Login is restricted to workspace id(s) {}.",
            expected.join(", ")
        ))
    }
}

/// Builds a terminal callback response for login failures.
fn login_error_response(
    message: &str,
    kind: io::ErrorKind,
    error_code: Option<&str>,
    error_description: Option<&str>,
) -> HandledRequest {
    let mut headers = Vec::new();
    if let Ok(header) = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]) {
        headers.push(header);
    }
    let body = render_login_error_page(message, error_code, error_description);
    HandledRequest::ResponseAndExit {
        headers,
        body,
        result: Err(io::Error::new(kind, message.to_string())),
    }
}

/// Returns true when the OAuth callback represents a missing Codex entitlement.
fn is_missing_codex_entitlement_error(error_code: &str, error_description: Option<&str>) -> bool {
    error_code == "access_denied"
        && error_description.is_some_and(|description| {
            description
                .to_ascii_lowercase()
                .contains("missing_codex_entitlement")
        })
}

/// Converts OAuth callback errors into a user-facing message.
fn oauth_callback_error_message(error_code: &str, error_description: Option<&str>) -> String {
    if is_missing_codex_entitlement_error(error_code, error_description) {
        return "Codex is not enabled for your workspace. Contact your workspace administrator to request access to Codex.".to_string();
    }

    if let Some(description) = error_description
        && !description.trim().is_empty()
    {
        return format!("Sign-in failed: {description}");
    }

    format!("Sign-in failed: {error_code}")
}

/// Extracts token endpoint error detail for both structured logging and caller-visible errors.
///
/// Parsed JSON fields are safe to log individually. If the response is not JSON, the raw body is
/// preserved only for the returned error path so the CLI/browser can still surface the backend
/// detail, while the structured log path continues to use the explicitly parsed safe fields above.
fn parse_token_endpoint_error(body: &str) -> TokenEndpointErrorDetail {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return TokenEndpointErrorDetail {
            error_code: None,
            error_message: None,
            display_message: "unknown error".to_string(),
        };
    }

    let parsed = serde_json::from_str::<JsonValue>(trimmed).ok();
    if let Some(json) = parsed {
        let error_code = json
            .get("error")
            .and_then(JsonValue::as_str)
            .filter(|error_code| !error_code.trim().is_empty())
            .map(ToString::to_string)
            .or_else(|| {
                json.get("error")
                    .and_then(JsonValue::as_object)
                    .and_then(|error_obj| error_obj.get("code"))
                    .and_then(JsonValue::as_str)
                    .filter(|code| !code.trim().is_empty())
                    .map(ToString::to_string)
            });
        if let Some(description) = json.get("error_description").and_then(JsonValue::as_str)
            && !description.trim().is_empty()
        {
            return TokenEndpointErrorDetail {
                error_code,
                error_message: Some(description.to_string()),
                display_message: description.to_string(),
            };
        }
        if let Some(error_obj) = json.get("error")
            && let Some(message) = error_obj.get("message").and_then(JsonValue::as_str)
            && !message.trim().is_empty()
        {
            return TokenEndpointErrorDetail {
                error_code,
                error_message: Some(message.to_string()),
                display_message: message.to_string(),
            };
        }
        if let Some(error_code) = error_code {
            return TokenEndpointErrorDetail {
                display_message: error_code.clone(),
                error_code: Some(error_code),
                error_message: None,
            };
        }
    }

    // Preserve non-JSON token-endpoint bodies for the returned error so CLI/browser flows still
    // surface the backend detail users and admins need, but keep that text out of structured logs
    // by only logging explicitly parsed fields above and avoiding `%err` logging at the callback
    // layer.
    TokenEndpointErrorDetail {
        error_code: None,
        error_message: None,
        display_message: trimmed.to_string(),
    }
}

/// Renders the branded error page used by callback failures.
fn render_login_error_page(
    message: &str,
    error_code: Option<&str>,
    error_description: Option<&str>,
) -> Vec<u8> {
    let code = error_code.unwrap_or("unknown_error");
    let (title, display_message, display_description, help_text) =
        if is_missing_codex_entitlement_error(code, error_description) {
            (
                "You do not have access to Codex".to_string(),
                "This account is not currently authorized to use Codex in this workspace."
                    .to_string(),
                "Contact your workspace administrator to request access to Codex.".to_string(),
                "Contact your workspace administrator to get access to Codex, then return to Codex and try again."
                    .to_string(),
            )
        } else {
            (
                "Sign-in could not be completed".to_string(),
                message.to_string(),
                error_description.unwrap_or(message).to_string(),
                "Return to Codex to retry, switch accounts, or contact your workspace admin if access is restricted."
                    .to_string(),
            )
        };
    LOGIN_ERROR_PAGE_TEMPLATE
        .render([
            ("error_title", html_escape(&title)),
            ("error_message", html_escape(&display_message)),
            ("error_code", html_escape(code)),
            ("error_description", html_escape(&display_description)),
            ("error_help", html_escape(&help_text)),
        ])
        .unwrap_or_else(|err| panic!("login error page template must render: {err}"))
        .into_bytes()
}

/// Escapes error strings before inserting them into HTML.
fn html_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// Exchanges an authenticated ID token for an API-key style access token.
pub(crate) async fn obtain_api_key(
    issuer: &str,
    client_id: &str,
    id_token: &str,
) -> io::Result<String> {
    // Token exchange for an API key access token
    #[derive(serde::Deserialize)]
    struct ExchangeResp {
        access_token: String,
    }
    let client = build_reqwest_client_with_custom_ca(reqwest::Client::builder())?;
    let token_endpoint = format!("{}/oauth/token", issuer.trim_end_matches('/'));
    let resp = client
        .post(token_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type={}&client_id={}&requested_token={}&subject_token={}&subject_token_type={}",
            urlencoding::encode("urn:ietf:params:oauth:grant-type:token-exchange"),
            urlencoding::encode(client_id),
            urlencoding::encode("openai-api-key"),
            urlencoding::encode(id_token),
            urlencoding::encode("urn:ietf:params:oauth:token-type:id_token")
        ))
        .send()
        .await
        .map_err(io::Error::other)?;
    if !resp.status().is_success() {
        return Err(io::Error::other(format!(
            "api key exchange failed with status {}",
            resp.status()
        )));
    }
    let body: ExchangeResp = resp.json().await.map_err(io::Error::other)?;
    Ok(body.access_token)
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::io;

    use anyhow::Context;
    use base64::Engine;
    use ontocode_app_server_protocol::AuthMode;
    use ontocode_config::types::AuthCredentialsStoreMode;
    use serde_json::Value;
    use serde_json::json;
    use tempfile::tempdir;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    use crate::auth::AuthDotJson;
    use crate::auth::REVOKE_TOKEN_URL_OVERRIDE_ENV_VAR;
    use crate::auth::load_auth_dot_json;
    use crate::auth::save_auth;
    use crate::pkce::PkceCodes;
    use crate::token_data::TokenData;
    use crate::token_data::parse_chatgpt_jwt_claims;
    use core_test_support::skip_if_no_network;
    use pretty_assertions::assert_eq;

    use super::DEFAULT_ISSUER;
    use super::GEMINI_AUTHORIZATION_SCOPE;
    use super::GeminiOAuthProvenance;
    use super::TokenEndpointErrorDetail;
    use super::build_authorize_url_with_extras;
    use super::build_gemini_manual_authorize_url;
    use super::callback_state_matches;
    use super::compose_success_url;
    use super::exchange_gemini_callback_to_provider_oauth_with_token_endpoint;
    use super::exchange_gemini_code_to_provider_oauth_with_token_endpoint;
    use super::generate_state;
    use super::html_escape;
    use super::is_missing_codex_entitlement_error;
    use super::parse_callback_query;
    use super::parse_token_endpoint_error;
    use super::persist_tokens_async;
    use super::redact_sensitive_query_value;
    use super::redact_sensitive_url_parts;
    use super::render_login_error_page;
    use super::sanitize_url_for_logging;
    use crate::gemini_oauth::GEMINI_PROVIDER_ID;
    use crate::gemini_oauth::GEMINI_TOKEN_ENDPOINT;

    #[serial_test::serial(logout_revoke)]
    #[tokio::test]
    async fn persist_tokens_async_revokes_previous_auth_without_failing_login() -> anyhow::Result<()>
    {
        skip_if_no_network!(Ok(()));

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/revoke"))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "error": {
                    "message": "revoke failed"
                }
            })))
            .expect(1)
            .mount(&server)
            .await;
        let _env_guard = EnvGuard::set(
            REVOKE_TOKEN_URL_OVERRIDE_ENV_VAR,
            format!("{}/oauth/revoke", server.uri()),
        );

        let codex_home = tempdir()?;
        save_auth(
            codex_home.path(),
            &chatgpt_auth("old-access", "old-refresh", "old-account"),
            AuthCredentialsStoreMode::File,
        )?;

        persist_tokens_async(
            codex_home.path(),
            /*api_key*/ None,
            jwt_for_account("new-account"),
            "new-access".to_string(),
            "new-refresh".to_string(),
            AuthCredentialsStoreMode::File,
        )
        .await?;

        let auth = load_auth_dot_json(codex_home.path(), AuthCredentialsStoreMode::File)?
            .context("auth.json should exist after login")?;
        assert_eq!(
            auth.tokens.context("new tokens should be persisted")?,
            TokenData {
                id_token: parse_chatgpt_jwt_claims(&jwt_for_account("new-account"))
                    .expect("new JWT should parse"),
                access_token: "new-access".to_string(),
                refresh_token: "new-refresh".to_string(),
                account_id: Some("new-account".to_string()),
            }
        );

        let requests = server
            .received_requests()
            .await
            .context("failed to fetch revoke requests")?;
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0]
                .body_json::<Value>()
                .context("revoke request should be JSON")?,
            json!({
                "token": "old-refresh",
                "token_type_hint": "refresh_token",
                "client_id": crate::auth::CLIENT_ID,
            })
        );
        server.verify().await;
        Ok(())
    }

    #[serial_test::serial(logout_revoke)]
    #[tokio::test]
    async fn persist_tokens_async_does_not_revoke_reused_refresh_token() -> anyhow::Result<()> {
        skip_if_no_network!(Ok(()));

        let server = MockServer::start().await;
        let _env_guard = EnvGuard::set(
            REVOKE_TOKEN_URL_OVERRIDE_ENV_VAR,
            format!("{}/oauth/revoke", server.uri()),
        );

        let codex_home = tempdir()?;
        save_auth(
            codex_home.path(),
            &chatgpt_auth("old-access", "shared-refresh", "old-account"),
            AuthCredentialsStoreMode::File,
        )?;

        persist_tokens_async(
            codex_home.path(),
            /*api_key*/ None,
            jwt_for_account("new-account"),
            "new-access".to_string(),
            "shared-refresh".to_string(),
            AuthCredentialsStoreMode::File,
        )
        .await?;

        let requests = server
            .received_requests()
            .await
            .context("failed to fetch revoke requests")?;
        assert_eq!(requests.len(), 0);
        Ok(())
    }

    #[test]
    fn gemini_parse_callback_query_extracts_expected_values() {
        let query = parse_callback_query(
            "/auth/callback?code=auth-code&state=state-123&error=access_denied&error_description=user%20cancelled",
        )
        .expect("callback query should parse");

        assert_eq!(query.path, "/auth/callback");
        assert_eq!(query.code, Some("auth-code".to_string()));
        assert_eq!(query.state, Some("state-123".to_string()));
        assert_eq!(query.error, Some("access_denied".to_string()));
        assert_eq!(query.error_description, Some("user cancelled".to_string()));
    }

    #[test]
    fn gemini_callback_state_matches_expected_value() {
        let query = parse_callback_query("/auth/callback?state=expected-state")
            .expect("callback query should parse");

        assert!(callback_state_matches(&query, "expected-state"));
        assert!(!callback_state_matches(&query, "wrong-state"));
    }

    #[tokio::test]
    async fn gemini_exchange_callback_rejects_state_mismatch() {
        let tmp = tempdir().expect("tempdir should create");
        let pkce = PkceCodes {
            code_verifier: "verifier".to_string(),
            code_challenge: "challenge".to_string(),
        };

        let err = exchange_gemini_callback_to_provider_oauth_with_token_endpoint(
            tmp.path(),
            AuthCredentialsStoreMode::File,
            "/auth/callback?code=auth-code&state=wrong",
            "expected-state",
            &pkce,
            "client-id",
            "http://127.0.0.1:1455/auth/callback",
            "credential-id",
            &["scope".to_string()],
            None,
            None,
            GEMINI_TOKEN_ENDPOINT,
        )
        .await
        .expect_err("state mismatch should fail");

        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(err.to_string(), "State mismatch");
    }

    #[tokio::test]
    async fn gemini_exchange_callback_rejects_callback_error() {
        let tmp = tempdir().expect("tempdir should create");
        let pkce = PkceCodes {
            code_verifier: "verifier".to_string(),
            code_challenge: "challenge".to_string(),
        };

        let err = exchange_gemini_callback_to_provider_oauth_with_token_endpoint(
            tmp.path(),
            AuthCredentialsStoreMode::File,
            "/auth/callback?error=access_denied&error_description=user%20denied&state=expected-state",
            "expected-state",
            &pkce,
            "client-id",
            "http://127.0.0.1:1455/auth/callback",
            "credential-id",
            &["scope".to_string()],
            None,
            None,
            GEMINI_TOKEN_ENDPOINT,
        )
        .await
        .expect_err("callback error should fail");

        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(err.to_string(), "Sign-in failed: user denied");
    }

    #[tokio::test]
    async fn exchange_gemini_callback_to_provider_oauth_with_local_token_endpoint_persists_credential()
    -> anyhow::Result<()> {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id_token": "id-token",
                "access_token": "access-token",
                "refresh_token": "refresh-token",
                "expires_in": 60u64,
            })))
            .expect(1)
            .mount(&server)
            .await;

        let tmp = tempdir()?;
        let pkce = PkceCodes {
            code_verifier: "verifier".to_string(),
            code_challenge: "challenge".to_string(),
        };
        let scopes = vec!["https://www.googleapis.com/auth/cloud-platform".to_string()];

        let tokens = exchange_gemini_callback_to_provider_oauth_with_token_endpoint(
            tmp.path(),
            AuthCredentialsStoreMode::File,
            "/auth/callback?code=auth-code&state=expected-state",
            "expected-state",
            &pkce,
            "client-id",
            "http://127.0.0.1:1455/auth/callback",
            "credential-id",
            &scopes,
            Some("acct@example.com".to_string()),
            Some("project-id".to_string()),
            &format!("{}/token", server.uri()),
        )
        .await?;

        assert_eq!(tokens.credential_id, "credential-id");
        assert_eq!(tokens.client_id, "client-id");
        assert_eq!(tokens.access_token, "access-token");
        assert_eq!(tokens.refresh_token, "refresh-token");
        assert_eq!(tokens.scopes, scopes);
        assert_eq!(tokens.account_id, Some("acct@example.com".to_string()));
        assert_eq!(tokens.project_id, Some("project-id".to_string()));
        assert_eq!(tokens.provenance, GeminiOAuthProvenance::Browser);
        assert!(tokens.expires_at.is_some());

        let auth = load_auth_dot_json(tmp.path(), AuthCredentialsStoreMode::File)?
            .context("auth json should be persisted")?;
        assert_eq!(auth.provider_oauth_credentials.len(), 1);
        assert_eq!(
            auth.provider_oauth_credentials[0].provider_id(),
            GEMINI_PROVIDER_ID.to_string(),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].client_id,
            Some("client-id".to_string()),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].refresh_token,
            Some("refresh-token".to_string()),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].account_id,
            Some("acct@example.com".to_string()),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].endpoint,
            Some("project-id".to_string()),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].scopes,
            vec!["https://www.googleapis.com/auth/cloud-platform".to_string()],
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].provenance,
            Some(GeminiOAuthProvenance::Browser.as_str().to_string()),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].token_endpoint,
            Some(GEMINI_TOKEN_ENDPOINT.to_string()),
        );
        assert!(!format!("{auth:?}").contains("access-token"));
        assert!(!format!("{auth:?}").contains("refresh-token"));

        server.verify().await;
        Ok(())
    }

    #[tokio::test]
    async fn exchange_gemini_code_to_provider_oauth_with_local_token_endpoint_persists_credential()
    -> anyhow::Result<()> {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id_token": "id-token",
                "access_token": "access-token",
                "refresh_token": "refresh-token",
                "expires_in": 60u64,
            })))
            .expect(1)
            .mount(&server)
            .await;

        let tmp = tempdir()?;
        let pkce = PkceCodes {
            code_verifier: "verifier".to_string(),
            code_challenge: "challenge".to_string(),
        };
        let scopes = vec!["https://www.googleapis.com/auth/cloud-platform".to_string()];

        let tokens = exchange_gemini_code_to_provider_oauth_with_token_endpoint(
            tmp.path(),
            AuthCredentialsStoreMode::File,
            "auth-code",
            "client-id",
            "https://codeassist.google.com/authcode",
            "credential-id",
            &scopes,
            Some("acct@example.com".to_string()),
            Some("project-id".to_string()),
            &format!("{}/token", server.uri()),
            &pkce,
        )
        .await?;

        assert_eq!(tokens.credential_id, "credential-id");
        assert_eq!(tokens.client_id, "client-id");
        assert_eq!(tokens.access_token, "access-token");
        assert_eq!(tokens.refresh_token, "refresh-token");
        assert_eq!(tokens.scopes, scopes);
        assert_eq!(tokens.account_id, Some("acct@example.com".to_string()));
        assert_eq!(tokens.project_id, Some("project-id".to_string()));
        assert_eq!(tokens.provenance, GeminiOAuthProvenance::UserCode);
        assert!(tokens.expires_at.is_some());

        let auth = load_auth_dot_json(tmp.path(), AuthCredentialsStoreMode::File)?
            .context("auth json should be persisted")?;
        assert_eq!(auth.provider_oauth_credentials.len(), 1);
        assert_eq!(
            auth.provider_oauth_credentials[0].provider_id(),
            GEMINI_PROVIDER_ID.to_string(),
        );
        assert_eq!(
            auth.provider_oauth_credentials[0].provenance,
            Some(GeminiOAuthProvenance::UserCode.as_str().to_string()),
        );
        assert!(!format!("{auth:?}").contains("access-token"));
        assert!(!format!("{auth:?}").contains("refresh-token"));

        server.verify().await;
        Ok(())
    }

    #[test]
    fn build_gemini_authorize_url_with_extras_supports_provider_oauth_flow_shape() {
        let pkce = PkceCodes {
            code_verifier: "test-verifier".to_string(),
            code_challenge: "test-challenge".to_string(),
        };
        let url = build_authorize_url_with_extras(
            "https://accounts.google.com/o/oauth2/v2/auth",
            "gemini-client-id",
            "http://127.0.0.1:1455/auth/callback",
            &pkce,
            "gemini-state",
            "openid email profile https://www.googleapis.com/auth/cloud-platform",
            /*include_originator*/ false,
            &[
                ("access_type", "offline"),
                ("prompt", "consent"),
                ("include_granted_scopes", "true"),
            ],
        );

        let parsed = url::Url::parse(&url).expect("authorization URL should parse");
        let query: HashMap<String, String> = parsed
            .query_pairs()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();

        assert_eq!(parsed.host_str(), Some("accounts.google.com"));
        assert_eq!(parsed.path(), "/o/oauth2/v2/auth");
        assert_eq!(query.get("response_type"), Some(&"code".to_string()));
        assert_eq!(
            query.get("client_id"),
            Some(&"gemini-client-id".to_string())
        );
        assert_eq!(
            query.get("redirect_uri"),
            Some(&"http://127.0.0.1:1455/auth/callback".to_string())
        );
        assert_eq!(
            query.get("scope"),
            Some(
                &"openid email profile https://www.googleapis.com/auth/cloud-platform".to_string()
            )
        );
        assert_eq!(
            query.get("code_challenge"),
            Some(&"test-challenge".to_string())
        );
        assert_eq!(
            query.get("code_challenge_method"),
            Some(&"S256".to_string())
        );
        assert_eq!(query.get("access_type"), Some(&"offline".to_string()));
        assert_eq!(query.get("prompt"), Some(&"consent".to_string()));
        assert_eq!(
            query.get("include_granted_scopes"),
            Some(&"true".to_string())
        );
        assert_eq!(query.get("state"), Some(&"gemini-state".to_string()));
        assert!(!query.contains_key("originator"));
    }

    #[test]
    fn build_gemini_manual_authorize_url_uses_code_assist_redirect() {
        let pkce = PkceCodes {
            code_verifier: "test-verifier".to_string(),
            code_challenge: "test-challenge".to_string(),
        };
        let state = generate_state();
        let url = build_gemini_manual_authorize_url("gemini-client-id", &pkce, &state);
        let parsed = url::Url::parse(&url).expect("authorization URL should parse");
        let query: HashMap<String, String> = parsed
            .query_pairs()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();

        assert_eq!(parsed.host_str(), Some("accounts.google.com"));
        assert_eq!(query.get("response_type"), Some(&"code".to_string()));
        assert_eq!(
            query.get("client_id"),
            Some(&"gemini-client-id".to_string())
        );
        assert_eq!(
            query.get("redirect_uri"),
            Some(&"https://codeassist.google.com/authcode".to_string())
        );
        assert_eq!(query.get("state"), Some(&state));
        assert_eq!(
            query.get("scope"),
            Some(&GEMINI_AUTHORIZATION_SCOPE.to_string())
        );
        assert_eq!(
            query.get("code_challenge"),
            Some(&"test-challenge".to_string())
        );
        assert_eq!(
            query.get("code_challenge_method"),
            Some(&"S256".to_string())
        );
        assert!(!query.contains_key("originator"));
    }

    fn chatgpt_auth(access_token: &str, refresh_token: &str, account_id: &str) -> AuthDotJson {
        AuthDotJson {
            auth_mode: Some(AuthMode::Chatgpt),
            openai_api_key: None,
            tokens: Some(TokenData {
                id_token: parse_chatgpt_jwt_claims(&jwt_for_account(account_id))
                    .expect("test JWT should parse"),
                access_token: access_token.to_string(),
                refresh_token: refresh_token.to_string(),
                account_id: Some(account_id.to_string()),
            }),
            last_refresh: None,
            agent_identity: None,
            provider_oauth_credentials: Vec::new(),
        }
    }

    fn jwt_for_account(account_id: &str) -> String {
        let encode = |bytes: &[u8]| base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
        let header_b64 = encode(br#"{"alg":"none","typ":"JWT"}"#);
        let payload_b64 = encode(
            serde_json::to_string(&json!({
                "https://api.openai.com/auth": {
                    "chatgpt_account_id": account_id,
                }
            }))
            .expect("payload should serialize")
            .as_bytes(),
        );
        let signature_b64 = encode(b"sig");
        format!("{header_b64}.{payload_b64}.{signature_b64}")
    }

    struct EnvGuard {
        key: &'static str,
        original: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: String) -> Self {
            let original = std::env::var_os(key);
            // SAFETY: this test executes serially with other revoke tests.
            unsafe {
                std::env::set_var(key, &value);
            }
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            // SAFETY: the guard restores the original environment before other revoke tests run.
            unsafe {
                match &self.original {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    fn parse_token_endpoint_error_prefers_error_description() {
        let detail = parse_token_endpoint_error(
            r#"{"error":"invalid_grant","error_description":"refresh token expired"}"#,
        );

        assert_eq!(
            detail,
            TokenEndpointErrorDetail {
                error_code: Some("invalid_grant".to_string()),
                error_message: Some("refresh token expired".to_string()),
                display_message: "refresh token expired".to_string(),
            }
        );
    }

    #[test]
    fn parse_token_endpoint_error_reads_nested_error_message_and_code() {
        let detail = parse_token_endpoint_error(
            r#"{"error":{"code":"proxy_auth_required","message":"proxy authentication required"}}"#,
        );

        assert_eq!(
            detail,
            TokenEndpointErrorDetail {
                error_code: Some("proxy_auth_required".to_string()),
                error_message: Some("proxy authentication required".to_string()),
                display_message: "proxy authentication required".to_string(),
            }
        );
    }

    #[test]
    fn parse_token_endpoint_error_falls_back_to_error_code() {
        let detail = parse_token_endpoint_error(r#"{"error":"temporarily_unavailable"}"#);

        assert_eq!(
            detail,
            TokenEndpointErrorDetail {
                error_code: Some("temporarily_unavailable".to_string()),
                error_message: None,
                display_message: "temporarily_unavailable".to_string(),
            }
        );
    }

    #[test]
    fn parse_token_endpoint_error_preserves_plain_text_for_display() {
        let detail = parse_token_endpoint_error("service unavailable");

        assert_eq!(
            detail,
            TokenEndpointErrorDetail {
                error_code: None,
                error_message: None,
                display_message: "service unavailable".to_string(),
            }
        );
    }

    #[test]
    fn redact_sensitive_query_value_only_scrubs_known_keys() {
        for key in [
            "access_token",
            "Authorization",
            "client_secret",
            "code",
            "Cookie",
            "id_token",
            "refresh_token",
            "Set-Cookie",
        ] {
            assert_eq!(
                redact_sensitive_query_value(key, "secret-value"),
                "<redacted>".to_string(),
                "{key} should be redacted"
            );
        }

        assert_eq!(
            redact_sensitive_query_value("redirect_uri", "http://localhost:1455/auth/callback"),
            "http://localhost:1455/auth/callback".to_string()
        );
    }

    #[test]
    fn redact_sensitive_url_parts_preserves_safe_url_shape() {
        let mut url = url::Url::parse(
            "https://user:pass@auth.openai.com/oauth/token?code=abc123&client_secret=client-secret&access_token=access-secret&refresh_token=refresh-secret&id_token=id-secret&Authorization=Bearer%20secret&Cookie=session%3Dsecret&redirect_uri=http%3A%2F%2Flocalhost%3A1455%2Fauth%2Fcallback#frag",
        )
        .expect("valid url");

        redact_sensitive_url_parts(&mut url);

        assert_eq!(
            url.as_str(),
            "https://auth.openai.com/oauth/token?code=%3Credacted%3E&client_secret=%3Credacted%3E&access_token=%3Credacted%3E&refresh_token=%3Credacted%3E&id_token=%3Credacted%3E&Authorization=%3Credacted%3E&Cookie=%3Credacted%3E&redirect_uri=http%3A%2F%2Flocalhost%3A1455%2Fauth%2Fcallback"
        );
    }

    #[test]
    fn sanitize_url_for_logging_redacts_sensitive_issuer_parts() {
        let redacted =
            sanitize_url_for_logging("https://user:pass@example.com/base?token=abc123&env=prod");

        assert_eq!(
            redacted,
            "https://example.com/base?token=%3Credacted%3E&env=prod".to_string()
        );
    }

    #[test]
    fn compose_success_url_omits_streamlined_success_by_default() {
        let url = url::Url::parse(&compose_success_url(
            /*port*/ 1455,
            DEFAULT_ISSUER,
            "e30.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL2F1dGgiOnt9fQ.sig",
            "e30.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL2F1dGgiOnt9fQ.sig",
            /*codex_streamlined_login*/ false,
        ))
        .expect("success url should parse");

        assert_eq!(
            url.query_pairs()
                .find(|(key, _)| key == "codex_streamlined_login"),
            None
        );
    }

    #[test]
    fn compose_success_url_includes_streamlined_success_when_requested() {
        let url = url::Url::parse(&compose_success_url(
            /*port*/ 1455,
            DEFAULT_ISSUER,
            "e30.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL2F1dGgiOnt9fQ.sig",
            "e30.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL2F1dGgiOnt9fQ.sig",
            /*codex_streamlined_login*/ true,
        ))
        .expect("success url should parse");

        assert_eq!(
            url.query_pairs()
                .find(|(key, _)| key == "codex_streamlined_login")
                .map(|(_, value)| value.into_owned()),
            Some("true".to_string())
        );
    }

    #[test]
    fn render_login_error_page_escapes_dynamic_fields() {
        let body = String::from_utf8(render_login_error_page(
            "<bad>",
            Some("code&value"),
            Some("\"quoted\""),
        ))
        .expect("login error page should be utf-8");

        assert!(body.contains(&html_escape("Sign-in could not be completed")));
        assert!(body.contains("&lt;bad&gt;"));
        assert!(body.contains("code&amp;value"));
        assert!(body.contains("&quot;quoted&quot;"));
    }

    #[test]
    fn render_login_error_page_uses_entitlement_copy() {
        let error_description = Some("missing_codex_entitlement");
        assert!(is_missing_codex_entitlement_error(
            "access_denied",
            error_description
        ));

        let body = String::from_utf8(render_login_error_page(
            "access denied",
            Some("access_denied"),
            error_description,
        ))
        .expect("login error page should be utf-8");

        assert!(body.contains("You do not have access to Codex"));
        assert!(body.contains("Contact your workspace administrator"));
        assert!(!body.contains("missing_codex_entitlement"));
    }
}
