# Cycle 10-R — Epic 2 (Build 8 of 9 tables) / AT-34-E2-002 (reconfirmation at HEAD)

- **Commit SHA:** `c76e1f9455cd8b06f3a9ba9e8792f83dc270f2a5` (HEAD at cycle start — no source
  change was needed, so this cycle's own commit carries only this receipt + `progress.md` +
  `kanban.md`)
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-002_reconfirmation_receipt.md`
  (new, this file), `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`,
  `docs/retro/events/sd34-at-34-e2-002.jsonl` (new)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` for this cycle's own diff. See "Dual audit"
  below for the epic-wide file-touch-set scope.
- **Wired-integration audit result:** `OK_NO_TOKENS` for this cycle's own diff. See "Dual audit"
  below for the epic-wide file-touch-set scope.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "A table returns a real record or a
  named refusal. It never returns a fabricated or defaulted entry. **Evidence:** per table, a
  RED→GREEN pair — observed refusing an absent key, and returning a real record for a present
  one."

## Why this cycle exists

`AT-34-E2-002` was already `complete` on `kanban.md` row 10, built at
`afbe67a22f849fe01408f1da1dc10c652a6dd535` and committed 2026-08-27
(`artifacts/epic-2-tables/AT-34-E2-002_cycle_receipt.md` +
`artifacts/epic-2-tables/fail-closed-proofs.md`). This bundle's standing lesson
(`workflow-instruction.md §12` row 19 / `decisions.md §12` L2 — "never carry your own number
forward, re-derive it") applies to a lane picking a completed criterion back up, not only to a
first cycle: Epic 1 (`AT-34-E1-008`, a corpus-wide `wiring_class` restamp) and Epic 3 have both
landed commits since, touching `src/rules_core/` and `data/corpus/**`. `AT-34-E2-001`'s own
reconfirmation cycle (`c76e1f9455`, immediately prior on this branch) already re-derived that
sibling criterion at HEAD; this cycle does the same for `AT-34-E2-002`'s own bar — per-table
fail-closed proof — rather than re-quoting the 2026-08-27 receipt.

## Re-derivation at HEAD

```
$ cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests
running 19 tests
test rules_core::rules_tables::companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults ... ok
... (18 more, all ok)
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 2905 filtered out

$ cargo test --locked --lib rules_core::rules_tables::simple_kind_tables
running 13 tests
test rules_core::rules_tables::simple_kind_tables::tests::an_absent_key_is_refused_not_fabricated ... ok
... (12 more, all ok)
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 2911 filtered out
```

`companion_chassis` grew from `15` tests (at this criterion's own original commit) to `19` at
HEAD — the `+4` are Epic 3/1 work in the same module (e.g.
`companion_absent_from_core_rulebook_companion_tables_reaches_zero`,
`the_reattributed_familiar_file_ships_no_copy_delta_creature_row`), none of which regressed or
replaced `companion_resolve_refuses_a_fabricated_key_it_never_defaults` — that test still exists,
verbatim, and still passes. `simple_kind_tables` is unchanged at `13`, matching
`AT-34-E2-001`'s own reconfirmation figure exactly (cross-check: both criteria re-derive the same
module count independently and agree).

```
$ cargo build --locked --bin v06_work_inventory
$ ./target/debug/v06_work_inventory --epic2-table-transcript > /tmp/e2002_transcript_check.txt
$ diff /tmp/e2002_transcript_check.txt docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt
(no output)
```

**Byte-identical to the committed transcript.** All 8 of the 8 tables this criterion covers
still report `HELD` on a named record and `REFUSED` on a fabricated key — no drift from the
corpus-wide `wiring_class` restamp (`AT-34-E1-008`) or from Epic 3's later work: neither touches
the `ability`/`template`/`trait`/`deity`/`domain`/`skill`/`language`/`companion` record
*identity* fields that `simple_kind_tables::resolve` and `companion_resolve` read.

```
$ grep -c '^| \`' docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md
8
```

The per-table proof artifact itself is unchanged (`git log --follow` shows one commit,
`afbe67a22f8`) and still names all 8 tables with their resolver, test, and transcript lines —
re-checked against the freshly-run transcript above, which matches every quoted line verbatim.

## Figures + their re-derive commands (every one, with its denominator)

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| Tables carrying a proven RED→GREEN fail-closed pair | `8` | `grep -c '^\| \`' docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md` | of the 8 tables Epic 2 builds (`technical-design.md §4`; `power` is Epic 5's, out of population) |
| `companion_chassis` unit tests passing | `19` | `cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests` | of `19` tests in that module at HEAD (was `15` at this criterion's original commit; `+4` from later Epic 1/3 work, none touching the fail-closed test) |
| `simple_kind_tables` unit tests passing | `13` | `cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` | of `13` tests in that module at HEAD (matches `AT-34-E2-001`'s independently re-derived figure exactly) |
| Transcript re-run vs committed | byte-identical | `diff <(./target/debug/v06_work_inventory --epic2-table-transcript) .../AT-34-E2-001_table_transcript.txt` | of the 8-kind transcript, no diff lines |
| Denominator gate on this package | `violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of `15` files checked |

## Dual audit — Epic 2's own §3 file-touch set

`BASE_BRANCH=ea2b3396f2fde9223dde93522bd2288b463a21ee`, over
`src/bin/v06_work_inventory.rs src/rules_core/ artifacts/epic-2-tables/`, excluding
`__tests__` and `*.test.*`:

```
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS
```

The second grep, over the same scope, returns 18 matches — same false-positive class already
reviewed and closed by `AT-34-E2-001`'s own reconfirmation cycle: the real, correctly-spelled
domain term `placeholder row` (PCGen's own CHOOSE-menu "no selection" rows,
`AT-34-E3-001`'s vacuous-placeholder sub-cause, e.g.
`src/bin/v06_work_inventory.rs:4863`), all inside Epic 3's later commits under
`src/rules_core/` and `src/bin/v06_work_inventory.rs` — none of it this criterion's own
deliverable. Confirmed directly against this criterion's own two files:

```
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/rules_tables/simple_kind_tables.rs \
    src/rules_core/rules_tables/companion_chassis.rs \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS_IN_E2_DELIVERABLE'
OK_NO_TOKENS_IN_E2_DELIVERABLE
```

`simple_kind_tables.rs` and `companion_chassis.rs` (this criterion's own deliverable files)
carry zero hits of either pattern.

## Row-count command output

```
$ grep -c '^| \`' docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md
8
```
8 of 8 tables still carry a proven RED→GREEN fail-closed pair at HEAD, re-derived fresh (test
counts and transcript both re-run, transcript diffed byte-identical) rather than re-quoted from
the original receipt.

## Build scope verified

- `cargo test --locked --no-run` (full root workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-002`) — exit 0.
- `apps/desktop/src-tauri` — **not touched this cycle** (`git diff --name-only <base>..HEAD -- apps/desktop/src-tauri` empty for this cycle's own commit); not run. This is a reconfirmation cycle with no source change.
- Run at SHA: `c76e1f9455cd8b06f3a9ba9e8792f83dc270f2a5` — `git fetch origin tranche/14` returned the same SHA already at HEAD, so no rebase was needed; every command above ran at this SHA.

## Sweep population

N/A — this cycle wrote or regenerated **zero** corpus records
(`git status --porcelain -- data/corpus` empty for the whole cycle). Expected delta from this
cycle alone is `0` — satisfied. The package-wide baseline movement between launch and HEAD is
attributed elsewhere (`AT-34-E2-001`'s reconfirmation receipt reports `+9`/`+9` from other
cycles' corpus additions); not re-litigated here since this cycle touched no corpus record.

## Oracle pin

N/A — no figure in this cycle is drawn from the pinned PCGen oracle corpus.

- **Status:** complete
- **Movement, four buckets:**
  - **closure** — none (the criterion was already `complete`; this cycle reconfirms, it does
    not close anything new).
  - **reclassification** — none. `docs/work-inventory.json` untouched
    (`git status --porcelain -- docs/work-inventory.json` empty).
  - **reachability** — none. No table's resolution logic changed.
  - **instrument-correction** — none. No drift was found between the original commit's
    evidence and HEAD's; this is a confirmed-clean re-derivation, not a fix.
- **Notes (judgment calls):**
  - `companion_chassis`'s test count grew (`15` → `19`) from unrelated Epic 1/3 work in the same
    module; verified none of the four new tests replaced or altered
    `companion_resolve_refuses_a_fabricated_key_it_never_defaults`, and that the resolver's
    fail-closed behavior itself (`companion_resolve`, `src/rules_core/rules_tables/companion_chassis.rs`)
    carries zero diff against `afbe67a22f8`'s committed state.
  - This cycle does not re-litigate `AT-34-E2-001`/`003`/`004` (all `complete` on `kanban.md`,
    each with its own receipt); `AT-34-E2-001` was reconfirmed immediately prior on this branch.
- **Next-cycle plan:** Epic 2 (`AT-34-E2-001..004`) is 4 of 4 complete; `001` and `002` are now
  both reconfirmed clean at HEAD with no drift. `003`/`004` remain reconfirmed only by their
  original 2026-08-27 receipts as of this cycle. Active bundle work is Epic 3 (Core Rulebook to
  zero) per `progress.md`'s latest entries.
