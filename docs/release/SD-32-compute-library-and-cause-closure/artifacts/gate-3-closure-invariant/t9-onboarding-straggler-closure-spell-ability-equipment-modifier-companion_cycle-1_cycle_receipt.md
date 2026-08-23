# Cycle 1 — gate-3-closure-invariant / `spell`/`ability`/`equipment_modifier`/`companion`/`equipment` non-PI stragglers (`decisions.md §20`)

- **Card ID:** card 11 (`epic-2-cause-closure`), rows 11 and 15 left `in-progress` per dispatch instruction.
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/equipment_gap.rs` — `is_disabled_line` helper; `find_by_key_field`/
    `find_copy_variant`/`find_exact_first_column` skip disabled (`#`-prefixed) lines; 4 new tests.
  - `src/bin/v06_work_inventory.rs` — generic `.FORGET`-directive filter in `enumerate_file`
    (applies to every `Kind`, not equipment-only); new `forget_directive` `TrapRule`; 3 new tests.
  - `scripts/shape_ledger.py` — `build_cross_book_key_index` (THIRD, book-agnostic `(kind, key)`
    fallback, consulted only when the primary join and the same-book `key_index` fallback both
    miss); `classify_unit`/`build_ledger`/`main` updated to consult it.
  - `scripts/shape_coverage_standing_gate.py`, `scripts/family_vocabulary_reconcile.py`,
    `scripts/card15_reconcile.py` — updated to build and pass the same `cross_book_key_index` (the
    three other production call sites of `shape_ledger.build_ledger`, per the prior cycle's own
    discovery that all three must be updated together or their own `no_record` figure silently
    disagrees with the CLI's).
  - `scripts/tests/test_shape_ledger.py` — 8 new tests for the cross-book fallback and its builder.
  - `data/corpus/advanced_players_guide/equipment/equipmods/{crrsve_brst_m,crrsve_brst_r}.json`,
    `data/corpus/ultimate_combat/equipment/equipmods/reach.json` — 3 new files, written by
    `gen_cache_equipment_gap` after the `equipment_gap.rs` fix (real content ingest).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff against the
  starting PIN `41b01d41a3451137a3ac7e27644d9b65861b3dd5`:
  `git diff --unified=0 41b01d41a3... -- <files above> | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no match. The wider `BASE_BRANCH...HEAD` form returns one PRE-EXISTING hit from earlier
  tranche/12 work this cycle did not touch — confirmed by the narrower PIN-scoped diff, per §6 step
  2's own instruction that the wide form is not a per-cycle signal).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same PIN-scoped diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match).
- **Acceptance criterion:** `decisions.md §20` — `no_record == 0` is Gate 3's closure condition.
  Scope handed to this cycle: `spell` 29, `ability` 5, `equipment_modifier` 4, `companion` 2, and
  `equipment`'s 5 non-PI stragglers — the non-PI residue after the sibling `§24` lane's PI-name-
  blocked population.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  re-verified via `scripts/fetch-pcgen-oracle.sh --check` at cycle start — a fresh worktree's oracle
  slot was empty, bootstrapped first).

## `§17a` re-derivation before planning

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_now.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_now.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```
→ `monster_ability 100, equipment 87, spell 29, ability 5, equipment_modifier 4, companion 2` —
matches the dispatch brief exactly (total 227). Per-unit ids re-derived for every kind in scope
before any disposition was assumed (`§17a`).

## Establishing the PI split myself (`§15`/`§19`, not assumed from the brief)

The brief scoped me to "the non-PI stragglers" without pre-deciding the boundary. Re-derived per
unit, importing `scripts/pi_scrub.py` (never copied) for the blacklist scan, plus a direct read of
each unit's own PCGen `.lst` row for a declared `NAMEISPI:YES`/`DESCISPI:YES` (declared PI always
wins over an absent blacklist hit — several units carry no blacklist-term hit but ARE declared PI on
their own row, e.g. every one of `inner_sea_world_guide`'s 5 and 3 of `inner_sea_intrigue`'s 4).

**`spell` 29 → 24 PI-blocked (sibling `§24` lane's scope) / 5 non-PI (this cycle's scope):**

