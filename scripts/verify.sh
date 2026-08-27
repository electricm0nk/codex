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
PIN_FILE="$SCRIPT_DIR/pcgen-oracle-pin.env"

if [[ ! -f "$BASELINES_FILE" ]]; then
    printf 'verify.sh: missing baselines file: %s\n' "$BASELINES_FILE" >&2
    exit 2
fi
# shellcheck source=verify-baselines.env
. "$BASELINES_FILE"

if [[ ! -f "$PIN_FILE" ]]; then
    printf 'verify.sh: missing PCGen oracle pin file: %s\n' "$PIN_FILE" >&2
    exit 2
fi
# shellcheck source=pcgen-oracle-pin.env
. "$PIN_FILE"

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
ALL_STAGES=(preflight-disk preflight-oracle oracle-pin-selftest producer-selftest pi-redaction-selftest provenance-selftest site-dashboard-selftest site-dashboard-check site-dashboard-pi-gate build-public-status-selftest site-public-status-check site-public-status-pi-gate site-asset-stamp-check reachability-audit-selftest reachability-audit groundtruth-guard-selftest supersession-gate-selftest shape-coverage-standing-gate-selftest shape-coverage-standing-gate denominator-gate figure-provenance pi-sweep declared-pi-audit audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop reach corpus-sweep supersession-gate frontend-install frontend-test frontend-typecheck clippy class-dump)
QUICK_STAGES=(preflight-disk preflight-oracle oracle-pin-selftest producer-selftest pi-redaction-selftest provenance-selftest site-dashboard-selftest site-dashboard-check site-dashboard-pi-gate build-public-status-selftest site-public-status-check site-public-status-pi-gate site-asset-stamp-check reachability-audit-selftest reachability-audit groundtruth-guard-selftest supersession-gate-selftest shape-coverage-standing-gate-selftest shape-coverage-standing-gate denominator-gate figure-provenance pi-sweep declared-pi-audit audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib reach frontend-install frontend-test frontend-typecheck class-dump)

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
# Stage: preflight-oracle
#
# Runs `scripts/fetch-pcgen-oracle.sh --check` — never touches the network.
# The `corpus-sweep` stage below already sha256-checks every CITED corpus
# file, but only in the FULL gate, only after a cargo build, and it names
# what changed rather than what to run to fix it. This stage is the cheap,
# build-free, whole-cone check that runs first and prints the exact fetch
# command when the oracle is absent or off-pin — placed in BOTH stage sets
# (right after preflight-disk) because a `--quick` run with no
# `PCGEN_CORPUS_ROOT` set does not fail today: corpus-gated tests print
# `skipping: no PCGEN_CORPUS_ROOT...` and pass (e.g. `tests/sd17_b5_equipment.rs:501`),
# so a corpus-less quick run is a weaker green with nothing saying so.
# ---------------------------------------------------------------------------

