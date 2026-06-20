# codex
Character Sheet builder, inspired by PCGEN, built on Rust, able to ingest pcgen lst files.

## PCGen import bridge

The `codex` crate hosts the PCGen import bridge under `src/pcgen_import/`.
Parsing and semantic conversion are deliberately separate stages. The first
implemented slice (GE03-E1-F1) is the PCC entry-file parser in
`src/pcgen_import/pcc.rs`, which turns a PCC entry file into a structured result
that preserves source identity, `PCC:` include edges, one-based line numbers,
raw directive evidence, and diagnostics for malformed include lines.

## Build and test

```bash
. "$HOME/.cargo/env"
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```
