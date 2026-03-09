# Plan: Add Audio, Video, File Support to OpenRouter ContentPart

## Context

根据 OpenRouter API 规范（docs/provider/openrouter.trimmed.yaml），openrouter 的 chat/completions 接口支持多模态输入，包括 audio、video、image 和 file。当前 `src/provider/openrouter/chat_types/messages.rs` 中的 `ContentPart` 枚举只定义了 `Text` 和 `ImageUrl`，需要补充缺失的类型。

同时需要在 `src/provider/openrouter/convert.rs` 中实现从 provider-agnostic types（已支持这些类型）到 OpenRouter 特定类型的转换。

## Required Changes

### 1. 更新 `src/provider/openrouter/chat_types/messages.rs`

添加新的 `ContentPart` 变体：
- `InputAudio { input_audio: InputAudio }`
- `InputVideo { input_video: InputVideo }`（支持两种格式：`input_video` 和 `video_url`）
- `Document { document: Document }`（文件类型）

添加相关辅助类型：
- `InputAudio` - data: URL 或 HTTP URL
- `InputVideo` / `VideoInput` - 视频 URL
- `Document` - 文件数据（base64 或 URL）

### 2. 更新 `src/provider/openrouter/convert.rs`

在 `From<TypesChatRequest> for ChatRequest` 实现中，为 `ContentPart` 转换添加分支：
- `TypesContentPart::InputAudio` → `ContentPart::InputAudio`
- `TypesContentPart::InputVideo` → `ContentPart::InputVideo`
- `TypesContentPart::Document` → `ContentPart::Document`

## Files to Modify

1. `src/provider/openrouter/chat_types/messages.rs` - 添加新类型定义
2. `src/provider/openrouter/convert.rs` - 添加转换逻辑

## Reference

- OpenRouter API 规范：`docs/provider/openrouter.trimmed.yaml`
- Provider-agnostic 类型：`src/provider/types/chat.rs` (已包含 `InputAudio`, `InputVideo`, `Document` 等类型)

## Verification

1. `cargo build` - 确保编译通过
2. 检查转换逻辑覆盖所有 `ContentPart` 变体
3. 确认 YAML 中定义的所有类型都已实现
