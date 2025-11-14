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
//! config.set_api_subdomain("api");
//! assert_eq!(config.get_api_domain(), "api.example.com");
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
  api_subdomain: Option<String>,
  development_mode: Option<bool>,

  // DpsAuthApi properties
  auth_api_subdomain: Option<String>,
  auth_api_port: Option<u16>,
  auth_api_protocol: Option<String>,
  auth_api_sqlite_file_path: Option<String>,
  auth_api_session_secret: Option<String>,
  auth_api_session_ttl_seconds: Option<u64>,
}

impl DpsConfig {
  /// Create a new `DpsConfig` instance, loading values from environment
  /// variables when present.
  ///
  /// Environment variables:
  /// - `DPS_DOMAIN`
  /// - `DPS_API_SUBDOMAIN`
  /// - `DPS_DEVELOPMENT_MODE` (use `"Y"` for true)
  /// - `DPS_AUTH_API_SUBDOMAIN`
  /// - `DPS_AUTH_API_PORT`
  /// - `DPS_AUTH_API_PROTOCOL`
  /// - `DPS_AUTH_API_SQLITE_FILE_PATH`
  /// - `DPS_AUTH_API_SESSION_SECRET`
  pub fn new() -> Self {
    Self {
      domain: load_env_string("DPS_DOMAIN"),
      api_subdomain: load_env_string("DPS_API_SUBDOMAIN"),
      development_mode: load_env_bool("DPS_DEVELOPMENT_MODE"),
      auth_api_subdomain: load_env_string("DPS_AUTH_API_SUBDOMAIN"),
      auth_api_port: load_env_u16("DPS_AUTH_API_PORT"),
      auth_api_protocol: load_env_string("DPS_AUTH_API_PROTOCOL"),
      auth_api_sqlite_file_path: load_env_string("DPS_AUTH_API_SQLITE_FILE_PATH"),
      auth_api_session_secret: load_env_string("DPS_AUTH_API_SESSION_SECRET"),
      auth_api_session_ttl_seconds: load_env_u64("DPS_AUTH_API_SESSION_TTL_SECONDS"),
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

  /// Returns the API subdomain or the default `"api"`.
  ///
  /// Env var: `DPS_API_SUBDOMAIN`
  pub fn get_api_subdomain(&self) -> String {
    self
      .api_subdomain
      .clone()
      .unwrap_or_else(|| "api".to_string())
  }

  /// Set the API subdomain (overrides env).
  pub fn set_api_subdomain(&mut self, value: &str) {
    self.api_subdomain = Some(value.to_string());
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

  /// Returns the SQLite file path for the Auth API or default
  /// `"data/development.db"`.
  ///
  /// Env var: `DPS_AUTH_API_SQLITE_FILE_PATH`
  pub fn get_auth_api_sqlite_file_path(&self) -> String {
    self
      .auth_api_sqlite_file_path
      .clone()
      .unwrap_or_else(|| "data/development.db".to_string())
  }

  /// Set the SQLite file path for Auth API.
  pub fn set_auth_api_sqlite_file_path(&mut self, value: &str) {
    self.auth_api_sqlite_file_path = Some(value.to_string());
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
  pub fn get_auth_api_session_ttl_seconds(&self) -> u64 {
    self.auth_api_session_ttl_seconds.unwrap_or(1209600)
  }

  /// Set or unset the auth session TTL in seconds.
  pub fn set_auth_api_session_ttl_seconds(&mut self, value: Option<u64>) {
    self.auth_api_session_ttl_seconds = value;
  }

  // --------------------
  // Computed getters
  // --------------------

  /// Returns the computed API domain: `{api_subdomain}.{domain}`.
  ///
  /// Example: `api.dps.localhost`
  pub fn get_api_domain(&self) -> String {
    format!("{}.{}", self.get_api_subdomain(), self.get_domain())
  }

  /// Returns the full Auth API URL, including protocol and optional port.
  ///
  /// Examples:
  /// - `https://auth.api.dps.localhost`
  /// - `http://auth.api.dps.localhost:3000`
  pub fn get_auth_api_url(&self) -> String {
    let protocol = self.get_auth_api_protocol();
    let auth_sub = self.get_auth_api_subdomain();
    let api_domain = self.get_api_domain();
    if let Some(port) = self.auth_api_port {
      format!("{protocol}://{auth_sub}.{api_domain}:{port}")
    } else {
      format!("{protocol}://{auth_sub}.{api_domain}")
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

fn load_env_u64(key: &str) -> Option<u64> {
  env::var(key).ok().and_then(|v| v.parse::<u64>().ok())
}

// --------------------
// Tests
// --------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_values() {
    let config = DpsConfig::new();
    assert_eq!(config.get_domain(), "dps.localhost");
    assert_eq!(config.get_api_subdomain(), "api");
    assert!(!config.get_development_mode());
    assert_eq!(config.get_auth_api_subdomain(), "auth");
    assert_eq!(config.get_auth_api_protocol(), "https");
    assert_eq!(
      config.get_auth_api_sqlite_file_path(),
      "data/development.db"
    );
    assert!(config.get_auth_api_port().is_none());
    assert!(config.get_auth_api_session_secret().is_none());
    assert!(config.get_auth_api_session_secret_bytes().is_none());
  }

  #[test]
  fn test_setters() {
    let mut config = DpsConfig::new();
    config.set_domain("example.com");
    config.set_development_mode(true);
    config.set_auth_api_port(Some(3000));
    config.set_auth_api_session_secret(Some("s3cr3t"));

    assert_eq!(config.get_domain(), "example.com");
    assert!(config.get_development_mode());
    assert_eq!(config.get_auth_api_port(), Some(3000));
    assert_eq!(
      config.get_auth_api_session_secret(),
      Some("s3cr3t".to_string())
    );
  }

  #[test]
  fn test_api_domain_computed() {
    let mut config = DpsConfig::new();
    config.set_api_subdomain("api");
    config.set_domain("dps.localhost");
    assert_eq!(config.get_api_domain(), "api.dps.localhost");
  }

  #[test]
  fn test_auth_api_url_without_port() {
    let mut config = DpsConfig::new();
    config.set_auth_api_protocol("https");
    config.set_auth_api_subdomain("auth");
    config.set_api_subdomain("api");
    config.set_domain("dps.localhost");
    assert_eq!(config.get_auth_api_url(), "https://auth.api.dps.localhost");
  }

  #[test]
  fn test_auth_api_url_with_port() {
    let mut config = DpsConfig::new();
    config.set_auth_api_protocol("http");
    config.set_auth_api_port(Some(3000));
    config.set_auth_api_subdomain("auth");
    config.set_api_subdomain("api");
    config.set_domain("dps.localhost");
    assert_eq!(
      config.get_auth_api_url(),
      "http://auth.api.dps.localhost:3000"
    );
  }

  #[test]
  fn test_readme_example() {
    let mut config = DpsConfig::new();
    config.set_domain("test.local");
    config.set_auth_api_protocol("http");
    config.set_auth_api_port(Some(8080));
    assert_eq!(config.get_auth_api_url(), "http://auth.api.test.local:8080");
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
  fn test_auth_api_session_ttl_seconds_default_and_set() {
    let mut c = DpsConfig::new();
    // default = 14 days in seconds
    assert_eq!(c.get_auth_api_session_ttl_seconds(), 1209600);
    c.set_auth_api_session_ttl_seconds(Some(3600));
    assert_eq!(c.get_auth_api_session_ttl_seconds(), 3600);
  }

  #[test]
  fn test_auth_api_session_ttl_seconds_from_env() {
    std::env::set_var("DPS_AUTH_API_SESSION_TTL_SECONDS", "1800");
    let c = DpsConfig::new();
    assert_eq!(c.get_auth_api_session_ttl_seconds(), 1800);
    std::env::remove_var("DPS_AUTH_API_SESSION_TTL_SECONDS");
  }
}
