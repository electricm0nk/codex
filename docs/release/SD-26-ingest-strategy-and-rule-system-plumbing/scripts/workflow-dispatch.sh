#!/usr/bin/env bash
# SD-26 Workflow orchestrator.
# Author-once; runs continuously while the bundle dispatches cycles.
# Per /governance/loop-instruction-template.md §2 + skill workflow-orchestrated-dispatch.
#
# Mechanically identical to SD-25's orchestrator at the dispatch-mechanism level;
# concurrency map and tiering overrides are different (see decisions.md §3).

set -euo pipefail

BUNDLE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PROGRESS_DOC="${BUNDLE_DIR}/progress.md"
CYCLE_DOCS_DIR="${BUNDLE_DIR}/cycles"
BUNDLE_BRANCH="${BUNDLE_BRANCH:-tranche/5-4}"
KANBAN_BOARD="${KANBAN_BOARD:-codex-tranche-5}"
BASE_CADENCE="${BASE_CADENCE:-60}"

DEFAULT_SUBAGENT_MODEL="sonnet"

# Per-epic concurrency + tiering overrides (decisions.md §3).
declare -A EPIC_PARALLEL EPIC_SUBAGENT
EPIC_PARALLEL[1]="no" ; EPIC_SUBAGENT[1]="sonnet"   # Identifier Cleanup
EPIC_PARALLEL[2]="no" ; EPIC_SUBAGENT[2]="sonnet"   # Oracle-Harness Comparator
EPIC_PARALLEL[3]="yes"; EPIC_SUBAGENT[3]="sonnet"   # JSON Cache Build (per-book parallel)
EPIC_PARALLEL[4]="yes"; EPIC_SUBAGENT[4]="sonnet"   # Book Stub Manifest
EPIC_PARALLEL[5]="no" ; EPIC_SUBAGENT[5]="sonnet"   # Doctrine-Cost Reduction
EPIC_PARALLEL[6]="no" ; EPIC_SUBAGENT[6]="haiku"    # Closure (haiku-defaulted; opus on adversarial step)

# Per-criterion tiering override for E6
declare -A SUBAGENT_OVERRIDE
SUBAGENT_OVERRIDE["6.3"]="haiku"   # release-notes
SUBAGENT_OVERRIDE["6.4"]="haiku"   # version-bump (0.5.98 → 0.5.99)
SUBAGENT_OVERRIDE["6.2"]="opus"    # architecture closure-pipeline + adversarial verify

CYCLE_COUNTER=0

log() { printf '[%s] %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$*" ; }

pick_next_criterion() {
    local epic_n criterion_id
    for epic_n in 1 2 3 4 5 6; do
        local todo_rows
        todo_rows=$(grep -E "^\| ${epic_n}\.[0-9]+ \|" "${PROGRESS_DOC}" 2>/dev/null \
                    | grep -E '\| pending \|' | head -1 || true)
        if [ -n "${todo_rows}" ]; then
            criterion_id=$(echo "${todo_rows}" | awk -F'|' '{print $2}' | xargs)
            echo "${epic_n} ${criterion_id}"
            return
        fi
    done
    local disc_entry
    disc_entry=$(grep -E '^\| HIGH \|' "${PROGRESS_DOC}" 2>/dev/null | head -1 || true)
    if [ -n "${disc_entry}" ]; then
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
        local on_branch
        on_branch=$(git -C "${BUNDLE_DIR}" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
        if [ "${on_branch}" != "${BUNDLE_BRANCH}" ]; then
            log "not on ${BUNDLE_BRANCH} (current=${on_branch}); checkout and pull"
            git -C "${BUNDLE_DIR}" fetch origin "${BUNDLE_BRANCH}" || true
            git -C "${BUNDLE_DIR}" checkout "${BUNDLE_BRANCH}" 2>&1 | grep -v 'Switched' || true
        fi

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

        local parallel subagent_model
        parallel="${EPIC_PARALLEL[${epic_n}]:-no}"
        subagent_model="${EPIC_SUBAGENT[${epic_n}]:-${DEFAULT_SUBAGENT_MODEL}}"

        local criterion_key="${epic_n}.${criterion_id##*.}"
        local overridden
        overridden="${SUBAGENT_OVERRIDE[${criterion_key}]:-}"
        if [ -n "${overridden}" ]; then
            subagent_model="${overridden}"
        fi

        local cycle_doc="${CYCLE_DOCS_DIR}/${epic_n}_${criterion_id//\./_}.md"
        if [ ! -f "${cycle_doc}" ]; then
            log "WARN no cycle doc at ${cycle_doc}; treating criterion as TODO-only. Writing a default."
            cat > "${cycle_doc}" <<DEFAULT
# Cycle ${cycle_id} — Epic ${epic_n} / Criterion ${criterion_id}

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

        claude code \
            --profile god-emporer \
            --model "${subagent_model}" \
            ${isolation_flag} \
            --task "${cycle_doc}" 2>&1 | tee -a "${BUNDLE_DIR}/artifacts/orchestrator.log"

        if ! git -C "${BUNDLE_DIR}" log --oneline "origin/${BUNDLE_BRANCH}..HEAD" 2>/dev/null | grep -q .; then
            log "WARN cycle did not produce a new commit on ${BUNDLE_BRANCH}; checking claim"
        fi

        sleep "${BASE_CADENCE}"
    done
}

main_loop
