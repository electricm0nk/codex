# Codex

Codex is a Rust + Tauri replacement effort for PCGen. PCGen is the heritage application and oracle substrate; Codex is the new program and implementation surface.

## Current state

**Current truthful posture:** Codex is a **developer proof harness plus a buildable desktop workbench surface**, not a finished end-user product.

### Verified live on 2026-06-28

From the live repo and desktop workspace:

- root `cargo test` passes
- focused GE-06 proof tests pass
- focused GE-08 proof tests pass
- `npm run typecheck` passes under `apps/desktop`
- `npm run build` passes under `apps/desktop`
- `npm run tauri:check` passes under `apps/desktop`
- `npx tauri build --debug` succeeds and produces a debug desktop binary

### What is real today

- **GE-03 import foothold** under `src/pcgen_import/` with a real PCC entry-file parser
- **GE-06 bounded pilot proof** under `src/rules_core/` and `tests/ge06_*`
- **GE-08 bounded homebrew/workbench proof** under `src/homebrew_authoring/`, `tests/ge08_*`, and `apps/desktop/`
- **Tauri desktop shell surface** under `apps/desktop/`

### What is not true yet

- this is not a general character builder
- this is not broad PCGen parity
- this is not public-release-ready product software
- this README does not grant implementation authority by itself; use the bounded handoff or source STC for scoped work

## Repository layout

```text
codex/
  src/
    pcgen_import/        # GE-03 importer foothold
    rules_core/          # GE-06 bounded pilot computation surfaces
    homebrew_authoring/  # GE-08 bounded package/preview surfaces
  tests/                 # bounded proof harness
  apps/desktop/          # React + Tauri desktop shell/workbench surface
  docs/release/          # every SD-NN bundle's full docs, including release-notes.md — see below
  AGENTS.md              # repo-root conduct surface for coding harnesses
  README.md              # first-contact onboarding surface
```

### Documentation structure

`programs/` does not exist in this repo (removed 2026-07-20). Every "SD-NN" (spec-domain) work bundle's documentation — including SD-13/16/17's legacy pre-`docs/release/`-convention artifacts and every bundle's CI-contracted `release-notes.md` — lives in one place:

- **`docs/release/SD-NN/`** — the canonical home for a bundle's full planning and execution documentation: `scope-draft.md`, `decisions.md`, `epic-breakdown.md`, `loop-instruction.md`, `progress.md`, `receipts.md`, per-cycle `artifacts/`, and `release-notes.md`. One folder per bundle, named exactly `SD-NN`. Copy `docs/release/template/template.md` when starting a new bundle; see `docs/release/README.md` for the full layout rule.
- **`docs/release/SD-NN/release-notes.md` is also a regex-locked CI/schema contract** (`^docs/release/[^/]+/release-notes\.md$`) consumed by `tools/release/`, `scripts/release/`, `publish-tester-release.yml`, and the desktop app's auto-update pipeline. Required section headers: `Summary`, `User-Visible Changes`, `Defects Fixed`, `Operational Notes`, `Verification Evidence`, `Known Issues`, `Update Eligibility` (enforced by `tools/release/check_release_manifest.py`). Note: ~25 already-published `update-manifest.json` files on the live `update-index` branch still reference the pre-2026-07-20 `programs/codex/requirements/` path with a locked content hash — those are not retroactively rewritten (a CI-only-write surface), so their "view release notes" binding is permanently stale by design.
- **`docs/release/SD-13/`, `SD-16/`, `SD-17/`** hold only the legacy `artifacts/` (and, for SD-16, `tranche-2.5/manifest.yaml`) that used to live under `programs/codex/requirements/` — relocated 2026-07-20, kept as-is otherwise (no retroactive full 10-file mirror for these pre-`docs/release/`-convention bundles).
- **`docs/doctrine-external/`** — a deliberate stub mirror of operator-side governance docs (`spec-domain-lifecycle.md`, `identifier-discipline.md`) so relative links from `docs/release/SD-NN/*.md` resolve in a cold clone. Not for new content.
- **The operator's separate, out-of-repo `/home/workspace/programs/codex/requirements/` planning-intake path** referenced in `docs/release/README.md` is unrelated to this repo's directory structure — it lives entirely on the operator's machine, outside any git clone. See that file's Cross-reference section for the full distinction.

## Getting started

These steps are grounded for the currently verified Linux desktop path (Ubuntu 24.04-style Tauri prerequisites). If you are onboarding on another platform, do not assume parity until you have repeated the proof there.