run_preflight_oracle() {
    stage_start "preflight-oracle — scripts/fetch-pcgen-oracle.sh --check"
    local log="$LOG_DIR/preflight-oracle.log"

    ( cd "$REPO_ROOT" && exec bash scripts/fetch-pcgen-oracle.sh --check ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        printf '    FAIL: the PCGen oracle is absent or off-pin. Remediation from the log:\n'
        sed 's/^/        /' "$log"
        stage_fail preflight-oracle "exit $status — $log"
        return
    fi

    # Exit 0 without the OK token means the script took a path nobody
    # intended — the same "exit 0 without CLEAN" guard corpus-sweep and
    # pi-sweep already carry.
    if ! grep -q '^pcgen-oracle: OK' "$log"; then
        stage_fail preflight-oracle "exited 0 without the pcgen-oracle: OK token — $log"
        return
    fi

    local sha
    sha=$(sed -n 's/^pcgen-oracle: OK \([0-9a-f]*\).*$/\1/p' "$log" | tail -1)
    actual "PCGEN_ORACLE_SHA=${sha:-unknown}"

    stage_pass preflight-oracle "oracle at pin ${sha:-$PCGEN_ORACLE_SHA}"
}

# ---------------------------------------------------------------------------
# Stage: oracle-pin-selftest
#
# Mirrors corpus-sweep-selftest verbatim: runs
# scripts/tests/test_fetch_pcgen_oracle.sh, the detection self-test for
# fetch-pcgen-oracle.sh. Cheap — a git init in mktemp, no build, no real
# network — and it never reads the real PCGen checkout.
# ---------------------------------------------------------------------------

run_oracle_pin_selftest() {
    stage_start "oracle-pin-selftest — scripts/tests/test_fetch_pcgen_oracle.sh"
    local log="$LOG_DIR/oracle-pin-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_fetch_pcgen_oracle.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail oracle-pin-selftest "self-test script missing at scripts/tests/test_fetch_pcgen_oracle.sh"
        return
    fi

    bash "$script" >"$log" 2>&1
    local status=$?

    local tally
    tally=$(sed -n 's/^passed: \([0-9]*\)  failed: \([0-9]*\)$/\1 passed, \2 failed/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail oracle-pin-selftest "self-test exit $status${tally:+; $tally} — $log"
        return
    fi

    # A self-test that discovers no cases proves nothing — the same guard
    # every other selftest stage in this script carries.
    local passed
    passed=$(sed -n 's/^passed: \([0-9]*\).*$/\1/p' "$log" | tail -1)
    if [[ -z "$passed" || "$passed" -eq 0 ]]; then
        stage_fail oracle-pin-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass oracle-pin-selftest "${tally:-$passed cases passed}"
}

# ---------------------------------------------------------------------------
# Stage: producer-selftest
#
# Runs `python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py` --
# the doneness-verdict-table self-test (launch-readiness remediation Step
# 4D, blocker B6). Grids `WIRING_CLASS_VALUES x` the generator's own status
# vocabulary over a fabricated document and asserts nothing lands in
# `doneness_unmapped`, plus the specific `(ambiguous, literal-/fixture-
# verified) -> held` and `(static, literal-verified) -> done` rulings. Cheap
# (stdlib unittest, no build, no network, a temp file per test) — placed in
# BOTH stage sets next to oracle-pin-selftest/corpus-sweep-selftest, the
# same "self-test for a table that raises on purpose deserves its own gate"
# reasoning those two carry.
# ---------------------------------------------------------------------------

run_producer_selftest() {
    stage_start "producer-selftest — python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py"
    local log="$LOG_DIR/producer-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_pf1e_dashboard_producer.py"

    if [[ ! -f "$script" ]]; then
        stage_fail producer-selftest "self-test script missing at scripts/tests/test_pf1e_dashboard_producer.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    # unittest's own summary line, e.g. "Ran 5 tests in 0.010s" -- parsed the
    # same way the bash selftests parse their own "passed: N  failed: M"
    # tally, so a run that silently discovered 0 tests (a bad import path, a
    # renamed TestCase) is caught the same way as those stages' "0 cases ran"
    # guard, not read as a vacuous pass.
    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail producer-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail producer-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass producer-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: pi-redaction-selftest
#
# Runs `python3 -m unittest scripts/tests/test_pi_redaction.py` -- the
# self-test for `scripts/observer/pi_redaction.py`, Decision 12's declared-PI
# oracle reader (SD31-D14-PROV-001, 2026-08-17: "withhold the name, keep the
# row"). Builds scratch pcgen-shaped fixtures (never the real pinned oracle,
# same posture `groundtruth-guard-selftest` already takes) and mutation-proves
# the core reader, the ambiguous-name exclusion (a bare word declared PI in
# one book must not flag an unrelated non-PI record sharing that word
# elsewhere -- the real false positive this cycle found: an unrelated
# Spycraft "Teleport" ritual colliding with the Core Rulebook's ordinary
# "Teleport" spell), and the exact-match leak scanner/redactor
# `site-dashboard-pi-gate` and the producer both depend on. Cheap (stdlib
# unittest, no build, no network) -- placed in BOTH stage sets next to
# `producer-selftest`, same "self-test for a screen that raises on purpose
# deserves its own gate" reasoning.
# ---------------------------------------------------------------------------

run_pi_redaction_selftest() {
    stage_start "pi-redaction-selftest — python3 -m unittest scripts/tests/test_pi_redaction.py"
    local log="$LOG_DIR/pi-redaction-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_pi_redaction.py"

    if [[ ! -f "$script" ]]; then
        stage_fail pi-redaction-selftest "self-test script missing at scripts/tests/test_pi_redaction.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail pi-redaction-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail pi-redaction-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass pi-redaction-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: provenance-selftest
#
# Runs `python3 -m unittest scripts/tests/test_provenance.py` -- the
# self-test for `scripts/observer/provenance.py`, Decision 14's provenance
# schema (SD31-D14-PROV-001, 2026-08-17 CONFIRMED ruling). Proves each of
# the six gate invariants both passes on clean data and fails on
# deliberately broken data: totality, exactly-one-authoritative-pair,
# denominator = authoritative + variant (mutation-proven against the real
# anti-gaming shape -- widening DENOMINATOR_STATUSES to also count
# `duplicate` fails the test), packaging-artifact trending to zero,
# descoped-structural requiring a signature, and a provenance change moving
# zero doneness fields. THIS SCHEMA IS NOT APPLIED TO THE MANDATE
# DENOMINATOR THIS CYCLE (the Supersession Register rebuild has not landed
# and race attribution is frozen per `§13`'s amendment) -- this stage
# guards the schema/gate MACHINERY only. Cheap (stdlib unittest, no build,
# no network) -- placed in BOTH stage sets next to `pi-redaction-selftest`.
# ---------------------------------------------------------------------------

run_provenance_selftest() {
    stage_start "provenance-selftest — python3 -m unittest scripts/tests/test_provenance.py"
    local log="$LOG_DIR/provenance-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_provenance.py"

    if [[ ! -f "$script" ]]; then
        stage_fail provenance-selftest "self-test script missing at scripts/tests/test_provenance.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail provenance-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail provenance-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass provenance-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: site-dashboard-selftest
#
# Runs `bash scripts/tests/test_publish_site_dashboard.sh` -- the detection
# self-test for `scripts/publish-site-dashboard.sh`'s `--check` mode
# (SD31-ATTRIB-003, operator request to version the public status feed).
# Against a tiny FAKE producer (not the real one, so this is cheap and
# deterministic), it proves `--check` reports "current" on an untouched
# tree, is stable across repeated runs, catches a genuinely stale committed
# copy, and never mutates the committed file on disk. Mutation-proven against
# the real bug this cycle found and fixed: the first version of `--check`
# rendered into a blank-slate scratch file instead of one seeded from the
# committed copy, so it reported STALE unconditionally, even with nothing
# changed -- sabotaging the seeding step reproduces exactly that (3 of 6
# cases fail). Placed next to `producer-selftest`, same "self-test for a
# check that raises on purpose deserves its own gate" reasoning.
# ---------------------------------------------------------------------------

run_site_dashboard_selftest() {
    stage_start "site-dashboard-selftest — bash scripts/tests/test_publish_site_dashboard.sh"
    local log="$LOG_DIR/site-dashboard-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_publish_site_dashboard.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail site-dashboard-selftest "self-test script missing at scripts/tests/test_publish_site_dashboard.sh"
        return
    fi

    bash "$script" >"$log" 2>&1
    local status=$?

    local tally
    tally=$(sed -n 's/^passed: \([0-9]*\)  failed: \([0-9]*\)$/\1 passed, \2 failed/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail site-dashboard-selftest "self-test exit $status${tally:+; $tally} — $log"
        return
    fi

    local passed
    passed=$(sed -n 's/^passed: \([0-9]*\).*$/\1/p' "$log" | tail -1)
    if [[ -z "$passed" || "$passed" -eq 0 ]]; then
        stage_fail site-dashboard-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass site-dashboard-selftest "${tally:-$passed cases passed}"
}

# ---------------------------------------------------------------------------
# Stage: site-dashboard-check
#
# Runs `scripts/publish-site-dashboard.sh --check` for real, against the
# actually-committed `site/dashboard/PF1e-dashboard.json` and the real
# `scripts/observer/pf1e_dashboard_producer.py` -- the freshness gate the
# operator asked for so the public site's copy can never silently drift from
# what `docs/work-inventory.json` currently says. Cheap (reads local repo
# files only, no pinned oracle, no cargo build), so it sits in both stage
# sets next to its own selftest. A failure here means: run
# `./scripts/publish-site-dashboard.sh` and commit the refreshed feed.
# ---------------------------------------------------------------------------

run_site_dashboard_check() {
    stage_start "site-dashboard-check — scripts/publish-site-dashboard.sh --check"
    local log="$LOG_DIR/site-dashboard-check.log"
    local script="$REPO_ROOT/scripts/publish-site-dashboard.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail site-dashboard-check "script missing at scripts/publish-site-dashboard.sh"
        return
    fi

    ( cd "$REPO_ROOT" && exec "$script" --check ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail site-dashboard-check "exit $status — $log"
        return
    fi

    if ! grep -q "is current" "$log"; then
        stage_fail site-dashboard-check "exited 0 without confirming currency — $log"
        return
    fi

    stage_pass site-dashboard-check "site/dashboard/PF1e-dashboard.json is current"
}

# ---------------------------------------------------------------------------
# Stage: site-dashboard-pi-gate
#
# Decision 12 (2026-08-17), binding implementation requirement #3: "A gate,
# proven able to fail. A verify.sh stage must fail when the committed feed or
# any shard carries a declared-PI name." Runs
# `scripts/site_dashboard_pi_gate.py` for real against whatever is actually
# committed under `site/dashboard/` -- the SAFETY NET behind the producer's
# own two precise, coordinate-based redactions
# (`build_unit_shards`'s `name` field, `_parse_lst_first_field`'s roster
# rows) and its blanket exact-match sweep over the whole assembled document.
# A hand-edit, a reverted redaction, or a future producer change that forgets
# to call the reader are all real failure modes a generation-time fix alone
# cannot catch -- `declared-pi-audit` above is this exact same shape applied
# to `data/corpus/`; this is that shape's `site/dashboard/` counterpart.
# Cheap (a ~2.5s Paizo-scoped oracle sweep, no build) -- placed in BOTH stage
# sets next to `site-dashboard-check`.
# ---------------------------------------------------------------------------

run_site_dashboard_pi_gate() {
    stage_start "site-dashboard-pi-gate — declared-PI names vs. what is committed under site/dashboard/"
    local log="$LOG_DIR/site-dashboard-pi-gate.log"
    local script="$REPO_ROOT/scripts/site_dashboard_pi_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail site-dashboard-pi-gate "gate script missing at scripts/site_dashboard_pi_gate.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail site-dashboard-pi-gate "declared-PI name found, or the oracle could not be read (exit $status) — $log"
        return
    fi

    if ! grep -q '^site-dashboard-pi-gate: CLEAN' "$log"; then
        stage_fail site-dashboard-pi-gate "exited 0 without reporting CLEAN — $log"
        return
    fi

    local summary
    summary=$(sed -n 's/^site-dashboard-pi-gate: CLEAN — \(.*\)$/\1/p' "$log" | tail -1)
    stage_pass site-dashboard-pi-gate "${summary:-clean}"
}

# ---------------------------------------------------------------------------
# Stage: build-public-status-selftest
#
# Runs `python3 -m unittest scripts/tests/test_build_public_status.py` --
# SITE-PUBSTATUS-001's own self-test for `scripts/site/build_public_status.py`
# (PI screening, the public done/partial/not-started doneness bucket
# mapping, and the standing/denominator wiring). Same shape as
# `pi-redaction-selftest`/`provenance-selftest` above: a scratch-fixture
# unit test, no pinned oracle or committed ledger required, cheap enough for
# both stage sets.
# ---------------------------------------------------------------------------

run_build_public_status_selftest() {
    stage_start "build-public-status-selftest — python3 -m unittest scripts/tests/test_build_public_status.py"
    local log="$LOG_DIR/build-public-status-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_build_public_status.py"

    if [[ ! -f "$script" ]]; then
        stage_fail build-public-status-selftest "self-test script missing at scripts/tests/test_build_public_status.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail build-public-status-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail build-public-status-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass build-public-status-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: site-asset-stamp-check
#
# `site/styles.css` has a stable filename, so a returning visitor's browser
# will keep serving its cached copy after a deploy unless the URL changes.
# On 2026-08-18 that shipped a live page whose HTML was current but whose CSS
# was not, and it was only caught because the operator looked at it on a
# phone. `scripts/site/stamp_asset_versions.py` appends `?v=<content-hash>` to
# every stylesheet reference; this gate fails when the committed stamps no
# longer match the committed stylesheet. A failure here means: run
# `python3 scripts/site/stamp_asset_versions.py` and commit the result.
# ---------------------------------------------------------------------------

run_site_asset_stamp_check() {
    stage_start "site-asset-stamp-check — scripts/site/stamp_asset_versions.py --check"
    local log="$LOG_DIR/site-asset-stamp-check.log"
    local script="$REPO_ROOT/scripts/site/stamp_asset_versions.py"

    if [[ ! -f "$script" ]]; then
        stage_fail site-asset-stamp-check "script missing at scripts/site/stamp_asset_versions.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" --check ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail site-asset-stamp-check "exit $status — $log"
        return
    fi

    if ! grep -q "^OK: " "$log"; then
        stage_fail site-asset-stamp-check "exited 0 without confirming currency — $log"
        return
    fi

    stage_pass site-asset-stamp-check "site/*.html cache-busting stamps match site/styles.css"
}

# ---------------------------------------------------------------------------
# Stage: site-public-status-check
#
# Runs `python3 scripts/site/build_public_status.py --check` directly against
# the actually-committed `site/status-data.json` and `site/status-data/*.json`
# -- the operator's explicit self-maintaining requirement ("the data set...
# needs to be a part of our normal process just like it is for our
# pf1e-dashboard.html"), applied as its own named, directly-invoked gate
# (rather than only transitively through `site-dashboard-check`'s call into
# `scripts/publish-site-dashboard.sh --check`, which also reaches this same
# script — see that script's own trailing step). Cheap (reads local repo
# files plus one pinned-oracle sweep for the redaction indices, no cargo
# build), so it sits in both stage sets next to its own selftest and PI
# gate. A failure here means: run
# `python3 scripts/site/build_public_status.py` and commit the refreshed
# projection.
# ---------------------------------------------------------------------------

run_site_public_status_check() {
    stage_start "site-public-status-check — scripts/site/build_public_status.py --check"
    local log="$LOG_DIR/site-public-status-check.log"
    local script="$REPO_ROOT/scripts/site/build_public_status.py"

    if [[ ! -f "$script" ]]; then
        stage_fail site-public-status-check "script missing at scripts/site/build_public_status.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" --check ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail site-public-status-check "exit $status — $log"
        return
    fi

    if ! grep -q "^OK: " "$log"; then
        stage_fail site-public-status-check "exited 0 without confirming currency — $log"
        return
    fi

    stage_pass site-public-status-check "site/status-data.json and site/status-data/*.json are current"
}

# ---------------------------------------------------------------------------
# Stage: site-public-status-pi-gate
#
# Decision 12's binding implementation requirement #3, applied to the
# PUBLIC status projection specifically (`site-dashboard-pi-gate` above
# covers the separate `site/dashboard/**` surface): "A verify.sh stage must
# fail when the committed feed or any shard carries a declared-PI name."
# Runs `scripts/site_public_status_pi_gate.py` for real against whatever is
# actually committed under `site/status-data.json` and
# `site/status-data/*.json` -- the SAFETY NET behind
# `build_public_status.py`'s own generation-time redaction
# (`redact_for_display`'s per-book name check plus its `type_facet`
# substring screen, and the final blanket exact-match sweep over the whole
# assembled document). Cloudflare Pages deploys `site/**` on push to `main`
# with no build step, so this gate is the last thing standing between a
# leaked name and a live page. Cheap (a ~2.5s Paizo-scoped oracle sweep, no
# build) -- placed in BOTH stage sets next to `site-public-status-check`.
# ---------------------------------------------------------------------------

run_site_public_status_pi_gate() {
    stage_start "site-public-status-pi-gate — declared-PI names vs. what is committed under site/status-data*"
    local log="$LOG_DIR/site-public-status-pi-gate.log"
    local script="$REPO_ROOT/scripts/site_public_status_pi_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail site-public-status-pi-gate "gate script missing at scripts/site_public_status_pi_gate.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail site-public-status-pi-gate "declared-PI name found, or the oracle could not be read (exit $status) — $log"
        return
    fi

    if ! grep -q '^site-public-status-pi-gate: CLEAN' "$log"; then
        stage_fail site-public-status-pi-gate "exited 0 without reporting CLEAN — $log"
        return
    fi

    local summary
    summary=$(sed -n 's/^site-public-status-pi-gate: CLEAN — \(.*\)$/\1/p' "$log" | tail -1)
    stage_pass site-public-status-pi-gate "${summary:-clean}"
}

# ---------------------------------------------------------------------------
# Stage: reachability-audit-selftest
#
# Runs `python3 -m unittest scripts/tests/test_reachability_audit.py` --
# SD31-E0-F1's own self-test (`decisions.md §4`). Feeds
# `scripts/reachability_audit.py` a FABRICATED dead-end (a wiring_class/
# status pair `_doneness_verdict_uncapped()` has no rule for) and confirms
# it is both reported and fails the audit's own exit code, per "prove it can
# fail before it is trusted" (`SD-30 state-goals-and-lessons.md §3.1`).
# Cheap (stdlib unittest, no build, no network) -- placed in BOTH stage sets
# next to producer-selftest, the same self-test-for-a-table-that-raises-on-
# purpose reasoning that stage carries.
# ---------------------------------------------------------------------------

run_reachability_audit_selftest() {
    stage_start "reachability-audit-selftest — python3 -m unittest scripts/tests/test_reachability_audit.py"
    local log="$LOG_DIR/reachability-audit-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_reachability_audit.py"

    if [[ ! -f "$script" ]]; then
        stage_fail reachability-audit-selftest "self-test script missing at scripts/tests/test_reachability_audit.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail reachability-audit-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail reachability-audit-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass reachability-audit-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: groundtruth-guard-selftest
#
# Runs `python3 -m unittest scripts/tests/test_ground_truth_evidence_guard.py
# scripts/tests/test_sample_ground_truth_units.py` -- the self-tests for
# `scripts/ground_truth_evidence_guard.py` and
# `scripts/sample_ground_truth_units.py` (SD31-E2-F1-002). Adversarial
# review Finding 14 (`SD31-W2-INTEGRATE-001`): ~800 lines of new Python
# shipped with no gate coverage at all. The guard's own LIVE run against
# the real ground-truth sample is deliberately kept OUT of this gate --
# `OPEN-ISSUES.md` rows 14/15 -- because it currently reds on the
# untouched-45 residual this cycle was barred from repairing, and
# `verify.sh` has only two stage tiers (no "registered but not default").
# The SELF-TESTS carry no such dependency (a hermetic fake corpus tree
# under a temp dir, same pattern as producer-selftest/
# reachability-audit-selftest above) and pass right now, so wiring them in
# is zero-risk. Cheap (stdlib unittest, no build, no network).
# ---------------------------------------------------------------------------

run_groundtruth_guard_selftest() {
    stage_start "groundtruth-guard-selftest — python3 -m unittest scripts/tests/test_ground_truth_evidence_guard.py scripts/tests/test_sample_ground_truth_units.py"
    local log="$LOG_DIR/groundtruth-guard-selftest.log"
    local script1="$REPO_ROOT/scripts/tests/test_ground_truth_evidence_guard.py"
    local script2="$REPO_ROOT/scripts/tests/test_sample_ground_truth_units.py"

    if [[ ! -f "$script1" || ! -f "$script2" ]]; then
        stage_fail groundtruth-guard-selftest "self-test script(s) missing: $script1 / $script2"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script1" "$script2" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail groundtruth-guard-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail groundtruth-guard-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass groundtruth-guard-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: supersession-gate-selftest
#
# Runs `python3 -m unittest scripts/tests/test_supersession_register_gate.py`
# -- SD31-D10-REGISTER-001's own self-test. Decision 10 (`decisions.md`) is
# the FIRST authorization in this package to shrink the mandate denominator,
# and it is a standing rule a cycle may apply WITHOUT a per-entry operator
# signature (unlike the Structural Exclusion Register, `decisions.md §3`) --
# so this gate, not a signature, is the only thing protecting that number.
# This self-test seeds a bad entry of BOTH refusal shapes the card demands
# (two records that materially differ; a variant-line book with no
# `reprint_proof`) plus two structural ones (core_essentials on either side;
# backwards SOURCEDATE order) and confirms each is refused, then confirms a
# genuinely clean entry — and a variant-line entry carrying real
# `reprint_proof` — both pass. Hermetic (a tiny fake corpus tree under a
# temp dir, same pattern as reachability-audit-selftest/
# groundtruth-guard-selftest above), no oracle dependency. Cheap, in BOTH
# stage sets.
# ---------------------------------------------------------------------------

run_supersession_gate_selftest() {
    stage_start "supersession-gate-selftest — python3 -m unittest scripts/tests/test_supersession_register_gate.py"
    local log="$LOG_DIR/supersession-gate-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_supersession_register_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail supersession-gate-selftest "self-test script missing at scripts/tests/test_supersession_register_gate.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail supersession-gate-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail supersession-gate-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass supersession-gate-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: shape-coverage-standing-gate-selftest
#
# Runs `python3 -m unittest scripts/tests/test_shape_coverage_standing_gate.py`
# -- SD-32 Gate 3's own self-test (card `gate-3-closure-invariant`,
# AT-32-G3-001/002/003). `scripts/shape_ledger.py`'s `classify_unit()`
# structurally always returns a family (falls through to F0/F8 rather than
# ever emitting an uncovered result), so on the real inventory
# `unclassified_count` can never organically go non-zero -- "prove it can
# fail before it is trusted" (`SD-30 state-goals-and-lessons.md §3.1`, the
# same discipline `reachability-audit-selftest` above applies) therefore
# feeds the gate a FABRICATED uncovered row and a fabricated pile mismatch
# and confirms both are reported and fail the gate's own exit code, plus
# confirms the fail-closed-on-empty-predicate path (AT-32-G3-002). Cheap
# (stdlib unittest, no build, no network) -- placed in BOTH stage sets next
# to supersession-gate-selftest, the same self-test-for-a-gate-that-raises-
# on-purpose reasoning that stage carries.
# ---------------------------------------------------------------------------

run_shape_coverage_standing_gate_selftest() {
    stage_start "shape-coverage-standing-gate-selftest — python3 -m unittest scripts/tests/test_shape_coverage_standing_gate.py"
    local log="$LOG_DIR/shape-coverage-standing-gate-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_shape_coverage_standing_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail shape-coverage-standing-gate-selftest "self-test script missing at scripts/tests/test_shape_coverage_standing_gate.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 -m unittest -v "$script" ) >"$log" 2>&1
    local status=$?

    local ran
    ran=$(sed -n 's/^Ran \([0-9]*\) tests\? in .*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail shape-coverage-standing-gate-selftest "self-test exit $status${ran:+; ran $ran}  — $log"
        return
    fi

    if [[ -z "$ran" || "$ran" -eq 0 ]]; then
        stage_fail shape-coverage-standing-gate-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass shape-coverage-standing-gate-selftest "$ran cases passed"
}

# ---------------------------------------------------------------------------
# Stage: shape-coverage-standing-gate
#
# Runs `scripts/shape_coverage_standing_gate.py` against the live
# `docs/work-inventory.json` and `data/corpus/` -- SD-32's Gate 3 closure
# invariant (AT-32-G3-001/002/003, `decisions.md` Decision 1a). Re-derives
# the shape ledger fresh on every invocation (reusing
# `scripts/shape_ledger.py`'s classification rather than re-deriving them)
# and fails when either `unclassified_count` is non-zero (an object no
# named shape covers) or the per-family piles do not reconcile to the
# population considered (a `build_ledger` regression that silently drops
# rows) -- "sum the piles, always" (`workflow-instruction.md §9` standing
# lesson 5). Cheap (Python + JSON, no cargo build, no network) -- placed in
# BOTH stage sets next to reachability-audit, the same live-corpus-run-
# every-time reasoning that stage carries.
# ---------------------------------------------------------------------------

run_shape_coverage_standing_gate() {
    stage_start "shape-coverage-standing-gate — python3 scripts/shape_coverage_standing_gate.py"
    local log="$LOG_DIR/shape-coverage-standing-gate.log"
    local script="$REPO_ROOT/scripts/shape_coverage_standing_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail shape-coverage-standing-gate "script missing at scripts/shape_coverage_standing_gate.py"
        return
    fi

    (
        cd "$REPO_ROOT" && exec python3 "$script" \
            --inventory "$REPO_ROOT/docs/work-inventory.json" \
            --corpus-root "$REPO_ROOT/data/corpus"
    ) >"$log" 2>&1
    local status=$?

    local population unclassified piles sha no_record budget_exceeded
    population=$(sed -n 's/^population (not-done units considered): \([0-9]*\)$/\1/p' "$log" | tail -1)
    unclassified=$(sed -n 's/^unclassified: \([0-9]*\)$/\1/p' "$log" | tail -1)
    piles=$(sed -n 's/^piles reconcile: \([A-Za-z]*\).*$/\1/p' "$log" | tail -1)
    no_record=$(sed -n 's/^join-status split.*no_record=\([0-9]*\)$/\1/p' "$log" | tail -1)
    budget_exceeded=$(sed -n 's/^.*exceeded: \([A-Za-z]*\)$/\1/p' "$log" | tail -1)
    sha=$(sed -n 's/^corpus SHA: \(.*\)$/\1/p' "$log" | tail -1)
    actual "SHAPE_COVERAGE_POPULATION=${population:-unknown}"
    actual "SHAPE_COVERAGE_NO_RECORD=${no_record:-unknown}"

    if (( status != 0 )); then
        stage_fail shape-coverage-standing-gate "population=${population:-?} unclassified=${unclassified:-?} piles_reconcile=${piles:-?} no_record=${no_record:-?} budget_exceeded=${budget_exceeded:-?} corpus_sha=${sha:-?} — $log"
        return
    fi

    stage_pass shape-coverage-standing-gate "population=${population:-?} unclassified=${unclassified:-?} no_record=${no_record:-?} corpus_sha=${sha:-?}"
}

# ---------------------------------------------------------------------------
# Stage: denominator-gate
#
# Runs `scripts/denominator_gate.py --check` -- `AT-33-E1-004`
# (`docs/release/SD-33-computed-value-verification/epic-breakdown.md`),
# enforcing `decisions.md` §2: a percentage reported without its
# denominator stated in the same construct (the same line) fails the
# build. Default target is this bundle's own generated evidence
# (`artifacts/**/*_cycle_receipt.md` + `progress.md`) -- deliberately not
# this bundle's planning prose (out of this criterion's write scope) and
# not every prior bundle's receipts (261 files, unaudited, a separate
# task). `DENOMINATOR_GATE_PATHS` (space-separated globs) overrides the
# default, matching the `${VAR:-default}` shape `VERIFY_LOG_DIR` and
# `PREFLIGHT_DISK_MIN_FREE_GB` already use -- this is how a deliberately-
# malformed receipt is proven to fail this exact stage without permanently
# committing a violation (see `AT-33-E1-004`'s cycle receipt for the live
# transcript). Cheap (stdlib re/glob, no build, no network) -- placed in
# BOTH stage sets next to shape-coverage-standing-gate, the same
# live-check-with-an-exit-code reasoning that stage carries.
# ---------------------------------------------------------------------------

run_denominator_gate() {
    stage_start "denominator-gate — python3 scripts/denominator_gate.py --check"
    local log="$LOG_DIR/denominator-gate.log"
    local script="$REPO_ROOT/scripts/denominator_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail denominator-gate "script missing at scripts/denominator_gate.py"
        return
    fi

    local -a paths=()
    if [[ -n "${DENOMINATOR_GATE_PATHS:-}" ]]; then
        # shellcheck disable=SC2206
        paths=( ${DENOMINATOR_GATE_PATHS} )
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" --check "${paths[@]}" ) >"$log" 2>&1
    local status=$?

    local checked violations
    checked=$(sed -n 's/^files_checked=\([0-9]*\)$/\1/p' "$log" | tail -1)
    violations=$(sed -n 's/^violations=\([0-9]*\)$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail denominator-gate "violations=${violations:-?} of files_checked=${checked:-?} — $log"
        return
    fi

    stage_pass denominator-gate "files_checked=${checked:-?} violations=0"
}

# ---------------------------------------------------------------------------
# Stage: figure-provenance
#
# Runs `scripts/denominator_gate.py --check-provenance` -- `AT-34-E1-006`
# (`docs/release/SD-34-book-completion/epic-breakdown.md`), enforcing
# `AGENTS.md` rule 9: a figure with no re-derive command reachable from it
# is not a figure, it is a recollection. Wired alongside `denominator-gate`
# in the same script, not as a standalone tool. Default target is this
# package's own artifacts (`PROVENANCE_DEFAULT_GLOBS` -- deliberately not
# SD-33's folder, which this bundle may not write to). `FIGURE_PROVENANCE_PATHS`
# (space-separated globs) overrides the default, the same `${VAR:-default}`
# shape `DENOMINATOR_GATE_PATHS` already uses. The PASS line states the
# figure population examined, closing `workflow-instruction.md §12` row 15
# ("a vacuous pass is not a pass").
# ---------------------------------------------------------------------------

run_figure_provenance() {
    stage_start "figure-provenance — python3 scripts/denominator_gate.py --check-provenance"
    local log="$LOG_DIR/figure-provenance.log"
    local script="$REPO_ROOT/scripts/denominator_gate.py"

    if [[ ! -f "$script" ]]; then
        stage_fail figure-provenance "script missing at scripts/denominator_gate.py"
        return
    fi

    local -a paths=()
    if [[ -n "${FIGURE_PROVENANCE_PATHS:-}" ]]; then
        # shellcheck disable=SC2206
        paths=( ${FIGURE_PROVENANCE_PATHS} )
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" --check-provenance "${paths[@]}" ) >"$log" 2>&1
    local status=$?

    local checked figures violations
    checked=$(sed -n 's/^files_checked=\([0-9]*\)$/\1/p' "$log" | tail -1)
    figures=$(sed -n 's/^figures_examined=\([0-9]*\)$/\1/p' "$log" | tail -1)
    violations=$(sed -n 's/^violations=\([0-9]*\)$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail figure-provenance "violations=${violations:-?} of figures_examined=${figures:-?} (files_checked=${checked:-?}) — $log"
        return
    fi

    stage_pass figure-provenance "files_checked=${checked:-?} figures_examined=${figures:-?} violations=0"
}

# ---------------------------------------------------------------------------
# Stage: reachability-audit
#
# Runs `scripts/reachability_audit.py` against the live `docs/work-inventory.json`
# (SD31-E0-F1/F2, `decisions.md §4`) -- the standing gate: does a path to
# `done` exist, given current engine capability, for every unit on the
# board? Exits non-zero ONLY when a `(wiring_class, status)` cell raises
# `ValueError` (unmapped -- absent from every rollup) AND carries on-board
# units; a `no-done-path` dead end (today: `ambiguous`, Decision 4's
# 2,109-unit gap) is a KNOWN, epic-owned capability gap, reported but not by
# itself gate-failing, so this stage does not turn permanently red while
# Epic 1/Epic 2 are still in flight. The reachable-ceiling number itself is
# read from this stage's own log, not re-derived by the caller.
# ---------------------------------------------------------------------------

run_reachability_audit() {
    stage_start "reachability-audit — python3 scripts/reachability_audit.py"
    local log="$LOG_DIR/reachability-audit.log"
    local script="$REPO_ROOT/scripts/reachability_audit.py"

    if [[ ! -f "$script" ]]; then
        stage_fail reachability-audit "script missing at scripts/reachability_audit.py"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" ) >"$log" 2>&1
    local status=$?

    local ceiling
    ceiling=$(sed -n 's/^  REACHABLE CEILING: \([0-9.]*\)%.*$/\1/p' "$log" | tail -1)
    actual "REACHABILITY_CEILING_PERCENT=${ceiling:-unknown}"

    if (( status != 0 )); then
        printf '    FAIL: an unmapped (wiring_class, status) cell carries on-board units. From the log:\n'
        grep -A5 'FAIL: unmapped' "$log" | sed 's/^/        /'
        stage_fail reachability-audit "exit $status — $log"
        return
    fi

    stage_pass reachability-audit "reachable ceiling ${ceiling:-unknown}%"
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

run_pi_sweep() {
    stage_start "pi-sweep — Product-Identity blacklist over src/rules_core/rules_tables"
    local log="$LOG_DIR/pi-sweep.log"

    # The provenance gate for kind-lane ingestion
    # (docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §37.3,
    # AT-29-003a). Cheap — reads ~137 source files, builds one small bin — so
    # it runs in `quick` too: a lane must not be able to land a PI leak in a
    # generated table on a fast loop and discover it only on a full sweep.
    ( cd "$REPO_ROOT" && exec cargo run --locked --quiet -j "$JOBS" --bin pi_sweep_rules_tables ) >"$log" 2>&1
    local status=$?

    local summary
    summary=$(sed -n 's/^pi-sweep: \([0-9]* hits .*\)$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail pi-sweep "unbaselined PI hit or stale baseline row (exit $status) — $log"
        return
    fi

    # A sweep that examined nothing asserts nothing — the same 0-matched
    # failure mode `reach` and `audit-selftest` each guard against.
    if ! grep -q '^pi-sweep: CLEAN' "$log"; then
        stage_fail pi-sweep "binary exited 0 without reporting CLEAN — $log"
        return
    fi

    stage_pass pi-sweep "${summary:-clean}"
}

# Stage: declared-pi-audit
#
# SD31-PI-REPAIR-001 (OPEN-ISSUES rows 38/39). `pi-sweep` above is the
# heuristic 55-term blacklist over `src/rules_core/rules_tables`; this stage
# is the corpus's OWN per-record declaration (`NAMEISPI:`/`DESCISPI:`),
# cross-checked against what actually shipped under `data/corpus/`. Two real
# defects reached `tranche/11` past every other gate because nothing did
# this cross-check: `cache_gen::ultimate_equipment.rs` shipped a
# `NAMEISPI:YES` record's real name unredacted, and `ingest_races.rs`
# hardcoded `pi_field: None` while a `LICENSE.json` claimed the declared-PI
# reader ran. Both are now checked directly, and a `LICENSE.json` opting
# into the structured `declared_pi_reader_verified` claim is verified
# against its own named writer source, not trusted as prose.
run_declared_pi_audit() {
    stage_start "declared-pi-audit — corpus NAMEISPI:/DESCISPI: declarations vs. what shipped"
    local log="$LOG_DIR/declared-pi-audit.log"

    ( cd "$REPO_ROOT" && exec cargo run --locked --quiet -j "$JOBS" --bin declared_pi_shipping_audit ) >"$log" 2>&1
    local status=$?

    if (( status != 0 )); then
        stage_fail declared-pi-audit "unredacted PI-declared record or unverified LICENSE.json claim (exit $status) — $log"
        return
    fi

    if ! grep -q '^declared-pi-audit: CLEAN' "$log"; then
        stage_fail declared-pi-audit "binary exited 0 without reporting CLEAN — $log"
        return
    fi

    stage_pass declared-pi-audit "clean"
}

# Stage: driver-selftest
#
# Runs scripts/tests/test_run_desktop_driver.sh — the self-test for
# apps/desktop/.claude/skills/run-desktop/driver.sh.
#
# Why this is a gate stage. The driver is the only mechanism that satisfies the
# "drive it on screen" acceptance item, and the tranche/7 retrospective ranks
# on-screen driving as the sole mechanism reaching the "wired into a twin the
# sheet doesn't read" defect class — 14% of that tranche's corrections, a class
# no passing test can reach by construction. When the driver breaks, that whole
# class stops being detectable and nothing says so: five agents invoked
# `driver.sh launch` during the first corpus-wide catch-up run, not one left a
# state file, three independently reported the same wrong root cause, and every
# player-visible family that run ingested shipped without on-screen
# verification. Nothing in the gate noticed.
#
# No build, no display, seconds to run: it drives throwaway decoy processes.
# ---------------------------------------------------------------------------
run_driver_selftest() {
    stage_start "driver-selftest — scripts/tests/test_run_desktop_driver.sh"
    local log="$LOG_DIR/driver-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_run_desktop_driver.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail driver-selftest "self-test script missing at scripts/tests/test_run_desktop_driver.sh"
        return
    fi

    bash "$script" >"$log" 2>&1
    local status=$?

    local tally
    tally=$(sed -n 's/^passed: \([0-9]*\)  failed: \([0-9]*\)$/\1 passed, \2 failed/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail driver-selftest "self-test exit $status${tally:+; $tally} — $log"
        return
    fi

    # Same 0-matched guard the other self-test stages carry: a self-test that
    # discovered no cases proves nothing while looking identical to one that
    # passed them all.
    local passed
    passed=$(sed -n 's/^passed: \([0-9]*\).*$/\1/p' "$log" | tail -1)
    if [[ -z "$passed" || "$passed" -eq 0 ]]; then
        stage_fail driver-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass driver-selftest "${tally:-$passed cases passed}"
}

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

# ---------------------------------------------------------------------------
# Stage: reclaim-selftest
#
# Runs scripts/tests/test_reclaim_orphan_targets.sh — the self-test for
# scripts/reclaim.sh's orphaned codex-target-* coverage and its liveness
# guards. Sits next to audit-selftest because it holds the same lesson: an
# unverified gate is worth little, and reclaim.sh guarding the #1 recorded
# incident class (disk-full/disk-pressure, 43 of ~60 incidents as of
# 2026-08-11) must not become another one. The safety property under test —
# never delete a live agent's target dir — is the difference between a
# reclaimed 27G and a destroyed 30-minute rebuild.
# ---------------------------------------------------------------------------

run_reclaim_selftest() {
    stage_start "reclaim-selftest — scripts/tests/test_reclaim_orphan_targets.sh"
    local log="$LOG_DIR/reclaim-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_reclaim_orphan_targets.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail reclaim-selftest "self-test script missing at scripts/tests/test_reclaim_orphan_targets.sh"
        return
    fi

    bash "$script" >"$log" 2>&1
    local status=$?

    local tally
    tally=$(sed -n 's/^passed: \([0-9]*\)  failed: \([0-9]*\)$/\1 passed, \2 failed/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail reclaim-selftest "self-test exit $status${tally:+; $tally} — $log"
        return
    fi

    # A self-test that discovers no cases proves nothing — same guard the
    # audit-selftest stage carries.
    local passed
    passed=$(sed -n 's/^passed: \([0-9]*\).*$/\1/p' "$log" | tail -1)
    if [[ -z "$passed" || "$passed" -eq 0 ]]; then
        stage_fail reclaim-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass reclaim-selftest "${tally:-$passed cases passed}"
}

# ---------------------------------------------------------------------------
# Stage: corpus-sweep-selftest
#
# Runs scripts/tests/test_corpus_literal_sweep.sh — the detection self-test
# for the `corpus_literal_sweep` binary. Same lesson as audit-selftest and
# reclaim-selftest, and the reason it is a stage rather than something someone
# remembers to run: the sweep below is the ONLY instrument that can confirm a
# `static` unit's bar, and an instrument whose ability to say NO is untested
# emits its CLEAN token with identical confidence whether it is working or
# dead. Two gates in this repo have already shipped with exactly that defect.
#
# Cheap after the first build, and it never reads the real corpus — every case
# runs against a synthetic repo root and a synthetic PCGen corpus under mktemp.
# ---------------------------------------------------------------------------

run_corpus_sweep_selftest() {
    stage_start "corpus-sweep-selftest — scripts/tests/test_corpus_literal_sweep.sh"
    local log="$LOG_DIR/corpus-sweep-selftest.log"
    local script="$REPO_ROOT/scripts/tests/test_corpus_literal_sweep.sh"

    if [[ ! -f "$script" ]]; then
        stage_fail corpus-sweep-selftest "self-test script missing at scripts/tests/test_corpus_literal_sweep.sh"
        return
    fi

    bash "$script" >"$log" 2>&1
    local status=$?

    local tally
    tally=$(sed -n 's/^passed: \([0-9]*\)  failed: \([0-9]*\)$/\1 passed, \2 failed/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail corpus-sweep-selftest "self-test exit $status${tally:+; $tally} — $log"
        return
    fi

    # A self-test that discovers no cases proves nothing — the same guard
    # audit-selftest and reclaim-selftest each carry.
    local passed
    passed=$(sed -n 's/^passed: \([0-9]*\).*$/\1/p' "$log" | tail -1)
    if [[ -z "$passed" || "$passed" -eq 0 ]]; then
        stage_fail corpus-sweep-selftest "0 cases ran — the self-test asserts nothing — $log"
        return
    fi

    stage_pass corpus-sweep-selftest "${tally:-$passed cases passed}"
}

# ---------------------------------------------------------------------------
# Stage: corpus-sweep
#
# Runs `corpus_literal_sweep` over every shipped record in `data/corpus/` and
# the real PCGen corpus. This is the corpus-literal byte-equality check a
# `static` unit's bar names: `wiring_class` calls a record `static` when its
# whole token closure is literal magnitudes, which makes "the shipped bytes
# equal the corpus bytes" a bar that is knowable WITHOUT any consumer-delta
# probe — and that nothing checked until this stage existed.
#
# FULL only, and deliberately a FAILURE rather than a skip when the corpus is
# absent: `v06_corpus_trap_report` prints `SKIP: no PCGen corpus`, and a skip
# is exactly how a gate dies without anyone noticing. The corpus location is
# `PCGEN_CORPUS_ROOT`, defaulting to `$HOME/workspace/repos/pcgen/data` — the
# same HOME-relative default `v06_work_inventory` uses, per SD-27 decisions.md
# §30 (workspace/ is Syncthing-synced; an absolute other-user path is not).
# The cheap, build-free, whole-cone check that this stage's absent-corpus
# failure mode should point you at FIRST is `preflight-oracle` (above,
# earlier in both stage sets) — it names the exact fetch command. This stage
# additionally sha256-checks every CITED corpus file's bytes, which
# `preflight-oracle` does not: complementary, not redundant.
#
# The record count is a FLOOR in scripts/verify-baselines.env, same direction
# as the test counts: the population growing is fine, the population silently
# shrinking is the failure this repo has actually suffered.
# ---------------------------------------------------------------------------

run_corpus_sweep() {
    stage_start "corpus-sweep — cargo run --locked --bin corpus_literal_sweep  (repo root)"
    local log="$LOG_DIR/corpus-sweep.log"

    ( cd "$REPO_ROOT" && exec cargo run --locked --quiet -j "$JOBS" --bin corpus_literal_sweep ) >"$log" 2>&1
    local status=$?

    local summary
    summary=$(sed -n 's/^corpus-literal-sweep: \([0-9]* records examined.*\)$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        stage_fail corpus-sweep "byte-level mismatch, absent corpus, or malformed record (exit $status) — $log"
        return
    fi

    # Exit 0 without the CLEAN token means the binary took a path nobody
    # intended — the pi-sweep stage carries the identical guard for the
    # identical reason.
    if ! grep -q '^corpus-literal-sweep: CLEAN' "$log"; then
        stage_fail corpus-sweep "binary exited 0 without reporting CLEAN — $log"
        return
    fi

    # A sweep that examined nothing asserts nothing. The binary exits 2 on a
    # zero population on its own; this is the second, independent reading of
    # the same fact, because the one failure this check exists to prevent is
    # the binary silently changing what it counts.
    local examined tokens
    examined=$(sed -n 's/^corpus-literal-sweep: \([0-9]*\) records examined.*$/\1/p' "$log" | tail -1)
    tokens=$(sed -n 's/^corpus-literal-sweep: .* \([0-9]*\) tokens compared.*$/\1/p' "$log" | tail -1)
    if [[ -z "$examined" || "$examined" -eq 0 || -z "$tokens" || "$tokens" -eq 0 ]]; then
        stage_fail corpus-sweep "0 records or 0 tokens compared — the sweep asserts nothing — $log"
        return
    fi
    if (( examined < BASELINE_CORPUS_LITERAL_RECORDS )); then
        stage_fail corpus-sweep "population shrank: $examined records examined, baseline floor is $BASELINE_CORPUS_LITERAL_RECORDS — $log"
        return
    fi
    if (( examined > BASELINE_CORPUS_LITERAL_RECORDS )); then
        note "BASELINE_CORPUS_LITERAL_RECORDS=$examined (was $BASELINE_CORPUS_LITERAL_RECORDS)"
    fi
    actual "BASELINE_CORPUS_LITERAL_RECORDS=$examined"

    stage_pass corpus-sweep "${summary:-clean}"
}

# ---------------------------------------------------------------------------
# Stage: supersession-gate
#
# Runs `scripts/supersession_register_gate.py` against the committed
# `docs/release/SD-31-corpus-closure-grind/artifacts/SUPERSESSION-REGISTER.json`
# (SD31-D10-REGISTER-001, `decisions.md` Decision 10 + its amendment). For
# every `objects[]` entry it RE-DERIVES both sides' raw `.lst` row from the
# pinned oracle (never trusts the register's own cached copy) and refuses
# the entry if the two are not still field-identical after stripping
# provenance/pricing tokens — a corpus drift or a hand-edited entry both
# fail here. It also refuses any entry naming `pathfinder_unchained` or
# `mythic_adventures` without a `reprint_proof` (default: variant, not a
# reprint), any entry naming `core_essentials` (Decision 9: not a book),
# and a backwards SOURCEDATE order. FULL only (needs the pinned oracle),
# placed immediately after `corpus-sweep`, same dependency.
# ---------------------------------------------------------------------------

run_supersession_gate() {
    stage_start "supersession-gate — python3 scripts/supersession_register_gate.py"
    local log="$LOG_DIR/supersession-gate.log"
    local script="$REPO_ROOT/scripts/supersession_register_gate.py"
    local register="$REPO_ROOT/docs/release/SD-31-corpus-closure-grind/artifacts/SUPERSESSION-REGISTER.json"

    if [[ ! -f "$script" ]]; then
        stage_fail supersession-gate "script missing at scripts/supersession_register_gate.py"
        return
    fi
    if [[ ! -f "$register" ]]; then
        stage_fail supersession-gate "register missing at docs/release/SD-31-corpus-closure-grind/artifacts/SUPERSESSION-REGISTER.json"
        return
    fi

    ( cd "$REPO_ROOT" && exec python3 "$script" --register "$register" ) >"$log" 2>&1
    local status=$?

    local checked
    checked=$(sed -n 's/^supersession_register_gate: \([0-9]*\) objects checked.*$/\1/p' "$log" | tail -1)

    if (( status != 0 )); then
        printf '    FAIL: at least one register entry was refused. From the log:\n'
        grep -A20 '  FAIL:' "$log" | sed 's/^/        /'
        stage_fail supersession-gate "exit $status — $log"
        return
    fi

    if ! grep -q '^  OK:' "$log"; then
        stage_fail supersession-gate "binary exited 0 without reporting OK — $log"
        return
    fi

    stage_pass supersession-gate "${checked:-0} objects, all clean"
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
        preflight-oracle)    run_preflight_oracle ;;
        oracle-pin-selftest) run_oracle_pin_selftest ;;
        producer-selftest)   run_producer_selftest ;;
        pi-redaction-selftest) run_pi_redaction_selftest ;;
        provenance-selftest) run_provenance_selftest ;;
        site-dashboard-selftest) run_site_dashboard_selftest ;;
        site-dashboard-check) run_site_dashboard_check ;;
        site-dashboard-pi-gate) run_site_dashboard_pi_gate ;;
        build-public-status-selftest) run_build_public_status_selftest ;;
        site-asset-stamp-check) run_site_asset_stamp_check ;;
        site-public-status-check) run_site_public_status_check ;;
        site-public-status-pi-gate) run_site_public_status_pi_gate ;;
        reachability-audit-selftest) run_reachability_audit_selftest ;;
        reachability-audit)  run_reachability_audit ;;
        groundtruth-guard-selftest) run_groundtruth_guard_selftest ;;
        supersession-gate-selftest) run_supersession_gate_selftest ;;
        shape-coverage-standing-gate-selftest) run_shape_coverage_standing_gate_selftest ;;
        shape-coverage-standing-gate) run_shape_coverage_standing_gate ;;
        denominator-gate)    run_denominator_gate ;;
        figure-provenance)   run_figure_provenance ;;
        pi-sweep)            run_pi_sweep ;;
        declared-pi-audit)   run_declared_pi_audit ;;
        audit-selftest)      run_audit_selftest ;;
        reclaim-selftest)    run_reclaim_selftest ;;
        driver-selftest)     run_driver_selftest ;;
        corpus-sweep-selftest) run_corpus_sweep_selftest ;;
        corpus-sweep)        run_corpus_sweep ;;
        supersession-gate)   run_supersession_gate ;;
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
