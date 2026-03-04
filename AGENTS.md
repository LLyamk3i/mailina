# Mailina Agent Guide

## Project Context
Mailina is a Rust CLI application designed with a strict architectural philosophy focused on **Affordances** and **Stateless Namespaces**. The codebase is currently in its initial scaffolding phase.

## Architectural Principles

### 1. Affordances over Abilities
- **Domain Nouns** (Structs) own their data and behavior.
- Do **not** create "Manager", "Service", or "Handler" structs to manage state.
- Methods should reflect what the object *can do* (affordances).
  - ✅ `settings.save()`
  - ✅ `settings.show()`
  - ❌ `config_manager.save_settings(settings)`

### 2. Stateless Namespaces
- Logic that doesn't own state belongs in **Stateless Namespaces** (modules with pure functions).
- Use Rust modules to group logic, not classes/structs.
- CLI parsing should be a lexical router using pattern matching.
  - ✅ `src/app/cli.rs` -> `pub fn configure(args: &[String])`
  - ❌ `struct CliParser { ... }`

### 3. The "One Word" Rule
- **Strictly** use single English words for variable names, properties, and arguments.
- Compress concepts into their essence.
  - ✅ `interval` vs ❌ `interval_seconds`
  - ✅ `limit` vs ❌ `fetch_limit`
  - ✅ `routes` vs ❌ `active_routes`
  - ✅ `tick` vs ❌ `parsed_interval`
  - ✅ `verb` vs ❌ `command_string`

## Code Organization (Planned)

```text
src/
├── domain/          # Domain Nouns (Structs + Affordances)
│   └── settings.rs  # Settings struct with load/save/show methods
├── app/             # Stateless Namespaces (Logic/Routing)
│   └── cli.rs       # CLI argument parsing and routing
├── io/              # I/O Interfaces
│   └── disk.rs      # File system interaction
└── main.rs          # Entry point
```

## Implementation Patterns

### CLI Parsing
Use slice pattern matching for argument routing:
```rust
match args {
    [verb, value] if verb == "interval" => { ... }
    [] => { ... }
    _ => { ... }
}
```

### Persistence
- Use `serde` and `serde_json` for serialization.
- `Settings::load()` and `Settings::save()` handle disk I/O directly (via `io::disk`).

## Commands
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`
