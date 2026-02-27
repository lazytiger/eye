//! Eye - Personal Intelligent Assistant Main Program
//!
//! Integrates all modules to provide a complete command-line interface

use eye::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
