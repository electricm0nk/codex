# F1 population converter run — structural-diff receipt

Worktree: `/home/ubuntu/workspace/worktrees/codex-epic-f1` on `sd36/epic-f1`
(`git log --oneline tranche/16..sd36/epic-f1` — 8 commits, converter changes already landed:
`71c729e408` parent-category resolution, `e426f45c8b` GatedFactGrant, `42583e3d74` WeaponSet/
WeaponAllOf expansion, `865dbd13a0`/`c9bf436fb8`/`ae40fe83d1`/`2e4b73c42a` adversarial-check
fixes, `e820153e60` package handle). This step ran the corpus-wide converter to a **scratch**
dump only — `data/sheet_rules/` on disk was never written. Worktree `git status --short` was
empty before and is empty after (verified at the end of this file).

## 0. Preconditions

| Check | Command | Result | Expected |
|---|---|---|---|
| Clean tree | `git status --short` | (empty) | empty |
| `_report.json` | `python3 -c "import json;r=json.load(open('data/sheet_rules/_report.json'));print(r['records'],r['converted'],r['rules_written'])"` | `49450 49450 71862` | `49450 49450 71862` |
| Unresolved references | `python3 -c "import json;print(len(json.load(open('data/sheet_rules/_defects/unresolved-references.json'))))"` | `11925` | `11925` |

## 1. The converter run

Command: `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --dump <scratch>/dump-after`

```
1:records=49450 converted=49450 refused=0 rules=71863 var_tables=5882 dumped=55348 -> <scratch>/dump-after (112.6s)
```

`/usr/bin/time -v` wall clock: **2:05.05** (125.05s); exit 0; peak RSS 746,068 KB (~729 MiB).
`records`/`converted` unmoved (`49450 -> 49450`, the frozen invariant holds). `rules_written`
71862 -> 71863 (+1, named in §4). `var_tables` 5309 -> 5882 (+573, explained in §3).

## 2. Structural diff (`structural_diff.py`, `--max-examples 100000`)

Full output: `structural-diff.txt`. Machine-readable summary: `structural-diff.json`.

```
== file set ==
  rule files:  +0 -0  (+1 -0 rule ids)
  _vars/:      +573 -0
  _defects/:   +1 -0
  other:       +0 -0  []

== counts (baseline -> fresh) ==
  records: 49450 -> 49450
  converted: 49450 -> 49450
  refused: 0 -> 0
  rules_written: 71862 -> 71863
  var_tables: 5309 -> 5882

== added granted_by edges by TARGET kind ==
  class_feature: 3722
  race_trait: 426
  companion: 192
  ability: 118
  domain: 33
  TOTAL: 4491
  added grants (total): 102

unexpected field deltas: 0
new rule ids: 1 ['ultimate_psionics:class:psion#bonus5']
removed granted_by edges: 0
removed grants: 0

verdict=PASS
```

**Exit 0. Verdict PASS** — no removed rule ids, no unexpected field deltas, no removed
`granted_by` edges, no removed `grants`, no moved `records`/`converted` count. This is the F1.6
gate (`unexpected field deltas: 0`, `records 49450 -> 49450`) passing cleanly.

## 3. `_vars` tables added: 573 — why (gated grants)

Every added `_vars/*.json` file is a condition-variable table a `GatedFactGrant`'s `when`
expression now references (F1-2, 3.1 item 4 — the gate is carried onto the effect instead of
being discarded by the old `let _ = when;`). None are proficiency-unrelated; each is
`declared_by` one or more rules that gained a `GatedFactGrant`. Three examples (path -> content,
truncated):

1. `_vars/v000aae4c54547015.json` — `"label":"Kineticist CF Elemental Overflow"`,
   `declared_by":["occult_adventures:class_feature:kineticist","occult_adventures:class_feature:kineticist_class"]`.
2. `_vars/v003ef064959a0796.json` — `"label":"Monk CF High Jump"`,
   `declared_by":["core_rulebook:class_feature:monk","core_rulebook:class_feature:monk_class"]`.
3. `_vars/v00a4193497a65c72.json` — `"label":"Brawler CF Martial Training"`,
   `declared_by":["advanced_class_guide:class_feature:brawler"]`.

Denominator: 573 of 573 added `_vars/` files; 0 removed. No `_vars` table was deleted or
mutated in place (file-set diff shows `_vars/: +573 -0`).

## 4. Rules added / removed — named, with cause

**Added: 1. Removed: 0.**

