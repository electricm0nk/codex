# Getting started

> Scope: toolchain setup, build/run/test commands for every crate and the frontend, running the
> desktop app, `verify.sh`, a first-contribution walkthrough done red-to-green, and the
> branch/commit/PR model this repo actually uses.
> Last verified: **2026-09-20 against `tranche/16` (`b22ea9e113`, SD-36 Epic D)**. New this pass
> (`docs/architecture/getting-started.md` did not exist before SD-36 Epic D). Commands verified by
> reading `scripts/verify.sh`'s own stage bodies (the authoritative source for exact invocations),
> `apps/desktop/package.json`'s `scripts` block, and `apps/desktop/.claude/skills/run-desktop/SKILL.md`.
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

This doc gets a competent developer who has never seen this repo to a landed first change. Read
[overview.md](./overview.md) first if you have not — it explains *why* the repo is shaped the way
the commands below assume.

## Prerequisites and toolchain

Grounded for Ubuntu 24.04-style Linux (the verified desktop path — see
[desktop-app.md](./desktop-app.md) if you're onboarding elsewhere and need to re-prove parity).

### 1. System packages (needed for the Tauri/GTK build)

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev pkg-config \
  xvfb xdotool imagemagick x11-utils lsof
```

The last five (`xvfb`, `xdotool`, `imagemagick`, `x11-utils`, `lsof`) are only needed if you'll
drive the desktop app headlessly (§"Running the desktop app" below); skip them if you'll only ever
run `npx tauri dev` from a real graphical session.

### 2. Rust, via rustup

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
. "$HOME/.cargo/env"
cargo --version
rustc --version
```

**The `~/.cargo/bin` shim gotcha.** `rustup` installs shims at `~/.cargo/bin/{cargo,rustc,...}`
that dispatch to your actual toolchain. If `~/.cargo/bin` is not the *first* Rust entry on `PATH`
(a system-packaged `cargo`, a stale venv, a container image that pre-seeded a different toolchain
earlier on `PATH`), you silently build with the wrong compiler version or lints diverge from
`clippy`'s `-D warnings` ceiling for no visible reason. Every long-running command in this repo's
own tooling exports `PATH="$HOME/.cargo/bin:$PATH"` defensively for exactly this reason — do the
same in your own shell profile or CI step rather than trusting an inherited `PATH`.

### 3. Node.js LTS

Install via your normal machine bootstrap path, then:

```bash
node --version   # verified with v22.x
npm --version    # verified with 10.9.x
```

### 4. The PCGen oracle checkout (optional — only for corpus-gated tests and ingest work)

A plain `cargo test --locked` at the repo root and in `apps/desktop/src-tauri` **does not need
this** — corpus-gated tests self-skip gracefully or hard-`#[ignore]` when it is absent (see
[testing.md](./testing.md) §"Corpus-gated tests"). You need it only to run `crates/codex-ingest`'s
generators/tests against real corpus bytes, or to re-run the oracle-parity harness.

```bash
scripts/fetch-pcgen-oracle.sh --check   # verify-only, no network — what verify.sh's preflight-oracle stage runs
scripts/fetch-pcgen-oracle.sh           # bootstrap/repair to the pinned commit if absent or off-pin
```

The pin lives at `scripts/pcgen-oracle-pin.env` (a specific upstream SHA, never a literal local
path) and is resolved at runtime through `$PCGEN_CORPUS_ROOT` (the `data/` dir) /
`$PCGEN_REPO_DIR` (the repo root) — **never write a hardcoded local checkout path into new docs or
scripts** (`AGENTS.md` §Concurrency and Measurement).

**The stale `CARGO_TARGET_DIR` gotcha.** If you (or a prior session in this checkout) exported a
custom `CARGO_TARGET_DIR` and it later got deleted, cleaned, or pointed at another tree, Cargo
reports confusing "couldn't create a temp dir" or link errors that look like a compiler bug —
they're disk/permission/staleness on that directory, not a Rust problem. `unset CARGO_TARGET_DIR`
before diagnosing anything else; if you deliberately use one, it must be one directory per agent
*per source tree* (never shared between a worktree and the main checkout — see `AGENTS.md`), and
you delete it when you finish.

## Build and test, per crate

There are **three** independent Rust build surfaces (two Cargo workspaces — see
[overview.md](./overview.md) §"Workspace and crate dependency graph") plus the frontend. **Avoid
running two `cargo` invocations at once in a shared checkout** — disk exhaustion from an
overlapping large build is this repo's second-largest recorded incident class
(`AGENTS.md` §Concurrency and Measurement), and `apps/desktop/src-tauri`'s own separate `target/`
means a desktop build and a root build share no cache benefit from running together anyway. If you
are dispatching more than one agent/process against the same checkout, give each its own
`CARGO_TARGET_DIR` (never shared between a worktree and the main checkout) rather than relying on
serialization alone.

### Root crate (`codex`)

```bash
cargo build                       # compiles the codex package only (no default-members widens this)
cargo test --locked --lib         # unit tests inside src/ — fast
cargo test --locked               # + every tests/*.rs integration file — the full root suite
cargo clippy --locked --tests -- -D warnings   # lints, including test targets, zero-warning ceiling
```

### Ingest crate (`crates/codex-ingest`)

Not built by a bare `cargo build`/`cargo test` at the repo root (no `default-members` — see
[overview.md](./overview.md)); build/test it explicitly:

```bash
cargo build -p codex-ingest
cargo test --locked --no-fail-fast -p codex-ingest
( cd crates/codex-ingest && cargo clippy --locked --tests -- -D warnings )
```

### Desktop shell, Rust side (`apps/desktop/src-tauri`, crate `codex-desktop`)

Its own Cargo workspace (`[workspace]` table with no members — see overview.md) — always `cd` into
it first:

```bash
cd apps/desktop/src-tauri
cargo test --locked
cargo clippy --locked --tests -- -D warnings
cargo check   # fast compile-only check; also available as `npm run tauri:check` from apps/desktop
```

### Frontend (`apps/desktop`, TypeScript/React)

```bash
cd apps/desktop
npm ci                 # clean-checkout install; use `npm install` only when deliberately changing deps
npm run typecheck       # tsc --noEmit
npm test                # node scripts/run-tests.mjs — NOT vitest; globs every src/**/*.test.ts, runs each via tsx
npm run build           # node ../../scripts/gen-corpus-bundle.mjs && vite build
```

`npm run typecheck` failing with `sh: 1: tsc: not found` almost always means `npm ci` was run with
dev dependencies stripped, or never run at all — `rm -rf node_modules && npm ci` recovers it.

## Running the desktop app

### Human path (real graphical Linux session)

```bash
cd apps/desktop
npx tauri dev        # opens a real window; Ctrl-C to stop
```

### Headless / agent path

There is no Chrome DevTools Protocol here — this is Tauri (Rust) + React, not Electron. Drive it
via the repo's own skill instead of hand-rolling xdotool calls:
`apps/desktop/.claude/skills/run-desktop/SKILL.md` (or invoke the `run-desktop` skill by name in a
Claude Code session). It wraps `driver.sh`, which launches the app under Xvfb and exposes
`screenshot`/`click`/`scroll`/`type`/`key`/`logs`/`diagnose`/`stop` subcommands against a state file,
so no shell needs to stay attached between calls.

```bash
export RUN_DESKTOP_AGENT=<your-unique-id>   # REQUIRED if any other agent might also drive the app —
                                             # omitting it makes you 'default', and two 'default' agents
                                             # collide (same DISPLAY, same state file, same logs)
./.claude/skills/run-desktop/driver.sh launch
./.claude/skills/run-desktop/driver.sh screenshot /tmp/01-hub.png
./.claude/skills/run-desktop/driver.sh click 1103 137
./.claude/skills/run-desktop/driver.sh stop
```

**Do not run the desktop app (human or headless path) concurrently with `scripts/verify.sh`** — both
build against Cargo caches that are not safe to share under concurrent writers on this box.

### `npm run ui-smoke` — the DOM-probe UI regression harness

```bash
cd apps/desktop
RUN_DESKTOP_AGENT=<unique-id> node scripts/ui-smoke/run.mjs [--only <row-id>] [--from <row-id>] [--resume] [--out <dir>] [--keep]
```

Drives the running app through a spec of UI rows (`apps/desktop/scripts/ui-smoke/spec.json`), asserting each one
via a DEV-only DOM command channel rather than pixel coordinates. `--only <id>` runs a single row;
`--from <id>` runs that row through the end of the spec; `--resume` reads a previous run's
`results.json` at `--out` and skips rows already green/manual, printing how many it skipped — the
right flag for continuing a large repair pass without re-running what already passed. See
[glossary.md](./glossary.md) for "ui-smoke" and "DOM probe" if the terms are unfamiliar.

## `scripts/verify.sh` — the one verification command

`scripts/verify.sh` is the single gate this repo trusts; nothing else claims to be "the" test
command. It is deliberately not built on `set -e` — every stage runs regardless of an earlier
failure, so one red stage never hides a second one, and the summary at the end is always complete.

```bash
bash scripts/verify.sh --list          # print every stage and which set (full/quick) it belongs to
bash scripts/verify.sh --quick         # fast subset — skips the ~490-binary root sweep and clippy
bash scripts/verify.sh --only clippy   # one stage, repeatable (--only can be passed more than once)
bash scripts/verify.sh                 # the full gate — every stage
bash scripts/verify.sh --show-actuals  # also prints measured numbers in verify-baselines.env format
```

**How long it takes.** The full run is dominated by `root-full` (`cargo test --locked
--no-fail-fast`, building on the order of hundreds of test binaries) and `clippy` (three crates) —
budget roughly 1-2 hours on a modest box. `--quick` (which still runs every non-build gate:
`denominator-gate`, `figure-provenance`, `pcgen-residue-gate`, `crate-wall`, and friends) finishes
in minutes.

**The nohup-and-poll rule for anything over ~10 minutes.** Many agent/CI harnesses (including a
coding-agent session's own shell tool) kill a single command after a bounded timeout — commonly 10
minutes — well short of a full `verify.sh` run. Never run a long command directly in a
timeout-bounded shell and hope it finishes; background it and poll the log instead:

```bash
LOG=/tmp/verify-full.log
nohup bash -c 'bash scripts/verify.sh; echo EXIT=$?' > "$LOG" 2>&1 &
echo $! > "$LOG.pid"
# then, in separate calls:
while kill -0 "$(cat "$LOG.pid")" 2>/dev/null; do sleep 20; done   # or poll with a bounded timeout and repeat
tail -20 "$LOG"
```

Never report a run as finished, or its stage results as trustworthy, until the log actually shows
`EXIT=0` (or the specific stage's own pass/fail line) — a truncated log from a killed background
process looks identical to a slow-but-fine one until you check for the exit marker.

**Never run `verify.sh` concurrently with a desktop launch**, and never run two `cargo` invocations
at once for the same reason given above (§"Build and test, per crate").

## First contribution, red to green

A worked example: adding a new integration test to an **existing** test-file family (per
[conventions.md](./conventions.md) §"Corpus-gated test pattern" and
[testing.md](./testing.md) §"Test conventions" — new tests join a family binary rather than
starting a new top-level `tests/<name>.rs` file, since one more file is one more link on every
future build).

1. **Pick the file.** Find the `tests/*.rs` file that already proves behavior adjacent to yours —
   e.g. a `tests/derived_evaluator_fixture_check_<family>.rs` sibling if you're adding a new
   per-family provenance check, or the specific `sdNN_*`/`geNN_*` file for the slice you're
   touching. `git grep -l` for the function or type you're about to change if you're not sure which
   file already covers it.
2. **Write the failing test first.** `AGENTS.md`'s non-negotiable rule 1: write or update a test
   that fails, confirm it fails **for the intended reason** (not a typo or a missing import), then
   implement the smallest change to pass.
   ```bash
   cargo test --locked --test <the_file_stem> -- <your_new_test_name>
   ```
   Confirm the failure message is the one you expect (an assertion mismatch on the behavior you're
   adding), not a compile error or a panic somewhere unrelated.
3. **Implement the smallest change.** Follow the idiom the surrounding code already uses — see
   [conventions.md](./conventions.md) for the repo-wide catalog (fail-honest computation,
   gate-then-explain pairing, the `list_all` idiom, etc.) rather than inventing a new shape.
4. **Re-run that one test, then its whole binary, then the wider suite**, in that order — cheapest
   feedback first:
   ```bash
   cargo test --locked --test <the_file_stem>
   cargo test --locked --lib
   cargo test --locked
   ```
5. **Lint.** `cargo clippy --locked --tests -- -D warnings` — zero warnings is a hard ceiling, not
   a target.
6. **Run the full gate once, at the end** — not after every small edit
   (`scripts/verify.sh`, backgrounded per §"nohup-and-poll" above if you're in a bounded shell).
7. **Commit** using this repo's real scope convention (below) and open a PR into `develop`.

## Branch model, commits, and PRs

```mermaid
%%{init: { "gitGraph": { "mainBranchName": "develop" } } }%%
gitGraph
    commit id: "prior tranche closed"
    branch "tranche/16"
    checkout "tranche/16"
    commit id: "feat(sd36,epic-a)"
    commit id: "fix(sd36,desktop)"
    commit id: "docs(sd36)"
    checkout develop
    merge "tranche/16" id: "PR: tranche/16 to develop"
    branch test
    checkout test
    merge develop id: "promote: develop to test (beta)"
    branch main
    checkout main
    merge test id: "promote: test to main (stable)"
```

*Caption: `develop` is the integration target every `tranche/N` branch's PR lands on; `test` (beta)
and `main` (stable) are downstream promotion targets, each gated by CI-enforced source-branch
checks — nobody commits directly to any of the three.*

**The real chain**: `feature/*`/`tranche/N` → `develop` (alpha channel) → `test` (beta channel) →
`main` (stable channel). Each arrow is a PR, and each PR's source branch is enforced by CI
(`.github/workflows/allow-only-develop-into-test.yml`, `allow-only-test-into-main.yml`) — a PR into
`test` must come from `develop`, and into `main` must come from `test`; see
[release-pipeline.md](./release-pipeline.md) for the full promotion-gate chain and the evidence
`promotion-gates.yml` requires.

**Two infrastructure branches must never be deleted**, regardless of how stale they look in a
branch listing:

- **`test`** — the beta promotion gate. If deleted, the `restore-test-branch` job in
  `allow-only-test-into-main.yml` re-creates it at `main`'s current tip.
- **`update-index`** — the live update-channel feed. `publish-tester-release.yml`'s `finalize` job
  is its only writer; the desktop app's self-updater reads `channels/<channel>.json` from it
  directly. It is not a stale work branch even though its tip carries some incidental legacy tree
  content alongside the live `channels/`/`manifests/` paths.

**Commit message scopes**, as actually used (`git log --format=%s | grep -oE '^[a-z]+\([^)]*\)'`):
`<type>(<sd-id>[,<scope>])`, e.g. `feat(sd36,epic-a): finish the crate wall`,
`fix(sd36,desktop): ui-smoke final proof`, `docs(sd36): mark C1 cycle complete`,
`refactor(sd36,epic-c1): split pilot_compute into submodules`. Common `<type>` values seen in
history: `feat`, `fix`, `docs`, `refactor`, `chore`, `retro`, `measure`, `merge`. The `<sd-id>`
names the bundle the work belongs to (`sd36`, `sd35`); the optional `<scope>` narrows it further
(an epic id, `desktop`, a criterion).

**Who merges what.** An operator (human) merges PRs into `develop`, `test`, and `main` — an agent
session opens the PR and ensures its gates are green, but does not merge it itself unless
explicitly told to for that specific PR. `docs/release/<bundle>/workflow-instruction.md` (per
bundle) is the place a specific bundle's dispatch/approval scope is spelled out in full if you are
operating under one.

## Pointers

- `AGENTS.md` (repo root) — the durable conduct surface for a coding harness in this repo: TDD is
  mandatory, no stubs in shipping code, read discipline, blocker discipline. Read it before taking
  implementation work.
- `CLAUDE.md` (repo root) — the lightweight activation surface pointing back at `AGENTS.md`.
- `docs/governance/` — standing doctrine this repo enforces mechanically where possible:
  `no-stub-mvp-doctrine.md`, `blocker-closure-doctrine.md`, `book-ingestion-playbook.md`,
  `wired-integration-stubs-registry.md` (the only place a stub exception is recorded).
- `docs/release/<bundle>/` — one folder per bundle: `scope-draft.md`, `decisions.md`,
  `epic-breakdown.md` (acceptance commands per criterion), `workflow-instruction.md` (dispatch
  procedure for that bundle), `progress.md`, `receipts.md`, `release-notes.md` (a
  regex-locked CI/schema contract — see [release-pipeline.md](./release-pipeline.md)). This is
  planning/execution narrative, not architecture — [overview.md](./overview.md) and its siblings
  describe what the code does *now*, independent of which bundle built it.

## See also

- [overview.md](./overview.md) — read this first: what Codex is, the crate/workspace structure, the data flow.
- [testing.md](./testing.md) — the full verification command set and fixture grammar in depth.
- [release-pipeline.md](./release-pipeline.md) — the publish workflow and branch-promotion gates in depth.
- [glossary.md](./glossary.md) — unfamiliar terms used above.
- [README.md](./README.md) — the doc set's index and reading paths by intent.
