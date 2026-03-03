use serde::{
    Deserialize, Serialize,
    de::{Visitor, value::MapAccessDeserializer},
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

#[derive(Debug)]
pub struct Content<T>(pub T);

impl<T> Serialize for Content<T>
where
    T: TextContent + Serialize,
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
    T: TextContent + Deserialize<'a>,
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
    T: TextContent + Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_any(ContentVisitor::<T>(PhantomData))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    /// List of messages for the conversation
    pub messages: Vec<Message>,
    /// Unique user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// A unique identifier for grouping related requests (e.g., a conversation or agent workflow) for observability.
    /// If provided in both the request body and the x-session-id header, the body value takes precedence. Maximum of 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sesion_id: Option<String>,
    /// Model to use for completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Frequency penalty (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    ///Maximum tokens in completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    pub presence_penalty: Option<f32>,
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
    pub tool_choice: Option<ToolChoice>,
    /// List of tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Nucleus sampling probability (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Output modalities for the response. Supported values are "text", "image", and "audio".
    pub modalities: Option<Vec<Modalities>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Modalities {
    Text,
    Audio,
    Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolFunction {
    name: String,
    description: Option<String>,
    parameters: Option<serde_json::Value>,
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function {
        function: ToolFunction,
        cache_control: Option<CacheControl>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    Function { function: FunctionChoice },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionChoice {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchemaFormat },
    Grammar { grammar: String },
    Python,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSchemaFormat {
    name: String,
    description: Option<String>,
    schema: Option<serde_json::Value>,
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
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
        images: Option<Vec<ImageUrl>>,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputAudio {
    /// Audio output identifier
    id: Option<String>,
    /// Audio expiration timestamp
    expires_at: Option<f64>,
    /// Base64 encoded audio data
    data: Option<String>,
    /// Audio transcript
    transcript: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUrl {
    /// URL or base64-encoded data of the generated image
    image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCall {
    /// Tool call identifier
    id: String,
    tool_type: ToolType,
    function: Function,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    Function,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    /// Function name to call
    name: String,
    /// Function arguments as JSON string
    arguments: serde_json::Value,
}

/// System message content
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemMessageContent {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
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
    File {
        ///File content as base64 data URL or URL
        file_data: Option<String>,
        /// File ID for previously uploaded files
        file_id: Option<String>,
        /// Original filename
        file_name: Option<String>,
    },
}

/// Video input object
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VideoUrl {
    /// URL of the video (data: URLs supported)
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Auto,
    Low,
    High,
}

/// Cache control for the content part
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CacheControl {
    Ephemeral {
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl: Option<TTL>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TTL {
    #[serde(rename = "5m")]
    FiveMinute,
    #[serde(rename = "1h")]
    OneHour,
}

/// Successful chat completion response
pub struct Response {
    /// Unique completion identifier
    pub id: String,

    pub choices: Vec<Choice>,
}

pub struct Choice {
    pub finish_reason: String,
    pub index: usize,
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
