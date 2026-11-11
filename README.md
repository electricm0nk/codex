# Codex

Codex (Campaign Codex) is a desktop app that builds tabletop RPG characters and prints their
character sheets. It is a Rust + Tauri app with a React front end.

- Game systems: Pathfinder 1e (the full product), Starfinder 1e and Cyberpunk RED.
- It creates, levels up, equips and saves characters, and prints a PDF sheet.
- It also has a DM toolkit, an encounter builder and a campaign manager.

PCGen data is converted ahead of time into Codex's own format, and PCGen is used as a test
oracle. The app itself does not run PCGen code.

What works today, and what does not, is in
[`docs/architecture/status.md`](docs/architecture/status.md). That file is kept up to date at
every release. This README does not repeat its figures.

## Install the app

Download the newest release from the
[Releases page](https://github.com/electricm0nk/codex/releases). All releases so far are
**alpha** pre-releases (`gh release list` shows the tags as `alpha-v<version>-<sha>`).

Each release has one file per platform:

| Platform | File | Notes |
|---|---|---|
| Linux (Debian/Ubuntu, x86-64) | `Codex_<version>_amd64.deb` | Recommended for Linux. |
| Linux (any distro, x86-64) | `Codex_<version>_amd64.AppImage` | Use the installer script below. |
| macOS (Apple Silicon only) | `Codex_<version>_aarch64.dmg` | Not signed. There is no Intel Mac build. |
| Windows (x64) | `Codex_<version>_x64-setup.exe` | Not signed. |

`checksums.sha256` in the same release lists the SHA-256 of every file.

### Linux (.deb)

```bash
sudo apt install ./Codex_<version>_amd64.deb
```

Then open **Codex** from your app menu.

### Linux (AppImage, any distro: Arch, Fedora, ...)

Run the installer script:

```bash
curl -fsSL https://raw.githubusercontent.com/electricm0nk/codex/develop/scripts/install-linux.sh | bash
```

It downloads the newest AppImage, checks its sha256, and saves it as
`~/.local/share/codex/Codex.AppImage`. It also adds a `codex` command in `~/.local/bin`, an
app-menu entry and the app icon. The file name has no version in it on purpose: the in-app updater writes each
new version over that same file. Run the script once only. After that, update from inside the
app. To remove it, run the same command with `bash -s -- --uninstall` in place of `bash`.
Your characters are kept.

To do it by hand instead, rename the file **before** the first launch, for the same reason:

```bash
mv Codex_<version>_amd64.AppImage Codex.AppImage
chmod +x Codex.AppImage
./Codex.AppImage
```

If it fails with a FUSE error, install FUSE 2: `sudo pacman -S fuse2` (Arch),
`sudo apt install libfuse2t64` (Ubuntu 24.04+) or `sudo dnf install fuse-libs` (Fedora).
If the window opens blank, start it with `WEBKIT_DISABLE_DMABUF_RENDERER=1 codex`.

### macOS

1. Open the `.dmg` and drag **Codex** to **Applications**.
2. The app is not signed, so macOS blocks the first launch. Clear the block with:

   ```bash
   xattr -dr com.apple.quarantine /Applications/Codex.app
   ```

   Or open it once, then go to **System Settings → Privacy & Security** and click **Open Anyway**.

### Windows

1. Run `Codex_<version>_x64-setup.exe`.
2. The installer is not signed. If SmartScreen shows "Windows protected your PC", click
   **More info**, then **Run anyway**.

### Updates

The app checks for updates itself. It reads the release feed on the `update-index` branch (see
[Long-lived infrastructure branches](#long-lived-infrastructure-branches)). The update panel
lets you pick a channel (`alpha`, `beta`, `stable`) and install a newer build. Today only `alpha`
has builds.

## Build from source

This path is verified on Ubuntu 24.04. Other platforms build in CI
(`.github/workflows/publish-tester-release.yml`) but are not verified for local development.
The full guide, with every gotcha, is
[`docs/architecture/getting-started.md`](docs/architecture/getting-started.md).

### 1. System packages

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev pkg-config
```

### 2. Rust (stable, via rustup)

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
. "$HOME/.cargo/env"
cargo --version
```

Keep `~/.cargo/bin` first on your `PATH`, so a system `cargo` does not shadow rustup's.

### 3. Node.js 22 LTS

Install Node.js 22 (CI uses `node-version: '22'`), then check:

```bash
node --version
npm --version
```

### 4. Front-end dependencies

```bash
cd apps/desktop
npm ci
```

Use `npm ci`, not `npm install`, so you get the exact locked versions. The Tauri CLI comes with
these dependencies; run it as `npx tauri ...`. You do not need a global install.

### 5. Run the app

From a graphical desktop session:

```bash
cd apps/desktop
npx tauri dev
```

### 6. Build an installer (optional)

```bash
cd apps/desktop
npx tauri build
```

The installers land in `apps/desktop/src-tauri/target/release/bundle/`. A release build takes a
long time and a lot of disk space.

`npm run build` (which `tauri build` calls) first runs `scripts/gen-corpus-bundle.mjs`. That
script copies the git-tracked game data in `data/corpus/` into the app bundle.

## Test

```bash
cargo test --locked                                  # root crate, from the repo root
(cd apps/desktop/src-tauri && cargo test --locked)   # desktop Rust crate
(cd apps/desktop && npm run typecheck && npm test)   # front end
```

The PCGen oracle checkout is **not** needed for these. Tests that need it skip themselves.
To fetch it for ingest work, run `scripts/fetch-pcgen-oracle.sh`.

`scripts/verify.sh` is the one full gate (all crates, lint and the front end). A full run takes
hours. See [`docs/architecture/getting-started.md`](docs/architecture/getting-started.md)
§"`scripts/verify.sh`" for how to run it, and
[`docs/architecture/testing.md`](docs/architecture/testing.md) for the test layout.

## Repository layout

```text
codex/
  src/                  # rules engine (rules_core), saved characters, campaigns, homebrew authoring
  crates/codex-ingest/  # PCGen converter + oracle-parity harness (tool side only, never in the app)
  apps/desktop/         # the desktop app: React front end + Tauri shell (src-tauri/)
  data/                 # converted game data (corpus, sheet_rules, starfinder-1e, cyberpunk-red, ...)
  tests/                # root integration tests
  scripts/              # verify.sh, oracle fetch, corpus bundle and other tooling
  tools/release/        # release-manifest tooling used by CI
  schemas/              # JSON schemas (rules, update feed)
  site/                 # public status site
  docs/architecture/    # current-state docs: start here
  docs/release/         # one folder per work bundle (SD-NN-<slug>), with release notes
  AGENTS.md             # rules for coding agents in this repo
```

Docs to read first:

- [`docs/architecture/README.md`](docs/architecture/README.md): the doc index and reading paths.
- [`docs/architecture/status.md`](docs/architecture/status.md): what is real, and the known gaps.
- [`docs/release/README.md`](docs/release/README.md): how the work-bundle folders are laid out.

Each bundle's `docs/release/<bundle>/release-notes.md` is also a CI contract.
`tools/release/check_release_manifest.py` checks its required section headers.

## Troubleshooting

### `npm run typecheck` fails with `sh: 1: tsc: not found`

The local dev dependencies are missing. Reinstall them:

```bash
cd apps/desktop
rm -rf node_modules
npm ci
```

### Desktop Rust tests fail with `resolved to /usr/lib/Codex/...`

The installed `.deb` puts game data in `/usr/lib/Codex`, and the tests find it before your
checkout. Point the tests at your checkout:

```bash
cd apps/desktop/src-tauri
CODEX_REPO_ROOT=/path/to/your/codex/checkout cargo test --no-fail-fast
```

### Build errors that look like a compiler bug

Errors like `ld terminated with signal 7 [Bus error]` or "couldn't create a temp dir" usually
mean the disk is full or `CARGO_TARGET_DIR` is stale. Check `df -h` and `unset CARGO_TARGET_DIR`
first.

## Long-lived infrastructure branches

- `test` is the promotion gate (beta channel) for stable releases: only the `test` branch may merge into `main` per `.github/workflows/allow-only-test-into-main.yml`. The workflow automatically re-creates `test` at `main`'s current tip if deleted. **Never delete it** — operator ruling 2026-09-07.

- `update-index` is the release channel-index feed. `.github/workflows/publish-tester-release.yml`'s `finalize` job is the only writer: it checks out (or orphan-creates) the branch, writes `channels/<channel>.json` and a mirrored `manifests/<manifest-tag>/update-manifest.json`, and pushes straight to `origin/update-index` on every successful `develop`/`main` publish run. The desktop app's updater (`apps/desktop/src/update/indexSource.ts`, `fetch.ts`) reads the channel pointer directly from `https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/<channel>.json` — this branch is the update feed, not a stale work branch. **Never delete it.** Its tip also carries a large snapshot of an old full repo tree alongside `channels/`/`manifests/`; that tree is incidental (an artifact of how the branch was first cut) and is not maintained or read by anything — only `channels/` and `manifests/` are live.

## Onboarding and contribution rules

- read `AGENTS.md` before taking implementation work
- treat this repo as the implementation surface and `docs/release/<bundle>/` as the per-bundle planning/control plane (see [Repository layout](#repository-layout))
- do not implement from a spec domain or README alone; use a bounded handoff or source STC
- prefer the smallest compliant change and verify it with real commands