### 1. Install Linux system dependencies

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
. "$HOME/.cargo/env"
```

Verify:

```bash
cargo --version
rustc --version
```

### 3. Install Node.js LTS and verify npm

Install Node.js LTS using your normal machine bootstrap path, then verify:

```bash
node --version
npm --version
```

GE-10 was verified with:
- `node v22.23.1`
- `npm 10.9.8`

### 4. Install repo-local desktop dependencies

```bash
cd apps/desktop
npm ci
```

No separate global Tauri CLI install is required for the repo walkthrough; use the repo-local CLI via `npx tauri ...`.

Why `npm ci` here:
- this is the correct clean-checkout command for a lockfile-governed repo
- it reproduces the committed dependency set instead of opportunistically updating it
- use `npm install` only when you intentionally mean to change or refresh dependencies

### 5. Build the desktop shell up front

Run this before later demo steps so you do not pause to install or build mid-walkthrough:

```bash
cd apps/desktop
npm run typecheck
npm run build
npm run tauri:check
npx tauri build --debug
```

Expected build artifact:

```text
apps/desktop/src-tauri/target/debug/codex
```

## Build and verification surfaces

### Core proof harness

From the repo root:

```bash
cargo test
```

### Focused bounded proof slices

```bash
cargo test ge06_
cargo test ge08_
```

### Desktop/frontend verification

```bash
cd apps/desktop
npm run typecheck
npm test
npm run build
npm run tauri:check
npx tauri build --debug
```

## Run the current demo

### Headless proof walkthrough

1. Complete the getting-started steps above.
2. Run `cargo test` from the repo root.
3. Run `cargo test ge06_` to verify the bounded deterministic pilot surface.
4. Run `cargo test ge08_` to verify the bounded Guard Stance homebrew/workbench surface.

### GUI walkthrough

From a **graphical Linux desktop session**:

```bash
cd apps/desktop
npx tauri dev
```

Or run the built binary directly:

```bash
apps/desktop/src-tauri/target/debug/codex
```

Expected current behavior:
- the app is a bounded GE-08 workbench surface
- it loads the Guard Stance proof package
- it displays package state, preview state, and a structured snapshot

## Known limitations

- a headless shell can build the desktop binary but cannot launch the GTK GUI successfully
- the current UI surface is bounded and demo-oriented, not a general character builder
- only the Linux desktop onboarding/build path was verified in the GE-10 pass
- product breadth, parity, and release packaging remain unfinished

## Troubleshooting

### `npm run typecheck` fails with `sh: 1: tsc: not found`

Root cause:
- the desktop package expects `typescript` from the local project install
- `typescript` is declared in `apps/desktop/package.json` as a devDependency
- during `npm run`, npm automatically exposes `node_modules/.bin/tsc`
- if `tsc` is missing, the usual cause is that `npm install` was never run in `apps/desktop`, or devDependencies were omitted during install

Recovery:

```bash
cd apps/desktop
rm -rf node_modules
npm ci
npm run typecheck
```

If your environment omitted devDependencies, force them back in:

```bash
cd apps/desktop
npm install --include=dev
npm run typecheck
```

### `npx tauri dev` keeps waiting for `http://localhost:1420/`

Root cause:
- `src-tauri/tauri.conf.json` expects the frontend dev server at `http://localhost:1420`
- if Vite starts on its default port instead, Tauri waits forever for the wrong URL

Repo fix:
- `apps/desktop/vite.config.ts` must explicitly set the dev server port to `1420`

Expected working behavior after the fix:

```text
VITE ... ready
➜  Local:   http://localhost:1420/
```

If your local checkout still starts Vite on `5173`, pull the latest repo changes or update `vite.config.ts` to pin:

```ts
server: { port: 1420, strictPort: true }
```

### App window shows `package root does not exist` for `tests/fixtures/ge08/guard-stance-package`

Root cause:
- the GE-08 workbench request passes a repo-root-relative fixture path
- if the Tauri command resolves that path from the process current directory instead of the Codex repo root, the app looks under `apps/desktop/src-tauri/` and fails

Repo fix:
- `apps/desktop/src-tauri/src/main.rs` must resolve repo-relative package roots from `CARGO_MANIFEST_DIR` back to the Codex repo root, not from `std::env::current_dir()`

Expected working behavior after the fix:
- the app window loads the Guard Stance package successfully
- the workbench shows package state, preview state, and the snapshot payload instead of a missing-path error

## Onboarding and contribution rules

- read `AGENTS.md` before taking implementation work
- treat this repo as the implementation surface and `docs/release/SD-NN/` as the per-bundle planning/control plane (see "Documentation structure" above)
- do not implement from a spec domain or README alone; use a bounded handoff or source STC
- prefer the smallest compliant change and verify it with real commands

## Related authority surfaces

The broader program-level governance and demo/onboarding packet live in the lab
workspace outside this repository. Only the artifacts a bounded slice needs are
mirrored here, under `docs/release/SD-NN/` — every bundle, current-convention
(SD-18 onward) and legacy (SD-13, SD-16, SD-17) alike:

```text
docs/release/SD-22/  # current-convention example
docs/release/SD-13/  # legacy example (artifacts/ only, relocated 2026-07-20 from programs/codex/requirements/)
```

Those surfaces are the planning and onboarding control plane. This repo is the implementation and proof surface.
