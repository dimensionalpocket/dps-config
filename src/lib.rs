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
//! assert_eq!(config.get_backend_url(), "https://auth.example.com/api");
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
#[allow(clippy::type_complexity)]
pub struct DpsConfig {
  // Global properties
  project_name: Option<String>,
  domain: Option<String>,
  api_path: Option<String>,
  development_mode: Option<bool>,

  // DpsBackend properties
  backend_subdomain: Option<String>,
  backend_port: Option<u16>,
  backend_protocol: Option<String>,
  backend_insecure_cookie: Option<bool>,
  backend_sqlite_main_file_path: Option<String>,
  backend_sqlite_main_pool_size: Option<u16>,
  backend_sqlite_collection_file_path: Option<String>,
  backend_sqlite_collection_pool_size: Option<u16>,
  backend_sqlite_session_file_path: Option<String>,
  backend_sqlite_session_pool_size: Option<u16>,
  backend_session_secret: Option<String>,
  backend_session_ttl_seconds: Option<u32>,

  // Session conversion functions
  session_sub_to_user_id_fn: Box<dyn Fn(&str) -> anyhow::Result<i64> + Send + Sync>,
  session_user_to_sub_fn: Box<dyn Fn(&serde_json::Value) -> anyhow::Result<String> + Send + Sync>,
}

impl DpsConfig {
  /// Create a new `DpsConfig` instance, loading values from environment
  /// variables when present.
  ///
  /// Environment variables:
  /// - `DPS_DOMAIN`
  /// - `DPS_API_PATH`
  /// - `DPS_DEVELOPMENT_MODE` (use `"Y"` for true)
  /// - `DPS_BACKEND_SUBDOMAIN`
  /// - `DPS_BACKEND_PORT`
  /// - `DPS_BACKEND_PROTOCOL`
  /// - `DPS_BACKEND_INSECURE_COOKIE` (use `"Y"` for true)
  /// - `DPS_BACKEND_SQLITE_MAIN_FILE_PATH`
  /// - `DPS_BACKEND_SQLITE_MAIN_POOL_SIZE`
  /// - `DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH`
  /// - `DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE`
  /// - `DPS_BACKEND_SQLITE_SESSION_FILE_PATH`
  /// - `DPS_BACKEND_SQLITE_SESSION_POOL_SIZE`
  /// - `DPS_BACKEND_SESSION_SECRET`
  /// - `DPS_BACKEND_SESSION_TTL_SECONDS`
  pub fn new() -> Self {
    Self {
      project_name: load_env_string("DPS_PROJECT_NAME"),
      domain: load_env_string("DPS_DOMAIN"),
      api_path: load_env_string("DPS_API_PATH"),
      development_mode: load_env_bool("DPS_DEVELOPMENT_MODE"),
      backend_subdomain: load_env_string("DPS_BACKEND_SUBDOMAIN"),
      backend_port: load_env_u16("DPS_BACKEND_PORT"),
      backend_protocol: load_env_string("DPS_BACKEND_PROTOCOL"),
      backend_insecure_cookie: load_env_bool("DPS_BACKEND_INSECURE_COOKIE"),
      backend_sqlite_main_file_path: load_env_string("DPS_BACKEND_SQLITE_MAIN_FILE_PATH"),
      backend_sqlite_main_pool_size: load_env_u16("DPS_BACKEND_SQLITE_MAIN_POOL_SIZE"),
      backend_sqlite_collection_file_path: load_env_string(
        "DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH",
      ),
      backend_sqlite_collection_pool_size: load_env_u16("DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE"),
      backend_sqlite_session_file_path: load_env_string("DPS_BACKEND_SQLITE_SESSION_FILE_PATH"),
      backend_sqlite_session_pool_size: load_env_u16("DPS_BACKEND_SQLITE_SESSION_POOL_SIZE"),
      backend_session_secret: load_env_string("DPS_BACKEND_SESSION_SECRET"),
      backend_session_ttl_seconds: load_env_u32("DPS_BACKEND_SESSION_TTL_SECONDS"),
      session_sub_to_user_id_fn: Box::new(|sub: &str| {
        sub.parse::<i64>().map_err(anyhow::Error::new)
      }),
      session_user_to_sub_fn: Box::new(|record: &serde_json::Value| {
        record
          .get("id")
          .and_then(|v| {
            if let Some(n) = v.as_u64() {
              Some(Ok(n.to_string()))
            } else {
              v.as_str().map(|s| Ok(s.to_string()))
            }
          })
          .unwrap_or_else(|| Err(anyhow::anyhow!("missing or invalid 'id' field")))
      }),
    }
  }

