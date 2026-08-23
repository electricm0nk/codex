# Cycle epic-2-race-trait-generic-ingest — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** `epic-2-cause-closure` (row 11; scope: `race_trait` `no_record` closure)
- **Actor:** `t9-onboarding`
- **Commit SHA:** (this cycle's own, see push output)
- **Base:** `857eb85d0370adce3bd113c0cbda4e755b631a0a` (pinned base, `git merge-base --is-ancestor`
  confirmed before starting — worktree landed exactly on the pin, `origin/tranche/12` unchanged
  under it for the whole cycle).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`), bootstrapped fresh into this worktree's repo-local slot via
  `scripts/fetch-pcgen-oracle.sh` after `scripts/verify.sh --only preflight-oracle` reported the
  slot empty (fresh-worktree footgun, per §2.1).

## 1. The mission — `race_trait` `no_record`, 1,883 -> 0 (as close as `§15` allows)

Re-derived fresh at cycle start (not trusted from the dispatch brief):
```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l0.json
python3 -c "import json,collections; r=json.load(open('/tmp/l0.json'))['rows']; \
  print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```
-> `race_trait` 1,883 — matches the brief's own figure exactly, and matches the total bundle
`no_record` (8,434) reported at cycle start.

## 2. Existing-mechanism search first (`decisions.md §17`)

Grepped `scripts/` and `src/bin/` before writing anything. `race_trait` already has THREE Rust
ingesters (`ingest_race_traits.rs`, `ingest_apg_race_traits.rs`, `ingest_races.rs`) — all
semantically-resolved, per-race, per-book, engine-reachable-through-the-picker generators, the
right tool for real new race content (bestiary_5's chassis, Skinwalker, Changeling/Dhampir/Samsaran
— all still open, still real work, not touched this cycle). None of them is a generic pass. Prior
cycles this bundle (`card11-t2b-remeasure.md`, several T2b receipts) spent substantial effort on
book-level classifier fixes and per-book content proofs without closing the bulk of the population,
because most of what remains resists confident per-row reclassification without real domain risk
(verified live this cycle: `advanced_class_guide`'s `.MOD` Favored-Class-Bonus rows looked
reclassifiable at a glance but are the SAME `race_favored_class_bonus_row`-shaped content the
classifier already deliberately leaves as `race_trait`; `advanced_players_guide`'s "header" `Racial
SLA ~ X` rows, which `card11-t2b-remeasure.md §2b` characterized as "not work," turned out on direct
inspection to carry real `DEFINE:`/`SPELLS:`/`BONUS:VAR` formula content — a live re-confirmation of
`AGENTS.md` rule 7/`decisions.md §17a`: "re-check the finding that looks good").

**The actual lever: `scripts/ingest_ability.py`'s own wave-1 shape.** That script already proved,
for `Kind::Ability`'s identical 4,824-unit-all-`no_record` population, that a generic, book-agnostic,
kind-filtered VERBATIM transcription (no semantic modelling, no per-row judgment) satisfies Gate 1's
actual bar — "shape measured," not "player-reachable." This cycle ports the identical pattern to
`race_trait`: new `scripts/ingest_race_trait_generic.py`.

## 3. What was built — TDD'd

New `scripts/ingest_race_trait_generic.py` (mirrors `ingest_ability.py` field-for-field: `row_tokens`,
`declared_pi`, `slugify`, `resolve_file`, PI screening via the same imported
`sd32_t9_pi_review_feat_equipment.normalized_term_hit`). One deliberate difference from
`ingest_ability.py`: **filters on the live `shape_ledger.py` join (`--ledger <output.json>`), never
the possibly-stale `status` field** — `docs/work-inventory.json`'s `status` has drifted from the
live corpus join before (`epic-2-t2b-pure-ability-pointer-row-fix_cycle-1_cycle_receipt.md` finding
5/6), so a unit already carrying a real corpus record from one of the three Rust ingesters is never
re-touched even if its `status` field is stale.

New `scripts/tests/test_ingest_race_trait_generic.py`, 8 tests. **RED proved for the intended
reason**: the `load_units`-uses-live-join regression guard test was written first against the real
function, then the function was temporarily changed to filter on `status == "not-ingested"` instead
of the ledger's `no_record` ids —
```
python3 -m unittest scripts.tests.test_ingest_race_trait_generic -v
```
only `test_unit_excluded_when_absent_from_no_record_ids_even_if_status_stale` failed
(`AssertionError: Lists differ: ['book:race_trait:stale-but-ingested', 'book:race_trait:genuinely-open'] != ['book:race_trait:genuinely-open']`),
the other 7 stayed green. Reverted, all 8 green again.

## 4. A real mid-cycle correction — directory collision, caught before push

First draft wrote to `data/corpus/<book>/race_trait/*.json` (flat, mirroring `ingest_ability.py`'s
own `.../ability/*.json` convention exactly). `cargo test --locked --test v06_work_inventory` caught
it: `arg_race_file_carries_favored_class_bonus_and_choice_suboption_rows_not_traits` panicked
(`Os { code: 20, kind: NotADirectory }`) — that test walks
`data/corpus/advanced_race_guide/race_trait/` assuming every entry is a race-named
*subdirectory* of the Rust ingesters' richer `category`/`type_tokens` schema; a flat file at that
level broke `std::fs::read_dir`. **Fixed by writing to a SIBLING directory,
`data/corpus/<book>/race_trait_generic/`**, not `race_trait/` itself — `shape_ledger.py`'s join
walks `data/corpus/<book>/**/*.json` with no subdirectory-name filter, so this is exactly as
measurable for Gate 1 while touching zero existing consumers of the curated `race_trait/` shape.
All 1,878 previously-written flat files deleted (they were all untracked — `git status --porcelain`
diffed clean before and after) and re-written at the corrected location.
`cargo test --locked --test v06_work_inventory` → 16/16 after the fix (was 15/16 before).
`cargo test --locked --test duergar_invisibility_sla_reaches_a_player_via_monster_codex` → 7/7
(monster_codex's own `race_trait/` dir untouched by this cycle).

## 5. Measured effect

```
python3 scripts/ingest_race_trait_generic.py --ledger /tmp/l0.json --out <report.json>
```
`population 1883, written 1878, name_pi_skipped 5, unresolved 0`. Report committed as
`artifacts/gate-3-closure-invariant/11-race-trait-generic-ingest-report.json` (`decisions.md §15`
naming, mirrors `17-ability-pi-skipped.json`'s convention).

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l2.json
```
(`docs/work-inventory.json` itself unregenerated — its unit *citations* don't change, only which
ones now join to a real corpus record, which the ledger re-derives fresh against the corpus every
run):
- `race_trait` `no_record`: **1,883 -> 5** (the 5 PI-skipped units, confirmed by id match against
  the report's `pi_skipped_records`).
- Bundle-wide `no_record`: **8,434 -> 6,556** (-1,878), matching the write count exactly.

**Independent byte-for-byte re-verification** (substitutes for `corpus_literal_sweep`, blocked — see
§6): for all 1,878 new records, re-read the cited `(book, source_file, line)` fresh from the pinned
oracle, re-tokenized independently, diffed against the stored `raw_tokens` (PI-redacted `DESC`
fields normalized before comparing) — **0 mismatches / 1,878 checked**. Path-shape independently
re-checked against `corpus_literal_sweep.rs::book_dir_of`'s own rule (>=5 segments, or 4 with
`dreamscarred_press`) — **0 malformed / 1,878 checked**.

## 6. Why `docs/work-inventory.json` is NOT regenerated, and `corpus_literal_sweep` not run clean

`corpus_literal_sweep` (required, unmodified, for a stamp-safe guarded regen) exits 2 corpus-wide:
```
corpus-literal-sweep: data/corpus/advanced_class_guide/domain/battle_spirit.json: source.path
  paizo/roleplaying_game/advanced_class_guide/acg_domains.lst is not
  <system>/<publisher>/<line>/<book>/<file>-shaped
```
Re-derived corpus-wide this cycle: **2,924 pre-existing records** (not this cycle's — every one of
my own 1,878 confirmed correctly-shaped, §5) still carry the `source.path` defect the dispatch
brief's own "near-miss" note flagged as "a repair lane is running now" — that repair is **not yet
complete**. This blocks every lane's guarded regen right now, `race_trait`'s included, out of this
cycle's write scope (`domain` and other kinds belong to different lanes). Not fixed here; named
plainly rather than assumed fixed. `docs/work-inventory.json` is therefore not regenerated or
committed this cycle — same precedent `epic-2-t2b-pure-ability-pointer-row-fix_cycle-1_cycle_receipt.md`
already established for the identical blocker. `NO_RECORD_BUDGET_COUNT`/`POPULATION` constants:
untouched, per the brief's explicit instruction.

## 7. Identifier / wired-integration dual audit (scoped to this cycle's own diff)

```
git diff --cached --unified=0 -- scripts/ingest_race_trait_generic.py scripts/tests/test_ingest_race_trait_generic.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
```
One hit: `from sd32_t9_pi_review_feat_equipment import (...)` — the already-existing module
`ingest_ability.py` itself imports, its own real filename (predates this cycle, `decisions.md
§18/§19` chain), not a new bundle-tag this cycle invented. Identical precedent already recorded in
`epic-2-ability-ingest_cycle-1_cycle_receipt.md`'s own identifier-audit section.
```
git diff --cached --unified=0 -- scripts/ingest_race_trait_generic.py scripts/tests/test_ingest_race_trait_generic.py \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```
-> `OK_NO_TOKENS`.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (one pre-existing-filename import, named above,
  not a new leak)
- **Wired-integration audit result:** `OK_NO_TOKENS`

## 8. Suites run this cycle, fresh

```
python3 -m unittest scripts.tests.test_ingest_race_trait_generic -v      -> 8 passed
cargo test --locked --test v06_work_inventory                            -> 16 passed, 0 failed, 1 ignored
cargo test --locked --test duergar_invisibility_sla_reaches_a_player_via_monster_codex
                                                                            -> 7 passed, 0 failed
cargo run --locked --bin derived_evaluator_fixture_check                 -> 1836 cleared, 0 failed, 0 not ingested
```
Full unscoped `cargo test --locked --lib`/`--tests` not run this cycle (no `.rs` file touched by
this cycle — only new Python scripts and new/appended corpus JSON — the targeted suites above are
the ones a `race_trait` corpus-shape change could plausibly break; named honestly, not claimed).

## 9. Pinned-count sweep

```
grep -rn "1883\|1,883" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v /target/ | grep -v /artifacts/corpus/
```
0 hits besides this cycle's own doc comment (a prose figure, not an assertion) and one unrelated
`Cargo.lock` checksum substring collision. No pinned assertion anywhere depends on the pre-cycle
`race_trait` `no_record` count.

## 10. Product Identity (`decisions.md §15`)

5 units skipped, never transcribed, named individually by coordinate in the committed report
(`11-race-trait-generic-ingest-report.json`'s `pi_skipped_records`): `inner_sea_gods:isg_abilities_races.lst:12`
(term hit), `inner_sea_races:isr_abilities_race.lst:67` (`NAMEISPI:YES`), and three rows in
`inner_sea_world_guide:pfs_iswg_abilities_race.lst` (lines 34-36, term hit).

**Superseded 2026-08-23 (`decisions.md §24`, SD-32 T9-onboarding-cause-closure cycle):** the "a name
cannot be redacted" premise this section was written under is superseded — `§24` ingests exactly this
shape under a Codex-generated neutral name. All 5 are now ingested (`data/corpus/<book>/
race_trait_generic/codex_named_unit_*.json`, `codex_generated_name: true`); they are no longer
permanently `no_record`. This receipt's own committed report (`11-race-trait-generic-ingest-report.json`)
and this paragraph had each quoted the units' literal display names in plain text — a `§24b`-4 exposure
now fixed by reducing both to coordinate-only form, since the underlying records ship under `§24`.

## 11. Reachability, claimed honestly (`decisions.md §20`'s own "lessons wave 1 paid for" item 4)

`reach_gate.rs` is not touched this cycle and defines no entry for `race_trait_generic`-directory
records. **Reachability claimed: 0.** These 1,878 records make their shapes measurable
(`shape_ledger.py`'s `join_status`/`family` fields now populate for them); none of them is wired
into `ingest_race_traits.rs`/`race_resolver.rs`'s player-facing picker path. Gate-1 measurability and
player-reachability are different claims, and only the first is made here.

## 12. What this cycle did NOT do (explicit)

- Did not fix the pre-existing `domain`-kind (and other kinds') `source.path` defect blocking
  `corpus_literal_sweep` corpus-wide — out of `race_trait` scope, a different lane's ingest script.
- Did not regenerate/commit `docs/work-inventory.json` — blocked by the above, same as the prior
  T2b cycle's own documented choice.
- Did not attempt the bestiary_5 chassis/Skinwalker/Changeling-Dhampir-Samsaran/Adoptive-Parentage
  work named in `card11-t2b-remeasure.md §7` — that is real, semantically-resolved, engine-reachable
  new content, a different shape of work from this cycle's generic measurability pass, and several
  of those clusters are blocked on an operator ruling this cycle has no authority to grant.
- Did not touch `NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION`.
- `kanban.md` row 11 left `in-progress`, per the brief; row 15 untouched.

## Next-cycle plan

1. Whichever lane owns the `domain`/other-kind `source.path` repair: finish it corpus-wide, then a
   guarded regen of `docs/work-inventory.json` becomes possible for every blocked lane at once
   (this cycle's `race_trait` fix included).
2. The 5 PI-blocked `race_trait` units stay `no_record` permanently by design — not future work.
3. The real remaining `race_trait` work (bestiary_5 chassis + Skinwalker heritage-selector, 133
   units; Changeling/Dhampir/Samsaran chassis; the 35-unit Adoptive-Parentage/`kind: trait`
   escalation) is unaffected by this cycle and still needs the operator rulings `card11-t2b-remeasure.md
   §7` already named.
4. If a future cycle wants these `race_trait_generic/` records player-reachable, that is new,
   separate, semantically-real work in `ingest_race_traits.rs`/`race_resolver.rs` — not a relabelling
   of this cycle's measurability pass.

## Disk usage

```
df -h /
```
-> `/dev/sda1  968G  308G  661G  32% /` — no pressure, no cleanup needed.
