#!/usr/bin/env bash
#
# verify.sh — the single verification command for this repo.
#
# WHY THIS EXISTS
# ---------------
# Verification used to be composed ad-hoc by whoever was running it, so every
# run had different gaps. Four distinct ways this repo shipped broken while a
# verification run reported green, all of them structural rather than careless:
#
#   1. `apps/desktop/src-tauri` is a SEPARATE cargo crate. The repo root has no
#      `[workspace]` table, so a root-level sweep never touches it. It shipped
#      un-compilable, twice. It also has no lib target, so `cargo test --lib`
#      fails there outright — plain `cargo test` is the correct command.
#   2. `cargo test` fail-fasts. One failure meant 124 of 488 suites ran and the
#      output still looked like a completed run. `--no-fail-fast` is mandatory,
#      and the number of suites actually executed is checked, not just the
#      summary line.
#   3. Piping a command to `grep`/`tail` yields the PIPE's exit status, not the
#      command's. That produced a false green on a full sweep that had failed.
#      Nothing in this script pipes a verified command. Every command writes to
#      a log file and its status is captured on the very next line.
#   4. The frontend test runner reports `0/0 test files passed.` and exits 0
#      when `node_modules` is absent. Install state is checked and repaired
#      before the suite runs, and a zero file count is a hard failure.
#
# EXIT-CODE DISCIPLINE
# --------------------
# `set -e` is deliberately NOT used: every stage must run so the summary is
# complete, and a stage's failure must be recorded rather than abort the run.
# `pipefail` is set as a belt-and-braces measure, but no verified command is
# ever piped in the first place. The single pattern used everywhere is:
#
#     ( cd "$dir" && exec cmd... ) >"$log" 2>&1
#     status=$?          # <- captured directly, first statement after the command
#
# Log parsing happens AFTER the status is captured and can only turn a pass
# into a fail (an extra assertion), never a fail into a pass.
#
# USAGE
# -----
#   scripts/verify.sh                  # full gate (slow: builds ~490 test binaries)
#   scripts/verify.sh --quick          # fast subset, no full sweep, no clippy
#   scripts/verify.sh --only clippy    # one stage (repeatable)
#   scripts/verify.sh --list           # list stages and which set they're in
#   scripts/verify.sh --show-actuals   # also print measured numbers in baseline format
#   scripts/verify.sh -j 4             # cargo build parallelism (default 2)
#
# RETROSPECTIVE EVENT
# -------------------
# Every run emits one `verification` event to the retrospective log (see
# `scripts/retro.py` and `docs/retro/schema.json`), pass or fail. It is emitted
# here rather than typed by whoever ran the command because a run nobody chose
# to record is exactly the one worth having: the stage that failed, got fixed
# in two minutes, and would never have seemed worth writing down. It is also
# what makes the denominator honest — a near-miss count means nothing without
# the number of runs behind it.
#
# The emission cannot affect this script's result. It happens after the
# summary, its status is discarded, and `RETRO_DISABLE=1` turns it off.
#
# Baselines live in scripts/verify-baselines.env, which documents how each
# number is compared and how to re-measure it deliberately.
#
# Exit status: 0 only if every selected stage passed. Non-zero otherwise.

set -uo pipefail

# ---------------------------------------------------------------------------
# Location
# ---------------------------------------------------------------------------

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
REPO_ROOT=$(cd -- "$SCRIPT_DIR/.." && pwd)
DESKTOP_DIR="$REPO_ROOT/apps/desktop"
TAURI_DIR="$DESKTOP_DIR/src-tauri"
BASELINES_FILE="$SCRIPT_DIR/verify-baselines.env"

if [[ ! -f "$BASELINES_FILE" ]]; then
    printf 'verify.sh: missing baselines file: %s\n' "$BASELINES_FILE" >&2
    exit 2
fi
# shellcheck source=verify-baselines.env
. "$BASELINES_FILE"

# ---------------------------------------------------------------------------
# Options
# ---------------------------------------------------------------------------

JOBS=2
MODE=full
SHOW_ACTUALS=0
ONLY_STAGES=()

# Stage sets. `quick` is everything that does not build the ~490-binary root
# sweep or run clippy over the whole tree — on a 4-core box those two dominate.
# `preflight-disk` is first in BOTH sets deliberately: disk exhaustion is this
# repo's #2 recorded incident class (docs/retro/tranche-7-retrospective.md
# §4.1, 5 of 34) and a ~490-binary root-full build is exactly what tips a box
# over — it must fail loudly before that build starts, not be discovered by
# `ld terminated with signal 7 [Bus error]` partway through it.
ALL_STAGES=(preflight-disk audit-selftest root-lib root-full desktop reach frontend-install frontend-test frontend-typecheck clippy class-dump)
QUICK_STAGES=(preflight-disk audit-selftest root-lib reach frontend-install frontend-test frontend-typecheck class-dump)