| Unit | Disposition |
|---|---|
| `adventurers_guide` ×4 (`ag_spells.lst` lines 4/16/34/46) | PI-blocked, declared `NAMEISPI:YES` on the row |
| `bestiary_4:summon_monster_ix_cthulhu` | PI-blocked, declared (matches `decisions.md §19b`'s own finding) |
| `inner_sea_faiths:blood_scent_achaekek` | PI-blocked, declared |
| `inner_sea_gods` ×4 | PI-blocked, declared/blacklist |
| `inner_sea_intrigue` ×4 | PI-blocked, declared |
| `inner_sea_magic` ×5 | PI-blocked, declared |
| `inner_sea_world_guide` ×5 | PI-blocked, declared |
| `bestiary:veil_self_only` | **Non-PI** — untouched this cycle, named below |
| `book_of_the_damned_volume_1:greater_teleport_self_plus_50_lbs_of_objects_only` | **Non-PI** — untouched this cycle, named below |
| `book_of_the_damned_volume_2:summon_demons_nascent_demon_lord` | **Non-PI — CLOSED this cycle** (cross-book instrument fallback) |
| `occult_adventures:repulsion` | **Non-PI — CLOSED this cycle** (cross-book instrument fallback) |
| `ultimate_combat:share_language_communal` | **Non-PI — CLOSED this cycle** (cross-book instrument fallback) |

**`ability` 5 → 1 PI-blocked / 4 non-PI, all 4 CLOSED this cycle (instrument fallback):**
`ultimate_campaign:trait_corpse_cannibal` is PI-blocked — its base declaration
(`uca_abilities_traits.lst:280`) carries `NAMEISPI:YES` directly on the row, and its own display
name embeds the deity the row itself names in a parenthetical — sibling `§24` lane's scope, not
touched or transcribed here, reported by coordinate only. The other 4
(`trait_hedge_magician`/`trait_natural_born_leader`/`trait_rich_parents`/(ultimate_psionics)
`trait_adopted`) are non-PI and closed (see below).

**`equipment_modifier` 4 → 1 PI-blocked / 3 non-PI, all 3 CLOSED this cycle (real ingest):**
`adventurers_guide:equipment_modifier` (`ag_equipmods.lst:1`) is PI-blocked — its own display name and its
`PRETYPE` prerequisite both tie it to a Product-Identity-declared base armor item (multiple sibling
rows in the same book declare `NAMEISPI:YES` for that concept); sibling `§24` lane's scope, reported
by coordinate only. The other 3 (`crrsve_brst_m`/`crrsve_brst_r`/`reach`) are non-PI and closed.

**`companion` 2:** both clear (`pi_scrub.blacklist_term_hit_including_concatenated` → `None` on name,
key, and description; no `NAMEISPI:`/`DESCISPI:` on either row) — **not** PI-parked. See "Companion 2"
below for the real (non-PI) reason they stay `no_record`.

## Cause 1: `equipment_modifier`'s 3 — a disabled duplicate line masks a live `.COPY=` mint

`equipment_gap.rs::find_citation`'s `try_files` tries `KEY:` matches (`find_by_key_field`) across a
book's files BEFORE `.COPY=` matches (`find_copy_variant`) — by design, per that function's own doc
comment (a `.COPY=` target is a stronger identity signal than a coincidental first-column match).
Traced `advanced_players_guide:equipment_modifier:crrsve_brst_m`/`crrsve_brst_r` and
`ultimate_combat:equipment_modifier:reach` to the SAME real defect, already half-documented in this
file's own code: `apg_equipmods.lst` carries a DISABLED (`#`-prefixed) old-style row declaring
`KEY:CRRSVE_BRST_M` (line 13 — PCGen's own comment explains why: Ultimate Equipment merged the
melee/ranged variants into one modifier) in the SAME FILE as a LIVE
`Special Ability ~ Corrosive Burst ~ Weapon.COPY=CRRSVE_BRST_M` (line 59, the real, still-shipping
alias mint). `find_by_key_field` matched the disabled line's `KEY:` field FIRST (no disabled-line
guard existed in that function at all), so `find_citation` bound the citation to the disabled line —
which `disabled_identity_column` then correctly refused to ship (downstream guard, already existed),
but as a silent TOTAL EXCLUSION rather than falling through to the live `.COPY=` mint that actually
resolves the object. `equipment_gap.rs`'s own comment on that downstream guard's call site already
named all three blocked units by their coordinate-safe identifiers (`CRRSVE_BRST_M`, `CRRSVE_BRST_R`,
`REACH`) as a KNOWN, unfixed shortfall.

**Fix:** `is_disabled_line` (new helper, same predicate `disabled_identity_column` already uses:
`line.trim_start().starts_with('#')`) is now consulted inside `find_by_key_field`, `find_copy_variant`,
and `find_exact_first_column` — a disabled line can never win a citation search while a live line for
the same identity exists anywhere in the search order.

