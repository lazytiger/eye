#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let resp = client.get("https://www.google.com").send().await?;
    println!("Status: {}", resp.status());
    Ok(())
}
