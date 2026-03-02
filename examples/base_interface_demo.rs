//! Demonstration of the BaseInterface trait usage
//!
//! This example shows how to use the simplified BaseInterface with send/listen methods

use eye::{
    config::settings,
    interface::{BaseInterface, CliInterface},
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("BaseInterface Demo");
    println!("==================");
    
    // Create interface configuration
    let config = settings::InterfaceConfig {
        prompt: "> ".to_string(),
        enable_colors: true,
        show_timestamp: true,
    };
    
    // Create CLI interface wrapped in Arc for shared ownership
    let interface = Arc::new(CliInterface::new(config));
    
    // Create channel for responses
    let (tx, _rx) = mpsc::channel::<String>(10);
    
    // Start listening in a separate task
    let listen_handle = tokio::spawn({
        let interface = Arc::clone(&interface);
        async move {
            println!("Starting to listen for input...");
            interface.listen(tx).await.unwrap();
        }
    });
    
    // Send some messages
    println!("\nSending messages to interface:");
    interface.send("Hello from BaseInterface!".to_string()).await?;
    interface.send("This is a test message.".to_string()).await?;
    
    // In a real application, you would process responses from the channel
    // For this demo, we'll just wait a bit and then stop
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Cancel the listening task
    listen_handle.abort();
    
    println!("\nDemo completed!");
    Ok(())
}