# Repository Guidelines

## Project Structure

```
src/
├── bin/mirror_log.rs      # CLI entry point (clap commands)
├── lib.rs                 # Public library API
├── db.rs                  # SQLite connection and migrations
├── log.rs                 # Core append/query logic
├── pipeline.rs            # Ingestion pipeline (capture → persist → structure → enrich)
├── chunk.rs               # Content chunking
├── schema.sql             # Database schema
├── attention/             # Attention layer (feature-gated)
├── embedding.rs           # Embedding support (`embedding` feature)
├── inference/             # HTTP inference backend (`inference` feature)
├── iteration/             # Iteration queries and types (`iteration` feature)
├── sources/               # Input sources: clipboard, CLI history
└── view/                  # Output formatting
tests/
├── integration_test.rs    # End-to-end CLI and library tests
├── edge_cases.rs          # Edge case module root
└── edge_cases/            # Subtests: database.rs, unicode.rs
docs/                      # USER_GUIDE.md, CANONICAL_PIPELINE.md
scripts/                   # dep-audit.sh, log-dep-audit.sh, log-dep-audit-matrix.sh
```

The default build is lean. Optional capabilities (`embedding`, `inference`, `iteration`, `clipboard`) are gated behind Cargo features and must not bleed into the core path.

## Build, Test, and Development Commands

```bash
# Check compilation
cargo check

# Format code
cargo fmt

# Lint (warnings are errors)
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test

# Build optimized release binary
cargo build --release
# Binary: target/release/mirror-log

# Build with optional embedding support
cargo build --release --features embedding

# Install locally
cargo install --path .
```

## Coding Style & Naming Conventions

- **Edition**: Rust 2021.
- **Formatting**: enforced by `cargo fmt` — run it before every commit.
- **Linting**: `cargo clippy --all-targets --all-features -- -D warnings` must pass clean.
- **Error handling**: use `thiserror`-derived types for library errors; avoid `unwrap` outside tests.
- **Naming**: follow standard Rust conventions — `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- **Feature gates**: wrap optional code in `#[cfg(feature = "...")]`; keep unconditional imports out of feature-only modules.

## Testing Guidelines

- Tests use the built-in `#[test]` framework — no external test runner.
- **Integration tests** live in `tests/integration_test.rs` and exercise the full CLI/library surface.
- **Edge case tests** live in `tests/edge_cases/` (database behaviour, unicode handling).
- Test fixtures are in `tests/fixtures/`.
- Name test functions descriptively: `test_add_event_deduplicates_by_hash`, not `test1`.
- Run the full suite with `cargo test`; run a specific test with `cargo test <test_name>`.
- There is no enforced coverage threshold, but new behaviour should have a corresponding test.

## Commit & Pull Request Guidelines

- Write commit messages in the imperative mood, sentence-cased:
  - ✅ `Add attention layer implementation`
  - ✅ `Fix null pointer dereference in event handler`
  - ✅ `Remove unused imports and simplify attention module`
  - ❌ `added stuff`, `fixes`, `wip`
- Version bump commits follow the pattern `Prepare vX.Y.Z for release` or `vX.Y.Z`.
- Keep commits focused — one logical change per commit.
- Pull requests should include a short description of what changed and why. Link any relevant issues.
- CI must pass (`cargo fmt --check`, `cargo clippy`, `cargo test`) before merging.

## Security & Configuration

- See `SECURITY.md` for the vulnerability reporting policy.
- `mirror.db` is the local SQLite database — never commit it.
- `.backup` files are gitignored; keep it that way.
- Secrets and API keys must never be hardcoded; use environment variables for any inference backend configuration.

## Dependency Auditing

Run the audit scripts periodically and before releases to track the dependency surface:

```bash
scripts/dep-audit.sh                        # default feature set
scripts/dep-audit.sh --features embedding   # optional feature path
scripts/log-dep-audit-matrix.sh             # log both surfaces to mirror.db
```
