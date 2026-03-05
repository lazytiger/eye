# Web Fetch Tool Design

## Overview
The Web Fetch tool allows the LLM to retrieve the content of a webpage. This is useful for reading articles, documentation, or any other web content needed to answer a user's query.

## Tool Definition

### Name
`fetch_webpage`

### Description
Fetches the content of a given URL and converts it to a readable markdown format.

### Parameters (JSON Schema)
```json
{
  "type": "object",
  "properties": {
    "url": {
      "type": "string",
      "description": "The URL of the webpage to fetch."
    }
  },
  "required": ["url"]
}
```

## Implementation Details

### Dependencies
- `reqwest`: For fetching webpage content.
- `html2text`: For converting HTML to readable markdown text.
- `url`: For URL validation.

### Execution Logic
1. **Parse Arguments**: Extract `url` from the input `serde_json::Value`.
2. **Validate URL**:
   - Check if the URL is valid.
   - Ensure it's an HTTP/HTTPS URL.
3. **Fetch Content**:
   - Send a GET request using `reqwest`.
   - Set User-Agent header to mimic a browser (e.g., `Mozilla/5.0...`) to avoid simple bot detection.
   - Handle HTTP errors (404, 500, timeouts).
4. **Convert to Markdown**:
   - Use `html2text::from_read` or similar function to convert the HTML response body to markdown text.
   - This removes scripts, styles, and other non-content elements, providing a clean text representation for the LLM.
5. **Return**:
   - `ExecuteResult::Success` with the markdown content string.

### Error Handling
- **Invalid URL**: Return `ExecuteResult::Failure` with "Invalid URL format".
- **Network Error**: Return `ExecuteResult::Failure` with connection error details.
- **Conversion Error**: Return `ExecuteResult::Failure` if the HTML cannot be processed.

## Example Usage

**Input:**
```json
{
  "url": "https://example.com/blog/post-1"
}
```

**Output:**
```json
"# Blog Post Title\n\nThis is the content of the blog post in markdown format..."
```
