---
canonical: true
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
date: 2026-11-22
status: planning — schema only; no receipt yet
---

# SD-43 Receipts

This file has two jobs: §1 holds the YAML receipts the closure scripts append (architecture truth-up and its
claims critic at E7.3; graphify at C2 and E7.4); §2 defines the per-cycle receipt every dispatched card writes
(`workflow-instruction.md §7` points here).

## 1. Closure-script receipts (appended by E7.3 and E7.4; C2 writes its own under `artifacts/cycle_0/`)

Each block is appended by the script, never hand-edited except to add a second, hand-mapped block when the
truth-up script maps 0 docs (SD-39…SD-42 precedent), marked `supersedes_note_on: <cycle_id>`. Every graphify
receipt records the indexed SHA and the empty `git status --porcelain` taken immediately before the run, and the
node counts before and after.

(no receipts yet)

## 2. Per-cycle receipt (every dispatched card)

**Path convention:** `artifacts/<epic>/<card-id>_cycle_receipt.md`, where `<epic>` is `cycle_0` for C0–C2,
`epic_0` … `epic_7` for E0–E7 (e.g. `artifacts/epic_3/E3.4_cycle_receipt.md`, `artifacts/epic_4/E4.G_cycle_receipt.md`).
Evidence beside it under `artifacts/<epic>/<card-id>/` (screenshots, PDFs, logs, census outputs).

**Schema** (every field present; "n/a" with a reason where a field does not apply):

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** opus | sonnet | haiku   **RETRO_ACTOR:** sd43-<card-id>   **RUN_DESKTOP_AGENT:** <value>
- **Commit SHA(s):** <sha>   **Base SHA:** <tranche-43 sha at start>   **Lane / tree:** <lane> / <path>
- **Files touched:** <list> (each inside FILES YOU OWN, or a named fence exit with its retro incident)
- **FILES YOU OWN closure check:** <producers git-grepped; second call sites; registrations; DTO producers; bundle.resources + packaged_resources.rs if a runtime read was added>
- **Acceptance criterion (verbatim from epic-breakdown.md and acceptance-and-verification.md):** <text>
- **RED (pre-change command, exit code, failing line quoted; equals the lead's dispatch-log RED):** <…>
- **GREEN (same command, output, `N passed` line):** <…>
- **Figures:** <value> — <predicate and denominator> — <command> → <output> (a second implementation when it moves a baseline)
- **Build scope verified:** root | apps/desktop/src-tauri | frontend — with the log path; every `test result: FAILED` attributed to its `Running` line by name
- **Clippy before first commit:** <result>   **pcgen-residue-gate:** PASS | n/a (no .rs change)
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations with commit>
- **Wired-integration four-check audit** (`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit", run on added lines per `workflow-instruction.md §6` step 5):
  - check 1 forbidden tokens: OK_NO_TOKENS | <violations>
  - check 2 no-op handlers: OK_NO_NOOP_HANDLERS | <violations>
  - check 3 mock leaks: OK_NO_MOCK_LEAKS | <violations>
  - check 4 "Would …" strings: OK_NO_WOULD_STRINGS | <violations>
  - (the four commands' raw output pasted below this list, not paraphrased)
- **Looked at (screen cards and every MC):** <PNG/PDF paths; passes; what was seen; differences left and why>
- **Rulings and defaults applied:** <decisions.md §3 ref / §30 Q-id + the alternative not taken>
- **Does not cover:** <the shapes this proof does not exercise, with the population and its denominator>
- **Status:** complete | partial | blocked-escalated | declined
- **Remainder (partial only):** <every remaining item by sub-cause; the populations sum to the total>
- **Retro events emitted:** <types + counts, or "none">
- **Discovered cards:** <ids added to kanban.md + epic-breakdown.md §0 + the G-CL count, same commit>
- **Next-cycle plan:** <…>
```

**MC receipts add:** the epic's retro summary (`scripts/retro.py summary --since <epic-start> --json` counts and
any key firing more than once), the lane sweep's found/removed counts, and the four-check audit over the whole
epic's diff. **E4.G's receipt** quotes the operator's reply verbatim with its date (the same line as
`progress.md ## Operator replies`). **E7.2's receipt** adds the G-CL and G-R outputs, the fresh-install
observations per artifact, and every `forward-scope-register.md §1` revisit command's output.

A receipt that lacks any of `closure check`, `RED`, `Does not cover` (and `Looked at` for an MC) fails the
G-CL greps, and the lead checks it when the card returns (`workflow-instruction.md §2.2`).
