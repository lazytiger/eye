# Time Tool Design

## Overview
The Time tool provides the capability for the LLM to access the current local date and time. This is essential for answering temporal questions based on the user's current context (e.g., "What is the date today?", "What time is it now?").

## Tool Definition

### Name
`get_current_time`

### Description
Retrieves the current local date and time.

### Parameters (JSON Schema)
```json
{
  "type": "object",
  "properties": {},
  "required": []
}
```

## Implementation Details

### Dependencies
- `chrono`: For date and time manipulation.

### Execution Logic
1. **Get Current Time**: 
   - Get the current local time using `chrono::Local::now()`.
2. **Format Output**:
   - Use the default `Display` implementation or `to_string()` which produces an ISO 8601-like format with timezone offset.
3. **Return**:
   - `ExecuteResult::Success` with the time string.

### Error Handling
- Generally, this operation should not fail unless the system clock is inaccessible.

## Example Usage

**Input:**
```json
{}
```

**Output:**
```json
"2023-10-27T10:30:00.123456+08:00"
```