`ultimate_psionics:class:psion#bonus5` — "Psion (psion feat picks)", `value` =
`ClassLevel(psion)/5 + 1`, `applies` = `1 <= ClassLevel(psion) <= 20`, gated to `Pool: psion_feat`,
`print: true`. Cause: the `current_class` closure-tracker fix (`closure.rs`, commit
`71c729e408`) correctly attributes a 5th `BONUS:ABILITYPOOL|Psion Feat` occurrence in the Psion
subclass block (`up_classes.lst`) to the Psion chassis, alongside the four it already held —
same value/gate shape as its siblings (`bonus1`-`bonus4`). This is
`KNOWN_ADDED_RULE_CAUSES` in `structural_diff.py` itself; the script asserts the cause is named,
not merely counted.

## 5. `provenance`/`closure_rows` field deltas — every affected record, with cause

**30 of 30** — exactly the pinned list in `structural_diff_expected_provenance_deltas.json`
(`_count: 30`), and **no record outside that list carries a `provenance` delta** (confirmed:
`unexpected field deltas: 0` in §2 — a `provenance` delta on any other record would have
appeared there). Cause for all 30: the `current_class` closure-tracker fix
(`closure.rs`, commit `71c729e408`) re-attributes `SUBCLASSLEVEL` rows in a class's own chassis
block that the pre-fix closure build misattributed off the class (the general form of the
Wizard silent-miss root cause in `silent-miss-diagnosis.md` Part 1 — that diagnosis traced
exactly this bug in `current_class` tracking, scoped there to Wizard's own miss; the fix
committed on this branch is generic, so it also re-attributes rows for 5 other class families).
Verified: for all 30, `sd.diff_rule(old, new) == ['provenance']` (the **only** field delta is
`provenance`; each record's own `grants`/`granted_by` — checked separately, since those two
fields are excluded from `diff_rule` — did **not** change on any of these 30 records; see §6).

The 30 split into 6 class families x their own `#bonusN` sub-rules:

| Class | Records | Count |
|---|---|---|
| `core_rulebook:class:cleric` | principal + `#bonus1`..`#bonus5` | 6 |
| `core_rulebook:class:wizard` | principal + `#bonus1`..`#bonus4` | 5 |
| `inner_sea_gods:class:exalted` | principal + `#bonus1`..`#bonus3` | 4 |
| `inner_sea_magic:class:eidolon_fey` | principal + `#bonus1`..`#bonus3` | 4 |
| `ultimate_magic:class:magus` | principal + `#bonus1`..`#bonus5` | 6 |
| `ultimate_psionics:class:psion` | principal + `#bonus1`..`#bonus4` | 5 |
| **Total** | | **30** |

(6+5+4+4+6+5 = 30.) `provenance.closure_rows` grows on each (additional `SUBCLASSLEVEL`
oracle lines now correctly attributed); nothing else on these 30 records changed.

## 6. Any OTHER field delta (the dangerous class) — 0, verified two ways

`structural_diff.py`'s own `unexpected field deltas: 0` (§2) is the primary gate. Independently
verified: for the 30 pinned §5 records, `granted_by`/`grants` (excluded from `diff_rule`,
checked separately here since a same-length replacement inside either would not show as a
`diff_rule` field name) are **byte-identical, old==new, on all 30** — the closure-tracker fix
changes `provenance.closure_rows` only, adds no new `Grant`/`granted_by` entry on any of these
30 records itself (their *targets* elsewhere do gain edges — see §7). No record anywhere in the
49,450-record population has a delta outside `{granted_by, grants, provenance}`, and every
`provenance` delta is on the pinned 30. **Unexpected field deltas: 0 of 49,450.**

## 7. `granted_by` edges added — every delta class, with denominator, mechanism, examples

**Total measured: 4,491** (target-kind breakdown: class_feature 3,722; race_trait 426;
companion 192; ability 118; domain 33). **Spec's mechanism-A estimate: 4,456.** Net difference:
**+35**. This section explains the difference by decomposing 4,491 by the edge's `by` shape
(who is recorded as the granter) and cross-referencing against the 4,456 rows that vanished
from `unresolved-references.json` (`11925 - 7469 = 4456`, exact match — §8).

### 7a. `by`: `Rule` (an existing rule is the granter) — 3,970 edges

