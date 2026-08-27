# Cycle AT-33-E5-last67-eqm — Epic 5 Re-verification / AT-33-E5-002

- **Commit SHA:** `c8bd0364f2`
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-eqm.oracle-results.json` (new — this lane's committed deliverable, 0 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-last67-eqm_cycle_receipt.md` (this file)
  - `scripts/oracle_harness/eqm-fixtures/` (new — 10 `.pcg` fixtures + 4 `.ftl` export templates, the live-oracle attempts this cycle made)
  - `docs/release/SD-33-computed-value-verification/progress.md` / `kanban.md` (updated in place)
  - `docs/retro/events/sd33-r5-eqm.jsonl` (new — 1 incident)
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
Matches the brief's stated 67. This lane's own 7-unit slice, identified by reading each unit's
whole corpus record (`raw_bonus_chains`, not a BONUS-filtered view):

| unit_id | shape | corpus_key | raw_bonus_chains |
|---|---|---|---|
| `core_rulebook:equipment_modifier:draco` | `EQMARMOR` material | `DRACO` | `VAR|ArmorCheckPenalty|-1|TYPE=Enhancement`, `EQMARMOR|ACCHECK|1|TYPE=Enhancement` |
| `core_rulebook:equipment_modifier:dragonhide` | `EQMARMOR` material | `Dragonhide` (KEY-overridden to `Material ~ Dragonhide`) | same pair |
| `core_rulebook:equipment_modifier:material_dragonhide` | `EQMARMOR` material | `Material ~ Dragonhide` | same pair |
| `core_rulebook:equipment_modifier:special_quality_spikes_shieldbash` | `EQMWEAPON|DAMAGESIZE` | `Special Quality ~ Spikes ~ Shieldbash` | `EQMWEAPON|DAMAGESIZE|1` |
| `core_rulebook:equipment_modifier:spike_sb` | `EQMWEAPON|DAMAGESIZE` | `SPIKE_SB` (KEY-overridden, same target) | `EQMWEAPON|DAMAGESIZE|1` |
| `ultimate_combat:equipment:arrow_iron_tipped_distance_20` | `EQMWEAPON|RANGEADD` | `Arrow (Iron-tipped Distance/20)` | `EQMWEAPON|RANGEADD|10`, `WEAPON|DAMAGE|-1` |
| `advanced_race_guide:equipment_modifier:material_darkleaf_cloth_clothing` | `EQM|WEIGHTDIV` | `Material ~ Darkleaf Cloth ~ Clothing` | `EQM|WEIGHTDIV|2` |

