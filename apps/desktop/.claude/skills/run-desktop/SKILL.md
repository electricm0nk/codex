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

- **`Timed out waiting for launch; see ...log`**: check
  `driver.sh logs 60` — if it ends mid-build with no `error[`/`panicked`,
  the container may just be slow; re-run `launch` (it's idempotent) or
  raise the retry budget (`seq 1 300` near the top of `cmd_launch` in
  `driver.sh`).
- **`App process started but no window titled '...' appeared`**: two
  possible causes. (1) The binary is running but the window title
  doesn't match — check whether `tauri.conf.json`'s
  `app.windows[0].title` changed, and update `WINDOW_TITLE` at the top
  of `driver.sh` to match; confirm with
  `DISPLAY=:99 xdotool search --name "" | xargs -I{} xprop -id {} WM_NAME`.
  (2) WebKitGTK's cold-start window creation legitimately took longer
  than the retry budget this run (observed once, taking a little over
  15s when the budget was 15s) — check with the same `xprop` command
  above; if the window is there now, just re-run `launch` (it's
  idempotent) or raise the budget (`seq 1 90` in the window-search loop
  in `cmd_launch`).
- **A `type`/`key` command appears to do nothing**: you likely skipped
  `click` (which sets focus) before it, or a dropdown/select still has
  focus and is eating the key — see Gotchas above.
- **`Error: Port 1420 is already in use`** from a manual (non-driver)
  `npx tauri dev`: `lsof -ti:1420 | xargs -r kill -9`, or just use
  `driver.sh stop` first.
