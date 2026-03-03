
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    None,
    Auto,
    Required,
    Function { function: FunctionChoice },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionChoice {
    name: String,
}

fn main() {
    let auto = ToolChoice::Auto;
    let json = serde_json::to_string(&auto).unwrap();
    println!("Auto: {}", json);
    
    // Check if it serializes to {"type":"auto"} which is incorrect for OpenAI
    // OpenAI expects "auto" string for auto, and object for function
    assert_eq!(json, r#"{"type":"auto"}"#); 
    println!("Test confirmed: ToolChoice::Auto serializes to object instead of string");
}
