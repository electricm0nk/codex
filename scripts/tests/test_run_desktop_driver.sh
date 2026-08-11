#!/usr/bin/env bash
# Self-test for apps/desktop/.claude/skills/run-desktop/driver.sh.
#
# Why this exists
# ---------------
# The driver is the ONLY mechanism that satisfies the "drive it on screen"
# acceptance item, and the tranche/7 retrospective ranks on-screen driving as
# the sole mechanism that caught 14% of that tranche's corrections — the only
# one that reaches the "wired into a twin the sheet doesn't read" defect class.
# When the driver breaks, that entire class of defect stops being detectable,
# silently.
#
# It did break. Five separate agents invoked `driver.sh launch` during the
# first corpus-wide catch-up run; not one left a state file behind. Three of
# them independently reported the same wrong diagnosis — "the binary exits
# before any window appears" — because the driver's own failure path destroyed
# the evidence before they could look at it: `cmd_launch` traps EXIT and calls
# `cmd_stop`, so any launch failure kills the app and the X server on the way
# out. By the time the agent ran `pgrep`, there was nothing to find. The app
# was never the problem.
#
# These cases pin the three driver behaviors that produced that outcome:
#   1. readiness detection must be scoped to OUR display (an unscoped
#      `pgrep -f target/debug/codex` latches onto a sibling agent's process and
#      starts the window-search budget before our own binary has started);
#   2. `stop` must not kill unrelated processes that merely mention our Xvfb
#      display on their command line (`pkill -f` matches any command line,
#      including the caller's own shell — this reaped a live test shell);
#   3. the failure path must emit diagnostics BEFORE cleanup runs.
#
# Runs against throwaway decoy processes and a scratch Xvfb. No build, no
# checkout mutation, seconds to run.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# Overridable so the cases can be pointed at a deliberately-regressed copy to
# confirm they still fail against it. A self-test whose detection power is
# never exercised passes with the same confidence whether it works or not —
# this repo has already been bitten twice by exactly that, on the identifier
# audit (see scripts/identifier-discipline-audit.sh's own header).
DRIVER="${RUN_DESKTOP_DRIVER:-$REPO_ROOT/apps/desktop/.claude/skills/run-desktop/driver.sh}"

passed=0
failed=0
TMPDIR_TEST="$(mktemp -d)"
DECOY_PIDS=()

cleanup() {
    local p
    for p in "${DECOY_PIDS[@]:-}"; do
        [[ -n "$p" ]] && kill -9 "$p" 2>/dev/null
    done
    rm -rf "$TMPDIR_TEST"
}
trap cleanup EXIT

ok()   { printf 'ok   %s\n' "$1"; passed=$((passed + 1)); }
nope() { printf 'FAIL %s\n     %s\n' "$1" "$2"; failed=$((failed + 1)); }

# `kill -0` is NOT a liveness check for a process that is our own child: a
# SIGKILLed child stays in the process table as a zombie until reaped, and
# `kill -0` succeeds on a zombie. Using it made case 3 below pass against a
# driver that was demonstrably killing the bystander — caught only by running
# the case against a deliberately-regressed copy.
is_alive() {
    local state
    state="$(awk '{print $3}' "/proc/$1/stat" 2>/dev/null)" || return 1
    [[ -n "$state" && "$state" != "Z" ]]
}

if [[ ! -x "$DRIVER" ]]; then
    nope "driver.sh is present and executable" "not found or not executable at $DRIVER"
    printf '\npassed: %d  failed: %d\n' "$passed" "$failed"
    exit 1
fi

# A decoy whose process NAME is exactly the app binary's, so anything matching
# the real app also matches this. `exec -a` would only change argv[0]; comm
# comes from the executable's own name, so copy a real binary under that name.
DECOY_BIN="$TMPDIR_TEST/codex-desktop"
cp /bin/sleep "$DECOY_BIN"

start_decoy_app() { # $1 = DISPLAY value to give it
    # stdout/stderr must be redirected: this function is called inside a
    # command substitution, which blocks until every descendant holding the
    # captured stdout closes it — a backgrounded decoy that inherits it hangs
    # the caller for the decoy's full lifetime.
    DISPLAY="$1" "$DECOY_BIN" 300 >/dev/null 2>&1 &
    local pid=$!
    DECOY_PIDS+=("$pid")
    # Wait for /proc/<pid>/environ to be readable before asserting on it.
    local i
    for i in $(seq 1 50); do
        [[ -r "/proc/$pid/environ" ]] && break
        sleep 0.1
    done
    echo "$pid"
}

AGENT_ID="drivertest-$$"
export RUN_DESKTOP_AGENT="$AGENT_ID"

# ---------------------------------------------------------------------------
# Case 0: the driver exposes its derived display number.
# Everything below needs to ask the driver which display it owns rather than
# re-deriving the hash itself — a test that duplicates the formula would keep
# passing if the formula changed underneath it.
# ---------------------------------------------------------------------------
DISPLAY_NUM="$("$DRIVER" _display_num 2>/dev/null)"
if [[ "$DISPLAY_NUM" =~ ^[0-9]+$ ]]; then
    ok "driver reports its own display number (_display_num -> :$DISPLAY_NUM)"
