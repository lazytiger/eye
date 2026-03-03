use derive_more::with_trait::Deref;
use serde::{
    de::{value::MapAccessDeserializer, Visitor}, Deserialize,
    Serialize,
};
use std::marker::PhantomData;

pub trait TextContent {
    fn get_text(&self) -> Option<String>;
    fn new(text: String) -> Self;
}

impl TextContent for SystemMessageContent {
    fn get_text(&self) -> Option<String> {
        match self {
            SystemMessageContent::Text {
                text,
                cache_control,
            } => {
                if cache_control.is_none() {
                    Some(text.clone())
                } else {
                    None
                }
            }
        }
    }

    fn new(t: String) -> Self {
        SystemMessageContent::Text {
            text: t,
            cache_control: None,
        }
    }
}
impl TextContent for ChatMessageContent {
    fn get_text(&self) -> Option<String> {
        match self {
            ChatMessageContent::Text {
                text,
                cache_control,
            } => {
                if cache_control.is_none() {
                    Some(text.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn new(t: String) -> Self {
        ChatMessageContent::Text {
            text: t,
            cache_control: None,
        }
    }
}

#[derive(Debug, Clone, Deref)]
pub struct Content<T: Clone>(pub T);

impl<T> Serialize for Content<T>
where
    T: TextContent + Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(text) = self.0.get_text() {
            serializer.serialize_str(text.as_str())
        } else {
            self.0.serialize(serializer)
        }
    }
}

pub struct ContentVisitor<T>(PhantomData<T>);

impl<'a, T> Visitor<'a> for ContentVisitor<T>
where
    T: TextContent + Deserialize<'a> + Clone,
{
    type Value = Content<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a struct")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content(T::new(value.to_string())))
    }

    fn visit_map<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let t: T = Deserialize::deserialize(MapAccessDeserializer::new(seq))?;
        Ok(Content(t))
    }
}

impl<'a, T> Deserialize<'a> for Content<T>
where
    T: TextContent + Deserialize<'a> + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_any(ContentVisitor::<T>(PhantomData))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Request {
    /// List of messages for the conversation
    pub messages: Vec<Message>,
    /// Unique user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// A unique identifier for grouping related requests (e.g., a conversation or agent workflow) for observability.
    /// If provided in both the request body and the x-session-id header, the body value takes precedence. Maximum of 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Model to use for completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Frequency penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    ///Maximum tokens in completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Random seed for reproducible results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// List of strings to stop generation on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    ///Sampling temperature (0-2)
    /// Defaults to 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Whether to enable parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Tool choice strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Content<ToolChoice>>,
    /// List of tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Nucleus sampling probability (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Output modalities for the response. Supported values are "text", "image", and "audio".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modalities>>,
}

impl Request {
    pub fn new(model: impl Into<String>) -> Self {
        Request {
            model: Some(model.into()),
            ..Default::default()
        }
    }

    pub fn add_user_message(&mut self, text: impl Into<String>) -> &mut Self {
        self.messages.push(Message::User {
            name: None,
            content: Content(ChatMessageContent::Text {
                text: text.into(),
                cache_control: None,
            }),
        });
        self
    }

    pub fn add_system_message(&mut self, text: impl Into<String>) -> &mut Self {
        self.messages.push(Message::System {
            name: None,
            content: Content(SystemMessageContent::Text {
                text: text.into(),
                cache_control: None,
            }),
        });
        self
    }

    pub fn add_assistant_message(&mut self, text: impl Into<String>) -> &mut Self {
        self.messages.push(Message::Assistant {
            name: None,
            content: Content(ChatMessageContent::Text {
                text: text.into(),
                cache_control: None,
            }),
            tool_calls: None,
            refusal: None,
            reasoning: None,
            images: None,
            audio: None,
        });
        self
    }

