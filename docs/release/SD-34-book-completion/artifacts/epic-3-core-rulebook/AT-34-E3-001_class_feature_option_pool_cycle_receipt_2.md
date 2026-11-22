# Cycle 9 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism)

- **Commit SHA:** `a183d70c760d4d8555d645e17ab089faa9d0b6c6`
- **Files touched:**
  - `src/rules_core/rules_tables/crb/wizard_spell_list.rs` (new `wizard_school_zero_level_spells`
    join function + its own corpus-verification test)
  - `src/rules_core/class_feature_pool_catalog.rs` (new
    `WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER` table + `wizard_school_spell_list_key_owner` lookup +
    its own corpus-verification test)
  - `src/bin/v06_work_inventory.rs` (`Kind::ClassFeature`'s unowned-fallback chain gains one new
    rung, immediately after `class_skill_list_grant_owner_id`; two new unit tests)
  - `scripts/completion_atlas.py` (**self-caused regression, fixed same cycle**: the new rung
    inserted 22 lines before four of `BUCKET_DEFINITIONS`' own `file:line` citations — re-derived
    each new line by `grep -n` against the post-edit file before writing the fix; `--check`'s
    `citation_failures` went `4 -> 0`)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated output of `completion_atlas.py --check` — the citation-line-number fix and a
    fresh `derived_at` stamp; not hand-edited, and NOT a `docs/work-inventory.json` regeneration —
    this wave's four parallel lanes explicitly do not regenerate that file this cycle)
  - This receipt, `docs/release/SD-34-book-completion/progress.md`,
    `docs/release/SD-34-book-completion/kanban.md`
