# Search Tool Design

## Overview
The Search tool enables the LLM to retrieve real-time information from the web. This is crucial for answering questions about current events, specific facts, or topics not covered in the training data.

## Tool Definition

### Name
`search_web`

### Description
Searches the web for a given query and returns a list of relevant results.

### Parameters (JSON Schema)
```json
{
  "type": "object",
  "properties": {
    "query": {
      "type": "string",
      "description": "The search query."
    },
    "num_results": {
      "type": "integer",
      "description": "The number of search results to return. Defaults to 5.",
      "minimum": 1,
      "maximum": 10
    }
  },
  "required": ["query"]
}
```

## Implementation Details

### Dependencies
- `reqwest`: For making HTTP requests to a search API.
- `serde`: For deserializing API responses.
- `anyhow`: For error handling.

### Execution Logic
1. **Parse Arguments**: Extract `query` and `num_results` from the input `serde_json::Value`.
2. **Construct Request**:
   - Determine the search provider (e.g., Google Custom Search JSON API, Bing Web Search API, SerpApi).
   - Construct the API request URL with the `query` and `num_results`.
   - Add necessary headers (API keys, content type).
3. **Execute Request**:
   - Use `reqwest` to send the HTTP GET request.
   - Handle network errors and HTTP status codes.
4. **Process Response**:
   - Deserialize the JSON response.
   - Extract relevant fields for each result (e.g., `title`, `link`, `snippet`).
   - Format the results into a structured JSON array or a concise text summary.
5. **Return**:
   - `ExecuteResult::Success` with the list of search results.

### Search Provider Abstraction
Ideally, the tool should be configurable to use different search backends. A trait or configuration struct can be used to switch between providers.

**Example Result Structure:**
```rust
struct SearchResult {
    title: String,
    link: String,
    snippet: String,
}
```

### Error Handling
- **Network Errors**: Return `ExecuteResult::Failure` indicating connection issues.
- **API Errors**: Return `ExecuteResult::Failure` with the API error message (e.g., quota exceeded, invalid key).
- **No Results**: Return `ExecuteResult::Success` with an empty list or a message indicating no results found.

## Example Usage

**Input:**
```json
{
  "query": "Rust async traits",
  "num_results": 3
}
```

**Output:**
```json
[
  {
    "title": "Async traits in Rust - The Rust Programming Language",
    "link": "https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html",
    "snippet": "Currently, async fn in traits are not supported natively..."
  },
  ...
]
```
