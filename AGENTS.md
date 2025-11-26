# AGENTS.md

## Build/Lint/Test Commands

- **Build**: `cargo build`
- **Test all**: `cargo test --quiet`
- **Test single test**: `cargo test <test_name> --verbose`
- **Linter**: `cargo clippy --allow-dirty --fix && cargo fmt`

## Working Guidelines

### Before Starting Tasks
- Read the `README.md` to get project context and progress
- When needing current date/time, run `date +%Y-%m-%d@%H:%M` for YYYY-MM-DD@HH:MM format (24-hour, no AM/PM)
- Git operations (commits, branches, merges, rebases) are NOT part of agent tasks

### Working with Plans
- Save plans in `docs/llm/plans/YYYY/MM/` directory structure
- Name files as `DD-task-name.md` (day + dash + lowercase task with dashes)
- Include implementation details: files to modify/create, code samples for new functions
- Never include git operations, changelogs, PR descriptions, version bumps, or release notes in plans
- Never worry about backwards compatibility if the version is pre-1.0.0
- Do not code immediately after writing plans - wait for user review/approval

### Implementing Plans
- Only start implementation after explicit user command
- During implementation, STOP and inform user if deviating from the plan in any way
- Never implement anything not explicitly mentioned in the plan
- After implementation is fully finished and all tests pass, run the linter command and fix any issues

