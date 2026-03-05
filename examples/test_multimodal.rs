//! Example to test multimodal support in Provider trait

use eye::provider::{
    ChatRequest, ChatMessage, Role, Content, ContentPart, ImageUrl, AudioUrl, VideoUrl, 
    ImageDetail, AudioFormat, VideoFormat, create_openai_compatible, Provider,
    ModelCapabilities
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing multimodal support in Provider trait...");
    
    // Create a provider
    let provider = create_openai_compatible("openai", "test-key", "gpt-4-vision-preview")?;
    
    println!("1. Testing text-only message:");
    test_text_message(&provider).await?;
    
    println!("\n2. Testing image message:");
    test_image_message(&provider).await?;
    
    println!("\n3. Testing audio message:");
    test_audio_message(&provider).await?;
    
    println!("\n4. Testing video message:");
    test_video_message(&provider).await?;
    
    println!("\n5. Testing mixed multimodal message:");
    test_mixed_message(&provider).await?;
    
    println!("\n6. Testing capabilities:");
    test_capabilities(&provider);
    
    println!("\nAll multimodal tests completed!");
    Ok(())
}

async fn test_text_message(provider: &impl Provider) -> Result<()> {
    // Create a simple text message
    let message = ChatMessage::new_text(Role::User, "Hello, world!");
    
    let request = ChatRequest {
        messages: vec![message],
        model: provider.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(100),
        n: Some(1),
        stop: None,
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        parallel_tool_calls: None,
        stream_options: None,
    };
    
    let response = provider.chat(request).await?;
    println!("  - Text message response received");
    println!("  - Model: {}", response.model);
    println!("  - Response text: {:?}", response.choices[0].message.text_content());
    
    Ok(())
}

async fn test_image_message(provider: &impl Provider) -> Result<()> {
    // Create a message with image
    let message = ChatMessage::new_with_image(
        Role::User,
        "What's in this image?",
        "https://example.com/image.jpg"
    );
    
    // Alternatively, create using the more flexible API
    let message2 = ChatMessage::new_multimodal(Role::User, vec![
        ContentPart::Text { text: "Describe this image:".to_string() },
        ContentPart::ImageUrl { 
            image_url: ImageUrl { 
                url: "https://example.com/photo.png".to_string(),
                detail: Some(ImageDetail::High),
            }
        },
    ]);
    
    let request = ChatRequest {
        messages: vec![message, message2],
        model: provider.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(200),
        n: Some(1),
        stop: None,
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        parallel_tool_calls: None,
        stream_options: None,
    };
    
    let response = provider.chat(request).await?;
    println!("  - Image message response received");
    println!("  - Model: {}", response.model);
    println!("  - Is multimodal: {}", response.choices[0].message.is_multimodal());
    
    Ok(())
}

async fn test_audio_message(provider: &impl Provider) -> Result<()> {
    // Create a message with audio
    let message = ChatMessage::new_with_audio(
        Role::User,
        "Transcribe this audio:",
        "https://example.com/audio.mp3"
    );
    
    // Alternatively, create with format specification
    let message2 = ChatMessage::new_multimodal(Role::User, vec![
        ContentPart::Text { text: "What language is this?".to_string() },
        ContentPart::AudioUrl { 
            audio_url: AudioUrl { 
                url: "https://example.com/speech.wav".to_string(),
                format: Some(AudioFormat::Wav),
                language: Some("en".to_string()),
            }
        },
    ]);
    
    let request = ChatRequest {
        messages: vec![message, message2],
        model: provider.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(200),
        n: Some(1),
        stop: None,
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        parallel_tool_calls: None,
        stream_options: None,
    };
    
    let response = provider.chat(request).await?;
    println!("  - Audio message response received");
    println!("  - Model: {}", response.model);
    
    Ok(())
}

async fn test_video_message(provider: &impl Provider) -> Result<()> {
    // Create a message with video
    let message = ChatMessage::new_with_video(
        Role::User,
        "Describe this video:",
        "https://example.com/video.mp4"
    );
    
    // Alternatively, create with format specification
    let message2 = ChatMessage::new_multimodal(Role::User, vec![
        ContentPart::Text { text: "Extract audio from this video:".to_string() },
        ContentPart::VideoUrl { 
            video_url: VideoUrl { 
                url: "https://example.com/movie.mov".to_string(),
                format: Some(VideoFormat::Mov),
                audio_only: Some(true),
            }
        },
    ]);
    
    let request = ChatRequest {
        messages: vec![message, message2],
        model: provider.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(200),
        n: Some(1),
        stop: None,
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        parallel_tool_calls: None,
        stream_options: None,
    };
    
    let response = provider.chat(request).await?;
    println!("  - Video message response received");
    println!("  - Model: {}", response.model);
    
    Ok(())
}

async fn test_mixed_message(provider: &impl Provider) -> Result<()> {
    // Create a mixed multimodal message
    let message = ChatMessage::new_multimodal(Role::User, vec![
        ContentPart::Text { text: "Analyze this content:".to_string() },
        ContentPart::ImageUrl { 
            image_url: ImageUrl { 
                url: "https://example.com/chart.png".to_string(),
                detail: Some(ImageDetail::Auto),
            }
        },
        ContentPart::AudioUrl { 
            audio_url: AudioUrl { 
                url: "https://example.com/explanation.mp3".to_string(),
                format: Some(AudioFormat::Mp3),
                language: None,
            }
        },
        ContentPart::VideoUrl { 
            video_url: VideoUrl { 
                url: "https://example.com/demo.webm".to_string(),
                format: Some(VideoFormat::Webm),
                audio_only: Some(false),
            }
        },
    ]);
    
    let request = ChatRequest {
        messages: vec![message],
        model: provider.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(300),
        n: Some(1),
        stop: None,
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        parallel_tool_calls: None,
        stream_options: None,
    };
    
    let response = provider.chat(request).await?;
    println!("  - Mixed multimodal message response received");
    println!("  - Model: {}", response.model);
    println!("  - Is multimodal: {}", response.choices[0].message.is_multimodal());
    
    Ok(())
}

fn test_capabilities(provider: &impl Provider) {
    let capabilities = provider.capabilities();
    
    println!("  - Model capabilities:");
    println!("    - TEXT_GENERATION: {}", capabilities.contains(ModelCapabilities::TEXT_GENERATION));
    println!("    - FUNCTION_CALLING: {}", capabilities.contains(ModelCapabilities::FUNCTION_CALLING));
    println!("    - VISION: {}", capabilities.contains(ModelCapabilities::VISION));
    println!("    - AUDIO_INPUT: {}", capabilities.contains(ModelCapabilities::AUDIO_INPUT));
    println!("    - VIDEO_INPUT: {}", capabilities.contains(ModelCapabilities::VIDEO_INPUT));
    println!("    - OBJECT_GENERATION: {}", capabilities.contains(ModelCapabilities::OBJECT_GENERATION));
    
    println!("  - Max context length: {}", provider.max_context_length());
}