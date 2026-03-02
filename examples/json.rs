use openai_api_rs::v1::types::FunctionParameters;
use serde_json::json;

fn main() {
    let v = json!({
        "type": "object",
        "properties": {
            "operation": {
                "type": "string",
                "enum": ["status", "diff", "log", "branch", "commit", "add", "checkout", "stash"],
                "description": "Git operation to perform"
            },
            "message": {
                "type": "string",
                "description": "Commit message (for 'commit' operation)"
            },
            "paths": {
                "type": "string",
                "description": "File paths to stage (for 'add' operation)"
            },
            "branch": {
                "type": "string",
                "description": "Branch name (for 'checkout' operation)"
            },
            "files": {
                "type": "string",
                "description": "File or path to diff (for 'diff' operation, default: '.')"
            },
            "cached": {
                "type": "boolean",
                "description": "Show staged changes (for 'diff' operation)"
            },
            "limit": {
                "type": "number",
                "description": "Number of log entries (for 'log' operation, default: 10)"
            },
            "action": {
                "type": "string",
                "enum": ["push", "pop", "list", "drop"],
                "description": "Stash action (for 'stash' operation)"
            },
            "index": {
                "type": "number",
                "description": "Stash index (for 'stash' with 'drop' action)"
            }
        },
        "required": ["operation"]
    });

    let tool: FunctionParameters = serde_json::from_value(v).unwrap();
    println!("{:?}", tool);
}