### RED → GREEN

```bash
cargo test --locked --lib rules_core::cache_gen::equipment_gap
```
- `find_citation_skips_a_disabled_key_line_in_favor_of_a_live_copy_variant` — reproduces the exact
  real-file shape (disabled `KEY:` line + live `.COPY=` line, same file), RED before the fix
  (`Some(("apg_equipmods.lst", 1))` — the disabled line — instead of line 2), GREEN after.
- `find_citation_returns_none_when_the_only_match_is_disabled` — negative control: when NO live line
  exists anywhere, still returns `None` (the downstream `disabled_identity_column` guard's residual
  case, unchanged).
- 20/20 pass (1 pre-existing `#[ignore]`d), 0 regressions.

### Blast-radius check before trusting the fix (`§17a`)

The ignored `find_citation_full_population_regression` test re-resolves EVERY already-shipped
`lst_token`-sourced equipment record's citation and asserts it is unchanged:

```bash
PCGEN_CORPUS_ROOT=<oracle> cargo test --locked --lib \
  rules_core::cache_gen::equipment_gap::tests::find_citation_full_population_regression -- --ignored --nocapture
```
Result: `checked=7466 mismatches=3` — **but re-run against `HEAD` (my changes reverted via
`git show HEAD:<file>`, restored after) shows the IDENTICAL 3 mismatches** (`holy_symbol_silver`,
`holy_symbol_wooden`, `masterwork_tool-2`, all a pre-existing, unrelated citation drift this fix does
not touch or worsen) — proven by running the full test twice, once against each version, both times
producing byte-identical mismatch output. My fix introduces **zero** new mismatches.

### Real ingest

```bash
CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run --locked --bin gen_cache_equipment_gap
```
```
Equipment gap cache generated: 0 equipment, 3 equipment_modifier records
```
`git status --porcelain -- data/corpus` before/after: exactly 3 new files, zero deletions, zero
modifications to any pre-existing file:
```
?? data/corpus/advanced_players_guide/equipment/equipmods/crrsve_brst_m.json
?? data/corpus/advanced_players_guide/equipment/equipmods/crrsve_brst_r.json
?? data/corpus/ultimate_combat/equipment/equipmods/reach.json
```
Citations verified correct (the live `.COPY=` lines, not the disabled duplicates):
`apg_equipmods.lst:59`/`:60`, `uc_equipmods.lst:54`. `reach.json` ships `completeness: "chassis_only"`
(description `null`, honestly, not fabricated) — its base declaration
("Special Quality ~ Reach") lives in `core_rulebook`, a DIFFERENT book than `ultimate_combat`'s own
`.COPY=` alias, and `gen_equipment_gap_tables.rs`'s SEPARATE `collect_base_fields` function (which
compiles the static gap table `equipment_gap.rs::generate()` reads `entry.description` from) only
looks within one book's own input files — a second, independent cross-book blindness in a DIFFERENT
generator, out of this cycle's scope (named as a discovery below, not fixed).

## Cause 2: a cross-book citation-redirect — a `(book, kind, key)` fallback isn't wide enough

The prior cycle's citation-redirect fix (`978d215227`) added a SAME-BOOK `(book, kind, key)`
fallback for the shape "this book's row cites a content-free overlay line, but the real record
(same book) was generated from a different physical file." Tracing `ability`'s 4 and `spell`'s
`repulsion`/`share_language_communal`/`summon_demons_nascent_demon_lord` found a WIDER version of the
identical root shape: a book's row deliberately does not (re-)declare a record that already exists
under a **different book entirely**.

- `ultimate_campaign:ability:trait_hedge_magician`/`trait_natural_born_leader`/`trait_rich_parents` —
  each cited from a `.MOD` PFS-legality-overlay row (`pfs_uca_abilities_traits.lst`), but the real
  base declarations are `#`-commented-out (disabled) in `ultimate_campaign`'s OWN file with the
  comment "From the APG" — the live declarations are in `advanced_players_guide`, already ingested
  (`data/corpus/advanced_players_guide/ability/{hedge_magician,natural_born_leader,rich_parents}.json`).
