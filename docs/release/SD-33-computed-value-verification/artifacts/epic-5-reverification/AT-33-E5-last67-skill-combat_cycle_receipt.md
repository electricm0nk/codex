# Cycle AT-33-E5-last67-skill-combat — Epic 5 Re-verification / AT-33-E5-002 (SKILL + COMBAT token-family lane, 23 of the 67-unit residual)

- **Commit SHA:** recorded on landing (see `progress.md` entry `AT-33-E5-last67-skill-combat`)
- **Files touched:**
  - `src/rules_core/corpus_loader.rs` — real engine fix, RED→GREEN (see below)
  - `scripts/oracle_harness/campaign_key.py` (new) — reusable campaign-KEY-vs-display-name fix
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-skill-combat.oracle-results.json` (new — this lane's committed deliverable, 14 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-last67-skill-combat_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/progress.md` / `kanban.md` (updated in place)
  - `docs/retro/events/sd33-r5-skillcombat.jsonl` (new)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## Population re-derivation (first action, per the brief)

```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss))"
67
```

Matches the brief's stated 67. This lane's own 23-unit slice (SKILL/COMBAT token families,
psionics included) was re-derived by reading every one of the 67 units' real `raw_bonus_chains`
(a fresh `find_combat.py` sweep, not filtered to a pre-supposed subtoken list — the brief's
"6 non-psionics COMBAT" figure was missing `rod_alertness`/`stone_of_good_luck_luckstone` until
this re-derivation found their real corpus paths, e.g.
`data/corpus/core_rulebook/equipment/magic_items/rod_alertness.json`, not the naive
`equipment/<name>.json` guess):

| Shape | Population | Units |
|---|---:|---|
| `SKILL` single-skill (or single-first-chain), `ultimate_psionics` book | 14 | `companion_stone_diplomacy`, `crystal_mask_{detection,discernment,dread,insightful_detection,psionic_craft}`, `eyes_of_expanded_vision`, `meld_stone_{alchemist,inflitrator}`, `psychoactive_skin_{chameleon,nimbleness,spider}`, `ring_self_sufficiency`, `third_eye_aware` |
| `COMBAT`-shape non-psionics (`INITIATIVE`/`TOHIT`/`TOHIT.Ranged`/formula-valued `AC`) | 6 | `rod_alertness` (INITIATIVE), `stone_of_good_luck_luckstone` (INITIATIVE, formula), `gunfighter_s_poncho` (AC, formula), `robe_of_vermin` (INITIATIVE+TOHIT), `scattershot_bracers` (TOHIT.Ranged), `staff_of_the_hierophant` (AC, formula) |
| `COMBAT`-shape, `ultimate_psionics` book | 1 | `companion_stone_far_sight` (TOHIT.RANGED) |
| `ultimate_psionics` dissonance `VAR`+`WEAPON`-formula pair | 2 | `special_quality_dissonance_enhancement_bonus_{alt,main}` |
| **Total** | **23** | matches the brief's stated denominator |

## Finding 1 (root-caused, FIXED, oracle-harness): the real cause of every prior wave's `oracle_harness_ultimate_psionics_campaign_load_failure`

Every prior lane (`AT-33-E5-remainder-equipment`, `AT-33-E5-last75`, `AT-33-E5-shape-combat`)
recorded `ultimate_psionics` items as blocked by `SEVERE Globals:130 Could not find campaign:
Ultimate Psionics`, treated as an unresolved pre-existing harness defect. Root-caused this cycle by
reading PCGen's own campaign-lookup source (`code/src/java/pcgen/core/Globals.java:125-134`,
`getCampaignKeyed`): a `.pcg`'s `CAMPAIGN:<name>` line is matched against each loaded `Campaign`'s
`getKeyName()` — its `KEY:` token when the `.pcc` carries one, **not** its `CAMPAIGN:` display
name. `ultimate_psionics.pcc` carries **both**: `CAMPAIGN:Ultimate Psionics` (line 2) **and**
`KEY:DSP - Ultimate Psionics` (line 3) — a separate internal key every OTHER book's own
`PRECAMPAIGN:...,INCLUDES=DSP - Ultimate Psionics` clause already references (confirmed:
`data/pathfinder/dreamscarred_press/path_of_war/_path_of_war.pcc:59`). Every prior fixture wrote
the display name, which never resolves.