Denominator: 3,970 of 4,491 (88.4%). Mechanism: mechanism-A parent-category resolution
(`resolve_rule` retries under the child category's parent, `closure.rs`/`ctx.rs`). **Every**
edge in this shape traces to a source that had at least one matching row removed from
`unresolved-references.json` (verified: 0 of 3,970 "by"-Rule sources are absent from the
removed-row source set). The raw per-source count does not reconcile 1:1 to the removed-row
count for every source, because the closure can hold the *same* (source, target) reference more
than once under different `when` gates (e.g. once unconditionally, once behind an
archetype-off variable) — each occurrence resolves and writes its own edge. Spot-checked, Bard:
21 removed defect rows (10 from `core_rulebook:class_feature:bard`, 11 from its
`bard_class__cbc80724848276a8` sibling) produced 39 added edges (18 more than 21) because each
target (e.g. `bard_armored_casting`) is referenced twice from each of the two bard source
records under differing `when`. Three examples (all pass per-edge verification, §9):

1. `advanced_class_guide:class_feature:animist_animist_spirit_magic` <- `by: {Rule:
   advanced_class_guide:class_feature:shaman_archetype_animist}`, `when: ClassLevel(shaman) >=
   1`.
2. `core_rulebook:companion:animal_companion_ac_bonus` <- `by: {Rule:
   core_rulebook:companion:base_companion_animal_companion}`, `when: Always`.
3. `advanced_race_guide:ability:racial_size_medium` <- `by: {Rule:
   advanced_race_guide:ability:rules_use_race_builder_system}`, `when: Not(Holds RuleTag
   AltRaceSize)`.

### 7b. `by`: `Choice` (a racial-subtype alternative is the granter) — 31 edges

Denominator: 31 of 4,491. Mechanism: same parent-category retry, applied to a racial-subtype
"pick one of several subtypes, each offering its own trait" shape — the target trait is
`granted_by` **each** subtype that can grant it (an OR, not a single owner). Three examples:

1. `advanced_race_guide:race_trait:dwarf_minesight` <- `by: {Choice:
   advanced_race_guide:race_trait:dwarf_racial_subtype_deep_delver}`.
2. `advanced_race_guide:race_trait:dwarf_mountaineer` <- `by: {Choice:
   advanced_race_guide:race_trait:dwarf_racial_subtype_mountain_dwarf}`.
3. `advanced_race_guide:race_trait:elf_arcane_focus` <- **two** edges, `by: {Choice:
   ...elf_racial_subtype_dusk_elf}` and `by: {Choice: ...elf_racial_subtype_tower_elf}` (either
   elven subtype grants Arcane Focus).

### 7c. `by`: `Class` (a class itself, at a level, is the granter) — 490 edges, TWO mechanisms

Denominator: 490 of 4,491, split:

**7c-i. 441 of 490 — mechanism A, matches the spec's own count exactly.** These trace 1:1 by
source class id to the **46 distinct class records / 441 rows** mechanism-A already names for
source kind "class" (`unres2.py`'s "A by source kind: class 441" — §1 of the epic doc). This is
the single strongest cross-check in this receipt: an independently measured count (edges
attributed to a class-level granter) landing on the exact figure the spec derived from a
different script (`unres2.py`, classifying defect rows by source kind). Examples:

1. `advanced_class_guide:class_feature:arcanist_arcane_reservoir` <- `by: {Class: {id: arcanist,
   at_level: 1}}` (removed defect: `advanced_class_guide:class:arcanist: Arcanist Class
   Feature|Arcanist ~ Arcane Reservoir`).
2. `advanced_class_guide:class_feature:bloodrager_bloodrage` <- `by: {Class: {id: bloodrager,
   at_level: 1}}` (removed defect: `advanced_class_guide:class:bloodrager: Bloodrager Class
   Feature|Bloodrager ~ Bloodrage`).
3. `advanced_class_guide:class_feature:arcanist_weapon_and_armor_proficiency` <- `by: {Class:
   {id: arcanist, at_level: 1}}`.

**7c-ii. 49 of 490 — a SEPARATE mechanism: the silent class -> class-ability miss (epic doc
§0.3 / §3.1 item 2), measured at scale for the first time in this run.** None of these 49 has
*any* matching source in the removed-defect-row set for their class (verified: `class:magus`,
`class:exalted`, `class:psion`, `class:wizard` have **zero** rows in
`unresolved-references.json` naming these specific targets, in EITHER baseline or fresh — these
edges were never a logged defect at all, exactly as `silent-miss-diagnosis.md` Part 1 predicted
for Wizard ("the edge is dropped before `resolve_holdable_rule` is ever called ... no defect row
exists"). The `current_class` closure-tracker fix (commit `71c729e408`) is generic (scopes a
`Class`-family file's `current_class` tracking to real `CLASS:` header rows, not every
`Plain`/`Copy` row), so it restores this class of previously-invisible edge for every affected
class, not only Wizard. Measured: **4 classes, 49 edges**:

| Class | Edges | What |
|---|---|---|
| `magus` | 33 | Forbidden Rites archetype's "pick a domain" grants — one per PF1 domain (`core_rulebook:domain:air` .. `:weather`), each `by: {Class: {id: magus, at_level: 0}}` gated `Holds(Rule: ultimate_magic:class_feature:forbidden_rites_domain_<x>_domain) AND ClassLevel(magus)>=0` |
| `exalted` | 11 | Exalted's own class abilities (`exalted_divine_brand`, `exalted_obedience`, `exalted_scholar`, `exalted_weapon_and_armor_proficiency`, `exalted_vitality`, `exalted_divine_boon`, `exalted_religious_speaker`, `exalted_expanded_portfolio`, `exalted_aspect_of_divinity`, `exalted_ardent_vision`, `exalted_perform_miracle`) |
| `psion` | 4 | `ultimate_psionics:ability:psionic`, `:class_feature:psion_manifesting`, `:class_feature:psion_weapon_proficiencies`, `core_rulebook:class_feature:all_automatic_proficiencies` |
| `wizard` | 1 | `core_rulebook:class_feature:wizard_class__d9affb050e701218` — **this is the Wizard n=1 fix the epic doc's §0.2/§0.3 named directly**: the class-ability record that previously had `granted_by: null` and no defect row now correctly reads `by: {Class: {id: wizard, at_level: 1}}` |

Three examples: `core_rulebook:domain:air <- {Class: magus@0}` (gated); `inner_sea_gods:ability:exalted_divine_brand
<- {Class: exalted@1}`; `core_rulebook:class_feature:wizard_class__d9affb050e701218 <- {Class: wizard@1}`.

### 7d. Reconciliation total

3,970 (Rule) + 31 (Choice) + 441 (Class/mechanism-A) + 49 (Class/silent-miss) = **4,491**,
matching the measured total exactly. The **+35 net** versus the 4,456 estimate is the sum of a
positive component from duplicate `when`-gated closure occurrences (§7a, e.g. +18 from Bard
alone) and the **49 wholly-new edges** from the silent-miss fix (§7c-ii, which is *not* part of
mechanism A and was never counted in the 4,456), **minus** a negative component: 28 of the
4,456 mechanism-A candidate rows did **not** produce any edge at all — they were correctly
deferred to a new defect kind instead (§8).

## 8. Unresolved-references rows removed and ADDED

**Removed: 4,456** (`11925 -> 7469`, exact). **Added: 0** — `unresolved-references.json` itself
gained no new rows.

**New defect kind: `_defects/ambiguous-parent-category-target.json` — 28 rows.** All 28 were
present in the baseline's `unresolved-references.json` (verified: every one of the 28 is a
member of the 4,456 removed set) and are **not** double-counted as "removed and gone" — they
moved to a sibling defect file rather than resolving. Mechanism: the parent-category retry
found the child category's target NAME under the parent, but **more than one** oracle line
shares that `(parent-category, name)` pair, so the retry — per the epic doc's own requirement
("the retry must keep KEY-exact matching ... never a name-similarity guess") — refuses to guess
and reports the reference as ambiguous instead of silently picking one. Three examples:

1. `advanced_race_guide:race_trait:gnome_racial_subtype_gear_gnome: Gnome Racial Trait|Gnome ~
   Master Tinker`.
2. `advanced_race_guide:race_trait:dwarf_racial_subtype_elder_dwarf: Dwarf Racial Trait|Dwarf ~
   Ancient Enmity`.
3. `ultimate_combat:class_feature:monk_archetype_master_of_many_styles: Monk Class Feature|Master
   Of Many Styles ~ Perfect Style`.

Denominator: 28 of 4,456 mechanism-A candidates (0.6%) correctly deferred rather than resolved;
4,428 of 4,456 (99.4%) resolved to a real edge.

## 9. `unres2.py` re-run against the fresh dump's defect file

Command: symlinked `<scratch>/unres2-fresh-run/data/sheet_rules -> dump-after`, ran
`python3 unres2.py` from that directory (the script's paths are relative to `data/sheet_rules`,
no CLI arg). Full output: `unres2-fresh.out`.

| Mechanism | Baseline (§1 of the epic doc) | Fresh dump (this run) | Unchanged? |
|---|---|---|---|
| A | 4,456 | **0** | **Reached 0, as required (F1.3)** |
| B | 63 | 63 | Yes |
| D | 3,033 | 3,033 | **Yes, byte-identical** |
| E | 3,565 | 3,565 | **Yes, byte-identical** |
| F | 808 | 808 | **Yes, byte-identical** |

Closure rows: 66,237 -> 66,356 (+119, consistent with the newly-held rules' own closure rows
feeding the union — not a gate, informational). **D/E/F did not shrink** (F1.3's own
requirement: "a drop in D/E/F means the retry stole rows from a different mechanism, not a real
fix" — none occurred). Proficiency rows by mechanism on the fresh dump: `{'F': 17, 'D': 7}` (no
`'A'` key — mechanism-A proficiency rows, 99 at baseline, are now 0, all resolved or deferred).

Fresh `unresolved-references.json` total: **7,469** = `11925 - 4456`, exact match to F1.3's
acceptance number (`11,925 - 4,456 = 7,469`).

## 10. Per-edge correctness

Verification: for a defect row `source: category|name`, resolve the target's oracle line
address(es) via the same child->parent map and oracle index `unres2.py` builds (rebuilt
independently, `build_oracle_idx.py`, 2,384 child categories / 80,132 `(category,name)` keys),
then confirm that the specific TARGET rule which gained the `granted_by` edge attributed to
that source has that oracle line address in its own `provenance.closure_rows` (not merely
"some rule in the package" — the exact target). A row whose row is in the new
`ambiguous-parent-category-target.json` file (§8) counts as a correct, intentional non-resolution.

**Seed: 20260921** (`random.seed(20260921)`, `random.sample(removed, 200)` over the 4,456
mechanism-A removed rows).

| Sample | n | Pass (direct match) | Pass (ambiguous, correctly deferred) | Fail |
|---|---|---|---|---|
| Random sample of 200 | 200 | 199 | 1 | **0** |
| ALL class->proficiency edges (source kind `class`, ref matches `roficien\|Weapon Prof\|Armor Prof\|Shield Prof`) | 17 | 17 | 0 | **0** |

**0 failures in either set.** The 1 "ambiguous-correctly-deferred" row in the random sample
(`advanced_race_guide:race_trait:elf_racial_subtype_tower_elf: Elf Racial Trait|Elf ~ Arcane
Focus` — note: this reference in isolation looked ambiguous to the checker but is in fact
resolved via the `Choice` mechanism, §7b; re-verified directly: `elf_arcane_focus` carries a
`by: {Choice: elf_racial_subtype_tower_elf}` edge whose target closure_rows contains the exact
oracle line) — full detail in `per_edge_correctness_final.out`. This satisfies F1.3's per-edge
pin requirement: "a count match alone ... is satisfied equally by an edge written to the right
record and one written to a colliding record of the same slug ... so the count is necessary but
not sufficient and the per-edge pin is required to close the row" — the pin found 0 mis-targeted
edges across 217 checked rows (200 random + 17 proficiency, with 1 overlap counted once each).

## 11. Verdict fields

- **Records unchanged:** **Yes.** `records=49450 converted=49450` before and after; 0 rows
  moved in `_report.json`'s frozen counts.
- **Unexpected field deltas:** **0** (§6; `structural_diff.py`'s own gate, cross-checked
  independently for the 30 pinned records).
- **Per-edge sample failures:** **0** (§10; 217 of 217 checked rows correct, seed `20260921`
  printed).
- **`structural_diff.py` exit code / verdict:** `0` / `PASS`.
- **`unres2.py` mechanism A on the fresh dump:** `0` (required by F1.3).
- **`unres2.py` mechanisms D/E/F on the fresh dump:** `3033/3565/808`, byte-identical to
  baseline (required by F1.3 — "must not shrink unexplained").
- **New defect-file kind:** `ambiguous-parent-category-target.json`, 28 rows, all pre-existing
  mechanism-A candidates correctly deferred rather than mis-resolved (§8).
- **Invariants:** `data/corpus/**` and `site/**` untouched (never read or written by this step);
  `python3 scripts/pcgen_residue_gate.py --check --closure` -> `verdict=PASS` (0 of 0) after
  this run; worktree `git status --short` empty before and after.

## 12. Worktree state at close

```
$ git status --short
(empty)
```

`<scratch>/dump-after/` (55,350 files) is left in place per this step's instructions, for the
next step. No file under `/home/ubuntu/workspace/worktrees/codex-epic-f1` was modified, staged,
or committed by this step; nothing was pushed.
