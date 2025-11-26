//! # dps-config
//!
//! Configuration management for the DPS ecosystem.
//!
//! This crate provides the [`DpsConfig`] struct — a lightweight configuration
//! container used by Rust components in the DPS ecosystem. It focuses on:
//! - optional values (fields are `Option<T>`),
//! - sensible defaults exposed by getters,
//! - environment variable loading in `DpsConfig::new()`,
//! - computed getters for derived values (domains, URLs).
//!
//! Environment variable conventions:
//! - Boolean true is represented as the string `"Y"`.
//! - Omitted or empty environment variables are treated as unset.
//!
//! # Examples
//!
//! ```rust
//! use dps_config::DpsConfig;
//!
//! let mut config = DpsConfig::new();
//! // defaults
//! assert_eq!(config.get_domain(), "dps.localhost");
//!
//! // override and compute
//! config.set_domain("example.com");
//! config.set_api_path("api");
//! assert_eq!(config.get_auth_api_url(), "https://auth.example.com/api");
//! ```

use std::env;

/// Central configuration container for DPS components.
///
/// All fields are private and optional. Consumers interact via getters and
/// setters. Getters return sensible defaults suitable for development when a
/// value is not configured.
///
/// Note: This struct intentionally does not perform validation — consuming
/// crates should validate values where required.
pub struct DpsConfig {
  // Global properties
  domain: Option<String>,
  api_path: Option<String>,
  development_mode: Option<bool>,

  // DpsAuthApi properties
  auth_api_subdomain: Option<String>,
  auth_api_port: Option<u16>,
  auth_api_protocol: Option<String>,
  auth_api_insecure_cookie: Option<bool>,
  auth_api_sqlite_main_file_path: Option<String>,
  auth_api_sqlite_main_pool_size: Option<u16>,
  auth_api_session_secret: Option<String>,
  auth_api_session_ttl_seconds: Option<u32>,
}

impl DpsConfig {
  /// Create a new `DpsConfig` instance, loading values from environment
  /// variables when present.
  ///
  /// Environment variables:
  /// - `DPS_DOMAIN`
  /// - `DPS_API_PATH`
  /// - `DPS_DEVELOPMENT_MODE` (use `"Y"` for true)
  /// - `DPS_AUTH_API_SUBDOMAIN`
  /// - `DPS_AUTH_API_PORT`
  /// - `DPS_AUTH_API_PROTOCOL`
  /// - `DPS_AUTH_API_INSECURE_COOKIE` (use `"Y"` for true)
  /// - `DPS_AUTH_API_SQLITE_MAIN_FILE_PATH`
  /// - `DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE`
  /// - `DPS_AUTH_API_SESSION_SECRET`
  /// - `DPS_AUTH_API_SESSION_TTL_SECONDS`
  pub fn new() -> Self {
    Self {
      domain: load_env_string("DPS_DOMAIN"),
      api_path: load_env_string("DPS_API_PATH"),
      development_mode: load_env_bool("DPS_DEVELOPMENT_MODE"),
      auth_api_subdomain: load_env_string("DPS_AUTH_API_SUBDOMAIN"),
      auth_api_port: load_env_u16("DPS_AUTH_API_PORT"),
      auth_api_protocol: load_env_string("DPS_AUTH_API_PROTOCOL"),
      auth_api_insecure_cookie: load_env_bool("DPS_AUTH_API_INSECURE_COOKIE"),
      auth_api_sqlite_main_file_path: load_env_string("DPS_AUTH_API_SQLITE_MAIN_FILE_PATH"),
      auth_api_sqlite_main_pool_size: load_env_u16("DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE"),
      auth_api_session_secret: load_env_string("DPS_AUTH_API_SESSION_SECRET"),
      auth_api_session_ttl_seconds: load_env_u32("DPS_AUTH_API_SESSION_TTL_SECONDS"),
    }
  }

  // --------------------
  // Global getters/setters
  // --------------------

  /// Returns the configured domain or the default `"dps.localhost"`.
  ///
  /// Env var: `DPS_DOMAIN`
  pub fn get_domain(&self) -> String {
    self
      .domain
      .clone()
      .unwrap_or_else(|| "dps.localhost".to_string())
  }

  /// Set the domain value (overrides any environment-provided value).
  pub fn set_domain(&mut self, value: &str) {
    self.domain = Some(value.to_string());
  }