usage() {
    sed -n '3,48p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --quick)        MODE=quick ;;
        --full)         MODE=full ;;
        --show-actuals) SHOW_ACTUALS=1 ;;
        --only)
            [[ $# -ge 2 ]] || { printf 'verify.sh: --only needs a stage name\n' >&2; exit 2; }
            ONLY_STAGES+=("$2"); shift ;;
        -j)
            [[ $# -ge 2 ]] || { printf 'verify.sh: -j needs a number\n' >&2; exit 2; }
            JOBS="$2"; shift ;;
        --list)
            printf 'stage                full  quick\n'
            for stage in "${ALL_STAGES[@]}"; do
                in_quick=no
                for q in "${QUICK_STAGES[@]}"; do [[ "$q" == "$stage" ]] && in_quick=yes; done
                printf '%-20s yes   %s\n' "$stage" "$in_quick"
            done
            exit 0 ;;
        -h|--help)      usage; exit 0 ;;
        *)              printf 'verify.sh: unknown argument: %s\n' "$1" >&2; exit 2 ;;
    esac
    shift
done

if [[ ${#ONLY_STAGES[@]} -gt 0 ]]; then
    SELECTED=("${ONLY_STAGES[@]}")
    for stage in "${SELECTED[@]}"; do
        known=no
        for s in "${ALL_STAGES[@]}"; do [[ "$s" == "$stage" ]] && known=yes; done
        [[ "$known" == yes ]] || { printf 'verify.sh: unknown stage: %s (see --list)\n' "$stage" >&2; exit 2; }
    done
elif [[ "$MODE" == quick ]]; then
    SELECTED=("${QUICK_STAGES[@]}")
else
    SELECTED=("${ALL_STAGES[@]}")
fi

# ---------------------------------------------------------------------------
# Logs
#
# A worktree-unique scratch dir so concurrent agents on sibling worktrees never
# read each other's logs. Kept on failure so the real output is inspectable;
# the path is always printed.
# ---------------------------------------------------------------------------

LOG_DIR=${VERIFY_LOG_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/codex-verify-XXXXXX")}
mkdir -p "$LOG_DIR"

# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------

PASS_NAMES=()
FAIL_NAMES=()
declare -a NOTES=()
declare -a ACTUALS=()

note()   { NOTES+=("$1"); }
actual() { ACTUALS+=("$1"); }

say() { printf '%s\n' "$*"; }
hr()  { printf -- '---------------------------------------------------------------\n'; }

stage_start() {
    hr
    printf '==> %s\n' "$1"
}

stage_pass() { PASS_NAMES+=("$1"); printf '    PASS  %s%s\n' "$1" "${2:+  ($2)}"; }
stage_fail() { FAIL_NAMES+=("$1"); printf '    FAIL  %s%s\n' "$1" "${2:+  ($2)}"; }

# Sums cargo's `test result: ok. N passed;` lines in a log.
# Pure log parsing — never consulted to decide pass/fail on its own.
count_passed() {
    awk '/^test result:/ { for (i = 1; i <= NF; i++) if ($(i+1) == "passed;") total += $i } END { print total + 0 }' "$1"
}

# Counts lines matching a pattern, ALWAYS emitting exactly one integer.
#
# `grep -c` prints its count and then exits 1 when the count is zero, so the
# obvious `$(grep -c ... || echo 0)` emits "0\n0" — two fields where the caller
# expects one. That silently truncated a `read` in this very script and left a
# clippy warning count empty, which then compared as 0 and passed. Non-numeric
# output is reported as the empty string so callers can tell "no matches" from
# "the count could not be taken" instead of conflating them as zero.
count_matching() {
    local pattern="$1" file="$2" n
    [[ -f "$file" ]] || { printf ''; return; }
    n=$(grep -c "$pattern" "$file" 2>/dev/null)
    if [[ "$n" =~ ^[0-9]+$ ]]; then
        printf '%s' "$n"
    else
        printf ''
    fi
}

# Counts the test binaries cargo actually executed.
count_running() {
    local n; n=$(count_matching '^[[:space:]]*Running ' "$1")
    printf '%s' "${n:-0}"
}

# Enforces a FLOOR. Passing when actual >= baseline; notes a stale baseline
# when actual > baseline so the recorded number gets updated deliberately.
check_floor() {
    local label="$1" actual_n="$2" baseline_n="$3" var="$4"
    actual "$var=$actual_n"
    if (( actual_n < baseline_n )); then
        printf '    %s: %s, baseline floor is %s — tests were LOST\n' "$label" "$actual_n" "$baseline_n"
        return 1
    fi
    if (( actual_n > baseline_n )); then
        note "$var baseline is stale: $baseline_n recorded, $actual_n measured. Update $BASELINES_FILE."
    fi
    return 0
}

# ---------------------------------------------------------------------------
# Stage: preflight-disk
#
# Fires FIRST in both stage sets, before any build starts. `root-full` builds
# ~490 test binaries and is exactly what has repeatedly tipped this box over
# (docs/retro/tranche-7-retrospective.md §4.1: `/tmp` tmpfs at 91% -> `ld
# terminated with signal 7 [Bus error]`; `/` at 91%, 98%, 98%; `/home` at
# 100%, 0 bytes available). The disk-pressure event this script already emits
# at the very end (`emit_disk_pressure_event` below) only ever *records* that
# after the fact — it cannot stop a build already in flight from failing on
# ENOSPC/SIGBUS partway through. This stage is the check that runs before any
# of that time is spent, checking both the repo's own filesystem and the
# filesystem the scratch log dir lives on (mktemp defaults to /tmp, a
# tmpfs — the exact partition that hit 91% and produced the Bus error above).
# ---------------------------------------------------------------------------

PREFLIGHT_DISK_MIN_FREE_GB=${PREFLIGHT_DISK_MIN_FREE_GB:-20}
PREFLIGHT_DISK_MAX_PERCENT=${PREFLIGHT_DISK_MAX_PERCENT:-90}

# Prints "used_pct avail_kb mount" for the filesystem containing `path`, or
# nothing if `df` can't be read. Never piped to the caller's exit-status path.
df_stats_for() {
    local path="$1" line
    line=$(df -Pk "$path" 2>/dev/null | awk 'NR==2 { gsub(/%/, "", $5); print $5, $4, $6 }')
    printf '%s' "$line"
}

check_disk_budget() {
    local label="$1" path="$2" used avail_kb mount avail_gb
    read -r used avail_kb mount < <(df_stats_for "$path")
    if [[ ! "$used" =~ ^[0-9]+$ || ! "$avail_kb" =~ ^[0-9]+$ ]]; then
        printf '    %s (%s, mounted at %s): could not read df output — skipping this check\n' \
            "$label" "$path" "${mount:-?}"
        return 0
    fi
    avail_gb=$(( avail_kb / 1024 / 1024 ))
    printf '    %s (%s, mounted at %s): %s%% used, %sG available\n' \
        "$label" "$path" "$mount" "$used" "$avail_gb"
    if (( used >= PREFLIGHT_DISK_MAX_PERCENT || avail_gb < PREFLIGHT_DISK_MIN_FREE_GB )); then
        return 1
    fi
    return 0
}

run_preflight_disk() {
    stage_start "preflight-disk — disk budget check before any build starts"
    local ok=0

    check_disk_budget "repo filesystem" "$REPO_ROOT" || ok=1
    # LOG_DIR (see "Logs" above) is already created by the time any stage
    # runs — check the filesystem it actually landed on, which is /tmp by
    # default (a tmpfs, and the exact partition that hit 91% and produced
    # the Bus error this stage exists to head off).
    check_disk_budget "scratch-log filesystem" "$LOG_DIR" || ok=1

    if (( ok != 0 )); then
        printf '    FAIL: disk budget below floor (max %s%% used, min %sG free).\n' \
            "$PREFLIGHT_DISK_MAX_PERCENT" "$PREFLIGHT_DISK_MIN_FREE_GB"
        printf '    A full sweep builds ~490 test binaries and needs real headroom; this is\n'
        printf '    exactly the condition that produced "ld terminated with signal 7 [Bus\n'
        printf '    error]" and aborted builds in this repo before (tranche-7-retrospective\n'
        printf '    §4.1). Reclaim space before re-running:\n'
        printf '        scripts/reclaim.sh --apply\n'
        printf '    (dry-run first with no flags to see what it would remove). Override the\n'
        printf '    floor deliberately with PREFLIGHT_DISK_MIN_FREE_GB / PREFLIGHT_DISK_MAX_PERCENT\n'
        printf '    if this box'"'"'s real headroom is genuinely different.\n'
        stage_fail preflight-disk "below the disk budget floor — see scripts/reclaim.sh"
        return
    fi
    stage_pass preflight-disk "disk budget OK"
}

# ---------------------------------------------------------------------------
# Stage: root crate lib tests
# ---------------------------------------------------------------------------

run_root_lib() {
    stage_start "root-lib — cargo test --locked --lib -j $JOBS  (repo root)"
    local log="$LOG_DIR/root-lib.log"
    ( cd "$REPO_ROOT" && exec cargo test --locked --lib -j "$JOBS" ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail root-lib "cargo exit $status — $log"
        return
    fi
    local passed; passed=$(count_passed "$log")
    if ! check_floor "root lib tests" "$passed" "$BASELINE_ROOT_LIB_TESTS" BASELINE_ROOT_LIB_TESTS; then
        stage_fail root-lib "$passed passed, floor $BASELINE_ROOT_LIB_TESTS — $log"
        return
    fi
    stage_pass root-lib "$passed passed"
}

# ---------------------------------------------------------------------------
# Stage: root crate full sweep
#
# --no-fail-fast is the whole point: without it a single failure stops the run
# and the summary still reads like a completed pass. The executed-binary count
# is checked for the same reason — a run can be green and still have skipped
# most of the suite.
#
# The aggregate counts above (total passed, total binaries) cannot catch one
# specific suite being silently dropped from execution: a suite disappearing
# and a different suite appearing in the same run holds both numbers steady.
# That is exactly what happened for a full tranche
# (docs/retro/events/tranche8-incident-retro.jsonl, 2026-08-01): root-full was
# RED on 29 of 33 runs, always attributed to the same "environmental"
# fixture bucket, and that normalized red concealed two parity suites that
# never executed once across the whole tranche while the aggregate pass/
# binary counts looked unremarkable. `expected_test_suites`/
# `executed_test_suites` below name the gap directly instead of hoping a
# floor on a total catches it.
#
# The expected suite list is DERIVED from the filesystem — every top-level
# `tests/*.rs` file is one cargo integration-test binary by cargo's own
# auto-discovery convention (subdirectories like tests/fixtures and
# tests/sd16-e5-f1 are not auto-discovered, so `-maxdepth 1` already excludes
# them correctly) — never hand-maintained. A hand-kept list of "critical"
# suites rots exactly like the roster and allowlist failures already in this
# log; this one can't drift because it IS the filesystem at check time.
# ---------------------------------------------------------------------------

expected_test_suites() {
    find "$REPO_ROOT/tests" -maxdepth 1 -name '*.rs' -printf '%f\n' 2>/dev/null \
        | sed 's/\.rs$//' | sort
}

# Cargo prints "     Running tests/<name>.rs (target/.../deps/<name>-<hash>)"
# for every integration-test binary it actually runs, name included in the
# line itself (verified against this repo's own cargo output before relying
# on it). Sorted+uniqued so a suite run under `--test-threads` retries once
# still diffs cleanly.
executed_test_suites() {
    grep -E '^[[:space:]]*Running tests/' "$1" 2>/dev/null \
        | sed -E 's#^[[:space:]]*Running tests/([^[:space:]]+)\.rs.*#\1#' \
        | sort -u
}

run_root_full() {
    stage_start "root-full — cargo test --locked --no-fail-fast -j $JOBS  (repo root)"
    say "    building ~490 test binaries; this is the slow one"
    local log="$LOG_DIR/root-full.log"
    ( cd "$REPO_ROOT" && exec cargo test --locked --no-fail-fast -j "$JOBS" ) >"$log" 2>&1
    local status=$?

    local passed binaries
    passed=$(count_passed "$log")
    binaries=$(count_running "$log")

    local missing missing_n
    missing=$(comm -23 <(expected_test_suites) <(executed_test_suites "$log"))
    missing_n=0
    [[ -n "$missing" ]] && missing_n=$(printf '%s\n' "$missing" | grep -c .)

    if (( status != 0 )); then
        stage_fail root-full "cargo exit $status; $passed passed across $binaries suites — $log"
        return
    fi

    local ok=0
    check_floor "root full tests" "$passed" "$BASELINE_ROOT_FULL_TESTS" BASELINE_ROOT_FULL_TESTS || ok=1
    check_floor "root test binaries executed" "$binaries" "$BASELINE_ROOT_TEST_BINARIES" BASELINE_ROOT_TEST_BINARIES || ok=1
    if (( missing_n > 0 )); then
        printf '    %s tests/*.rs file(s) present but NEVER EXECUTED (no "Running" line in the log): %s\n' \
            "$missing_n" "$(printf '%s' "$missing" | tr '\n' ' ')"
        ok=1
    fi
    if (( ok != 0 )); then
        stage_fail root-full "$passed passed across $binaries suites, $missing_n suite(s) never ran — $log"
        return
    fi
    stage_pass root-full "$passed passed across $binaries suites, all $(expected_test_suites | grep -c .) tests/*.rs suites executed"
}

# ---------------------------------------------------------------------------
# Stage: desktop crate
#
# Separate crate, separate Cargo.lock, separate invocation. Plain `cargo test`
# — NOT `--lib`: this crate is bin-only and `--lib` errors out with
# "no library targets found", which is easy to misread as a clean run.
# ---------------------------------------------------------------------------

run_desktop() {
    stage_start "desktop — cargo test --locked -j $JOBS  (apps/desktop/src-tauri)"
    local log="$LOG_DIR/desktop.log"
    ( cd "$TAURI_DIR" && exec cargo test --locked -j "$JOBS" ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail desktop "cargo exit $status — $log"
        return
    fi
    local passed; passed=$(count_passed "$log")
    if ! check_floor "desktop tests" "$passed" "$BASELINE_DESKTOP_TESTS" BASELINE_DESKTOP_TESTS; then
        stage_fail desktop "$passed passed, floor $BASELINE_DESKTOP_TESTS — $log"
        return
    fi
    stage_pass desktop "$passed passed"
}

# ---------------------------------------------------------------------------
# Stage: reach gate
#
# The desktop crate's content-reach suite, run on its own so --quick still
# covers it. Every ingested (book, content-kind) pair must have a verified
# consumer carrying real payload across the IPC boundary; see
# apps/desktop/src-tauri/src/reach_gate.rs.
# ---------------------------------------------------------------------------

run_reach() {
    stage_start "reach — cargo test --locked -j $JOBS reach_gate  (apps/desktop/src-tauri)"
    local log="$LOG_DIR/reach.log"
    ( cd "$TAURI_DIR" && exec cargo test --locked -j "$JOBS" reach_gate ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail reach "cargo exit $status — $log"
        return
    fi
    local passed; passed=$(count_passed "$log")
    # A reach gate that runs zero tests asserts nothing. This repo has already
    # shipped tests that passed while asserting nothing; a filter that matches
    # no test name would reproduce that exactly.
    if (( passed == 0 )); then
        stage_fail reach "0 tests matched the reach_gate filter — the gate is not running at all — $log"
        return
    fi
    stage_pass reach "$passed passed"
}

# ---------------------------------------------------------------------------
# Stage: frontend install
#
# apps/desktop/scripts/run-tests.mjs spawns each test file through
# node_modules/.bin/tsx. With node_modules absent the glob still runs, finds
# its files, and every spawn fails — or, worse, an empty tree reports
# "0/0 test files passed." Either way the suite looks harmless. Repair first.
# ---------------------------------------------------------------------------

run_frontend_install() {
    stage_start "frontend-install — npm ci if node_modules is absent  (apps/desktop)"
    if [[ -x "$DESKTOP_DIR/node_modules/.bin/tsx" ]]; then
        stage_pass frontend-install "node_modules present"
        return
    fi
    local log="$LOG_DIR/frontend-install.log"
    say "    node_modules/.bin/tsx missing — running npm ci"
    ( cd "$DESKTOP_DIR" && exec npm ci ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail frontend-install "npm ci exit $status — $log"
        return
    fi
    if [[ ! -x "$DESKTOP_DIR/node_modules/.bin/tsx" ]]; then
        stage_fail frontend-install "npm ci succeeded but tsx is still missing — $log"
        return
    fi
    stage_pass frontend-install "npm ci"
}

# ---------------------------------------------------------------------------
# Stage: frontend tests
# ---------------------------------------------------------------------------

run_frontend_test() {
    stage_start "frontend-test — npm test  (apps/desktop)"
    local log="$LOG_DIR/frontend-test.log"
    ( cd "$DESKTOP_DIR" && exec npm test ) >"$log" 2>&1
    local status=$?

    # `<n>/<total> test files passed.` is the runner's own summary line.
    # Extracted with grep -o + parameter expansion rather than a GNU-awk
    # capture group, so this does not silently report 0/0 under mawk.
    local summary ran total rest
    summary=$(grep -o '[0-9][0-9]*/[0-9][0-9]* test files passed\.' "$log" 2>/dev/null | tail -1)
    ran=${summary%%/*}
    rest=${summary#*/}
    total=${rest%% *}
    ran=${ran:-0}
    total=${total:-0}

    if (( status != 0 )); then
        stage_fail frontend-test "npm test exit $status; $ran/$total files passed — $log"
        return
    fi
    # Zero files is the false-green case, and it exits 0 in some shapes. It is
    # never a legitimate result for this repo.
    if (( total == 0 )); then
        stage_fail frontend-test "0 test files discovered — the suite did not actually run — $log"
        return
    fi
    if (( ran != total )); then
        stage_fail frontend-test "$ran of $total files passed — $log"
        return
    fi
    if ! check_floor "frontend test files" "$total" "$BASELINE_FRONTEND_TEST_FILES" BASELINE_FRONTEND_TEST_FILES; then
        stage_fail frontend-test "$total files, floor $BASELINE_FRONTEND_TEST_FILES — $log"
        return
    fi
    stage_pass frontend-test "$ran/$total files"
}

# ---------------------------------------------------------------------------
# Stage: frontend typecheck
# ---------------------------------------------------------------------------

run_frontend_typecheck() {
    stage_start "frontend-typecheck — npm run typecheck  (apps/desktop)"
    local log="$LOG_DIR/frontend-typecheck.log"
    ( cd "$DESKTOP_DIR" && exec npm run typecheck ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail frontend-typecheck "tsc exit $status — $log"
        return
    fi
    stage_pass frontend-typecheck "tsc --noEmit clean"
}

# ---------------------------------------------------------------------------
# Stage: clippy
#
# Compared against the RECORDED baseline rather than a hardcoded literal, and
# counted from cargo's real diagnostic lines. Cargo emits a per-target summary
# ("`codex` (lib) generated N warnings") in the same `warning:` shape, so those
# are excluded — counting them would inflate the total and let real new lint
# debt hide under an inflated ceiling.
# ---------------------------------------------------------------------------

# Lints one crate. Writes "status errors warnings log" on ONE line for the
# caller; makes no pass/fail decision itself. `errors` or `warnings` come back
# empty when the count could not be taken, which the caller treats as a
# failure rather than as zero.
clippy_one_crate() {
    local label="$1" dir="$2"
    local log="$LOG_DIR/clippy-$label.log"
    ( cd "$dir" && exec cargo clippy --locked --tests -j "$JOBS" ) >"$log" 2>&1
    local status=$?

    local errors warnings
    errors=$(count_matching '^error' "$log")
    # Cargo emits a per-target summary ("`codex` (lib) generated N warnings")
    # in the same `warning:` shape as a real diagnostic. Counting those would
    # inflate the total and let genuinely new lint debt hide under an inflated
    # ceiling, so they are excluded. Done with a temp file rather than a pipe
    # so the count is a single clean integer.
    local filtered="$log.diagnostics"
    grep '^warning:' "$log" 2>/dev/null | grep -v 'generated [0-9]* warning' >"$filtered" 2>/dev/null
    warnings=$(wc -l <"$filtered" 2>/dev/null | tr -d ' ')
    [[ "$warnings" =~ ^[0-9]+$ ]] || warnings=""

    printf '%s %s %s %s' "$status" "${errors:-NaN}" "${warnings:-NaN}" "$log"
}

run_clippy() {
    stage_start "clippy — cargo clippy --locked --tests -j $JOBS  (BOTH crates)"
    local ok=0 summary=()

    # Both crates, for the same reason the test stages are split: the root
    # invocation does not reach apps/desktop/src-tauri at all.
    local names=(root desktop)
    local dirs=("$REPO_ROOT" "$TAURI_DIR")
    local ceilings=("$BASELINE_CLIPPY_WARNINGS_ROOT" "$BASELINE_CLIPPY_WARNINGS_DESKTOP")
    local vars=(BASELINE_CLIPPY_WARNINGS_ROOT BASELINE_CLIPPY_WARNINGS_DESKTOP)

    local i label status errors warnings log
    for i in 0 1; do
        label="${names[$i]}"
        read -r status errors warnings log <<<"$(clippy_one_crate "$label" "${dirs[$i]}")"
        actual "${vars[$i]}=$warnings"
        summary+=("$label:$warnings")

        # A count that could not be taken must FAIL, never be treated as zero.
        # An empty/NaN count compares as 0 in bash arithmetic, which would pass
        # any ceiling — this script shipped exactly that bug once already.
        if ! [[ "$errors" =~ ^[0-9]+$ && "$warnings" =~ ^[0-9]+$ ]]; then
            printf '    %s: could not count clippy output (errors=%s warnings=%s) — %s\n' \
                "$label" "$errors" "$warnings" "$log"
            ok=1; continue
        fi
        if (( status != 0 )); then
            printf '    %s: cargo exit %s — %s\n' "$label" "$status" "$log"
            ok=1; continue
        fi
        if (( errors != 0 )); then
            printf '    %s: %s errors — %s\n' "$label" "$errors" "$log"
            ok=1; continue
        fi
        # CEILING, not a floor: new lint debt fails, paying debt down does not.
        if (( warnings > ${ceilings[$i]} )); then
            printf '    %s: %s warnings exceeds recorded ceiling %s — %s\n' \
                "$label" "$warnings" "${ceilings[$i]}" "$log"
            ok=1; continue
        fi
        if (( warnings < ${ceilings[$i]} )); then
            note "${vars[$i]} ceiling is loose: ${ceilings[$i]} recorded, $warnings measured. Lower it in $BASELINES_FILE."
        fi
    done

    if (( ok != 0 )); then
        stage_fail clippy "${summary[*]} — logs in $LOG_DIR"
        return
    fi
    stage_pass clippy "${summary[*]} warnings, 0 errors"
}

# ---------------------------------------------------------------------------
# Stage: class state dump
#
# Every class must compute at every level. The binary prints one JSON document
# to stdout; `levels_blocked` must be empty for all of them. Parsed with
# python3 (already a hard dependency of this repo's release scripts) rather
# than by grepping, so a shape change in the dump is an error, not a silent
# zero match.
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# Stage: audit-selftest
#
# Runs scripts/tests/test_identifier_discipline_audit.sh — the detection
# self-test for scripts/identifier-discipline-audit.sh.
#
# Why this is a gate stage and not a script somebody remembers to run: the
# audit script's own header records TWO occasions on which the gate passed
# clean over a real planted bundle tag (the misplaced `\b`, and the missing
# `:(glob)` pathspec magic). Both were found by hand, neither by a test. A
# gate whose detection power is untested emits `OK_NO_BUNDLE_TAGS` with the
# same confidence whether it is working or broken — which makes the token
# worthless exactly when it matters. Added 2026-08-10 by SD-29 Epic 1, whose
# acceptance criterion is that this audit "returns 0 findings".
#
# No build, no baseline, seconds to run: it operates on throwaway git repos
# under mktemp, never on this checkout.
# ---------------------------------------------------------------------------

run_audit_selftest() {
    stage_start "audit-selftest — scripts/tests/test_identifier_discipline_audit.sh"
    local log="$LOG_DIR/audit-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_identifier_discipline_audit.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail audit-selftest "self-test script missing at scripts/tests/test_identifier_discipline_audit.sh"
        return
    fi

    bash "$script" >"$log" 2>&1
    local status=$?

    local tally
    tally=$(sed -n 's/^passed: \([0-9]*\)  failed: \([0-9]*\)$/\1 passed, \2 failed/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail audit-selftest "self-test exit $status${tally:+; $tally} — $log"
        return
    fi

    # A self-test that discovers no cases proves nothing — same failure mode
    # the `reach` stage guards with its 0-tests-matched check.
    local passed
    passed=$(sed -n 's/^passed: \([0-9]*\).*$/\1/p' "$log" | tail -1)
    if [[ -z "$passed" || "$passed" -eq 0 ]]; then
        stage_fail audit-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass audit-selftest "${tally:-$passed cases passed}"
}

run_class_dump() {
    stage_start "class-dump — cargo run --locked --bin v06_class_state_dump  (repo root)"
    local log="$LOG_DIR/class-dump.log"
    local json="$LOG_DIR/class-dump.json"
    ( cd "$REPO_ROOT" && exec cargo run --locked --quiet -j "$JOBS" --bin v06_class_state_dump ) >"$json" 2>"$log"
    local status=$?

    if (( status != 0 )); then
        stage_fail class-dump "binary exit $status — $log"
        return
    fi

    local report
    report=$(python3 - "$json" "$BASELINE_COMPUTED_CLASSES" <<'PY'
import json, sys

path, expected = sys.argv[1], int(sys.argv[2])
try:
    with open(path) as handle:
        doc = json.load(handle)
except Exception as exc:                       # shape change must be loud
    print(f"FAIL unparseable dump: {exc}")
    raise SystemExit(0)

classes = doc.get("classes")
if not isinstance(classes, list):
    print("FAIL dump carries no `classes` list")
    raise SystemExit(0)

blocked = [c.get("id", "?") for c in classes if c.get("levels_blocked")]
total = len(classes)

if blocked:
    print(f"FAIL {len(blocked)} of {total} classes blocked: {', '.join(sorted(blocked))}")
elif total < expected:
    print(f"FAIL only {total} classes in the dump, expected at least {expected}")
else:
    print(f"OK {total}/{total} computing")
    print(f"ACTUAL BASELINE_COMPUTED_CLASSES={total}")
PY
)
    local py_status=$?
    if (( py_status != 0 )); then
        stage_fail class-dump "dump parser exit $py_status — $json"
        return
    fi

    local verdict; verdict=$(printf '%s\n' "$report" | head -1)
    local measured; measured=$(printf '%s\n' "$report" | sed -n 's/^ACTUAL //p')
    [[ -n "$measured" ]] && actual "$measured"

    case "$verdict" in
        OK*)   stage_pass class-dump "${verdict#OK }" ;;
        *)     stage_fail class-dump "${verdict#FAIL } — $json" ;;
    esac
}

# ---------------------------------------------------------------------------
# Run
# ---------------------------------------------------------------------------

MODE_LABEL="$MODE"
(( ${#ONLY_STAGES[@]} > 0 )) && MODE_LABEL="--only"
say "codex verify — mode: $MODE_LABEL, jobs: $JOBS"
say "repo:  $REPO_ROOT"
say "logs:  $LOG_DIR"

for stage in "${SELECTED[@]}"; do
    case "$stage" in
        preflight-disk)      run_preflight_disk ;;
        audit-selftest)      run_audit_selftest ;;
        root-lib)            run_root_lib ;;
        root-full)           run_root_full ;;
        desktop)             run_desktop ;;
        reach)               run_reach ;;
        frontend-install)    run_frontend_install ;;
        frontend-test)       run_frontend_test ;;
        frontend-typecheck)  run_frontend_typecheck ;;
        clippy)              run_clippy ;;
        class-dump)          run_class_dump ;;
    esac
