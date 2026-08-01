# Codex Vanilla-Machine Demo Runbook

## Purpose

This runbook is the canonical walkthrough for taking a new developer from a clean, currently verified Linux desktop machine to the current Codex proof surfaces without mid-demo prerequisite surprises.

## Verified posture for this runbook

This runbook is grounded by the 2026-06-28 verification pass on a Linux/Ubuntu 24.04-style environment.

Verified toolchain versions during grounding:
- `cargo 1.96.0`
- `rustc 1.96.0`
- `node v22.23.1`
- `npm 10.9.8`
- `tauri-cli 2.11.3` via `npx tauri --version`

Important boundary:
- build steps work in a headless shell
- launching the desktop GUI requires a graphical Linux desktop session

## Step 1 — Install prerequisites and build the desktop shell first

### 1A. Install Linux system dependencies

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

These packages follow the Tauri v2 Linux prerequisite surface for Debian/Ubuntu and match the class of libraries present on the environment used to verify Codex.

### 1B. Install Rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then start a fresh shell or load Rust into the current one:

```bash
. "$HOME/.cargo/env"
```

Verify:

```bash
cargo --version
rustc --version
```

### 1C. Install Node.js LTS and verify npm

Install Node.js LTS using your standard method for the machine. Then verify:

```bash
node --version
npm --version
```

GE-10 was grounded with:
- `node v22.23.1`
- `npm 10.9.8`

### 1D. Install repo-local desktop dependencies

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm ci
```

Important note:
- a separate global Tauri CLI install is not required for the repo walkthrough because `@tauri-apps/cli` is already a dev dependency and the docs use `npx tauri ...`
- `npm ci` is preferred here because GE-10 is defining a clean-machine reproducible path; `npm install` can drift the dependency graph on a fresh checkout

### 1E. Complete the first desktop-shell build now

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
npm run build
npm run tauri:check
npx tauri build --debug
```

Expected outcome:
- TypeScript passes
- production web bundle is emitted under `apps/desktop/dist/`
- the Tauri Rust layer compiles
- a debug desktop binary is produced under:

```text
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/debug/codex_desktop_shell_scaffold
```

Do not move to later steps until this build succeeds. The entire point of GE-10 Step 1 is to clear dependency and build friction up front.

## Step 2 — Verify the headless proof surfaces

Run the full root proof harness:

```bash
cd /home/ubuntu/workspace/repos/codex
. "$HOME/.cargo/env"
cargo test
```

Then run the two bounded proof slices directly:

```bash
cargo test ge06_
cargo test ge08_
```

What these prove:
- GE-06 proves the bounded deterministic PF1 Human Fighter pilot calculations and receipt posture
- GE-08 proves the bounded Guard Stance homebrew package validation/preview/workbench substrate

What they do **not** prove:
- full Pathfinder breadth
- full PCGen parity
- end-user product readiness

## Step 3 — Inspect the canonical proof inputs

Read the bounded deterministic pilot input:

```text
/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
```

Read the bounded GE-08 package manifest:

```text
/home/ubuntu/workspace/repos/codex/tests/fixtures/ge08/guard-stance-package/manifest.yaml
```

This grounds the operator in the exact proof objects rather than leaving the demo at the level of green test names.

## Step 4 — Reconfirm the desktop boundary and launch the current app surface

If you are on a graphical Linux desktop session, run:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
npm run build
npm run tauri:check
npx tauri dev
```

Alternative: run the built binary directly:

```bash
/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/target/debug/codex_desktop_shell_scaffold
```

Expected current behavior:
- the app is a bounded GE-08 workbench surface
- it loads the Guard Stance proof package
- it shows package state, preview state, and a structured snapshot

If you are in a headless shell:
- stop after the build and state plainly that GUI launch must be done from a graphical session
- do not misreport GTK/session failure as a Codex build failure

## Step 5 — Explain the current state honestly

After the operator has seen the proof surfaces, summarize the project like this:

- Codex is currently a **developer proof harness plus a buildable desktop workbench surface**
- the core Rust tests pass
- the desktop frontend and Tauri build path pass
- the current GUI surface is bounded and demo-oriented, not a general character builder
- the project is real, but still far from finished product scope

## Troubleshooting notes

### `cargo: command not found`
Load Rust into the shell:

```bash
. "$HOME/.cargo/env"
```

### `node` or `npm` missing
Install Node.js LTS, restart the shell, and verify with `node --version` and `npm --version`.

### `npm run typecheck` fails with `sh: 1: tsc: not found`
This means the local desktop dependencies are not fully installed, or devDependencies were omitted.

Recovery:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
rm -rf node_modules
npm ci
npm run typecheck
```

If the machine or CI environment installed only production dependencies:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm install --include=dev
npm run typecheck
```

### `npx tauri dev` waits forever for `http://localhost:1420/`
This means the Tauri dev URL and the Vite dev-server port are out of alignment.

Expected contract:
- `src-tauri/tauri.conf.json` uses `devUrl: "http://localhost:1420"`
- `vite.config.ts` must pin the dev server to port `1420`

Recovery:
- ensure your checkout includes the repo fix that sets:

```ts
server: { port: 1420, strictPort: true }
```

Working signal:

```text
VITE ... ready
➜  Local:   http://localhost:1420/
```

### App window shows `package root does not exist` for `tests/fixtures/ge08/guard-stance-package`
This means the desktop command is resolving a repo-relative package path from the wrong working directory.

Expected contract:
- the React side may pass `tests/fixtures/ge08/guard-stance-package`
- the Tauri command must resolve that path from the Codex repo root, not from `apps/desktop/src-tauri`

Recovery:
- ensure your checkout includes the repo fix in `apps/desktop/src-tauri/src/main.rs` that resolves package roots from `CARGO_MANIFEST_DIR` back to the repo root

Working signal:
- the GE-08 workbench loads the Guard Stance package and shows snapshot data instead of a missing-path error

### GTK launch failure in a terminal-only environment
This is expected in a headless shell. Use a graphical desktop session for the GUI-launch step.
