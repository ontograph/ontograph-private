use std::cmp::Reverse;
use std::fmt;

/// Redacted provider-auth error text that can safely be rendered in status UIs.
///
/// Callers may pass in sensitive substrings to scrub before the message is
/// stored, keeping token-like values out of debug and display output.
#[derive(Clone, PartialEq, Eq)]
pub struct ProviderAuthRedactedError {
    message: String,
}

impl ProviderAuthRedactedError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn scrubbed(message: impl Into<String>, sensitive_values: &[&str]) -> Self {
        let mut message = message.into();
        let mut sensitive_values = sensitive_values
            .iter()
            .copied()
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        sensitive_values.sort_by_key(|value| Reverse(value.len()));

        for value in sensitive_values {
            message = message.replace(value, "<redacted>");
        }

        Self { message }
    }
}

impl fmt::Debug for ProviderAuthRedactedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderAuthRedactedError")
            .field("message", &self.message)
            .finish()
    }
}

impl fmt::Display for ProviderAuthRedactedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ProviderAuthRedactedError {}

#[cfg(test)]
#[path = "redacted_tests.rs"]
mod tests;