  // --------------------
  // Global getters/setters
  // --------------------

  /// Returns the configured project name or the default `"My Project"`.
  ///
  /// Env var: `DPS_PROJECT_NAME`
  pub fn get_project_name(&self) -> String {
    self
      .project_name
      .clone()
      .unwrap_or_else(|| "My Project".to_string())
  }

  /// Set the project name value (overrides any environment-provided value).
  pub fn set_project_name(&mut self, value: &str) {
    self.project_name = Some(value.to_string());
  }

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
  // DpsBackend getters/setters
  // --------------------

  /// Returns the backend API subdomain or default `"auth"`.
  ///
  /// Env var: `DPS_BACKEND_SUBDOMAIN`
  pub fn get_backend_subdomain(&self) -> String {
    self
      .backend_subdomain
      .clone()
      .unwrap_or_else(|| "auth".to_string())
  }

  /// Set the backend API subdomain.
  pub fn set_backend_subdomain(&mut self, value: &str) {
    self.backend_subdomain = Some(value.to_string());
  }

  /// Returns the configured backend API port, if any.
  ///
  /// Env var: `DPS_BACKEND_PORT`
  pub fn get_backend_port(&self) -> Option<u16> {
    self.backend_port
  }

  /// Set the backend API port. Use `None` to unset.
  pub fn set_backend_port(&mut self, value: Option<u16>) {
    self.backend_port = value;
  }

  /// Returns the backend API protocol or default `"https"`.
  ///
  /// Env var: `DPS_BACKEND_PROTOCOL`
  pub fn get_backend_protocol(&self) -> String {
    self
      .backend_protocol
      .clone()
      .unwrap_or_else(|| "https".to_string())
  }

  /// Set the backend API protocol (e.g. "http" or "https").
  pub fn set_backend_protocol(&mut self, value: &str) {
    self.backend_protocol = Some(value.to_string());
  }

  /// Returns whether insecure cookies are enabled for Backend API.
  /// Defaults to `false`.
  ///
  /// Env var: `DPS_BACKEND_INSECURE_COOKIE` using `"Y"` for `true`.
  pub fn get_backend_insecure_cookie(&self) -> bool {
    self.backend_insecure_cookie.unwrap_or(false)
  }

  /// Set whether insecure cookies are enabled for Backend API.
  pub fn set_backend_insecure_cookie(&mut self, value: bool) {
    self.backend_insecure_cookie = Some(value);
  }

  /// Returns the SQLite main database file path for the Backend API or default
  /// `"data/main-development.db"`.
  ///
  /// Env var: `DPS_BACKEND_SQLITE_MAIN_FILE_PATH`
  pub fn get_backend_sqlite_main_file_path(&self) -> String {
    self
      .backend_sqlite_main_file_path
      .clone()
      .unwrap_or_else(|| "data/main-development.db".to_string())
  }

  /// Set the SQLite main database file path for Backend API.
  pub fn set_backend_sqlite_main_file_path(&mut self, value: &str) {
    self.backend_sqlite_main_file_path = Some(value.to_string());
  }

  /// Returns the SQLite main database connection pool size for Backend API.
  /// Defaults to `1`.
  ///
  /// Env var: `DPS_BACKEND_SQLITE_MAIN_POOL_SIZE`
  pub fn get_backend_sqlite_main_pool_size(&self) -> u16 {
    self.backend_sqlite_main_pool_size.unwrap_or(1)
  }

  /// Set the SQLite main database connection pool size for Backend API.
  /// Use `None` to reset to default.
  pub fn set_backend_sqlite_main_pool_size(&mut self, value: Option<u16>) {
    self.backend_sqlite_main_pool_size = value;
  }