7 of 7 assigned to a shape; the sibling lanes' own shape lists (wield-size-no-penalty x3,
dissonance x2) account for the other 11 `equipment_modifier` ids in the 67, confirmed by grep
below (no unclaimed id in this lane's population):
```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
mine={'core_rulebook:equipment_modifier:draco','core_rulebook:equipment_modifier:dragonhide','core_rulebook:equipment_modifier:material_dragonhide','core_rulebook:equipment_modifier:special_quality_spikes_shieldbash','core_rulebook:equipment_modifier:spike_sb','ultimate_combat:equipment:arrow_iron_tipped_distance_20','advanced_race_guide:equipment_modifier:material_darkleaf_cloth_clothing'}
print(len(mine), mine.issubset(set(miss)))"
7 True
```

## The method attempted (per the brief's own routed instruction)

For the 6 genuine `equipment_modifier` units (all but the arrow, which is a real top-level
`equipment` ammunition record carrying its own `EQMWEAPON|RANGEADD` chain directly — no host
needed, tested standalone), the brief's routed method is: pick a concrete host item, attach the
modifier, compare the host's computed value with and without it. `scripts/oracle_harness/`'s
proven runner (`charbuild_remainder_run_one.sh`, the direct-`java` path, `PCGEN_REPO_DIR` resolved
via the pinned checkout — no literal path in this doc) was reused unmodified.

The attachment mechanism attempted: a hand-authored `.pcg` `CUSTOMIZATION:[BASEITEM:<key>|
DATA:EQMOD=<modkey>]` block on the `EQUIPNAME:` line — the exact real-PCGen-save-format syntax,
confirmed byte-for-byte against real player-saved characters (`~/workspace/repos/pcgen/characters/
CodeMonkey.pcg`: `EQUIPNAME:Bedroll|...|CUSTOMIZATION:[BASEITEM:Bedroll|DATA:EQMOD=CLOTH]`) and
against `pcgen.io.PCGVer2Creator.appendEquipmentLines`/`Equipment.formatSaveLine` (the writer that
produces this exact shape) and `pcgen.io.PCGVer2Parser.parseEquipmentLine` (the reader), traced
line-by-line including the nested `[...]` `PCGTokenizer` bracket parser.

Hosts chosen and why:
- **EQMARMOR materials (draco/dragonhide/material_dragonhide):** host = `Leather Armor (Base)`
  (`core_rulebook/cr_equip_arms_armor.lst`) — a plain, unmagical armor with a KNOWN, literal
  baseline (`ACCHECK:0`, `BONUS:VAR|ArmorCheckPenalty|0|TYPE=BaseArmor`), so any change is
  attributable to the attached modifier alone, not to some other pre-existing chain. Comparable
  quantity: the `ArmorCheckPenalty` PC-level variable (the same one `general::compute_var_effect` +
  `general::apply_eqmod_var_bonus` already resolve for this bundle's `var-bonus-shape` lane, and
  the same mechanism wave 4 confirmed correct for Mithral on Panoply of the Fierani Knight, per
  `general.rs`'s own doc comment) — queried live via PCGen's own `VAR.ArmorCheckPenalty.INTVAL`
  export token (`plugin.exporttokens.VarToken`).
- **EQM|WEIGHTDIV (material_darkleaf_cloth_clothing):** host = `Outfit (Explorer's)`
  (`core_rulebook/cr_equip_general.lst`, `TYPE:Goods.Clothing...`, `WT:8`) — the modifier's own
  `TYPE:BaseMaterial.MasterworkQuality.Cloth.Clothing` requires a `Clothing`-type host; Outfit
  (Explorer's) is the CRB's own plain clothing item with a known weight. Comparable quantity: the
  host's own weight (queried live via `EQ.MERGELOC.0.WT`, `plugin.exporttokens.EqToken`) — a
  fractional divide is a real possible outcome (`8/2=4`, an integer here, but the harness would
  have reported a decimal had one occurred; none did, since the mechanism did not engage at all —
  see below).
- **EQMWEAPON|RANGEADD (arrow_iron_tipped_distance_20):** no host needed — the record IS the item.
  Attempted standalone, queried via `WEAPON.0.RANGE`.
- **EQMWEAPON|DAMAGESIZE (special_quality_spikes_shieldbash / spike_sb):** host = `Heavy Wooden
  Shield (Base)` (carries a real `DAMAGE:1d4` shield-bash token, confirmed by this bundle's own
  `arms_armor.rs` module doc comment). Comparable quantity: per the brief's own explicit routing
  ("if the comparable quantity is a die step or a die string rather than a number, say so
  explicitly"), this would have been the shield-bash die STRING (`WEAPON.0.DAMAGE`), not a scalar —
  never reached (see below).

## What actually happened — a real, unresolved harness gap (not a guess)

**EQMARMOR materials (leather armor, draco):**
```
$ bash scripts/oracle_harness/charbuild_remainder_run_one.sh <leather_baseline.pcg> <var_acp.txt.ftl> ...
VAR.ArmorCheckPenalty=0
$ bash scripts/oracle_harness/charbuild_remainder_run_one.sh <leather_draco.pcg> <var_acp.txt.ftl> ...
VAR.ArmorCheckPenalty=0    # expected -1 (DRACO's own BONUS:VAR|ArmorCheckPenalty|-1|TYPE=Enhancement)
```
No load warning (`Could not find equipment`/`Could not find EquipmentModifier`) was logged in
either run — ruling out a simple key-lookup miss. A debug pass added `EQ.MERGELOC.0.NAME` and
`EQ.MERGELOC.0.SPROP` queries: both stayed at the item's plain, unmodified values (`Leather Armor`,
empty `SPROP`) with the customization attached — the modifier is not being attached to the item at
all, not merely failing to contribute its bonus.

**EQM|WEIGHTDIV (outfit, darkleaf):**
```
$ bash .../charbuild_remainder_run_one.sh <outfit_baseline.pcg> <eqwt.txt.ftl> ...
EQ.0.WT=8
$ bash .../charbuild_remainder_run_one.sh <outfit_darkleaf.pcg> <eqwt.txt.ftl> ...
EQ.0.WT=8    # expected 4 (WEIGHTDIV|2 halving the base 8)
```
Same null result on an independent shape/host/export-token combination.

**EQMWEAPON|RANGEADD (arrow, standalone):** the standalone run crashed loading `Ultimate Combat`
(`LSTERROR ... Could not get Reference Manufacturer for Category: Cavalier Class Feature`, exit
code 1, no output file) — a pre-existing data-loading defect in that book unrelated to this
modifier, not reachable to a fix within this cycle's write scope.

**EQMWEAPON|DAMAGESIZE (shieldbash):** not reached — the CUSTOMIZATION mechanism's failure on two
independent, simpler shapes made a third live attempt not a productive use of remaining budget;
recording it honestly as not-attempted rather than padding the finding list.

Two independent shapes (materials, weight-division), two independent hosts, two independent export
tokens, all show the SAME null result with no error — this is a genuine, reproducible gap in using
a hand-authored `.pcg` `CUSTOMIZATION:` block to attach an `equipment_modifier` at harness-build
time, not a per-unit fluke. Source-level tracing (`PCGVer2Parser.parseEquipmentLine`,
`PCGTokenizer`, `Equipment.setBase`/`.load`/`.getBaseItemName`) found no syntactic defect in the
constructed `.pcg` lines — the failure is at a level this cycle did not reach (candidates for next
cycle: a version-gate on `PCGVER2.0`'s `CUSTOMIZATION` path this pinned build silently skips in
headless/`BatchExporter` mode; a `ReferenceContext` state PCGen's own live customizer UI populates
that a raw file load does not).

## Retrospective event

```
$ python3 scripts/retro.py incident --recurrence-key pcg-customization-eqmod-not-applied ...
```
logged to `docs/retro/events/sd33-r5-eqm.jsonl` (1 incident, `--silent` — the failure produced a
plausible-looking unchanged export, not an error, on both independently-tested shapes).

## Verdict discipline

Zero rows written. No unit was forced through a partially-working pipeline to produce a number —
per doctrine, "no fabricated number" applies as much to a harness gap as to an unhandled corpus
shape. Every one of the 7 units stays **unexamined** (not `unverifiable`, since `unverifiable`
requires a genuine structural absence of a comparable magnitude — these DO have a comparable
magnitude; the harness could not yet reach it).

## Status: blocked-escalated

**Not `complete`.** 0 of this lane's 7-unit population reached a real, committed oracle
disposition this cycle. The blocker is named precisely (the `.pcg` `CUSTOMIZATION:` attachment
mechanism, confirmed non-functional on 2 independent shapes/hosts/export-tokens, syntactically
traced and ruled out as a construction error) with a concrete next-cycle plan below — not "ran out
of time" vaguely.

## Movement, four buckets

- **Closure:** 0 — no unit reached a committed disposition this cycle.
- **Reclassification:** none — no unit's `docs/work-inventory.json` `status` field changed.
- **Reachability:** 0 confirmed reachable this cycle; the arrow's own `EQMWEAPON|RANGEADD` chain
  and the shieldbash `DAMAGESIZE` chain remain untested (blocked/not-reached respectively).
- **Instrument-correction:** 1 found — the `.pcg` `CUSTOMIZATION:` host-attachment mechanism does
  not work as documented/expected in this harness, discovered and named this cycle
  (`scripts/retro.py incident`, `sd33-r5-eqm.jsonl`, recurrence-key
  `pcg-customization-eqmod-not-applied`) rather than silently producing a false `agree` (a
  false-`agree` risk this cycle's own careful baseline-vs-modified comparison design caught: had
  only the modified run been checked without a baseline, `VAR.ArmorCheckPenalty=0` would have
  looked like a plausible real value instead of a stuck one).

## Notes

This cycle spent its budget establishing, with real live-PCGen evidence (not assumption), that the
straightforward "hand-write a `.pcg` CUSTOMIZATION block" attachment method — while syntactically
identical to real PCGen-saved output — does not take effect in this harness. That is itself a real,
useful, re-derivable finding: it rules out the naive approach for every future `equipment_modifier`
lane, not just this one's 7 units, and points the next cycle at a proven-working alternative (see
below) rather than a second blind attempt at the same mechanism.

## RED→GREEN

No `src/rules_core/` or `src/bin/` change landed this cycle. No resolver was written for any of the
4 shapes (`EQMARMOR` materials, `EQMWEAPON|DAMAGESIZE`, `EQMWEAPON|RANGEADD`, `EQM|WEIGHTDIV`)
because no shape reached a trustworthy oracle value to verify a resolver against — writing an
"ours" resolver before the oracle side is real would risk exactly the "no fabricated number"
violation doctrine forbids, and `general::compute_var_effect` + `general::apply_eqmod_var_bonus`
already exist and would have been reused unmodified for the `EQMARMOR` shape had the oracle side
worked (confirmed by reading the existing resolver before writing anything new).

## Test scoping

No `src/` or `apps/` file changed this cycle (fixtures/results/receipt only), so the root `cargo
test` sweep and `apps/desktop/src-tauri` were not run — nothing in this cycle's diff could regress
either. `git status --porcelain` before every write confirmed the tree carried only this lane's own
untracked additions.

```
$ python3 -c "import json
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last67-eqm.oracle-results.json'))
print('rows', len(d['results']))"
rows 0
```

## Identifier / wired-integration audit (final diff)

```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- scripts/oracle_harness docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- scripts/oracle_harness docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```
The only match in the full scoped diff is this receipt's own line quoting the audit command
above (the same self-referential shape every prior `AT-33-E5-00x` receipt exhibits, e.g.
`AT-33-E5-last75_cycle_receipt.md`'s own audit section) — no real `STUB`/`MOCK`/`placeholder`/
`todo`/`fixme`/`hack` token appears in any `.pcg`, `.ftl`, `.json`, or `.jsonl` file this cycle
added. `OK_NO_TOKENS`.

## Next-cycle plan

1. **Do not retry the `.pcg` `CUSTOMIZATION:` mechanism blind.** Instead, use the SAME proven
   pattern this bundle already validated for `equipment_modifier` verification
   (`e5_disagreement_fixes_ours.rs`, Panoply of the Fierani Knight / Armor of Grim Triumph / Amulet
   of Mighty Fists): a *corpus item that already embeds `EQMOD:<key>` in its own LST definition*.
   Since no such real item exists for `DRACO`/`Dragonhide`/`Material ~ Dragonhide`/
   `Material ~ Darkleaf Cloth ~ Clothing`/`SPIKE_SB`/`Special Quality ~ Spikes ~ Shieldbash` in the
   pinned corpus (confirmed this cycle: `grep` across `data/corpus/*/equipment/*/*.json` for these
   keys under an `EQMOD` raw token finds none), the next cycle needs either (a) a minimal
   operator-approved homebrew-campaign LST snippet defining one throwaway item per modifier with
   the `EQMOD:` baked in at load time (the exact mechanism PCGen itself uses for every real magic
   item), keeping it entirely inside `scripts/oracle_harness/` and never touching `data/corpus/`,
   or (b) root-causing why the live `.pcg` CUSTOMIZATION path silently no-ops in this specific
   pinned build/headless mode (candidate: instrument PCGen's own `PCGVer2Parser` with a debug build,
   or diff against a GUI-driven save of the identical customization to see what state differs).
2. **EQMWEAPON|RANGEADD (arrow):** fix or route around the `Ultimate Combat` `Cavalier Class
   Feature` load crash first (a pre-existing oracle-data defect, not this lane's own construction)
   — try loading `Ultimate Combat` alone in a minimal fixture to confirm the crash is
   book-load-order-independent before concluding it blocks every `ultimate_combat` unit, not just
   this one.
3. **EQMWEAPON|DAMAGESIZE (shieldbash):** once (1) is resolved, attach and read `WEAPON.0.DAMAGE`
   as a die STRING per the brief's own routing; if genuinely non-scalar even then, record
   `unverifiable`/`no_comparable_scalar` honestly rather than fabricating a resolver.
4. Re-run `AT-33-E6-001` once this lane and its 3 siblings (the other 60 of the 67, plus the 4
   disagreements) all land — population will still be short of 8,330 by up to 7 units unless a
   further cycle closes this lane specifically.
