use super::ContextualUserFragment;
use ontocode_utils_string::truncate_middle_with_token_budget;

const MAX_HOOK_ADDITIONAL_CONTEXT_TOKENS: usize = 1_000;
const HOOK_ADDITIONAL_CONTEXT_START_MARKER: &str = "<hook_additional_context";
const HOOK_ADDITIONAL_CONTEXT_END_MARKER: &str = "</hook_additional_context>";

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HookAdditionalContext {
    text: String,
}

impl HookAdditionalContext {
    pub(crate) fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl ContextualUserFragment for HookAdditionalContext {
    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            HOOK_ADDITIONAL_CONTEXT_START_MARKER,
            HOOK_ADDITIONAL_CONTEXT_END_MARKER,
        )
    }

    fn body(&self) -> String {
        let text =
            truncate_middle_with_token_budget(&self.text, MAX_HOOK_ADDITIONAL_CONTEXT_TOKENS).0;
        format!(" source=\"hooks\">\n{text}\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_with_source_label() {
        let fragment = HookAdditionalContext::new("remember hook note");

        assert_eq!(
            fragment.render(),
            "<hook_additional_context source=\"hooks\">\nremember hook note\n</hook_additional_context>"
        );
    }

    #[test]
    fn caps_large_hook_context() {
        let raw = "hook context ".repeat(2_000);
        let rendered = HookAdditionalContext::new(raw).render();

        assert!(rendered.starts_with("<hook_additional_context source=\"hooks\">\n"));
        assert!(rendered.ends_with("\n</hook_additional_context>"));
        assert!(rendered.contains("tokens truncated"));
    }
}
