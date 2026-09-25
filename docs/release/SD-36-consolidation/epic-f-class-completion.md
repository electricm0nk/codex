---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-21
---

# SD-36 Epic F — Class completion

Planned against `tranche/16` HEAD `9d661623d3`. Every figure below carries its command, or is
marked **estimate**. Read-only analysis scripts (re-runnable, committed alongside this document):
`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py` (run from the repo root:
`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py`) and
`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure.py` (post-option-A class-closure
simulation, same invocation pattern). Both scripts are read-only: no repo file was edited and no
cargo build was run to produce their output; both are outside every path the PCGen residue gate
scans (`scripts/pcgen_residue_gate.py`'s `LIVE_ROOTS` is `src/rules_core`, `src/saved_character`,
`src/campaign`, `src/homebrew_authoring`, `apps/desktop` — `docs/release/**` is not a live root),
confirmed by a `--check --closure` run after they were added (§13 below).

**Amended 2026-09-21 for the operator's rulings** (§9): section 3 rewritten for OPTION A, new
section 3b (F1b), sections 8 and 9 rewritten. **Amended again 2026-09-21 against an adversarial
review's 9 findings** — all 9 verified against the code/data, all 9 CONFIRMED and closed in place
(design, order, a carrier rule, an acceptance command, or a size changed; nothing merely noted).
Total size rose from 81-115 to 100-140 agent-hours; the three operator rulings (§9) are unchanged.
**Amended a third time 2026-09-21: the same adversarial review's remaining 6 findings (10-15 of
15; an orchestration bug truncated the first pass at 9) applied** — all 6 re-verified against the
code/data, all 6 CONFIRMED and closed in place (a save-shape accessor, a carrier precedence rule,
four strengthened acceptance rows, a named-exception rule, an acceptance command that could not
pass, and an oracle-pin extension). No size range changed; each fix is absorbed inside its
batch's existing hour estimate as noted at the fix site.
See §12 for the full review log.

**Status:** scoped this pass, docs only, no code touched. F0–F5 are all **open** on `kanban.md` /
`progress.md`. Nothing in this document is a completion claim.

---

## 0. Corrections to the brief (each changes a batch)

**0.1 Weapon proficiency is ALREADY converted into Codex vocabulary.**
`crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs:1390-1410` converts the class weapon
grant into `Fact::Proficiency(ProfRef::{Weapon | WeaponGroup | Chosen | DeityFavoredWeapon})`;
the enum is `src/rules_core/sheet_rule.rs:416-425` (it also has `WeaponTag`, `ArmorGroup`,
`ShieldGroup`). Evidence:
- `data/sheet_rules/ultimate_combat/class_feature/samurai_proficiencies.json` exists, holds
  `{"FactGrant":{"Proficiency":{"WeaponGroup":"Samurai"}}}`, `granted_by`
  `ultimate_combat:ability:samurai`.
- `data/sheet_rules/core_rulebook/class_feature/wizard_weapon_and_armor_proficiency.json` holds
  the five named Wizard weapons (Club, Dagger, Crossbow (Heavy), Crossbow (Light), Quarterstaff).
- `data/sheet_rules/core_rulebook/class_feature/single_simple_weapon_proficiency.json` holds a
  `Chosen` grant plus an `offers` choice ("one simple weapon of choice").
- Files carrying a converted proficiency grant, by kind:
  `grep -rl '"Proficiency"' data/sheet_rules --include=*.json | awk -F/ '{print $4}' | sort | uniq -c`
  -> class_feature 159, monster 75, feat 42, ability 35, race_trait 24, equipment 21,
  monster_ability 11, class 6, companion 6, template 1, trait 1.
The brief's "12 classes have NO proficiency record, dropped at ingest" is true of `data/corpus`
only. The converter reads the pinned oracle directly, and `data/sheet_rules` has the grants.

