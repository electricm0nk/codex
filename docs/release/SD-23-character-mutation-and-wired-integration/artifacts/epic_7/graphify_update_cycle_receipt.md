# Criterion 27 — Graphify update cycle (cycle 13)

`update_graphify.py` run against `origin/develop` (integration target), bundle SD-23, receipts at `docs/release/SD-23-character-mutation-and-wired-integration/receipts.md`.

## Result: failed (non-blocking, per doctrine)

```
[graphify-update] graphify CLI: /home/ubuntu/.local/bin/graphify
[graphify-update] invocation: /home/ubuntu/.local/bin/graphify cluster-only /home/ubuntu/workspace/repos/codex --budget 500000 --exclude node_modules,target,dist,build,.git,out,dist-ssr,.next,coverage
[graphify-update] graphify exit=1, elapsed=0.5s, outcome=failed
[graphify-update] log written: graphify-out/.truth-up-run-2026-07-21T03-38-33Z.log
[graphify-update] appended receipt to receipts.md
[graphify-update] graphify update complete (with failure receipt; pipeline continues per operator directive)
```

Captured log (`graphify-out/.truth-up-run-2026-07-21T03-38-33Z.log`):
```
error: no graph found at /home/ubuntu/workspace/repos/codex/graphify-out/graph.json — run /graphify first
```

## Root cause

`graphify cluster-only` requires a pre-existing `graph.json` at `graphify-out/graph.json` to cluster. This repo has never had one built. The error message ("run /graphify first") and `graphify --help`'s subcommand list (no CLI subcommand that builds a fresh graph from scratch — `path`/`explain`/`diagnose`/`merge-graphs` all read an existing `graph.json`) indicate the base graph is normally bootstrapped via an interactive `/graphify` slash-command session (LLM-assisted extraction), not a CLI invocation this script or an automated cycle can trigger.

## Disposition

Per the graphify-update skill and Criterion 27's own acceptance text: **graphify non-zero exit does NOT refuse the closure pipeline.** The failure receipt (`row_or_kind: graphify:update`, `outcome: failed`, `graphify_exit_code: 1`) is the audit trail; this cycle correctly recorded it and moved on rather than attempting an out-of-scope fix (bootstrapping a fresh graph is a heavier, interactive, LLM-assisted operation this automated cycle isn't equipped or authorized to perform). Recorded as an open item for the operator: bootstrapping `graphify-out/graph.json` once (via an interactive `/graphify` session) would let future `cluster-only` runs succeed.

Commit SHA (receipt + log): `a5b22d4`.
