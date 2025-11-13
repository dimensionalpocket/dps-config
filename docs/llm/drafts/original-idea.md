This repo holds the DpsConfig struct. It's a struct used by other crates to hold configuration values of apps.

It's part of the DPS ecosystem, which encompasses multiple components and systems in different languages:

- DpsAuthApi, DpsLogsApi, DpgMetricsApi, etc: Rust graphQL APIs
- DpsAuthWeb, DpsMonitorWeb, etc: Vue frontend apps

The DpsConfig struct has setters and getters holding configuration values for those components made in Rust. It has the following expectations:

- No validation. All values are optional. Validation will be done by the crate that requires it.
- Most properties have default values that are hardcoded inside getters.
- The are extra computed getters that do not have associated setters. E.g., `get_api_url()` is a computed getter that returns a combination of several other properties (protocol, subdomain, domain, port, etc).

Each non-computed property has its own attribute, a getter, and a setter.
- For instance, the `api_subdomain` property has an `api_subdomain` attribute, a `get_api_subdomain()` method, a `set_api_subdomain()` method, and an accompanying environment variable to auto-load from (`DPS_API_SUBDOMAIN`).

Environment variables that refer to boolean configs will hold a `"Y"` string for truthy values.

# Config List

Global configs (env variables in brackets):

- `domain` (`DPS_DOMAIN`): the main domain of the website. Defaults to `dps.localhost`.
- `api_subdomain` (`DPS_API_SUBDOMAIN`): the subdomain part of all APIs. Defaults to `api`.
- `development_mode` (`DPS_DEVELOPMENT_MODE`): enables development-only features in multiple services.

DpsAuthApi-specific configs:

- `auth_api_subdomain` (`DPS_AUTH_API_SUBDOMAIN`): the sub-subdomain of the DpsAuthApi service. Defaults to `auth`.
- `auth_api_port` (`DPS_AUTH_API_PORT`): the port of the DpsAuthApi service. No default value, which does not render a port number in the URL. Should be set in development (suggestion: `3000`).
- `auth_api_protocol` (`DPS_AUTH_API_PROTOCOL`): the protocol for DpsAuthApi service. Defaults to `https`. Should be set to `http` in local development.
- `auth_api_sqlite_file_path` (`DPS_AUTH_API_SQLITE_FILE_PATH`): SQLite database file path. Defaults to `"data/development.db"`.
- `auth_api_session_secret` (`DPS_AUTH_API_SESSION_SECRET`): 32-byte session secret for encryption. No default value.

List of computed getters:

- `get_api_domain()`: returns the global API subdomain without protocol and port, a combination of `api_subdomain` + `domain`.
  - Example: `api.dps.localhost`.
  - This is used as the cookies domain setting so that all APIs can have access to cookies.
- `get_auth_api_url()`: returns a combination of `auth_api_subdomain` + `api_subdomain` + `domain` + `auth_api_port` (if available).
  - Example: `http://auth.api.dps.localhost:3000`
