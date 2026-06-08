use codex_api::is_azure_responses_provider;
use codex_model_provider_info::ModelProviderInfo;

use crate::provider::ProviderCapabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderEngine {
    OpenAiResponses,
    AmazonBedrockResponses,
    AnthropicMessages,
    GeminiGenerateContent,
    GitHubCopilot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NativeProviderDescriptor {
    engine: ProviderEngine,
    accepted_names: &'static [&'static str],
    base_url_markers: &'static [&'static str],
}

impl NativeProviderDescriptor {
    fn matches(self, provider_info: &ModelProviderInfo) -> bool {
        self.accepted_names
            .iter()
            .any(|name| provider_info.name.eq_ignore_ascii_case(name))
            || provider_info.base_url.as_deref().is_some_and(|base_url| {
                self.base_url_markers
                    .iter()
                    .any(|marker| base_url.contains(marker))
            })
    }

    fn provider_descriptor(self) -> ProviderDescriptor {
        ProviderDescriptor {
            engine: self.engine,
            capabilities: ProviderCapabilities {
                image_generation: false,
                web_search: false,
                requires_openai_auth: false,
                supports_models_route_probe: false,
                ..ProviderCapabilities::default()
            },
        }
    }
}

const NATIVE_PROVIDER_DESCRIPTORS: &[NativeProviderDescriptor] = &[
    NativeProviderDescriptor {
        engine: ProviderEngine::AnthropicMessages,
        accepted_names: &["claude", "anthropic"],
        base_url_markers: &["api.anthropic.com"],
    },
    NativeProviderDescriptor {
        engine: ProviderEngine::GeminiGenerateContent,
        accepted_names: &["gemini"],
        base_url_markers: &["generativelanguage.googleapis.com"],
    },
    NativeProviderDescriptor {
        engine: ProviderEngine::GitHubCopilot,
        accepted_names: &["github-copilot", "github copilot"],
        base_url_markers: &["api.githubcopilot.com"],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProviderDescriptor {
    engine: ProviderEngine,
    capabilities: ProviderCapabilities,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderDescriptorSource {
    OpenAiCompatible,
    AmazonBedrock,
    NativeProviderDescriptor,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderDiagnosticSnapshot {
    pub(crate) name: String,
    pub(crate) source: ProviderDescriptorSource,
    pub(crate) engine: ProviderEngine,
    pub(crate) capabilities: ProviderCapabilities,
}

impl ProviderDescriptor {
    pub(crate) fn for_provider(provider_info: &ModelProviderInfo) -> Self {
        if provider_info.is_amazon_bedrock() {
            Self::amazon_bedrock()
        } else if let Some(native_provider) = native_provider_descriptor(provider_info) {
            native_provider.provider_descriptor()
        } else {
            Self::openai_compatible(provider_info)
        }
    }

    pub(crate) fn engine(self) -> ProviderEngine {
        self.engine
    }

    pub(crate) fn capabilities(self) -> ProviderCapabilities {
        self.capabilities
    }

    #[cfg(test)]
    pub(crate) fn diagnostic_snapshot(
        provider_info: &ModelProviderInfo,
    ) -> ProviderDiagnosticSnapshot {
        let source = if provider_info.is_amazon_bedrock() {
            ProviderDescriptorSource::AmazonBedrock
        } else if native_provider_descriptor(provider_info).is_some() {
            ProviderDescriptorSource::NativeProviderDescriptor
        } else {
            ProviderDescriptorSource::OpenAiCompatible
        };
        let descriptor = Self::for_provider(provider_info);

        ProviderDiagnosticSnapshot {
            name: provider_info.name.clone(),
            source,
            engine: descriptor.engine(),
            capabilities: descriptor.capabilities(),
        }
    }

    fn openai_compatible(provider_info: &ModelProviderInfo) -> Self {
        Self {
            engine: ProviderEngine::OpenAiResponses,
            capabilities: ProviderCapabilities {
                requires_openai_auth: provider_info.requires_openai_auth,
                supports_models_route_probe: !is_azure_responses_provider(
                    &provider_info.name,
                    provider_info.base_url.as_deref(),
                ),
                ..ProviderCapabilities::default()
            },
        }
    }

    fn amazon_bedrock() -> Self {
        Self {
            engine: ProviderEngine::AmazonBedrockResponses,
            capabilities: ProviderCapabilities {
                namespace_tools: true,
                image_generation: false,
                web_search: false,
                requires_openai_auth: false,
                supports_models_route_probe: false,
            },
        }
    }
}

fn native_provider_descriptor(
    provider_info: &ModelProviderInfo,
) -> Option<&'static NativeProviderDescriptor> {
    NATIVE_PROVIDER_DESCRIPTORS
        .iter()
        .find(|descriptor| descriptor.matches(provider_info))
}

#[cfg(test)]
mod tests {
    use codex_model_provider_info::WireApi;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn openai_provider_uses_openai_responses_engine_with_default_capabilities() {
        assert_eq!(
            ProviderDescriptor::for_provider(&ModelProviderInfo::create_openai_provider(
                /*base_url*/ None
            )),
            ProviderDescriptor {
                engine: ProviderEngine::OpenAiResponses,
                capabilities: ProviderCapabilities::default(),
            }
        );
    }

    #[test]
    fn azure_provider_uses_openai_responses_engine_without_models_route_probe() {
        assert_eq!(
            ProviderDescriptor::for_provider(&ModelProviderInfo {
                name: "azure".to_string(),
                base_url: Some("https://example.openai.azure.com/openai/v1".to_string()),
                ..ModelProviderInfo::create_openai_provider(/*base_url*/ None)
            }),
            ProviderDescriptor {
                engine: ProviderEngine::OpenAiResponses,
                capabilities: ProviderCapabilities {
                    supports_models_route_probe: false,
                    ..ProviderCapabilities::default()
                },
            }
        );
    }

    #[test]
    fn custom_openai_compatible_provider_preserves_non_openai_auth_requirement() {
        assert_eq!(
            ProviderDescriptor::for_provider(&ModelProviderInfo {
                name: "custom".to_string(),
                base_url: Some("http://localhost:1234/v1".to_string()),
                wire_api: WireApi::Responses,
                requires_openai_auth: false,
                ..Default::default()
            }),
            ProviderDescriptor {
                engine: ProviderEngine::OpenAiResponses,
                capabilities: ProviderCapabilities {
                    requires_openai_auth: false,
                    ..ProviderCapabilities::default()
                },
            }
        );
    }

    #[test]
    fn amazon_bedrock_provider_uses_bedrock_engine_and_capabilities() {
        assert_eq!(
            ProviderDescriptor::for_provider(&ModelProviderInfo::create_amazon_bedrock_provider(
                /*aws*/ None
            )),
            ProviderDescriptor {
                engine: ProviderEngine::AmazonBedrockResponses,
                capabilities: ProviderCapabilities {
                    namespace_tools: true,
                    image_generation: false,
                    web_search: false,
                    requires_openai_auth: false,
                    supports_models_route_probe: false,
                },
            }
        );
    }

    fn native_provider(engine: ProviderEngine) -> ProviderDescriptor {
        ProviderDescriptor {
            engine,
            capabilities: ProviderCapabilities {
                image_generation: false,
                web_search: false,
                requires_openai_auth: false,
                supports_models_route_probe: false,
                ..ProviderCapabilities::default()
            },
        }
    }

    fn provider_with_identity(name: &str, base_url: Option<&str>) -> ModelProviderInfo {
        ModelProviderInfo {
            name: name.to_string(),
            base_url: base_url.map(str::to_string),
            requires_openai_auth: false,
            ..Default::default()
        }
    }

    #[test]
    fn native_provider_registry_selects_engines_by_id_name_and_base_url() {
        let cases = [
            (
                "claude id",
                provider_with_identity("claude", None),
                ProviderEngine::AnthropicMessages,
            ),
            (
                "anthropic name",
                provider_with_identity("Anthropic", None),
                ProviderEngine::AnthropicMessages,
            ),
            (
                "anthropic base URL",
                provider_with_identity("custom", Some("https://api.anthropic.com/v1")),
                ProviderEngine::AnthropicMessages,
            ),
            (
                "gemini id",
                provider_with_identity("gemini", None),
                ProviderEngine::GeminiGenerateContent,
            ),
            (
                "gemini base URL",
                provider_with_identity(
                    "custom",
                    Some("https://generativelanguage.googleapis.com/v1beta"),
                ),
                ProviderEngine::GeminiGenerateContent,
            ),
            (
                "github copilot id",
                provider_with_identity("github-copilot", None),
                ProviderEngine::GitHubCopilot,
            ),
            (
                "github copilot name",
                provider_with_identity("GitHub Copilot", None),
                ProviderEngine::GitHubCopilot,
            ),
            (
                "github copilot base URL",
                provider_with_identity("custom", Some("https://api.githubcopilot.com")),
                ProviderEngine::GitHubCopilot,
            ),
        ];

        for (case, provider_info, engine) in cases {
            assert_eq!(
                ProviderDescriptor::for_provider(&provider_info),
                native_provider(engine),
                "{case}"
            );
        }
    }

    #[test]
    fn provider_diagnostics_reuses_descriptor_for_native_providers() {
        let cases = [
            (
                provider_with_identity("claude", None),
                ProviderEngine::AnthropicMessages,
            ),
            (
                provider_with_identity(
                    "custom",
                    Some("https://generativelanguage.googleapis.com/v1beta"),
                ),
                ProviderEngine::GeminiGenerateContent,
            ),
            (
                provider_with_identity("GitHub Copilot", None),
                ProviderEngine::GitHubCopilot,
            ),
        ];

        for (provider_info, engine) in cases {
            assert_eq!(
                ProviderDescriptor::diagnostic_snapshot(&provider_info),
                ProviderDiagnosticSnapshot {
                    name: provider_info.name,
                    source: ProviderDescriptorSource::NativeProviderDescriptor,
                    engine,
                    capabilities: native_provider(engine).capabilities(),
                }
            );
        }
    }

    #[test]
    fn provider_diagnostics_reports_descriptor_source_and_capabilities() {
        let openai_provider = ModelProviderInfo::create_openai_provider(/*base_url*/ None);
        assert_eq!(
            ProviderDescriptor::diagnostic_snapshot(&openai_provider),
            ProviderDiagnosticSnapshot {
                name: openai_provider.name,
                source: ProviderDescriptorSource::OpenAiCompatible,
                engine: ProviderEngine::OpenAiResponses,
                capabilities: ProviderCapabilities::default(),
            }
        );

        let bedrock_provider = ModelProviderInfo::create_amazon_bedrock_provider(/*aws*/ None);
        assert_eq!(
            ProviderDescriptor::diagnostic_snapshot(&bedrock_provider),
            ProviderDiagnosticSnapshot {
                name: bedrock_provider.name,
                source: ProviderDescriptorSource::AmazonBedrock,
                engine: ProviderEngine::AmazonBedrockResponses,
                capabilities: ProviderCapabilities {
                    namespace_tools: true,
                    image_generation: false,
                    web_search: false,
                    requires_openai_auth: false,
                    supports_models_route_probe: false,
                },
            }
        );
    }
}
