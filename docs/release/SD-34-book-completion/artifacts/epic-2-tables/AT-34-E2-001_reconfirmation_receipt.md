# Cycle 9-R — Epic 2 (Build eight of the nine tables) / AT-34-E2-001 (reconfirmation at HEAD)

- **Commit SHA:** `e403495d29e87664131693a0cefce320cab57eaa` (HEAD at cycle start — no code
  change was needed, so this cycle's own commit carries only this receipt + `progress.md` +
  `kanban.md`)
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_reconfirmation_receipt.md`
  (new, this file), `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`,
  `docs/retro/events/sd34-at-34-e2-001.jsonl` (new)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` for this cycle's own diff. See "Dual audit" below for the epic-wide file-touch-set scope.
- **Wired-integration audit result:** `OK_NO_TOKENS` for this cycle's own diff. See "Dual audit" below for the epic-wide file-touch-set scope.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "For each kind, either an engine
  table exists and holds records, or a proof by execution that the kind needs none — e.g. every
  unit of that kind is `display`-class and its terminal state is a rendered description.
  **Evidence:** per kind, either the table's location and a transcript of it holding a named
  record, or the counts showing no magnitude is involved. 'No table needed' is a finding that
  must be proven, never assumed to save work."

## Why this cycle exists

`AT-34-E2-001` was already `complete` on `kanban.md` row 9, built at commit `052a9182bf` and
committed 2026-08-26/27 (`artifacts/epic-2-tables/AT-34-E2-001_cycle_receipt.md`). This bundle's
own standing lesson (`workflow-instruction.md §12` row 19 / `decisions.md §12` L2 — "never carry
your own number forward, re-derive it") applies to a dispatched lane picking the criterion back
up, not only to a first cycle: Epic 3 and Epic 1 (`AT-34-E1-008`) have since landed commits that
touch `src/rules_core/` and `data/corpus/**` (a corpus-wide `wiring_class` restamp, among other
things). This cycle re-derives the criterion's own evidence at HEAD rather than re-quoting the
2026-08-26 receipt, and reports whether anything moved.

## Re-derivation at HEAD

```
$ cargo test --locked --lib rules_core::rules_tables::simple_kind_tables
running 13 tests
test rules_core::rules_tables::simple_kind_tables::tests::an_unknown_kind_yields_an_empty_table_not_a_panic ... ok
test rules_core::rules_tables::simple_kind_tables::tests::trait_kind_resolves_to_the_trait_generic_directory ... ok
test rules_core::rules_tables::simple_kind_tables::tests::skill_table_holds_craft_rope ... ok
test rules_core::rules_tables::simple_kind_tables::tests::language_table_holds_xenophobic ... ok
test rules_core::rules_tables::simple_kind_tables::tests::domain_table_holds_battle_spirit ... ok
test rules_core::rules_tables::simple_kind_tables::tests::domain_table_resolves_a_pi_renamed_record_by_coordinate_not_by_the_real_name ... ok
test rules_core::rules_tables::simple_kind_tables::tests::deity_table_holds_a_pi_masked_codex_named_record ... ok
test rules_core::rules_tables::simple_kind_tables::tests::trait_table_holds_trait_adopted ... ok
test rules_core::rules_tables::simple_kind_tables::tests::race_trait_generic_table_resolves_the_sibling_directory_not_race_trait_itself ... ok
test rules_core::rules_tables::simple_kind_tables::tests::template_table_holds_arcanist_spellbook ... ok
test rules_core::rules_tables::simple_kind_tables::tests::an_absent_key_is_refused_not_fabricated ... ok
test rules_core::rules_tables::simple_kind_tables::tests::ability_table_holds_aberrant_bloodline ... ok
test rules_core::rules_tables::simple_kind_tables::tests::every_seven_kind_table_is_non_empty_at_head ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 2911 filtered out
```

13 of 13 pass at HEAD (2 more than the 11 that existed at `052a9182bf` — `AT-34-E2-002`'s later
cycle added `domain_table_resolves_a_pi_renamed_record_by_coordinate_not_by_the_real_name` and
`race_trait_generic_table_resolves_the_sibling_directory_not_race_trait_itself`, both still
green). No test that existed at the original commit regressed.

```
$ cargo build --locked --bin v06_work_inventory
$ ./target/debug/v06_work_inventory --epic2-table-transcript > /tmp/e2001_transcript_check.txt
$ grep -oE "kind=[a-z]+" /tmp/e2001_transcript_check.txt | sort -u
kind=ability
kind=companion
kind=deity
kind=domain
kind=language
kind=skill
kind=template
kind=trait
$ grep "HELD" /tmp/e2001_transcript_check.txt | grep -oE "kind=[a-z]+" | sort -u | wc -l
8
$ diff /tmp/e2001_transcript_check.txt docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt
(no output)
```

**The freshly-run transcript at HEAD is byte-identical to the committed one.** All 8 of the 8
kinds Epic 2 builds (`ability`, `template`, `trait`, `deity`, `domain`, `skill`, `language`,
`companion`) still report `HELD` on a named record and `REFUSED` on a fabricated key — no drift
from the corpus-wide `wiring_class` restamp (`AT-34-E1-008`) or from Epic 3's later work, because
none of that work touches the `ability`/`template`/`trait`/`deity`/`domain`/`skill`/`language`
corpus directories' record *identity* (only `wiring_class`/`wiring_class_signals` fields, which
`simple_kind_tables::resolve` does not read).

