#!/usr/bin/env bash
# Driver for launching and interacting with the Codex desktop (Tauri) app
# under a headless X virtual display. Each subcommand is a short-lived
# invocation that reads/writes a small state file, so there is no
# persistent driver process to keep alive — the X server and the app
# process are the state.
#
# Usage:
#   driver.sh launch [app-root]   # start Xvfb + npx tauri dev, wait for the window
#   driver.sh screenshot <path>   # capture exactly the app window to <path> (PNG)
#   driver.sh click <x> <y>       # click at window-relative coordinates
#   driver.sh type <text>         # type text into the focused element
#   driver.sh key <keyname>       # send a key (Return, Escape, Page_Down, Tab, ...)
#   driver.sh title               # print the window's WM_NAME (titlebar text)
#   driver.sh geometry            # print the window's position/size
#   driver.sh logs                # tail the tauri dev launch log
#   driver.sh stop                # kill the app + Vite + Xvfb, clean up

set -euo pipefail

# Multiple concurrent agents (e.g. named swarm teammates) must NOT share a
# display, state file, log file, or process-kill pattern -- doing so causes
# launches/stops to collide across agents (one agent's launch/stop killing
# another's running session, or windows/keystrokes crossing between them).
# Set RUN_DESKTOP_AGENT to a distinct identifier per concurrent agent (e.g.
# RUN_DESKTOP_AGENT=frontend) before invoking this script. Unset/default
# behavior is unchanged from before this existed (AGENT_ID=default,
# DISPLAY_NUM=99), so a single solo agent needs no changes.
AGENT_ID="${RUN_DESKTOP_AGENT:-default}"

# Deterministic per-agent DISPLAY_NUM: a few well-known names get fixed,
# easy-to-recognize numbers; anything else hashes into a safe range so two
# arbitrary agent names still (almost certainly) don't collide.
case "$AGENT_ID" in
  default) DISPLAY_NUM=99 ;;
  frontend) DISPLAY_NUM=96 ;;
  backend) DISPLAY_NUM=97 ;;
  qa) DISPLAY_NUM=98 ;;
  *) DISPLAY_NUM=$(( 60 + $(cksum <<<"$AGENT_ID" | cut -d' ' -f1) % 30 )) ;;
esac

# Deterministic per-agent VITE DEV PORT, for exactly the same reason as
# DISPLAY_NUM above -- and it was missing until SD-29 Epic 7 round 8
# (`decisions.md §65.7`), which cost that cycle two false item-8 runs.
#
# THE FAILURE, precisely: `vite.config.ts` pinned `port: 1420, strictPort: true`
# for every agent. A second concurrent agent's vite therefore could not bind and
# died, and `tauri dev` -- which only needs *something* answering on its
# configured `devUrl` -- happily attached to the FIRST agent's dev server. The
# result is the worst possible shape of wrong: agent B's Rust backend serving
# agent B's freshly-ingested records, painted by agent A's frontend source. The
# window renders, every record is present, and the screenshot is evidence about
# a tree the agent does not control. Round 8 caught it only because its new book
# label was missing on screen while its 38 new records were present -- had it
# changed no frontend string, the false artifact would have passed.
#
# The previous mitigation made it worse rather than better: the launch path
# below used to `kill -9` whatever held 1420, which is a sibling agent's dev
# server. So the two possible outcomes were "borrow another agent's frontend" or
# "clobber another agent's app", with nothing in between.
#
# `default` keeps 1420 so a solo agent's behaviour is byte-identical to before.
# Everything else derives from DISPLAY_NUM, which is already the per-agent
# uniquifier, so two agents collide here only if they already collide there.
if [ "$AGENT_ID" = "default" ]; then
  DEV_PORT=1420
else
  DEV_PORT=$(( 14200 + DISPLAY_NUM ))
fi
export CODEX_DEV_PORT="$DEV_PORT"

STATE_FILE="/tmp/run-desktop-driver-${AGENT_ID}.state"
WINDOW_TITLE="Codex"
APP_BIN_NAME="codex-desktop"
LOG_FILE="/tmp/run-desktop-driver-${AGENT_ID}.tauri-dev.log"
XVFB_LOG_FILE="/tmp/run-desktop-driver-${AGENT_ID}.xvfb.log"