  /// Returns the API path or the default `"api"`.
  ///
  /// Env var: `DPS_API_PATH`
  pub fn get_api_path(&self) -> String {
    self.api_path.clone().unwrap_or_else(|| "api".to_string())
  }

  /// Set the API path (overrides env).
  pub fn set_api_path(&mut self, value: &str) {
    self.api_path = Some(value.to_string());
  }

  /// Returns whether development mode is enabled. Defaults to `false`.
  ///
  /// Env var: `DPS_DEVELOPMENT_MODE` using `"Y"` for `true`.
  pub fn get_development_mode(&self) -> bool {
    self.development_mode.unwrap_or(false)
  }

  /// Set development mode explicitly.
  pub fn set_development_mode(&mut self, value: bool) {
    self.development_mode = Some(value);
  }

  // --------------------
  // DpsAuthApi getters/setters
  // --------------------

  /// Returns the auth API subdomain or default `"auth"`.
  ///
  /// Env var: `DPS_AUTH_API_SUBDOMAIN`
  pub fn get_auth_api_subdomain(&self) -> String {
    self
      .auth_api_subdomain
      .clone()
      .unwrap_or_else(|| "auth".to_string())
  }

  /// Set the auth API subdomain.
  pub fn set_auth_api_subdomain(&mut self, value: &str) {
    self.auth_api_subdomain = Some(value.to_string());
  }

  /// Returns the configured auth API port, if any.
  ///
  /// Env var: `DPS_AUTH_API_PORT`
  pub fn get_auth_api_port(&self) -> Option<u16> {
    self.auth_api_port
  }

  /// Set the auth API port. Use `None` to unset.
  pub fn set_auth_api_port(&mut self, value: Option<u16>) {
    self.auth_api_port = value;
  }

  /// Returns the auth API protocol or default `"https"`.
  ///
  /// Env var: `DPS_AUTH_API_PROTOCOL`
  pub fn get_auth_api_protocol(&self) -> String {
    self
      .auth_api_protocol
      .clone()
      .unwrap_or_else(|| "https".to_string())
  }

  /// Set the auth API protocol (e.g. "http" or "https").
  pub fn set_auth_api_protocol(&mut self, value: &str) {
    self.auth_api_protocol = Some(value.to_string());
  }

  /// Returns whether insecure cookies are enabled for Auth API.
  /// Defaults to `false`.
  ///
  /// Env var: `DPS_AUTH_API_INSECURE_COOKIE` using `"Y"` for `true`.
  pub fn get_auth_api_insecure_cookie(&self) -> bool {
    self.auth_api_insecure_cookie.unwrap_or(false)
  }

  /// Set whether insecure cookies are enabled for Auth API.
  pub fn set_auth_api_insecure_cookie(&mut self, value: bool) {
    self.auth_api_insecure_cookie = Some(value);
  }

  /// Returns the SQLite main database file path for the Auth API or default
  /// `"data/main-development.db"`.
  ///
  /// Env var: `DPS_AUTH_API_SQLITE_MAIN_FILE_PATH`
  pub fn get_auth_api_sqlite_main_file_path(&self) -> String {
    self
      .auth_api_sqlite_main_file_path
      .clone()
      .unwrap_or_else(|| "data/main-development.db".to_string())
  }

  /// Set the SQLite main database file path for Auth API.
  pub fn set_auth_api_sqlite_main_file_path(&mut self, value: &str) {
    self.auth_api_sqlite_main_file_path = Some(value.to_string());
  }

  /// Returns the SQLite main database connection pool size for Auth API.
  /// Defaults to `1`.
  ///
  /// Env var: `DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE`
  pub fn get_auth_api_sqlite_main_pool_size(&self) -> u16 {
    self.auth_api_sqlite_main_pool_size.unwrap_or(1)
  }

  /// Set the SQLite main database connection pool size for Auth API.
  /// Use `None` to reset to default.
  pub fn set_auth_api_sqlite_main_pool_size(&mut self, value: Option<u16>) {
    self.auth_api_sqlite_main_pool_size = value;
  }

  /// Returns the auth API session secret as an owned `String`, if configured.
  ///
  /// Env var: `DPS_AUTH_API_SESSION_SECRET`
  pub fn get_auth_api_session_secret(&self) -> Option<String> {
    self.auth_api_session_secret.clone()
  }

  /// Set or unset the auth API session secret.
  pub fn set_auth_api_session_secret(&mut self, value: Option<&str>) {
    self.auth_api_session_secret = value.map(|s| s.to_string());
  }

