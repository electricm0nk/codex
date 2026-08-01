#!/usr/bin/env bash
#
# reclaim.sh — the executable counterpart to the CARGO_TARGET_DIR-per-agent rule.
#
# WHY THIS EXISTS
# ---------------
# `docs/retro/tranche-7-retrospective.md` §4.1 records disk exhaustion as this
# repo's second-largest orchestration failure mode: 5 of 34 logged incidents,
# including `/home` hitting 100% used with 0 bytes available and "30+
# per-agent CARGO_TARGET_DIRs under ~/.cache totalling >600G, many 18-35G
# each." The retrospective's own diagnosis is the design constraint this
# script answers: "The rule shipped in the brief; the matching `rm -rf` did
# not." `AGENTS.md` correctly mandates a per-agent, per-source-tree
# CARGO_TARGET_DIR and tells agents to delete it when they finish — but
# nothing ever enforced or automated that deletion, so it did not happen at
# the rate the rule needed. This script is the missing `rm -rf`.
#
# WHAT IT RECLAIMS, IN CATEGORIES (see --help for the authoritative list)
# ------------------------------------------------------------------------
#   cargo-target   Abandoned CARGO_TARGET_DIRs (identified by CACHEDIR.TAG,
#                   which cargo writes into every target dir it creates)
#                   under the Claude scratchpad root and under the repo's
#                   own $HOME/.cache/codex-* convention.
#   verify-logs    Stale scripts/verify.sh log directories under /tmp
#                   (codex-verify-XXXXXX).
#   worktrees      git worktrees whose branch is merged into develop or
#                   whose PR is closed/merged, pruned via `git worktree
#                   remove` + `git worktree prune`.
#   branches       Local branches already merged into develop, or whose
#                   upstream is gone.
#
# SAFETY, IN ORDER OF HOW OFTEN IT WOULD HAVE MATTERED
# -----------------------------------------------------
#   1. Dry-run is the default. Nothing is deleted without --apply.
#   2. NEVER delete a cargo-target dir a live build is using. "In use" is
#      decided by inspecting /proc/<pid>/environ and /proc/<pid>/cwd for
#      REAL cargo/rustc processes, found by kernel-reported `comm` (via
#      `ps -o comm=`), NOT by grepping full command lines. `pgrep -f cargo`
#      matches this very script's own text (it says "cargo" a few dozen
#      times) — that is the self-match trap, and `comm` cannot be spoofed by
#      our own argv content. Self and all ancestor PIDs are excluded on top
#      of that, belt-and-braces.
#   3. NEVER delete a git worktree with uncommitted changes or unpushed
#      commits — checked via `git status --porcelain` and the upstream
#      ahead-count, not assumed from branch name or PR state alone.
#   4. NEVER touch /home/ubuntu/workspace/repos/pcgen, this repo's own
#      checkout, or any worktree/branch currently checked out anywhere.
#   5. `git stash` is never invoked anywhere in this script.
#   6. Age-thresholded: cargo-target and verify-logs candidates younger than
#      --older-than (default 6h) are always skipped, live-process check or
#      not — a build that finished 10 minutes ago should not be raced.
#   7. Every deletion goes through one function (`safe_rm_rf`) that refuses
#      an empty, relative, "/", or forbidden path before doing anything.
#      `rm -rf "$X/"` with `X` unset is catastrophic; nothing in this script
#      constructs an rm target any other way.
#
# USAGE
# -----
#   scripts/reclaim.sh                    # dry run, all categories
#   scripts/reclaim.sh --apply            # actually delete
#   scripts/reclaim.sh --only cargo-target --apply
#   scripts/reclaim.sh --older-than 12    # raise the age floor to 12h
#   scripts/reclaim.sh --help
#
# Emits a retro.py `incident` event (recurrence-key `disk-full`, the same key
# every manual disk-exhaustion cleanup in this repo has used) when --apply
# actually reclaims bytes — that is a real incident-prevention datapoint, not
# routine housekeeping, and the retrospective log is honest only if runs like
# this are recorded the same way a human's manual cleanup was.
#
# Exit status: 0 on a normal run (including one that finds nothing to do or
# skips everything). Non-zero only on a usage error or an internal safety
# refusal (e.g. an unset path variable) — see individual guard functions.