# How long to wait for WebKitGTK to create the app window after the binary
# process appears. Measured on the 4-core CI box, idle and solo: ~35s. The
# original budget was 45s, which left no headroom — and a run that put six
# concurrent agents on those 4 cores (load average 9-12) blew through it every
# time, then reported the app as crashed. Override for a slower box.
WINDOW_TIMEOUT_SECS="${RUN_DESKTOP_WINDOW_TIMEOUT:-180}"

# How long to wait for the binary to appear at all. `launch` runs
# `npx tauri dev`, so this budget has to cover COMPILING the crate, not just
# starting it. Observed on this box: a launch expired at 346s with the log
# reading `Building 495/496: codex-desktop(bin)` — one crate unit short of
# running. Any sibling commit touching a dependency forces that full rebuild,
# which on 4 contended cores takes longer than the old 300s allowed.
LAUNCH_TIMEOUT_SECS="${RUN_DESKTOP_LAUNCH_TIMEOUT:-900}"

# Every process that is OUR app on OUR display.
#
# Two independent match sources, because neither alone is sufficient:
#   - `pgrep -x "$APP_BIN_NAME"` matches on the process's own executable name,
#     so it still finds the binary when CARGO_TARGET_DIR relocates it out of
#     `target/debug/` (which a dispatched agent with a per-agent target dir
#     always does — a path-only match silently finds nothing there);
#   - the legacy `target/debug/codex` command-line match, kept for any build
#     that lands under the old name.
#
# The DISPLAY filter is what makes this safe to act on. `pgrep -f` matches any
# command line CONTAINING the pattern, which includes the caller's own shell
# when the pattern appears in the command it is running; requiring the
# candidate's own environ to carry our DISPLAY, and its executable name to be
# a codex binary, excludes both that and every sibling agent's app.
our_app_pids() {
  local pid comm
  {
    pgrep -x "$APP_BIN_NAME" 2>/dev/null || true
    pgrep -f "target/debug/codex" 2>/dev/null || true
  } | sort -u | while read -r pid; do
    [[ -n "$pid" && "$pid" != "$$" ]] || continue
    comm="$(cat "/proc/$pid/comm" 2>/dev/null || true)"
    case "$comm" in *codex*) ;; *) continue ;; esac
    if tr '\0' '\n' < "/proc/$pid/environ" 2>/dev/null | grep -qx "DISPLAY=:$DISPLAY_NUM"; then
      echo "$pid"
    fi
  done
}

# Kill only codex processes whose DISPLAY env matches ours, so one agent's
# stop/cleanup can never reap another agent's running app even if both
# somehow ended up with an unscoped process-name match.
kill_our_codex_processes() {
  local pid
  for pid in $(our_app_pids); do
    kill -9 "$pid" 2>/dev/null || true
  done
}

# Kill the Xvfb serving exactly our display, matched on its ARGUMENT rather
# than on a substring of the whole command line. `pkill -f "Xvfb :$N "` reaps
# anything whose command line merely contains that text — observed live: it
# killed the shell that was invoking the driver, which then produced no output
# and no error, and looked exactly like the app dying.
kill_our_xvfb() {
  local pid
  for pid in $(pgrep -x Xvfb 2>/dev/null || true); do
    if [ "$(tr '\0' '\n' < "/proc/$pid/cmdline" 2>/dev/null | sed -n 2p)" = ":$DISPLAY_NUM" ]; then
      kill -9 "$pid" 2>/dev/null || true
    fi
  done
}

# What the failure paths print BEFORE cleanup runs.
#
# This is the whole reason three separate agents concluded "the binary exits
# before any window appears" when the binary was fine: `cmd_launch` traps EXIT
# and calls `cmd_stop`, so a launch failure killed the app and the X server on
# the way out. Every post-mortem `pgrep` then came back empty, which reads
# exactly like a crash. Print the evidence while it still exists.
cmd_diagnose() {
  local pids window_count
  pids="$(our_app_pids | tr '\n' ' ')"
  if [ -n "${pids// /}" ]; then
    echo "app process: ALIVE on :$DISPLAY_NUM (pid(s): ${pids% })"
  else
    echo "app process: none running on :$DISPLAY_NUM"
  fi

  echo "window inventory on :$DISPLAY_NUM (looking for WM_NAME exactly '$WINDOW_TITLE'):"
  window_count=0
  local w name
  for w in $(DISPLAY=":$DISPLAY_NUM" xdotool search --name "" 2>/dev/null || true); do
    name="$(DISPLAY=":$DISPLAY_NUM" xdotool getwindowname "$w" 2>/dev/null || true)"
    echo "  window $w  WM_NAME=${name:-<unset>}"
    window_count=$((window_count + 1))
  done
  [ "$window_count" -eq 0 ] && echo "  (no windows — X server unreachable or nothing mapped yet)"

  if [ -f "$LOG_FILE" ]; then
    echo "last 40 lines of $LOG_FILE:"
    tail -n 40 "$LOG_FILE" | sed 's/^/  /'
  else
    echo "no launch log at $LOG_FILE"
  fi
}

