# Cycle t9-class-feature-concat-pi-rust-generator-fix — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-class-feature-concat-pi-rust-generator-fix`)
- **Commit SHA:** see push log (this file is written pre-commit; SHA recorded
  in `progress.md`'s appended receipt)
- **Files touched:**
  - `src/rules_core/pi_screening.rs` — new Rust port of
    `scripts/pi_scrub.py`'s `canonicalize`/`normalized_term_hit`/
    `blacklist_term_hit_including_concatenated` (word-bounded, case-folded,
    bounded-OCR-normalized scan, OR an alphanumeric-normalized concatenated
    fallback bounded to 6 normalized characters), plus 9 new unit tests.
  - `src/rules_core/cache_gen/class_feature.rs` — new
    `redact_concatenated_blacklist_tokens`, wired into `generate()`
    unconditionally (both the `name_is_pi` and ordinary branches), plus a
    `pi_field` bookkeeping fix (the `name_is_pi` branch's `redacted_fields`
    build previously used an exact `pi_field.as_deref() == Some("description")`
    equality check, which the new step's own `pi_field` mutation could break);
    3 new unit tests plus a RED→GREEN mutation proof (temporarily neutered
    the new function to `false`, confirmed the new redaction test failed for
    the intended reason, reverted).
  - `data/corpus/*/class_feature/**/*.json` — all 17,954 `class_feature`
    records regenerated via `cargo run --locked --bin gen_cache_class_feature`
    against the pinned oracle: 17,940 modified, 3 newly written (one
    already-name-PI unit whose slug changed — see §3 below — plus 2 sibling
    files at a directory that previously had none), 14 removed (see §4).
  - `docs/retro/events/{t9-onboarding,sd31-transcribe}.jsonl` — one
    `correction` (population re-derivation, §17a) and one `incident`
    (stale-orphan generator finding, §4).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's
  own diff, `git diff a32e235321 -- src/rules_core/cache_gen/class_feature.rs
  src/rules_core/pi_screening.rs`; the wide `BASE_BRANCH...HEAD` form against
  `origin/develop` returns one pre-existing, unmodified-by-this-cycle line —
  confirmed by an empty diff on that exact line's file region — per
  `workflow-instruction.md §6` step 2's own caution about that form).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scoped diff).
- **Acceptance criterion:** close the 35-class_feature-hit remainder named in
  `t9-template-concat-pi-redaction-regen_cycle-1_cycle_receipt.md` §5 — wire
  the concatenated-blacklist-term scrub into the Rust `class_feature`
  generator path, then regenerate the already-shipped corpus through the
  guarded path so the leak actually leaves the corpus, matching that
  receipt's own standard for `template`.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`).
- **Status:** complete.
- **Notes:** see full body below.
- **Discovery forwards:** none new for `class_feature` itself. Two
  pre-existing, out-of-this-cycle's-scope conditions surfaced during
  verification, named here per `decisions.md §17a`/§15, not silently
  dropped:
  - `tests/pi_table_sweep.rs::rules_tables_carry_no_unbaselined_product_
    identity_hits` fails against `src/rules_core/rules_tables/
    feat_gap_tables.rs` (three hand-authored `feat` records, unrelated kind,
    unrelated file — confirmed by an EMPTY `git diff` on that file this
    cycle). This is the exact condition `pi_screening.rs`'s own pre-existing
    comment on `RENAME_SCRUB_SUPPLEMENTAL_TERMS` already names as "a
    pre-existing, unrelated leak this cycle does not own fixing" — not
    newly caused, confirmed pre-existing.
  - `tests/pi_screening_regeneration_round_trip.rs::crb_apg_acg_license_
    classification_round_trips_against_the_compiled_source_text` fails on
    `advanced_players_guide/spell` stale-leftover records — the `spell` kind
    is a sibling lane's named territory (see this brief's own "Territory"
    section); zero `spell` files appear in this cycle's `git status
    --porcelain`.
  - `corpus_literal_sweep` reports 2 NEW findings
    (`inner_sea_combat/class_feature/ranger_combat_style/…`,
    `ultimate_wilderness/class_feature/commando/…`) of the SAME
    `recurrence_key: corpus-literal-sweep-pi-exemption-gap` condition
    already recorded against `inner_sea_magic/ability/hidden_wand.json` — a
    redacted token is, by construction, no longer byte-present in the
    source. A sibling lane is fixing the sweep's own PI-exemption gap; not
    touched here per this cycle's explicit territory boundary
    (`src/bin/corpus_literal_sweep.rs` is off-limits).
- **Next-cycle plan:** none required for this shape — the population is at
  0. `docs/work-inventory.json`'s stamp-loss guard remains the one
  repo-wide blocker; re-run the regen's own verification (§5 below) once
  that guard is resolved by the sibling lane, to confirm this cycle's
  corpus write did not itself need a stamp regen.

---

## 1. Population, re-derived fresh (`decisions.md §17a` — do not trust 35)

The dispatch brief's own 35/5-books figure was itself flagged for
re-derivation (`t9-template-concat-pi-redaction-regen`'s receipt named it
"confirmed out of this cycle's reach, same as the deferring receipt named" —
i.e. carried forward without a fresh corpus-wide scan). Re-derived this cycle
with the SAME shared instrument the closing check below uses:

```python
import glob, sys; sys.path.insert(0, 'scripts')
from pi_scrub import blacklist_term_hit_including_concatenated, normalized_term_hit
# for each data/corpus/*/class_feature/**/*.json raw_tokens value v (skipping
# the literal redaction marker): count it when normalized_term_hit(v) is None
# (not already caught by the plain word-bounded scan) AND
# blacklist_term_hit_including_concatenated(v) is not None (the concatenated
# fallback DOES catch it).
```

```
BEFORE this cycle's fix+regen: 71 records / 108 raw-token hits, 7 books
  advanced_players_guide     1
  adventurers_guide         27
  book_of_the_damned_volume_2 8
  inner_sea_combat          14
  inner_sea_magic           17
  ultimate_combat            3
  ultimate_wilderness        1
```

**Not 35.** `book_of_the_damned_volume_2` (8) and `ultimate_combat` (3)
matched the prior receipt exactly. `advanced_players_guide` (1) matched too.
`adventurers_guide` (11→27) and `inner_sea_magic` (12→17) grew, and
`inner_sea_combat` (14) / `ultimate_wilderness` (1) are books that did not
exist in the prior receipt's scan at all — both are recent additions to
`BOOK_PRIMARY_FILES` from sibling cycles between the prior receipt and this
one (`git log` on `class_feature.rs`'s `BOOK_PRIMARY_FILES` shows commits
after the prior receipt's own base). Logged:
`scripts/retro.py correction 1787506778470-t9-onboarding-5b4ec9` (claimed 35,
actual 71, verified-by the scan above).

**The other 3 hits (38 minus 35), re-checked:** `equipment` and
`equipment_modifier` were named as closed by sibling lanes in the prior
receipt (both `no_record`-population movements, unrelated to this defect
class specifically — the prior receipt's own §3/§4 already separated these).
A fresh scan of `data/corpus/*/equipment/**/*.json` and
`data/corpus/*/equipment_modifier/**/*.json` with the identical instrument
this cycle: **0 concat-only hits in either kind.** Still closed. `spell`:
also 0 — a sibling lane's named territory, not re-verified further here
per this cycle's own scope boundary.

## 2. The Rust fix

`redact_desc_token_if_pi` (pre-existing) only ever screened the `DESC` raw
token, gated on `description`'s own PI classification. `scrub_name_pi_tokens`
(pre-existing) only ever ran on the `name_is_pi` branch. Neither covers a
record whose NAME and DESCRIPTION are both clean but some OTHER raw token
(`DEFINE`, `BONUS`, `TYPE`, `KEY`, …) carries a blacklisted term concatenated
PascalCase-style into an identifier with no separator
(`AldoriDefensiveParryLVL`, `CalistrianHunter ~ …`) — the exact gap the
Python-side `blacklist_term_hit_including_concatenated` was built to close
for the generic-ingest paths, per `scripts/pi_scrub.py`'s own module doc
comment.

`src/rules_core/pi_screening.rs` gains a byte-behavior port of that function
(`normalized_term_hit`/`normalized_term_hits`/
`blacklist_term_hit_including_concatenated`, plus the `canonicalize`/
`word_bounded_contains`/`alnum_normalize` helpers it depends on), using the
SAME 61-term `PI_BLACKLIST_TERMS` this file already carried (deliberately one
term ahead of `pi_scrub.py`'s signed-off 60 — that specific count difference
is pre-existing, documented, and out of this cycle's scope per that array's
own comment). `class_feature.rs` gains `redact_concatenated_blacklist_tokens`,
called unconditionally on every unit's raw tokens (both the `name_is_pi` and
ordinary branches), with `license`/`pi_field`/`pi_marker` bookkeeping updated
to list `"raw_tokens"` whenever it fires.

## 3. RED → GREEN

`redact_concatenated_blacklist_tokens_redacts_a_pascalcase_concatenated_hit`
reproduces the live `adventurers_guide/aldori_swordlord/defensive_parry.json`
shape exactly (KEY/DEFINE/BONUS tokens carrying the concatenated hit).
Mutation proof: temporarily replaced the function body with `return false`,
re-ran — the new test failed for the intended reason (`assertion failed:
any_redacted`), 2 sibling tests in the same file stayed green (proving the
mutation was scoped, not a blanket break). Reverted; full
`cache_gen::class_feature` (69 tests) and `rules_core::pi_screening` (28
tests) suites green after revert.

## 4. Corpus regen — guarded path, matching the `template` cycle's standard

```
cargo run --locked --bin gen_cache_class_feature   (PCGEN_CORPUS_ROOT set to the pinned oracle)
class_feature cache generated: 17954 records across 23 books (154 renamed
under a Codex-generated neutral name, decisions.md §24)
```

`git status --porcelain -- data/corpus`: 17,940 modified, 3 new, 0 deleted
(by the generator itself — see below for this cycle's OWN deletions).

**Concat-only leak population after the regen: 0** (identical instrument to
§1, re-run against the regenerated corpus).

**A second, unrelated-to-concatenation defect surfaced live during
verification and was closed this cycle as part of "match the template
cycle's standard" (the already-shipped records must actually leave the
corpus):** `name_pi_skipped` this run was 154, versus ~140 in the last prior
run (`git blame`/receipt history). `class_feature.rs`'s generator never
deletes a stale output file when a unit's PI classification changes between
runs — it was last run before "Aldori"/"Magaambya" joined the blacklist
(`decisions.md §19a`, already merged well before this cycle). 14 units that
were literal, un-redacted (`license: "OGL"`) at that time are name-PI now,
and this run correctly wrote their redacted `codex_named_unit_*` replacement
— but the OLD, un-redacted file for the SAME `(book, source_file,
source_line)` coordinate was left behind, untouched (`git status
--porcelain` shows it absent from the diff — confirmed for all 14 before
deleting any of them).

Found by: scanning `data/corpus/*/class_feature/**/*.json` for two records
sharing `(source.path, source.line)`, one `codex_generated_name: true`
(this run's fresh output) and one `false` (the stale orphan). **16 pairs
found; 2 are pre-existing, unrelated, legitimate multi-citation pairs
(`enlightened_bloodrager/bloodline_feat[-2].json`,
`core_rulebook/draconic_bloodline/draconic_bloodline[-2].json` — both
sides untouched by this cycle's `git status`, both carry the same `key`,
a known pre-existing PCGen multi-row-per-line shape, left alone).** The
other **14** are the orphan shape above, all in `adventurers_guide`
(class `magic_warrior`, feature-name pattern `*_spell_access_lvl_{0-6}`,
7 files; class `aldori_swordlord`, 1 file) and `ultimate_combat` (class
`monk_bonus_feat` and `master_of_many_styles`, 3 files each, 6 total) —
named here by directory/pattern, never by the record's own leaked name
text, per this bundle's own PI discipline. `grep -rl` across `src/`,
`tests/`, `apps/` confirmed zero references to any of the 14 paths before
`git rm`. Deleted; the fresh, correctly-redacted `codex_named_unit_*` file
at the same coordinate is the surviving record for each.

Logged: `scripts/retro.py incident 1787506792387-sd31-transcribe-858321`
(`recurrence_key: generator-orphans-stale-citation-on-reclassification`).

## 5. Verification

```
cargo test --locked --lib -- cache_gen::class_feature   69 passed, 10 ignored (live-oracle-only), 0 failed
cargo test --locked --lib -- pi_screening               28 passed, 0 failed
cargo run --locked --bin declared_pi_shipping_audit      65 violations, unchanged before/after, 0 in class_feature (all bestiary_4/monster_ability, a sibling lane's territory per decisions.md §26's own prior receipt)
concat-only scan (§1's instrument)                        71 -> 0
corpus_literal_sweep --json-out                           9 findings across 8 records (7 pre-existing shape + 2 new instances of the SAME named recurrence-key, both this cycle's redacted tokens diverging from source bytes by construction)
derived_evaluator_fixture_check --json-out                1836 cleared / 2577 rows, 0 failed
v06_work_inventory (both report env vars set, no --allow-stamp-loss)   refused (would drop 6506/8247 stamps); NOT forced; docs/work-inventory.json byte-unchanged (git status --porcelain confirms)
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json     no_record total 326 at this cycle's own commit, before this cycle's own §5 push-rebase picked up 4 concurrent sibling commits (monster_ability round 5, equipment_modifier find_citation repair, a corpus_literal_sweep fix, and a work-inventory regen) -- re-run AFTER the rebase: 301
```

`class_feature` carries 0 `no_record` before and after this cycle, both
readings — this cycle's defect is PI-redaction correctness on already-
ingested `class_feature` records, not an ingest gap, and this cycle's diff
touches no OTHER kind's `data/corpus` directory (`git status --porcelain`
scoped to `data/corpus/*/{monster_ability,equipment,equipment_modifier,
spell,companion}/**` is empty throughout, both before and after the
rebase). The 326->301 movement is 4 sibling lanes' concurrent work absorbed
via `git fetch && git rebase origin/tranche/12` per `workflow-instruction.md
§5`, not this cycle's own — named here rather than silently claimed
(`decisions.md §12c`).

## 6. What remains (explicit)

- **Nothing, for the concatenated-blacklist-term `class_feature` defect
  itself** — population is 0, proved by the same instrument that found 71.
- **`docs/work-inventory.json`'s stamp-loss guard** is the one repo-wide
  condition blocking a full inventory regen; a sibling lane is fixing it
  (`recurrence-key: corpus-literal-sweep-pi-exemption-gap`), per this
  cycle's own dispatch brief. Not this cycle's to fix (`src/bin/
  corpus_literal_sweep.rs` and `data/corpus/inner_sea_magic/**` are named
  off-limits).
- **`tests/pi_table_sweep.rs`'s `feat_gap_tables.rs` failure** and
  **`tests/pi_screening_regeneration_round_trip.rs`'s `advanced_players_
  guide/spell` failure** are both pre-existing, out-of-this-cycle's-kind
  conditions (confirmed by empty `git status`/`git diff` on the files
  those failures cite) — named here so they are not lost, not fixed here.
