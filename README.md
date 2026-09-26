# Codex

Codex is a Rust + Tauri replacement effort for PCGen. PCGen is the heritage application and oracle substrate; Codex is the new program and implementation surface.

> Last verified: **2026-09-26 against `tranche/16`, HEAD `e70a8745ed`** (SD-36 Epic F5 closure pass; figures from `class_census --json`, `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f4-f5/census-f5.json`).

**Architecture and design documentation** for anyone — human or agent — working in this repo lives at [`docs/architecture/`](docs/architecture/README.md). Start there for the system map, module boundaries, design conventions, and the current real-vs-stubbed capability status. First stop for setup: [`docs/architecture/README.md`](docs/architecture/README.md) (doc-set index and reading paths) and [`docs/architecture/getting-started.md`](docs/architecture/getting-started.md) (toolchain setup, the full build/test/run command set, and `scripts/verify.sh`).

## Current state

**Current truthful posture:** Codex's PF1e compute engine is not a single-class proof slice; it covers the whole corpus it has ingested — all 37 books of the converted sheet-rule package (49,450 records, 0 refused). The class census merges every engine registry into 137 distinct class ids corpus-wide: every one of the 63 non-prestige ids reaches a fully `Computed` sheet at every level of its own sweep (63 of 63 non-prestige, 0 blocked — Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Pathfinder Unchained, Ultimate Combat, the occult, intrigue, magic, psionic and wilderness classes, and the CRB NPC classes alike); the 74 prestige ids are Blocked alone by the game rule and 68 of 74 reach `Computed` in a carrier mix; the multiclass mix panel is 185 of 185 `Computed` (`cargo run --locked -j 8 --bin class_census -- --json <path>`, 2026-09-26; the census holds race fixed to one Human fixture, so it proves every level, not every race — see status.md for the exact scope). The desktop app is a real, wired, end-to-end character-creation/leveling/equipment/DM-toolkit/encounter-builder/campaign-manager product; its Create picker offers 59 classes from the engine's own roster and its level-up offers every prestige class with the entry requirements printed, verified by 76 automated UI-smoke rows (66 of the first 69 green, 3 native file dialogs manual; the 7 class-roster/prestige rows 7 of 7 green). **Separately** — this is a different measurement and must not be conflated with the compute claim above (status.md says so explicitly) — the corpus's ingestion/classification state is a frozen snapshot at 100% of 49,450 catalogued units as of 2026-09-15 (see status.md's "Corpus coverage"): that figure answers "is every corpus unit present and classified," not "does the compute engine produce a value for it today." See [`docs/architecture/status.md`](docs/architecture/status.md) for the evidence-backed capability matrix and the named exceptions: 6 of 74 prestige carrier mixes stay Blocked on an oracle save formula that states no PF1 save (closable only by a book-cited override, FS-15), and 4 of the 63 Computed non-prestige ids — the Ex-* states ex_antipaladin, ex_barbarian, ex_inquisitor, ex_paladin — are census-only and never offered at creation, by ruling; the Create picker offers the other 59 of 63.

The maintained, closure-updated statement of what is real vs stubbed today is
[`docs/architecture/status.md`](docs/architecture/status.md) — it supersedes any
snapshot list this README used to carry. The full verification command set is
[`docs/architecture/testing.md`](docs/architecture/testing.md) and
[`docs/architecture/getting-started.md`](docs/architecture/getting-started.md). This README does
not grant implementation authority by itself; use the bounded handoff or source
STC for scoped work.

## Repository layout

