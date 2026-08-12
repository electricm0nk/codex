#!/usr/bin/env bash
# SD-25 Workflow orchestrator — REFERENCE SPEC, NOT A LIVE DISPATCHER.
#
# VERIFIED 2026-07-21 (decisions.md §10): the `claude code --profile … --task …`
# invocation below (see the Tier 5 block) does not exist in the live CLI —
# `claude --help` has no `code` subcommand and no `--profile`/`--task` flags.
# As written, this script's dispatch step would fail every time and the
# main_loop would spin on its `sleep ${BASE_CADENCE}` branch forever, exactly
# the silent-no-op failure mode the workflow-orchestrated-dispatch skill's
# v1.1.0 warning predicted. SD-25 is dispatched by the in-harness `Workflow`
# tool, driven from a live session, NOT by running this file as a background
# process. This script stays in the repo as the deterministic per-epic
# concurrency + tiering SPEC — EPIC_PARALLEL / EPIC_SUBAGENT /
# PARALLEL_OVERRIDE / SUBAGENT_OVERRIDE below are the source of truth each
# Workflow call reads from and honors — plus `pick_next_criterion()` as a
# reference implementation of the selection logic a Workflow script can port.
#
# Per /governance/loop-instruction-template.md §2 + skill workflow-orchestrated-dispatch.
#
# Fires every 60s (operator-pinned default; the operator can change BASE_CADENCE).
# Reads the loop-instruction's per-epic concurrency map from decisions.md §3.
# Picks the next unclaimed criterion from progress.md ## TODO + ## DISCOVERED.
# Dispatches it to a Sonnet subagent (Haiku for E8 housekeeping; Opus for E8 adversarial-verify).
# Applies the canonical concurrent-write protocol (loop-instruction.md §5).

set -euo pipefail

BUNDLE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PROGRESS_DOC="${BUNDLE_DIR}/progress.md"
CYCLE_DOCS_DIR="${BUNDLE_DIR}/cycles"
ARTIFACTS_DIR="${BUNDLE_DIR}/artifacts"
BUNDLE_BRANCH="${BUNDLE_BRANCH:-tranche/5-3}"
KANBAN_BOARD="${KANBAN_BOARD:-codex-tranche-5}"
BASE_CADENCE="${BASE_CADENCE:-60}"

# Default subagent model tier — overridden per-epic by the concurrency map below.
DEFAULT_SUBAGENT_MODEL="sonnet"

# Per-epic concurrency + tiering overrides (decisions.md §3 + loop-instruction.md §3).
declare -A EPIC_PARALLEL EPIC_SUBAGENT
EPIC_PARALLEL[1]="no" ; EPIC_SUBAGENT[1]="sonnet"   # Identifier Cleanup
EPIC_PARALLEL[2]="no" ; EPIC_SUBAGENT[2]="sonnet"   # Operator Pre-Launch
EPIC_PARALLEL[3]="yes"; EPIC_SUBAGENT[3]="sonnet"   # Hub-of-Hubs (criterion 3.4 = no)
EPIC_PARALLEL[4]="yes"; EPIC_SUBAGENT[4]="sonnet"   # PCGen Runner (criterion 4.4 = no)
EPIC_PARALLEL[5]="no" ; EPIC_SUBAGENT[5]="sonnet"   # Corpus Ingest Diagnostic
EPIC_PARALLEL[6]="no" ; EPIC_SUBAGENT[6]="sonnet"   # UI-Eval Defects (dynamic)
EPIC_PARALLEL[7]="no" ; EPIC_SUBAGENT[7]="sonnet"   # Deferred Per-Class Work (7.N corpus intake = yes, see override)
EPIC_PARALLEL[8]="no" ; EPIC_SUBAGENT[8]="haiku"    # Closure (most sub-steps haiku; final verify opus)