## Figures + their re-derive commands (every one, with its denominator)

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| Kinds holding a named record | `8` | `grep "HELD" .../AT-34-E2-001_table_transcript.txt \| grep -oE "kind=[a-z]+" \| sort -u \| wc -l` (re-run at HEAD, output identical to the committed transcript) | of the 8 kinds Epic 2 builds (`technical-design.md §4`) |
| `simple_kind_tables` unit tests passing | `13` | `cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` | of `13` tests in that module at HEAD (was `11` at the original commit; `+2` from `AT-34-E2-002`) |
| Denominator gate on this package | `violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of `15` files checked |
| `corpus_literal_sweep` | `48,708 of 51,482` examined, `0` findings, `CLEAN` | `cargo run --locked --bin corpus_literal_sweep` | of `51,482` records read (moved from the package's stated baseline `48,699 of 51,473` by `+9`/`+9` — later cycles' corpus additions, not this cycle's; this cycle wrote no corpus record, `git status --porcelain -- data/corpus` is empty throughout) |

## Dual audit — Epic 2's own §3 file-touch set

`BASE_BRANCH=ea2b3396f2fde9223dde93522bd2288b463a21ee`, over
`src/bin/v06_work_inventory.rs src/rules_core/ artifacts/epic-2-tables/`, excluding
`__tests__` and `*.test.*`:

```
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS

$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
```

The second grep returns matches, all inside `+` lines added by **Epic 3's** later commits under
`src/rules_core/` (the shared file-touch scope with Epic 2, per `workflow-instruction.md §3`),
none of which are Epic 2's own deliverable. Every hit is the real, correctly-spelled domain term
`placeholder row` — PCGen's own CHOOSE-menu "no selection" rows in the corpus data model
(`AT-34-E3-001`'s vacuous-placeholder sub-cause, e.g. `src/bin/v06_work_inventory.rs:4691`:
`"PCGen's own CHOOSE-menu \"no selection\" placeholder row for the Barbarian class; no DESC, no
mechanical token; not a Pathfinder rules feature."`) — a described real-world corpus artifact, not
a stub, mock, or unfinished marker in shipping logic. This is a **single-token audit
false-positive class**, self-healable by review per `workflow-instruction.md §8`: reviewed here
and confirmed non-stub; `simple_kind_tables.rs` (this criterion's own deliverable) itself carries
zero hits of either pattern.

## Row-count command output

```
$ grep "HELD" docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt | grep -oE "kind=[a-z]+" | sort -u | wc -l
8
```
8 of 8 kinds still report `HELD` on a named record at HEAD, with the transcript re-derived fresh
and diffed byte-identical against the committed one — not re-quoted from the original receipt.

## Build scope verified

- `cargo test --locked --no-run -j2` (full root workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-001`) — see command output in this cycle's log; run **after** the last check in this cycle that could move a figure (the transcript regeneration and sweep both completed first, and `git status --porcelain -- data/corpus` was empty throughout).
- `apps/desktop/src-tauri` — **not touched this cycle** (`git diff --name-only <base>..HEAD -- apps/desktop/src-tauri` empty for this cycle's own commit); not run. This is a reconfirmation cycle with no source change, so the desktop crate's status is unaffected by it.
- Run at SHA: this cycle rebased cleanly onto `e403495d29` with no upstream movement (`git fetch origin tranche/14` returned the same SHA already at `HEAD`), so the build-scope commands above ran at `e403495d29e87664131693a0cefce320cab57eaa`, the same commit this receipt's own commit is built on top of.

## Sweep population

`corpus_literal_sweep`: `48,708 of 51,482` examined, `0` findings, `CLEAN`. This cycle added or
regenerated **zero** corpus records (`git status --porcelain -- data/corpus` empty for the whole
cycle), so the expected delta from this cycle alone is `0` — satisfied. The population itself has
moved `+9`/`+9` versus the package's stated launch baseline (`48,699 of 51,473`) from other
cycles' work between launch and HEAD; that movement is not attributable to this cycle and is not
re-litigated here.

## Oracle pin

N/A — no figure in this cycle is drawn from the pinned PCGen oracle corpus.

- **Status:** complete
- **Movement, four buckets:**
  - **closure** — none (the criterion was already `complete`; this cycle reconfirms, it does not
    close anything new).
  - **reclassification** — none. `docs/work-inventory.json` untouched
    (`git status --porcelain -- docs/work-inventory.json` empty).
  - **reachability** — none. No table's resolution logic changed.
  - **instrument-correction** — none. No drift was found between the original commit's evidence
    and HEAD's; this is a confirmed-clean re-derivation, not a fix.
- **Notes (judgment calls):**
  - This cycle intentionally does not re-litigate `AT-34-E2-002`/`003`/`004` (all already
    `complete` on `kanban.md`, each with its own receipt in this directory) — those are separate
    criteria with their own rows.
  - The two extra passing tests (`13` vs the original `11`) belong to `AT-34-E2-002`'s cycle, not
    this one; they are reported here only because re-deriving "how many tests exist and pass in
    this module" at HEAD is this criterion's own honest denominator, not because this cycle wrote
    them.
- **Next-cycle plan:** Epic 2 (`AT-34-E2-001..004`) is 4 of 4 complete and reconfirmed clean at
  HEAD. Epic 3 (Core Rulebook to zero) is the bundle's active work, per `progress.md`'s latest
  entries; no further action is needed on Epic 2 from this cycle.