**0.1a (review finding 1, CONFIRMED, changes 3.1/3.2/3.4/F1 size).** The AUTO arm that emits
these `Fact::Proficiency` grants DISCARDS its own PRE-gate, and the effect it emits has no gate
field to carry it. Verified at `crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`: the
`"AUTO"` arm computes `let when = gates_of(ctx, &gates, level_gate)?;` (line 1392), builds
`facts`, pushes each as `acc.grants.push(Effect::FactGrant(f))`, then the arm's last line is
`let _ = when;` (line 1420 — `grep -n 'let _ = when' convert.rs` finds exactly one hit, this
one; every other AUTO-adjacent path that computes a `when` attaches it to a `Grant { by, when
}`). `Effect::FactGrant(Fact)` (`src/rules_core/sheet_rule.rs:470-472`) carries no gate; only
`Grant { by: Granter, when: Applies }` (`sheet_rule.rs:428-431`) does, and `Grant` is RULE-level
(who may hold the whole rule), not effect-level (whether this one fact inside an already-held
rule applies). §3.2's claim "Conditional grants keep their `when` gate (`Grant.when`); the
engine … counts the grant only when the gate is decidable" is **false for AUTO proficiency
grants specifically** — the gate is thrown away at conversion time, before any engine ever sees
it. A reader built on the current §3.4 design would report a PRE-gated named-weapon or
archetype-only proficiency (Marksman's conjunction, Kensai's PRE-gated grant — both already
named in this document's own n=1 list, §3.5) as unconditionally granted: a fabricated fact on a
paper sheet, landing directly in `character_is_proficient_with`
(`feat_pillar_and_pool_aggregation.rs:2543-2550`), which feeds the attack line. This is the
exact hazard `docs/governance/no-stub-mvp-doctrine.md` forbids, so F1 cannot ship the reader
without the converter carrying the gate. See the amended 3.1(4), 3.2, 3.4, and F1's acceptance
table for the fix.

**0.2 The real defect: the LINK from class to its proficiency rule is lost.** The Wizard
proficiency rule has no `granted_by`. `data/sheet_rules/_defects/unresolved-references.json`
carries `core_rulebook:class_feature:wizard: Wizard Class Feature|Wizard ~ Weapon and Armor
Proficiency`. Cause: `resolve_holdable_rule` (`sheet_rule/prereq.rs:687-702`) calls
`ctx.resolve_rule(category, name)` (`sheet_rule/ctx.rs:324-334`), which looks up
`(category, key)` literally. "Wizard Class Feature" is a CHILD ability category whose parent is
"Special Ability" (oracle `core_rulebook/cr_abilitycategories.lst:40`:
`ABILITYCATEGORY:Wizard Class Feature ... CATEGORY:Special Ability`). The record is indexed
under the parent, so the lookup misses. Full breakdown in §1.

**0.3 A second, SILENT link defect exists (Wizard is the n=1).**
`core_rulebook:class_feature:wizard_class__d9affb050e701218` (the `CATEGORY:Class` "Wizard"
ability, oracle `cr_abilities_class.lst:108`) has `granted_by: null`, and NO defect row exists
for source `core_rulebook:class:wizard`. So the hop class -> class-ability is lost without a
defect being logged. Hypothesis (unverified): `(CLASS, WIZARD)` resolves through `by_cat_key`
to a different record (slug `wizard` collides: `class_feature/wizard.json` is the
FavoredClass ability from line 78), so the edge is written to the wrong target.
Upper bound: 41 of 136 chassis-bearing class records have no outgoing grant edge at all
(script in §1 notes; the 41 includes records whose class id is PI-redacted, where a
slug-keyed count cannot see the edge — so 41 is a ceiling, not a count).

**0.4 Hit die is already converted — for 178 of 185 class records, 7 named exceptions (review
finding 13, CONFIRMED, corrects the document's "every class principal rule carries" claim).**
Measured over all `data/sheet_rules/*/class/*.json` (185 files): **178 of 185** carry the
`{"family":{"StatBlock":"Hit die"},...}` prose row; the 7 without it are
`occult_adventures/psychic_detective`, `ultimate_psionics/gifted_blade`,
`ultimate_psionics/gifted_blade_marksman_power_list`, `ultimate_psionics/unlocked_talent`,
`bestiary/sorcerer_cleric_arcane`, `ultimate_intrigue/vwarlock`, `ultimate_intrigue/vcabalist`.
Four of these live in `CLASS_FAMILY_BOOKS` (`generic_class_chassis.rs:57-73`), so they can enter
the 78-record generic population and reach `Computed` with no hit die. A reader is needed, not
a converter change — moved to F0 as a prerequisite of F3 (review finding 3; was originally
placed in F4, too late for F3 which needs it first), returning `Option<u8>`: `None` for the 7
named exceptions, never a fabricated `0`.

> **F0-check fix (this cycle, finding 5):** `ClassChassis` did not carry this reader at F0's
> original close-out ("F0 closed" was declared while this named territory item was untouched —
> `git diff --stat` across all five F0 commits showed zero changes to
> `class_chassis_sheet_rules.rs`). Landed now: `pub hit_die: Option<u8>` on `ClassChassis`,
> read off the principal rule's `StatBlock "Hit die"` prose the same way `BaseAttack`/`BaseSave`
> are already read off `target`. Measured over the real chassis-bearing population (177 records
> — 185 total minus the 7 named exceptions above, MINUS an 8th, previously unnoted exception,
> `core_rulebook/monk`, whose principal rule degraded entirely to `value: Text` with no
> `BaseAttack`/`BaseSave` row either, so it was already absent from `records()` before this fix):
> every one of the 177 carries a real hit die, 0 named exceptions within the population `record()`
> actually returns. §2's territory list also names `pub skill_ranks_per_level: Option<u8>`,
> assumed to be converted the same way `hit_die` is; measured instead (`grep -rho
> '"StatBlock":"[^"]*"' data/sheet_rules/*/class/*.json | sort | uniq -c` → only `178
> "StatBlock":"Hit die"`, no other label at all, across all 185 class files): this repo's
> corpus carries NO such row today. The field and its reader are landed and real (same
> `stat_block_prose_text` helper, generalized to any label), but honestly return `None` for
> every record until a future converter change starts emitting it — see
> `docs/retro/events/sub-agent-f0-check-fix.jsonl`.

**The roster/census rule must not silently drop these
7 classes.** F4's roster rule (§6) omits a class from the Create picker only for a NAMED reason
(`hit_die_absent` | `not_computed` | `prestige` | `ex_state`); `hit_die_absent` is a distinct
reason from `not_computed` — a class can be `Computed` by the census (it has no chassis-blocking
diagnostic) and still be `hit_die_absent` (no HP figure to print), and the roster excludes it
under that name rather than reporting it `Blocked`. New F4 acceptance row (F4.5) and RED test
below.

**0.5 The 78 generic classes are 56 prestige + 22 base** (`docs/architecture/status.md:163-166`).
A naive `is_supported_generic_class_family_single_class` would report 56 prestige classes
ALONE as `Computed`. The arm must exclude `tags` containing `"Prestige"` (the converted record
carries it: `arcane_archer.json` -> `['PC','Prestige']`).

**0.6 No converter-backed WEAPON pin exists.**
`crates/codex-ingest/tests/rules_core_weapon_tables_via_converter.rs` is 56 lines and holds one
test, for ARMOR (`class_armor_proficiencies_match_their_own_corpus_records`). The 42 weapon
rows are hand transcriptions with no oracle pin. F1 adds the pin.

**0.7 Prestige entry requirements are already converted.** The class principal rule's
`applies` gate carries them in Codex vocabulary (Arcane Archer: `BaseAttack >= 6`, the three
feats, `HighestSpellLevel Arcane >= 1`). The fixture
`tests/fixtures/rules_core/prestige-class-entry-requirements.json` still holds raw `pre_tokens`
(test-side only, so legal). F0's carrier rule reads the converted gate.

**0.8 `data/sheet_rules` is byte-reproducible; `data/corpus` is the mutated one.** Verify
stage `sheet-rules-check` (`scripts/verify.sh:2518-2529`) runs
`cargo run --locked --quiet -j 2 -p codex-ingest --bin sheet_rule_convert -- --check`
(**correction, review finding 8**: the bin lives in the `codex-ingest` package, which is not a
default-run package at the workspace root; `cargo run --locked --bin sheet_rule_convert -- --check`
from repo root fails before doing anything —
`error: no bin target named 'sheet_rule_convert' in default-run packages` (verified: exit 101).
Every command in this document that invokes this bin now carries `-p codex-ingest -j 2`. Verified
green at HEAD `9d661623d3` with the corrected form: `records=49450 converted=49450 refused=0
rules=71862 var_tables=5309 verdict=PASS (121.2s)` — no post-hoc mutation exists in
`data/sheet_rules` today). The license/PI post-hoc hazard applies to `data/corpus`, which Epic F
must not regenerate. The converter bin also has `--one <id>` for n=1
(`crates/codex-ingest/src/bin/sheet_rule_convert.rs:21`; same `-p codex-ingest` prefix applies —
the bin's own doc comment omits it too, which is presumably how this got missed).

**0.9 Live reading of converted data has precedent and ships.**
`class_chassis_sheet_rules.rs` reads `data/sheet_rules/<book>/class/<slug>.json` at runtime;
`apps/desktop/src-tauri/tauri.conf.json:40` bundles `data/sheet_rules/` as a resource.
A live converted-rule engine already exists: `sheet_rule::held_set` / `render_sheet`
(`sheet_rule.rs:1891`, `:2039`), consumed by `feat_prereqs.rs:392`,
`level_up_option_filter.rs`, and the receipt (`class_shared_core.rs:29`).

---

## 1. The 11,925 unresolved references, by MECHANISM

Command: `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py` (classifies
each row of `data/sheet_rules/_defects/unresolved-references.json` against (a) every
`ABILITYCATEGORY:` line in the pinned oracle's `pathfinder/` tree — 2,384 child categories found —
and (b) the set of 66,237 `provenance.closure_rows` of all converted rules, so "target is a
converted record" means an oracle line that a converted rule actually cites).

| # | Mechanism | Rows | Fixed by parent-category map? |
|---|---|---|---|
| A | Reference names a CHILD ability category; the target IS a converted record under the parent category | **4,456** | **Yes — all** |
| B | Child category; target exists in the oracle but is not a converted unit | 63 | No (nothing to link to) |
| D | PLAIN category (2,911 of them `Special Ability`); target IS a converted record; resolver still misses | **3,033** | No — separate cause, NOT diagnosed (mostly monster / companion `X ~ Y` abilities; 177 have >1 oracle definition of the key) |
| E | Plain category; target exists in the oracle but in a book or file family that is not ingested (e.g. `Sorcerer Bloodline|Aberrant Bloodline`) | 3,565 | No (not ingested — correct as a defect) |
| F | Target found nowhere: bracketed `[Surprise Strike]`, comma-joined lists, case, parenthesised options with nested parens | 808 | No — 3+ small parser causes |
| | **Total** | **11,925** | 4,456 of 11,925 (37.4%) |

Sum check: 4456+63+3033+3565+808 = 11,925.

Mechanism A by source kind: class_feature 3,159; class 441 (46 distinct class records);
race_trait 349; ability 271; companion 203; monster_ability 26; feat 6; equipment 1.

Proficiency-related rows (regex `roficien|Weapon Prof|Armor Prof|Shield Prof`): 123 total of
11,925 = **99 in A**, 7 in D, 17 in F. (The 17 F rows are archetype/feat grants — Kensai, Spire
Defender, ARG archetypes — not base-class grants; they do not block any census class.)

**Sheet-visible effect today.** An unresolved reference becomes
`Holdable::MissingRule`, which evaluates Exclude (`sheet_rule.rs:1545`), and no `granted_by`
row is written on the target. So: (1) the target rule is never in the class's `held_set`;
(2) its `FactGrant`s (proficiency, class skills, languages) never reach the character's
facts; (3) `render_sheet` never prints it from the converted path. For the 31 tabled classes
the player still sees class features because `pilot_compute`'s bespoke per-class modules
print them from Rust tables — the converted path is NOT what prints them. That is why a
corpus-wide fix is risky: for 46 classes, 441 class->feature edges would suddenly be held and
printed by the converted path too, next to the bespoke path's own lines (possible duplicate
lines and changed snapshots in every pinned desktop fixture).

**Does the fix need a whole-package converter re-run? Yes.** Resolution is corpus-wide (the
index spans all books) and `granted_by` is written on the TARGET rule, which may be in any
book. There is no per-book scoping that is correct. Scope the EFFECT, not the run:

Safe procedure (one agent, own `git worktree` + own `CARGO_TARGET_DIR`, never the shared tree):
1. `cargo run --locked --quiet -j 2 -p codex-ingest --bin sheet_rule_convert -- --check` at HEAD
   -> must be green (verified green, see 0.8). Red = a post-hoc mutation exists in
   `data/sheet_rules`; STOP and escalate.
2. Record `python3 -c "import json;r=json.load(open('data/sheet_rules/_report.json'));print(r['records'],r['converted'],r['rules_written'])"`
   -> `49450 49450 71862`. `records`/`converted` are INPUT counts; the fix must not move them.
3. n=1: apply the resolver change; `cargo run --locked -j 2 -p codex-ingest --bin sheet_rule_convert -- --one core_rulebook:class_feature:wizard_class__d9affb050e701218`
   and the same with `--one core_rulebook:class_feature:wizard_weapon_and_armor_proficiency`; time both;
   confirm the `granted_by` row appears.
4. Full run in the worktree. Then a STRUCTURAL diff (scratch script): same file set, same
   rule-id set, and for every rule the JSON minus `granted_by` is byte-identical. Allowed
   deltas: `granted_by` additions; `_defects/unresolved-references.json` shrinks;
   nothing else. Any other delta = STOP.
5. Do NOT bump `converter_version` in the same commit (it is stamped into all 71,862 rules
   and would bury the semantic diff). Bump in a separate mechanical commit.
6. `data/corpus/**`, `site/status-data.json`, `scripts/gen-corpus-bundle.mjs` output: untouched.
   Gate: `git status --porcelain -- data/corpus site | wc -l` -> 0;
   `python3 scripts/site/check_frozen_status.py --check`; `python3 scripts/pcgen_residue_gate.py --check --closure`.

The frozen **49,450** is a record (input-unit) count. Adding `granted_by` rows to existing
rules does not add a record. Adding a new record KIND would — see §3 (F1) on selectors.

---

## 2. F0 — Permanent census instrument (RED first)

**Files (territory):** new `src/rules_core/class_census.rs` (public; the one merged registry);
`src/rules_core/pilot_compute/generic_class_chassis.rs` (add `pub(crate) fn covered_classes()`);
`src/rules_core/pilot_compute/mod.rs` (re-export); new `src/bin/class_census.rs`;
new `scripts/gen_class_status_table.py` + `scripts/tests/test_gen_class_status_table.py`;
`scripts/verify.sh` (stage `class-census`, both stage sets); `scripts/verify-baselines.env`;
`src/rules_core/pilot_compute/class_chassis_sheet_rules.rs` (review finding 3, moved here from
F4: `pub hit_die: Option<u8>` and NEW `pub skill_ranks_per_level: Option<u8>`, both parsed from
the converted record's `StatBlock` prose rows, both `None` when the row is absent — F3 and F4
each read the resulting `Option` and report `Unknown` / omit-from-roster rather than fold `None`
as zero; see F3's new F3.0 acceptance row).
`src/bin/v06_class_state_dump.rs` and stage `class-dump` stay untouched (31 of 31 keeps its gate).

**Registries merged** (same list the deleted `zz_class_census` used, `status.md:93-105`):
`ClassId/ApgClassId/AcgClassId/PuClassId`, `UcClassId::ALL`,
`untabled_base_class_registry()`, `crb_untabled_class_chassis::covered_classes()`,
`generic_class_chassis::covered_classes()`, plus every `class:<slug>` whose converted record
is tagged `Prestige` in the 38 books (replaces the fixture as the prestige list; the fixture
stays a test oracle and a parity test asserts the two lists are equal, 74 of 74).

**Sweep.**
- Base class: alone, level 1..=max_level, fixed race (as today), canonical seeds.
- Prestige class: NEVER alone for the Computed column. One deterministic mix per class.
  **Carrier rule, fully specified (review finding 14, CONFIRMED — the prior two-clause version
  had no stated precedence and made a prestige row's Computed status an artefact of the carrier,
  not the class).** Verified over the 74 prestige ids (77 tagged records before the 3
  cross-book slug dedupes the census already performs): 23 carry a `BaseAttack`/total-AB
  requirement, max value 7 (`fighter 7 + max_level 10 = 17 <= 20` — the cap is never actually
  reached by a BAB term alone, so this axis is safe as originally written); `mystic_theurge` and
  `evangelist` each carry BOTH an Arcane and a Divine `HighestSpellLevel` term; 43 carry
  neither a caster nor a BAB term at all (their carrier falls to the floor-5 `fighter`, who has
  no caster level — any prestige chassis expression keyed to caster level would then evaluate 0,
  a confidently wrong number in a row the census calls Computed, not an artefact of a real
  build).
  > **`scripts/retro.py` correction (F0-check fix, this cycle):** the 23/43/66/2/6 figures above
  > were measured against the carrier rule AS ORIGINALLY WRITTEN, which recursed into
  > `Applies::Not`/`Applies::AtLeast` when scanning for a caster-kind mention -- an ungrounded
  > mention inside either (a NEGATION or one OPTIONAL alternative among several) got counted the
  > same as a mandatory, positive requirement. Fixed to scan only mandatory, positive top-level
  > gate terms; a caster signal (`HighestSpellLevel`/`CasterLevel`) reachable ONLY through
  > `Not`/`AtLeast` now reports `Unknown` instead of a wrong carrier. Re-measured split: 55
  > fighter, 6 wizard-only, 5 cleric-only, 1 dual-caster (`mystic_theurge` only -- `evangelist`'s
  > dual-caster clause is one optional `AtLeast` alternative, not two independent mandatory
  > terms), 7 Unknown (`dragon_disciple`, `evangelist`, `pure_legion_enforcer` -- the finding's
  > own three named defects -- plus `dark_tempest`, `elocater`, `psion_uncarnate`, `thrallherd`,
  > each carrying a mandatory `HighestSpellLevel(Any)` term this two-carrier model cannot ground
  > to either Arcane or Divine). See `docs/retro/events/sub-agent-f0-check-fix.jsonl` and
  > `src/rules_core/class_census.rs`'s `determine_carriers`/`gate_has_any_caster_signal`.
  - carrier = `wizard` if the converted `applies` gate contains `HighestSpellLevel "Arcane"` (and
    not `"Divine"`); `cleric` if `"Divine"` (and not `"Arcane"`); else `fighter`.
  - **Dual-caster case (`mystic_theurge`, `evangelist` — both Arcane AND Divine terms):** a
    single-class carrier cannot ground both. Use a **second, independent carrier**: report TWO
    mixes for the row, `[wizard N, <prestige> M]` and `[cleric N, <prestige> M]`, and the census
    column `carrier` (new, below) names both; the row is Computed only when BOTH mixes reach
    Computed (so a Computed verdict on a dual-caster prestige class is never grounded in only
    half of what it legally needs).
  - carrier level = smallest level meeting every NUMERIC requirement in the gate
    (`BaseAttack >= n`, skill ranks n -> level n, spell level L -> caster level 2L-1), floor 5.
  - **Precedence when the cap bites (previously unstated): the `carrier + prestige max_level <=
    20` cap always wins.** If the smallest level meeting every numeric requirement would push
    the mix over 20, the carrier level is capped at `20 - prestige max_level` instead, and the
    now-unmet numeric term (there is always at least one once the cap has bitten) is listed in
    `entry_gate: unmet` with its required value AND the value the capped carrier actually
    reaches — this is a real, printed shortfall, not a silently-dropped requirement, and it
    counts the same as any other unmet entry-gate term (2, below): non-blocking, printed, never
    simulated as met.
  - prestige levels swept 1..=max_level with the carrier fixed.
  - Requirements the carrier cannot meet (feats, alignment, deity, race, "special", or a numeric
    term the cap left unmet per the precedence above) are reported in a column
    `entry_gate: met|unmet|partially-met` with every unmet term listed. `partially-met` is used
    only for the dual-caster case when one of the two independent-carrier mixes fails its own
    entry gate while the other succeeds — named explicitly, never folded into a single verdict.
    They do NOT count against Computed: the entry gate is printed rule text and is already
    non-blocking by design (`prestige_entry_gate.met|unmet|partially-met`). Paper-sheet
    doctrine: print, do not simulate.
  - **New census column `carrier`** (review finding 14): names the carrier class(es) used for
    each prestige row (`wizard` | `cleric` | `fighter` | `wizard+cleric` for the dual-caster
    case). **New test:** any prestige row whose chassis expression (the converted record's
    `value`/effects, read the same way F1's reader already reads them) references a caster
    level (`Expr::CasterLevel` or a `HighestSpellLevel` read) must have a caster carrier
    (`wizard`, `cleric`, or both) named in that column; if it does not (e.g. a future prestige
    record this rule cannot classify), the row reports `entry_gate: unknown` and
    `status: Unknown` for that class — **never a confidently-wrong 0** the way the un-fixed
    43-of-74 floor-5-fighter case would produce today. RED test:
    `a_prestige_row_referencing_caster_level_names_a_caster_carrier_or_reports_unknown`.
- Second column `alone_status`: every prestige class alone must be `Blocked` with the F2
  game-rule diagnostic (74 of 74). This is a negative control, not an exclusion.
- Mix panel: the existing multiclass negative-control mixes, re-used as census rows, each with a
  histogram of claim-blocking diagnostic ids. This is the measurement F3 needs before any F3
  code is written. **The input count is measured here, not asserted as 183** (review finding 8,
  detailed in §5): no file in the repo states 183; F0d derives it mechanically
  (`scripts/extract_multiclass_census_panel.py`) from the reproducible command sequence §5
  specifies — **measured 2026-09-21: 185** (88 `tests/sd18_widening/` + 84
  `tests/sd13_progression/` + 13 hand-written top-level files; see
  `scripts/verify-baselines.env`'s `BASELINE_CENSUS_MIX_COMPUTED` entry for the exact commands)
  — and records that command next to the number.

**Output:** `--json <path>`: per class {family, book, max_level, status per level,
blocking-diagnostic ids, entry_gate, alone_status, in_desktop_roster}. Stage fails if any
count falls below its baseline. Baselines can only rise:
`BASELINE_CENSUS_IDS=135`, `BASELINE_CENSUS_COMPUTED=42`,
`BASELINE_CENSUS_MIX_COMPUTED=185` (measured at F0d, by the §5 command sequence, review finding 8
— never guessed or copied from this document's own prose; provenance in
`scripts/verify-baselines.env`),
`BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED=74`.

**Final acceptance number for the epic: 135 of 135** (61 base-type ids alone at every level +
74 prestige ids in their canonical mix at every prestige level), **mix panel
`BASELINE_CENSUS_MIX_COMPUTED`=185 of itself** (review finding 8: the figure is measured by F0d's
command sequence in §5, not hardcoded here as 183 — no source in the repo states 183; F0d's own
sweep of all 185 rows against the shared canonical fixture measured 185 of 185 Computed, 0
Blocked — see `docs/release/SD-36-consolidation/artifacts/epic-f/mix-panel-histogram.md`),
prestige-alone Blocked 74 of 74. **Classes excluded by a game rule: none.** Ex-Barbarian,
Ex-Paladin (and `ex_antipaladin` if the census finds it a distinct id — then the 135 rises and
the correction is logged) are legal sheet states and stay counted. If the measured denominator
is not 135 the agent records a `scripts/retro.py correction`, never a silent change.

| Criterion | Acceptance command |
|---|---|
| F0.1 RED truth (review finding 12d: pinned, not self-authored) | `cargo run --locked --bin class_census -- --json /tmp/census.json` prints `ids=135 computed=42 blocked=19` (42 of 61 non-prestige computed, 19 of 61 non-prestige blocked — an F0-check fix corrected `blocked` from `ids - computed`, which folded the 74 never-swept prestige ids into "blocked" and printed a false `blocked=93 of 135`), AND `cargo test --locked -j 2 --lib class_census::tests::census_id_set_matches_the_published_partition -- --exact` green with a non-zero test count (the bare filter without `-- --exact` previously matched and silently passed 0 tests): the merged id set equals `status.md`'s own partition 31+3+20+7+74, and `computed==42` is asserted against that partition at F0 landing time — the instrument is pinned to the previously published census before it is allowed to move, not left to author its own denominator in the same batch that reads it |
| F0.2 stage (review finding 10: the original command cannot pass for a correctly registered stage) | `bash scripts/verify.sh --list` prints one row per stage headed `stage  full  quick` (verified: 51 rows today, e.g. `sheet-rules-check    yes   no`; `bash scripts/verify.sh --list \| grep -c class-dump` -> 1, not 2 — a correctly registered stage appears once, with `yes`/`yes` in its two columns, never twice). `bash scripts/verify.sh --list \| grep -E '^class-census +yes +yes'` prints the row (asserts membership in BOTH stage sets by the columns, not a row count); `bash scripts/verify.sh --only class-census` green |
| F0.3 generated table | `python3 scripts/gen_class_status_table.py --check` exits 0; `python3 -m pytest scripts/tests/test_gen_class_status_table.py -q` green |
| F0.4 list parity | `cargo test --locked --lib class_census` green (prestige list == fixture's 74 of 74; no id in two families) |

RED-first tests: `class_census::tests::every_registry_is_swept_once` (135, partition sums
31+3+20+7+74=135), `census_id_set_matches_the_published_partition` (review finding 12d),
`prestige_carrier_is_deterministic`, `a_prestige_row_referencing_caster_level_names_a_caster_carrier_or_reports_unknown`
(review finding 14), `prestige_alone_is_blocked_with_the_game_rule`
(RED until F2). Risks: carrier rule meets a requirement shape it cannot parse -> row reports
`entry_gate: unknown` and the test lists every such class by name (no silent default).
**Amendment (option A):** the bin also takes `--sheet-dump <dir>` and `--only <class>` — the headless
whole-character render F1b needs (§3b.3). F0 is therefore a hard prerequisite of F1b.
Also a hard prerequisite of F3 (review finding 3): F3 cannot compute HP or skill points without
`hit_die` and `skill_ranks_per_level` on `ClassChassis`, both added here.
**Size: 10-13 agent-hours** (was 8-10; +2-3 for the two `ClassChassis` readers moved in from F4
per review finding 3). **Full verify passes: 0** (scoped: `--lib`, the bin, the new stage).

**F0d landed (2026-09-21): the mix panel, measured.** Delivered:
`scripts/extract_multiclass_census_panel.py` (mechanical extractor, + unit tests in
`scripts/tests/test_extract_multiclass_census_panel.py`), the committed
`tests/fixtures/rules_core/multiclass_census_panel.json` (185 rows), `MixPanelRow`/
`load_mix_panel`/`sweep_mix_panel_row`/`sweep_mix_panel`/`mix_panel_blocking_histogram` in
`src/rules_core/class_census.rs`, the `--json` output's new `mix_panel*` fields in
`src/bin/class_census.rs`, and two new RED-first tests:
`mix_panel_size_matches_the_measured_multiclass_negative_control_count` (the sync test this
section calls for — re-derives 88+84+13=185 fresh by grep on every run, never a bare pinned
literal alone) and `sweep_mix_panel_row_reports_a_real_status_and_dedupes_blocking_ids`.
Measured result: **185 of 185 mix-panel rows reach Computed, 0 Blocked**, under the census's own
shared canonical fixture — see
`docs/release/SD-36-consolidation/artifacts/epic-f/mix-panel-histogram.md` for the finding and
why it differs from each negative-control test's own (Blocked, but against a different,
individually-tuned) fixture. Committed artifact:
`docs/release/SD-36-consolidation/artifacts/epic-f/census-f0d.json`.

---

## 3. F1 — Link repair corpus-wide (OPTION A, ruled 2026-09-21) + class weapon proficiency

**Ruling:** Decision 1 = A. Fix ALL 4,456 parent-category links in this bundle, not only the
99 proficiency rows. F1 is therefore two things: (i) the corpus-wide link repair in the
converter, (ii) the proficiency reader that the repaired links feed. The print-path work the
ruling makes necessary is its own batch, **F1b (§3b)**, and F1's population run may not
be committed before F1b's n=1 and n=5 classifications exist.

**Where conversion happens:** `crates/codex-ingest/src/pcgen_import/sheet_rule/`
(`convert.rs` AUTO arm 1390-1410; `prereq.rs:687` `resolve_holdable_rule`; `ctx.rs:324`
`resolve_rule`; `closure.rs` for `.MOD`/`.COPY` row merging). Oracle constructs that carry a
class's grants (all seen in the pinned oracle): class-line `ABILITY:Class|AUTOMATIC|<Class>`;
`CATEGORY=Class|<Class>.MOD` rows granting `<Class> Class Feature|<Class> ~ <Feature>`, gated
by an archetype-off variable; `ABILITY:Internal|AUTOMATIC|Weapon Prof ~ Simple|Martial|Auto`
indirections; direct `AUTO:WEAPONPROF|<names>|TYPE=<selector>`; PRE-gated named grants;
`%LIST` choices.

### 3.1 The converter change (three parts, all generic)

1. **Parent-category resolution.** `resolve_rule(category, name)`: when `(category, key)`
   misses and `category` is a child `ABILITYCATEGORY`, retry with the parent. The child ->
   parent map comes from the `ABILITYCATEGORY` rows already in the closure tree (2,384 child
   categories in the pinned oracle; `unres2.py`). Closes mechanism A = **4,456 rows**:
   class_feature 3,159; class 441 (46 class records); race_trait 349; ability 271;
   companion 203; monster_ability 26; feat 6; equipment 1. A child category may also narrow
   by `TYPE:`; the retry must keep KEY-exact matching, so a parent hit is the same record the
   oracle tool would pick (never a name-similarity guess).
2. **The silent class -> class-ability miss (0.3).** Diagnose on Wizard first
   (`core_rulebook:class_feature:wizard_class__d9affb050e701218` has `granted_by: null` and no
   defect row). Whatever the cause, the converter must never drop an edge without a defect
   row: add an invariant test "every `ABILITY:*|AUTOMATIC|` grant on a converted record yields
   either a `granted_by` row on some rule or a `_defects` row" (count in = count out).
3. **Remaining no-proficiency base classes.** A simulation of the post-A graph
   (`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure.py`; approximate:
   `when` gates ignored, target matched by first closure row) says a weapon grant is reachable
   for **59 of 136** chassis class records after A (39 of 136 before). Of the 77 of 136 that
   still reach none, 66 are prestige (many genuinely grant nothing) and **11 are base classes
   that PF1 certainly arms**: Wizard, Psion (closure size 2 = the silent miss), Magus,
   Kineticist, Occultist, Psychic, Vigilante, Cavalier, Oracle, Summoner, Antipaladin. Option A
   alone does NOT close these. Each must be traced to a MECHANISM (expected: the silent miss,
   plus grants that sit on a rule gated by a variable the class-level facts cannot decide) and
   fixed generically in the same pass. The static 42-row table hides this for 9 of the 11
   today; the F1.2 parity test exposes it.

Mechanisms D (3,033), E (3,565), F (808) and B (63) are NOT in ruling A's scope. They go to
`forward-scope-register.md` with §1's counts — EXCEPT any row that sits inside a class closure
and keeps `closure_complete` false (§3b.5 lists them): those are Epic F work.

4. **Carry the AUTO grant's gate onto the effect (review finding 1, new required sub-task).**
   Today's `let _ = when;` (`convert.rs:1420`) must become real carriage. Two shapes considered:
   (a) add `Effect::GatedFactGrant { fact: Fact, when: Applies }` alongside `FactGrant` and make
   `held_set`'s effect-application step (fixpoint step 3, `sheet_rule.rs`) evaluate `when`
   against the same `CharacterFacts`/`Applies` machinery `Grant.when` already uses before
   folding the fact in; or (b) split a gated AUTO row onto its own sibling rule (`<id>#gated-N`)
   whose own `applies` holds the gate, so `held_set`'s existing rule-level gating (which already
   works) filters it without a new effect variant. (a) is smaller (no new rule proliferation,
   no siblings-of interaction with review finding 7's blast-radius math) and is the chosen
   design; (b) is the fallback if `Applies` cannot be evaluated at effect-application time for a
   reason not yet found. Either way this sub-task is scheduled INSIDE F1, before the population
   run (F1's n=1 list already includes Marksman for exactly this reason). Until it lands, the
   reader (§3.4) must return `Unknown`, never `Some(set)`, for any class whose closure contains
   an AUTO row with a non-trivially-true gate (i.e. `when != Applies::Always`) — never silently
   drop the gate to ship on schedule.

### 3.2 Schema (Codex vocabulary; mostly exists)

`ProfRef::Weapon(name)`, `WeaponGroup(tag)` (tier or group), `Chosen(choice)`,
`DeityFavoredWeapon`. Two additions:
- `ProfRef::WeaponAllOf(Vec<Tag>)` for conjunctive selectors (`Martial` AND `Ranged`). Today
  the converter emits the joined string `"Light.Martial"`; that is a lossy word, not a list.
- `ProfRef::WeaponSet { label: Tag, members: Vec<String> }` for membership selectors (3.3).
**Correction (review finding 1):** `Grant.when` gates whether the character may hold the RULE at
all; it does not gate an individual `Fact::Proficiency` grant living inside that rule's effects
— those are unconditional once the rule is held. The AUTO conversion path (3.1 item 4) currently
drops its own PRE-gate entirely rather than routing it through `Grant.when`, because the gate is
per-fact (which named weapon), not per-rule. The fix is the new `Effect::GatedFactGrant { fact,
when }` from 3.1 item 4: the engine evaluates `when` against `CharacterFacts` when folding the
effect (fixpoint step 3, not step 2's rule admission) and prints the gate words on the line only
when the gate is decidable from the input; an undecidable gate excludes the fact, never
approximates it as granted.

### 3.3 Weapon-group membership WITHOUT a new record kind (katana / `Samurai`)

Membership lives on the oracle's weapon-PROFICIENCY rows (`uc_profs_weapon.lst:100`
`Katana.MOD TYPE:Samurai.HeavyBlade`), which are not one of the 19 converted kinds
(`_report.json by_kind`), and the converted `equipment/katana.json` tags lack `Samurai`.
A `weapon_proficiency` kind would move the frozen 49,450. Proposal: **resolve membership at
ingest, store it on the GRANTING rule.** The converter already holds the whole oracle tree in
`closure.rs`; add a read-only index `proficiency name -> TYPE tags` built from
`*_profs_weapon.lst` rows plus their `.MOD` rows (closure-only input, never a record). When an
`AUTO` weapon selector is not one of the three tiers and not a `Weapon Group <x>` tag already
on equipment rules, expand it:
`{"Proficiency":{"WeaponSet":{"label":"Samurai","members":["Katana","Naginata","Wakizashi"]}}}`
(members are whatever the pinned rows say — the three named here are an **estimate** from PF1,
to be replaced by the converter's output). Live matching then needs no selector logic: a set is
a named list. Tier selectors match `WeaponTableEntry.proficiency`; `Weapon Group` selectors
match `weapon_group`; `WeaponAllOf` matches tier AND `is_melee`/`is_ranged`; a conjunct the
weapon record cannot answer (`Light`, `Thrown`) is ALSO expanded to a `WeaponSet` at ingest.
Record count stays 49,450; the delta is inside existing rules' `grants`.
Pin: a converter-backed test re-derives each `WeaponSet.members` from the oracle rows.

**`Auto` and `KoboldTailAttachment` are real, resolvable selectors, not junk (review finding 15
CONFIRMED as originally written; SUPERSEDED by F1 adversarial finding 1 and the F1 re-check round
1 spec-divergence finding — ruling below).** The vocabulary the reader consumes was originally
measured to contain `Auto` (11 occurrences) and `KoboldTailAttachment` (2 occurrences) alongside
the expected joined conjunctions (`Light.Martial`, `Martial.Ranged`, `Martial.Thrown`,
`OneHanded.Simple`, ...), and this section first called both junk — neither a PF1 weapon tier
nor a weapon group. That call was wrong: both tags name real, live oracle members. `TYPE=Auto`
selects five live weapon-proficiency rows (`cr_profs_weapon.lst:10-14` — Grapple, Ray Spells,
Touch Spells, Splash Weapon, Unarmed Strike, PF1's universal proficiencies, each tagged
`TYPE:Auto....`); `TYPE=KoboldTailAttachment` selects one (`arg_profs_weapon.lst:22`,
`Kobold Tail Attachment  TYPE:Melee.Light.Natural.KoboldTailAttachment`). A converter carve-out
that special-cased these two names in front of the new membership machinery (F1 adversarial
finding 1) would have silently discarded both grants instead of resolving them, and did so for
one commit on this branch before being removed; the ruling stands: both resolve through
`WeaponMembershipIndex::members_with_all` like every other non-tier selector, no exclusion list.
F1.2 as originally written pinned only the 42 hand-typed static rows, leaving every OTHER reader
row — exactly the new surface F1 adds — unpinned regardless of this ruling. Fix scheduled inside
F1.2 below: every `ProfRef` the reader returns for a census class must be re-derivable from a
named oracle row (a tier, a `Weapon Group <x>` tag, or an expanded `WeaponSet`); a tag matching
NONE of the three shapes — genuinely member-less against the pinned oracle, which `Auto` and
`KoboldTailAttachment` are not — makes that class's proficiency answer `Unknown`, never a
fabricated membership. RED test: `an_unrecognized_proficiency_tag_makes_the_class_unknown`,
against a selector with no real oracle member (a synthetic/fabricated tag in the test fixture,
since a full-corpus measurement after this ruling landed found 0 of 49,450 converted records
carry a genuinely member-less `TYPE=` selector — `_defects/unrecognized-proficiency-tag.json` is
empty on a fresh `--dump`), never against `Auto` or `KoboldTailAttachment`.

### 3.4 Live consumption — read the converted record; do NOT add ~93 Rust rows

- Ruling 7 forbids MOVING `rules_tables` to data before Starfinder; it does not require new
  data to be authored as Rust. Precedent 0.9.
- Ruling 4: 93 hand-typed rows are the fabricated-row hazard; the existing 42 have no oracle
  pin (0.6).
- Residue gate: converted vocabulary has zero oracle tokens; the gate is 0 of 0 on it today.
- Note from the code: `held_set` applies only `FactDeclare`, `CountsAs`, `Waives`, `Revokes`
  effects (`sheet_rule.rs` fixpoint step 3). It does NOT fold `FactGrant` into
  `CharacterFacts` (`CharacterFacts::from_character` leaves `proficiencies` empty). So the
  reader collects `Effect::FactGrant(Fact::Proficiency)` itself from the held rules.
- Mechanism: new `src/rules_core/pilot_compute/class_proficiency_sheet_rules.rs` (sibling of
  `class_chassis_sheet_rules.rs`). For `(class slug, class level)`: build a `HeldSeed` with
  that one class, run `held_set` (which already honours `Granter::Class { at_level }` and
  `Granter::Rule`, `sheet_rule.rs` fixpoint step 2), collect proficiency grants from the held
  rules. Returns `ClassWeaponProficiencyView {tiers, named, groups, all_of, sets,
  printed_conditions}`, cached per `(class, level)`. `weapon_tables::class_weapon_proficiency`
  keeps its 42 static rows (first precedence); `character_is_proficient_with`
  (`feat_pillar_and_pool_aggregation.rs:2543-2550`) falls back to the reader.
- **Known-empty vs unknown** (no fabricated "proficient with nothing"): the converter writes
  a per-class boolean `closure_complete` on the class rule. It is true iff no rule in the
  class's grant closure has an unresolved reference whose oracle target (transitively, read
  from the oracle tree at ingest) carries a weapon-proficiency grant, and no unfindable
  (mechanism F) reference at all. The reader returns `Some(empty)` only when it is true.
  Live code never reads `_defects/`.
- **Gated grants (review finding 1).** `closure_complete` alone is not sufficient: a closure can
  be complete (every reference resolves) and still contain an AUTO row whose gate was dropped at
  the OLD (pre-3.1-item-4) conversion. Until 3.1 item 4 ships, the reader treats any class whose
  closure contains an AUTO proficiency row with `when != Applies::Always` in the ORACLE (i.e. a
  PRE-gated or archetype-conditioned grant) as `Unknown`, not `Some(set)` — checked from the
  oracle gate text at ingest time (same closure walk `closure_complete` already does), not from
  the (currently gate-less) converted effect. Once 3.1 item 4 lands, `Effect::GatedFactGrant`
  carries the gate itself and this special case is removed; the reader then folds gated facts
  normally (grant included only when `when` evaluates Include, printed with its condition text
  when it evaluates Include-with-caveat, excluded — never assumed — when undecidable).

### 3.4a Package handle for the reader (review finding 5 — CONFIRMED, new required sub-task)

The 3.4 reader is specified as a `pilot_compute` module that builds a `HeldSeed` and runs
`held_set` over a `SheetRulePackage` — but `compute_pilot_base_chassis` (`class_shared_core.rs:1698`)
and `build_pilot_headless_receipt` (`:1680-1695`, which calls only `compute_pilot_base_chassis`)
are the PURE functions every suite in the repo and the F0 census bin call, and neither has a
package in hand: `sheet_lines` on the returned struct is documented "Empty from
`compute_pilot_base_chassis` itself, which has no `data/sheet_rules/` package in hand"
(`class_shared_core.rs:28-32`), and `with_sheet_rules` — the only thing that loads one — has
exactly ONE production caller today, `apps/desktop/src-tauri/src/character_hub.rs:750`
(verified: `git grep -n with_sheet_rules -- src apps` -> one production hit plus three test-only
hits in `sheet_rule.rs`). The blocking diagnostic the proficiency reader must clear is raised
from `character_is_proficient_with` (`feat_pillar_and_pool_aggregation.rs:2543-2550`), inside
that same package-less pure-function path. Making F1.5 pass therefore requires the census (and
every other caller of `build_pilot_headless_receipt`, not just the desktop) to load the package.

**Precedent exists, but one layer too high.** The desktop ALREADY does this:
`character_hub.rs:723-736`'s private `fn sheet_rule_package()` is a process-wide `OnceLock`
around `corpus_loader::load_sheet_rules`, with a named `Err` fallback ("no sheet rules under
{dir} … regenerate with `cargo run --locked --bin sheet_rule_convert`") rather than a silent
empty package. That pattern is desktop-crate-local (keyed off
`authoring_workbench::codex_repo_root()`) and is not reachable from `rules_core`'s own pure
functions or from the `class_census` bin, which is not a desktop-crate binary.

**Fix:** add a rules_core-level, process-wide lazily-loaded package handle — same shape as
`class_chassis_sheet_rules.rs`'s own `static CACHE: OnceLock<RecordCache>` (`:277`), keyed off
`crate::support::paths::repo_root()` (the same root `class_chassis_sheet_rules.rs` already
uses, no desktop dependency) — exposed as a function the proficiency reader (3.4) and
`build_pilot_headless_receipt` can call, returning `Result<&'static SheetRulePackage, String>`
with the same "named reason, never a silent empty package" contract the desktop's version
already has. `build_pilot_headless_receipt` calls it and, on `Err`, reports every reader that
needs it `Unknown` (never `Some(empty)`) with the reason attached — never a hard panic, since
every existing test and every existing caller of `build_pilot_headless_receipt` must keep
working even before this handle's first successful load in a given process.
**Before this sub-task is marked done:** measure and record (a) first-load wall time of the
~54,775-file `data/sheet_rules/` package from a cold OS cache and a warm one, (b) the resident
memory the loaded `SheetRulePackage` holds for its 71,862 rules, both from a real run, not an
estimate — and re-estimate F1's hours with this task in scope (folded into the size figure
below). **Roster question the reviewer raised, answered:** F4's roster (§6) calls
`list_class_creation_roster`, which is desktop-crate code and therefore already has the
desktop's own `sheet_rule_package()` OnceLock available (character_hub.rs:723) — it does NOT
need to call the census (135x20 receipts) at picker-open time; it reads the (much smaller)
per-process cached package handle plus the already-cheap `ClassChassis`/proficiency-reader
calls, same cost shape `sheet_lines_for` already pays today for every open character. The
census bin is the only caller that sweeps all 135 classes, and it pays the package load once
per process, not once per class.

### 3.5 Run, scope and diff

Procedure = §1's six steps (own worktree, `--check` green first, structural diff that
allows only `granted_by` additions + the new `grants`/`closure_complete` fields on the rules
3.2-3.4 touch, `records 49450 -> 49450`, no `converter_version` bump in the same commit,
`data/corpus` and `site/` untouched). Under option A the diff is LARGE by design: expect up to
4,456 new `granted_by` rows. The structural diff script therefore also emits a per-kind count
of added edges, which must equal mechanism A's per-kind table (3.1) or the difference is
explained row by row.

**n=1 measurement:** Wizard (both defects), then Samurai (selector), Marksman (conjunction +
PRE-gated named weapon), Commoner ("one simple weapon" choice). Record wall time of `--one`,
of the full converter run, of the structural diff, BEFORE the population run. Then F1b's
n=1 / n=5 sheet diffs. Only then the population commit.

| Criterion | Acceptance command |
|---|---|
| F1.1 reproducible before | `cargo run --locked --quiet -j 2 -p codex-ingest --bin sheet_rule_convert -- --check` exits 0 at the pre-change commit |
| F1.2 oracle pin for the reader (extended, review finding 15; ruling corrected by F1 adversarial finding 1 and the F1 re-check round 1 spec-divergence finding) | `cargo test --locked -p codex-ingest --test class_weapon_proficiency_via_converter` green: reader output == each of the 42 static rows, AND every reader row for every census class re-derived from a named oracle row (tier, `Weapon Group <x>`, or expanded `WeaponSet` — not only the 42 static rows), AND `an_unrecognized_proficiency_tag_makes_the_class_unknown` green (a class whose closure carries a `TYPE=` selector matching none of the three shapes against the pinned oracle — a genuinely member-less tag, proven with a synthetic fixture, since the live corpus carries none — reports `Unknown`, never a fabricated proficiency; `Auto` and `KoboldTailAttachment` are NOT such tags — both resolve to real oracle members, `cr_profs_weapon.lst:10-14` and `arg_profs_weapon.lst:22`, and must convert like any other selector) |
| F1.3 links closed (per-edge pin, review finding 12a) | `python3 -c "import json;print(len(json.load(open('data/sheet_rules/_defects/unresolved-references.json'))))"` -> 11,925 - 4,456 = **7,469** (or the difference explained per row); `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py` reports mechanism A = 0 AND D/E/F unchanged at 3,033/3,565/808 (a drop in D/E/F means the retry stole rows from a different mechanism, not a real fix); AND for each of the 4,456 added edges, a per-edge correctness assertion that the TARGET rule's `provenance.closure_rows` contains the oracle line the reference names — a count match alone (4,456 edges added) is satisfied equally by an edge written to the right record and one written to a colliding record of the same slug (0.3's own hypothesis: `wizard` collides across `class_feature/wizard.json` and two `wizard_class__<hash>.json` records), so the count is necessary but not sufficient and the per-edge pin is required to close the row |
| F1.4 no silent drop | `cargo test --locked -p codex-ingest automatic_grants_are_never_dropped_silently` green |
| F1.5 coverage | census: classes blocked on `combat.baseline_weapon_proficiency_unknown` -> 0 of 135 |
| F1.6 structural diff | diff script: `unexpected field deltas: 0`, `records 49450 -> 49450`, added edges per kind == 3.1's table |
| F1.7 gates | `python3 scripts/pcgen_residue_gate.py --check --closure` 0 of 0; `python3 scripts/site/check_frozen_status.py --check`; `--check` green after; `git status --porcelain -- data/corpus site \| wc -l` -> 0 |
| F1.8 gate carried, not dropped (review finding 1) | `cargo test --locked -p codex-ingest a_pre_gated_weapon_proficiency_is_not_granted_unconditionally` green (Marksman conjunction + PRE-gated named weapon, and Kensai archetype-only grant: reader reports the gate condition, never an unconditional grant); `grep -n 'let _ = when' crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs` -> 0 hits |
| F1.9 package handle (review finding 5) | `cargo test --locked --lib rules_core::sheet_rule_package` (or its chosen name) green: returns `Ok` when `data/sheet_rules/` is present, a named `Err` (never panic, never silent empty) when it is not; `cargo test --locked --lib build_pilot_headless_receipt_reports_unknown_when_package_absent` green; load-time and memory figures recorded in the F1 receipt |

RED first: F1.2 (fails: reader absent); F1.4 (fails today on Wizard);
**F1.8 `a_pre_gated_weapon_proficiency_is_not_granted_unconditionally`** (fails today: Marksman's
PRE-gated named weapon reads as unconditionally granted because `convert.rs:1420` discards the
gate — review finding 1; this test must be authored and RED before 3.1 item 4 is touched, and
green only once `Effect::GatedFactGrant` ships);
`a_child_ability_category_resolves_through_its_parent`; `wizard_is_not_granted_the_simple_tier`;
`samurai_set_members_come_from_the_oracle_rows`; `a_class_with_an_incomplete_closure_stays_unknown`.
Flips: `weapon_tables.rs:1019-1044` (len stays 42; new sibling test
`every_census_class_has_a_known_proficiency_answer` iterates `class_census`);
`combat.rs:2101` renamed `..._and_samurai_reach_computed` (meaning changes — say so in the
commit); `untabled_base_class_features.rs:1473` -> all 27 of 27; `:1502` (Magus) replaced by the
incomplete-closure test, which keeps the "unknown stays unknown" contract alive on a synthetic
record; any test pinning the defect-file length or `_report.json` degradation counts.
Risks: the 11 base classes of 3.1(3) need more than one mechanism; print blast radius (F1b);
a child category that maps to two parents in different books (the map must be per-tree-order,
first wins, logged); 3.1 item 4 (gate carriage) touches the same `held_set` fixpoint F1b's
R2/R3 also touch — sequence 3.1 item 4 before F1b's join work, not after, so F1b's n=1/n=5
dumps already reflect gated facts correctly and are not re-run.
**Size: 32-42 agent-hours** (was 14-20 under option B originally, revised to 20-28 for option A,
now +6 to 26-34 for review finding 1's `Effect::GatedFactGrant` work, further +6-8 for review
finding 5's process-wide package handle, its named-fallback tests, and the required cold/warm
load-time and memory measurement — an architectural addition with no line item or hour estimate
before this review).

---

## 3b. F1b — Print-path reconciliation (new, required by option A)

### 3b.0 The blast radius is under-counted by sibling amplification (review finding 7 — CONFIRMED)

Holding one newly-granted rule also holds every SIBLING of that rule — `held_set`'s `add`
closure (`sheet_rule.rs:1892-1899`): after inserting `id`, it walks `package.siblings_of(id)`
(`:1066-1069`, every rule keyed `"{id}#<suffix>"` in the same file) and inserts each one too, at
no additional gate. The package holds **71,862 rules for 49,450 records**
(`data/sheet_rules/_report.json`, verified: `records=49450 … rules=71862`) — **~1.45 rules per
record**, so sibling amplification is the COMMON case across the package, not an edge case. The
4,456-edge figure in §1's table and used throughout 3b.1-3b.6 and F1b's sizing counts new
`granted_by` EDGES, not newly-HELD (and potentially newly-printed) rules; the true count is
materially higher whenever a newly-reachable A-target has siblings.

**Fix, scheduled before F1b's n=1 (not after):** compute the true figure offline from the
converted package, once the A-target list is known (§1's `unres2.py` output): for each of
the 4,456 A-target rule ids, count `1 + |siblings_of(id)|`, split by the target's own `print:
true` / `false`. Put THIS number — not 4,456 — in the 3b.1 table below and in F1b's hour
estimate; 4,456 remains correct as the edge count (F1.3/F1.6's acceptance still uses it
correctly, since those measure `granted_by` rows, not held/printed rules) but must not be
conflated with the print-surface size again anywhere in F1b.

### 3b.1 Where a newly held rule surfaces (from the code)

`held_set` (`sheet_rule.rs:1891`) is a fixpoint over a `HeldSeed`. The seed
(`HeldSeed::from_character`, `sheet_rule.rs:1264-1298`) is: race, classes, feats, traits,
equipment, spells, skills, race-trait keys from the race resolver, and — the bridge that
matters — **`class_features`: every bespoke `class_feature.*` explanation id the chassis
computed**, slug-joined to a converted `class_feature` rule (`<class>_<tail>` then `<tail>`,
longest tail first). A rule is keyed by rule id and `add` refuses a second insert, so
**one rule can never print twice from the converted path**, whatever granted it.

Consumers of `held_set` / `render_sheet` today:

| # | Consumer | File | What a new edge changes |
|---|---|---|---|
| 1 | Receipt `sheet_lines` -> desktop "Rules and features" section | `class_shared_core.rs:40-52` `with_sheet_rules`; `character_hub.rs:742` `sheet_lines_for`; `rule_system_adapter.rs:353`; `CharacterSheet.tsx:2438` | New printed lines for every newly held `print:true` rule |
| 2 | Feat prerequisite verdicts | `feat_prereqs.rs:377-392` `PrereqFacts` (+ `feat_prereqs/converted_gate.rs`, `combat.rs`, `metamagic.rs`, `item_creation.rs`), desktop `feat_catalog.rs` | `Holds{Rule}` / `RuleTag` gates over class features flip unmet -> met |
| 3 | Level-up option pools | `level_up_option_filter.rs` `filter_option_pool`; `character_hub.rs:760` `feat_options_for`; `level_up/fighter.rs` | eligible/refused lists move |
| 4 | Prestige entry gate (non-blocking) | `prestige_class_entry_gate.rs:206-215` | met/unmet notes move (F0's `entry_gate` column) |
| 5 | Class-feature description join | desktop `class_feature_feat_bridge.rs`; frontend `classFeaturesModel.ts:301-330` `noticeHasSheetRule` | a `.unsupported` notice disappears when its rule now prints |
| 6 | Resolved prose | `pilot_compute/resolved_prose.rs:301` | reads the package, not the held set — unaffected |

What the bespoke `pilot_compute` modules ALSO feed: only consumer 1's neighbour, the desktop
**"Class Features" section**, built from `class_feature.*` / `class_chassis.*` explanations
(`classFeaturesModel.ts`). Bespoke ids are FACET ids, not rule ids:
`class_feature.acg.brawler.knockout_dc`, `...knockout_uses_per_day`,
`class_feature.pu.unchained_rogue.sneak_attack_dice` (from `class_fighter.rs`,
`class_rogue.rs`), against converted slugs `brawler_knockout`, `rogue_sneak_attack`,
`fighter_bravery`. Today the seed's exact-tail join misses most facet ids (`knockout_dc` is no
rule slug), so those rules are NOT held and print nothing. A third source also exists:
`data/class_feature_grants/<book>/<class>.json` (240 files,
`git ls-files data/class_feature_grants | wc -l`), consumed by
`class_feature_grant_consumer.rs` into `class_feature.<owner>.corpus_record.<slug>`
explanations — which DO join exactly and already seed their rules.

So, by source kind of the 4,456 new edges:

| Edges | Kind | Effect on a player sheet | Duplicate / conflict possible? |
|---|---|---|---|
| 3,159 + 441 | class_feature, class | Rule text lines appear in "Rules and features" for features the bespoke path shows only as numbers (or not at all). Rules already seeded via `corpus_record` ids: no change (same rule id). | **Yes — the only place.** (a) same feature shown as a number in "Class Features" and as a converted NUMBER in "Rules and features" with a different value; (b) the seed's fuzzy join picked a different rule than the oracle edge, so two near-identical lines print; (c) a newly held rule carries `Waives`/`Revokes` and REMOVES a line that printed before. |
| 349 | race_trait | Sub-abilities of applied racial traits (languages, weapon familiarity) now held and printed. Race-trait keys are seeded from the race resolver, so parents are held today. | Low: the desktop race section prints trait names from the resolver; the converted path adds the granted sub-rule text. Pure gain unless a sub-rule has a numeric value the race bundle also prints (checked by the same rule as class features). |
| 271 | ability | Held only when the parent ability is held. | Same as its parent's kind. |
| 203 | companion | **Inert for character sheets.** No companion id is ever put in a `HeldSeed`; desktop companion catalogs read `rules_tables` Rust. | None today. Pure stored gain. |
| 26 | monster_ability | **Inert.** Monsters are never seeded in a character held set; the monster catalog reads Rust tables. `sheet_rule_bucket_v_render` seeds single rule ids — its per-unit output may gain child lines. | Parity-bin output only. |
| 6 + 1 | feat, equipment | A held feat / item now also holds the ability it grants. | Pure gain; moves consumer 2/3 verdicts. |

Receipt STATUS is not a consumer: no claim-blocking diagnostic is raised from the held set
(`grep -n 'claim_blocking: true' src/rules_core/feat_prereqs.rs src/rules_core/feat_prereqs/*.rs src/rules_core/pilot_compute/class_feature_grant_consumer.rs src/rules_core/pilot_compute/resolved_prose.rs`
-> no hits).

### 3b.2 The de-duplication rule (mechanical, not per class)

Doctrine: the converted path prints the RULE TEXT; `pilot_compute` computes the numbers that
feed sheet totals. One resolver, one rule, one test family.

- **R1 Identity.** A printed line's key is its converted rule id. `held_set` already
  guarantees one line per rule id. Nothing to build.
- **R2 Join (corrected, review finding 2 — CONFIRMED, original spec verified broken).** The
  original "longest leading run of the remaining segments" spec does not do what the worked
  example claims. Verified: `class_feature.acg.brawler.knockout_dc` has no rule slug
  `brawler_knockout_dc` (`ls data/sheet_rules/advanced_class_guide/class_feature | grep
  brawler_knockout` finds only `brawler_knockout.json`), so a naive longest-leading-run walk
  drops the last segment and matches `brawler` — which DOES exist as a rule, twice over
  (`advanced_class_guide/class/brawler.json`, the class PRINCIPAL rule, and
  `advanced_class_guide/class_feature/brawler.json`) — not `brawler_knockout`, the intended
  target. The same failure hits `class_feature.fighter.weapon_and_armor_proficiency` (no rule
  `fighter_weapon_and_armor_proficiency`; `data/sheet_rules/core_rulebook/class_feature/fighter.json`
  exists and is the class principal rule) and the literally-empty-tailed
  `class_feature.fighter.corpus_record.`. Three pairs the naive spec gets right by accident:
  `class_feature.fighter.bravery`->`fighter_bravery`, `...armor_training`->`fighter_armor_training`,
  `class_feature.acg.brawler.ac_bonus`->`brawler_ac_bonus` (all three exist as their own rule
  files, verified). Every other numeric facet of every class collapses onto its class's
  PRINCIPAL rule, which then also gets treated as if it held that facet's number — making R3
  fire "unequal value" against a rule that has nothing to do with that number, for nearly every
  facet, on every class. **Corrected spec:** strip the namespace segment (`acg`, `pu`, ...) and
  the `corpus_record` family segment, then match the remaining underscore-joined segment string
  against rule slugs by LONGEST COMMON PREFIX, counted in whole underscore-joined words, with a
  hard minimum of `class_slug` PLUS AT LEAST ONE feature word (`class_slug` alone is never a
  valid match length) — and the function explicitly REFUSES to return the class's own principal
  rule id (`<class_slug>` with no feature-word suffix) or the bare class slug under any
  circumstance, even if a caller's segment walk would otherwise land there. Signature becomes
  `sheet_rule::rule_for_explanation(package, class_slug, explanation_id) -> JoinResult` where
  `JoinResult` is `Matched(RuleId) | Ambiguous(Vec<RuleId>) | None` (not a plain `Option`): two
  rule slugs sharing the same longest prefix is a real, distinguishable failure mode (surfaced,
  never silently resolved to either candidate), and `None` (no rule shares even the minimum
  prefix) is distinguished from a match. `HeldSeed::from_character` and the frontend's
  `noticeHasSheetRule` both use this one join (the frontend through a `rule_id` the DTO
  already carries as the line id — no new wire field); both treat `Ambiguous` and `None`
  identically to "no converted rule for this facet" (facet keeps printing alone, R3 does not
  fire).
- **R3 Who wins (corrected, review finding 6 — CONFIRMED self-contradiction, resolved).** The
  original R3 built a RUNTIME mechanism: unequal converted-vs-engine numbers push a non-blocking
  `sheet_rule.value_disagrees_with_engine` diagnostic and render the converted line text-only.
  But 3b.3 step 4 makes `changed-value = 0` a hard gate to even reach the population run, 3b.4
  makes any `changed-value` line a STOP requiring a source fix before re-running, and F1b.4
  requires the diagnostic count to be exactly 0 across every census class. Verified these are
  mutually exclusive as written: if F1b.4 holds in the shipped state, R3's runtime diagnostic
  arm NEVER fires on any path a census sweep reaches — an empty handler wired to a condition
  that provably cannot occur in shipped data, which `docs/governance/no-stub-mvp-doctrine.md`'s
  "no empty event handlers on user-facing affordances" forbids (the affordance is real: the
  desktop's "Rules and features" section renders whatever this handler decides). If instead R3
  is meant to fire on real data, F1b.4's STOP rule makes the batch deadlock on the first real
  disagreement, since 3b.4 requires it be fixed at the source before any re-run, never
  allow-listed — so the runtime arm, if it exists, is provably dead code at ship time either
  way. R3's "engine number wins, converted line prints text only" was also in tension with the
  paper-sheet doctrine on its own terms: it kept a printed rule text next to a number the rule
  itself does not support, on a live user-facing sheet. **Resolution: adopted the reviewer's
  recommended fix.** The STOP rule in 3b.4 stays the batch gate (unchanged: any disagreement is
  fixed at its source, converter formula or bespoke module, before the population run
  proceeds). The disagreement check itself moves OUT of runtime and becomes a TEST-ONLY
  assertion: `tests/sd36_sheet_value_agreement.rs` (already named in F1b.4) iterates every
  census class at levels 1/10/max, computes both the converted rule's value and the bespoke
  module's value for each joined pair (via R2's `rule_for_explanation`), and FAILS BY NAME
  (`rule_id`, `explanation_id`, both values) on any disagreement. There is no runtime
  `Effect`/diagnostic for this at all — the reconciliation "who wins" question never needs a
  live answer, because F1b.4 requires there to be nothing left to reconcile before the batch
  ships. `sheet_rule.value_disagrees_with_engine` as a runtime `ComputationDiagnostic` variant
  is DROPPED from this document (it was never going to fire in a shipped state; keeping an
  unused variant is its own small stub). The rule TEXT half of R3 (always prints; nothing to
  reconcile there) is unchanged.
  - Rule TEXT: the converted line always prints (it is the rule).
  - NUMBER: when a held rule's value is `Number`/`Dice` AND at least one non-`.unsupported`
    bespoke explanation joins to it (via R2): the TEST above requires the values to already be
    equal by the time this ships; if they are, print as is (nothing further to build — no
    runtime "who wins" branch).
  - A bespoke facet with no converted rule keeps printing alone (nothing to reconcile).
  - Near-duplicate rules (3b.1 case b): after R2 the seed and the oracle edge pick the same
    rule or the seed picks none; a remaining pair is two distinct oracle records and both
    print (that is what the book says).
- **R4 Removal.** A line removed by a newly held `Waives`/`Revokes` is legal only when the
  removing rule's own `when` gate includes; the classification in 3b.3 lists every one.
- **Implementation point:** `PilotBaseChassisComputation::with_sheet_rules`
  (`class_shared_core.rs:40-52`) — one post-pass `reconcile_sheet_lines`. Rust only; no
  `rules_tables` change (ruling 7); no PCGen token (ruling 3).
- **Tests (RED first):** unit — equal pair prints once per section; facet id joins by longest
  prefix; join never crosses classes; (no unit test for "unequal pair" runtime behavior — review
  finding 6 removed the runtime disagreement arm; disagreement is caught only by
  `sd36_sheet_value_agreement`, the population test below, which is itself the STOP gate);
  **`a_facet_id_never_joins_to_the_class_principal_rule`** (review finding 2 — RED today against
  the naive spec: asserts `rule_for_explanation(pkg, "brawler", "class_feature.acg.brawler.knockout_dc")`
  returns `Matched(brawler_knockout)`, not `Matched(brawler)`, and the same for
  `class_feature.fighter.weapon_and_armor_proficiency` returning `None`/`Ambiguous`, never
  `Matched(fighter)`); **`the_three_known_good_pairs_still_join`** (`fighter_bravery`,
  `fighter_armor_training`, `brawler_ac_bonus` — a regression guard on the corrected spec's
  common-prefix rule, since the naive spec got these right and the fix must not break them).
  Population —
  `tests/sd36_sheet_value_agreement.rs`: for every census class at levels 1, 10, max:
  `sheet_rule.value_disagrees_with_engine` count is 0 of the total pairs checked (acceptance),
  and every disagreement found on the way is fixed at its source (converter formula or bespoke
  module), never allow-listed.

### 3b.3 Blast-radius measurement BEFORE the population run

**Instrument.** No existing bin renders a whole character's sheet headlessly:
`sheet_rule_bucket_v_render` seeds single rule ids only. So F0's `class_census` bin gains
`--sheet-dump <dir>`: for each (class, level) it writes one sorted JSON file with
`explanations[]`, `diagnostics[]`, `sheet_lines[] {id, kind, label, printed, condition}`,
`feat_eligible[]`, `feat_refused[]`, `entry_gate`. Same code path the desktop uses
(`compute_pilot_base_chassis` + `with_sheet_rules` + `filter_option_pool`). This makes F0 a
hard prerequisite of F1b.

**Steps** (worktree B = converter change; shared tree = before):
0. **Sibling-amplified count (review finding 7), before n=1.** Once F1's A-target list is
   produced, run the offline script from 3b.0 and record the true print-surface figure (edges
   1+siblings, split by `print:true/false`) in this section's table, replacing the raw 4,456.
   This number, not 4,456, sizes the rest of 3b.
1. n=1: Wizard 1, 10, 20. Dump before, regenerate `data/sheet_rules` in worktree B, dump
   after, diff.
2. n=5 across families: Fighter (CRB, bespoke-heavy), Brawler (ACG, facet ids), Unchained
   Rogue (PU, `corpus_record` ids), Samurai (UC, generic chassis + selector), Kineticist
   (untabled, largest closure: 62 rules) — levels 1, 10, 20; plus one race-trait-heavy build
   (Dwarf Fighter 1) and one mix (Fighter 4 / Wizard 4).
3. Classify EVERY changed line, by script + human check, into exactly one of:
   `added-correct` (new rule text; the class holds it at that level per the oracle row),
   `duplicate` (same feature already printed from another rule id),
   `changed-value` (a number the sheet showed before is now different),
   `removed` (a line gone; split into removed-duplicate / removed-by-waiver / removed-unexplained),
   `verdict-moved` (feat eligible/refused or entry-gate note changed; must be explained by a
   named newly held rule).
4. Gate to proceed to the population run: `changed-value = 0`, `removed-unexplained = 0`,
   `duplicate = 0` after R2/R3. Record per-class wall time; project the population cost
   (135 classes x 3 levels) before running it.
5. Commit the classification as `docs/release/SD-36-consolidation/artifacts/epic-f/f1b-blast-radius-n1.md`
   and `...-n5.md` (tables + the diff commands).

### 3b.4 Fixture protocol

Pinned surfaces that can move (found by
`git grep -l 'render_sheet\|held_set\|PrereqFacts\|filter_option_pool\|sheet_lines\|sheetLines'`):
- Rust, root: `src/rules_core/sheet_rule.rs` (`evaluate_tests`), `feat_prereqs.rs` + its four
  submodules, `level_up_option_filter.rs`, `level_up/fighter.rs`,
  `pilot_compute/prestige_class_entry_gate.rs`, `tests/sheet_rule_book_collision_census.rs`.
- Rust, ingest: `crates/codex-ingest/tests/sd27_feat_prerequisite_enforcement.rs`; the
  `sheet_rule_parity` / `sheet_rule_bucket_v_render` outputs and any stored parity ledger
  under `docs/release/SD-35*/artifacts/`; tests pinning `_defects` lengths or `_report.json`.
- Desktop Rust (separate workspace): `character_hub.rs` tests (feat option counts near
  `:7296`, `:10815`; sheet-line tests), `class_feature_feat_bridge.rs`, `feat_catalog.rs`.
- Frontend: `rulesAndFeaturesSection.test.ts`, `classFeaturesModel` tests, `previewData.ts`,
  `characterSheetRefresh.ts`, `loadSavedCharacterDetail.ts` fixtures; the 7 desktop version
  fixtures do not carry sheet lines (version-only) — confirm, do not assume.
- ui-smoke: any row asserting a Rules-and-features or feat-picker count
  (`apps/desktop/scripts/ui-smoke/spec.json`).
- Verify baselines: test counts rise only; `sheet-rules-check` re-pins by construction.

**Rule.** A fixture or expectation may be updated ONLY when every moved line in it is
classified `added-correct`, `removed-duplicate`, `removed-by-waiver` or an explained
`verdict-moved`, and that classification is committed as a receipt
(`artifacts/epic-f/fixture-rebaseline-<file>.md`: before, after, class per line, the newly
held rule id that explains it). **Any `changed-value` or `removed-unexplained` line is a
STOP**: fix the converter or the join, re-run, re-classify. No bulk snapshot regeneration;
no `UPDATE_SNAPSHOTS`-style env run without the receipt. One owner per fixture file.

### 3b.5 Knock-on effects on the census

- **Computed count.** The 441 class edges cannot LOWER it: receipt status comes from
  `pilot_compute` diagnostics, and nothing held raises a claim-blocking diagnostic (3b.1).
  They cannot RAISE it by themselves either; the rise comes from the F1 reader. An undecidable
  `when` on a new edge evaluates Exclude (rule simply not held) — it never blocks.
  Non-blocking movement to expect: prestige `entry_gate` notes flip toward `met`; fewer
  `.unsupported` notices in the desktop Class Features section.
- **`closure_complete` after A** (`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure.py`,
  approximate, upper bound because `when` gates are ignored): of 136 class records, **86
  closures are clean**; mechanism **D sits in 2** (Antipaladin 1 row, Sanguine Angel 4);
  **F sits in 7** (Alchemist 9, Slayer 1, Ex-Antipaladin 1, Diabolist 1, Exalted 1, Magus 1,
  Marksman 1); **B in 0**; **E (target not ingested) in 45 closures, 164 rows** (Ex-Antipaladin
  85, Druid 8, Student of Perfection 6, Tactician 6, Magus 5, Dread 4, ...). E rows are why
  `closure_complete` is defined through the ORACLE target's content (3.4): an un-ingested target
  that carries no weapon grant does not make the answer unknown. The 9 D/F classes of 136 are
  Epic F work: each row is fixed by its parser cause (comma-joined lists, brackets, case) —
  16 rows in all (of the 3,033 D + 808 F rows corpus-wide — the remainder of each mechanism
  stays out of scope per §12's decision and goes to `forward-scope-register.md`).
- **Reader coverage.** 59 of 136 reach a weapon grant after A; the 11 base classes in 3.1(3)
  are the real residue and must reach a known answer before F1.5 can pass. The 66 prestige
  records with no grant become `Some(empty)` only where `closure_complete` holds; in a mix
  the union with the carrier class answers the question anyway.

### 3b.6 Acceptance, size

| Criterion | Acceptance command |
|---|---|
| F1b.0 sibling-amplified count (review finding 7) | offline script (3b.0) run against the real A-target list; a table of `edges / (1+siblings) total / split by print:true` committed to the n=1 receipt; the 4,456 figure is never reused as a print-surface size anywhere in 3b's remaining acceptance rows |
| F1b.1 instrument | `cargo run --locked --bin class_census -- --sheet-dump /tmp/dump --only wizard` writes 3 files |
| F1b.2 n=1 / n=5 receipts | both artifacts exist; each reports `changed-value=0 removed-unexplained=0 duplicate=0` |
| F1b.3 join | `cargo test --locked --lib rule_for_explanation` green, INCLUDING `a_facet_id_never_joins_to_the_class_principal_rule` and `the_three_known_good_pairs_still_join` (review finding 2) |
| F1b.4 agreement (test-only, review finding 6) | `cargo test --locked --test sd36_sheet_value_agreement` green (0 disagreements out of every joined pair, all census classes, at levels 1/10/max; this is the ONLY place the check runs — no runtime diagnostic exists to also assert empty) |
| F1b.5 fixtures (script, not a human read, review finding 12c) | `python3 scripts/tests/check_fixture_rebaseline_receipts.py` (new): diffs the fixture paths `git show --name-only HEAD` lists for the commit against the filenames the `fixture-rebaseline-*.md` receipts name, and exits non-zero on any mismatch (a file changed with no receipt, or a receipt naming a file the commit did not touch) — replaces comparing `ls \| wc -l` against a count a human reads off `git show --stat`, which is not a command that can fail |

**Size: 20-30 agent-hours** (instrument 3-4; **sibling-amplified count script, review finding 7,
+1-2 (new)**; n=1/n=5 + classification 4-6 — likely to rise once the true (not 4,456) figure is
known, since n=5's per-class dumps get larger; R2/R3 + tests 6-8 — raised from 4-6 by review
finding 2: the join is now a 3-arm `JoinResult` with an explicit principal-rule refusal and two
named regression tests, not a bare `Option` longest-prefix walk; fixture re-baselines with
receipts 5-8 — the uncertain part, it scales with how many expectations move, now against the
true sibling-amplified figure rather than the undercounted one).

---

## 4. F2 — Gate arm, CLASS_FAMILY_BOOKS, prestige alone

**Restated purpose (review finding 4 — CONFIRMED).** F2's 0.5 originally read "the 78 [is] 56
prestige + 22 base" and F2.3 accepted "42 + (generic base classes now passing)", implying the
new generic gate arm raises `Computed` for those 22 base ids. Verified this is very likely a
+0 change for those 22: `docs/architecture/status.md:163-166` states the 22 (19-of-20
untabled-exotic overlap + all 3 UC classes) are "already counted in their own rows above, not
double-counted here" — i.e. `has_supported_class_chassis` already returns true for them through
an EXISTING arm (`is_supported_untabled_base_class_single_class`,
`is_supported_crb_untabled_class_single_class`, verified present at
`class_shared_core.rs:3426-3427,3439,3452`). Adding a generic arm that (correctly, per 0.5)
EXCLUDES `Prestige`-tagged records changes nothing for ids an existing arm already covers, and
the 56 prestige rows are exactly what the new arm excludes. The per-family table's blocked set
(93 of 135) is 74 prestige + 11 untabled-exotic + 7 CRB NPC + Samurai, all blocked on weapon
proficiency (an F1 concern) or the prestige rule (F2's other half, the alone-diagnostic), not on
a missing gate arm. **F2's real job is therefore two things that are NOT "raise Computed by
adding generic-family classes": (a) the prestige-alone diagnostic (F2.2, unaffected by this
finding, still real work) and (b) a comment/documentation correction at
`class_shared_core.rs:3420-3425` whose premise ("no BAB/save row exists") is false for the 56
prestige classes — worth fixing for its own sake, but it is a comment fix, not a Computed-count
batch.** F2.3 is rewritten below to a falsifiable row instead of one that a +0 change would
still pass. Every Computed-count rise this document expects instead comes from F1's reader (the
weapon-proficiency union), named per class there.

Files: `class_shared_core.rs` (new `is_supported_generic_class_family_single_class`, shaped
like `:3439-3444`, returns false when the record is tagged `Prestige`; added to
`has_supported_class_chassis`; the comment at 3420-3425 rewritten — its premise "no BAB/save
row exists" is false for 56 of the 74 prestige classes); `generic_class_chassis.rs` (+`core_rulebook`,
`advanced_players_guide` LAST; `pub(crate) fn is_prestige(slug)`); desktop
`class_catalog_generic.rs:296` (same two books, same order); `class_occult_and_psionic.rs`
(prestige-alone branch).

Prestige alone: claim-blocking diagnostic `prestige_class.requires_base_class_levels`, text:
"A prestige class cannot be a character's first class. Add levels in a base class first."
Chassis numbers are NOT emitted for the alone case (no half sheet). `class_chassis.unsupported`
no longer fires for a known prestige id.

| Criterion | Acceptance command |
|---|---|
| F2.1 | `cargo test --locked --lib generic_class_chassis` green with the re-measured pin (78 -> measured; ceiling 96, less any slug a bespoke arm or earlier book already owns) |
| F2.2 | `cargo test --locked --lib prestige_alone` green; census `alone_blocked=74` of 74 |
| F2.3 (rewritten, review finding 4) | census `computed == 42` of 135 BEFORE F2's gate-arm change AND `computed == 42` of 135 AFTER it (falsifiable: a rise here means an id was double-counted against an existing arm, which is itself a bug to fix before proceeding — the original "`>= 42 + (generic base classes now passing)`" row would pass on a +0 change and prove nothing) |

RED first: `a_generic_family_base_class_passes_the_shared_gate`,
`a_prestige_class_alone_states_the_game_rule`. Flips:
`untabled_base_class_features.rs:1463 a_prestige_class_id_still_fails_the_gate` keeps its name
and meaning, asserts the new diagnostic id; desktop `character_hub.rs:6603` swaps its blocked
roster members (Samurai, Magus) for a prestige-alone build so it stays non-vacuous.
Risk: a CRB/APG prestige slug equals a slug an earlier book already gave (first-in-list wins) —
the pin test lists every shadowed slug. **Size: 4-6 agent-hours.**

---

## 5. F3 — Multiclass for every class with a chassis

**Gate.** `multiclass_class_level_supported` (`class_occult_and_psionic.rs:748-755`): true when
the isolated single-class input passes `has_supported_class_chassis` OR the class is a prestige
class with a chassis row at that level. A mix needs >= 1 non-prestige class (the F2 rule).
`table_class_id` stays for the CRB table path. `multiclass_good_saves`
(**corrected location, review finding 11, CONFIRMED**: `class_occult_and_psionic.rs:3808`, not
`class_shared_core.rs` — that file ends at line 3718, before the document's originally cited
`near 3796`; the delegating call is `class_occult_and_psionic.rs:3804-3810`,
`good_saves_for(table_class_id(class_id)?)`): when no CRB table row exists, good/poor for a
GENERIC (non-table) class needs deriving from the `ClassChassis` row, which is not
implementable as originally written.

**Save-shape derivation (review finding 11, CONFIRMED — not implementable from the record as
described, and the target function was misattributed).** `ClassChassis.saves` is a PRIVATE
`[Expr; 3]` (`class_chassis_sheet_rules.rs:62-84`) with no public accessor today, so "derive
good/poor from the Expr shape" requires a new reader (unplanned in the original text). Worse,
the shape can be ABSENT even when the field is present: a save formula that degraded at
conversion prints WORDS, not an `Expr` — the exact bug whose per-occurrence fix moved the
generic-class population 62 -> 78 (`generic_class_chassis.rs:139-160` documents the
degradation-masking history) — so a shape classifier that assumes every `Expr` cleanly matches a
"good" or "poor" progression shape will silently misclassify a degraded save as poor rather than
flagging it. **Fix:** add `impl ClassChassis { pub fn save_shape(&self, index: usize) ->
Option<SaveProgression> }` in `class_chassis_sheet_rules.rs` (the file that already owns the
private `saves` field), where `SaveProgression` is a new enum `{ Good, Poor, Degraded,
Unrecognized }`: `Degraded` when the record's prose/expr pairing shows the known
words-not-Expr symptom, `Unrecognized` when the `Expr` shape matches neither the good nor the
poor closed-form (`level/2+2` vs `level/3`, PF1's only two class-level save shapes) after
degradation is ruled out. `multiclass_good_saves`'s generic-class arm calls `save_shape`, not a
bare good/poor bool. **RED test** (before the arm is written):
`every_generic_class_save_shape_is_recognized_or_named` — iterates all 78 generic-class
records at all three save indices, asserts `save_shape` returns `Some(Good)` or `Some(Poor)` for
each, and on a `Degraded`/`Unrecognized`/`None` result FAILS BY NAME (record id, save index),
listing every such record rather than passing on average agreement. **A class with any
`Degraded`/`Unrecognized`/`None` save shape stays `Blocked` in a mix** (the mix's
`multiclass_class_level_supported` gate returns false for it), never folded in as `poor` by a
silent default — a wrong save progression on a printed sheet is exactly the fabricated-number
hazard the doctrine forbids. Fractional rule and floor-once unchanged for classes with a
recognized shape.

**Prerequisite readers moved to F0/F1 (review finding 3 — CONFIRMED).** F3 as originally
written promised to compute, once per character, "HP (per-class hit die x levels + Con)" and
"skill points (per-class ranks x levels + Int)" straight off `ClassChassis`. Verified `ClassChassis`
(`class_chassis_sheet_rules.rs:62-84`) carries only `{book, slug, level_var, display_name, tags,
max_level, base_attack, saves}` — no hit die, no skill-rank field. Hit die is added by F4, not
F3 (§6, `pub hit_die: Option<u8>`); F3 was scheduled BEFORE F4 in the original §8 order, so it
could not have compiled against a field F4 had not yet added. Skill ranks per level are added by
NOBODY in any section of this document — verified `git grep -n
'skill_ranks_per_level|skills_per_level|ranks_per_level|rank_budget' -- src apps` returns nothing,
and no converter reader for it is named anywhere. **Fix:** the two `ClassChassis` readers —
`hit_die: Option<u8>` (moved here from F4's §6, parsed from the same converted `StatBlock "Hit
die"` prose 0.4 already confirms exists on every class principal rule) and a NEW
`skill_ranks_per_level: Option<u8>` (parsed from the converted record's `StatBlock "Skill
Ranks per Level"` prose row — same StatBlock family as hit die, same parse shape; unverified
whether every class principal rule carries this row, so F0/F1 must check and name any class
where it is absent) — are now PREREQUISITES of F3, built in F0/F1, not F4. A class with `None`
for either field is reported `Unknown` by the census (new acceptance row F3.0 below), never
folded as zero HP or zero skill points — the paper-sheet doctrine's "no fabricated row" applies
to a silently-zeroed total exactly as it does to a silently-granted proficiency. F4's §6 loses
its `hit_die` bullet (moved here, no duplicate work) and gains only the roster-omission
consumption of it.

**What is computed vs printed (paper-sheet doctrine; ONE generic fold, zero per-class rewrites).**
- Computed once for the character: BAB (sum), saves (fractional), HP (per-class hit die x
  levels + Con), skill points (per-class ranks x levels + Int), class-skill union,
  weapon-proficiency union (already so, `:2543-2550`), max ranks / feats / ability increases
  (character level — assumed already character-level; F0 histogram confirms).
- Taken verbatim from each class's ISOLATED single-class run (the trick
  `compute_multiclass_base_chassis` already uses): class-feature text lines, spell slots and
  caster level per class, per-class pools. Explanation ids are re-scoped
  `multiclass.<class>.<original id>` so assertion (a) below stays true.
- Printed only: prestige entry requirements, stacking notes ("caster level +1 of existing
  class" prints as text; no simulation of which class advances).
The F0 mix-panel histogram decides the exact work list. **What claim-blocks
`[barbarian 12, fighter 1]` is not measured yet** — F3 code does not start before that
histogram exists.

**The "183" denominator has no source anywhere in the repo (review finding 8 — CONFIRMED, fixed
below).** No doc, test, or baseline file states 183: verified
`git grep -n '\b183\b' -- docs/architecture docs/release/SD-36-consolidation` returns only
unrelated `POOL.183.*` rows from oracle-parity export fixtures. Independent count: the message
literal `"stay claim-blocked"` occurs **235** times under `tests/` (`grep -rn 'stay
claim-blocked' tests/ | wc -l` -> 235; verified split: 80 in `tests/sd13_progression/`
including its 2-row `rows.rs` macro shell, 78 in `tests/sd18_widening/` including its 64-row
`rows.rs::MULTICLASS_NEG_ROWS`, the remaining ~77 one per hand-written sd13/sd18/v0.6 file).
`MULTICLASS_NEG_ROWS` (`tests/sd18_widening/rows.rs:295`) is 64 rows, all consumed by
`sd18_multiclass_neg_control_test!`; `multiclass_negative_controls!`
(`tests/sd13_progression/rows.rs:90`) is a macro invoked per-file across the progression
fixtures. **Fix (adopted):** the census's `BASELINE_CENSUS_MIX_COMPUTED` baseline is derived,
before F0 sets it, by a committed reproducible command —
`cargo test --locked --test sd18_widening -- --list | grep -c multiclass` plus
`cargo test --locked --test sd13_progression -- --list | grep -c multiclass` plus an explicit,
enumerated list (not a guess) of every hand-written file carrying its own
`"stay claim-blocked"` assertion outside those two macro-driven suites — and that exact command
sequence is what F0 records as the baseline's provenance. No number is set from memory or from
this document's own arithmetic; whatever the command sequence returns (**not necessarily 183 and
not necessarily 235** — 235 counts a string literal across ALL of `tests/`, which is a looser
superset than "multiclass negative-control tests", so the real denominator is somewhere between
the two and must be measured, not asserted) is the baseline, and F3.1 below is rewritten to cite
it as a variable, not a hardcoded 183.

**The negative-control tests** (`tests/sd18_widening/rows.rs` MULTICLASS_NEG_ROWS 64 rows,
`tests/sd13_progression/rows.rs`, plus hand-written sd13/v0.6 files). Names unchanged, test
list byte-identical (`cargo test --locked --test sd18_widening -- --list`, same for
`sd13_progression`, diffed before/after). Assertion (a) — no `class_chassis.<class>.*` /
`class_feature.<class>.*` explanation fires for a mix — KEPT verbatim. Assertion (b)
`any(claim_blocking)` "must stay claim-blocked in this slice" becomes
`assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{diagnostics:?}")`, the strong
shape of `the_nine_classes_with_a_real_proficiency_row_reach_computed`. This IS a change of
meaning for (b); the commit message and `receipts.md` say so, citing the Fighter/Wizard
precedent (19 tests). Vacuity guards in the macros updated to require that the row's input
really has `class_levels.len() >= 2`. Sabotage parity re-proven: remove the generic gate arm
-> all `BASELINE_CENSUS_MIX_COMPUTED` red (review finding 8: measured figure, not 183); restore
-> green.

| Criterion | Acceptance command |
|---|---|
| F3.0 unknown, not zero (review finding 3) | for every census class where `ClassChassis.hit_die` or `.skill_ranks_per_level` is `None`, the census reports that class's HP/skill-point figures `Unknown` (named list, not a silent 0); `cargo test --locked --lib a_class_missing_hit_die_reports_hp_unknown` and `..._missing_skill_ranks_reports_skill_points_unknown` green |
| F3.1 (review finding 8: denominator is measured, not 183; measured at F0d = 185) | census `mix_computed == BASELINE_CENSUS_MIX_COMPUTED` (185, F0d's measured figure, provenance = the §5 command sequence and `scripts/verify-baselines.env`) of itself; prestige canonical mixes 74 of 74 — **already true as of F0d, unconditionally**: F0d's own sweep of the panel against the census's shared canonical fixture measured `mix_panel_computed=185` (0 Blocked) BEFORE any F3 code exists (`docs/release/SD-36-consolidation/artifacts/epic-f/census-f0d.json`, `mix-panel-histogram.md`). F3's own multiclass BAB/save/HP/skill-point work is therefore not what makes THIS panel's 185 mixes reach Computed under this fixture; F3's real work list must come from the classes/fixture shapes the panel does NOT cover (§5's own "What claim-blocks..." histogram premise did not hold against this fixture — recorded, not silently dropped) |
| F3.2 | `--list` diffs for `sd18_widening` (891 of 891) and `sd13_progression` (1,136 of 1,136): IDENTICAL |
| F3.3 (review finding 8; measured at F0d = 185) | sabotage log: `BASELINE_CENSUS_MIX_COMPUTED` (185) red under sabotage (of the measured total), 0 red restored (artifact under `artifacts/epic-f/`, cites the same measured figure as F3.1 — never a separate hardcoded number) |
| F3.4 | `cargo test --locked --test sd21_multiclass_fighter_wizard_chassis_computes --test sd24_multiclass_integration` green |

RED first: `tests/sd36_multiclass_any_class.rs` — `[barbarian 12, fighter 1]`,
`[fighter 6, arcane_archer 3]`, `[magus 4, samurai 2]`, `[wizard 5, loremaster 2]` reach
Computed; BAB/saves/HP/skill-point totals asserted against hand-worked PF1 values (oracle
first — presence is not correctness); `every_generic_class_save_shape_is_recognized_or_named`
(review finding 11, RED today: `save_shape` does not exist). Risks: a bespoke class module that
reads character level where it should read class level (each found by the histogram, fixed
generically in the fold, not per class); a generic class whose save `Expr` is `Degraded` or
`Unrecognized` stays `Blocked` in a mix rather than silently folding as poor (review finding
11). **Size: 17-26 agent-hours** (was 16-24; +1-2 for F3.0's Unknown-not-zero
tests, review finding 3; the `save_shape` accessor and its RED test are absorbed inside this
range — a small addition to an existing reader, not a new sub-batch) — **the least certain
figure in this document.**

---

## 6. F4 — Desktop

Files: `apps/desktop/src-tauri/src/character_hub.rs` (+`list_class_creation_roster`, mirror of
`list_race_creation_roster` 4696-4778); `apps/desktop/src-tauri/src/main.rs` (register);
`apps/desktop/src-tauri/src/pf1_adapter.rs:636` (seeds from the shared module);
new `src/rules_core/class_seeds.rs` (single source; moved from
`src/bin/v06_class_state_dump.rs:129-303`, which then imports it);
`class_chassis_sheet_rules.rs`'s `hit_die` field (review finding 3: reader itself now lands in
F0, not here — F4 only CONSUMES `Option<u8>`: `None` -> the roster omits the class and the
census names it);
frontend new `apps/desktop/src/characterHub/classRoster.ts` + `classRoster.test.ts`;
`characterHubModel.ts:409-472` (CLASS_OPTIONS reduced to a typed fallback used only when the
command fails, with a visible notice — not a silent stub); `CreateCharacterForm.tsx`,
`LevelUpDialog.tsx` (offer: advance an existing class, add a base class, add a prestige class
with its printed entry requirements and met/unmet note; character level cap 20 kept),
`characterProgression.ts`, `spellsTabModel.ts`, `skillsModel.ts`, `classPreviewModel.ts` and
their three test files; `apps/desktop/scripts/ui-smoke/spec.json`.

Roster rule: a class is offered iff the census says Computed at every level AND
`ClassChassis.hit_die.is_some()` (engine-derived, so the picker can never again offer an
uncomputable class — and never a Computed class with no HP figure to print, review finding
13). Creation roster = base + NPC classes, grouped by family; prestige classes appear ONLY in
level-up. Ex-* states: census-only, never offered at creation (ruled 2026-09-21, §9/§14).
**Named-exception guarantee (review finding 13, CONFIRMED — a class the census calls Computed
that the roster silently omits is otherwise an unexplained gap).** `in_desktop_roster == false`
on any census row implies a NAMED reason from a closed enum, never a bare boolean:
`hit_die_absent` (the 7 of 185 identified in 0.4 — the failure mode this finding names),
`not_computed`, `prestige`, or `ex_state`. The roster and the census both read the same reason
field, so the two can never silently diverge.

| Criterion | Acceptance command |
|---|---|
| F4.1 | `cd apps/desktop/src-tauri && cargo test --locked list_class_creation_roster` green; roster length == census computed base count |
| F4.2 | `cd apps/desktop && npm test -- classRoster characterHubModel characterProgression skillsModel` green; `npm run typecheck` green |
| F4.3 (strengthened, review finding 12b) | `git grep -c 'fn canonical_seeds_for' -- src apps` -> 1 (the single definition) AND `git grep -n 'use .*canonical_seeds_for' -- src/bin apps` -> 2 (both call sites import it rather than each declaring their own copy — a `grep -c` on the bare call-site name alone would print the same count whether the second file imports the shared function or redefines it, so the `use` check is required to prove single-source) |
| F4.4 | ui-smoke rows green: `create-character-samurai`, `-magus`, `-warrior`, `-kineticist`, `-inquisitor-generic` (one per newly offered family) and `level-up-fighter6-into-arcane-archer` |
| F4.5 (new, review finding 13) | `cargo test --locked --lib no_computed_class_is_unoffered_without_a_named_reason` green: every census row with `in_desktop_roster == false` carries one of `hit_die_absent \| not_computed \| prestige \| ex_state`; the 7 `hit_die_absent` classes named in 0.4 are asserted present under that reason by id |

RED first: roster command test (fails: command absent); `classRoster.test.ts`; seed-parity
test (dump bin and adapter produce identical seeds for all seeded classes);
`no_computed_class_is_unoffered_without_a_named_reason` (review finding 13, RED today: the
reason enum does not exist, and the 7 `hit_die_absent` classes are silently dropped by a bare
`None` check with no name attached). Risks: the 7 desktop version fixtures and any snapshot
that pins 31 options; ui-smoke needs the DOM-probe harness (no browser on this box). **Size:
12-16 agent-hours** (the named-reason enum and its test are absorbed inside this range).

---

## 7. F5 — Closure deltas

`docs/architecture/status.md` class table GENERATED between markers by
`scripts/gen_class_status_table.py` (stage fails on drift); the five head-count sites listed at
`status.md:56-70` updated together (README headline, desktop-app.md, rules-data-tables.md,
rules-engine.md, status Posture) and the grep at `status.md:70` re-run to zero stale hits;
`docs/architecture/rules-engine.md` dispatch section (generic gate arm, prestige rule,
multiclass fold, proficiency reader); `epic-breakdown.md` + Epic F tables;
`decisions.md` §11 (close class gaps inside SD-36), §12 (converter resolver scope — the option
chosen), §13 (proficiency read from converted data; ruling-7 reasoning), §14 (prestige legal
only in a mix; entry gate printed; Ex-* census-only); `workflow-instruction.md` §0/§3 rows;
`kanban.md`, `progress.md`, `receipts.md` (Epic F evidence, sabotage log, structural diff),
`release-notes.md`, `forward-scope-register.md` (mechanisms D, E, F of §1 with counts
and commands); `scripts/verify-baselines.env`; retro events as they happen; PR #393 body;
graphify LAST against the final tree. **Size: 4-6 agent-hours (Haiku for the mechanical
edits, Sonnet for status/rules-engine prose).**

---

## 8. Order, overlap, verify budget (revised for option A)

```
F0 (census + --sheet-dump)
 -> F1 part 1: converter change authored + n=1 (worktree B, nothing committed to data/)
 -> F1b n=1, n=5 classification   [GATE: changed-value=0, removed-unexplained=0]
 -> F1b R2/R3 join + reconcile (shared tree, Rust)
 -> F1 population run: data/sheet_rules regenerated + structural diff + fixture re-baselines
 -> F1 part 2: proficiency reader + parity pin
 -> F2 -> F3 -> F4 (Rust) -> F5
```
What MUST land before F2/F3/F4: F0 whole, INCLUDING its `hit_die` / `skill_ranks_per_level`
`ClassChassis` readers (review finding 3 — F3 cannot compute HP or skill points without them,
and they were originally mis-scheduled into F4, after F3); F1 population commit and the F1
reader (F2's census rise and F3's proficiency union read them); F1b R2/R3 and all fixture
re-baselines (so F2/F3 start from a green tree and their own fixture moves are attributable to
them, not to option A).
What can overlap (disjoint files, no cargo): F1 converter AUTHORING with F4 FRONTEND authoring
(`apps/desktop/src/**`, except the files in 3b.4's frontend list, which belong to F1b until it
lands); F5 package-doc skeletons and `decisions.md` entries any time; F3's hand-worked PF1
expectation tables (pure authoring). Only one cargo build at a time; the converter run uses
its own worktree + `CARGO_TARGET_DIR` (delete after). F2 and F3 both edit
`class_shared_core.rs` and `class_occult_and_psionic.rs` -> strictly sequential; F1b's
reconcile post-pass also touches `class_shared_core.rs:40-52` -> F1b before F2.

**Full `scripts/verify.sh` passes: 2 planned + 1 repair = 3.** Pass 1 after the F1/F1b
population commit (option A moves data under every suite; its failures must be attributed to
it alone before F2/F3 stack on top). Pass 2 after F4. Scoped suites only between batches.

| Batch | Agent-hours |
|---|---|
| F0 | 10-13 (adds `--sheet-dump`; +2-3 for the `hit_die`/`skill_ranks_per_level` readers moved in from F4, review finding 3) |
| F1 | 32-42 (was 20-28; +6 for `Effect::GatedFactGrant` gate-carriage, review finding 1; +6-8 for the process-wide package handle, review finding 5) |
| F1b | 20-30 (was 16-24; +1-2 for the sibling-amplified blast-radius script, review finding 7; +2-4 for the corrected 3-arm join, review finding 2) |
| F2 | 4-6 (unchanged; purpose restated, not resized — review finding 4) |
| F3 | 17-26 (was 16-24; +1-2 for F3.0's Unknown-not-zero tests, review finding 3) |
| F4 | 12-16 |
| F5 | 5-7 (adds forward-scope rows for D/E/F/B and the F1b receipts index) |
| **Total** | **100-140** (was 81-115 before this review; was 56-80 under option B). Every increase above is attributed to a numbered review finding, not a round-number pad — see §12. |

## 9. Operator decisions — RULED 2026-09-21

1. **Converter resolver scope = OPTION A.** Fix all 4,456 parent-category links corpus-wide,
   in this bundle. Consequences accepted: F1b exists; one extra full verify pass; +25-35
   agent-hours over option B. (A separate, later adversarial review of THIS document found five
   design gaps inside option A's own scope — a dropped gate, a broken join, a missing
   prerequisite, an undercounted blast radius, a missing package-load architecture — adding a
   further ~19-25 agent-hours on top, §8's revised total. Those are correctness fixes to
   option A's execution, not a reason to revisit the option-A ruling itself.) Mechanisms D
   (3,033), E (3,565), F (808), B (63) stay out of scope EXCEPT the 16 D/F rows inside class
   closures (3b.5), which Epic F closes.
2. **Unmet prestige entry requirements = PRINT met/unmet, do not block.**
3. **Ex-* classes = census-only**, not offered in the Create picker.

No further operator decision is open. One thing to watch, not a decision: if F1b's n=5 shows
`changed-value > 0` that cannot be fixed at the source inside the batch, that is a blocker to
raise (AGENTS.md Blocker Discipline), not a line to re-baseline.

## 10. Assumptions not yet verified

- What claim-blocks a non-Fighter/Wizard CRB mix (no build run). F0 measures it.
- Cause of the silent Wizard class->class-ability miss (0.3) and of mechanism D (3,033 rows).
- `sheet_rule_convert` has no output-dir flag (only `--one`, `--check` seen) -> worktree run.
- `--check` is green at HEAD today (stage exists; not re-run for this document).
- HP and skill points for a mix are not already character-level.
- The 18 CRB/APG prestige slugs do not collide with a bespoke dispatch arm or an earlier book.
- (Resolved from code) `held_set` honours `Granter::Class { at_level }`; it does NOT fold `FactGrant` into facts — the reader collects them.
- `closure.py` / `unres2.py` match a target by its first closure row and ignore `when` gates: counts derived from them (59 of 136; D in 2, F in 7, E in 45 closures) are approximations, upper bounds for the defect columns.
- The cause of the 11 base classes that reach no weapon grant after A (3.1 item 3).
- The 7 desktop version fixtures carry no sheet lines.
- No stored parity ledger pins `sheet_rule_bucket_v_render` per-unit line counts.
- `Samurai` set members (estimate until the converter prints them).
- The 135 denominator survives adding CRB/APG to the family books (e.g. `ex_antipaladin`).

## 11. Ten most critical files

1. `src/rules_core/pilot_compute/class_shared_core.rs`
2. `src/rules_core/pilot_compute/class_occult_and_psionic.rs`
3. `src/rules_core/pilot_compute/generic_class_chassis.rs`
4. `src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`
5. `crates/codex-ingest/src/pcgen_import/sheet_rule/prereq.rs` (+ `ctx.rs`); and for F1b `src/rules_core/sheet_rule.rs` (`held_set`, `HeldSeed::from_character`, new `rule_for_explanation`)
6. `crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`
7. `src/rules_core/rules_tables/crb/weapon_tables.rs`
8. `src/rules_core/pilot_compute/feat_pillar_and_pool_aggregation.rs`
9. `tests/sd18_widening/rows.rs` + `tests/sd13_progression/rows.rs`
10. `apps/desktop/src-tauri/src/character_hub.rs` + `apps/desktop/src-tauri/src/pf1_adapter.rs`

---

## 12. Review log — adversarial review, 2026-09-21

Fifteen findings in total against the prior version of this document (an orchestration bug
applied only the first 9 in the earlier pass; the remaining 6 — findings 10-15 — are applied
below). Each was independently re-verified against the code and data (commands run, files
read — not taken on the reviewer's word), then this document was amended in place. **All
fifteen were CONFIRMED; none were rejected.** No finding required a carve-out or a softened
acceptance criterion; every fix either strengthened an acceptance row or added one. Total size
rose from 81-115 to 100-140 agent-hours (§8) from findings 1-9; findings 10-15 changed no size
range (each is absorbed inside its batch's existing estimate, noted at the fix site) but
strengthen six acceptance rows, add a public accessor, a carrier precedence rule, and a
named-exception rule. The three operator rulings (§9) are untouched by any of the fifteen.

| # | Section | Verdict | Evidence checked | What changed |
|---|---|---|---|---|
| 1 | 0.1/3.2/3.4, F1 reader gate drop | **CONFIRMED** | Read `convert.rs:1392-1420` directly: `let _ = when;` is the AUTO arm's last line, and `grep -n 'let _ = when' convert.rs` finds exactly that one hit. Read `Effect`/`Fact`/`Grant` in `sheet_rule.rs:428-493`: `FactGrant(Fact)` has no gate field; only rule-level `Grant.when` does. | New 0.1a; new 3.1 item 4 (`Effect::GatedFactGrant`); 3.2 and 3.4 corrected; new F1.8 acceptance row + RED test on Marksman/Kensai; F1 size +6h |
| 2 | 3b.2 R2 join algorithm | **CONFIRMED** | `ls data/sheet_rules/advanced_class_guide/class_feature \| grep brawler`: `brawler.json` (class_feature) and `advanced_class_guide/class/brawler.json` (class principal) both exist; no `brawler_knockout_dc.json`. `git grep` for the 939 bespoke facet ids confirmed `class_feature.acg.brawler.knockout_dc` and `...fighter.weapon_and_armor_proficiency` are real ids with no matching rule slug at full length. Confirmed the three "gets it right" pairs (`fighter_bravery`, `fighter_armor_training`, `brawler_ac_bonus`) exist as their own rule files. | R2 rewritten: longest-common-prefix in whole words, minimum class_slug+1 feature word, explicit refusal of the bare class slug / principal rule id, `Option`->3-arm `JoinResult`; two new RED tests; F1b.3 acceptance row updated; F1b size +2-4h |
| 3 | 5 (F3) / 8 (order), HP & skill points | **CONFIRMED** | Read `ClassChassis` struct (`class_chassis_sheet_rules.rs:62-84`): no hit-die or skill-rank field. `git grep -n 'skill_ranks_per_level\|skills_per_level\|ranks_per_level\|rank_budget' -- src apps` -> zero hits anywhere in the document or the code. Confirmed F4's §6 was the only place `hit_die` was scheduled, after F3 in the original order. | Both `ClassChassis` readers moved into F0 as an explicit prerequisite of F3; new `skill_ranks_per_level: Option<u8>` reader added (was entirely absent); new F3.0 acceptance row (Unknown, not zero, on `None`); F0 size +2-3h, F3 size +1-2h |
| 4 | 4 (F2) / F2.3 | **CONFIRMED** | Read `status.md:163-166` directly: the 22 non-prestige ids in the 78 are stated as "already counted in their own rows above, not double-counted here" through existing arms. Confirmed those arms exist (`class_shared_core.rs:3426-3427,3439,3452`). The math: F2's new arm only affects non-`Prestige`-tagged ids (0.5's own requirement), and those 22 already return `Computed` through an older arm — so the new arm is a +0 change for them. | F2's purpose restated (prestige-alone diagnostic + comment fix, not a Computed-count batch); F2.3 rewritten from a row a +0 change would pass (`>= 42 + ...`) to a falsifiable one (`== 42` before AND after) |
| 5 | 3.4/F1.5, package load | **CONFIRMED** | `git grep -n with_sheet_rules -- src apps`: exactly one production caller (`character_hub.rs:750`), three test-only callers. Read `build_pilot_headless_receipt` (`class_shared_core.rs:1680-1695`): calls only `compute_pilot_base_chassis`, no package. Read the doc comment on `sheet_lines` (`:28-32`) confirming the same. Found the existing desktop-local precedent (`character_hub.rs:723-736`'s `OnceLock`) the document had not cited. | New 3.4a: rules_core-level `OnceLock` package handle (mirroring the desktop's own pattern and `class_chassis_sheet_rules.rs`'s `CACHE`), named `Err` fallback, required load-time/memory measurement before the sub-task is done, explicit statement that F4's roster uses the desktop's existing handle (not a 135x20 census sweep); new F1.9 acceptance row; F1 size +6-8h |
| 6 | 3b.2 R3 vs 3b.3/3b.4/F1b.4 | **CONFIRMED** | Read all four passages together: F1b.4 requires the runtime diagnostic count to be 0 across all census classes; 3b.4 makes any disagreement a STOP that must be fixed at the source before re-running — so the runtime diagnostic arm R3 built can never fire in a shipped state (dead code / empty handler), or, read the other way, the STOP rule makes the batch deadlock the moment it does fire. Genuinely contradictory as written. | Adopted reviewer's recommended fix: runtime diagnostic (`sheet_rule.value_disagrees_with_engine`) dropped entirely; the check moves to a test-only assertion in `sd36_sheet_value_agreement.rs` (already named in F1b.4) that fails by rule id; STOP rule in 3b.4 stays the batch gate, unchanged; R3 rewritten |
| 7 | 3b.1 blast radius | **CONFIRMED** | Read `held_set`'s `add` closure (`sheet_rule.rs:1892-1899`): every held rule also holds its siblings via `siblings_of` (`:1066-1069`), unconditionally. Computed `71862/49450 ≈ 1.45` rules per record from `_report.json` — matches the reviewer's ratio exactly, confirming sibling amplification is the common case, not an edge case. | New 3b.0: the 4,456 edge count is explicitly NOT the print-surface size; a new offline sibling-count script is scheduled before F1b's n=1 (new step 0), its output replaces 4,456 in 3b's sizing; new F1b.0 acceptance row; F1b size +1-2h |
| 8 | 5 (F3)/F0 baselines/F3.1, the "183" figure | **CONFIRMED** | `git grep -n '\b183\b' -- docs/architecture docs/release/SD-36-consolidation`: only unrelated `POOL.183.*` export rows, no doc states 183. Counted `"stay claim-blocked"`: 235 total under `tests/`, split 80 in `sd13_progression/` (incl. its 2-row macro shell), 78 in `sd18_widening/` (incl. its 64-row `MULTICLASS_NEG_ROWS`) — close to but not exactly the reviewer's 78/15 split (78/78, i.e. 14 individual sd18_widening files, not 15 — immaterial to the finding, which is that 183 is unsourced either way). | Every hardcoded "183" replaced with `BASELINE_CENSUS_MIX_COMPUTED`, a value F0 must derive from a stated, reproducible command sequence (never asserted from this document's own arithmetic) — F0's baseline definition, F3.1, F3.3, and the epic's final acceptance line all updated to reference the measured baseline instead of a literal |
| 9 | 0.8/§1 step 1/F1.1, broken command | **CONFIRMED** | Ran it: `cargo run --locked --bin sheet_rule_convert -- --check` from repo root -> `error: no bin target named 'sheet_rule_convert' in default-run packages`, exit 101. Ran the corrected form: `cargo run --locked --quiet -j 2 -p codex-ingest --bin sheet_rule_convert -- --check` -> green, `records=49450 converted=49450 refused=0 rules=71862 var_tables=5309 verdict=PASS (121.2s)`. | Every occurrence of the bare command in the document (0.8, §1 step 1, §1 step 3's `--one` calls, F1.1) corrected to include `-j 2 -p codex-ingest`; 0.8 records the verified-green baseline output so a future reader does not have to re-derive it |
| 10 | F0.2, acceptance command | **CONFIRMED** | Ran it myself: `bash scripts/verify.sh --list` prints a 51-line table headed `stage  full  quick`, one row per stage (e.g. `class-dump           yes   yes`). `bash scripts/verify.sh --list \| grep -c class-dump` -> **1**, not 2 — the original F0.2 row asserted `-> 2` for a correctly registered stage, which only a double-registration bug could produce. | F0.2 rewritten to `grep -E '^class-census +yes +yes'` (asserts membership in both stage sets by the columns, not a row count) plus the existing `--only class-census` green check |
| 11 | 5 (F3), save derivation | **CONFIRMED** | Read `ClassChassis` (`class_chassis_sheet_rules.rs:62-84`): `saves: [Expr; 3]` and `base_attack: Expr` are private, no accessor. `git grep -n multiclass_good_saves -- src` -> defined at `class_occult_and_psionic.rs:3808`, not `class_shared_core.rs` (`wc -l` on that file: 3718 lines, ends before the document's cited `near 3796`). Read `generic_class_chassis.rs:139-160`'s documented degradation-masking history (words-not-Expr bug, 62 -> 78 fix). | Corrected the file/line citation; added `ClassChassis::save_shape(index) -> Option<SaveProgression>` (`Good\|Poor\|Degraded\|Unrecognized`) as a new named accessor in `class_chassis_sheet_rules.rs`; new RED test `every_generic_class_save_shape_is_recognized_or_named` over all 78 generic records, fails by name; a `Degraded`/`Unrecognized`/`None` shape keeps that class `Blocked` in a mix rather than folding as poor |
| 12 | F1.3, F4.3, F1b.5, F0.1, weak acceptance | **CONFIRMED** | (a) Confirmed the collision risk is real: `ls data/sheet_rules/core_rulebook/class_feature/ \| grep wizard` lists both `wizard.json` (FavoredClass ability) and two `wizard_class__<hash>.json` records. Confirmed `_defects` length 11,925 myself. (b) Confirmed `fn canonical_seeds_for` is defined once (`src/bin/v06_class_state_dump.rs:129`) with no `use` import anywhere yet (F4 has not built the second caller). (c)/(d) read the document's own acceptance rows directly: none of the four commands can fail on their own stated criterion. | (a) F1.3 gets a per-edge `provenance.closure_rows` pin plus a D/E/F-unchanged assertion (3,033/3,565/808); (b) F4.3 gets a `use .*canonical_seeds_for` count alongside the definition count; (c) F1b.5 becomes a script (`check_fixture_rebaseline_receipts.py`) that diffs `git show --name-only` against the receipt filenames and can exit non-zero; (d) F0.1 gets a cross-check test pinning the merged id set to `status.md`'s own 31+3+20+7+74 partition before the instrument is allowed to move |
| 13 | 0.4 / 6 (F4), hit die absent | **CONFIRMED** | Measured myself over all 185 `data/sheet_rules/*/class/*.json`: 178 carry the `StatBlock "Hit die"` prose row, 7 do not — `occult_adventures/psychic_detective`, `ultimate_psionics/{gifted_blade, gifted_blade_marksman_power_list, unlocked_talent}`, `bestiary/sorcerer_cleric_arcane`, `ultimate_intrigue/{vwarlock, vcabalist}` — exact match to the reviewer's list. Confirmed 4 of the 7 live in `CLASS_FAMILY_BOOKS` (`generic_class_chassis.rs:57-73`), so they can enter the 78-record generic population. | 0.4 corrected to "178 of 185; 7 named exceptions" with the list; roster rule (§6) now requires `hit_die.is_some()` in addition to Computed; new named-reason enum (`hit_die_absent \| not_computed \| prestige \| ex_state`) so `in_desktop_roster == false` is never a bare boolean; new F4.5 acceptance row + RED test `no_computed_class_is_unoffered_without_a_named_reason` |
| 14 | 2 (F0), prestige carrier | **CONFIRMED** | Measured myself over the 77 tagged-`Prestige` records (74 distinct ids after the 3 cross-book slug dedupes the census already performs — `cyphermage`, `hellknight`, `red_mantis_assassin` each appear in two books): 23 carry a `BaseAttack` requirement, max value 7 (cap-safe on that axis alone, confirmed); `mystic_theurge` and `evangelist` each carry BOTH `HighestSpellLevel Arcane` and `HighestSpellLevel Divine` terms (confirmed by direct read of both records' `applies`); 43 carry neither a caster nor a BAB term (confirmed — these fall to the floor-5 fighter carrier with no caster level at all). | Cap-bites precedence stated explicitly (cap wins; the newly-unmet numeric term is listed in `entry_gate: unmet` with both its required and reached values); dual-caster case gets a second, independent carrier (`wizard`+`cleric`, both must reach Computed); new census column `carrier`; new test `a_prestige_row_referencing_caster_level_names_a_caster_carrier_or_reports_unknown` — a row whose chassis expression references caster level without a named caster carrier reports `Unknown`, never a confidently-wrong 0 |
| 15 | 3.3 (WeaponSet) / F1.2, proficiency vocabulary | **CONFIRMED, ruling later corrected** | Measured myself over all converted `ProfRef::WeaponGroup` values in `data/sheet_rules/**/*.json`: besides the expected tier/group joins, found `Auto` (11 occurrences) and `KoboldTailAttachment` (2 occurrences) — neither a PF1 weapon tier nor weapon group, exact match to the reviewer's evidence. Confirmed the 49,450 freeze premise is unaffected (records/converted are input counts). **This finding's "neither is a PF1 weapon tier or weapon group" reading was later shown wrong by F1 adversarial finding 1 and the F1 re-check round 1 spec-divergence finding: both tags are real, resolvable oracle members (`cr_profs_weapon.lst:10-14`'s five `TYPE:Auto` rows; `arg_profs_weapon.lst:22`'s `TYPE:...KoboldTailAttachment` row), not junk — a converter carve-out excluding them by name was written and then removed on this branch precisely because it discarded two real grants. §3.3 and the F1.2 acceptance row were amended in place rather than only noted here.** | F1.2 extended: every `ProfRef` the reader returns for a census class must be re-derivable from a named oracle row (tier, `Weapon Group <x>`, or expanded `WeaponSet`), not only the 42 static rows; RED test `an_unrecognized_proficiency_tag_makes_the_class_unknown` — a tag matching none of the three shapes (a genuinely member-less tag, proven with a synthetic fixture, NOT `Auto` or `KoboldTailAttachment`) makes that class's proficiency answer `Unknown`, never a fabricated membership |

---

## 13. Provenance — read-only artifacts and residue-gate check

- Analysis scripts `docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py` and
  `.../closure.py` are copied verbatim from the planning scratchpad; both read `data/sheet_rules`
  and a local checkout of the pinned PCGen oracle (`~/workspace/repos/pcgen/data`, not part of
  this repo) and write no repo file. Neither is under a `pcgen_residue_gate.py` `LIVE_ROOTS`
  path (`src/rules_core`, `src/saved_character`, `src/campaign`, `src/homebrew_authoring`,
  `apps/desktop`) — `docs/release/**` is outside every live root — so committing them changes
  nothing the gate scans.
- Verified: `python3 scripts/pcgen_residue_gate.py --check --closure` -> `live_files=0
  live_hits=0 verdict=PASS` after both scripts were added, identical to the pre-add baseline.