Confirmed live, both directions, same fixture, same direct-`java` runner
(`scripts/oracle_harness/charbuild_remainder_run_one.sh`, unmodified):

```
$ # display name (every prior fixture's convention) -- BROKEN
$ grep CAMPAIGN crystal_mask_psionic_craft.pcg
CAMPAIGN:Core Rulebook
CAMPAIGN:Ultimate Psionics
$ bash charbuild_remainder_run_one.sh crystal_mask_psionic_craft.pcg ...
SEVERE main Globals:130 Could not find campaign: Ultimate Psionics
INFO main BatchExporter:122 Loading sources [Core Rulebook] using game mode Pathfinder_RPG
WARNING main CharacterManager:260 * Could not add equipment: Crystal Mask (Psionic Craft). Check loaded campaigns.
SKILL.MISC=0

$ # real KEY (this cycle's fix) -- WORKS
$ sed 's/CAMPAIGN:Ultimate Psionics/CAMPAIGN:DSP - Ultimate Psionics/' crystal_mask_psionic_craft.pcg > fixed.pcg
$ bash charbuild_remainder_run_one.sh fixed.pcg ...
INFO main BatchExporter:122 Loading sources [Core Rulebook, Ultimate Psionics] using game mode Pathfinder_RPG
SKILL.MISC=10
```

`10` is the corpus's own `BONUS:SKILL|Spellcraft|10|TYPE=Competence` chain, exactly. **Also
confirmed this is NOT a settings-directory caching artifact**: re-ran against a brand-new, never-
used `-s` settings directory both ways — the display-name form fails cold every time, the KEY form
succeeds cold every time. **Also confirmed this is book-specific, not publisher-wide**: `Path of
War` (same `dreamscarred_press/` directory, no `KEY:` divergence) loads fine on its display name
with a fresh settings dir; only `ultimate_psionics`'s own KEY/CAMPAIGN divergence breaks it.

**Fix, committed, reusable:** `scripts/oracle_harness/campaign_key.py` —
`CAMPAIGN_KEY_OVERRIDES = {"Ultimate Psionics": "DSP - Ultimate Psionics"}` plus
`campaign_line_value()`, for every future lane building an `ultimate_psionics` `.pcg` fixture.
Documented in the module's own docstring with the full root-cause chain so the next lane does not
re-discover this from scratch.

## Finding 2 (root-caused, FIXED, engine, RED→GREEN): `equipment_id_resolve` silently failed on every OUTPUTNAME-divergent, KEY-less record

Building this lane's own "ours" probe manifest (`e5_statsave_skill_ours`, reused unmodified)
against all 14 SKILL units found only **2 of 14** resolved (`eyes_of_expanded_vision`,
`third_eye_aware`); the other 12 hit `per_item.first() == None`. Root-caused, not guessed:
`equipment_record_from_json` (`src/rules_core/corpus_loader.rs:239-292`) sets the in-memory
record's `name` field from the ingested JSON's `data.name` — which for a record with an
`OUTPUTNAME:` token but no `KEY:` token (common in `ultimate_psionics`: `Companion Stone
(Diplomacy)` has no `KEY:` line, so its real identity is its own first-column name, but its
`OUTPUTNAME:Companion Stone of [NAME]` gets substituted into `data.name` at ingestion) is a
**display string**, not the record's identity. `equipment_key_token()` only synthesized a `KEY`
token as a fallback when `raw_tokens` was **completely empty**, not merely lacking a literal
`KEY:` entry — so a KEY-less-but-OUTPUTNAME-bearing record's identity silently fell back to the
wrong `.name`, and `equipment_id_resolve` never found it. This is the SAME underlying defect the
`AT-33-E5-shape-combat` lane named narrowly as `engine_id_resolve_fails_templated_variant_record`
for 2 units (`Psychoactive Skin (Defender)`/`(Hero)`) — this fix closes the general case, not just
those two.

**RED→GREEN** (`src/rules_core/corpus_loader.rs`):
- RED: `outputname_divergent_record_still_resolves_by_its_real_key` — real, verbatim on-disk
  record (`ultimate_psionics:equipment:companion_stone_diplomacy`), asserts
  `equipment_id_resolve("Companion Stone (Diplomacy)", ...)` returns the real record with its
  real `BONUS:SKILL|Diplomacy|4|TYPE=Competence` chain. Failed before the fix
  (`panicked at ... "Companion Stone (Diplomacy) must resolve by its real KEY..."`).
