# Implementation Plan - Add Project Name Configuration

Add a new configuration "project name" to `DpsConfig` in both Rust and TypeScript implementations.

## Proposed Changes

### Rust Implementation (`src/lib.rs`)

1. **Update `DpsConfig` struct**:
   ```rust
   pub struct DpsConfig {
     // Global properties
     project_name: Option<String>,
     domain: Option<String>,
     // ...
   }
   ```

2. **Update `DpsConfig::new()`**:
   ```rust
   pub fn new() -> Self {
     Self {
       project_name: load_env_string("DPS_PROJECT_NAME"),
       domain: load_env_string("DPS_DOMAIN"),
       // ...
     }
   }
   ```

3. **Add getter and setter**:
   ```rust
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
   ```

4. **Update tests**:
   - Update `test_default_values` and `test_setters`.
   - Add `test_project_name` to verify `DPS_PROJECT_NAME` env var.

### TypeScript Implementation (`src/index.ts`)

1. **Update `DpsConfig` class properties**:
   ```typescript
   export class DpsConfig {
     private projectName?: string;
     private domain?: string;
     // ...
   }
   ```

2. **Update constructor**:
   ```typescript
   constructor(envPrefix: string = "") {
     this.projectName = this.loadEnvString(envPrefix, "DPS_PROJECT_NAME");
     this.domain = this.loadEnvString(envPrefix, "DPS_DOMAIN");
     // ...
   }
   ```

3. **Add getter and setter**:
   ```typescript
   getProjectName(): string {
     return this.projectName ?? "My Project";
   }

   setProjectName(value: string) {
     this.projectName = value;
   }
   ```

### Tests (`src/index.test.ts`)

1. **Update existing tests**:
   - Update "should have correct default values" to check `getProjectName()`.
   - Update "should work with setters" to check `setProjectName()`.

2. **Add new test**:
   - Add test case for loading `DPS_PROJECT_NAME` from environment variables.

## Verification Plan

1. Run `cargo test` for Rust implementation.
2. Run `bun test` for TypeScript implementation.
3. Run `cargo clippy --allow-dirty --fix && cargo fmt` for linting and formatting.
