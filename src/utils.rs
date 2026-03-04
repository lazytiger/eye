use reqwest::Client;
use std::sync::OnceLock;
use std::time::Duration;

static USER_AGENT: OnceLock<String> = OnceLock::new();

/// Set the user agent string.
///
/// This should be called before [user_agent].
pub fn set_user_agent(s: String) {
    let _ = USER_AGENT.set(s);
}

/// Get the user agent string.
///
/// If not set, a default user agent string will be returned.
pub fn user_agent() -> &'static str {
    USER_AGENT.get_or_init(||"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.2365.66".into())
}

static REQWEST_CLIENT: OnceLock<Client> = OnceLock::new();

/// Get the reqwest client.
///
/// If not set, a default client will be returned.
///
/// The default client will have a connect timeout of 10 seconds and a timeout of 120 seconds.
pub fn reqwest_client() -> &'static Client {
    REQWEST_CLIENT.get_or_init(|| {
        Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Client builder failed")
    })
}

/// Set the reqwest client.
///
/// This should be called before [reqwest_client].
pub fn set_reqwest_client(client: Client) {
    let _ = REQWEST_CLIENT.set(client);
}
