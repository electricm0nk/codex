---
name: run-desktop
description: Build, run, and drive the Codex desktop app (Tauri + React). Use when asked to start the desktop app, build it, screenshot its UI, or click through/interact with the running app.
---

This is a Tauri 2 (Rust) + React desktop app, not Electron — there is no
Chrome DevTools Protocol to attach to. Drive it via
`.claude/skills/run-desktop/driver.sh` under Xvfb: it launches the real
app, then exposes `screenshot`/`click`/`scroll`/`type`/`key` and a few
other subcommands, backed by ImageMagick `import` and `xdotool`. All
paths below are relative to `apps/desktop/`.

## Prerequisites

Already present in this container image; on a fresh Ubuntu 24.04 box you'd need:

```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev pkg-config \
  xvfb xdotool imagemagick x11-utils lsof
```

Rust (via rustup) and Node.js LTS must also be installed — see the repo
root `README.md` §§1-3 if starting from nothing. Verified in this
container with `node v22.22.3`, `npm 10.9.8`, Rust via `~/.cargo/bin`.

## Setup

```bash
cd apps/desktop
npm ci
```

## Build

No separate build step is needed to develop/test — `driver.sh launch`
(below) builds the debug binary itself via `npx tauri dev` (first run
~2-3 min; cargo caches after that). If you want to confirm the build
health directly:

```bash
npm run typecheck   # tsc --noEmit
npm run build       # vite build
npm run tauri:check # cargo check --manifest-path src-tauri/Cargo.toml
```

## Run (agent path)

```bash
./.claude/skills/run-desktop/driver.sh launch
```

Starts Xvfb on `:99`, runs `npx tauri dev`, waits for the binary process
to exist and for a window titled "Codex" to appear, then
writes `/tmp/run-desktop-driver.state`. Every subsequent subcommand is a
short, independent invocation that reads that state file — there is no
persistent driver process to keep a shell attached to.

```bash
./.claude/skills/run-desktop/driver.sh screenshot /tmp/shot.png   # PNG of the window, exactly its content
./.claude/skills/run-desktop/driver.sh click 1103 137             # window-relative coordinates
./.claude/skills/run-desktop/driver.sh type "Aldric Ironhand"     # types into whatever has focus
./.claude/skills/run-desktop/driver.sh key ctrl+a                 # select-all, then `type` to replace
./.claude/skills/run-desktop/driver.sh title                      # WM_NAME — confirms the titlebar text
./.claude/skills/run-desktop/driver.sh geometry                   # window position/size
./.claude/skills/run-desktop/driver.sh logs [n]                   # tail -n <n> the tauri-dev log (default 60)
./.claude/skills/run-desktop/driver.sh stop                       # kill app + Vite + Xvfb, clean up
```