# Per-criterion concurrency overrides (decisions.md §3 / loop-instruction.md §3):
# epic-level yes with a serial exception, or epic-level no with a parallel exception.
declare -A PARALLEL_OVERRIDE
PARALLEL_OVERRIDE["3.4"]="no"    # multi-file Tauri command routing, serial
PARALLEL_OVERRIDE["4.4"]="no"    # multi-artifact verification, serial
PARALLEL_OVERRIDE["7.N"]="yes"   # SD-24 carry-forward corpus intake: 4 disjoint per-book cycles, worktree isolation

# Tiering override for E8.3 (release-notes) + E8.4 (version-bump) + adversarial pass:
declare -A SUBAGENT_OVERRIDE
SUBAGENT_OVERRIDE["7.P"]="haiku"   # SD-24 documentation-staleness batch (register §B) — mechanical doc edits
SUBAGENT_OVERRIDE["8.3"]="haiku"
SUBAGENT_OVERRIDE["8.4"]="haiku"
SUBAGENT_OVERRIDE["8.1.final"]="opus"

# Incrementing cycle-id counter (lives in memory; reset by CLAIM-EXISTS).
CYCLE_COUNTER=0

log() { printf '[%s] %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$*" ; }

# Tier 1: pick the next criterion from the progress doc.
# Reads ## TODO first (deterministic seed) then ## DISCOVERED (priority-bumped dynamic items).
pick_next_criterion() {
    local epic_n criterion_id touched_parallel_subdoc touched_files

    # Walk ## TODO first (in epic-number, criterion-number order).
    # Status-matrix rows look like "| 1.1 Source-code identifier audit | not-started | ... |":
    # the criterion id is the first token of the first cell, the state is the second cell.
    # Criterion ids can be alphabetic (7.N, 7.O); dynamic placeholder rows (6.2..6.N, 7.2..7.M)
    # carry state "dynamic-pending" so this "not-started" filter skips them until spawned.
    for epic_n in 1 2 3 4 5 6 7 8; do
        local todo_rows
        todo_rows=$(grep -E "^\| ${epic_n}\.[0-9A-Z]+ " "${PROGRESS_DOC}" 2>/dev/null \
                    | grep -E '\| not-started \|' | head -1 || true)
        if [ -n "${todo_rows}" ]; then
            criterion_id=$(echo "${todo_rows}" | awk -F'|' '{print $2}' | awk '{print $1}')
            echo "${epic_n} ${criterion_id}"
            return
        fi
    done

    # Then ## DISCOVERED, priority-bump-tag order (HIGH first).
    local disc_entry
    disc_entry=$(grep -E '^\| HIGH \|' "${PROGRESS_DOC}" 2>/dev/null | head -1 || true)
    if [ -n "${disc_entry}" ]; then
        # Format: "<ISO-8601> | <epic-of-origin> | <criterion-of-origin> | <priority-bump-tag> | <description> | <suggested-epic-and-criterion>"
        local origin_epic suggested_criterion
        origin_epic=$(echo "${disc_entry}" | awk -F'|' '{print $3}' | xargs)
        suggested_criterion=$(echo "${disc_entry}" | awk -F'|' '{print $7}' | xargs)
        if [ -n "${origin_epic}" ] && [ -n "${suggested_criterion}" ]; then
            echo "${origin_epic} ${suggested_criterion}"
            return
        fi
    fi

    echo ""
}

main_loop() {
    while true; do
        # Tier 2: pre-launch check — working tree clean + on the bundle branch.
        local on_branch
        on_branch=$(git -C "${BUNDLE_DIR}" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
        if [ "${on_branch}" != "${BUNDLE_BRANCH}" ]; then
            log "not on ${BUNDLE_BRANCH} (current=${on_branch}); checkout and pull"
            git -C "${BUNDLE_DIR}" fetch origin "${BUNDLE_BRANCH}" || true
            git -C "${BUNDLE_DIR}" checkout "${BUNDLE_BRANCH}" 2>&1 | grep -v 'Switched' || true
        fi

        # Tier 3: pick the next criterion (returns empty if nothing eligible).
        local picked
        picked=$(pick_next_criterion)
        if [ -z "${picked}" ]; then
            log "no eligible criterion; sleeping ${BASE_CADENCE}s"
            sleep "${BASE_CADENCE}"
            continue
        fi

        local epic_n criterion_id
        epic_n=$(echo "${picked}" | awk '{print $1}')
        criterion_id=$(echo "${picked}" | awk '{print $2}')
        CYCLE_COUNTER=$((CYCLE_COUNTER + 1))
        local cycle_id
        cycle_id="cycle-$(date -u +%Y%m%dT%H%M%S)Z-${CYCLE_COUNTER}"

        log "dispatching E${epic_n} ${criterion_id} (cycle=${cycle_id})"

        # Tier 4: per-epic concurrency + per-criterion tiering.
        local parallel subagent_model
        parallel="${EPIC_PARALLEL[${epic_n}]:-no}"
        subagent_model="${EPIC_SUBAGENT[${epic_n}]:-${DEFAULT_SUBAGENT_MODEL}}"

        # Override on per-criterion basis if present.
        local criterion_key="${epic_n}.${criterion_id##*.}"
        local overridden
        overridden="${SUBAGENT_OVERRIDE[${criterion_key}]:-}"
        if [ -n "${overridden}" ]; then
            subagent_model="${overridden}"
        fi
        local parallel_overridden
        parallel_overridden="${PARALLEL_OVERRIDE[${criterion_key}]:-}"
        if [ -n "${parallel_overridden}" ]; then
            parallel="${parallel_overridden}"
        fi

        # Tier 5: invoke the cycle task document with the chosen subagent.
        # Doc name matches epic-breakdown.md's "./cycles/<epic>_<criterion>.md" (e.g. cycles/1_1.md
        # for criterion 1.1) — criterion_id already carries the epic prefix.
        local cycle_doc="${CYCLE_DOCS_DIR}/${criterion_id//\./_}.md"
        if [ ! -f "${cycle_doc}" ]; then
            log "WARN no cycle doc at ${cycle_doc}; treating criterion as TODO-only. Writing a default."
            cat > "${cycle_doc}" <<DEFAULT
# Cycle ${cycle_id} — Epic ${epic_n} / Criterion ${criterion_id}

## FILE-TOUCH GRANT

(Read this criterion's OWN row in epic-breakdown.md §Epic ${epic_n} — per loop-instruction.md §6
and carry-forward register item C1, the grant must never be copied from a prior cycle's block.)

## RED

(Not yet authored. Author the failing test before any production change.)

## GREEN

(Implement per the per-cycle story in epic-breakdown.md §Epic ${epic_n}.)

## RE-AUDIT

(Run loop-instruction.md §6 dual-audit; capture OK_NO_BUNDLE_TAGS + OK_NO_TOKENS.)
DEFAULT
        fi

        local isolation_flag=""
        if [ "${parallel}" = "yes" ]; then
            isolation_flag="--isolation worktree"
        fi

        # The orchestrator invokes the coding harness with the chosen subagent model.
        # The harness reads ${cycle_doc} + loop-instruction.md + epic-breakdown.md §Epic ${epic_n}.
        claude code \
            --profile god-emporer \
            --model "${subagent_model}" \
            ${isolation_flag} \
            --task "${cycle_doc}" 2>&1 | tee -a "${BUNDLE_DIR}/artifacts/orchestrator.log"

        # Tier 6: apply concurrent-write protocol (after the cycle completes its own write).
        # The cycle's Step 6/7 in loop-instruction.md is responsible for the actual commit + push;
        # the orchestrator just verifies the push landed.
        if ! git -C "${BUNDLE_DIR}" log --oneline "origin/${BUNDLE_BRANCH}..HEAD" 2>/dev/null | grep -q .; then
            log "WARN cycle did not produce a new commit on ${BUNDLE_BRANCH}; checking claim"
        fi

        # Short restart before next tick — the actual cadence floor is BASE_CADENCE.
        sleep "${BASE_CADENCE}"
    done
}

main_loop