  /// Returns the auth API session secret as bytes (`Vec<u8>`), if configured.
  ///
  /// This convenience getter is useful for supplying secrets to encryption or
  /// session libraries without requiring callers to convert from `String`.
  pub fn get_auth_api_session_secret_bytes(&self) -> Option<Vec<u8>> {
    self
      .auth_api_session_secret
      .as_ref()
      .map(|s| s.as_bytes().to_vec())
  }

  /// Returns the session TTL for auth in seconds. Defaults to 14 days
  /// (1209600 seconds) when not configured.
  ///
  /// Env var: `DPS_AUTH_API_SESSION_TTL_SECONDS`
  pub fn get_auth_api_session_ttl_seconds(&self) -> u32 {
    self.auth_api_session_ttl_seconds.unwrap_or(1209600)
  }

  /// Set or unset the auth session TTL in seconds.
  pub fn set_auth_api_session_ttl_seconds(&mut self, value: Option<u32>) {
    self.auth_api_session_ttl_seconds = value;
  }

  // --------------------
  // Computed getters
  // --------------------

  /// Returns the full Auth API URL, including protocol and optional port.
  ///
  /// Examples:
  /// - `https://auth.dps.localhost/api`
  /// - `http://auth.dps.localhost:3000/api`
  pub fn get_auth_api_url(&self) -> String {
    let protocol = self.get_auth_api_protocol();
    let auth_sub = self.get_auth_api_subdomain();
    let domain = self.get_domain();
    let api_path = self.get_api_path();
    if let Some(port) = self.auth_api_port {
      format!("{protocol}://{auth_sub}.{domain}:{port}/{api_path}")
    } else {
      format!("{protocol}://{auth_sub}.{domain}/{api_path}")
    }
  }
}

impl Default for DpsConfig {
  fn default() -> Self {
    Self::new()
  }
}

// --------------------
// Helper functions
// --------------------

fn load_env_string(key: &str) -> Option<String> {
  match env::var(key) {
    Ok(v) if !v.is_empty() => Some(v),
    _ => None,
  }
}

fn load_env_bool(key: &str) -> Option<bool> {
  env::var(key).ok().map(|v| v == "Y")
}

fn load_env_u16(key: &str) -> Option<u16> {
  env::var(key).ok().and_then(|v| v.parse::<u16>().ok())
}

fn load_env_u32(key: &str) -> Option<u32> {
  env::var(key).ok().and_then(|v| v.parse::<u32>().ok())
}

// --------------------
// Tests
// --------------------

#[cfg(test)]
mod tests {
  use super::*;
  use serial_test::serial;

  #[test]
  #[serial]
  fn test_default_values() {
    let config = DpsConfig::new();
    assert_eq!(config.get_domain(), "dps.localhost");
    assert_eq!(config.get_api_path(), "api");
    assert!(!config.get_development_mode());
    assert_eq!(config.get_auth_api_subdomain(), "auth");
    assert_eq!(config.get_auth_api_protocol(), "https");
    assert!(!config.get_auth_api_insecure_cookie());
    assert_eq!(
      config.get_auth_api_sqlite_main_file_path(),
      "data/main-development.db"
    );
    assert_eq!(config.get_auth_api_sqlite_main_pool_size(), 1);
    assert!(config.get_auth_api_port().is_none());
    assert!(config.get_auth_api_session_secret().is_none());
    assert!(config.get_auth_api_session_secret_bytes().is_none());
  }

  #[test]
  fn test_setters() {
    let mut config = DpsConfig::new();
    config.set_domain("example.com");
    config.set_api_path("v1");
    config.set_development_mode(true);
    config.set_auth_api_port(Some(3000));
    config.set_auth_api_session_secret(Some("s3cr3t"));

    assert_eq!(config.get_domain(), "example.com");
    assert_eq!(config.get_api_path(), "v1");
    assert!(config.get_development_mode());
    assert_eq!(config.get_auth_api_port(), Some(3000));
    assert_eq!(
      config.get_auth_api_session_secret(),
      Some("s3cr3t".to_string())
    );
  }

  #[test]
  fn test_auth_api_url_without_port() {
    let mut config = DpsConfig::new();
    config.set_auth_api_protocol("https");
    config.set_auth_api_subdomain("auth");
    config.set_domain("dps.localhost");
    assert_eq!(config.get_auth_api_url(), "https://auth.dps.localhost/api");
  }