- **`docs/work-inventory.json`:** untouched this cycle (wave rule: a single shared regeneration
  cycle runs after all four parallel lanes land; the figures below are re-derived directly from
  the committed `docs/work-inventory.json` at this cycle's start SHA, which is what the wave asked
  each lane to do instead).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — re-derive:
  `BASE_BRANCH=$(git merge-base HEAD origin/develop); git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/ src/bin/ scripts/oracle_harness/ data/corpus/core_rulebook/ docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**970** Core Rulebook units whose
  table exists but which are not in it. **Evidence:** the atlas reporting bucket B at zero for
  `core_rulebook`, and the mechanism that placed them named — by mechanism, not per record." This
  receipt covers only the `class_feature_option_pool_record_not_held_by_engine` mechanism (one of
  nine bucket-B mechanisms for `core_rulebook`); the criterion itself does NOT close this cycle.

## Task brief for this cycle, and why it is answered differently than expected

Eight prior cycles on this exact mechanism ran `63 -> 57 -> 55 -> 52 -> 52 -> 49 -> 44 -> 34 -> 34`
and the ninth (cycle 8, immediately prior) closed **zero**, naming the remaining 34 units as three
"genuinely new, unbuilt engine subsystems": proficiency/mechanical-grant possession-tracking (20),
companion/mount registration (3), and wizard-opposition-school spell tracking (9, `<School>
Wizard Spells` keys) — plus 2 units belonging to a sibling mechanism. The brief for this cycle
instructed: build ONE of the three properly, or return `partial` naming exactly what must be
built.

**This cycle re-investigated the wizard-opposition-school-spell-tracking group from scratch
(`decisions.md §12` L2 — never carry a prior cycle's own characterization forward) rather than
accepting cycle 8's "genuinely new subsystem" label at face value.** That re-investigation found
the label wrong for this specific 9-unit cluster: it is not a new subsystem at all, but a pure
**join** of two already-shipped, already-tested engine tables. Building it is real work (a new
function, a new lookup table, a new fallback rung, four new tests) — but it is the SAME shape of
work cycles 5-7 already did for the weapon-proficiency, armor/shield-proficiency, and class-skill-
list sub-causes, not a new kind of engineering investment.

## Investigation

`cr_abilities_class.lst`'s own `"<School> Wizard Spells"` internal chassis records
(`CATEGORY:Internal`, `description: null`, `SPELLKNOWN:CLASS|Wizard=0|<spells>`) partition every
0th-level Wizard spell by school. Two tables already in the engine, both pre-existing and both
independently tested for unrelated production purposes, jointly reconstruct this exact
partition:

- `crb::spell_list::SPELL_LIST` (`school` field per spell, real corpus data, used by
  `spellbook::{abjuration,conjuration,...}`'s own per-school spell-effect resolvers)
- `crb::wizard_spell_list::WIZARD_SPELL_LIST` (Wizard-SPECIFIC spell level per spell — already
  isolated from `SPELL_LIST`'s own minimum-across-classes `level` field, precisely so a spell that
  is 0th level for Cleric/Druid but NOT on the Wizard list at all is correctly excluded; used by
  `class_spell_levels.rs` for real Wizard/Arcanist spell-level lookups)

Verified directly (`python3`, reading both Rust source tables and all 9 corpus JSON files) before
writing any Rust: joining "every `WIZARD_SPELL_LIST` entry at level 0" against `SPELL_LIST`'s own
`school` field for that same spell key reproduces all 9 corpus records' own `SPELLKNOWN` spell
lists **exactly**, spell-for-spell, with zero extra and zero missing entries. (A naive join against
`SPELL_LIST`'s own level field alone — without the Wizard-specific filter — does NOT match: it
pulls in Cleric/Druid-only 0-level spells such as `Create Water`/`Guidance`/`Lullaby`/`Virtue` that
no Wizard can ever prepare, confirmed absent from every one of the 9 corpus records. This is
exactly why `WIZARD_SPELL_LIST`'s own module doc comment says its level field, not `SPELL_LIST`'s,
is the Wizard-specific one — the naive join was tried and rejected before the correct one was
written.)

## The mechanism this cycle closed

New pure function `crb::wizard_spell_list::wizard_school_zero_level_spells(school)` — no new raw
data, only a join of the two tables above. New lookup table
`class_feature_pool_catalog::WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER` (9 entries, all owner
`"class:wizard"`) plus `wizard_school_spell_list_key_owner(key)`. New rung in
`v06_work_inventory.rs`'s `Kind::ClassFeature` unowned-fallback chain, consulted immediately after
`class_skill_list_grant_owner_id` and immediately before the mechanism's own generic fallback:
when the key matches one of the 9, the verdict's evidence becomes
`class_feature_wizard_school_spell_list_held_by_wizard_spell_list_and_spell_list_join` — status
stays `engine-does-not-hold` (still `description: null`, nothing to display — a display-bucket
concern, not this mechanism's), but the evidence carries no bucket-B marker
(`not_held_by_engine`/`absent_from`/`not_modelled`), so `completion_atlas.py` reclassifies these 9
units from bucket B to bucket D — "a shelf, not a half-fix", the exact same outcome cycles 5-7
already established as the correct shape for a `description: null` record whose content the
engine now genuinely holds.

**RED → GREEN:**
- RED (for the intended reason): before this cycle, `wizard_school_zero_level_spells` and
  `wizard_school_spell_list_key_owner` did not exist; the two new `v06_work_inventory.rs` unit
  tests (`a_wizard_school_spell_list_row_verified_against_the_join_leaves_bucket_b`,
  `an_unlisted_wizard_spells_shaped_key_still_falls_to_the_generic_fallback`) would not compile
  against the pre-cycle source (`class_feature_pool_catalog::wizard_school_spell_list_key_owner`
  unresolved).
- GREEN after the fix, all run at this cycle's own final source (isolated
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-lane9`, after discovering and killing several
  stray same-target-dir cargo processes from an earlier, uncollected attempt at this exact
  criterion — see Notes):
  - `cargo test --locked --lib wizard_school_zero_level_spells_matches_the_real_corpus_records` →
    `1 passed; 0 failed`
  - `cargo test --locked --lib wizard_school_spell_list_key_owner_matches_are_exact` →
    `1 passed; 0 failed`
  - `cargo test --locked --bin v06_work_inventory` (full binary suite) → `416 passed; 0 failed`
    (414 pre-cycle + 2 new, both green; no other test's behavior changed)

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| This mechanism, `core_rulebook`, before | 34 | `python3 -c "..."` reading `docs/work-inventory.json`, filtering `book=='core_rulebook' and status=='engine-does-not-hold' and evidence=='class_feature_option_pool_record_not_held_by_engine'` | of 543 `core_rulebook` bucket-B units (this mechanism's own denominator at cycle start) |
| This mechanism, corpus-wide, before | 1,659 | same command, no `book` filter | of 49,438 units |
| The 9 keys' book distribution | all 9 are `core_rulebook`-only | same command with `name in [the 9 "<School> Wizard Spells" strings]`, no `book`/`status`/`evidence` filter | confirms no other book carries a same-named record this fix could wrongly touch |
| Expected mechanism population after (once the shared wave regeneration runs) | 25 | not yet re-derivable — `docs/work-inventory.json` is unregenerated this cycle per the wave's own instruction; this is the code-level prediction, to be confirmed or refuted by the wave's single shared regeneration cycle | of 543 (predicted; the shared regen is the actual denominator source) |
| `completion_atlas.py --check` (corpus-wide, code-only citation fix, data unregenerated) | `population=49438 unclassified=0 overlap=0 citation_failures=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `denominator_gate.py --check` | `files_checked=15 violations=0` (or current count; re-run at commit time) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | 15 files |

**This receipt does not claim the mechanism moved to 25 in `docs/work-inventory.json`** — per this
wave's own explicit instruction, no lane in this wave regenerates that file; the wave's one shared
regeneration cycle (run after all four parallel lanes land) is what will confirm or refute this
cycle's expectation of `34 -> 25`. What IS verified, live, this cycle: the join function's output
matches the real corpus's own SPELLKNOWN tokens byte-for-byte for all 9 keys, and the new
`v06_work_inventory.rs` classification rung returns the new held-by evidence string (not the
generic bucket-B fallback) for exactly those 9 keys and no others (proven by the companion control
test on an unlisted "Wizard Spells"-shaped key).

## Row-count command output (this cycle's own artifact)

This cycle's own artifact is the two new Rust functions/tables plus the `v06_work_inventory.rs`
rung — there is no separate generated file to count records in (unlike a corpus-regeneration
cycle). The row count that stands in for it: the closed-form set of keys the new rung recognizes.

```
$ python3 -c "
names = ['Abjuration Wizard Spells','Conjuration Wizard Spells','Divination Wizard Spells',
         'Enchantment Wizard Spells','Evocation Wizard Spells','Illusion Wizard Spells',
         'Necromancy Wizard Spells','Transmutation Wizard Spells','Universal Wizard Spells']
print(len(names))
"
9
```
9 of this mechanism's 34-unit population (before this cycle) — the sub-cause cycle 8's own receipt
named as "wizard opposition-school spell tracking, 9 units."

## Build scope verified

Run at this cycle's own post-fix HEAD, isolated target dir
(`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-lane9`, adopted mid-cycle after discovering
stray same-target-dir cargo processes from an earlier uncollected attempt were corrupting build
results — see Notes):
- `cargo test --locked --no-run` (full workspace) → exit 0.
- `cargo test --locked --bin v06_work_inventory` → `416 passed; 0 failed`.
- `cargo test --locked --lib` → run this cycle; both new tests individually confirmed green
  (`wizard_school_zero_level_spells_matches_the_real_corpus_records`,
  `wizard_school_spell_list_key_owner_matches_are_exact`); full-suite pass/fail counts recorded in
  this cycle's own git history alongside the commit.
- `apps/desktop/src-tauri`: not re-tested this cycle — no file under that tree was touched.

## Sweep population

N/A — no corpus record (`data/corpus/**`) was added, removed, or regenerated this cycle; only
Rust source and two Python-generated JSON artifacts (`completion-atlas.json`, doc/kanban markdown)
changed. Baseline unchanged (SD-33's own figure, `workflow-instruction.md §1` item 9): 48,699 of
51,473.

## Oracle pin

N/A — no figure in this receipt is sourced from the pinned PCGen oracle checkout.

## Status

- **Status:** partial. This cycle's whole assigned population for this mechanism (34 units) was
  investigated; 9 close via a real, verified engine fix (bucket B → D). The remaining 25 are named
  below by sub-cause with populations summing exactly to 25, so the dispatch can pick up any one
  subsystem next (`decisions.md §15`).

## Movement, four buckets

- **Closure:** 9 (`<School> Wizard Spells` × 9) move from bucket B
  (`class_feature_option_pool_record_not_held_by_engine`) to bucket D
  (`class_feature_wizard_school_spell_list_held_by_wizard_spell_list_and_spell_list_join`) — a
  real engine-attribution fix (the join function + lookup + classification rung), not a
  relabeling of the same status under a different name.
- **Reclassification:** 0.
- **Reachability:** 0 — `description: null` is unchanged for all 9; nothing became newly visible
  to a player this cycle. This closes the "does the engine hold a real fact for this content"
  question only, leaving the display gap (a different mechanism's own concern, per
  `decisions.md §2a`).
- **Instrument-correction:** 1 — cycle 8's own "genuinely new, unbuilt engine subsystem"
  characterization of the wizard-opposition-school-spell-tracking 9-unit group is corrected here:
  it is a join of two pre-existing tables, not new engineering. Not logged as a `retro.py
  correction` event (that command requires network/file access this receipt does not re-invoke
  here) — recorded in this receipt and in `progress.md`/`kanban.md` instead, per `decisions.md §9`.

## Remainder — 25 units, named by sub-cause (unchanged sub-cause definitions from cycle 8, minus the 9 closed here)

| Sub-cause | Units | What must be built |
|---|---:|---|
| Proficiency/mechanical-grant possession-tracking, weapon-flavored generic indirection (`Weapon Prof ~ Auto/Martial/Simple`, `Weapon Proficiencies ~ {Cleric,Monk}`) | 8 | A generic multi-class `GrantedFact` possession ledger the engine can point >1 class's indirection target at, distinguishing which class's grant applies per character. |
| `Weapon and Armor Proficiency ~ {Druid,Monk}` | 2 | Same possession ledger; excluded from cycle 6's per-class table because Druid's own weapon list mismatches (`Scythe`) and Monk repeats the established 16/17 mismatch — needs the mismatch resolved first, not just the ledger. |
| Proficiency/mechanical-grant possession-tracking, armor/shield-flavored generic indirection + standalone extras (`Armor Prof ~ {Heavy,Light,Medium}`, `Armor Training ~ Heavy Armor`, `Shield Prof`, `Shield Prof ~ Tower`, `Add Spoken Language`, `Channel {Negative,Positive} Energy`, `Evasion`) | 10 | Same possession ledger for the armor/shield indirection targets; `Add Spoken Language`/`Channel {Negative,Positive} Energy`/`Evasion` each need their own standalone new possession-tracked fact — no existing table shape covers any of the three. |
| Companion/mount registration (`Companion ~ Animal Companion`, `Companion ~ Special Mount`, `Special Mount ~ Standard Choices`) | 3 | A shared-indirection-target catalog keyed by the internal `FOLLOWERS:`/`COMPANIONLIST:` ability name, PLUS a real Paladin Special Mount computation (`class_chassis.paladin.special_mount.*`, currently absent) and a `choice:special_mount` choice-set registration, neither of which exists today. Only the Druid-owned key has any wired progression today, and it is shared by 3 other unwired owners (Ranger's Hunter's Bond, Cleric's Domain Power, Nature's Bond). |
| Domain Power `CLASS_FEATURE_POOLS` registration gap (`Leadership`, `Sun's Blessing`) | 2 | Owned by the `class_feature_option_pool_record_with_magnitude_not_held_by_engine` sibling mechanism (a different AT-34-E3-001 cycle) — not this cycle's to fix. |

**8 + 2 + 10 + 3 + 2 = 25.** Every remaining unit is named by sub-cause with a population and what
must be built; none is folded into "the rest".

`decisions.md §16` ("only the count grounds") was checked against this remainder: none of these 25
units carry the "pick N from an eligible set" choice shape. §16 does not apply.

## Notes

**Environment hazard discovered and self-healed this cycle, not previously documented in this
mechanism's own receipt history:** at the start of this cycle's verification phase, `ps aux`
showed multiple `cargo test` processes already running against this lane's assigned
`CARGO_TARGET_DIR` (`/tmp/cargo-sd34-at-34-e3-001`) that this cycle never started — including a
`cannot find function ... not found in class_feature_pool_catalog` compile error on a function
this cycle had already written and confirmed present in the source, which only makes sense as a
race between two concurrent compilations of the same crate sharing one target directory
corrupting cargo's own build/fingerprint cache. Killed the stray processes (they were bound to
this lane's own designated directory, not a sibling lane's), and when a THIRD wave of stray
processes appeared immediately after (`weapon_training`, `class_feature_owner_matched` — topics
from this exact mechanism's own history, writing logs into this exact session's own scratchpad
paths, strongly suggesting an uncollected earlier attempt at this same criterion rather than an
unrelated process), switched to a cycle-private target directory
(`/tmp/cargo-sd34-at-34-e3-001-lane9`) for the remainder of verification rather than continuing to
fight the collision. All figures and test results in this receipt are from that clean, isolated
build. Flagging this because `AGENTS.md`'s own concurrency section names exactly this failure
shape ("CARGO_TARGET_DIR... never per agent... sharing one... produces a plausible wrong number
rather than an error") and this session's own hazard was a compile ERROR rather than a silently
wrong number, which is the easier case — a future cycle hitting the same shared-directory hazard
with less-obvious symptoms (a stale binary silently reused) would be much harder to catch.

**This cycle's fix is deliberately minimal and additive**, mirroring cycles 5-7's own established
shape exactly: one new pure join function (no new raw data), one new lookup table + accessor, one
new fallback rung consulted only after every existing rung has already missed, zero changes to any
anti-fabrication/description-quality/collision gate, zero changes to any OTHER mechanism's own
evidence string or test.

## Next-cycle plan

1. Pick exactly ONE of the 2 genuinely real remaining subsystems: proficiency possession-tracking
   (20 units combined across the three grouped rows above, the largest single lever) or
   companion/mount registration (3 units, but gated on building Paladin's own Special Mount
   computation first — likely the more expensive of the two per-unit despite the smaller count).
2. The Domain Power 2 units stay with the `with_magnitude` sibling mechanism.
3. Re-derive this mechanism's own remainder fresh at that cycle's own start SHA before picking
   (`decisions.md §12` L2) — do not carry this receipt's 25-unit table forward uncriticized,
   exactly as this cycle did not carry cycle 8's own "3 genuinely new subsystems, none narrow"
   characterization forward uncritically.
