# DeepSeek Chat Completion API

## Request Body

| Field | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `messages` | `array` | Required | A list of messages comprising the conversation so far. |
| `model` | `string` | Required | ID of the model to use. Possible values: `deepseek-chat`, `deepseek-reasoner`. |
| `frequency_penalty` | `number` | Optional | Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim. Default: 0. |
| `max_tokens` | `integer` | Optional | The maximum number of tokens to generate in the completion. |
| `presence_penalty` | `number` | Optional | Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics. Default: 0. |
| `response_format` | `object` | Optional | An object specifying the format that the model must output. |
| `stop` | `string/array` | Optional | Up to 4 sequences where the API will stop generating further tokens. |
| `stream` | `boolean` | Optional | If set, partial message deltas will be sent, like in ChatGPT. Tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a `data: [DONE]` message. |
| `stream_options` | `object` | Optional | Options for streaming response. |
| `temperature` | `number` | Optional | What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. Default: 1. |
| `top_p` | `number` | Optional | An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. Default: 1. |
| `tools` | `array` | Optional | A list of tools the model may call. Currently, only functions are supported as a tool. |
| `tool_choice` | `string/object` | Optional | Controls which (if any) function is called by the model. |
| `logprobs` | `boolean` | Optional | Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the `content` of `message`. |
| `top_logprobs` | `integer` | Optional | An integer between 0 and 20 specifying the number of most likely tokens to return at each position, each with an associated log probability. `logprobs` must be set to `true` if this parameter is used. |
| `thinking` | `object` | Optional | Configuration for the "thinking" feature. |

## Response

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | `string` | A unique identifier for the chat completion. |
| `choices` | `array` | A list of chat completion choices. |
| `created` | `integer` | The Unix timestamp (in seconds) of when the chat completion was created. |
| `model` | `string` | The model used for the chat completion. |
| `system_fingerprint` | `string` | This fingerprint represents the backend configuration that the model runs with. |
| `object` | `string` | The object type, which is always `chat.completion`. |
| `usage` | `object` | Usage statistics for the completion request. |
