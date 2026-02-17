//! # fracta-ai — AI Primitives
//!
//! LLM call interface, embedding generation, prompt-template management.
//!
//! Provides the Engine-layer building blocks for AI: an async trait for
//! LLM providers, embedding storage/retrieval, and a prompt-template
//! system. AI outputs are always materialized as open-format files.
//!
//! ## Architecture
//!
//! ```text
//! Platform Shell (Swift/Kotlin)
//!        │
//!        ▼
//! ┌──────────────┐
//! │  fracta-ffi   │  ← FfiAiEngine wraps a concrete provider
//! └──────┬───────┘
//!        │
//!        ▼
//! ┌──────────────┐
//! │  fracta-ai    │  ← This crate: traits + providers
//! └──────────────┘
//! ```
//!
//! Providers are pluggable: cloud (OpenAI, Anthropic), local models, or
//! platform-native (Apple Intelligence via Swift callback). The crate
//! ships with an `EchoProvider` for testing and development.

use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════
// Error Types
// ═══════════════════════════════════════════════════════════════════════════

/// AI subsystem errors.
#[derive(Debug, thiserror::Error)]
pub enum AiError {
    /// No provider configured.
    #[error("AI provider not configured")]
    ProviderNotConfigured,

    /// Provider request failed (network, API error, etc.).
    #[error("Request failed: {0}")]
    RequestFailed(String),

    /// Response could not be parsed.
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Provider rate-limited the request.
    #[error("Rate limited — try again later")]
    RateLimited,

    /// Prompt exceeded the model's token limit.
    #[error("Token limit exceeded: requested {requested}, limit {limit}")]
    TokenLimitExceeded { limit: usize, requested: usize },
}

// ═══════════════════════════════════════════════════════════════════════════
// Chat Types
// ═══════════════════════════════════════════════════════════════════════════

/// Role of a participant in a chat conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatRole {
    /// System prompt — sets behavior and context.
    System,
    /// User message.
    User,
    /// Assistant (AI) response.
    Assistant,
}

impl fmt::Display for ChatRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatRole::System => write!(f, "system"),
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
        }
    }
}

/// A single message in a chat conversation.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: content.into(),
        }
    }
}

/// Request to complete a chat conversation.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    /// The conversation so far.
    pub messages: Vec<ChatMessage>,
    /// Maximum tokens in the response (None = provider default).
    pub max_tokens: Option<u32>,
    /// Sampling temperature (0.0 = deterministic, 1.0+ = creative).
    pub temperature: Option<f32>,
}

/// Response from a completion request.
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    /// The generated text.
    pub content: String,
    /// Approximate tokens consumed (prompt + completion).
    pub tokens_used: u32,
    /// Model identifier that generated this response.
    pub model: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Provider Trait
// ═══════════════════════════════════════════════════════════════════════════

/// An AI provider that can generate chat completions.
///
/// Providers must be thread-safe (`Send + Sync`) because the FFI layer
/// wraps them in objects shared across threads.
pub trait AiProvider: Send + Sync {
    /// Generate a completion for the given conversation.
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, AiError>;

    /// The model name this provider uses.
    fn model_name(&self) -> &str;
}

// ═══════════════════════════════════════════════════════════════════════════
// Echo Provider (testing / development)
// ═══════════════════════════════════════════════════════════════════════════

/// A testing provider that echoes back the user's message.
///
/// Used for development and integration testing without requiring
/// API keys or network access.
pub struct EchoProvider;

impl AiProvider for EchoProvider {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, AiError> {
        let last_user = request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, ChatRole::User))
            .map(|m| m.content.clone())
            .unwrap_or_default();

        // Simulate a thoughtful response based on the input
        let content = if last_user.is_empty() {
            "I didn't receive a message. How can I help you?".to_string()
        } else {
            format!(
                "I received your message: \"{}\"\n\n\
                 This is the Fracta AI echo provider (development mode). \
                 Connect a real provider (OpenAI, Anthropic, or local model) \
                 in Settings to get actual AI responses.",
                last_user
            )
        };

        // Rough token estimate: ~4 chars per token
        let tokens_used = (last_user.len() + content.len()) as u32 / 4;

        Ok(CompletionResponse {
            content,
            tokens_used,
            model: "echo-v1".to_string(),
        })
    }

    fn model_name(&self) -> &str {
        "echo-v1"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_provider_basic() {
        let provider = EchoProvider;
        let request = CompletionRequest {
            messages: vec![
                ChatMessage::system("You are a helpful assistant."),
                ChatMessage::user("Hello, world!"),
            ],
            max_tokens: None,
            temperature: None,
        };

        let response = provider.complete(&request).unwrap();
        assert!(response.content.contains("Hello, world!"));
        assert_eq!(response.model, "echo-v1");
        assert!(response.tokens_used > 0);
    }

    #[test]
    fn test_echo_provider_empty_input() {
        let provider = EchoProvider;
        let request = CompletionRequest {
            messages: vec![ChatMessage::system("System prompt only.")],
            max_tokens: None,
            temperature: None,
        };

        let response = provider.complete(&request).unwrap();
        assert!(response.content.contains("didn't receive"));
    }

    #[test]
    fn test_echo_provider_multi_turn() {
        let provider = EchoProvider;
        let request = CompletionRequest {
            messages: vec![
                ChatMessage::system("Context"),
                ChatMessage::user("First message"),
                ChatMessage::assistant("First reply"),
                ChatMessage::user("Second message"),
            ],
            max_tokens: Some(100),
            temperature: Some(0.7),
        };

        let response = provider.complete(&request).unwrap();
        // Should echo the LAST user message
        assert!(response.content.contains("Second message"));
        assert!(!response.content.contains("First message"));
    }

    #[test]
    fn test_chat_message_constructors() {
        let sys = ChatMessage::system("Hello");
        assert_eq!(sys.role, ChatRole::System);
        assert_eq!(sys.content, "Hello");

        let user = ChatMessage::user("World");
        assert_eq!(user.role, ChatRole::User);

        let asst = ChatMessage::assistant("!");
        assert_eq!(asst.role, ChatRole::Assistant);
    }

    #[test]
    fn test_model_name() {
        let provider = EchoProvider;
        assert_eq!(provider.model_name(), "echo-v1");
    }
}