set -uo pipefail

# ---------------------------------------------------------------------------
# Location
# ---------------------------------------------------------------------------

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
REPO_ROOT=$(cd -- "$SCRIPT_DIR/.." && pwd)

# Absolute paths this script must never remove or descend past, regardless of
# category or flags. Resolved with `pwd -P` at use-time so a symlink cannot
# smuggle a forbidden path past a string comparison.
FORBIDDEN_PATHS=(
    "$REPO_ROOT"
    "/home/ubuntu/workspace/repos/pcgen"
)

# ---------------------------------------------------------------------------
# Options
# ---------------------------------------------------------------------------

APPLY=0
OLDER_THAN_HOURS=6
ONLY_CATEGORIES=()
SCRATCHPAD_ROOT="${RECLAIM_SCRATCHPAD_ROOT:-/tmp/claude-1000}"
CACHE_ROOT="${RECLAIM_CACHE_ROOT:-$HOME/.cache}"
VERIFY_TMP_ROOT="${RECLAIM_VERIFY_TMP_ROOT:-${TMPDIR:-/tmp}}"
DEVELOP_REF="${RECLAIM_DEVELOP_REF:-origin/develop}"

ALL_CATEGORIES=(cargo-target verify-logs worktrees branches)

usage() {
    sed -n '3,66p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --apply)        APPLY=1 ;;
        --older-than)
            [[ $# -ge 2 ]] || { printf 'reclaim.sh: --older-than needs a number of hours\n' >&2; exit 2; }
            OLDER_THAN_HOURS="$2"; shift ;;
        --only)
            [[ $# -ge 2 ]] || { printf 'reclaim.sh: --only needs a category name\n' >&2; exit 2; }
            ONLY_CATEGORIES+=("$2"); shift ;;
        --scratchpad-root)
            [[ $# -ge 2 ]] || { printf 'reclaim.sh: --scratchpad-root needs a path\n' >&2; exit 2; }
            SCRATCHPAD_ROOT="$2"; shift ;;
        --cache-root)
            [[ $# -ge 2 ]] || { printf 'reclaim.sh: --cache-root needs a path\n' >&2; exit 2; }
            CACHE_ROOT="$2"; shift ;;
        --verify-tmp-root)
            [[ $# -ge 2 ]] || { printf 'reclaim.sh: --verify-tmp-root needs a path\n' >&2; exit 2; }
            VERIFY_TMP_ROOT="$2"; shift ;;
        --develop-ref)
            [[ $# -ge 2 ]] || { printf 'reclaim.sh: --develop-ref needs a ref\n' >&2; exit 2; }
            DEVELOP_REF="$2"; shift ;;
        -h|--help)      usage; exit 0 ;;
        *)              printf 'reclaim.sh: unknown argument: %s (see --help)\n' "$1" >&2; exit 2 ;;
    esac
    shift
done

if [[ ! "$OLDER_THAN_HOURS" =~ ^[0-9]+$ ]]; then
    printf 'reclaim.sh: --older-than must be a non-negative integer, got: %s\n' "$OLDER_THAN_HOURS" >&2
    exit 2
fi

if [[ ${#ONLY_CATEGORIES[@]} -gt 0 ]]; then
    SELECTED=("${ONLY_CATEGORIES[@]}")
    for c in "${SELECTED[@]}"; do
        known=no
        for k in "${ALL_CATEGORIES[@]}"; do [[ "$k" == "$c" ]] && known=yes; done
        [[ "$known" == yes ]] || { printf 'reclaim.sh: unknown category: %s (see --help)\n' "$c" >&2; exit 2; }
    done
else
    SELECTED=("${ALL_CATEGORIES[@]}")
fi

MODE_LABEL="DRY RUN"
(( APPLY == 1 )) && MODE_LABEL="APPLY"

# ---------------------------------------------------------------------------
# Reporting state
# ---------------------------------------------------------------------------

declare -a RECLAIMED_LINES=()
declare -a SKIPPED_LINES=()
TOTAL_RECLAIMED_BYTES=0
ANY_APPLIED=0

say() { printf '%s\n' "$*"; }
hr()  { printf -- '---------------------------------------------------------------\n'; }

human_size() {
    local bytes="${1:-0}"
    awk -v b="$bytes" 'BEGIN {
        split("B KB MB GB TB PB", units, " ");
        u = 1;
        while (b >= 1024 && u < 6) { b /= 1024; u++ }
        printf "%.1f%s", b, units[u]
    }'
}

dir_size_bytes() {
    local dir="$1"
    du -sb "$dir" 2>/dev/null | awk '{print $1}'
}

# ---------------------------------------------------------------------------
# Guard: refuse to build an rm -rf target out of an empty, relative, "/", or
# forbidden path. This is the ONLY function in this script that calls rm -rf.
# ---------------------------------------------------------------------------

is_forbidden_path() {
    local candidate="$1" resolved forbidden_resolved
    resolved=$(cd -- "$candidate" 2>/dev/null && pwd -P) || resolved="$candidate"
    for forbidden in "${FORBIDDEN_PATHS[@]}"; do
        [[ -z "$forbidden" ]] && continue
        forbidden_resolved=$(cd -- "$forbidden" 2>/dev/null && pwd -P) || forbidden_resolved="$forbidden"
        if [[ "$resolved" == "$forbidden_resolved" || "$resolved" == "$forbidden_resolved"/* ]]; then
            return 0
        fi
    done
    return 1
}

safe_rm_rf() {
    local path="$1" reason="${2:-}"
    if [[ -z "$path" ]]; then
        say "    REFUSED: empty path passed to safe_rm_rf ${reason:+($reason)} — this is a script bug, not a user error"
        return 3
    fi
    if [[ "$path" != /* ]]; then
        say "    REFUSED: relative path passed to safe_rm_rf: $path — this is a script bug"
        return 3
    fi
    if [[ "$path" == "/" ]]; then
        say "    REFUSED: refusing to rm -rf /"
        return 3
    fi
    if is_forbidden_path "$path"; then
        say "    REFUSED: $path is inside a forbidden path (repo checkout or pcgen), refusing to remove"
        return 3
    fi
    if [[ ! -e "$path" ]]; then
        # Already gone (race with another cleanup) — not an error.
        return 0
    fi
    rm -rf -- "$path"
}

# ---------------------------------------------------------------------------
# Process inspection
#
# `comm` (via `ps -o comm=`) is the kernel-reported executable basename and
# cannot be spoofed by this script's own argv text. This deliberately does
# NOT use `pgrep -f`, whose full-command-line match would hit this very
# script (it contains the literal string "cargo" many times over) — the
# self-match trap the brief that produced this script explicitly calls out.
# Self and every ancestor PID are excluded on top of that, belt-and-braces.
# ---------------------------------------------------------------------------

self_and_ancestor_pids() {
    local pid=$$
    printf '%s\n' "$pid"
    local guard=0
    while (( guard < 200 )); do
        local ppid
        ppid=$(ps -o ppid= -p "$pid" 2>/dev/null | tr -d ' ')
        [[ -z "$ppid" || "$ppid" == "0" || "$ppid" == "1" ]] && break
        printf '%s\n' "$ppid"
        pid="$ppid"
        guard=$(( guard + 1 ))
    done
}

live_build_pids() {
    local excluded
    excluded=$'\n'$(self_and_ancestor_pids)$'\n'
    ps -eo pid=,comm= 2>/dev/null | while read -r pid comm; do
        [[ -z "$pid" ]] && continue
        case "$comm" in
            cargo|rustc|verify.sh) ;;
            *) continue ;;
        esac
        [[ "$excluded" == *$'\n'"$pid"$'\n'* ]] && continue
        printf '%s\n' "$pid"
    done
}

# True (0) if any live cargo/rustc/verify.sh process has this exact directory
# as its CARGO_TARGET_DIR, or its cwd is inside it.
target_dir_in_use() {
    local dir="$1" pid envfile cwd
    while read -r pid; do
        [[ -z "$pid" ]] && continue
        envfile="/proc/$pid/environ"
        if [[ -r "$envfile" ]]; then
            if tr '\0' '\n' <"$envfile" 2>/dev/null | grep -qxF "CARGO_TARGET_DIR=$dir"; then
                return 0
            fi
        fi
        cwd=$(readlink -f "/proc/$pid/cwd" 2>/dev/null || true)
        if [[ -n "$cwd" && ( "$cwd" == "$dir" || "$cwd" == "$dir"/* ) ]]; then
            return 0
        fi
    done < <(live_build_pids)
    return 1
}

any_verify_running() {
    live_build_pids | grep -q .
}

# ---------------------------------------------------------------------------
# Age check: nothing under `dir` (up to a bounded depth, for speed on large
# target trees) newer than --older-than hours. A directory that has been
# touched more recently than the threshold is presumed active regardless of
# what the process table shows — a build that just finished should not be
# raced on a process-table false negative.
# ---------------------------------------------------------------------------

older_than_threshold() {
    local dir="$1" hours="$2"
    (( hours == 0 )) && return 0
    if find "$dir" -maxdepth 4 -newermt "-${hours} hours" -print -quit 2>/dev/null | grep -q .; then
        return 1
    fi
    return 0
}

# ---------------------------------------------------------------------------
# CACHEDIR.TAG is a generic convention (the Cache Directory Tagging spec),
# NOT cargo-specific — fontconfig, uv, man-db and others all write one into
# their own cache roots. A first pass of this script that trusted
# CACHEDIR.TAG alone flagged `~/.cache/fontconfig` and `~/.cache/uv` for
# removal on a real dry run against this repo's own environment, which is
# exactly the false positive this function exists to prevent. A cargo target
# dir is distinguished by its actual shape: `.rustc_info.json` at the top
# level, or a `debug`/`release` build-output directory. CACHEDIR.TAG is used
# only to find candidates fast; this is what confirms one.
# ---------------------------------------------------------------------------

is_cargo_target_dir() {
    local dir="$1"
    [[ -f "$dir/CACHEDIR.TAG" ]] || return 1
    if [[ -f "$dir/.rustc_info.json" || -d "$dir/debug" || -d "$dir/release" ]]; then
        return 0
    fi
    return 1
}

# ---------------------------------------------------------------------------
# Category: cargo-target
# ---------------------------------------------------------------------------

reclaim_cargo_target() {
    say "== cargo-target — abandoned CARGO_TARGET_DIRs =="
    local roots=("$SCRATCHPAD_ROOT" "$CACHE_ROOT")
    local seen=""
    local root tag dir size age_ok in_use

    for root in "${roots[@]}"; do
        [[ -d "$root" ]] || { say "    (root does not exist, skipping: $root)"; continue; }
        while IFS= read -r -d '' tag; do
            dir=$(dirname -- "$tag")
            # De-dupe: the two roots can overlap in principle (e.g. via a
            # symlink); never process the same real path twice.
            local real; real=$(cd -- "$dir" 2>/dev/null && pwd -P) || real="$dir"
            [[ "$seen" == *"|$real|"* ]] && continue
            seen="$seen|$real|"

            if ! is_cargo_target_dir "$real"; then
                # A CACHEDIR.TAG from some other tool (fontconfig, uv,
                # man-db, ...) sharing the scan root. Not our business.
                continue
            fi

            if is_forbidden_path "$real"; then
                SKIPPED_LINES+=("cargo-target  $real  SKIPPED (forbidden path)")
                continue
            fi

            size=$(dir_size_bytes "$real")
            [[ "$size" =~ ^[0-9]+$ ]] || size=0

            if target_dir_in_use "$real"; then
                say "    SKIP  $real  ($(human_size "$size"))  — a live cargo/rustc process is using it"
                SKIPPED_LINES+=("cargo-target  $real  $(human_size "$size")  SKIPPED (in use)")
                continue
            fi

            if ! older_than_threshold "$real" "$OLDER_THAN_HOURS"; then
                say "    SKIP  $real  ($(human_size "$size"))  — modified within the last ${OLDER_THAN_HOURS}h"
                SKIPPED_LINES+=("cargo-target  $real  $(human_size "$size")  SKIPPED (too young)")
                continue
            fi

            if (( APPLY == 1 )); then
                if safe_rm_rf "$real" "cargo-target"; then
                    say "    REMOVED  $real  ($(human_size "$size"))"
                    RECLAIMED_LINES+=("cargo-target  $real  $(human_size "$size")")
                    TOTAL_RECLAIMED_BYTES=$(( TOTAL_RECLAIMED_BYTES + size ))
                    ANY_APPLIED=1
                fi
            else
                say "    WOULD REMOVE  $real  ($(human_size "$size"))"
                RECLAIMED_LINES+=("cargo-target  $real  $(human_size "$size")")
                TOTAL_RECLAIMED_BYTES=$(( TOTAL_RECLAIMED_BYTES + size ))
            fi
        done < <(find "$root" -maxdepth 6 -name CACHEDIR.TAG -print0 2>/dev/null)
    done
}

# ---------------------------------------------------------------------------
# Category: verify-logs
# ---------------------------------------------------------------------------

reclaim_verify_logs() {
    say "== verify-logs — stale scripts/verify.sh log directories =="
    local root="$VERIFY_TMP_ROOT" dir size

    [[ -d "$root" ]] || { say "    (root does not exist, skipping: $root)"; return; }

    while IFS= read -r -d '' dir; do
        if is_forbidden_path "$dir"; then
            SKIPPED_LINES+=("verify-logs  $dir  SKIPPED (forbidden path)")
            continue
        fi

        size=$(dir_size_bytes "$dir")
        [[ "$size" =~ ^[0-9]+$ ]] || size=0

        if ! older_than_threshold "$dir" "$OLDER_THAN_HOURS"; then
            say "    SKIP  $dir  ($(human_size "$size"))  — modified within the last ${OLDER_THAN_HOURS}h"
            SKIPPED_LINES+=("verify-logs  $dir  $(human_size "$size")  SKIPPED (too young)")
            continue
        fi

        if any_verify_running; then
            say "    SKIP  $dir  ($(human_size "$size"))  — a verify.sh/cargo/rustc process is currently live; leaving all verify-log dirs alone this run"
            SKIPPED_LINES+=("verify-logs  $dir  $(human_size "$size")  SKIPPED (a verify run is live)")
            continue
        fi

        if (( APPLY == 1 )); then
            if safe_rm_rf "$dir" "verify-logs"; then
                say "    REMOVED  $dir  ($(human_size "$size"))"
                RECLAIMED_LINES+=("verify-logs  $dir  $(human_size "$size")")
                TOTAL_RECLAIMED_BYTES=$(( TOTAL_RECLAIMED_BYTES + size ))
                ANY_APPLIED=1
            fi
        else
            say "    WOULD REMOVE  $dir  ($(human_size "$size"))"
            RECLAIMED_LINES+=("verify-logs  $dir  $(human_size "$size")")
            TOTAL_RECLAIMED_BYTES=$(( TOTAL_RECLAIMED_BYTES + size ))
        fi
    done < <(find "$root" -maxdepth 1 -type d -name 'codex-verify-*' -print0 2>/dev/null)
}

# ---------------------------------------------------------------------------
# Category: worktrees
#
# Only touches worktrees `git worktree list` itself knows about — the
# standard, safe mechanism (`git worktree remove` refuses a dirty tree on its
# own, and this script checks first anyway so it can report clearly rather
# than fail obscurely).
# ---------------------------------------------------------------------------

pr_state_for_branch() {
    local branch="$1"
    command -v gh >/dev/null 2>&1 || { printf 'UNKNOWN'; return; }
    gh pr list --state all --head "$branch" --json state --jq '.[0].state // "NONE"' 2>/dev/null || printf 'UNKNOWN'
}

reclaim_worktrees() {
    say "== worktrees — merged/closed-PR git worktrees =="
    local main_worktree; main_worktree="$REPO_ROOT"
    local wt branch dirty ahead pr_state merged_locally eligible reason

    while IFS= read -r wt; do
        [[ -z "$wt" ]] && continue
        local real; real=$(cd -- "$wt" 2>/dev/null && pwd -P) || real="$wt"
        [[ "$real" == "$(cd -- "$main_worktree" && pwd -P)" ]] && continue

        if is_forbidden_path "$real"; then
            SKIPPED_LINES+=("worktrees  $real  SKIPPED (forbidden path)")
            continue
        fi
        if [[ ! -d "$real" ]]; then
            # Worktree metadata exists but the dir is gone already; let
            # `git worktree prune` (run unconditionally below) handle it.
            continue
        fi

        branch=$(git -C "$real" rev-parse --abbrev-ref HEAD 2>/dev/null || true)

        dirty=$(git -C "$real" status --porcelain 2>/dev/null)
        if [[ -n "$dirty" ]]; then
            say "    SKIP  $real  (branch: ${branch:-detached})  — uncommitted changes present"
            SKIPPED_LINES+=("worktrees  $real  SKIPPED (uncommitted changes)")
            continue
        fi

        ahead=0
        if git -C "$real" rev-parse --abbrev-ref --symbolic-full-name '@{u}' >/dev/null 2>&1; then
            ahead=$(git -C "$real" rev-list --count '@{u}..HEAD' 2>/dev/null || echo 0)
        else
            # No upstream at all: treat as unpushed, refuse to touch.
            ahead=1
        fi
        if [[ "$ahead" != "0" ]]; then
            say "    SKIP  $real  (branch: ${branch:-detached})  — $ahead unpushed commit(s)"
            SKIPPED_LINES+=("worktrees  $real  SKIPPED (unpushed commits)")
            continue
        fi

        merged_locally=no
        if [[ -n "$branch" ]] && git -C "$REPO_ROOT" branch --merged "$DEVELOP_REF" 2>/dev/null | sed 's/^[*+ ] //' | grep -qxF "$branch"; then
            merged_locally=yes
        fi

        pr_state=$(pr_state_for_branch "$branch")

        eligible=no
        reason=""
        if [[ "$merged_locally" == yes ]]; then
            eligible=yes; reason="branch merged into $DEVELOP_REF"
        elif [[ "$pr_state" == MERGED || "$pr_state" == CLOSED ]]; then
            eligible=yes; reason="PR state $pr_state"
        fi

        if [[ "$eligible" != yes ]]; then
            say "    SKIP  $real  (branch: ${branch:-detached})  — not merged into $DEVELOP_REF and PR state is ${pr_state} (not MERGED/CLOSED)"
            SKIPPED_LINES+=("worktrees  $real  SKIPPED (not merged/closed)")
            continue
        fi

        if (( APPLY == 1 )); then
            # Deliberately NOT --force: `git worktree remove` refuses on its
            # own for reasons this script does not separately check (a
            # locked worktree, a dirty submodule) — that refusal is a second,
            # independent safety layer on top of the dirty/unpushed checks
            # above, not a redundant obstacle to work around.
            if git -C "$REPO_ROOT" worktree remove "$real" 2>/dev/null; then
                say "    REMOVED  $real  (branch: $branch, $reason)"
                RECLAIMED_LINES+=("worktrees  $real  ($reason)")
                ANY_APPLIED=1
            else
                say "    FAILED to remove worktree $real — git refused (locked, dirty submodule, or similar); leaving it in place"
                SKIPPED_LINES+=("worktrees  $real  SKIPPED (git worktree remove refused)")
            fi
        else
            say "    WOULD REMOVE  $real  (branch: $branch, $reason)"
            RECLAIMED_LINES+=("worktrees  $real  ($reason)")
        fi
    done < <(git -C "$REPO_ROOT" worktree list --porcelain 2>/dev/null | awk '/^worktree /{ $1=""; sub(/^ /,""); print }')

    if (( APPLY == 1 )); then
        git -C "$REPO_ROOT" worktree prune 2>/dev/null || true
    fi
}

# ---------------------------------------------------------------------------
# Category: branches
# ---------------------------------------------------------------------------

reclaim_branches() {
    say "== branches — local branches merged into $DEVELOP_REF or gone from origin =="
    local current_branch checked_out_branches b upstream_status

    current_branch=$(git -C "$REPO_ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || true)
    checked_out_branches=$(git -C "$REPO_ROOT" worktree list --porcelain 2>/dev/null | awk '/^branch /{ sub(/^branch refs\/heads\//,""); print }')

    while IFS= read -r b; do
        [[ -z "$b" ]] && continue
        case "$b" in
            develop|main|master) continue ;;
        esac
        if [[ "$b" == "$current_branch" ]]; then
            continue
        fi
        if printf '%s\n' "$checked_out_branches" | grep -qxF "$b"; then
            SKIPPED_LINES+=("branches  $b  SKIPPED (checked out in a worktree)")
            continue
        fi

        local merged=no
        if git -C "$REPO_ROOT" branch --merged "$DEVELOP_REF" 2>/dev/null | sed 's/^[*+ ] //' | grep -qxF "$b"; then
            merged=yes
        fi

        upstream_status=$(git -C "$REPO_ROOT" for-each-ref --format='%(upstream:track)' "refs/heads/$b" 2>/dev/null)
        local gone=no
        [[ "$upstream_status" == "[gone]" ]] && gone=yes

        if [[ "$merged" != yes && "$gone" != yes ]]; then
            say "    SKIP  $b  — not merged into $DEVELOP_REF and upstream is not gone"
            SKIPPED_LINES+=("branches  $b  SKIPPED (not merged, upstream present)")
            continue
        fi

        local why="merged into $DEVELOP_REF"
        [[ "$gone" == yes && "$merged" != yes ]] && why="upstream gone"

        if (( APPLY == 1 )); then
            if git -C "$REPO_ROOT" branch -d "$b" >/dev/null 2>&1; then
                say "    DELETED  $b  ($why)"
                RECLAIMED_LINES+=("branches  $b  ($why)")
                ANY_APPLIED=1
            else
                say "    SKIP  $b  — git branch -d refused (not fully merged into current branch); leaving it"
                SKIPPED_LINES+=("branches  $b  SKIPPED (git refused -d, not fully merged)")
            fi
        else
            say "    WOULD DELETE  $b  ($why)"
            RECLAIMED_LINES+=("branches  $b  ($why)")
        fi
    done < <(git -C "$REPO_ROOT" for-each-ref refs/heads --format='%(refname:short)' 2>/dev/null)
}

# ---------------------------------------------------------------------------
# Retro event
# ---------------------------------------------------------------------------

emit_retro_event() {
    (( ANY_APPLIED == 1 )) || return 0
    [[ -z "${RETRO_DISABLE:-}" ]] || return 0
    local emitter="$SCRIPT_DIR/retro.py"
    [[ -f "$emitter" ]] || return 0
    command -v python3 >/dev/null 2>&1 || return 0

    local n=${#RECLAIMED_LINES[@]}
    python3 "$emitter" incident \
        --source reclaim.sh --derived \
        --impact "reclaim.sh --apply removed $n item(s), $(human_size "$TOTAL_RECLAIMED_BYTES") total, across categories: ${SELECTED[*]}" \
        --detected-by "scripts/reclaim.sh scanning $SCRATCHPAD_ROOT, $CACHE_ROOT, $VERIFY_TMP_ROOT, and git worktree/branch state" \
        --recurrence-key disk-full \
        --resolution "automated reclaim: ${RECLAIMED_LINES[*]}" \
        --quiet >/dev/null 2>&1 || true
}

# ---------------------------------------------------------------------------
# Run
# ---------------------------------------------------------------------------

say "codex reclaim — mode: $MODE_LABEL, older-than: ${OLDER_THAN_HOURS}h, categories: ${SELECTED[*]}"
say "repo:              $REPO_ROOT"
say "scratchpad root:    $SCRATCHPAD_ROOT"
say "cache root:         $CACHE_ROOT"
say "verify-tmp root:    $VERIFY_TMP_ROOT"
hr

for cat in "${SELECTED[@]}"; do
    case "$cat" in
        cargo-target) reclaim_cargo_target ;;
        verify-logs)  reclaim_verify_logs ;;
        worktrees)    reclaim_worktrees ;;
        branches)     reclaim_branches ;;
    esac
    hr
done

say "SUMMARY"
if (( APPLY == 1 )); then
    say "  reclaimed: ${#RECLAIMED_LINES[@]} item(s), $(human_size "$TOTAL_RECLAIMED_BYTES") total"
else
    say "  would reclaim: ${#RECLAIMED_LINES[@]} item(s), $(human_size "$TOTAL_RECLAIMED_BYTES") total (DRY RUN — pass --apply to actually remove)"
fi
if (( ${#SKIPPED_LINES[@]} > 0 )); then
    say "  skipped: ${#SKIPPED_LINES[@]} item(s)"
    for s in "${SKIPPED_LINES[@]}"; do
        say "    - $s"
    done
fi

emit_retro_event

exit 0