  /// Returns the SQLite collection database file path for the Backend API or
  /// default `"data/collection-development.db"`.
  ///
  /// Env var: `DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH`
  pub fn get_backend_sqlite_collection_file_path(&self) -> String {
    self
      .backend_sqlite_collection_file_path
      .clone()
      .unwrap_or_else(|| "data/collection-development.db".to_string())
  }

  /// Set the SQLite collection database file path for Backend API.
  pub fn set_backend_sqlite_collection_file_path(&mut self, value: &str) {
    self.backend_sqlite_collection_file_path = Some(value.to_string());
  }

  /// Returns the SQLite collection database connection pool size for Backend API.
  /// Defaults to `1`.
  ///
  /// Env var: `DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE`
  pub fn get_backend_sqlite_collection_pool_size(&self) -> u16 {
    self.backend_sqlite_collection_pool_size.unwrap_or(1)
  }

  /// Set the SQLite collection database connection pool size for Backend API.
  /// Use `None` to reset to default.
  pub fn set_backend_sqlite_collection_pool_size(&mut self, value: Option<u16>) {
    self.backend_sqlite_collection_pool_size = value;
  }

  /// Returns the SQLite session database file path for the Backend API or default
  /// `"data/session-development.db"`.
  ///
  /// Env var: `DPS_BACKEND_SQLITE_SESSION_FILE_PATH`
  pub fn get_backend_sqlite_session_file_path(&self) -> String {
    self
      .backend_sqlite_session_file_path
      .clone()
      .unwrap_or_else(|| "data/session-development.db".to_string())
  }

  /// Set the SQLite session database file path for Backend API.
  pub fn set_backend_sqlite_session_file_path(&mut self, value: &str) {
    self.backend_sqlite_session_file_path = Some(value.to_string());
  }

  /// Returns the SQLite session database connection pool size for Backend API.
  /// Defaults to `1`.
  ///
  /// Env var: `DPS_BACKEND_SQLITE_SESSION_POOL_SIZE`
  pub fn get_backend_sqlite_session_pool_size(&self) -> u16 {
    self.backend_sqlite_session_pool_size.unwrap_or(1)
  }

  /// Set the SQLite session database connection pool size for Backend API.
  /// Use `None` to reset to default.
  pub fn set_backend_sqlite_session_pool_size(&mut self, value: Option<u16>) {
    self.backend_sqlite_session_pool_size = value;
  }

  /// Returns the backend API session secret as an owned `String`, if configured.
  ///
  /// Env var: `DPS_BACKEND_SESSION_SECRET`
  pub fn get_backend_session_secret(&self) -> Option<String> {
    self.backend_session_secret.clone()
  }

  /// Set or unset the backend API session secret.
  pub fn set_backend_session_secret(&mut self, value: Option<&str>) {
    self.backend_session_secret = value.map(|s| s.to_string());
  }

  /// Returns the backend API session secret as bytes (`Vec<u8>`), if configured.
  ///
  /// This convenience getter is useful for supplying secrets to encryption or
  /// session libraries without requiring callers to convert from `String`.
  pub fn get_backend_session_secret_bytes(&self) -> Option<Vec<u8>> {
    self
      .backend_session_secret
      .as_ref()
      .map(|s| s.as_bytes().to_vec())
  }

  /// Returns the session TTL for backend in seconds. Defaults to 14 days
  /// (1209600 seconds) when not configured.
  ///
  /// Env var: `DPS_BACKEND_SESSION_TTL_SECONDS`
  pub fn get_backend_session_ttl_seconds(&self) -> u32 {
    self.backend_session_ttl_seconds.unwrap_or(1209600)
  }

  /// Set or unset the backend session TTL in seconds.
  pub fn set_backend_session_ttl_seconds(&mut self, value: Option<u32>) {
    self.backend_session_ttl_seconds = value;
  }

  // --------------------
  // Session conversion functions
  // --------------------

  /// Returns the function that converts a session `sub` string to an `i64` user ID.
  ///
  /// Default: Parses the string as `i64`, returning a parse error on failure.
  pub fn get_session_sub_to_user_id_fn(&self) -> &dyn Fn(&str) -> anyhow::Result<i64> {
    self.session_sub_to_user_id_fn.as_ref()
  }

