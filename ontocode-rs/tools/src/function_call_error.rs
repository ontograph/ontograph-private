use thiserror::Error;

/// Error returned while executing a model-visible tool invocation.
#[derive(Debug, Error, PartialEq)]
pub enum FunctionCallError {
    #[error("{0}")]
    RespondToModel(String),
    #[error("Fatal error: {0}")]
    Fatal(String),
}

/// Stable machine-readable reason code for a tool execution failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCallErrorReasonCode {
    RespondToModel,
    Fatal,
}

impl FunctionCallErrorReasonCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RespondToModel => "respond_to_model",
            Self::Fatal => "fatal",
        }
    }
}

impl FunctionCallError {
    pub const fn reason_code(&self) -> FunctionCallErrorReasonCode {
        match self {
            Self::RespondToModel(_) => FunctionCallErrorReasonCode::RespondToModel,
            Self::Fatal(_) => FunctionCallErrorReasonCode::Fatal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reason_codes_are_stable() {
        assert_eq!(
            FunctionCallError::RespondToModel("retry".to_string()).reason_code(),
            FunctionCallErrorReasonCode::RespondToModel
        );
        assert_eq!(
            FunctionCallError::Fatal("boom".to_string()).reason_code(),
            FunctionCallErrorReasonCode::Fatal
        );
        assert_eq!(
            FunctionCallErrorReasonCode::RespondToModel.as_str(),
            "respond_to_model"
        );
        assert_eq!(FunctionCallErrorReasonCode::Fatal.as_str(), "fatal");
    }
}