- `ultimate_psionics:ability:trait_adopted` — same shape, real declaration lives elsewhere too.
- `occult_adventures:spell:repulsion` — `ingest_spells.rs`'s own `already_ingested_oa` deliberately
  skips re-declaring a spell already modelled by `crb`/`apg`/`acg`/`advanced_race_guide`/
  `ultimate_intrigue`/`ultimate_magic` ("a handful of rows exist only to widen an existing spell's
  class access", per that file's own doc comment) — "Repulsion" ships under `crb`'s own citation.
- `ultimate_combat:spell:share_language_communal` — same `already_ingested_uc` shape.
- `book_of_the_damned_volume_2:spell:summon_demons_nascent_demon_lord` — `ingest_spells.rs`'s own
  `BookInput` doc comment for this book already documents WHY: the `.lst` declares this spell twice,
  once unconditionally (`botd2_spells.lst`, the ingested file) and once in a
  `!PRECAMPAIGN:1,Inner Sea World Guide`-gated "no-duplicates restatement"
  (`botd2_spells_ndl.lst`, where THIS unit's citation lives) — the gated file is deliberately not
  ingested (this pipeline models no campaign gating), and the real record ships under
  `inner_sea_world_guide`'s own citation instead
  (`data/corpus/inner_sea_world_guide/spell/summon_demons_nascent_demon_lord.json`, verified present).

Neither the primary `(book, source_file, source_line)` join nor the same-book `(book, kind, key)`
fallback can ever resolve these — the record's real book is neither of those two.

**Fix:** `build_cross_book_key_index` (new, `scripts/shape_ledger.py`) — a THIRD, book-agnostic
`(kind, key)` index, consulted by `classify_unit` only when BOTH the primary join and the same-book
fallback miss. A PCGen `KEY:` token is meant to be a globally unique identifier within its category
(unlike `data.name`, a display string with no such guarantee — the exact hazard
`equipment_gap.rs`'s own `held`-map comment already documents), so a same-key match across two
DIFFERENT books is expected to be the literal same object — but "expected" is not "proven":
**when two different books' records under the identical `(kind, key)` carry DIFFERENT formula
tokens, the index marks that key `None` (ambiguous) rather than guessing which book a third book's
reference means** (`§1a`: under-include rather than invent). Two books legitimately restating the
identical record (same tokens) is not that hazard and resolves normally.

### RED → GREEN

```bash
python3 -m unittest scripts.tests.test_shape_ledger -v
```
8 new tests, all RED before (`AttributeError`/`TypeError` on the not-yet-existing function/parameter)
GREEN after: cross-book match when both narrower tiers miss; cross-book NEVER fires when the same-
book fallback already resolves it (last-resort ordering); an ambiguous (divergent-content) collision
declines rather than guesses; a `(kind, key)` match never fires across a DIFFERENT kind; backward
compatibility with no `cross_book_key_index` argument; plus 4 tests for the builder itself (indexes
by kind+key across books; two books sharing a key with DIFFERENT content is ambiguous; two books
sharing a key with IDENTICAL content is not ambiguous; a record with no `data.key` is not indexed).
**47/47 `test_shape_ledger.py` green.**

### Consumers updated (per the prior cycle's own discovery: 3 other production call sites exist)

`build_ledger`/`main` plumb `cross_book_key_index` through (built via a WHOLE-CORPUS walk,
deliberately not restricted to the not-done units' own `books` set — the fallback's entire purpose is
finding a record under a book NONE of those units belong to). `shape_coverage_standing_gate.py`
(the live Gate 3 standing gate), `family_vocabulary_reconcile.py`, and `card15_reconcile.py` all
updated identically, so no consumer's own `no_record` figure can silently disagree with the CLI's.
**75/75 across the 3 touched Python test suites; `card15_reconcile.py` smoke-tested end-to-end
(`exit=0`) to confirm the wiring change doesn't break its own script.**

### Result, verified not assumed

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after_crossbook.json
```
| Kind | Before | After | Delta | Cause |
|---|---:|---:|---:|---|
| `ability` | 5 | 1 | **-4** | cross-book fallback (4 real closures; 1 PI-blocked remains) |
| `spell` | 29 | 26 | **-3** | cross-book fallback (`repulsion`, `share_language_communal`, `summon_demons_nascent_demon_lord`) |
| `equipment` | 87 | 79 | **-8** | cross-book fallback, side effect (generic fix, not spell/ability-scoped) — 8 named units spot-checked, e.g. `bestiary_2:equipment:maul_of_the_titans` correctly resolves against `core_rulebook`'s own already-shipped "Maul of the Titans" record (verified: both files exist, same key, real shared content) |
| `equipment_modifier` | 4 | 4 | 0 (unaffected — its 3 non-PI units are the disabled-line shape, fixed separately above) | — |
| `companion` | 2 | 2 | 0 (unaffected — see below) | — |
| `monster_ability` | 100 | 100 | 0 (out of scope, untouched) | — |

Standing gate re-run: `no_record budget: 212/34631 vs. baseline 21521/36028 -- exceeded: False`, exit 0.

After the equipment_modifier real-ingest regen (Cause 1) on top: `equipment_modifier` 4→1,
bundle-wide `no_record` **227 → 209** (18-unit net movement this cycle: 3 real closures
(`equipment_modifier`) + 15 instrument-corrected (4 `ability` + 3 `spell` + 8 `equipment`)).

## Cause 3: `.FORGET` is a removal directive, minted as a unit anywhere in the codebase — fixed as a class

`equipment_gap.rs::generate()` already excludes `.FORGET` rows downstream (its own doc comment names
the exact incident: `advanced_class_guide/_pfs/pfs_acg_equip.lst:6-7`, "Dust Knuckles"/"False Face"
marked removed from Pathfinder-Society-legal play — a removal directive, not a declared item). The
brief's own precedent (the prior `equipment-modifier-no-record-wave5` receipt) already named the
upstream cause: `docs/work-inventory.json`'s enumeration (`src/bin/v06_work_inventory.rs`) has NO
`.FORGET` handling at all, unlike `.MOD` (which gets dedicated special-case routing). Confirmed by
reading the source: `enumerate_file` has a `.MOD` branch (`if let Some(mod_at) = first.find(".MOD")`)
with no `.FORGET` counterpart anywhere in the file.

**Fix:** a generic `if first.ends_with(".FORGET") { ...; continue; }` check, applying to every `Kind`
uniformly (checked per the brief's explicit instruction: "fix it generically — for every kind, not
just equipment"), plus a new `forget_directive` `TrapRule` entry documenting the shape for the next
reader of `TRAP_RULES`.

### RED → GREEN

```bash
cargo test --locked --bin v06_work_inventory forget_directive
```
3 new tests: a `.FORGET` row is dropped, not enumerated (`Kind::Equipment`, RED before —
`out.units.len()` was 1, not 0 — GREEN after); the SAME drop for a NON-equipment kind
(`Kind::Ability`), proving the fix is generic, not equipment-specific; an ordinary (non-`.FORGET`)
row is entirely unaffected. **359/360 `v06_work_inventory` tests pass** (the 1 failure,
`e14_harness_tests::a_key_two_books_share_grounds_only_the_book_whose_corpus_was_read`, is a
PRE-EXISTING failure confirmed identical on `HEAD` before this cycle's changes — verified by
swapping in `git show HEAD:src/bin/v06_work_inventory.rs`, re-running, seeing the SAME panic message
about `ultimate_equipment`'s corpus directory, restoring my version — unrelated to `.FORGET`).

### Blast-radius check (`§17a`) — real corpus, every book/kind

```bash
PCGEN_CORPUS_ROOT=<oracle> cargo run --locked --bin v06_work_inventory -- --summary
```
`grep -o '"forget_directive": [0-9]*'` on the full summary output (every book, every kind) →
**exactly ONE occurrence, `advanced_class_guide: "forget_directive": 2`** — matching the 2 targeted
units precisely, and confirming **no other book or kind is affected** (the fix's blast radius is
exactly the population it was meant to close, corpus-wide, not sampled).

### What is NOT done, named explicitly: the code fix is not yet materialized in the committed inventory

`docs/work-inventory.json` (49,513 units) was **not regenerated** this cycle. The code fix is tested
and verified correct against the live corpus (above), but the CURRENT committed inventory still
enumerates the 2 `.FORGET` units from before the fix, so `shape_ledger.py`'s `no_record` count
(209, reported above) does **not yet** reflect their removal — a full inventory regen is a
repo-wide, ~49.5k-unit operation this cycle's own warnings (and this bundle's own recorded near-miss
history) flag as needing a full pinned-count sweep across `tests/`/`src/`/`scripts/`/`apps/` before
landing, which is out of this bounded cycle's scope. **This is a landed fix pending materialization,
not a closure claim** — reported honestly per `§16`'s "a unit moved out of a shape is not a unit
closed until proven."

## `equipment`'s remaining 3 non-PI stragglers (`bestiary_2`/`bestiary_3`/`mythic_adventures`) — traced, not closed

Re-verified the prior cycle's own trace, unchanged this cycle:

1. **`bestiary_2:equipment` ×1** (`_pfs/pfs_b2_equip_arms_armor.lst:8`) and **`bestiary_3:equipment`
   ×1** (`_pfs/pfs_b3_equip_arms_armor.lst:10`) — a PFS-overlay row citing a SHORTHAND key for an
   item already ingested under a longer, qualified key. The new cross-book `(kind, key)` fallback
   correctly does **not** fire here — the two key STRINGS genuinely differ (shorthand vs. qualified),
   so a bare key match would repeat exactly the name-collision hazard this cycle's own ambiguity
   guard exists to avoid. A real fix needs a name-alias table (shorthand → canonical key); not
   attempted this cycle (out of the time available after the three larger causes above).
2. **`mythic_adventures:equipment` ×1** (`ma_equip.lst:137`, "Nexus Crystal") — a plain, undeclared,
   non-blacklisted base item row (`TYPE:Artifact.Minor`), absent from `equipment_gap_tables.rs`'s
   generated output even after a fresh regen. Genuinely untraced — not a `.COPY=`/`.MOD`/disabled-line
   shape, not PI, not a cross-book redirect (no sibling book carries this key). Root cause not found
   this cycle.

## Companion's 2 — the `§19c` PI framing in the dispatch brief was WRONG; re-derived the real reason

The brief's framing ("long-parked pending an operator PI ruling, `§19c`") does not survive
re-derivation. `§19c` in `decisions.md` is about a DIFFERENT population (the ~360 generic-token
uncertain units from the T9 PI review) and never mentions `companion`. Ran the actual instrument:

```python
import pi_scrub
pi_scrub.blacklist_term_hit_including_concatenated(name)  # -> None
pi_scrub.blacklist_term_hit_including_concatenated(key)   # -> None
pi_scrub.blacklist_term_hit_including_concatenated(desc)  # -> None
```
on both `advanced_race_guide:companion:shaitan_binder_eidolon_earth_glide` and
`..._noble_eidolon` — zero hits, and neither row carries a `NAMEISPI:`/`DESCISPI:` declaration.
**These are PI-clear, not PI-parked.**

The REAL reason they stay un-ingested is reachability, and it is an already-adjudicated, generic
doctrine, not a defect: `advanced_race_guide/companion_data.rs` (GENERATED by
`transcribe_companion_tables.py`) is a `#[rustfmt::skip]`-style generated table whose own doc comment
explicitly lists both units under "NOT transcribed — ability rows no creature row of this book owns,
so nothing could ever reach them on screen. Dropped rather than emitted unreachable (`decisions.md
§50`, adopted from the monster lane; `§56.1`)." Verified: neither row's `PRERACE:1,RACETYPE=Shaitan
Binder Eidolon` prerequisite has a matching companion CREATURE row anywhere in
`arg_races_companion.lst` (`grep -n "Shaitan" arg_races_companion.lst` → no match) — the eidolon
archetype these abilities describe is never itself declared as a companion creature in this book, so
no player-facing consumer could ever display them even if ingested.

**Not touched this cycle.** Fabricating a corpus record for content nothing can ever reach would
violate `§1a` (a relabelled shape is not a closed shape) and the no-stub doctrine (`no-stub-mvp-
doctrine.md`) the same way the `§50`/`§56.1` precedent was adjudicated to forbid for the monster lane.
This is the SAME open tension the prior equipment cycle already escalated for the 82 PI-excluded
`equipment` units (Gate 3's literal `no_record == 0` vs. a correctly-excluded-by-design population) —
named again here for `companion`, not re-litigated or silently re-decided by this cycle.

## Closure figures — four separate numbers (`decisions.md §16`)

- **Closure** (real content ingested, `no_record` → `matched`/`no_formula_tokens`, new corpus JSON
  written): **3** `equipment_modifier` units (`crrsve_brst_m`, `crrsve_brst_r`, `reach`).
- **Reclassification:** none.
- **Reachability (Gate 2):** honest claim of **0** for the 3 closed units — not wired into any
  consumer this cycle (Gate-1 measurability only, same precedent every prior equipment-widening
  cycle in this bundle has set).
- **Instrument correction** (mismeasurement fixed, zero content written): **15 units** — `ability` 4,
  `spell` 3, `equipment` 8 — moved from `no_record` to their true `join_status` via the cross-book
  fallback. **This is not closure.** These units were never un-ingested; they were mismeasured
  because the instrument's join could not look past the book boundary, exactly the shape the brief's
  own precedent (the citation-redirect fix) established and instructs to book separately.
- **Landed, not yet materialized:** the `.FORGET` census fix (2 `equipment` units, corpus-wide
  verified as the ONLY population it affects) — code-correct and tested, but `docs/work-
  inventory.json` was not regenerated this cycle, so this does not yet move any measured count. Not
  counted in the four numbers above; named separately per its own honesty requirement.

## PI screening

Zero drops from this cycle's own new writes (`equipment_gap.rs`'s existing, unmodified PI screen ran
unconditionally on the 3 new records the same way it always has — this fix sits entirely upstream of
that screen, only changing which citation line a record resolves against). No blacklist term or PI
item name is written anywhere in this receipt, the diff, or any commit message — every PI-blocked
unit above is named by kind:book:coordinate only (verified: the diff and this receipt were grepped
against the full 60-term `PI_BLACKLIST_TERMS` list, zero hits).

## Fixture discipline (`decisions.md §3`)

`is_disabled_line` is a pure structural predicate (no fabrication possible). `build_cross_book_key_index`
never fabricates a formula token — it only re-groups tokens `build_corpus_index`-style scans already
read from real, on-disk corpus JSON, and marks a genuine content divergence `None` rather than
picking one side. The 3 new equipment_modifier corpus records were generated by the SAME unmodified
`generate()` write path every other equipment_gap record uses — `CacheRecord`'s `raw_tokens`/
`description`/`source` fields are transcribed from the real `.lst` bytes at the resolved citation
line, never invented.

## What is NOT done, named explicitly (no silent narrowing)

- `spell`'s `bestiary:veil_self_only` and `book_of_the_damned_volume_1:greater_teleport_self_plus_50_lbs_of_objects_only`
  — both are same-book `.COPY=` variant rows whose base spell already exists, but `ingest_spells.rs`'s
  `is_base_declaration` EXCLUDES `.COPY=`/`.MOD` rows from parsing entirely (no `.COPY=` resolution
  exists anywhere in this pipeline, unlike `equipment_gap.rs`). This is a GENERIC gap (likely affects
  other books' spell `.COPY=` variants too, not sampled this cycle) requiring real new parsing logic
  in a core, multi-book-shared ingest path — correctly out of this cycle's remaining time budget
  rather than a rushed change to a high-blast-radius generic pass. Scoped as a follow-up.
- `equipment`'s `bestiary_2`/`bestiary_3` shorthand-key aliases (2) and `mythic_adventures`'s
  untraced gap (1) — see above.
- `companion`'s 2 — correctly excluded by existing `§50`/`§56.1` reachability doctrine, not a defect;
  the brief's `§19c` framing corrected.
- `gen_equipment_gap_tables.rs`'s own cross-book blindness in `collect_base_fields` (discovered while
  tracing `reach`'s `null` description) — a SECOND, independent instance of the same cross-book gap
  class this cycle fixed once (in `shape_ledger.py`) and once (in `equipment_gap.rs`'s citation
  search), but in a THIRD location (a different generator's description-inheritance lookup). Named,
  not fixed — `reach.json` ships honestly as `chassis_only` rather than blocked or fabricated.

## Discoveries

- The cross-book `(kind, key)` fallback is corpus-wide generic, not spell/ability-scoped — it
  recovered 8 `equipment` units as a side effect with zero code touching `equipment`'s own ingestion
  path, the same shape the prior cycle's own same-book fallback exhibited for `spell`.
- `gen_equipment_gap_tables.rs`'s `collect_base_fields` has the identical cross-book blindness this
  cycle fixed twice elsewhere — worth a future cycle checking whether it explains any of the
  remaining `equipment_modifier`/`equipment` `chassis_only`/`unknown` population.
- `ingest_spells.rs` has no `.COPY=` resolution mechanism at all (confirmed: `is_base_declaration`
  explicitly excludes both `.MOD` and `.COPY=`) — a generic gap, likely wider than the 2 units named
  here, worth a dedicated future cycle given its blast radius (touches the shared multi-book spell
  ingest pipeline `decisions.md §17` already collapsed from 7 binaries into 1).

## Disk

```bash
df -h /
```
(see final message)

## Post-rebase addendum (`§17a`) — a real regression found in a concurrent sibling commit, fixed as part of this cycle

`git fetch origin tranche/12 && git rebase origin/tranche/12` (§5 protocol) landed
`11a84bced5` ("shape_ledger.py kind-aware join; run ingest_generic_kind --kind trait"), a concurrent
sibling fix making `build_corpus_index`/`build_corpus_key_index`'s join key include `kind` (fixing a
real cross-kind collision: `trait`→`ability` 487, `class_feature`→`race_trait_generic` 25). Re-running
this cycle's own re-derivation immediately after the rebase surfaced a **second, unrelated regression**
introduced by that same commit: `equipment_modifier` `no_record` jumped to **1,003** (from this
cycle's own pre-rebase 1) — not a genuine collision, but a **real, corpus-wide false negative**. Every
`equipment_modifier` record in this ENTIRE corpus lives at `<book>/equipment/equipmods/*.json`
(`equipment_gap.rs::generate()`'s own write path; confirmed by direct search — **zero** bare
`equipment_modifier/` directories exist anywhere), but the sibling fix's kind derivation ("the
directory one level under book") read that as kind `"equipment"`, not `"equipment_modifier"` — so
EVERY `equipment_modifier` record's own join now failed, including this cycle's own 3 freshly-ingested
records.

**Fixed as part of this cycle** (my own deliverable depended on it; `decisions.md §22`, fix the source):
new `kind_from_path_parts` helper (`scripts/shape_ledger.py`) — a directory-NAME check (only
`equipment/equipmods/` maps to kind `equipment_modifier`; the other 3 real `equipment/<X>/`
sub-groupings that exist corpus-wide — `arms_armor`/`general`/`magic_items` — correctly stay kind
`equipment`, confirmed these are the only 4 subdirectory names that exist under any book's
`equipment/`), composed with the existing `normalize_kind_dir`'s `_generic`-suffix handling. Used at
all 3 kind-derivation call sites (`build_corpus_index`, `build_corpus_key_index`, and this cycle's own
`build_cross_book_key_index`). 3 new tests (the real `equipmods`-nesting case; a negative control for a
plain `equipment/*.json` record; a negative control for a real `equipment` sub-grouping directory like
`arms_armor`), RED confirmed before the fix (`AssertionError: ... not found in {(...'equipment'...)}`),
GREEN after. **53/53 `test_shape_ledger.py`.**

### Result — the true, corrected `equipment_modifier` state, re-derived not assumed

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
```
`equipment_modifier` `no_record`: 1,003 → **19**. My own 3 closures (`crrsve_brst_m`/`crrsve_brst_r`/
`reach`) and 1 PI-blocked unit (`ag_equipmods.lst:1`, sibling `§24` scope) are among that 19's
complement/inclusion as expected — but the fix also un-masks **18 units this cycle's own scope never
named**, previously hidden by the pre-existing kind-blind join's own false-positive matches:
`advanced_class_guide` ×14, `pathfinder_unchained` ×4 (all `special_ability_*` armor/weapon/ammunition/
shield modifiers). **Not investigated or closed this cycle** — genuinely new, real `equipment_modifier`
work this cycle's own dispatch scope did not include, discovered only as a byproduct of fixing the
regression that blocked my own 3 closures from registering. Named here per `§16`/`§17a`, not silently
absorbed into this cycle's own closure count.

### Bundle-wide `no_record`, final, this cycle's real net effect

| Kind | Cycle start (227 total) | This cycle's own scope closed | Post-rebase kind-fix (out of my scope, sibling's own commit) | Final |
|---|---:|---:|---:|---:|
| `ability` | 5 | -4 (instrument) | 0 | 1 |
| `spell` | 29 | -3 (instrument) | 0 | 26 |
| `equipment_modifier` | 4 | -3 (closure) | +18 (newly un-masked, not this cycle's scope) | 19 |
| `equipment` | 87 | -8 (instrument) | 0 | 79 |
| `companion` | 2 | 0 | 0 | 2 |
| `monster_ability` | 100 | 0 (out of scope) | 0 | 100 |
| `class_feature` | 0 | — (not this cycle's kind at all) | +25 (sibling's own reported discovery) | 25 |
| **Total** | **227** | | | **252** |

Bundle-wide `no_record` moved 227→252 net — **up**, not down, because the corrected, kind-aware
instrument (a mix of the sibling's own fix and this cycle's `equipmods`-nesting correction) is more
honest than what it replaced, surfacing 43 previously-hidden real gaps (18 `equipment_modifier` + 25
`class_feature`) against this cycle's own 18-unit reduction (3 closed + 15 instrument-corrected in the
kinds this cycle was actually scoped to). Standing gate still passes:
`no_record budget: 252/34631 vs. baseline 21521/36028 -- exceeded: False`. This is `decisions.md §20`'s
own point made concrete: a more accurate measurement can legitimately raise the count, and that is not
a regression to hide — it is the gate doing its job.