done

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------

hr
say "SUMMARY"
say "  passed:  ${#PASS_NAMES[@]}  ${PASS_NAMES[*]:-}"
if (( ${#FAIL_NAMES[@]} > 0 )); then
    say "  FAILED:  ${#FAIL_NAMES[@]}  ${FAIL_NAMES[*]}"
fi

if (( ${#NOTES[@]} > 0 )); then
    say ""
    say "BASELINE NOTES (not failures — update deliberately):"
    for n in "${NOTES[@]}"; do say "  - $n"; done
fi

if (( SHOW_ACTUALS == 1 )); then
    say ""
    say "MEASURED (scripts/verify-baselines.env format):"
    for a in "${ACTUALS[@]}"; do say "  $a"; done
fi

# ---------------------------------------------------------------------------
# Retrospective event
#
# Runs after every stage has finished and its own status is discarded, so it
# is incapable of turning a FAIL into a PASS or the reverse — the one property
# that matters, given point 3 above. Every way it could go wrong (no python3,
# no emitter, a bad schema) collapses to a no-op: a missing event is a gap in
# a retrospective, whereas a verify.sh that fails because its logging failed
# would be a gate nobody trusts.
# ---------------------------------------------------------------------------

RESULT=PASS
(( ${#FAIL_NAMES[@]} > 0 )) && RESULT=FAIL

emit_retro_event() {
    [[ -z "${RETRO_DISABLE:-}" ]] || return 0
    local emitter="$SCRIPT_DIR/retro.py"
    [[ -f "$emitter" ]] || return 0
    command -v python3 >/dev/null 2>&1 || return 0

    local passed_csv="" failed_csv=""
    (( ${#PASS_NAMES[@]} > 0 )) && passed_csv=$(IFS=,; printf '%s' "${PASS_NAMES[*]}")
    (( ${#FAIL_NAMES[@]} > 0 )) && failed_csv=$(IFS=,; printf '%s' "${FAIL_NAMES[*]}")

    # `--mode=` and not `--mode `: MODE_LABEL is literally "--only" for an
    # --only run, and a separate argument beginning with a dash is read as the
    # next flag rather than as this one's value. The whole emission then fails
    # with a usage error that the `|| true` below swallows in silence.
    local args=(
        "$emitter" verification
        --source verify.sh
        --derived
        --mode="$MODE_LABEL"
        --result "$RESULT"
        --log-dir "$LOG_DIR"
        --duration-seconds "$SECONDS"
        --quiet
    )
    [[ -n "$passed_csv" ]] && args+=(--stages-passed "$passed_csv")
    [[ -n "$failed_csv" ]] && args+=(--stages-failed "$failed_csv")

    python3 "${args[@]}" >/dev/null 2>&1 || true
}

# Disk pressure, observed at the moment it is most likely to be true.
#
# The disk has hit 100% twice on this box, once livelocking a partition
# resize, and a full test sweep building ~490 binaries is the single largest
# consumer on it. Nobody remembers to record that afterwards, and by the time
# anyone looks the space has usually been reclaimed — so the one honest place
# to notice is here, right after the build that caused it.
#
# `dedupe_key` is per day per filesystem: a hundred verify runs on a bad day
# produce one event, not a hundred, so the count means "days under pressure"
# rather than "times anyone happened to run the gate".
emit_disk_pressure_event() {
    [[ -z "${RETRO_DISABLE:-}" ]] || return 0
    local emitter="$SCRIPT_DIR/retro.py"
    [[ -f "$emitter" ]] || return 0
    command -v python3 >/dev/null 2>&1 || return 0
    command -v df >/dev/null 2>&1 || return 0

    local threshold=${RETRO_DISK_THRESHOLD:-90}
    local used_pct mount
    read -r used_pct mount < <(df -P "$REPO_ROOT" 2>/dev/null | awk 'NR==2 { gsub(/%/, "", $5); print $5, $6 }')
    [[ "$used_pct" =~ ^[0-9]+$ ]] || return 0
    (( used_pct >= threshold )) || return 0

    python3 "$emitter" incident \
        --source verify.sh --derived \
        --impact "$mount at ${used_pct}% used after a verify run" \
        --detected-by "df, at the end of scripts/verify.sh" \
        --recurrence-key disk-pressure \
        --dedupe-key "disk-pressure:$(date -u +%Y-%m-%d):$mount" \
        --used-percent "$used_pct" \
        --quiet >/dev/null 2>&1 || true
}

emit_retro_event
emit_disk_pressure_event

# Only the full gate is a complete verification. Say so, so a --quick pass is
# never mistaken for one.
say ""
if (( ${#FAIL_NAMES[@]} > 0 )); then
    say "RESULT: FAIL — logs in $LOG_DIR"
    exit 1
fi
if [[ "$MODE" == quick && ${#ONLY_STAGES[@]} -eq 0 ]]; then
    say "RESULT: PASS (--quick subset only; run without --quick before merging)"
else
    say "RESULT: PASS"
fi
say "logs in $LOG_DIR"
exit 0
