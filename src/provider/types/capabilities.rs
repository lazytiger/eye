//! Provider capabilities
//!
//! This module defines the ProviderCapabilities bitflags.

bitflags::bitflags! {
    /// Provider capabilities
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProviderCapabilities: u32 {
        const CHAT = 1 << 0;
        const EMBEDDINGS = 1 << 1;
        const VISION = 1 << 2;
        const AUDIO_INPUT = 1 << 3;
        const AUDIO_OUTPUT = 1 << 4;
        const VIDEO_INPUT = 1 << 5;
        const FUNCTION_CALLING = 1 << 6;
        const JSON_MODE = 1 << 7;
        const STREAMING = 1 << 8;
    }
}
