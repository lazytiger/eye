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
/// The default client has:
/// - Connect timeout: 10 seconds
/// - Request timeout: 300 seconds (5 minutes) - increased for LLM API calls
/// - Pool max idle: 10
/// - Pool idle timeout: 90 seconds
/// - HTTP2 keep-alive ping
pub fn reqwest_client() -> &'static Client {
    REQWEST_CLIENT.get_or_init(|| {
        Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(300)) // 5 minutes for LLM responses
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(90))
            .http2_keep_alive_interval(Duration::from_secs(30))
            .http2_keep_alive_timeout(Duration::from_secs(20))
            .http2_keep_alive_while_idle(true)
            .user_agent(user_agent())
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
