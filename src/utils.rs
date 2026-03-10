use reqwest::Client;
use std::sync::OnceLock;
use crate::config::settings::HttpClientConfig;

static REQWEST_CLIENT: OnceLock<Client> = OnceLock::new();

/// Get the reqwest client.
///
/// The client is built from the global settings configuration.
/// If settings are not initialized, a default client will be returned.
///
/// The default client has:
/// - Connect timeout: 10 seconds
/// - Request timeout: 300 seconds (5 minutes) - increased for LLM API calls
/// - Pool max idle: 10
/// - Pool idle timeout: 90 seconds
/// - HTTP2 keep-alive ping
pub fn reqwest_client() -> &'static Client {
    REQWEST_CLIENT.get_or_init(|| {
        // Try to get settings from global config
        if let Ok(settings) = crate::config::settings::get_settings() {
            settings.http.build_client()
        } else {
            // Use default HTTP config if settings not initialized
            HttpClientConfig::default().build_client()
        }
    })
}

/// Set the reqwest client.
///
/// This should be called before [reqwest_client].
pub fn set_reqwest_client(client: Client) {
    let _ = REQWEST_CLIENT.set(client);
}