  /// Set the session sub to user ID conversion function.
  pub fn set_session_sub_to_user_id_fn(
    &mut self,
    f: impl Fn(&str) -> anyhow::Result<i64> + Send + Sync + 'static,
  ) {
    self.session_sub_to_user_id_fn = Box::new(f);
  }

  /// Returns the function that extracts a `sub` string from a JSON record.
  ///
  /// Default: Returns the `id` property of the record as a string, or an error
  /// if `id` is missing or invalid.
  pub fn get_session_user_to_sub_fn(
    &self,
  ) -> &dyn Fn(&serde_json::Value) -> anyhow::Result<String> {
    self.session_user_to_sub_fn.as_ref()
  }

  /// Set the session user to sub conversion function.
  pub fn set_session_user_to_sub_fn(
    &mut self,
    f: impl Fn(&serde_json::Value) -> anyhow::Result<String> + Send + Sync + 'static,
  ) {
    self.session_user_to_sub_fn = Box::new(f);
  }

  // --------------------
  // Computed getters
  // --------------------

  /// Returns the full Backend API URL, including protocol and optional port.
  ///
  /// Examples:
  /// - `https://auth.dps.localhost/api`
  /// - `http://auth.dps.localhost:3000/api`
  pub fn get_backend_url(&self) -> String {
    let protocol = self.get_backend_protocol();
    let backend_sub = self.get_backend_subdomain();
    let domain = self.get_domain();
    let api_path = self.get_api_path();
    if let Some(port) = self.backend_port {
      format!("{protocol}://{backend_sub}.{domain}:{port}/{api_path}")
    } else {
      format!("{protocol}://{backend_sub}.{domain}/{api_path}")
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
    assert_eq!(config.get_project_name(), "My Project");
    assert_eq!(config.get_domain(), "dps.localhost");
    assert_eq!(config.get_api_path(), "api");
    assert!(!config.get_development_mode());
    assert_eq!(config.get_backend_subdomain(), "auth");
    assert_eq!(config.get_backend_protocol(), "https");
    assert!(!config.get_backend_insecure_cookie());
    assert_eq!(
      config.get_backend_sqlite_main_file_path(),
      "data/main-development.db"
    );
    assert_eq!(config.get_backend_sqlite_main_pool_size(), 1);
    assert_eq!(
      config.get_backend_sqlite_collection_file_path(),
      "data/collection-development.db"
    );
    assert_eq!(config.get_backend_sqlite_collection_pool_size(), 1);
    assert_eq!(
      config.get_backend_sqlite_session_file_path(),
      "data/session-development.db"
    );
    assert_eq!(config.get_backend_sqlite_session_pool_size(), 1);
    assert!(config.get_backend_port().is_none());
    assert!(config.get_backend_session_secret().is_none());
    assert!(config.get_backend_session_secret_bytes().is_none());
  }

  #[test]
  fn test_setters() {
    let mut config = DpsConfig::new();
    config.set_project_name("Custom Project");
    config.set_domain("example.com");
    config.set_api_path("v1");
    config.set_development_mode(true);
    config.set_backend_port(Some(3000));
    config.set_backend_session_secret(Some("s3cr3t"));

    assert_eq!(config.get_project_name(), "Custom Project");
    assert_eq!(config.get_domain(), "example.com");
    assert_eq!(config.get_api_path(), "v1");
    assert!(config.get_development_mode());
    assert_eq!(config.get_backend_port(), Some(3000));
    assert_eq!(
      config.get_backend_session_secret(),
      Some("s3cr3t".to_string())
    );
  }

  #[test]
  fn test_backend_url_without_port() {
    let mut config = DpsConfig::new();
    config.set_backend_protocol("https");
    config.set_backend_subdomain("auth");
    config.set_domain("dps.localhost");
    assert_eq!(config.get_backend_url(), "https://auth.dps.localhost/api");
  }

  #[test]
  fn test_backend_url_with_port() {
    let mut config = DpsConfig::new();
    config.set_backend_protocol("http");
    config.set_backend_port(Some(3000));
    config.set_backend_subdomain("auth");
    config.set_domain("dps.localhost");
    assert_eq!(
      config.get_backend_url(),
      "http://auth.dps.localhost:3000/api"
    );
  }

  #[test]
  fn test_readme_example() {
    let mut config = DpsConfig::new();
    config.set_domain("test.local");
    config.set_backend_protocol("http");
    config.set_backend_port(Some(8080));
    assert_eq!(config.get_backend_url(), "http://auth.test.local:8080/api");
  }

  #[test]
  #[serial]
  fn test_backend_insecure_cookie() {
    // Test setter first (without env var interference)
    let mut c = DpsConfig::new();
    assert!(!c.get_backend_insecure_cookie());
    c.set_backend_insecure_cookie(true);
    assert!(c.get_backend_insecure_cookie());
    c.set_backend_insecure_cookie(false);
    assert!(!c.get_backend_insecure_cookie());

    // Test env var loading
    std::env::set_var("DPS_BACKEND_INSECURE_COOKIE", "Y");
    let c2 = DpsConfig::new();
    assert!(c2.get_backend_insecure_cookie());
    std::env::remove_var("DPS_BACKEND_INSECURE_COOKIE");
  }

  #[test]
  #[serial]
  fn test_backend_sqlite_main_pool_size() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_backend_sqlite_main_pool_size(), 1);
    c.set_backend_sqlite_main_pool_size(Some(12));
    assert_eq!(c.get_backend_sqlite_main_pool_size(), 12);
    c.set_backend_sqlite_main_pool_size(None);
    assert_eq!(c.get_backend_sqlite_main_pool_size(), 1);