  #[test]
  fn test_auth_api_url_with_port() {
    let mut config = DpsConfig::new();
    config.set_auth_api_protocol("http");
    config.set_auth_api_port(Some(3000));
    config.set_auth_api_subdomain("auth");
    config.set_domain("dps.localhost");
    assert_eq!(
      config.get_auth_api_url(),
      "http://auth.dps.localhost:3000/api"
    );
  }

  #[test]
  fn test_readme_example() {
    let mut config = DpsConfig::new();
    config.set_domain("test.local");
    config.set_auth_api_protocol("http");
    config.set_auth_api_port(Some(8080));
    assert_eq!(config.get_auth_api_url(), "http://auth.test.local:8080/api");
  }

  #[test]
  #[serial]
  fn test_auth_api_insecure_cookie() {
    // Test setter first (without env var interference)
    let mut c = DpsConfig::new();
    assert!(!c.get_auth_api_insecure_cookie());
    c.set_auth_api_insecure_cookie(true);
    assert!(c.get_auth_api_insecure_cookie());
    c.set_auth_api_insecure_cookie(false);
    assert!(!c.get_auth_api_insecure_cookie());

    // Test env var loading
    std::env::set_var("DPS_AUTH_API_INSECURE_COOKIE", "Y");
    let c2 = DpsConfig::new();
    assert!(c2.get_auth_api_insecure_cookie());
    std::env::remove_var("DPS_AUTH_API_INSECURE_COOKIE");
  }

  #[test]
  #[serial]
  fn test_auth_api_sqlite_main_pool_size() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_auth_api_sqlite_main_pool_size(), 1);
    c.set_auth_api_sqlite_main_pool_size(Some(12));
    assert_eq!(c.get_auth_api_sqlite_main_pool_size(), 12);
    c.set_auth_api_sqlite_main_pool_size(None);
    assert_eq!(c.get_auth_api_sqlite_main_pool_size(), 1);

    // Test env var loading
    std::env::set_var("DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE", "8");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_auth_api_sqlite_main_pool_size(), 8);
    std::env::remove_var("DPS_AUTH_API_SQLITE_MAIN_POOL_SIZE");
  }

  #[test]
  #[serial]
  fn test_auth_api_sqlite_main_file_path() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(
      c.get_auth_api_sqlite_main_file_path(),
      "data/main-development.db"
    );
    c.set_auth_api_sqlite_main_file_path("data/custom.db");
    assert_eq!(c.get_auth_api_sqlite_main_file_path(), "data/custom.db");

    // Test env var loading
    std::env::set_var("DPS_AUTH_API_SQLITE_MAIN_FILE_PATH", "data/test-main.db");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_auth_api_sqlite_main_file_path(), "data/test-main.db");
    std::env::remove_var("DPS_AUTH_API_SQLITE_MAIN_FILE_PATH");
  }

  #[test]
  fn test_auth_api_session_secret_bytes() {
    let mut config = DpsConfig::new();
    config.set_auth_api_session_secret(Some("my-secret-key"));

    let secret_bytes = config.get_auth_api_session_secret_bytes();
    assert!(secret_bytes.is_some());
    assert_eq!(secret_bytes.unwrap(), b"my-secret-key".to_vec());
  }

  #[test]
  fn test_auth_api_session_secret_bytes_none() {
    let config = DpsConfig::new();
    assert!(config.get_auth_api_session_secret_bytes().is_none());
  }

  #[test]
  #[serial]
  fn test_auth_api_session_ttl_seconds() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_auth_api_session_ttl_seconds(), 1209600); // 14 days
    c.set_auth_api_session_ttl_seconds(Some(3600));
    assert_eq!(c.get_auth_api_session_ttl_seconds(), 3600);

    // Test env var loading
    std::env::set_var("DPS_AUTH_API_SESSION_TTL_SECONDS", "1800");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_auth_api_session_ttl_seconds(), 1800);
    std::env::remove_var("DPS_AUTH_API_SESSION_TTL_SECONDS");
  }

  #[test]
  #[serial]
  fn test_api_path() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_api_path(), "api");
    c.set_api_path("v1");
    assert_eq!(c.get_api_path(), "v1");

    // Test env var loading
    std::env::set_var("DPS_API_PATH", "api/v2");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_api_path(), "api/v2");
    std::env::remove_var("DPS_API_PATH");
  }
}