resolve_app_root() {
  if [ -n "${1:-}" ]; then
    echo "$1"
    return
  fi
  # .claude/skills/run-desktop/driver.sh -> apps/desktop is three levels up
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  (cd "$script_dir/../../.." && pwd)
}

cmd_launch() {
  local app_root
  app_root="$(resolve_app_root "${1:-}")"
  if [ ! -f "$app_root/package.json" ]; then
    echo "driver.sh launch: $app_root does not look like apps/desktop (no package.json)" >&2
    exit 1
  fi

  # Idempotent: clean up any previous run first.
  cmd_stop || true

  # Ensure we don't leak Xvfb/tauri processes on launch failures or interrupts.
  trap 'cmd_stop || true' EXIT INT TERM
  echo "Starting Xvfb on :$DISPLAY_NUM ..."
  Xvfb ":$DISPLAY_NUM" -screen 0 1920x1200x24 >"$XVFB_LOG_FILE" 2>&1 &
  local xvfb_pid=$!
  for _ in $(seq 1 30); do
    DISPLAY=":$DISPLAY_NUM" xdotool getdisplaygeometry >/dev/null 2>&1 && break
    sleep 0.3
  done
  DISPLAY=":$DISPLAY_NUM" xdotool getdisplaygeometry >/dev/null 2>&1 \
    || { echo "Xvfb did not come up; see $XVFB_LOG_FILE" >&2; exit 1; }

  # Our OWN dev port must be free or `tauri dev` fails outright. Scoped to
  # $DEV_PORT rather than a hardcoded 1420: this line used to kill whatever held
  # 1420, which under concurrency is a sibling agent's dev server, not a stale
  # process of ours (`decisions.md §65.7`). With a per-agent port the only thing
  # this can now reap is our own leftover.
  local stale_port_pids
  stale_port_pids="$(lsof -ti:"$DEV_PORT" 2>/dev/null || true)"
  [ -n "$stale_port_pids" ] && printf '%s\n' "$stale_port_pids" | xargs -r kill -9 2>/dev/null || true

  echo "Launching npx tauri dev on port $DEV_PORT (first build can take several minutes) ..." >&2
  # `--config` merges over tauri.conf.json, so the window loads OUR vite rather
  # than whatever answers on the shared default. `CODEX_DEV_PORT` (exported
  # above) is what vite.config.ts reads for the matching listen port; both sides
  # must move together or tauri waits forever on a port nothing serves.
  (cd "$app_root" && DISPLAY=":$DISPLAY_NUM" \
     npx tauri dev --config "{\"build\":{\"devUrl\":\"http://localhost:$DEV_PORT\"}}") \
     >"$LOG_FILE" 2>&1 &
  local tauri_pid=$!

  # Readiness is checked by polling for the actual binary process, NOT by
  # grepping the log file: `npx tauri dev` relays cargo's output through
  # something PTY-like, and when that's redirected to a file (as here) the
  # writes can arrive in one late burst well after the process is already
  # up and the window exists — a log-only check can report "timed out"
  # minutes after the app was already usable. Process-table state has no
  # such buffering delay. The log is still tailed on a genuine failure.
  #
  # Scoped to OUR display (see our_app_pids). The unscoped form matched any
  # agent's app process, so a sibling's already-running app satisfied this poll
  # instantly — and the window search below then spent its entire budget
  # looking for a window on an empty display while our own binary was still
  # compiling. That produced a "no window appeared" failure whose real cause
  # was that the app had not started yet.
  local ready=""
  for _ in $(seq 1 "$LAUNCH_TIMEOUT_SECS"); do
    if [ -n "$(our_app_pids)" ]; then
      ready=1
      break
    fi
    if grep -qi "error\[" "$LOG_FILE" 2>/dev/null || grep -q "panicked at" "$LOG_FILE" 2>/dev/null; then
      echo "Build/launch failed — see $LOG_FILE" >&2
      cmd_diagnose >&2
      exit 1
    fi
    sleep 1
  done
  [ -n "$ready" ] || { echo "Timed out after ${LAUNCH_TIMEOUT_SECS}s waiting for the app process to start (it may still have been compiling — check the log tail below for a 'Building N/M' line, and raise RUN_DESKTOP_LAUNCH_TIMEOUT if so); see $LOG_FILE" >&2; cmd_diagnose >&2; exit 1; }

  # Find the app window by its configured title (tauri.conf.json app.windows[0].title).
  # This also incidentally proves the window title is what it's supposed to be.
  # WebKitGTK's cold-start window/webview creation lags process start by a
  # variable amount (observed anywhere from ~1s to >15s) — budget generously.
  #
  # xdotool's --name regex match is case-insensitive regardless of anchors, so
  # a bare `search --name "$WINDOW_TITLE"` can return the "second, seemingly-
  # inert window whose WM_NAME is the raw binary name" (see Gotchas) whenever
  # the product name and binary name differ only by case — which happens for
  # this app. Filter candidates client-side against an exact, case-sensitive
  # WM_NAME match instead of trusting xdotool's own matching.
  local window_id=""
  local window_attempts=$(( WINDOW_TIMEOUT_SECS * 2 ))
  for _ in $(seq 1 "$window_attempts"); do
    local candidate wm_name
    for candidate in $(DISPLAY=":$DISPLAY_NUM" xdotool search --name "$WINDOW_TITLE" 2>/dev/null || true); do
      wm_name="$(DISPLAY=":$DISPLAY_NUM" xdotool getwindowname "$candidate" 2>/dev/null || true)"
      if [ "$wm_name" = "$WINDOW_TITLE" ]; then
        window_id="$candidate"
        break
      fi
    done
    [ -n "$window_id" ] && break
    # If the app process died while we were waiting, stop waiting and say so —
    # "no window appeared" and "the app crashed" need different fixes, and
    # reporting the first when the second happened is what sent three agents
    # after a nonexistent app bug.
    if [ -z "$(our_app_pids)" ]; then
      echo "App process exited before a window titled '$WINDOW_TITLE' appeared" >&2
      cmd_diagnose >&2
      exit 1
    fi
    sleep 0.5
  done
  if [ -z "$window_id" ]; then
    echo "App process is running but no window titled '$WINDOW_TITLE' appeared within ${WINDOW_TIMEOUT_SECS}s" >&2
    echo "(raise the budget with RUN_DESKTOP_WINDOW_TIMEOUT=<seconds> on a loaded box)" >&2
    cmd_diagnose >&2
    exit 1
  fi

  cat > "$STATE_FILE" <<EOF
DISPLAY_NUM=$DISPLAY_NUM
XVFB_PID=$xvfb_pid
TAURI_PID=$tauri_pid
WINDOW_ID=$window_id
APP_ROOT=$app_root
EOF

  trap - EXIT INT TERM

  echo "Ready. DISPLAY=:$DISPLAY_NUM WINDOW_ID=$window_id"
}

