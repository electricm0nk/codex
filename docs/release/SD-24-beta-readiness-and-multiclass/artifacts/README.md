# SD-24 — Cycle Artifacts Index

> **Operating method:** see `./scope-draft.md` and `./loop-instruction.md`. Per-cycle artifacts land in `./artifacts/<epic>/<cycle-id>_cycle_receipt.md`. This index is appended-to as cycles complete.

The epic subdirectories are pre-created at package construction time. The first cycle of each epic writes its receipt there. Per `./loop-instruction.md §3`, each cycle's artifact follows the schema:

```
# Cycle <cycle-id> — <epic-name> / Criterion <n>
- **Card ID:** t_<hex>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS / <violation list>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Status:** complete | returned-to-backlog | DISCOVERED-forked
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
- **Discovery forwards:** <list of ## DISCOVERED entries added>
- **Next-cycle plan:** <what the next cycle picks up>
```

## Epic subdirectories

| Epic | Subdirectory | Filled by |
|---|---|---|
| Epic 1 — Code-Side Identifier Cleanup | `./epic_1/` | Criterion 1.1, 1.2 cycles |
| Epic 2 — Operator Pre-Launch | `./epic_2/` | Criterion 2.1–2.5 cycles |
| Epic 3 — Wired-Integration Audit + Remediation | `./epic_3/` | Criterion 3.1 (audit) + 3.2–3.4 (remediation) |
| Epic 4 — Per-Class Coverage Audit | `./epic_4/` | Criteria 4.1–4.5 (audit + matrix + plan + deferral) |
| Epic 5 — Multiclass F+W | `./epic_5/` | Criteria 5.1–5.5 (dispatch + fixture + integration + audit + deferral) |
| Epic 6 — Equipment/Armor/Spells 100% | `./epic_6/` | Criteria 6.1 (audit) + 6.2–6.5 (content completion log) |
| Epic 7 — Unwired Workflows + Tauri Surface | `./epic_7/` | Criteria 7.1–7.5 (per-command) |
| Epic 8 — Closure Epilogue | `./epic_8/` | Criteria 8.1–8.4 (final scan + architecture pipeline + release notes + version bump) |

## Closure-readiness report

At Epic 8's Criterion 8.1 (Final criterion scan), the cycle produces `./artifacts/epic_8/closure-readiness-report.md` summarizing the bundle's evaluation: per-criterion state, self-heal cycle count, override flags honored, deferred items.

## Per-cycle dynamic artifacts

Some cycles produce additional artifacts beyond the per-cycle receipt:

- Epic 3: `wired-integration-audit.md` (the read-only sweep output)
- Epic 4: `per-class-coverage-matrix.md`, `remediation-plan.md`, `apg-acg-multiclass-deferred.md`
- Epic 5: `multiclass-fixture.md`, `integration-test-cycle_receipt.md`, `apg-acg-multiclass-deferred.md`
- Epic 6: `equipment-coverage-matrix.md`, `content-completion-log.md`, per-spell `*-cycle_receipt.md`
- Epic 7: `tauri-command-surface.md`, per-command `*-cycle_receipt.md`
- Epic 8: `final-criterion-scan-cycle_receipt.md`, `release-notes-cycle_receipt.md`