- GREEN: changed the synthesis condition from `if tokens.is_empty()` to
  `if !tokens.iter().any(|t| t.key == "KEY")` — always uses the ingestion pipeline's own `data.key`
  field (which is ALWAYS the real corpus identity per `equipment_id_resolve`'s own doc comment,
  confirmed against the ingestion `source.record_key` convention) when no literal `KEY:` token
  survived parsing, not only when the whole token list was empty.
- Test run: `cargo test --locked --lib corpus_loader::` → 6 passed, 0 failed (the new test plus 5
  pre-existing, all green — no regression).
- `equipment_effects::` regression check: `cargo test --locked --lib rules_core::equipment_effects`
  → **70 passed, 0 failed** (this cycle touched only `corpus_loader.rs`, one call-site upstream of
  every `equipment_effects` resolver — confirmed no downstream test broke).

**After the fix:** `e5_statsave_skill_ours` resolves **14 of 14** manifest units (was 2 of 14).

## The 14 examined rows (SKILL, `ultimate_psionics`) — real, live, both sides

`ours` = `general::compute_general_effect`'s real, live-computed `SkillCheckBonus.bonus`
(`e5_statsave_skill_ours`, re-derive: `cargo run --locked --bin e5_statsave_skill_ours -- . <manifest> <out>`
using the manifest embedded in this receipt's git history). `oracle` = the real, live pinned-PCGen
`SKILL.<name>.MISC` export, using each unit's pre-existing `statsave-fixtures/skill-pcg/*.pcg`
fixture (built by an earlier wave, reused unmodified) with the `CAMPAIGN:` line corrected per
Finding 1 (`sed 's/CAMPAIGN:Ultimate Psionics/CAMPAIGN:DSP - Ultimate Psionics/'`, applied at run
time in a scratch copy — the committed fixture files themselves are untouched, per this lane's
write scope).

| unit_id | ours | oracle | verdict |
|---|---:|---:|---|
| `companion_stone_diplomacy` | 4 | 4 | agree |
| `crystal_mask_detection` | 10 | 10 | agree |
| `crystal_mask_discernment` | 10 | 10 | agree |
| `crystal_mask_dread` | 10 | 10 | agree |
| `crystal_mask_insightful_detection` | 9 | 9 | agree |
| `crystal_mask_psionic_craft` | 10 | 10 | agree |
| `eyes_of_expanded_vision` | 1 | 1 | agree |
| `meld_stone_alchemist` | 8 | 8 | agree |
| `meld_stone_inflitrator` | 8 | 8 | agree |
| `psychoactive_skin_chameleon` | 10 | 10 | agree |
| `psychoactive_skin_nimbleness` | 10 | 10 | agree |
| `psychoactive_skin_spider` | 20 | 20 | agree |
| `ring_self_sufficiency` | 10 | 10 | agree |
| `third_eye_aware` | 10 | 10 | agree |

**14 of 14 agree, 0 disagree, 0 unverifiable.**

**Scope note on the 3 multi-chain units** (`crystal_mask_insightful_detection`,
`meld_stone_alchemist`, `meld_stone_inflitrator`): each carries more than one `raw_bonus_chains`
entry (2 SKILL chains, or a mix of SKILL/comma-joined-SKILL/SITUATION chains). Following the
sibling STAT/skill lanes' own already-documented first-named-target convention
(`e5_statsave_skill_ours`'s own module doc comment), this cycle verified the FIRST `SKILL`-type
chain only (`compute_general_effect`'s own `find_map` behavior, unchanged) — `Perception`/
`Craft (Alchemy)`/`Bluff` respectively. The additional chains on these 3 units (a second
single-skill `SKILL` chain, or a comma-joined multi-skill `SKILL` chain, or `SITUATION`-shape
chains) were **not independently verified this cycle** — named here rather than silently folded
into "done", per the same doctrine `AT-33-E5-last75`'s own `meld_stone_alchemist`-adjacent
comma-joined-chain finding already established (`no single PCGen token` for a comma-joined name).

Re-derive:
```
$ python3 -c "import json,collections
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-skill-combat.oracle-results.json'))['results']
print('rows', len(d)); print(collections.Counter(x['verdict'] for x in d))
print('reasonless', len([x for x in d if x['verdict']=='unverifiable' and not (x.get('reason') or '').strip()]))
ids=[x['unit_id'] for x in d]; print('dupes', len(ids)-len(set(ids)))"
rows 14
Counter({'agree': 14})
reasonless 0
dupes 0
```

## The 9 NOT examined this cycle — concrete, per-shape, not "ran out of time"

### `COMBAT`-family (7: 6 non-psionics + 1 psionics)

Investigated live this cycle, not deferred blind. Two real, distinct, still-open blockers found:

1. **No engine resolver exists for `COMBAT|INITIATIVE`/`COMBAT|TOHIT`/`COMBAT|TOHIT.Ranged` at
   all** — confirmed by reading `src/rules_core/equipment_effects/arms_armor.rs`:
   `armor_class_bonus_from_bonus_chains` only ever matches `qualifiers[1] == "AC"`. This is the
   SAME gap `AT-33-E5-shape-combat`'s own receipt already named ("no engine resolver exists for
   any bare non-AC COMBAT subtoken at all ... needs its own dedicated cycle") — confirmed still
   true, not re-fixed this cycle (out of this lane's reachable-this-turn budget on top of Findings
   1-2 above).
2. **A second, NEW, deeper blocker found this cycle on the oracle side, independent of (1):**
   even where a comparable PCGen export token exists (`INITIATIVEMOD`/`INITIATIVEMISC`, real
   tokens confirmed by reading `plugin.exporttokens.{InitiativeModToken,InitiativeMiscToken}.java`
   directly), a live round-trip against `rod_alertness`
   (`BONUS:COMBAT|INITIATIVE|1|TYPE=Insight`, no `PRE`-gate on the raw LST line) shows the
   equipment-granted Insight initiative bonus is **not reflected in either export token at all**:
   baseline (no item) `INITIATIVEMOD=+2`/`INITIATIVEMISC=+0`; with `Rod (Alertness)` equipped
   (confirmed equipped — no "Could not add equipment" warning in the run log)
   `INITIATIVEMOD=+2`/`INITIATIVEMISC=+0`, unchanged. This is a genuinely new finding — not
   previously named by any prior wave, because no prior wave got far enough to try a real
   INITIATIVE round-trip. Not root-caused further this cycle (candidate causes not yet
   distinguished: a PF1 gamemode formula that reads a different bonus category than
   `COMBAT|INITIATIVE` for its `Initiative` variable, vs. a `CControl.INITIATIVE`/`INITIATIVEBONUS`
   control-variable wiring gap in the pinned build).
3. 3 of the 7 are additionally FORMULA-valued (`2+Global_LuckBonus`/`1+Global_LuckBonus`):
   `gunfighter_s_poncho`, `staff_of_the_hierophant` (AC), `stone_of_good_luck_luckstone`
   (INITIATIVE) — our engine's `qualifiers[2].parse::<i16>()` fails closed (returns `None`, not a
   fabricated 0) on any non-literal formula string; no formula evaluator exists in
   `arms_armor.rs` for this shape. `Global_LuckBonus` itself is architecturally 0 for an isolated
   single-item test character (confirmed: it is read, never `DEFINE`'d to a nonzero default,
   anywhere in the pinned corpus — `grep -rn "Global_LuckBonus"` finds only consuming `BONUS:`
   chains, no `DEFINE:Global_LuckBonus|<n>`), so the formula VALUE is knowable, but implementing a
   general (even narrowly-scoped) formula evaluator is real new engine surface, correctly out of
   this lane's remaining budget on top of blocker (1)-(2) above.

None of the 7 were forced through a partially-working pipeline to bank a row. `robe_of_vermin` and
`scattershot_bracers`/`companion_stone_far_sight` (`TOHIT`/`TOHIT.Ranged`, literal-valued) are
blocked by (1)+(2) identically to the `INITIATIVE` units — the same missing resolver, and (per (2)
above, confirmed on the one subtoken this cycle actually round-tripped) no confidence the export
side would even reflect the bonus if the resolver existed.

### `ultimate_psionics` dissonance `VAR`+`WEAPON`-formula pair (2)

`special_quality_dissonance_enhancement_bonus_{alt,main}` — `DEFINE:DissonanceEnhancementBonus{Alt,Main}|0`
plus `BONUS:VAR|...|1` (sets the named variable to 1) plus
`BONUS:WEAPON|DAMAGE,TOHIT|DissonanceEnhancementBonus{Alt,Main}|TYPE=ENHANCEMENT` (uppercase
`TYPE=ENHANCEMENT`, applies the variable's value to a wielded weapon). Two compounding blockers,
both real, both already-named by a prior wave and confirmed still open, not re-investigated deeper
this cycle: (a) `equipmods::compute_equipmods_effect`'s literal string match on
`qualifiers[3] == "TYPE=Enhancement"` is case-sensitive and would not match `TYPE=ENHANCEMENT` even
if reached (`AT-33-E5-last75_cycle_receipt.md`, Finding 4); (b) these are `equipment_modifier`
records (`category: Equipmods`, `VISIBLE:NO`), meant to be attached to a base weapon via `EQMOD:`,
not equipped standalone — no `.pcg` plumbing for a modifier-attached-to-a-base-weapon fixture was
built this cycle (the existing `statsave-fixtures/skill-pcg/` convention is equip-alone-only). Not
attempted this cycle; named for the next cycle with the concrete next step below.

## Verdict discipline

No `disagree` recorded — the 14 examined agree exactly. No `unverifiable` recorded — every one of
the 9 not-examined units genuinely was not reached with a comparable value (an engine resolver gap
and/or an oracle export-token gap), so leaving them **unexamined** (not writing a guessed or
premature verdict) is the correct disposition per `AT-33-E5-last75`'s own established precedent
("mattock_of_the_titans's MAGICHIT mismatch ... withheld from both agree and disagree").

## Status: blocked-escalated

**Not `complete`.** 14 of this lane's 23-unit population are genuinely examined with real,
live-both-sides `(ours, oracle, verdict)` rows (all 14 agree). The remaining 9 are named per-shape
above with two concrete, distinct, real blockers (a missing engine resolver for non-AC `COMBAT`
subtokens, confirmed still open from `AT-33-E5-shape-combat`; a NEW, independently-confirmed
oracle export-token gap for at least the `INITIATIVE` subtoken; a formula evaluator gap for 3
formula-valued units; a base-weapon+`EQMOD`-attachment fixture gap plus a case-sensitivity bug for
the dissonance pair) and a concrete next-cycle plan — not "ran out of time" vaguely.

## Movement, four buckets

- **Closure:** 14 units of this lane's 23-unit population get a real, committed oracle disposition
  for the first time (14 agree).
- **Reclassification:** none — no unit's `docs/work-inventory.json` `status` field changed (oracle
  results live in this directory's own JSON, matching every prior `AT-33-E5-00x` lane's
  convention).
- **Reachability:** 0 units newly reachable via new engine code this cycle for the COMBAT/
  dissonance shapes (both fixes this cycle — Finding 1 harness fix, Finding 2 engine fix — unblock
  the SKILL population, already counted in Closure above; the COMBAT/dissonance reachability gaps
  remain exactly where `AT-33-E5-shape-combat` left them, confirmed not silently regressed).
- **Instrument-correction:** 2 found and FIXED this cycle — (1) the real root cause of every prior
  wave's `ultimate_psionics` "Could not find campaign" failure (a `CAMPAIGN:` display-name vs.
  `KEY:` divergence, not a fundamental oracle-data gap as every prior wave assumed — `scripts/
  retro.py correction`, `sd33-r5-skillcombat.jsonl`); (2) `equipment_id_resolve`'s general
  OUTPUTNAME-divergent-identity gap (of which `AT-33-E5-shape-combat`'s 2-unit finding was a
  narrower symptom), fixed with a real RED→GREEN engine change.

## Notes

- **Both root-caused defects are genuinely new to this cycle**, not re-derivations of an already-
  known fix: Finding 1 directly contradicts (and corrects) `AT-33-E5-last75`'s own Finding 1 claim
  that the direct-`java` runner alone "fixes" `ultimate_psionics` campaign loading — that claim's
  own worked example (`hunter_s_sight`) was actually `advanced_class_guide`, never an
  `ultimate_psionics` item; this cycle is the first to run an `ultimate_psionics` item's campaign
  load to a clean, no-SEVERE-line, no-equip-warning success.
- The `equipment_catalog_rows()` count assertion
  (`rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts`,
  currently `8100` vs. a live `8119`) is a **pre-existing red observed during test scoping, not
  caused by this cycle's change** — confirmed by inspection: that function and its whole call
  chain (`hand_authored_equipment_rows`/`equipment_gap_tables::equipment_gap_rows`) never call
  `corpus_loader.rs` (grep confirms zero references), so this cycle's `equipment_record_from_json`
  fix cannot have moved it. Named honestly, not fixed (outside this lane's write scope and its
  named criterion).
- `scripts/oracle_harness/campaign_key.py` deliberately keeps `CAMPAIGN_KEY_OVERRIDES` to the one
  confirmed-live divergence — never speculatively pre-populated from every `.pcc`'s own `KEY:`
  token, which would risk a false "fix" for a book whose KEY equals its CAMPAIGN name (the common
  case, e.g. Core Rulebook).

## RED→GREEN

`src/rules_core/corpus_loader.rs`: RED (`outputname_divergent_record_still_resolves_by_its_real_key`
fails — `panicked at ... "Companion Stone (Diplomacy) must resolve by its real KEY..."`) → GREEN
(6/6 `corpus_loader::` tests pass; 70/70 `equipment_effects::` tests pass, no regression). Before
this cycle: `last67-skill-combat.oracle-results.json` did not exist; 0 of this lane's 23-unit
population had any per-unit disposition, and 12 of the 14 SKILL units could not even resolve
through the engine. After: 14 real per-unit rows, each backed by a live PCGen export AND a live
engine computation, both independently re-derivable.

## Test scoping

Ran `cargo test --locked --lib corpus_loader::` (6/6, includes the new RED→GREEN test) and
`cargo test --locked --lib rules_core::equipment_effects` (70/70, regression check on the one
resolver chain this cycle's fix sits upstream of). Ran 15 real, live direct-`java`
`charbuild_remainder_run_one.sh` invocations against the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`): 14 SKILL items + 1
`rod_alertness` INITIATIVE probe (used to investigate, not to bank a row — see the 9-not-examined
section). Ran `cargo build --locked --bin e5_statsave_skill_ours` (exits 0). **Did not** run the
root `cargo test` sweep (the pre-existing, unrelated `equipment_resolver` catalog-count red would
make a full-sweep exit code misleading without this receipt's own note; scoped runs above are the
real, relevant coverage for this cycle's actual diff) or `apps/desktop/src-tauri` (a separate cargo
workspace; no file in it touched this cycle).

```
$ cd /home/ubuntu/workspace/repos/pcgen && data check:
$ bash scripts/fetch-pcgen-oracle.sh --check
pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 /home/ubuntu/workspace/repos/pcgen
```

## Next-cycle plan

1. **`COMBAT|INITIATIVE`/`TOHIT`/`TOHIT.Ranged` resolver + the export-token gap (Finding 2 above,
   7 units):** before writing any new `arms_armor.rs`/new-module resolver, first resolve the
   export-token question — try `INITCOMP`/`CControl.INITIATIVE`'s real formula source in the
   pinned Pathfinder gamemode (`system/gameModes/Pathfinder/*.lst`, search for what variable/
   bonus-type actually feeds the character's own `Initiative` computation) to confirm whether
   `BONUS:COMBAT|INITIATIVE` is even the code path PCGen's own PF1 gamemode reads, before trusting
   any comparison built on it.
2. **Formula evaluator for `<int>+Global_LuckBonus` (3 of the 7 units):** `Global_LuckBonus` is
   confirmed architecturally 0 for an isolated test character (no `DEFINE` sets it anywhere in the
   pinned corpus); a narrowly-scoped evaluator for exactly this pattern is real, justified new
   surface once (1) is resolved.
3. **Dissonance pair (2 units):** fix `compute_equipmods_effect`'s `TYPE=Enhancement` case-
   sensitivity (a one-line, low-risk change verified against `AT-33-E5-last75`'s own finding)
   FIRST — cheap and independent of the fixture-plumbing problem; then build the base-weapon+
   `EQMOD:`-attachment `.pcg` pattern (no existing fixture generator does this) to get a live
   comparable value.
4. Re-run `AT-33-E6-001` once this and the sibling disagreement/remainder lanes land — population
   will still be short of 8,330 by up to 9 units (this lane's own remainder) unless a further
   cycle closes them.
