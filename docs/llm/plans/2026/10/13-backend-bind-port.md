# Add `backend_bind_port` (internal bind port, separate from public `backend_port`)

Date: 2026-10-13

## Overview

`DpsConfig` currently exposes a single port concept: `backend_port` (`DPS_BACKEND_PORT`), which is the
**public** port appended to `get_backend_url()`. There is no way to express the **internal** port a
server process should bind.

These are different concerns:

- **Public port** (`DPS_BACKEND_PORT`): appears in generated URLs (`https://auth.domain:PORT/api`). In a
  proxied deployment it is usually unset, because the proxy terminates `443` and forwards internally.
- **Bind / internal port** (`PORT`, injected by the platform, e.g. Railway `PORT=8000`): the socket the
  process listens on. It must **never** appear in generated URLs (clients connect without a port).

Conflating them causes bugs: binding on `DPS_BACKEND_PORT` breaks on platforms that inject `PORT`, and
using `PORT` in URLs leaks an internal port to clients. This plan adds a distinct bind-port property to
both the Rust and TypeScript implementations, without changing `backend_port` or `get_backend_url()`.

## Locked decisions

1. **Two independent properties.** Keep `backend_port` (`DPS_BACKEND_PORT`) exactly as-is: public,
   `Option<u16>`, feeds `get_backend_url()`. Add `backend_bind_port`, which feeds server binding only.
2. **Bind-port resolution chain:** `DPS_BACKEND_BIND_PORT` (explicit override) -> `PORT` (generic
   platform/runtime) -> `3000`. Resolved getter returns `u16` (never `Option`), matching siblings like
   `get_backend_sqlite_main_pool_size()`.
3. **`DPS_BACKEND_BIND_PORT` exists for deterministic local overrides** without being forced to set the
   generic `PORT`. `PORT` is the fallback, not a required input.
4. **`get_backend_url()` is unchanged.** It continues to use `backend_port` (the public port) and omits
   the port when unset.
5. **No rename.** `get_backend_port()` / `set_backend_port()` keep their names (renaming to
   `*_public_port` is a possible future cleanup, not part of this plan). The new getter's name makes the
   internal role explicit.
6. **Both implementations.** Rust (`src/lib.rs`) and TypeScript (`src/index.ts`), with tests for both.

## Implementation Details

### Rust (`src/lib.rs`)

#### 1. Add the struct field

In the `DpsBackend properties` block, next to `backend_port`:

```rust
  // DpsBackend properties
  backend_subdomain: Option<String>,
  backend_port: Option<u16>,
  backend_bind_port: Option<u16>,
  backend_protocol: Option<String>,
```

#### 2. Load it in `DpsConfig::new()`

Resolve the fallback chain at construction (consistent with the other eager env loads). Add next to the
`backend_port` line:

```rust
      backend_port: load_env_u16("DPS_BACKEND_PORT"),
      backend_bind_port: load_env_u16("DPS_BACKEND_BIND_PORT").or_else(|| load_env_u16("PORT")),
```

Also add `DPS_BACKEND_BIND_PORT` to the env-var list in the `new()` doc comment (and mention `PORT`):

```rust
  /// - `DPS_BACKEND_SUBDOMAIN`
  /// - `DPS_BACKEND_PORT`
  /// - `DPS_BACKEND_BIND_PORT` (falls back to the generic `PORT`, then `3000`)
  /// - `DPS_BACKEND_PROTOCOL`
```

#### 3. Add the getter and setter

Place them right after `set_backend_port` in the `DpsBackend getters/setters` section:

```rust
  /// Returns the internal port a server should bind, resolving
  /// `DPS_BACKEND_BIND_PORT`, then the generic `PORT` environment variable, then
  /// the default `3000`.
  ///
  /// This is the port the process listens on and is intentionally separate from
  /// [`DpsConfig::get_backend_port`], which is the *public* port appended to
  /// [`DpsConfig::get_backend_url`] and is usually unset behind a proxy.
  ///
  /// Env vars: `DPS_BACKEND_BIND_PORT`, `PORT`
  pub fn get_backend_bind_port(&self) -> u16 {
    self.backend_bind_port.unwrap_or(3000)
  }

  /// Set or unset the internal bind port. Use `None` to reset to the default.
  pub fn set_backend_bind_port(&mut self, value: Option<u16>) {
    self.backend_bind_port = value;
  }
```

Do **not** touch `get_backend_port()` or `get_backend_url()`.

#### 4. Add Rust tests

In the `#[cfg(test)] mod tests` block (uses `serial_test::serial`):

```rust
  #[test]
  #[serial]
  fn test_backend_bind_port_default() {
    std::env::remove_var("DPS_BACKEND_BIND_PORT");
    std::env::remove_var("PORT");
    let c = DpsConfig::new();
    assert_eq!(c.get_backend_bind_port(), 3000);
  }

  #[test]
  #[serial]
  fn test_backend_bind_port_from_env() {
    std::env::set_var("DPS_BACKEND_BIND_PORT", "9000");
    let c = DpsConfig::new();
    assert_eq!(c.get_backend_bind_port(), 9000);
    std::env::remove_var("DPS_BACKEND_BIND_PORT");
  }

  #[test]
  #[serial]
  fn test_backend_bind_port_prefers_dps_over_port() {
    std::env::set_var("DPS_BACKEND_BIND_PORT", "9000");
    std::env::set_var("PORT", "8000");
    let c = DpsConfig::new();
    assert_eq!(c.get_backend_bind_port(), 9000);
    std::env::remove_var("DPS_BACKEND_BIND_PORT");
    std::env::remove_var("PORT");
  }

  #[test]
  #[serial]
  fn test_backend_bind_port_falls_back_to_port() {
    std::env::remove_var("DPS_BACKEND_BIND_PORT");
    std::env::set_var("PORT", "8000");
    let c = DpsConfig::new();
    assert_eq!(c.get_backend_bind_port(), 8000);
    std::env::remove_var("PORT");
  }

  #[test]
  #[serial]
  fn test_backend_bind_port_does_not_affect_url() {
    std::env::remove_var("DPS_BACKEND_BIND_PORT");
    std::env::set_var("PORT", "8000");
    let c = DpsConfig::new();
    assert_eq!(c.get_backend_url(), "https://auth.dps.localhost/api");
    std::env::remove_var("PORT");
  }
```

### TypeScript (`src/index.ts`)

The TS constructor eagerly loads values and filters the input env by `ENV_KEYS`, so bare `PORT` must be
added to `ENV_KEYS` to be captured.

#### 1. Add both env keys to `ENV_KEYS`

```typescript
    "DPS_BACKEND_SUBDOMAIN",
    "DPS_BACKEND_PORT",
    "DPS_BACKEND_BIND_PORT",
    "PORT",
    "DPS_BACKEND_PROTOCOL",
```

#### 2. Add the private field

Next to `backendPort`:

```typescript
  private backendSubdomain?: string;
  private backendPort?: number;
  private backendBindPort?: number;
  private backendProtocol?: string;
```

#### 3. Resolve it in the constructor

Next to the `backendPort` load:

```typescript
    this.backendPort = this.loadEnvNumber(envPrefix, "DPS_BACKEND_PORT");
    this.backendBindPort =
      this.loadEnvNumber(envPrefix, "DPS_BACKEND_BIND_PORT") ??
      this.loadEnvNumber(envPrefix, "PORT");
```

Note: with a non-empty `envPrefix` (e.g. `"VITE_"`), this reads `VITE_DPS_BACKEND_BIND_PORT` and
`VITE_PORT`. That is consistent with every other property; a server-side bind port is not a
frontend/Vite concern, so no special-casing is needed.

#### 4. Add the getter and setter

Right after `setBackendPort`:

```typescript
  /**
   * Returns the internal port a server should bind, resolving
   * `DPS_BACKEND_BIND_PORT`, then the generic `PORT` environment variable, then
   * the default `3000`. Distinct from `getBackendPort()` (public URL port).
   */
  getBackendBindPort(): number {
    return this.backendBindPort ?? 3000;
  }

  setBackendBindPort(value: number | undefined) {
    this.backendBindPort = value;
  }
```

Do **not** touch `getBackendPort()` or `getBackendUrl()`.

#### 5. Add TS tests (`src/index.test.ts`)

```typescript
  it("should default bind port to 3000", () => {
    const config = new DpsConfig({});
    expect(config.getBackendBindPort()).toBe(3000);
  });

  it("should load bind port from DPS_BACKEND_BIND_PORT", () => {
    const config = new DpsConfig({ DPS_BACKEND_BIND_PORT: "9000" });
    expect(config.getBackendBindPort()).toBe(9000);
  });

  it("should prefer DPS_BACKEND_BIND_PORT over PORT", () => {
    const config = new DpsConfig({ DPS_BACKEND_BIND_PORT: "9000", PORT: "8000" });
    expect(config.getBackendBindPort()).toBe(9000);
  });

  it("should fall back to PORT for the bind port", () => {
    const config = new DpsConfig({ PORT: "8000" });
    expect(config.getBackendBindPort()).toBe(8000);
  });

  it("should not include the bind port in the backend URL", () => {
    const config = new DpsConfig({ PORT: "8000" });
    expect(config.getBackendUrl()).toBe("https://auth.dps.localhost/api");
  });
```

## README.md updates

1. In the `### DpsBackend` property table, add a row after `backend_port`:

   ```markdown
   | `backend_bind_port` | `DPS_BACKEND_BIND_PORT` (falls back to `PORT`) | `3000` | Internal port the server binds (never used in URLs) |
   ```

2. Clarify the `backend_port` row description to make the separation explicit:

   ```markdown
   | `backend_port` | `DPS_BACKEND_PORT` | none | **Public** port appended to the backend URL (omitted if unset) |
   ```

3. Add a short note under the `### DpsBackend` table (or just above it):

   > `backend_port` is the **public** port used when building URLs. `backend_bind_port` is the
   > **internal** port the process listens on, resolved from `DPS_BACKEND_BIND_PORT`, then the generic
   > `PORT` environment variable (e.g. injected by a platform like Railway), then `3000`. The two are
   > independent: behind a proxy, `DPS_BACKEND_PORT` is typically unset while `PORT` is set.

4. In the Environment Variables example near the end of the README, add:

   ```bash
   export DPS_BACKEND_BIND_PORT="3000"
   # or, on a platform that injects it:
   export PORT="8000"
   ```

## Files to Modify

1. `src/lib.rs` — struct field, `new()` init, getter/setter, `new()` doc env list, tests
2. `src/index.ts` — `ENV_KEYS`, private field, constructor init, getter/setter
3. `src/index.test.ts` — new tests
4. `README.md` — property table row, `backend_port` clarification, separation note, env example

## Out of scope

- Renaming `backend_port` -> `backend_public_port` (possible future cleanup; would ripple to consumers).
- Changing `get_backend_url()`.
- Any change to how `dps-backend` (or other consumers) uses these getters. A follow-up `dps-backend`
  plan will replace its local `resolve_port` with `get_backend_bind_port()` for `DpsBackend::serve`.

## Order of Work

1. `src/lib.rs`: add field, load in `new()`, getter/setter, doc comment list.
2. `src/lib.rs`: add tests.
3. Run `cargo test --quiet`.
4. `src/index.ts`: add `ENV_KEYS` entries, field, constructor init, getter/setter.
5. `src/index.test.ts`: add tests.
6. Run `bun test`.
7. Update `README.md`.
8. Run `cargo clippy --allow-dirty --fix && cargo fmt`.