else
    nope "driver reports its own display number" \
         "_display_num printed '${DISPLAY_NUM}', expected an integer"
    printf '\npassed: %d  failed: %d\n' "$passed" "$failed"
    exit 1
fi

OTHER_NUM=$(( DISPLAY_NUM + 1 ))

# ---------------------------------------------------------------------------
# Case 1: readiness detection ignores an app process on a DIFFERENT display.
# This is the documented "known gap" that made a sibling agent's running app
# satisfy our readiness poll. When it fires, the window-search budget starts
# counting before our own binary has even been spawned, so the search always
# times out — and the trap then kills everything, which is what produced the
# "binary exits before any window appears" misdiagnosis.
# ---------------------------------------------------------------------------
foreign_pid="$(start_decoy_app ":$OTHER_NUM")"
found="$("$DRIVER" _app_pid 2>/dev/null)"
if [[ -z "$found" ]]; then
    ok "readiness ignores an app process on another agent's display"
else
    nope "readiness ignores an app process on another agent's display" \
         "_app_pid returned '$found' for a process whose DISPLAY is :$OTHER_NUM, not :$DISPLAY_NUM"
fi
kill -9 "$foreign_pid" 2>/dev/null
wait "$foreign_pid" 2>/dev/null

# ---------------------------------------------------------------------------
# Case 2: readiness DOES detect an app process on our own display.
# The negative case above is trivially satisfiable by a helper that never
# matches anything; this is the paired positive that makes it meaningful.
# ---------------------------------------------------------------------------
own_pid="$(start_decoy_app ":$DISPLAY_NUM")"
found="$("$DRIVER" _app_pid 2>/dev/null)"
if [[ "$found" == *"$own_pid"* ]]; then
    ok "readiness detects an app process on our own display"
else
    nope "readiness detects an app process on our own display" \
         "_app_pid returned '$found', expected it to include pid $own_pid"
fi
kill -9 "$own_pid" 2>/dev/null
wait "$own_pid" 2>/dev/null

# ---------------------------------------------------------------------------
# Case 3: `stop` does not reap an unrelated process that merely MENTIONS our
# Xvfb display on its command line.
# `pkill -9 -f "Xvfb :$N "` matches any command line containing that string —
# including the command line of a shell that happens to be running the driver,
# or a wrapper script that names the display in a comment. Observed live: a
# test shell killed itself this way mid-run, producing an empty result with no
# error message and no output at all.
# ---------------------------------------------------------------------------
#
# The decoy must genuinely carry the text on its command line. A first attempt
# used `bash -c "echo 'Xvfb :N ...'; sleep 300"`, which proved nothing: bash
# exec-optimizes the final command of a `-c` string, so the surviving process's
# cmdline was just `sleep 300` and the pattern never matched it. `exec -a` sets
# argv[0] directly, which is exactly the shape of the real incident.
bash -c "exec -a 'Xvfb :$DISPLAY_NUM bystander-not-an-x-server' sleep 300" >/dev/null 2>&1 &
bystander_pid=$!
DECOY_PIDS+=("$bystander_pid")
sleep 0.5

"$DRIVER" stop >/dev/null 2>&1

if is_alive "$bystander_pid"; then
    ok "stop leaves an unrelated process that merely names our display alive"
else
    nope "stop leaves an unrelated process that merely names our display alive" \
         "pid $bystander_pid was killed by 'driver.sh stop'; its command line only mentioned 'Xvfb :$DISPLAY_NUM '"
fi
kill -9 "$bystander_pid" 2>/dev/null
wait "$bystander_pid" 2>/dev/null

# ---------------------------------------------------------------------------
# Case 4: the failure path can report what it saw, and reports it BEFORE any
# cleanup. `_diagnose` is what the window-search timeout must call; if it
# cannot enumerate windows and app liveness, the timeout message is once again
# an unfalsifiable "no window appeared" and the next agent re-derives the wrong
# root cause from an empty process table.
# ---------------------------------------------------------------------------
diag="$("$DRIVER" _diagnose 2>&1)"
if [[ "$diag" == *"app process"* && "$diag" == *"window"* ]]; then
    ok "_diagnose reports app-process liveness and window inventory"
else
    nope "_diagnose reports app-process liveness and window inventory" \
         "output did not mention both 'app process' and 'window': ${diag:0:300}"
fi

# ---------------------------------------------------------------------------
# Case 5: the window-search budget is generous enough for a cold WebKitGTK
# start. Measured on this box, idle and solo: the "Codex" window appeared
# ~35s after the binary started. The historical budget was 90 iterations of
# `sleep 0.5` = 45s, which leaves no headroom at all on a loaded box — and run
# 1 loaded this box to a load average of 9-12 with six concurrent agents.
# ---------------------------------------------------------------------------
budget="$("$DRIVER" _window_timeout 2>/dev/null)"
if [[ "$budget" =~ ^[0-9]+$ ]] && (( budget >= 120 )); then
    ok "window-search budget is at least 120s (is ${budget}s)"
else
    nope "window-search budget is at least 120s" \
         "_window_timeout printed '${budget}'; a cold WebKitGTK start measured ~35s idle and run 1 ran 6 agents on 4 cores"
fi

printf '\npassed: %d  failed: %d\n' "$passed" "$failed"
(( failed == 0 ))
