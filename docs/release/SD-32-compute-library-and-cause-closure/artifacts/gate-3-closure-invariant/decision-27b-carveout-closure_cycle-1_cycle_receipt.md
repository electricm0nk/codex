# Cycle decision-27b-carveout-closure — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `decision-27b-carveout-closure`)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — `BOOKS` dict gains `occult_adventures`
    (races_lsts auto-detected from `docs/work-inventory.json`'s own
    `kind==monster`/`kind==monster_ability` units, per this script's generic
    unit-set mechanism — no per-book glob).
  - `src/bin/gen_book_cache.rs` — new `MonsterBookSpec` entry for
    `occult_adventures` (`races_lsts: ["oa_races_b3.lst"]`,
    `abilities_lsts: ["oa_abilities_race.lst", "oa_abilities_race_b3.lst"]`);
    stale comment near `horror_adventures`'s spec corrected to note the
    overturned disposition.
  - `src/rules_core/rules_tables/occult_adventures/mod.rs` — wires
    `monster_data` module, `monsters_static()`/`monster_abilities_static()`.
  - `src/rules_core/rules_tables/occult_adventures/monster_data.rs` (new,
    generated) — 1 `MonsterStatBlock` (Kami (Shikigami)) + 5
    `MonsterAbilityRecord`s, all `owners: &[]`.
  - `src/rules_core/rules_tables/monster_chassis.rs` — `MONSTER_BOOKS` gains
    the `occult_adventures` row; `widening_the_facet_vocabulary_does_not_
    reclassify_any_existing_record` re-derived from its own live failing run
    (never guessed): 3706 → 3711 records, digest `0x38f4aedd6de1caf3` →
    `0xc4c144e1483d297d`.
  - `data/corpus/occult_adventures/monster/kami_shikigami.json`,
    `data/corpus/occult_adventures/monster_ability/{homunculus_companion_
    poison,homunculus_companion_sympathetic_alchemy,homunculus_companion_
    telepathic_link,shikigami_improvised_weapon_mastery,shikigami_spell_
    like_abilities}.json` (new, via `cargo run --bin gen_book_cache --
    occult_adventures`) and `data/corpus/occult_adventures/LICENSE.json`
    (screening-note append, records_processed 1242 → 2876, same
    generator-owned mechanism every prior monster-lane round used).
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — new `BOOK_OA` wire
    code, `occult_adventures` added to `book_display_name`/`book_wire_code`'s
    exhaustive matches (both panic on an unregistered book); owner-less
    pin `bonus_bestiary_ability_keys_carry_the_namespace` re-derived from its
    own live failure: 1048 → 1053.
  - `apps/desktop/src-tauri/src/reach_gate.rs` — new
    `("occult_adventures", "monsters")`/`("occult_adventures",
    "monster_abilities")` reach arms; `UNREACHED_RECORD_FINDINGS` and the
    exact-key owner-less pin list both gain an `occult_adventures` entry (5
    keys, matching `monster_data.rs`'s own header).
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` —
    `occult_adventures_counts()` now chains `chassis_book_counts` (same
    pattern `mythic_adventures_counts` already uses) so the panel reports
    the book's second compiled family.
  - `scripts/sd32_t9_pi_review_companion_monsterability.py` — "Shaitan"/
    "shaitan" and "burrowing"/"fish"/"solid" moved from documented
    deliberate holdouts into the allowlists, citing the operator spot-check
    in `t9-pi-review-companion-monsterability.md` §7 that already ruled
    `advanced_race_guide:Earth Glide (Shaitan Binder Eidolon)` **clear**
    ("Shaitan" is the genie-subtype term from the core Bestiary's elemental
    taxonomy, not a Golarion-specific name) and `scripts/pi_scrub.py`'s own
    canonical blacklist scan (independently re-run, 0 hits on both records'
    full text).
  - `data/corpus/advanced_race_guide/companion/{earth_glide,noble_eidolon}.json`
    (new, via `python3 scripts/ingest_companion.py`, the pre-existing
    generic no-record closer — no new ingest logic written).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff, content only
  — the one raw match was the pre-existing filename
  `sd32_t9_pi_review_companion_monsterability.py` in a diff header line, not
  introduced content).
- **Wired-integration audit result:** `OK_NO_TOKENS` (scoped diff, 0 hits).
- **Acceptance criterion:** `decisions.md §27b` — EVERYTHING: overturn the
  `occult_adventures` 5-unit `monster_ability` exclusion (reachability
  finding, not an ingest exemption) and the `companion` 2-unit exclusion
  (adjudicated reachability exclusion, not a pending PI ruling); ingest both,
  report reachability separately (`§16`).
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (both overturned exclusions closed; card 11 stays
  `in-progress` — 150 `no_record` units remain outside this cycle's two
  named targets).
- **Notes:** see full body below.
- **Discovery forwards:** none new — the wider carve-out sweep (§3 of the
  brief) found nothing else silently excluded; the remaining `out of
  scope`/`excluded`/`deferred`/`parked` hits in `decisions.md`/`progress.md`/
  `kanban.md` are either (a) already-escalated `## Open blockers`-style
  entries awaiting an operator ruling (T9's 2,712-unit reachability register:
  21 PI-excluded, 6 structurally-correct `.MOD`/`.COPY` exclusions), which is
  disposition 2 of Blocker Discipline, not a silent carve-out, or (b) another
  lane's explicitly-named, disclosed scope boundary (the class_feature lane's
  39-of-64 `TYPE:*Choice` collision groups held for hand review; the
  monster_ability lane's 17 pre-existing `unscreenable`/`unmodelled_facet`
  residual in `bestiary`) — named in the open receipts, not hidden, and
  outside this cycle's granted territory.
