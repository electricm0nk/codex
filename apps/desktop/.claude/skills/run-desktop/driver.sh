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

STATE_FILE="/tmp/run-desktop-driver.state"
DISPLAY_NUM=99
WINDOW_TITLE="Codex Character Hub"
LOG_FILE="/tmp/run-desktop-driver.tauri-dev.log"

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

  echo "Starting Xvfb on :$DISPLAY_NUM ..."
  Xvfb ":$DISPLAY_NUM" -screen 0 1280x900x24 >/tmp/run-desktop-driver.xvfb.log 2>&1 &
  local xvfb_pid=$!
  for _ in $(seq 1 30); do
    DISPLAY=":$DISPLAY_NUM" xdotool getdisplaygeometry >/dev/null 2>&1 && break
    sleep 0.3
  done
  DISPLAY=":$DISPLAY_NUM" xdotool getdisplaygeometry >/dev/null 2>&1 \
    || { echo "Xvfb did not come up; see /tmp/run-desktop-driver.xvfb.log" >&2; exit 1; }

  # Vite's dev port must be free or `tauri dev` fails outright.
  local stale_port_pid
  stale_port_pid="$(lsof -ti:1420 2>/dev/null || true)"
  [ -n "$stale_port_pid" ] && kill -9 $stale_port_pid 2>/dev/null || true

  echo "Launching npx tauri dev (first build can take several minutes) ..." >&2
  (cd "$app_root" && DISPLAY=":$DISPLAY_NUM" npx tauri dev) >"$LOG_FILE" 2>&1 &
  local tauri_pid=$!

  # Readiness is checked by polling for the actual binary process, NOT by
  # grepping the log file: `npx tauri dev` relays cargo's output through
  # something PTY-like, and when that's redirected to a file (as here) the
  # writes can arrive in one late burst well after the process is already
  # up and the window exists — a log-only check can report "timed out"
  # minutes after the app was already usable. Process-table state has no
  # such buffering delay. The log is still tailed on a genuine failure.
  local ready=""
  for _ in $(seq 1 300); do
    if pgrep -f "target/debug/codex_desktop_shell_scaffold" >/dev/null 2>&1; then
      ready=1
      break
    fi
    if grep -qi "error\[" "$LOG_FILE" 2>/dev/null || grep -q "panicked at" "$LOG_FILE" 2>/dev/null; then
      echo "Build/launch failed — see $LOG_FILE" >&2
      tail -n 40 "$LOG_FILE" >&2
      exit 1
    fi
    sleep 1
  done
  [ -n "$ready" ] || { echo "Timed out waiting for launch; see $LOG_FILE" >&2; tail -n 40 "$LOG_FILE" >&2; exit 1; }

  # Find the app window by its configured title (tauri.conf.json app.windows[0].title).
  # This also incidentally proves the window title is what it's supposed to be.
  # WebKitGTK's cold-start window/webview creation lags process start by a
  # variable amount (observed anywhere from ~1s to >15s) — budget generously.
  local window_id=""
  for _ in $(seq 1 90); do
    window_id="$(DISPLAY=":$DISPLAY_NUM" xdotool search --name "$WINDOW_TITLE" 2>/dev/null | head -n1 || true)"
    [ -n "$window_id" ] && break
    sleep 0.5
  done
  [ -n "$window_id" ] || { echo "App process started but no window titled '$WINDOW_TITLE' appeared" >&2; exit 1; }

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
  tail -n "${1:-60}" "$LOG_FILE"
}

cmd_stop() {
  if [ -f "$STATE_FILE" ]; then
    # shellcheck disable=SC1090
    source "$STATE_FILE"
    pkill -9 -f "target/debug/codex_desktop_shell_scaffold" 2>/dev/null || true
    pkill -9 -f "node.*/apps/desktop/node_modules/.bin/vite" 2>/dev/null || true
    pkill -9 -f "node.*/apps/desktop/node_modules/.bin/tauri dev" 2>/dev/null || true
    [ -n "${XVFB_PID:-}" ] && kill -9 "$XVFB_PID" 2>/dev/null || true
    rm -f "$STATE_FILE"
  fi
  # Belt-and-suspenders: a stale Xvfb on our fixed display from a previous
  # crashed session, independent of whether a state file exists.
  pkill -9 -f "Xvfb :$DISPLAY_NUM " 2>/dev/null || true
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
  stop) shift; cmd_stop "$@" ;;
  *)
    echo "Usage: driver.sh {launch|screenshot|focus|click|scroll|type|key|title|geometry|logs|stop}" >&2
    exit 1
    ;;
esac
