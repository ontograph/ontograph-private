use codex_context_fragments::ContextualUserFragment;

pub struct DiagnosticFragment {
    pub content: String,
    pub max_tokens: usize,
}

impl DiagnosticFragment {
    pub fn new(content: String) -> Self {
        Self {
            content,
            max_tokens: 1000,
        }
    }
}

impl ContextualUserFragment for DiagnosticFragment {
    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        ("<diagnostic>", "</diagnostic>")
    }

    fn body(&self) -> String {
        // Enforce hard token cap by character approximation for now
        // A better approach would be to use the real tokenizer, but that's expensive here.
        let max_chars = self.max_tokens * 4;
        if self.content.len() > max_chars {
            format!(
                "{}... [truncated diagnostic content exceeded {} tokens]",
                &self.content[..max_chars],
                self.max_tokens
            )
        } else {
            self.content.clone()
        }
    }

    fn type_markers() -> (&'static str, &'static str)
    where
        Self: Sized,
    {
        ("<diagnostic>", "</diagnostic>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_fragment_renders_markers() {
        let fragment = DiagnosticFragment::new("test content".to_string());
        assert_eq!(fragment.render(), "<diagnostic>test content</diagnostic>");
    }

    #[test]
    fn test_diagnostic_fragment_truncates_large_content() {
        let large_content = "a".repeat(5000);
        let fragment = DiagnosticFragment {
            content: large_content.clone(),
            max_tokens: 10,
        };
        // max_tokens 10 -> 40 chars
        let rendered = fragment.body();
        assert!(rendered.len() < 100);
        assert!(rendered.contains("truncated"));
        assert_eq!(&rendered[..40], &large_content[..40]);
    }
}