load_state() {
  [ -f "$STATE_FILE" ] || { echo "No running app — run 'driver.sh launch' first" >&2; exit 1; }
  # shellcheck disable=SC1090
  source "$STATE_FILE"
}

cmd_screenshot() {
  local out="${1:?usage: driver.sh screenshot <path>}"
  load_state
  DISPLAY=":$DISPLAY_NUM" import -window "$WINDOW_ID" "$out"
  echo "Wrote $out"
}

# No window manager runs under Xvfb. Two consequences, both load-bearing:
#   - `xdotool windowactivate` fails outright (_NET_ACTIVE_WINDOW unsupported)
#     — never call it.
#   - `xdotool type --window <id>` / `key --window <id>` silently deliver
#     nothing to the WebKitGTK webview (xdotool logs "XGetInputFocus
#     returned the focused window of 1" — the synthetic events don't reach
#     the focused DOM element even though they reach the X window). Use
#     `xdotool windowfocus <id>` (ICCCM-based, no WM required) once, then
#     plain unqualified `type`/`key` (no --window), which follow real X
#     input focus and work correctly.
cmd_focus() {
  load_state
  DISPLAY=":$DISPLAY_NUM" xdotool windowfocus "$WINDOW_ID"
}

cmd_click() {
  local x="${1:?usage: driver.sh click <x> <y>}"
  local y="${2:?usage: driver.sh click <x> <y>}"
  load_state
  DISPLAY=":$DISPLAY_NUM" xdotool windowfocus "$WINDOW_ID"
  DISPLAY=":$DISPLAY_NUM" xdotool mousemove --window "$WINDOW_ID" "$x" "$y" click 1
}

