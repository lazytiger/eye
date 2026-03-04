use reqwest::Client;
use std::sync::OnceLock;
use std::time::Duration;

static USER_AGENT: OnceLock<String> = OnceLock::new();

pub fn set_user_agent(s: String) {
    let _ = USER_AGENT.set(s);
}

pub fn get_user_agent() -> &'static str {
    USER_AGENT.get().map(|s| s.as_str()).unwrap_or("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.2365.66")
}

static REQWEST_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn reqwest_client() -> &'static Client {
    REQWEST_CLIENT.get_or_init(|| {
        Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Client builder failed")
    })
}