| command | what it does |
|---|---|
| `launch [app-root]` | Start Xvfb + `npx tauri dev`, wait for the window. `app-root` defaults to `apps/desktop` (resolved from the driver's own location). Idempotent — stops any previous run first. |
| `screenshot <path>` | `import -window <id> <path>` — captures exactly the app window, no desktop background/black bars. |
| `focus` | `xdotool windowfocus <id>` alone, if you need it standalone (click/type/key already call this). |
| `click <x> <y>` | Window-relative coordinates, origin top-left of the window content (below the OS titlebar, since there is none here — origin is the webview's own `(0,0)`). |
| `scroll <x> <y> [ticks=5] [up\|down=down]` | Scroll-wheel button clicks at that point — focus-independent, unlike `Page_Down`/`Home`/`End` (see Gotchas). |
| `type <text>` | Types into whatever currently has input focus — click a field first. Uses `--delay 50`; see Gotchas if characters still drop. |
| `key <keyname>` | An `xdotool key` keysym, e.g. `Return`, `Escape`, `ctrl+a`, `Tab`. |
| `title` | Prints `WM_NAME` — the window's titlebar text. |
| `geometry` | Prints window position + size. |
| `logs [n]` | Tails the `npx tauri dev` output log. |
| `diagnose` | App-process liveness on our display, the full window inventory with each `WM_NAME`, and the launch-log tail. Every `launch` failure path prints this automatically, **before** cleanup runs. |
| `stop` | Kills the app binary, Vite, and Xvfb by pattern match; removes the state file. Safe to call even if nothing is running. |

Example end-to-end sequence (create a character, confirm it computed):

```bash
./.claude/skills/run-desktop/driver.sh launch
./.claude/skills/run-desktop/driver.sh screenshot /tmp/01-hub.png        # empty/list state
./.claude/skills/run-desktop/driver.sh click 1103 137                    # "Create new character"
./.claude/skills/run-desktop/driver.sh click 650 229                    # name field
./.claude/skills/run-desktop/driver.sh type "Aldric Ironhand"
./.claude/skills/run-desktop/driver.sh click 185 751                    # "Create character"
sleep 1
./.claude/skills/run-desktop/driver.sh scroll 640 400 6                 # result renders below the fold
./.claude/skills/run-desktop/driver.sh screenshot /tmp/02-outcome.png
./.claude/skills/run-desktop/driver.sh stop
```

Coordinates above match the 1280x900 Xvfb screen / 1280x800 window this
skill launches at; if you resize, re-derive them from a screenshot.

## Concurrent agents — `RUN_DESKTOP_AGENT`

**Every dispatched desktop agent must export `RUN_DESKTOP_AGENT` to a value
unique to itself before invoking `driver.sh`.** If you omit it, you are
`default` — and so is anyone else who omits it. Two agents sharing `default`
collide: same `DISPLAY`, same state file, same logs, and one agent's
`launch`/`stop` can kill the other's running app. This is not hypothetical —
it collided twice in tranche/7, on a mechanism that had already existed for a
week (`docs/retro/tranche-7-retrospective.md` §4.1, §6.3).

**The mechanism, as implemented in `driver.sh` (read the script, not this
summary, before relying on it):**

- `AGENT_ID="${RUN_DESKTOP_AGENT:-default}"` — the whole isolation scheme
  keys off this one variable, and its default is the literal string
  `default`.
- `DISPLAY_NUM` is derived deterministically from `AGENT_ID`: `default`→`:99`,
  `frontend`→`:96`, `backend`→`:97`, `qa`→`:98`; any other `AGENT_ID` hashes
  via `cksum` into `:60`–`:89`, so two arbitrary distinct names land on
  different X displays with overwhelming probability but no absolute
  guarantee (a hash collision between two unrelated agent names is possible,
  though not observed in this program).
- State, launch log, and Xvfb log are all namespaced by `AGENT_ID`:
  `/tmp/run-desktop-driver-${AGENT_ID}.state`,
  `.tauri-dev.log`, `.xvfb.log`.
- The `stop`/cleanup kill loop (`kill_our_codex_processes`) only kills a
  `target/debug/codex` process whose own `/proc/<pid>/environ` has
  `DISPLAY=:$DISPLAY_NUM` matching *this* agent's display — so a correctly
  distinct `RUN_DESKTOP_AGENT` genuinely prevents one agent's `stop` from
  reaping another's app.
- **Closed 2026-08-11** (was: "known gap, still live"). The *readiness* poll
  inside `cmd_launch` used to use an unfiltered `pgrep -f "target/debug/codex"`,
  so a sibling agent's already-running app satisfied it before your own binary
  had started — and the window search then burned its whole budget on an empty
  display. Readiness now goes through `our_app_pids()`, which filters every
  candidate by its own `DISPLAY` environ exactly as the kill loop does, and
  matches on the executable's name rather than a path (a dispatched agent with
  its own `CARGO_TARGET_DIR` builds the binary outside `target/debug/`, where
  the old pattern matched nothing at all). Covered by
  `scripts/tests/test_run_desktop_driver.sh` cases 1-2 and by the
  `driver-selftest` stage of `scripts/verify.sh`. See
  `docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md` Decision 43.

**Practical rule:** pick a short, stable, role-named value —
`RUN_DESKTOP_AGENT=frontend`, `RUN_DESKTOP_AGENT=qa`, or the dispatched
agent's own name — and export it in the agent's environment before the first
`driver.sh` call. Do this for every concurrently-dispatched desktop agent,
not only when you know another one is already running: the whole point of
the mechanism is that agents cannot see each other's assignments in advance.

## On-screen verification (DoD item 8) — `verify-on-screen.sh`

The repeatable entry point for Definition-of-done item 8: prove a record
family's value actually renders on the player-visible screen, not merely
that a code path exists. One command per record:

```bash
export RUN_DESKTOP_AGENT=<your-cycle-id>   # REQUIRED — script refuses 'default'
./.claude/skills/run-desktop/verify-on-screen.sh \
  --family race_trait --record "Ironskinned" \
  --expect "Duergar" --expect "natural armor" \
  --out docs/release/<bundle>/artifacts/<cycle>/item8
```

What it does: launch (or reuse this agent's already-running app), click the
hub's "Browse …" link for the family, filter the catalog via its search box
to `--record`, screenshot, then **select-all + copy in the webview and read
the X clipboard back** (`read-clipboard.py`, python3-gi — no xclip in this
container). The record name, every `--expect` string, and a per-family
screen marker must all be present in the *rendered text* — a screenshot
alone can't be machine-checked, and the extraction is what catches the
"gate green, screen empty" defect class.

- Families: `equipment` · `spell` · `race_trait` · `monster`.
- **PASS** (exit 0): `<out>/<slug>.png` + `<out>/<slug>.verify.md` — the
  report carries family/record/expects, UTC time, HEAD, agent id, and the
  rendered lines that matched. Cite both paths in the cycle receipt.
- **FAIL** (exit nonzero): artifacts are renamed `<slug>.FAILED.png` /
  `<slug>.FAILED.verify.md` so they can never be mistaken for passing
  evidence. Failure paths: launch failure, wrong-screen navigation (marker
  guard — catches coordinate drift loudly), empty clipboard, record not
  rendered, expect string missing. Zero `--expect` strings is itself an
  error: a check that expects nothing verifies nothing.
- The app is **left running** after each record so a cycle can verify many
  records cheaply; run `driver.sh stop` once at cycle end. `--fresh` forces
  a relaunch; `--slug` overrides the derived artifact basename.
- Do not run concurrently with `scripts/verify.sh` (memory note below).
- Coordinates in the script's nav table were calibrated on the driver's
  1920x1200 Xvfb screen; if UI layout changes move a hub link or search
  box, the marker/record guards fail loudly — recalibrate from a
  screenshot and update the table in `verify-on-screen.sh`.

## Run (human path)

From a real graphical Linux session (not headless):

```bash
cd apps/desktop
npx tauri dev        # opens a real window; Ctrl-C to stop
```

## Test

```bash
cd apps/desktop
npm test             # frontend: tsx-run *.test.ts files
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml   # or: cd src-tauri && cargo test
cd ../.. && cargo test   # root rules-core/persistence suite
```

---

## Gotchas

- **`xdotool windowactivate` fails outright.** No window manager runs
  under plain Xvfb, so `_NET_ACTIVE_WINDOW` isn't supported and
  `windowactivate` errors. Never call it.
- **`xdotool type --window <id>` / `key --window <id>` silently drop
  input.** They log `XGetInputFocus returned the focused window of 1`
  and the keystrokes never reach the WebKitGTK webview's focused DOM
  element, even though the X window itself received the events. Fix:
  call `xdotool windowfocus <id>` once (ICCCM-based, doesn't need a WM),
  then use plain **unqualified** `xdotool type`/`key` (no `--window`),
  which follow real X input focus. `driver.sh click` already calls
  `windowfocus` before every click, so click-then-type sequences work
  without you having to think about it.
- **Home/End/Page_Down don't scroll the page when a `<select>` or input
  has focus** — the focused form control consumes the key itself
  (e.g. `Home`/`End` jump between `<select>` options) instead of it
  bubbling up to scroll the document. Use `driver.sh scroll <x> <y>`
  instead — scroll-wheel button clicks are focus-independent.
- **`xdotool type`'s default per-character delay (~12ms) can drop
  trailing characters** into this webview — observed "Aldric Ironhand"
  arrive as "Aldric Iro". `driver.sh type` already uses `--delay 50`,
  which has been reliable; if you bypass the driver and call `xdotool
  type` directly, set the delay yourself.
- **Readiness must be checked by polling the process table, not by
  grepping the `npx tauri dev` log file.** Cargo's build-progress output
  goes through something PTY-like inside `tauri dev`; when redirected to
  a file it can arrive in one late burst well after the app is already
  up and the window exists, making a log-content check report a false
  "timed out" minutes after the app was actually ready. `driver.sh`
  polls `pgrep -f target/debug/codex` instead,
  which has no such delay.
- **`npx tauri build --debug` fails at the AppImage bundling step** in
  this container — `xdg-open` isn't installed
  (`failed to bundle project: xdg-open binary not found`). This does
  NOT block development: the debug binary itself is already built and
  written to `src-tauri/target/debug/codex`
  *before* bundling runs, and `npx tauri dev` (what the driver uses)
  never bundles at all. Install `xdg-utils` if you specifically need the
  AppImage artifact.
- **Port 1420 conflicts** if a previous `tauri dev` wasn't fully killed
  (`Error: Port 1420 is already in use`). `driver.sh launch` kills
  anything on that port before starting, so this only bites you if you
  bypass the driver and launch `npx tauri dev` by hand twice.
- **Two X windows exist**, not one: the visible app window (titled
  "Codex") and a second, seemingly-inert window whose
  `WM_NAME` is the raw binary name `codex`.
  `driver.sh` finds the right one by searching for the configured
  window *title*, not by binary/class name — this also happens to prove
  the titlebar text is correct as a side effect of launching.
- Vite logs harmless warnings about schema files being "outside of Vite
  serving allow list" for paths under a sibling worktree — cosmetic,
  not a functional blocker; ignore them.

## Troubleshooting

**Read the `diagnose` block first.** Every `launch` failure path prints app
liveness, the window inventory, and the log tail before cleanup runs. Do not
reason from a post-mortem `pgrep`: `cmd_launch` traps `EXIT` and calls
`cmd_stop`, so by the time the command returns, the app and the X server are
gone *whatever* went wrong. An empty process table after a failed launch is the
driver's cleanup, not evidence about the app. Three cycles read it as a crash
and spent their budgets on a nonexistent app bug (Decision 43).

- **`libEGL warning: DRI3 error: Could not get DRI3 device`** is **not** an
  error and not a cause of anything. It appears on every successful launch on a
  headless box: there is no GPU under Xvfb, so WebKitGTK falls back to software
  rendering and carries on. Ignore it.
- **`Timed out waiting for the app process to start`**: the binary never
  started. The `diagnose` log tail names the real reason. Two common ones:
  a build error (look for `error[`), or the `beforeDevCommand` (vite) dying —
  `Killed` there means the OOM killer, see the memory note below.
- **`App process exited before a window ... appeared`**: the binary really did
  die. Look for `panicked at` in the log tail. `Failed to initialize gtk
  backend!` means the process had no usable `DISPLAY` — check that Xvfb for
  your display number is actually alive.
- **`App process is running but no window titled '...' appeared within Ns`**:
  the app is up but the window search failed. Either the title changed (compare
  the `diagnose` window inventory's `WM_NAME`s against `tauri.conf.json`'s
  `app.windows[0].title`, and update `WINDOW_TITLE` in `driver.sh`), or the box
  is loaded enough that WebKitGTK's cold start exceeded the budget. Measured
  idle and solo on the 4-core CI box: **~35s**. Default budget is **180s**;
  raise it with `RUN_DESKTOP_WINDOW_TIMEOUT=<seconds>`.
- **Memory, not disk, is the binding constraint for launching.** The CI box has
  **22 GiB RAM and zero swap**. A concurrent cargo build will get vite
  OOM-killed and the launch fails at `beforeDevCommand`. **Do not run
  `driver.sh launch` and `scripts/verify.sh` at the same time** — serialize
  them.
- **A `type`/`key` command appears to do nothing**: you likely skipped
  `click` (which sets focus) before it, or a dropdown/select still has
  focus and is eating the key — see Gotchas above.
- **`Error: Port 1420 is already in use`** from a manual (non-driver)
  `npx tauri dev`: `lsof -ti:1420 | xargs -r kill -9`, or just use
  `driver.sh stop` first.