cmd_scroll() {
  local x="${1:?usage: driver.sh scroll <x> <y> [ticks] [direction: down|up]}"
  local y="${2:?usage: driver.sh scroll <x> <y> [ticks] [direction: down|up]}"
  local ticks="${3:-5}"
  local direction="${4:-down}"
  local button=5 # 5 = scroll down, 4 = scroll up
  [ "$direction" = "up" ] && button=4
  load_state
  # Scroll-wheel button clicks are focus-independent, unlike
  # Page_Down/Home/End which a focused <select> or input can swallow
  # instead of letting them scroll the page — see Gotchas.
  local args=()
  for _ in $(seq 1 "$ticks"); do args+=(click "$button"); done
  DISPLAY=":$DISPLAY_NUM" xdotool mousemove --window "$WINDOW_ID" "$x" "$y" "${args[@]}"
}

cmd_type() {
  local text="${1:?usage: driver.sh type <text>}"
  load_state
  # xdotool's default per-character delay (~12ms) is fast enough to drop
  # trailing characters into this webview (observed "Aldric Ironhand"
  # arrive as "Aldric Iro"). --delay 50 has been reliable in testing.
  DISPLAY=":$DISPLAY_NUM" xdotool type --delay 50 -- "$text"
}

cmd_key() {
  local keyname="${1:?usage: driver.sh key <keyname>}"
  load_state
  DISPLAY=":$DISPLAY_NUM" xdotool key -- "$keyname"
}

cmd_title() {
  load_state
  DISPLAY=":$DISPLAY_NUM" xprop -id "$WINDOW_ID" WM_NAME
}

cmd_geometry() {
  load_state
  DISPLAY=":$DISPLAY_NUM" xdotool getwindowgeometry "$WINDOW_ID"
}

cmd_logs() {
  [ -f "$LOG_FILE" ] || { echo "No log file at $LOG_FILE — run 'driver.sh launch' first" >&2; exit 1; }
  tail -n "${1:-60}" "$LOG_FILE"
}

cmd_stop() {
  if [ -f "$STATE_FILE" ]; then
    # shellcheck disable=SC1090
    source "$STATE_FILE"
    if [ -n "${TAURI_PID:-}" ]; then
      pkill -9 -P "$TAURI_PID" 2>/dev/null || true
      kill -9 "$TAURI_PID" 2>/dev/null || true
    fi
    kill_our_codex_processes
    [ -n "${XVFB_PID:-}" ] && kill -9 "$XVFB_PID" 2>/dev/null || true
    rm -f "$STATE_FILE"
  fi
  # Belt-and-suspenders: a stale Xvfb on our fixed display from a previous
  # crashed session, independent of whether a state file exists. Matched on
  # the argument, not on a command-line substring — see kill_our_xvfb.
  kill_our_xvfb
}

case "${1:-}" in
  launch) shift; cmd_launch "$@" ;;
  screenshot) shift; cmd_screenshot "$@" ;;
  focus) shift; cmd_focus "$@" ;;
  click) shift; cmd_click "$@" ;;
  scroll) shift; cmd_scroll "$@" ;;
  type) shift; cmd_type "$@" ;;
  key) shift; cmd_key "$@" ;;
  title) shift; cmd_title "$@" ;;
  geometry) shift; cmd_geometry "$@" ;;
  logs) shift; cmd_logs "$@" ;;
  diagnose) shift; cmd_diagnose "$@" ;;
  stop) shift; cmd_stop "$@" ;;
  # Introspection hooks. Public enough to debug with, named with a leading
  # underscore because they exist for scripts/tests/test_run_desktop_driver.sh
  # to assert against the real derivation rather than re-implement it.
  _display_num) echo "$DISPLAY_NUM" ;;
  _app_pid) our_app_pids ;;
  _window_timeout) echo "$WINDOW_TIMEOUT_SECS" ;;
  _launch_timeout) echo "$LAUNCH_TIMEOUT_SECS" ;;
  _diagnose) shift; cmd_diagnose "$@" ;;
  *)
    echo "Usage: driver.sh {launch|screenshot|focus|click|scroll|type|key|title|geometry|logs|diagnose|stop}" >&2
    exit 1
    ;;
esac
