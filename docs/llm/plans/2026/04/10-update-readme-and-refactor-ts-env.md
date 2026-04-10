# Plan: Update README and Refactor TS Env Loading

Previously, "project name" was added to `DpsConfig` but not documented in `README.md`. Also, the TypeScript implementation needs to stop using `process.env` directly to support Vite apps.

## User Requirements
- Update `README.md` with `project_name`.
- Change `DpsConfig` constructor in `src/index.ts` to `constructor(env: Record<string, string | undefined>, envPrefix: string = "")`.
- `env` argument is required and has no default.
- Store `env` in the instance (preferably a filtered subset for security).
- Update `README.md` with the new constructor usage.
- Discuss security: filtering `env` to only used keys.

## Proposed Changes

### 1. Update `README.md`
- Add `project_name` to the Global configuration table.
- Update Bun/TypeScript examples to show `new DpsConfig(process.env)` and `new DpsConfig(import.meta.env, 'VITE_')`.

### 2. Modify `src/index.ts`
- Define a constant or static property with all supported environment variable keys (without prefix).
- Update the constructor to accept `env`.
- Filter `env` to only include keys we care about.
- Update `loadEnvString`, `loadEnvBool`, and `loadEnvNumber` to use the internal filtered env.

### 3. Update `src/index.test.ts`
- Pass an environment object to all `new DpsConfig()` calls.
- Update tests to verify that it uses the passed object and not `process.env`.

## Security Considerations
By filtering the passed `env` object to only known keys (e.g., `DPS_DOMAIN`, `DPS_PROJECT_NAME`, etc.), we avoid storing sensitive unrelated environment variables (like `DATABASE_URL` or `SECRET_KEY` if they aren't part of `DpsConfig`) inside the config instance. This reduces the risk if the `DpsConfig` instance is ever serialized or inspected in a debugger.

## Plan

1. *Update `README.md` with `project_name` and new TypeScript constructor signature.*
   - Add `project_name` to the table.
   - Update examples.
2. *Modify `src/index.ts` to implement the new constructor and filtered env storage.*
   - Implement filtering logic.
   - Update helper methods.
3. *Update `src/index.test.ts` to reflect the changes.*
   - Fix all constructor calls.
   - Add a test case for the filtering logic.
4. *Complete pre-commit steps*
   - Ensure proper testing, verification, review, and reflection are done.
5. *Submit the changes.*
