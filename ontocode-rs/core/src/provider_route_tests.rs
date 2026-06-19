use pretty_assertions::assert_eq;

use ontocode_model_provider_info::ModelProviderInfo;

use super::child_provider_route;
use super::resolved_provider_route;
use crate::config::Config;
use crate::session::tests::make_session_and_context;

#[tokio::test]
async fn resolved_provider_route_defaults_to_openai_fallback() {
    let (_session, turn_context) = make_session_and_context().await;

    assert_eq!(
        resolved_provider_route(
            &turn_context.config,
            Some(turn_context.model_info.slug.as_str()),
        ),
        ontocode_provider_auth::ProviderRoute::new(
            "openai",
            turn_context.model_info.slug.as_str(),
            ontocode_provider_auth::ProviderAuthKind::OAuth,
        )
    );
}

#[tokio::test]
async fn child_provider_route_keeps_stable_provider_id_after_role_override() {
    let (_session, turn_context) = make_session_and_context().await;
    let mut config: Config = (*turn_context.config).clone();
    config.model_provider_id = "gemini-cli".to_string();
    config.model_provider = ModelProviderInfo::create_gemini_cli_provider();
    config.model = Some("gemini-2.5-pro".to_string());

    assert_eq!(
        child_provider_route(&config),
        ontocode_provider_auth::ProviderRoute::new(
            "gemini-cli",
            "gemini-2.5-pro",
            ontocode_provider_auth::ProviderAuthKind::Unauthenticated,
        )
    );
    assert_eq!(config.model_provider.name, "Gemini CLI");
}

#[tokio::test]
async fn child_provider_route_reflects_api_key_provider_and_requested_model() {
    let (_session, turn_context) = make_session_and_context().await;
    let mut config: Config = (*turn_context.config).clone();
    config.model_provider_id = "gemini".to_string();
    config.model_provider = ModelProviderInfo::create_gemini_provider();
    config.model = Some("gemini-2.5-pro".to_string());

    assert_eq!(
        child_provider_route(&config),
        ontocode_provider_auth::ProviderRoute::new(
            "gemini",
            "gemini-2.5-pro",
            ontocode_provider_auth::ProviderAuthKind::ApiKey,
        )
    );
}