- **Next-cycle plan:** see §7 below.

---

## 1. Re-derived the two overturned exclusions before touching anything (`§17a`)

Never trusted the brief's own figures without re-deriving. `python3
scripts/shape_ledger.py --inventory docs/work-inventory.json` confirmed the
honest pre-cycle total: **157 `no_record`**, broken out per-kind exactly as
the brief stated: `monster_ability` 98, `class_feature` 25,
`equipment_modifier` 19, `equipment` 10, `companion` 2, `spell` 2, `ability`
1.

`occult_adventures`'s 5 units: `python3 scripts/classify_monster_ability_
rows.py occult_adventures` → `occult_adventures 1 5 0 0 5 0 0` (1 monster
row, 5 ability rows, all 5 orphan). Cross-referenced against
`docs/work-inventory.json`'s own `(book, kind, status)` rows: exactly 5
`monster_ability` `not-ingested` units — 3 in `oa_abilities_race.lst`
(`Homunculus Companion ~ Sympathetic Alchemy/Telepathic Link/Poison`), 2 in
`support/oa_abilities_race_b3.lst` (`Shikigami ~ Improvised Weapon
Mastery/Spell-Like Abilities`). Read the pinned oracle's own
`_occult_adventures.pcc` directly: line 74-75 gate
`oa_abilities_race_b3.lst`/`oa_races_b3.lst` on
`!PRECAMPAIGN:1,INCLUDES=Bestiary 3` — a **negated** gate this repo's own
registered `bestiary_3` book fails, meaning PCGen's own chargen would not
load this file for a campaign that includes Bestiary 3. That is a
**reachability** fact about this repo's modelled campaign set, not a
statement that the row does not exist in the book — exactly `decisions.md
§27b`'s reasoning, confirmed against the primary source rather than assumed
from the brief.

`companion`'s 2 units: `python3 scripts/shape_ledger.py --output
/tmp/shape_ledger_out.json` then filtered `join_status=="no_record" and
kind=="companion"` → `advanced_race_guide:companion:shaitan_binder_eidolon_
earth_glide` and `...noble_eidolon`, both `arg_abilities_companion.lst`
(lines 31/30). Six sibling "Shaitan Binder Eidolon" companion units (the
stat bonuses, `Charisma Bonus` etc.) already have real corpus records
(`data/corpus/advanced_race_guide/companion/{charisma,constitution,...}_
bonus.json`) despite `work-inventory.json`'s own stale `status:
"not-ingested"` field on all 8 — confirms the brief's `§17a` warning that
the inventory's `status` field is not the source of truth; `shape_ledger`'s
live join against `data/corpus/**` is.

## 2. `occult_adventures` — ingested by the proved-five-times mechanism, no new code

Registered `occult_adventures` in `scripts/transcribe_monster_tables.py`'s
`BOOKS` dict (mapping only — the script derives its own races/abilities file
set from `docs/work-inventory.json`'s unit set per book, not from a
per-book glob, so no separate "races_lsts" config exists at this layer).
Ran `python3 scripts/transcribe_monster_tables.py occult_adventures`:
wrote `src/rules_core/rules_tables/occult_adventures/monster_data.rs`
verbatim from the pinned oracle, reporting "5 orphan ability row(s)
transcribed WITHOUT an owner ... reachability NOT claimed" — the exact
honest non-claim `decisions.md §16` requires.

Wired the module (`mod.rs`'s `monsters_static()`/`monster_abilities_
static()`, mirroring `mythic_adventures`'s identical shape) and registered
`MONSTER_BOOKS` in `monster_chassis.rs`. Added the matching `MonsterBookSpec`
to `src/bin/gen_book_cache.rs` (`races_lsts: ["oa_races_b3.lst"]` — the one
file holding the book's sole `kind: monster` unit; `oa_races.lst`'s 4
`kind: race` units are a **different kind and a sibling lane's territory**,
deliberately left unregistered here to avoid emitting `MonsterStatBlock`
records into scope this cycle does not own).

Ran `cargo run --bin gen_book_cache -- occult_adventures` with
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` env vars set
(no `--allow-stamp-loss`, no destructive full-corpus regen — one targeted
book generator run): "1 new monsters (0 already on disk, left untouched), 5
new monster abilities (0 already on disk, left untouched)". `git status
--porcelain` before commit: only the files named above, **zero deletions**.

## 3. `companion` — the last 2 units, via the pre-existing generic closer

`python3 scripts/ingest_companion.py --dry-run` found the exact 2 target
units (`population: 769`, `skipped_existing_already_ingested: 767`) but both
`pi_skipped` under `sd32_t9_pi_review_companion_monsterability.py`'s own
`classify_uncertain_content` heuristic — a **stricter, non-canonical**
screen than `scripts/pi_scrub.py`'s blacklist scan (both tools coexist by
design in this script; the blacklist scan is `exact_bucket`, this heuristic
narrows `uncertain` further into `clear`/`still_undecidable`).

Both flags were already resolved, not merely suspected: `t9-pi-review-
companion-monsterability.md` §7 (an operator spot-check) already ruled
`advanced_race_guide:Earth Glide (Shaitan Binder Eidolon)` **clear** by
name — `"Shaitan" here is the genie-subtype term from the core Bestiary's
elemental taxonomy (djinn/efreeti/shaitan/marid), not a Golarion-specific
name.` `scripts/pi_scrub.py`'s own `normalized_term_hits` (imported, not
re-implemented; ran directly against both records' full `KEY`/`DESC` text)
independently confirmed **zero** blacklist hits. The remaining flag on
`Earth Glide` (`'burrowing', 'fish', 'solid'`) is the documented `a/an/the
<noun>` species-reference false-positive class this same file's other
widenings already correct for — none of the three is a species name.

Moved `"Shaitan"`/`"shaitan"` and `"burrowing"`/`"fish"`/`"solid"` from this
file's own documented "deliberately OFF" holdout comments into the
allowlist sets, citing the operator ruling and the independent `pi_scrub.py`
re-confirmation. Re-ran `--dry-run`: `pi_skipped: 0`, `written: 2`. Ran for
real: `python3 scripts/ingest_companion.py` → 2 new corpus files,
`skipped_existing_already_ingested: 767` unchanged (idempotent — no other
unit's disposition moved).

## 4. Pinned counts re-derived from live failures, never guessed (`§17a`)

- `monster_chassis::widening_the_facet_vocabulary_does_not_reclassify_
  any_existing_record`: temporarily bumped the length assertion to 3711 to
  reveal the real digest from its own panic output (`left: 14177688836309330301`
  → `0xc4c144e1483d297d`), the same discipline round 6's receipt used.
- `monster_catalog::tests::bonus_bestiary_ability_keys_carry_the_namespace`'s
  `owner_less_records_held` pin: live failure printed `left: 1053, right:
  1048` — repinned to 1053 (+5, matching the 5 new owner-less
  `occult_adventures` records exactly).
- `reach_gate::tests::every_ingested_family_is_accounted_for` /
  `unsurfaced_families_are_exactly_the_recorded_findings`: after adding
  `("occult_adventures", "monsters")`/`("occult_adventures",
  "monster_abilities")` reach arms, `occult_adventures/monsters` and
  `occult_adventures/monster_abilities` no longer appear in either
  failure's "no recorded finding" list (confirmed by diffing the live
  failure output before/after the arms were added).

## 5. RED → GREEN / test evidence

```
cargo test --locked --lib monster_chassis::
  8 passed, 0 failed (was 7 passed/1 failed before the repin)

cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean

cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (was 25 passed/1 failed before the repin)

cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed — IDENTICAL split to round 4/5/6's own recorded baseline.
  Re-confirmed by diff: neither `occult_adventures/monsters` nor
  `occult_adventures/monster_abilities` names any of the 8 failures' printed
  detail, before OR after my `advanced_race_guide/companion` writes (that
  book's `companions` family was ALREADY one of the 8 pre-existing failures
  before this cycle touched it -- my 2 new records join the same
  already-red, already-documented gap, not a new one; the failing-test
  count and the failing-test NAMES are both unchanged).

cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  15 passed, 0 failed (the round-6-era baseline's 2 known failures --
  `the_two_ingested_books_totals_reconcile_with_their_license_artifacts`,
  `every_book_landed_in_rules_tables_is_reported` -- are already green on
  this branch, resolved by a sibling lane's work landed since round 6;
  re-confirmed present-and-green both before and after my own edit to this
  file, not caused by it)

python3 -m unittest scripts.tests.test_transcribe_monster_tables
  18 tests, 17 passed, 1 failed (pre-existing, unrelated:
  InternalBundleAbilityHopIsResolved::test_an_ability_no_bundle_names_
  stays_an_orphan_and_is_not_shipped -- confirmed present against HEAD
  before this cycle's diff too, not this cycle's territory)

python3 -m unittest scripts.tests.test_ingest_companion_idempotent_rerun \
  scripts.tests.test_sd32_companion_allowlist_widening \
  scripts.tests.test_sd32_t9_pi_normalization_and_inheritance \
  scripts.tests.test_classify_companion_rows_book_dirs
  24 tests, 24 passed, 0 failed
```

## 6. Corpus regeneration — no unexpected deletions, no stamp loss

`git status --porcelain` before every commit: `git status --porcelain |
grep '^ D\|^D '` → empty at every checkpoint. No `--allow-stamp-loss` used;
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` env vars set
for the `gen_book_cache` run (no destructive full-corpus regen attempted —
one targeted `occult_adventures` generator run, one `ingest_companion.py`
run, both additive-only per their own printed reports).

## 7. What remains (three separate figures per `decisions.md §16`)

**Closure this cycle: 7 units, real ingestion** (5 `occult_adventures`
`monster_ability`, 2 `advanced_race_guide` `companion`), **0 reclassified.**
`no_record`: 157 → **150** (re-derived: `python3 scripts/shape_ledger.py
--inventory docs/work-inventory.json` → `no_record 150`, breakdown
`monster_ability` 93, `class_feature` 25, `equipment_modifier` 19,
`equipment` 10, `spell` 2, `ability` 1, `companion` 0).

**Reachability, reported honestly and separately (`§16`):** both new
`occult_adventures` records (`monster` 1, `monster_ability` 5) ship
`owners: &[]`/no `list_monster_catalog` ownership path — reachability **0**,
proven live and pinned by exact key in `reach_gate.rs::UNREACHED_RECORD_
FINDINGS`. Both new `companion` records ship the same way — reachability
**0** for the same reason as the other 16 `advanced_race_guide`/`companions`
records the pre-existing, sibling-owned `list_companion_catalog` reach gap
already carries (unchanged failing-test count, `§5` above).

**Instrument correction:** none new this cycle beyond the two pins named in
§4 (both are re-derivations of counts this cycle's own diff moved, not
independent corrections of a stale prior figure).

**Sweep for other inherited exclusions (brief item 3), found and disposed:**

| Coordinate | Stated reason | Survives `§27b`? | Disposition |
|---|---|---|---|
| `occult_adventures`, 5 `monster_ability` units | negated `PRECAMPAIGN` gate (reachability) | No — overturned | **Closed this cycle** |
| `advanced_race_guide`/`companion`, 2 units | "correctly parked" (was believed a pending PI ruling; actually an adjudicated reachability exclusion) | No — overturned | **Closed this cycle** |
| T9 reachability register (`named_features_wired`-shaped), `monster` family, 21/28 units | "PI-excluded needing a separate ruling" | Already an escalated `## Open blockers`-shaped request, not a silent carve-out (Blocker Discipline disposition 2) | Named, not closed — different epic/lane, `## Open blockers` already stands |
| Same register, 6/28 units | "structurally-correct `.MOD`/`.COPY` exclusions" | These are duplicate-overlay rows, not un-ingested objects — `§27b`'s test is about ingest, not about re-counting a delta row as a second object | Named, not this cycle's territory |
| `class_feature` lane, 39 of 64 `TYPE:*Choice` collision groups | "EXCLUDED from this cycle's rescue... reported as a hand-review population for the next cycle" | Disclosed scope boundary of a sibling lane's own dedup mechanism, not a silent ingest exclusion; merge-safety, not cost/awkwardness | Named, not this cycle's territory (`class_feature` is a sibling lane) |
| `bestiary`, 17 of 197 orphan candidates | "excluded by the pre-existing, unrelated `unscreenable`/`unmodelled_facet` screens" | Sibling `monster_ability` lane's own named residual (the other 98-unit population, not `occult_adventures`) | Named, not this cycle's territory |
| `declared-pi-audit`, 28 pre-existing violations | flagged as "next-cycle scope, not closed here" | Separate PI-audit epic, already disclosed as pre-existing and not this cycle's cause | Named, not this cycle's territory |

No other unnamed "out of scope"/"excluded"/"deferred"/"parked"/"correctly
skipped"/"not applicable"/"pending an operator ruling" language was found in
`decisions.md`, `progress.md`, or `kanban.md` describing a `no_record`-shaped
ingest carve-out. Every remaining hit is either historical narrative (an
already-superseded ruling, e.g. `decisions.md §10`'s account of the first
dispatch run) or a disclosed, named scope boundary belonging to a sibling
lane's own territory this cycle does not hold write scope over.

## 8. Next-cycle plan

1. **The 150 remaining `no_record` units** are the other lanes' territory
   per this cycle's brief (`monster_ability` 93 = the sibling lane's other
   98 minus nothing closed by them this cycle; `class_feature` 25,
   `equipment_modifier` 19, `equipment` 10, `spell` 2, `ability` 1 = the
   final-32 lane's). No further action from this lane.
2. **T9's 2,712-unit reachability register** (21 PI-excluded + 6
   structural + 1 genuine gap in the `monster` family alone; ~2,598 units
   not yet forensically checked) already carries an open `## Open
   blockers`-shaped ruling request in `kanban.md` — a different epic
   (reachability/consumer wiring, not Gate 1 ingest) and outside this
   cycle's granted territory. Named here for completeness, not
   re-escalated.
3. **`advanced_race_guide`/`companions`' 18-unit (was 16) unreached
   population** is a pre-existing, disclosed reach-gate gap
   (`list_companion_catalog` wiring) owned by a companion-reach lane this
   dispatch's brief does not name. My 2 new records join it honestly
   (`§16`); no new mechanism work is implied for this lane.
