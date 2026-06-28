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
repos/codex/
  src/
    pcgen_import/        # GE-03 importer foothold
    rules_core/          # GE-06 bounded pilot computation surfaces
    homebrew_authoring/  # GE-08 bounded package/preview surfaces
  tests/                 # bounded proof harness
  apps/desktop/          # React + Tauri desktop shell/workbench surface
  AGENTS.md              # repo-root conduct surface for coding harnesses
  README.md              # first-contact onboarding surface
```

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
apps/desktop/src-tauri/target/debug/codex_desktop_shell_scaffold
```

## Build and verification surfaces

### Core proof harness

```bash
cd /home/ubuntu/workspace/repos/codex
. "$HOME/.cargo/env"
cargo test
```

### Focused bounded proof slices

```bash
cargo test ge06_
cargo test ge08_
```

### Desktop/frontend verification

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
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
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npx tauri dev
```

Or run the built binary directly:

```bash
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/debug/codex_desktop_shell_scaffold
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
- treat this repo as the implementation surface and `programs/codex` as the wider planning/control plane
- do not implement from a spec domain or README alone; use a bounded handoff or source STC
- prefer the smallest compliant change and verify it with real commands

## Related authority surfaces

Inside the lab workspace, the broader program-level governance and demo/onboarding packet live under:

```text
programs/codex/
programs/codex/requirements/GE-10-demo-proof-and-onboarding/
```

Those surfaces are the planning and onboarding control plane. This repo is the implementation and proof surface.
