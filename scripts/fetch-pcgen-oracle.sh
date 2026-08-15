#!/usr/bin/env bash
#
# fetch-pcgen-oracle.sh — bootstrap / verify the pinned PCGen oracle checkout.
#
# WHY THIS EXISTS
# ----------------
# Every corpus instrument (`corpus_literal_sweep`, `v06_work_inventory`,
# `v06_corpus_trap_report`, `oracle_validation::pcgen_runner`, ...) resolves
# the PCGen oracle via `$PCGEN_CORPUS_ROOT` (default
# `$HOME/workspace/repos/pcgen/data`) and `$PCGEN_REPO_DIR` (repo root,
# default `$HOME/workspace/repos/pcgen`). Until `scripts/pcgen-oracle-pin.env`
# existed, nothing pinned WHICH pcgen commit that was — a cloud runner or a
# fresh box could sweep against a different oracle checkout and still report
# CLEAN. This script is the one place that reads the pin, fetches or verifies
# a checkout against it, and prints the exports every other tool needs.
#
# USAGE
# -----
#   scripts/fetch-pcgen-oracle.sh [--dest DIR] [--force] [--check] [--quiet]
#
#   --dest DIR   defaults to $PCGEN_REPO_DIR, else $HOME/workspace/repos/pcgen
#                (matches every binary's default and reclaim.sh's never-touch
#                guard). The corpus root is always $DEST/data.
#   --check      verify only, never touches the network. This is what
#                verify.sh's preflight-oracle stage runs.
#   --force      move an existing off-pin checkout to the pin. Never discards
#                a modified tracked cone file — a dirty tracked file always
#                refuses, --force included.
#   --quiet      suppress the human-readable status line on stderr; the two
#                `export ...` lines still print on stdout, so
#                `eval "$(scripts/fetch-pcgen-oracle.sh --quiet)"` works.
#
# ENV OVERRIDES (for tests)
# --------------------------
#   PCGEN_ORACLE_PIN_FILE   path to the pin file (default: the pin next to
#                            this script, scripts/pcgen-oracle-pin.env).
#
# EXIT CODES
# ----------
#   0  on-pin: checkout HEAD matches the pin, tracked cone files are clean,
#      the corpus dir resolves. Prints `pcgen-oracle: OK <sha> <dest>` then
#      the two export lines.
#   1  absent / off-pin / dirty tracked cone files / refused.
#   2  usage error, pin file unreadable or incomplete, git missing.
#
set -uo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)

PIN_FILE="${PCGEN_ORACLE_PIN_FILE:-$SCRIPT_DIR/pcgen-oracle-pin.env}"

DEST=""
FORCE=0
CHECK=0
QUIET=0

usage() {
    sed -n '3,32p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dest)
            [[ $# -ge 2 ]] || { printf 'fetch-pcgen-oracle.sh: --dest needs a directory\n' >&2; exit 2; }
            DEST="$2"; shift ;;
        --force)   FORCE=1 ;;
        --check)   CHECK=1 ;;
        --quiet)   QUIET=1 ;;
        -h|--help) usage; exit 0 ;;
        *) printf 'fetch-pcgen-oracle.sh: unknown argument: %s\n' "$1" >&2; exit 2 ;;
    esac
    shift
done

command -v git >/dev/null 2>&1 || { printf 'fetch-pcgen-oracle.sh: git not found\n' >&2; exit 2; }

if [[ ! -f "$PIN_FILE" ]]; then
    printf 'fetch-pcgen-oracle.sh: missing pin file: %s\n' "$PIN_FILE" >&2
    exit 2
fi
# shellcheck disable=SC1090
. "$PIN_FILE"

if [[ -z "${PCGEN_ORACLE_REPO:-}" || -z "${PCGEN_ORACLE_SHA:-}" ]]; then
    printf 'fetch-pcgen-oracle.sh: pin file %s missing PCGEN_ORACLE_REPO/PCGEN_ORACLE_SHA\n' "$PIN_FILE" >&2
    exit 2
fi

if [[ -z "$DEST" ]]; then
    DEST="${PCGEN_REPO_DIR:-$HOME/workspace/repos/pcgen}"
fi

# shellcheck disable=SC2206
SPARSE_PATHS=(${PCGEN_ORACLE_SPARSE_PATHS:-data/pathfinder})

log() { (( QUIET )) || printf '%s\n' "$*" >&2; }

print_exports() {
    printf 'export PCGEN_CORPUS_ROOT=%s/data\n' "$DEST"
    printf 'export PCGEN_REPO_DIR=%s\n' "$DEST"
}

corpus_ready() {
    [[ -d "$DEST/data/pathfinder/paizo/roleplaying_game" ]]
}

# Dirty tracked files WITHIN the pinned cone only — untracked files are not
# this script's business, and a change outside the cone is not a drifted
# oracle for anything this repo reads.
dirty_cone_files() {
    ( cd "$DEST" && git status --porcelain --untracked-files=no -- "${SPARSE_PATHS[@]}" ) 2>/dev/null
}