    pub fn add_tool_message(
        &mut self,
        id: impl Into<String>,
        content: impl Into<String>,
    ) -> &mut Self {
        self.messages.push(Message::Tool {
            tool_call_id: id.into(),
            content: Content(ChatMessageContent::Text {
                text: content.into(),
                cache_control: None,
            }),
        });
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Modalities {
    Text,
    Audio,
    Image,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolFunction {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function {
        function: ToolFunction,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    Function { function: FunctionChoice },
}

impl TextContent for ToolChoice {
    fn get_text(&self) -> Option<String> {
        let name = match self {
            ToolChoice::None => "none",
            ToolChoice::Auto => "auto",
            ToolChoice::Required => "required",
            ToolChoice::Function { .. } => return None,
        };
        Some(name.to_string())
    }

    fn new(text: String) -> Self {
        match text.as_str() {
            "none" => ToolChoice::None,
            "auto" => ToolChoice::Auto,
            "required" => ToolChoice::Required,
            _ => ToolChoice::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionChoice {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchemaFormat },
    Grammar { grammar: String },
    Python,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchemaFormat {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    System {
        /// Optional name for the system message
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        content: Content<SystemMessageContent>,
    },
    User {
        /// Optional name for the user
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// User message content
        content: Content<ChatMessageContent>,
    },
    Assistant {
        /// Optional name for the assistant
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Assistant message content
        content: Content<ChatMessageContent>,
        /// Tool calls made by the assistant
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
        /// Refusal message if content was refused
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning: Option<String>,
        // TODO reasoning_details
        /// Generated images from image generation models
        #[serde(skip_serializing_if = "Option::is_none")]
        images: Option<Vec<OutputImage>>,
        /// Audio output data or reference
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<OutputAudio>,
    },
    Tool {
        /// ID of the assistant message tool call this message responds to
        tool_call_id: String,
        /// Tool response content
        content: Content<ChatMessageContent>,
    },
}

/// Audio output data or reference
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputAudio {
    /// Audio output identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// Audio expiration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<f64>,
    /// Base64 encoded audio data
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    /// Audio transcript
    #[serde(skip_serializing_if = "Option::is_none")]
    transcript: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    /// URL or base64-encoded data of the generated image
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputImage {
    image_url: ImageUrl,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    /// Tool call identifier
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    /// Function name to call
    pub name: String,
    /// Function arguments as JSON string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}

/// System message content
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemMessageContent {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatMessageContent {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    ImageUrl {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<ImageDetail>,
    },
    InputAudio {
        /// Base64 encoded audio data
        data: String,
        /// Audio format (e.g., wav, mp3, flac, m4a, ogg, aiff, aac, pcm16, pcm24). Supported formats vary by provider.
        format: String,
    },
    InputVideo {
        video_url: VideoUrl,
    },
    VideoUrl {
        video_url: VideoUrl,
    },
    File {
        ///File content as base64 data URL or URL
        #[serde(skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
        /// File ID for previously uploaded files
        #[serde(skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        /// Original filename
        #[serde(skip_serializing_if = "Option::is_none")]
        file_name: Option<String>,
    },
}

/// Video input object
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct VideoUrl {
    /// URL of the video (data: URLs supported)
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Auto,
    Low,
    High,
}

/// Cache control for the content part
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CacheControl {
    Ephemeral {
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<TTL>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TTL {
    #[serde(rename = "5m")]
    FiveMinute,
    #[serde(rename = "1h")]
    OneHour,
}

/// Successful chat completion response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    /// Unique completion identifier
    pub id: String,
    /// List of completion choices
    pub choices: Vec<Choice>,
    /// Unix timestamp of creation
    pub created: u64,
    /// Model used for completion
    pub model: String,
    pub object: String,
    /// System fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Token usage statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetail {
    /// Tokens used for reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
    /// Tokens used for audio output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    /// Accepted prediction tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_prediction_tokens: Option<u32>,
    /// Rejected prediction tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_prediction_tokens: Option<u32>,
}

/// Detailed prompt token usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetail {
    /// Cached prompt tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
    /// Tokens written to cache. Only returned for models with explicit caching and cache write pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u32>,
    /// Audio input tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    /// Video input tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub finish_reason: Option<String>,
    /// Choice index
    pub index: usize,
    /// Assistant message for requests and responses
    pub message: ResponseMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Assistant message content
    pub content: Content<ChatMessageContent>,
    /// Optional name for the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls made by the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Refusal message if content was refused
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Reasoning output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    /// Generated images from image generation models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<OutputImage>>,
    /// Audio output data or reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<OutputAudio>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_control() -> anyhow::Result<()> {
        let cache_control = CacheControl::Ephemeral {
            ttl: Some(TTL::FiveMinute),
        };
        let serialized = serde_json::to_string(&cache_control)?;
        assert_eq!(serialized, r#"{"type":"ephemeral","ttl":"5m"}"#);
        let cache_control = CacheControl::Ephemeral {
            ttl: Some(TTL::OneHour),
        };
        let serialized = serde_json::to_string(&cache_control)?;
        assert_eq!(serialized, r#"{"type":"ephemeral","ttl":"1h"}"#);

        let value = serde_json::json!({
            "type": "ephemeral",
            "ttl": "5m"
        });
        let cache_control: CacheControl = serde_json::from_value(value)?;
        println!("{:?}", serde_json::to_string(&cache_control)?);
        Ok(())
    }

    #[test]
    fn test_system_message() -> anyhow::Result<()> {
        let right = Message::System {
            name: Some("Eye".into()),
            content: Content(SystemMessageContent::Text {
                text: "You are a helpful assistant".to_string(),
                cache_control: None,
            }),
        };
        let message = serde_json::json!({
            "role": "system",
            "name":"Eye",
            "content": "You are a helpful assistant",
        });
        let left: Message = serde_json::from_value(message.clone())?;
        json_eq(&left, &right)?;

        Ok(())
    }

    fn json_eq<T: ?Sized + Serialize>(left: &T, right: &T) -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(left)?, serde_json::to_string(right)?);
        Ok(())
    }
}
