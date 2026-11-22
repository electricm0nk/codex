# SD-32 row-15 integration + audit follow-ups — cycle receipt

**Cycle:** `row15-integration-and-audit-followups` (dispatched to close the three items the
closure-readiness audit confirmed open and unowned: row 15's own stated integration cycle,
the `apps/desktop/src-tauri equipment_catalog::` red, and `forward-scope-register.md` C2.5;
plus finish the `declared_pi_shipping_audit` verification the audit's own turn could not).
**Base/HEAD:** pinned base `901f1e083ea61734a0b3c7bf8262ddebf00e5a59`; first rebase onto
`origin/tranche/12` was a no-op (already at tip); worked from there, committed locally
(`ddb1fc8ff5`), then re-fetched at push time and found `origin/tranche/12` had moved to
`f76242cc69` — rebase onto that tip **conflicted** in `apps/desktop/src-tauri/src/
equipment_catalog.rs` (see "Collision" below). Final pushed HEAD is on top of `f76242cc69`.
**Territory respected:** no writes under `data/corpus/**/monster_ability/**`,
`scripts/transcribe_monster_tables.py`, `monster_chassis.rs`, `class_feature_pool_catalog.rs`.
Confirmed via `git status --porcelain` throughout.

**Collision, honestly reported (not the "no collision" this receipt originally claimed pre-rebase
— corrected here rather than silently left stale):** the sibling `unowned-reds` lane's commit
`26012fb4b4` (landed on `origin/tranche/12` while this cycle was mid-flight) **independently found
and fixed the exact same 3 `equipment_catalog::` reds**, reading the exact same 9 stale pins the
same way and landing **byte-for-byte identical assertion VALUES** (only the explanatory comment
prose differs — confirmed by diffing the two versions directly: every `assert_eq!` target number
matches). On rebase, this cycle's redundant `equipment_catalog.rs` diff was dropped in favour of
the already-upstream sibling fix (`git checkout` the upstream side, not this cycle's own) — the
independent agreement on every number is itself a form of corroboration this bundle values (two
different derivations reaching the same ground truth), so nothing is lost by deferring to
whichever commit landed first. `26012fb4b4`'s own commit message also opened kanban row 19 for 15
OTHER `apps/desktop` reds (`reach_gate`/`class_feature_feat_bridge`/`character_hub`/
`companion_catalog`/`corpus_ingest_diagnostic`/`intelligent_item_catalog`/`spell_catalog`) — not
touched by this cycle, no overlap there.

## 1. Row 15 (`census-scope-closure`) — the integration cycle, run

Per the audit's own finding (`closure-readiness-audit_cycle-1_cycle_receipt.md` §3): the content
numbers were already zero-gap; only row 15's own stated "integration cycle next" bookkeeping step
had not run. Re-derived fresh, not requoted from the audit (§17a):

| Gate | Command | Live result, this cycle |
|---|---|---|
| Gate 0 | `python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" --inventory docs/work-inventory.json` | `discovered=186 in_scope=38 excluded=148 unexplained=0` |
| Gate 1 | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` | population `34397`, `unclassified_count=0`, `join_status_counts={'matched': 11578, 'no_formula_tokens': 22819}` — **no `no_record` key at all: 0** |
| Gate 3 | `scripts/verify.sh --only shape-coverage-standing-gate` | `PASS (population=34397 unclassified=0 no_record=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)` |

**This is better than the audit's own figure** (`no_record=56`, all `monster_ability`): the sibling
T9 lane's `be100ceea6` ("56->0 no_record, T9 round 9") landed between the audit's commit
(`91b24b9460`) and this cycle's base, closing the last residual the audit had named as sibling
territory. Row 15's own scope test (`decisions.md §12b`: every kind-unenumerable object enumerated,
classified, and Gate-3-covered, or proven not-an-object) is satisfied with a stronger number than
even the audit reported. Set `kanban.md` row 15 to `complete`, prepended a consolidation paragraph
citing the live re-derivation above (not the lanes' own prose), and corrected the row's `Cycle` note.
Row's status/cycle-note cells only — the rest of the 6.2KB row body is untouched; parses correctly
(verified with a small Python regex extraction of the row's first 6 fields, not by eyeballing the
32KB line).

## 2. `apps/desktop/src-tauri equipment_catalog::` — 3 reds, reproduced and fixed

Reproduced exactly as the audit found: `cargo test --locked --manifest-path
apps/desktop/src-tauri/Cargo.toml equipment_catalog::` → **14/17 pass, 3 FAILED**
(`catalog_spans_every_ingested_book_with_their_real_counts` 72≠71,
`filter_equipment_catalog_matches_category_exactly_across_every_book` 1095≠1077,
`description_coverage_is_pinned_per_book` 97≠72).

**Root cause, isolated exactly** (matching this bundle's own precedent method): `ea2a72dd64`
("PI-name-blocked equipment/spell close via §24 neutral-name ingest") and `f0b6a8b96f` regenerated
`src/rules_core/rules_tables/equipment_gap_tables.rs` — adding 74 `name_pi_citation: Some(..)` rows
(38 General / 18 ArmsArmor / 18 MagicItems) across 10 books (ISG 25, AG 18, ISI 8, ISWG 7, ISC 7, B4
3, MYTHIC 3, UE 1, ISR 1, BOTD2 1) — without sweeping `apps/desktop/src-tauri`, a **separate cargo
workspace** a root sweep misses. Verified via a temporary diagnostic test (`zzz_scratch_dump_
diagnostics`, added, run, then removed before commit — not part of the final diff) that dumped the
real `build_equipment_catalog()` output per book/category, confirming every failing and previously-
untested-but-also-stale assertion:

| Pin | Old | New | Confirmed by |
|---|---:|---:|---|
| `count_by_book("ISR")` | 71 | 72 | `find data/corpus/inner_sea_races/equipment -name '*.json' \| wc -l` = 72 |
| `count_by_book("ISWG")` | 46 | 53 | ground truth |
| `count_by_book("B4")` | 5 | 8 | ground truth |
| `count_by_book("ISG")` | 125 | 150 | ground truth |
| `count_by_book("MYTHIC")` | 252 | 255 | ground truth |
| `count_by_book("ISC")` | 65 | 72 | ground truth |
| `count_by_book("ISI")` | 34 | 42 | ground truth |
| `count_by_book("BOTD2")` | 5 | 6 | ground truth |
| `count_by_book("AG")` | 97 | 116 | `find data/corpus/adventurers_guide/equipment -name '*.json' \| wc -l` = 116 |
| total `entries.len()` | 8025 | 8100 | sum of all 28 `count_by_book` pins == 8100, independently equal to the live `.len()` |
| `with_description("ISG")` | 72 | 97 | 25 codex-named ISG rows, all carrying real description (name-only PI) |
| `with_description("ISI")` | 9 | 12 | ground truth |
| `with_description("BOTD2")` | 3 | 4 | ground truth |
| `with_description("AG")` | 14 | 18 | ground truth |
| total `with_description` | 4719 | 4756 | sum of all 28 `with_description` pins == 4756 |
| ArmsArmor filter total | 1077 | 1095 | exactly the 18 ArmsArmor-category rows among the 74 new `name_pi_citation: Some` rows |

**Every book NOT listed above (CRB/APG/ACG/B1/ARG/PU/UI/UE/UM/UPSI/UC/UW/OA/HA/MC/B2/B3/ISTEM/ISM)
was independently confirmed unchanged** against the same ground-truth dump — not assumed from a
first-panic-then-stop test run (Rust's `assert_eq!` aborts the whole test function at its first
failure, so every assertion *after* the first failing one in each of the 3 failing tests had never
actually been re-executed; the audit's "left/right" figures for those 3 lines were real, but nothing
downstream of them was verified until this cycle's diagnostic dump).

**Mutation-proved** (`§1a`): reverted `count_by_book("ISR")` to the stale `71`, reran just that test
— RED for the exact right reason (`left: 72, right: 71`, matching the live value exactly) — then
restored `72`, reran the full 17-test suite: **17/17 GREEN**.

**Data confirmed genuinely grown, not corrupted**: this is the ninth-through-however-many-more stale
pin in this bundle, fixed the bundle's standard way — retargeting to a proven live truth, never
loosening an assertion.

**Post-hoc note:** at push time, `origin/tranche/12` had moved and the sibling `unowned-reds`
lane's `26012fb4b4` turned out to have landed this exact fix independently, first. See "Collision"
in the header above — this cycle's own redundant `equipment_catalog.rs` diff was dropped in favour
of the already-upstream commit on rebase; every target number the two derivations reached agrees
exactly, which is itself corroborating evidence the fix is right.

## 3. `forward-scope-register.md` C2.5 — corrected

C2.5 described the first SD-32 dispatch run's (2026-08-22) `returned-to-backlog` disposition of
T2a/T2b/T9/T4/T12/T7/T8, filed as future-SD-N work. `decisions.md §10`/`§13` overturned that the
same day; the work landed inside SD-32 itself (T1/T2a/T2b/T4/T7/T8 via `epic-2-cause-closure/4` +
`§20`'s generic-ingest campaign; T8 via `§11`'s granted write scope; T9's `monster_ability` residual
to zero via `be100ceea6`, re-confirmed live this cycle in §1 above; T12 via the cycle chain
`452c70d035`/`2382bed37b`/`a0ee0db4f4`/`d10da0a7ea`/`cd60d08042`). The entry was never updated.
Replaced with a `RESOLVED, removed` row (matching the pattern `progress.md`'s own "## Open blockers"
section already uses) citing every resolving commit, and naming the one genuinely new population the
`cd60d08042` cycle surfaced along the way (`class_feature_pool_catalog.rs`'s pool population) as now
owned by `kanban.md` row 18, not re-added here — a closed shape does not get a forward-scope row, per
this register's own "no unowned tidiness entries" convention.

## 4. `declared_pi_shipping_audit` — run to completion

**First attempt used the wrong oracle path** (`pcgen_data_root()` defaults to the FORBIDDEN
`~/workspace/repos/pcgen/data` when `PCGEN_CORPUS_ROOT` is unset in the process environment — my own
error, caught before the run finished by inspecting `/proc/<pid>/environ`) — killed (`kill -9`)
before it produced any output, no result trusted or reported from that run.

**Second run, correct pinned oracle** (`PCGEN_CORPUS_ROOT` = the repo-local
`artifacts/corpus/operator-supplied/pcgen/data` slot, corpus SHA
`7f818006e371188e5717fd18d74d18a420747fc6`), run to completion in the background per the dispatch's
own instruction (foreground polling, not "resume when it reports"):

```
cargo run --locked --bin declared_pi_shipping_audit
declared-pi-audit: FAIL — 65 violation(s) across 65 file(s)
```

**The real number, verified (not corroborated): 65.** All 65 are `DESC-PI-SHIPPED` in
`data/corpus/bestiary_4/monster_ability/**` (e.g.
`kaiju_bezravnis_web.json: cites .../b4_abilities_race.lst:971 (DESCISPI:YES) but
data.description=Some("[redacted PI]") license=Some("OGL") pi_field=None (expected
description="[redacted PI]", license="PI-REDACTED", pi_field="description")`). This exactly
reproduces `decisions.md §26`'s own prior finding of the same 65, same book, same shape,
confirming it is a **stable, already-known metadata-labeling gap**, not a new or growing PI
exposure: every one of the 65 records' `description` field already carries the real redaction
marker (`"[redacted PI]"`) — **no actual Product Identity text ships in any of the 65** — the
defect is that `license`/`pi_field` were never stamped to reflect the redaction that already
happened, so `declared_pi_shipping_audit`'s cross-check against the record's own PCGen
`DESCISPI:YES` declaration correctly flags the metadata mismatch.

**This corroborates, and upgrades, the audit's own evidence chain**: the closure-readiness
audit could only cite (a) `sd32_t9_corpus_wide_pi_rescan.py`'s 0 field-hits over 51,360 records
and (b) the generator static audit's structural PASS. This cycle adds (c) the actual verified
binary result the audit's own turn ran out of budget for — **65, not "unknown, presumed low"** —
closing the "corroboration is not verification" gap the dispatch named. `data/corpus/**/
monster_ability/**` is explicitly sibling T9-lane territory this dispatch is scoped away from
(fixing the `license`/`pi_field` stamp on these 65 records is a metadata-write inside that
territory) — reported by name and count per `decisions.md §15`'s standing rule (a cycle that
finds a suspected-PI-shaped defect stops and reports it, never silently transcribes or skips),
not fixed here.

**First-attempt error, corrected before any result was trusted**: the first invocation omitted
`PCGEN_CORPUS_ROOT`, which defaults to the FORBIDDEN `~/workspace/repos/pcgen/data` — caught via
`/proc/<pid>/environ` before the run produced any output, killed with `kill -9`, and re-run with
the correct repo-local pinned oracle. No figure from the first attempt is reported or used.
