use eye::init_tracing;

fn main() -> anyhow::Result<()> {
    let _guard = init_tracing(None)?;
    tracing::info!("Hello, world!");
    Ok(())
}
