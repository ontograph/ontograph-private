use pretty_assertions::assert_eq;

use ontocode_model_provider_info::ModelProviderInfo;

use super::ProviderAuthKind;
use super::ProviderRoute;
use super::derive_provider_route;
use super::provider_auth_kind_for_provider_info;

#[test]
fn provider_route_keeps_stable_provider_id_separate_from_display_name() {
    let provider = ModelProviderInfo::create_gemini_cli_provider();
    let route = derive_provider_route("gemini-cli", "gemini-2.5-pro", &provider);

    assert_eq!(
        route,
        ProviderRoute::new(
            "gemini-cli",
            "gemini-2.5-pro",
            ProviderAuthKind::Unauthenticated
        )
    );
    assert_eq!(provider.name, "Gemini CLI");
}

#[test]
fn provider_auth_kind_defaults_to_openai_oauth_for_openai_provider() {
    let provider = ModelProviderInfo::create_openai_provider(/*base_url*/ None);

    assert_eq!(
        provider_auth_kind_for_provider_info(&provider),
        ProviderAuthKind::OAuth
    );
}

#[test]
fn provider_auth_kind_prefers_api_key_and_bearer_routes_over_oauth() {
    let api_key_provider = ModelProviderInfo {
        env_key: Some("GEMINI_API_KEY".to_string()),
        ..ModelProviderInfo::create_gemini_cli_provider()
    };
    let bearer_provider = ModelProviderInfo {
        experimental_bearer_token: Some("configured-bearer-token".to_string()),
        ..ModelProviderInfo::create_gemini_cli_provider()
    };

    assert_eq!(
        provider_auth_kind_for_provider_info(&api_key_provider),
        ProviderAuthKind::ApiKey
    );
    assert_eq!(
        provider_auth_kind_for_provider_info(&bearer_provider),
        ProviderAuthKind::ApiKey
    );
}