```text
codex/
  src/
    rules_core/          # PF1e compute engine: rule-data tables, pilot compute chassis, boundary contract
    homebrew_authoring/  # homebrew package authoring + preview surfaces (e.g. the Guard Stance proof package)
    saved_character/     # saved-character on-disk persistence
    campaign/             # campaign on-disk persistence
  crates/codex-ingest/    # PCGen .pcc/.lst corpus ingest + oracle-parity comparator — dev-dependency of the desktop shell only, never a runtime dependency of the live sheet engine
  tests/                  # integration test suite (`cargo test --locked`)
  apps/desktop/           # React + Tauri desktop app — the real, wired character-sheet product surface
  docs/release/           # every SD-NN bundle's full docs, including release-notes.md — see below
  AGENTS.md               # repo-root conduct surface for coding harnesses
  README.md               # first-contact onboarding surface
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

### Root crate test suite

From the repo root:

```bash
cargo test
```

### Focused test slices, by grand-epic origin

```bash
cargo test ge06_    # rules-engine / pilot-compute tests
cargo test ge08_    # homebrew-authoring (Guard Stance) tests
```

`scripts/verify.sh` is the one gate this repo trusts for a full pass (root suite, `crates/codex-ingest`,
the desktop Rust crate, lint, and the frontend) — see
[`docs/architecture/getting-started.md`](docs/architecture/getting-started.md) §"`scripts/verify.sh`"
for the full stage list and the nohup-and-poll pattern a full run needs (it takes roughly 1-2 hours).

### Desktop/frontend verification

```bash
cd apps/desktop
npm run typecheck
npm test
npm run build
npm run tauri:check
npx tauri build --debug
```

## Run the app

### Headless test walkthrough

1. Complete the getting-started steps above.
2. Run `cargo test` from the repo root.
3. Run `cargo test ge06_` to verify the rules-engine / pilot-compute tests.
4. Run `cargo test ge08_` to verify the homebrew-authoring (Guard Stance) tests.

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
- the app opens on the real Character Hub — create, load, clone, and level up a character; the
  create-flow's picker reads the engine's class roster (`list_class_creation_roster`): it offers every
  non-prestige class the class census measures `Computed` at every level of its own sweep and that
  states a hit die — 59 classes (63 of 63 non-prestige Computed, less the 4 census-only Ex-* states;
  `class_census --json`, `roster_offered=59`, 2026-09-26). Prestige classes are offered at level-up
  with their entry requirements printed met/unmet, never blocked. See
  [`docs/architecture/status.md`](docs/architecture/status.md)'s capability matrix for the classes that
  do **not** reach a fully computed sheet (every prestige class taken alone; 6 of 74 prestige carrier mixes)
- the DM Toolkit, encounter builder, campaign manager, and equipment/spell/class/race/monster
  catalogs are all real, wired features reachable from here, not stubs or placeholders
- the GE-08 homebrew authoring workbench (the Guard Stance proof package's validate/persist/preview
  round trip) is one real feature within this surface, not the whole app

For driving the app without a graphical session, see
[`docs/architecture/getting-started.md`](docs/architecture/getting-started.md) §"Running the desktop
app" — the `run-desktop` skill launches and drives it under a virtual (Xvfb) display, and is how the
76-row UI-smoke regression suite cited in
[`docs/architecture/status.md`](docs/architecture/status.md) is produced.

## Known limitations

- the 4 Ex-* states (ex_antipaladin, ex_barbarian, ex_inquisitor, ex_paladin) are Computed but
  census-only — never offered at creation, by operator ruling (0 of the other 59 Computed
  non-prestige classes are missing from the Create picker)
- no prestige class reaches `Computed` taken alone (74 of 74 Blocked on
  `prestige_class.requires_base_class_levels`, the game rule); 68 of 74 reach `Computed` in a carrier
  mix, and the other 6 are Blocked on an unrecognized source save formula (FS-15) — see
  [`docs/architecture/status.md`](docs/architecture/status.md)'s class-coverage table for the exact
  figures and evidence
- multiclass joins any class with a chassis (mix panel 185 of 185 `Computed`); a member that cannot
  join is named (`multiclass.class_unsupported`, `multiclass.save_shape.*`)
- character level is capped at 20, matching PF1's own rule; the engine refuses level 21+ rather
  than silently accepting it
- only the Linux desktop onboarding/build path has been verified; do not assume parity on another
  platform until you have repeated the proof there

See [`docs/architecture/status.md`](docs/architecture/status.md) for the full evidence-backed
capability matrix and every limitation's citation.

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

## Long-lived infrastructure branches

- `test` is the promotion gate (beta channel) for stable releases: only the `test` branch may merge into `main` per `.github/workflows/allow-only-test-into-main.yml`. The workflow automatically re-creates `test` at `main`'s current tip if deleted. **Never delete it** — operator ruling 2026-09-07.

- `update-index` is the release channel-index feed. `.github/workflows/publish-tester-release.yml`'s `finalize` job is the only writer: it checks out (or orphan-creates) the branch, writes `channels/<channel>.json` and a mirrored `manifests/<manifest-tag>/update-manifest.json`, and pushes straight to `origin/update-index` on every successful `develop`/`main` publish run. The desktop app's updater (`apps/desktop/src/update/indexSource.ts`, `fetch.ts`) reads the channel pointer directly from `https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/<channel>.json` — this branch is the update feed, not a stale work branch. **Never delete it.** Its tip also carries a large snapshot of an old full repo tree alongside `channels/`/`manifests/`; that tree is incidental (an artifact of how the branch was first cut) and is not maintained or read by anything — only `channels/` and `manifests/` are live.

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