# Verify-only: never touches the network. Assumes $DEST/.git exists.
do_check() {
    local head dirty
    head=$(git -C "$DEST" rev-parse HEAD 2>/dev/null) || {
        printf 'fetch-pcgen-oracle.sh: cannot read HEAD at %s\n' "$DEST" >&2
        return 1
    }
    dirty=$(dirty_cone_files)
    if [[ -n "$dirty" ]]; then
        printf 'fetch-pcgen-oracle.sh: dirty tracked oracle files at %s -- a locally edited .lst is a drifted oracle the SHA check cannot see:\n' "$DEST" >&2
        printf '%s\n' "$dirty" >&2
        return 1
    fi
    if [[ "$head" != "$PCGEN_ORACLE_SHA" ]]; then
        printf 'fetch-pcgen-oracle.sh: %s is off-pin: HEAD=%s pinned=%s\n' "$DEST" "$head" "$PCGEN_ORACLE_SHA" >&2
        printf '    fix with: scripts/fetch-pcgen-oracle.sh --dest %s --force\n' "$DEST" >&2
        return 1
    fi
    if ! corpus_ready; then
        printf 'fetch-pcgen-oracle.sh: %s is at the pinned SHA but data/pathfinder/paizo/roleplaying_game is missing (incomplete sparse cone?)\n' "$DEST" >&2
        return 1
    fi
    log "pcgen-oracle: OK $head $DEST"
    print_exports
    return 0
}

# Moves an EXISTING, CLEAN, off-pin checkout to the pin. Never called on a
# dirty checkout — the caller re-checks immediately before this.
do_fetch_to_pin() {
    if ( cd "$DEST" && git fetch --depth 1 origin "$PCGEN_ORACLE_SHA" ) >/dev/null 2>&1; then
        ( cd "$DEST" && git checkout --detach FETCH_HEAD ) >/dev/null 2>&1
        return $?
    fi
    log "fetch-pcgen-oracle.sh: direct SHA fetch refused by the remote, falling back to git fetch --filter=blob:none origin"
    if ! ( cd "$DEST" && git fetch --filter=blob:none origin ) >/dev/null 2>&1; then
        printf 'fetch-pcgen-oracle.sh: git fetch failed at %s\n' "$DEST" >&2
        return 1
    fi
    ( cd "$DEST" && git checkout --detach "$PCGEN_ORACLE_SHA" ) >/dev/null 2>&1
}

# Creates $DEST from nothing: sparse cone, depth-1, blob-filtered fetch at the
# pinned SHA.
do_fresh_clone() {
    mkdir -p "$DEST" || { printf 'fetch-pcgen-oracle.sh: cannot create %s\n' "$DEST" >&2; return 1; }
    if ! ( cd "$DEST" && git init -q ) >/dev/null 2>&1; then
        printf 'fetch-pcgen-oracle.sh: git init failed at %s\n' "$DEST" >&2
        return 1
    fi
    ( cd "$DEST" && git remote add origin "$PCGEN_ORACLE_REPO" ) >/dev/null 2>&1
    ( cd "$DEST" && git sparse-checkout init --cone ) >/dev/null 2>&1
    ( cd "$DEST" && git sparse-checkout set "${SPARSE_PATHS[@]}" ) >/dev/null 2>&1
    if ( cd "$DEST" && git fetch --depth 1 --filter=blob:none origin "$PCGEN_ORACLE_SHA" ) >/dev/null 2>&1; then
        ( cd "$DEST" && git checkout --detach FETCH_HEAD ) >/dev/null 2>&1
        return $?
    fi
    log "fetch-pcgen-oracle.sh: depth-1 filtered SHA fetch refused by the remote, falling back to git fetch --filter=blob:none origin"
    if ! ( cd "$DEST" && git fetch --filter=blob:none origin ) >/dev/null 2>&1; then
        printf 'fetch-pcgen-oracle.sh: git fetch failed for fresh clone at %s\n' "$DEST" >&2
        return 1
    fi
    ( cd "$DEST" && git checkout --detach "$PCGEN_ORACLE_SHA" ) >/dev/null 2>&1
}

main() {
    if [[ -d "$DEST" ]]; then
        if [[ ! -d "$DEST/.git" ]]; then
            printf 'fetch-pcgen-oracle.sh: %s exists but is not a git checkout -- cannot prove the pin; move it aside and re-run\n' "$DEST" >&2
            exit 1
        fi

        if do_check; then
            exit 0
        fi

        # do_check already printed the specific reason. A dirty tracked cone
        # file is a hard refusal regardless of --force.
        local dirty
        dirty=$(dirty_cone_files)
        if [[ -n "$dirty" ]]; then
            exit 1
        fi

        if (( CHECK )); then
            exit 1
        fi

        if (( ! FORCE )); then
            printf 'fetch-pcgen-oracle.sh: %s is off-pin; re-run with --force to move it to the pin\n' "$DEST" >&2
            exit 1
        fi

        # Re-check immediately before mutating: never discard modified
        # tracked files, even under --force.
        dirty=$(dirty_cone_files)
        if [[ -n "$dirty" ]]; then
            printf 'fetch-pcgen-oracle.sh: refusing --force -- dirty tracked oracle files at %s:\n' "$DEST" >&2
            printf '%s\n' "$dirty" >&2
            exit 1
        fi

        if do_fetch_to_pin && do_check; then
            exit 0
        fi
        printf 'fetch-pcgen-oracle.sh: --force fetch/checkout failed at %s\n' "$DEST" >&2
        exit 1
    fi

    # DEST absent.
    if (( CHECK )); then
        printf 'fetch-pcgen-oracle.sh: no checkout at %s -- run scripts/fetch-pcgen-oracle.sh --dest %s to bootstrap it\n' "$DEST" "$DEST" >&2
        exit 1
    fi

    if do_fresh_clone && do_check; then
        exit 0
    fi
    printf 'fetch-pcgen-oracle.sh: fresh clone failed at %s\n' "$DEST" >&2
    exit 1
}

main
