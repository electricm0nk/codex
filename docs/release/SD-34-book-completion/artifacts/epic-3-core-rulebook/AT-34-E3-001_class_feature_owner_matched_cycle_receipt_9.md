# Cycle 9 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism)

This cycle owns **exactly one** of the nine mechanisms `decisions.md §14` decomposed
`AT-34-E3-001` into, and within it, **only the non-excluded-class remainder** — a sibling
lane owns the excluded-class majority (`decisions.md §18`'s operator ruling on
`ANTI_FABRICATION_GATE_EXCLUDED_CLASSES`). It does **not** close AT-34-E3-001 itself.

**Numbered cycle 9, not 8:** a sibling lane, working the SAME wave on the excluded-class
sub-cause of this SAME mechanism, independently numbered its own cycle "8" and pushed
`AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md` first (`bfe90f020a` /
`97139fd2bd`). This receipt was originally authored under that same filename before the
collision was discovered on rebase; renumbered to 9 and renamed rather than overwriting the
sibling's landed file.

- **Commit SHA:** this cycle's own two-file diff (this receipt plus the
  `class_feature_pool_catalog.rs` test) was verified compiling/passing at `618079d0fb`; the
  final pushed SHA is named in the push result, landed past a further concurrently-pushed
  sibling commit (`534c9c2a61`, the `class_feature_option_pool_record_with_magnitude_not_
  held_by_engine` mechanism's own cycle 9 — a DIFFERENT mechanism, untouched by this receipt).
  No further code change followed the `618079d0fb` test verification; only this receipt's own
  content (see Notes) and rebasing past additional concurrent landings occurred afterward.
- **Files touched:**
  - `src/rules_core/class_feature_pool_catalog.rs` — new test
    `class_feature_owner_matched_non_excluded_remainder_is_24_and_named_by_subcause`, a
    characterization test proving this cycle's own re-derived split by direct corpus query,
    using a frozen local copy of the original seven-class exclusion literal (a live import of
    `class_feature_grant_consumer`'s own constant became impossible mid-cycle — see below).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_9.md`
    (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/work-inventory.json` — **not touched this cycle** (wave rule: no regeneration this
    wave; a single shared regeneration cycle runs after all four parallel lanes land).
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — **NOT touched by this
    cycle's own final diff.** This cycle initially widened
    `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` from private to `pub(crate)` so the test above
    could import it live; the sibling `§18` fix landed first and renamed/repurposed that same
    constant to `LEVEL_UP_PILLAR_FILTERED_CLASSES` (2 classes: Druid, Monk — anti-fabrication
    for the other five is now enforced by corpus-citation, not an allowlist). Resolving the
    rebase conflict kept the sibling's version entire; this cycle's own visibility widening
    became moot and was dropped.
- **Identifier audit result:** the cumulative epic-3 file-touch-set diff from `origin/develop`
  (`ea2b3396f2`) carries 20 `sd13_*`/`sd25_*` matches, all inside the sibling `§18` commit
  (`bfe90f020a`), naming real, pre-existing acceptance test files (not fabricated bundle
  tags). This cycle's own isolated diff (`git diff --unified=0 bfe90f020a...HEAD --
  src/rules_core/class_feature_pool_catalog.rs`) → **0 matches, OK_NO_BUNDLE_TAGS**.
- **Wired-integration audit result:** the same cumulative diff carries 17 stub-token-shaped
  matches, all pre-existing (the same population cycle 7's own receipt already characterized
  as real corpus-terminology, none a code stub). This cycle's own isolated diff → **0
  matches, OK_NO_TOKENS**.

## Re-derived split — the dispatch's own inherited number did NOT match

The dispatch brief for this cycle stated a sibling lane owns "the 161 blocked by
`ANTI_FABRICATION_GATE_EXCLUDED_CLASSES`" and this lane owns "the ~81 NOT gated by that
list", and explicitly instructed re-deriving rather than trusting that split. Re-derived
directly against the live corpus and `docs/work-inventory.json`:

```bash
python3 -c "
import json, glob
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
key_to_rec = {}
for path in glob.glob('data/corpus/core_rulebook/class_feature/**/*.json', recursive=True):
    rec = json.load(open(path))
    k = rec.get('data', {}).get('key')
    if k: key_to_rec.setdefault(k, []).append(rec)
EXCLUDED = {'wizard','bard','paladin','cleric','sorcerer','druid','monk'}
excl = non_excl = 0
for u in cr:
    owner = (key_to_rec[u['corpus_key']][0]['data'].get('class') or '').lower()
    if owner in EXCLUDED: excl += 1
    else: non_excl += 1
print('total', len(cr), 'excluded', excl, 'non_excluded', non_excl)
"
```
→ **total 242, excluded 218, non-excluded 24** — not 161/81. `decisions.md §18` (an operator
ruling logged the same day this cycle ran) independently states **"218 of 242"** for the exact
same population, and the sibling `§18`-fix cycle's own receipt independently re-derived the
same 218 by grouping against `data/class_feature_grants/`. Three independent derivations agree
on 218/24, none agree with the dispatch's stale 161/81. Reported as an
**instrument-correction**, not a finding this cycle takes credit for closing.

## What the 24 non-excluded units actually are

Per-record inspection (raw corpus JSON, not a proxy) of all 24, split by whether a real
`description` exists:

- **18** carry `description: null` — the zero-description internal-bookkeeping shape
  `atlas-defects.md` entries 1–3 already name as the bundle's OPEN definitional question
  (is a corpus record with no content at all ever `held`?). Per this cycle's explicit
  instruction: **left in bucket B, not reclassified into X or U on this cycle's own
  authority.** Examples: `Archetype {Barbarian,Fighter,Ranger,Rogue}`, `Barbarian ~ Standard
  Class{,  Full, Ex-Class}`, `Barbarian/Rogue ~ Uncanny Dodge Tracker`, `Assassin ~ Hide in
  Plain Sight`, `Shadowdancer ~ {Evasion, Improved Evasion, Slippery Mind}`, `Dragon Disciple ~
  Draconic Bloodline`, `Expert Class Skills`, `Pathfinder Chronicler ~ Bardic Knowledge`.
- **6** carry a REAL corpus description, but every one of them is correctly refused by an
  existing, independently-tested safety gate in `class_feature_pool_catalog.rs`
  (`load_class_feature_catalog`'s render-and-refuse / engine-effect-token / class-level-phrase
  gates — each already has its own dedicated live-corpus regression test in that module,
  cited by this cycle's new test's doc comment):
  - `Rogue Talent ~ Bleeding Attack` — description carries `%1|SneakAttackDice`, a real
    per-character formula this catalog cannot resolve (`dropped_pcgen_args`).
  - `Rage Power ~ Knockback` — description carries a bare `%1` with no pipe tail
    (`bare_percent_reference`).
  - `Arcane Trickster ~ Invisible Thief` — description states "her arcane trickster level",
    a class-level-scaled value (`class_specific_level_phrase`).
  - `Rogue Talent ~ Finesse Rogue` — carries `ABILITY:FEAT|VIRTUAL|Weapon Finesse`, a genuine
    mechanical grant token (`engine_effect_token_present`).
  - `Rogue Talent ~ Improved Evasion` — carries `ABILITY:Special Ability|AUTOMATIC|Improved
    Evasion` (`engine_effect_token_present`).
  - `Rogue Talent ~ Skill Mastery` — carries `SELECT:3+INT`/`CHOOSE:SKILL|ALL`, a real
    per-character choice (`engine_effect_token_present`).

  None of these 6 is a narrow catalog-widening bug: each needs real per-character grant or
  formula wiring (a Rogue Talent pick actually consumed by `pilot_compute`, a
  sneak-attack-dice-scaled damage formula, a virtual-feat grant on talent selection) — a real
  engineering task this single-mechanism cycle's narrow scope cannot safely improvise without
  risking exactly the kind of fabrication `class_feature_pool_catalog`'s own safety gates exist
  to prevent.

**18 + 6 = 24**, sums exactly to this cycle's owned population.

## TDD — RED confirmed for the intended reason, then GREEN

```bash
# RED: asserted the dispatch's own stale claim (161) instead of the re-derived figure (218)
cargo test --locked --lib class_feature_owner_matched_non_excluded_remainder_is_24
```
→ FAILED: `assertion `left == right` failed: excluded-class population (sibling lane's, do not
touch)\n  left: 218\n right: 161` — fails for the intended reason (the dispatch's number is
wrong, not a typo in the test or an unrelated compile error).

```bash
# GREEN: corrected to the re-derived figure
cargo test --locked --lib class_feature_owner_matched_non_excluded_remainder_is_24
```
→ passed.

```
$ cargo test --locked --lib class_feature_pool_catalog::tests::
test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 2891 filtered out; finished in 44.42s
```
Run after the final rebase, including cycle 9's own concurrently-landed
`wizard_school_spell_list_key_owner_matches_are_exact` and the sibling `§18` fix's changes to
`class_feature_grant_consumer.rs` — all 40 tests in the module pass together, including
cycle 7's own pre-existing `class_feature_owner_matched_but_not_held_346_sub_causes_are_
named_and_sum_exactly`, unaffected by any of this wave's concurrent landings (it reads
`docs/work-inventory.json` and the live corpus directly, not this cycle's or the sibling's
production code).

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 242 (unchanged this cycle). Command: see
  "Re-derived split" above; denominator: `core_rulebook` units with
  `status=='engine-does-not-hold'` and this evidence string.
- **This lane's owned population:** 24 (18 null-description + 6 real-description-but-gated),
  re-derived above; denominator: the 242-unit mechanism population, owner NOT in the
  original seven-class exclusion set.
- **Sibling lane's population:** 218 (excluded-class), not touched this cycle — see the
  sibling's own `AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md` for its
  disposition (the `§18` citation-based widening).
- **`completion_atlas.py --check` (population-wide):** not re-run this cycle —
  `docs/work-inventory.json` is untouched (wave rule), so this figure is unchanged from the
  pre-wave baseline.
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → pre-existing `violations=5`, all inside
  `progress.md`, verbatim-quoting corpus prose ("75% chance…"), already flagged by the
  already-merged `AT-34-E3-004` cycle. This cycle adds no new `.md` prose containing a bare
  percentage (its own new prose is this receipt file, which contains none).

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
242
```
This cycle's own artifact (this receipt plus the new characterization test) moved **zero**
units of status in `docs/work-inventory.json` — the mechanism's `core_rulebook` population
remains 242 (218 excluded-class, owned by the sibling lane; 24 owned by this lane, all 24
accounted for and none closable this cycle without new grant/formula engine wiring or new
ingest work, per the table above).

## Build scope verified

- `cargo test --locked --lib class_feature_pool_catalog::tests::` (the whole module) → `40
  passed; 0 failed`, run at commit `618079d0fb`, post-rebase past every SAME-wave concurrent
  landing on this shared file.
- `cargo test --locked --no-run` (full workspace) → exit 0, run at the same commit.
- `apps/desktop/src-tauri`: not touched this cycle's own final diff (only
  `src/rules_core/class_feature_pool_catalog.rs` and this receipt) — not re-run, matching
  `workflow-instruction.md §2.5`'s scoping rule.
- Run **after** the last commit in this cycle that can move a figure this receipt depends on
  (`decisions.md §12` L7) — the rebase's conflict resolutions are merges, not new
  figure-moving edits; both the RED→GREEN proof and the 40-test module rerun above post-date
  the final rebase.

## Sweep population

N/A — this cycle added or regenerated zero corpus records (`data/corpus/` untouched) and did
not regenerate `docs/work-inventory.json` (wave rule). `corpus_literal_sweep`'s
examined-population is therefore unchanged from the pre-wave baseline — not re-run or
re-asserted here since this cycle did not touch the corpus.

## Oracle pin

N/A — no figure in this receipt is derived from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** the dispatch's own inherited 161/81 split for this mechanism's
  excluded/non-excluded population is corrected to 218/24, matching both `decisions.md §18`
  and the sibling `§18`-fix cycle's own independent re-derivation for the same population.
  This does not change any unit's status in `docs/work-inventory.json` — it corrects a claim
  about the population's composition, not the population's count (242 is unchanged and was
  already correct).

## Remainder — every unit named by sub-cause, populations sum exactly to 24

| Sub-cause | Population | Disposition this cycle |
|---|---:|---|
| `description: null`, no real corpus content of any kind (zero-description internal bookkeeping) | 18 | **Left in bucket B.** The bundle's OPEN definitional question (`atlas-defects.md` entries 1–3) — not this cycle's to decide, and not reclassified into X or U on this cycle's own authority. |
| Real description, but correctly refused by `class_feature_pool_catalog`'s render-and-refuse / class-level-phrase / engine-effect-token gates (each already independently tested against the live corpus) | 6 | **Left in bucket B.** Genuinely needs real per-character grant/formula wiring (talent-pick consumption, sneak-attack-dice-scaled formula, virtual-feat grant on selection) — a real engineering task, not a narrow catalog-widening or classify()-gate bug this cycle can close without risking exactly the kind of fabrication `class_feature_pool_catalog`'s own safety gates exist to prevent. |

`18 + 6 = 24` — both rows corpus-verified per-record (not estimated), and pinned by this
cycle's own new characterization test, which fails if any of the 24 is later found to already
pass every safety gate (i.e., if a future catalog change makes one of the 6 servable, or a
future ingest pass gives one of the 18 a real description, the test's `real_desc_unrefused_
unexpected` assertion or its fixed counts will catch it and force re-investigation rather than
silently drift stale).

**For the next cycle (or a future ingest/wiring cycle):**
1. The 6 real-description units each need one of: (a) a per-character Rogue Talent /
   Rage Power selection-and-grant mechanism inside `pilot_compute` (not this catalog — the
   catalog is deliberately character-independent), or (b) a `SneakAttackDice`-style
   formula resolver wired into the description-render path for character-specific magnitude
   substitution. Both are real engineering, not this single-mechanism cycle's scope.
2. The 18 null-description units remain gated on the bundle's open definitional-question
   ruling (`atlas-defects.md`) — do not act on them without that ruling.
3. This mechanism (`class_feature_owner_matched_by_name_but_record_not_held_by_engine`)
   therefore has **zero further non-excluded work available** until either (1) or (2) above is
   resolved by a differently-scoped cycle. The sibling's 218-unit excluded-class work is
   tracked in its own receipt (`AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md`).

## Notes

- This cycle's own contribution is a correction and a verification artifact, not a movement —
  legitimate per `decisions.md §9` ("a measurement wave that banks zero units is a legitimate
  deliverable... judged on whether the denominator it produces is true, not on movement").
- **Concurrent-write hazard hit and self-corrected this cycle:** a sibling lane, working the
  SAME wave, independently numbered its own cycle for THIS SAME mechanism "8" and pushed
  `AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md` before this cycle's first
  push attempt. Resolving the resulting rebase conflicts with `git checkout --ours` at every
  step (intending to keep this cycle's own content) instead discarded the sibling's landed
  receipt — a `git rebase`'s "ours"/"theirs" polarity is the OPPOSITE of a merge's (during a
  rebase, "ours" is the upstream commit already in place, "theirs" is the commit being
  replayed), and treating it like a merge deleted the sibling's file. Caught by re-diffing
  `origin/tranche/14...HEAD` before push and finding the sibling's receipt path absent; fixed
  by restoring it verbatim from the sibling's own commit (`97139fd2bd`) and separately
  re-authoring this cycle's own receipt content, which had been silently overwritten by the
  same mistake at the final conflict step. Confirmed by `git diff --stat origin/tranche/14
  HEAD` showing exactly this cycle's own two intended files as pure additions before pushing.
  A further sibling commit (`534c9c2a61`, a DIFFERENT mechanism's own cycle 9,
  `class_feature_option_pool_record_with_magnitude_not_held_by_engine`) landed on the branch
  during this same recovery; `git reset --hard HEAD` was used once, after confirming (`git
  diff HEAD`) that every difference it discarded was an unintended stray revert of files this
  cycle never edited (the sibling's own progress.md/kanban.md/retro-jsonl/engine-code
  entries), never a real edit of this cycle's own.
- Six units with real descriptions were investigated in enough depth to be confident they are
  NOT closable by this cycle's own mechanism (each independently blocked by a different,
  already-tested gate) rather than merely "not attempted" — this is the honest boundary the
  dispatch asked for, not a stopping-short.

## Next-cycle plan

1. A future cycle scoped explicitly to "Rogue Talent / Rage Power per-character grant wiring"
   (a DIFFERENT, larger mechanism than this cycle's own narrow one) could ground the 6
   real-description units named above — but it needs real formula/selection modeling, not a
   text-serving catalog change, and should be sized and dispatched as its own criterion rather
   than folded into this mechanism's remaining scope.
2. The 18 null-description units wait on the bundle's operator ruling on the OPEN
   zero-description definitional question (`atlas-defects.md`).
3. This mechanism's non-excluded population (24) is now fully characterized and pinned by a
   test; no further re-derivation work is needed here until (1) or (2) lands.

- **Status:** partial. This cycle's own owned population (24, non-excluded) has zero units
  available for closure this cycle — both named sub-causes (18 null-description, 6
  gate-refused) are genuinely blocked on work outside this single-mechanism cycle's scope
  (an open definitional ruling, and real grant/formula engine wiring respectively), not on
  anything this cycle declined to do. Per `decisions.md §15`, a cycle that names its whole
  remainder by sub-cause with populations summing exactly reports `partial`, not
  `blocked-escalated` — this is a sequencing/scope report, not a request for an operator
  ruling on either sub-cause (both are already ruled: `atlas-defects.md`'s question is
  explicitly still open and not this cycle's to resolve, and the gate-refused 6 are correctly
  behaving per existing, tested, deliberate design).
