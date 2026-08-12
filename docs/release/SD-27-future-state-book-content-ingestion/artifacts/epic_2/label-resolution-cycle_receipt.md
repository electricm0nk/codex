# Cycle label-resolution — Epic 2 / Criterion 2.0

- **Cycle ID:** `label-resolution`
- **Criterion:** 2.0
- **Owner:** Backend
- **Status:** complete
- **Route class:** Sonnet
- **Started at:** 2026-07-27T20:10:00Z
- **Completed at:** 2026-07-27T20:16:00Z

## Inputs

- `data/stubs/*.json` (21 files, all carrying `planned_resolution_bundle: "SD-27+ (unscheduled)"`)
- `docs/governance/wired-integration-stubs-registry.md` (21 `book_stub` entries, `#0003`-`#0023`)
- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md:102`
- `docs/release/v0.6/risks-and-open-questions.md` §"Open questions" item 2
- Operator's choice (2026-07-27): **`"SD-27"`**

## Outputs

- All 21 `data/stubs/*.json`: `planned_resolution_bundle` → `"SD-27"`
- All 21 registry `book_stub` entries: `Remediation cycle` → `SD-27`
- `docs/release/v0.6/risks-and-open-questions.md` item 2: reconciliation note appended
- This receipt.

## Operations

1. Pulled the operator's choice: **`"SD-27"`**.
2. Updated all 21 `data/stubs/*.json` files' `planned_resolution_bundle`.
3. Updated all 21 registry entries' `Remediation cycle` field (identical text across all 21; verified
   via grep before editing, so a single substitution safely covered every entry).
4. Checked `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/decisions.md:102` — **already
   correct**, already carries `planned_resolution_bundle: "SD-27"` (SD-26 pinned this as its own default
   at authoring time; no edit needed, verified rather than blindly rewritten).
5. Updated `docs/release/v0.6/risks-and-open-questions.md` item 2 — the single authorized write into
   `docs/release/v0.6/`, per `loop-instruction.md §6`'s cycle-2.0 exception. Appended a reconciliation
   note rather than rewriting v0.6's own historical decision record.
6. Ran the partition self-check: every touched path matched the cycle-2.0 exception exactly (21 stubs +
   registry + v0.6 risks doc; no other file touched).
7. Ran the dual-audit gate against `HEAD~2` (this cycle's own base — the prep + E1.1 commits):
   `OK_NO_BUNDLE_TAGS`, `AUDIT PASSED. All four checks clean.`
8. Commit + push + receipt (this file).

## Verification

- `data/stubs/*.json`: 21/21 carry `"planned_resolution_bundle": "SD-27"` — verified via
  `grep -h planned_resolution_bundle data/stubs/*.json | sort | uniq -c` → `21 "SD-27"`.
- Registry: 21/21 `Remediation cycle` fields carry `SD-27` — verified via the same substitution count
  (21 replacements applied, matching the 21 pre-existing identical lines).
- `decisions.md:102`: confirmed already correct, no drift.
- `docs/release/v0.6/risks-and-open-questions.md`: reconciliation recorded, historical decision text
  preserved (not overwritten).
- Dual-audit gate: clean.

## Notes

- Entries `#0001` and `#0002` in the registry are **not** `book_stub` entries (permanent exceptions with
  their own remediation framing) and were correctly left untouched — only the 21 entries matching
  `data/stubs/*.json` were in scope.
- `beginner_box` and `core_essentials` (descoped from SD-27's in-scope book pair, but still registered
  stubs) were included in this resolution along with the other 19 — the label resolves *which bundle
  number this stub cites*, not *whether the book is in this bundle's active scope*. Their
  `content_kind_counts` stay `null`; only `planned_resolution_bundle` changed.
- This is the bundle's one operator-lever-pull cycle. The choice was **`"SD-27"`**, matching SD-26's own
  original default pin (`decisions.md §10`) — SD-26's decisions.md never actually drifted from this
  value; only the 21 landed stubs and the registry had drifted to `"SD-27+ (unscheduled)"`.