    // Test env var loading
    std::env::set_var("DPS_BACKEND_SQLITE_MAIN_POOL_SIZE", "8");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_backend_sqlite_main_pool_size(), 8);
    std::env::remove_var("DPS_BACKEND_SQLITE_MAIN_POOL_SIZE");
  }

  #[test]
  #[serial]
  fn test_backend_sqlite_main_file_path() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(
      c.get_backend_sqlite_main_file_path(),
      "data/main-development.db"
    );
    c.set_backend_sqlite_main_file_path("data/custom.db");
    assert_eq!(c.get_backend_sqlite_main_file_path(), "data/custom.db");

    // Test env var loading
    std::env::set_var("DPS_BACKEND_SQLITE_MAIN_FILE_PATH", "data/test-main.db");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_backend_sqlite_main_file_path(), "data/test-main.db");
    std::env::remove_var("DPS_BACKEND_SQLITE_MAIN_FILE_PATH");
  }

  #[test]
  #[serial]
  fn test_backend_sqlite_collection_pool_size() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_backend_sqlite_collection_pool_size(), 1);
    c.set_backend_sqlite_collection_pool_size(Some(12));
    assert_eq!(c.get_backend_sqlite_collection_pool_size(), 12);
    c.set_backend_sqlite_collection_pool_size(None);
    assert_eq!(c.get_backend_sqlite_collection_pool_size(), 1);

    // Test env var loading
    std::env::set_var("DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE", "8");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_backend_sqlite_collection_pool_size(), 8);
    std::env::remove_var("DPS_BACKEND_SQLITE_COLLECTION_POOL_SIZE");
  }

  #[test]
  #[serial]
  fn test_backend_sqlite_collection_file_path() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(
      c.get_backend_sqlite_collection_file_path(),
      "data/collection-development.db"
    );
    c.set_backend_sqlite_collection_file_path("data/custom-collection.db");
    assert_eq!(
      c.get_backend_sqlite_collection_file_path(),
      "data/custom-collection.db"
    );

    // Test env var loading
    std::env::set_var(
      "DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH",
      "data/test-collection.db",
    );
    let c2 = DpsConfig::new();
    assert_eq!(
      c2.get_backend_sqlite_collection_file_path(),
      "data/test-collection.db"
    );
    std::env::remove_var("DPS_BACKEND_SQLITE_COLLECTION_FILE_PATH");
  }

  #[test]
  #[serial]
  fn test_backend_sqlite_session_pool_size() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_backend_sqlite_session_pool_size(), 1);
    c.set_backend_sqlite_session_pool_size(Some(12));
    assert_eq!(c.get_backend_sqlite_session_pool_size(), 12);
    c.set_backend_sqlite_session_pool_size(None);
    assert_eq!(c.get_backend_sqlite_session_pool_size(), 1);

    // Test env var loading
    std::env::set_var("DPS_BACKEND_SQLITE_SESSION_POOL_SIZE", "8");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_backend_sqlite_session_pool_size(), 8);
    std::env::remove_var("DPS_BACKEND_SQLITE_SESSION_POOL_SIZE");
  }

  #[test]
  #[serial]
  fn test_backend_sqlite_session_file_path() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(
      c.get_backend_sqlite_session_file_path(),
      "data/session-development.db"
    );
    c.set_backend_sqlite_session_file_path("data/custom-session.db");
    assert_eq!(
      c.get_backend_sqlite_session_file_path(),
      "data/custom-session.db"
    );

    // Test env var loading
    std::env::set_var(
      "DPS_BACKEND_SQLITE_SESSION_FILE_PATH",
      "data/test-session.db",
    );
    let c2 = DpsConfig::new();
    assert_eq!(
      c2.get_backend_sqlite_session_file_path(),
      "data/test-session.db"
    );
    std::env::remove_var("DPS_BACKEND_SQLITE_SESSION_FILE_PATH");
  }

  #[test]
  fn test_backend_session_secret_bytes() {
    let mut config = DpsConfig::new();
    config.set_backend_session_secret(Some("my-secret-key"));

    let secret_bytes = config.get_backend_session_secret_bytes();
    assert!(secret_bytes.is_some());
    assert_eq!(secret_bytes.unwrap(), b"my-secret-key".to_vec());
  }

  #[test]
  fn test_backend_session_secret_bytes_none() {
    let config = DpsConfig::new();
    assert!(config.get_backend_session_secret_bytes().is_none());
  }

  #[test]
  #[serial]
  fn test_backend_session_ttl_seconds() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_backend_session_ttl_seconds(), 1209600); // 14 days
    c.set_backend_session_ttl_seconds(Some(3600));
    assert_eq!(c.get_backend_session_ttl_seconds(), 3600);

    // Test env var loading
    std::env::set_var("DPS_BACKEND_SESSION_TTL_SECONDS", "1800");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_backend_session_ttl_seconds(), 1800);
    std::env::remove_var("DPS_BACKEND_SESSION_TTL_SECONDS");
  }

  #[test]
  #[serial]
  fn test_project_name() {
    // Test default and setter
    let mut c = DpsConfig::new();
    assert_eq!(c.get_project_name(), "My Project");
    c.set_project_name("New Name");
    assert_eq!(c.get_project_name(), "New Name");

    // Test env var loading
    std::env::set_var("DPS_PROJECT_NAME", "Env Project");
    let c2 = DpsConfig::new();
    assert_eq!(c2.get_project_name(), "Env Project");
    std::env::remove_var("DPS_PROJECT_NAME");
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

  #[test]
  fn test_session_sub_to_user_id_fn_default() {
    let config = DpsConfig::new();
    let to_user_id = config.get_session_sub_to_user_id_fn();
    assert_eq!(to_user_id("12345").unwrap(), 12345);
    assert!(to_user_id("invalid").is_err());
  }

  #[test]
  fn test_session_sub_to_user_id_fn_custom() {
    let mut config = DpsConfig::new();
    config.set_session_sub_to_user_id_fn(|sub| Ok(sub.len() as i64));
    assert_eq!(config.get_session_sub_to_user_id_fn()("hello").unwrap(), 5);
  }

  #[test]
  fn test_session_user_to_sub_fn_default() {
    let config = DpsConfig::new();
    let to_sub = config.get_session_user_to_sub_fn();
    let record = serde_json::json!({ "id": 42, "name": "test" });
    assert_eq!(to_sub(&record).unwrap(), "42");
    assert!(to_sub(&serde_json::json!({})).is_err());
  }

  #[test]
  fn test_session_user_to_sub_fn_custom() {
    let mut config = DpsConfig::new();
    config.set_session_user_to_sub_fn(|record| {
      record
        .get("sub")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("missing 'sub'"))
    });
    let record = serde_json::json!({ "sub": "user-123", "name": "test" });
    assert_eq!(
      config.get_session_user_to_sub_fn()(&record).unwrap(),
      "user-123"
    );
  }
}
