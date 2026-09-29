# F3c5: Internal natural-attack helper rows convert as facts (SD-36 Epic F3, converter step 5)

This step closes FS-20 and moves Dragon Disciple's carrier mix to Computed: **prestige mixes 68 of
74**. It runs under the same single exception as F3b2, F3b2b, F3c3 and F3c4b:

- `data/sheet_rules/**` is regenerated under the F1c structural-diff protocol.
- `data/corpus/**` and `site/**` are untouched (`git diff --quiet -- data/corpus site` is clean).
- The inventory stays at **49,450** records, and no rule or unit is added for a helper.
- `python3 scripts/pcgen_residue_gate.py --check --closure` is `verdict=PASS`.

**Where things changed:**

- **Converter:**
  - `crates/codex-ingest/src/pcgen_import/sheet_rule/natural_attack.rs` (new) holds the rule.
  - `.../sheet_rule/mod.rs` `build_index` runs the scan after pick options are registered.
  - The `.../sheet_rule/convert.rs` `ABILITY` arm emits the fact.
  - `.../ctx.rs` holds the index field.
- **Schema:** `src/rules_core/sheet_rule.rs` gains `Fact::NaturalAttack(String)`, and
  `sheet_rule_catalog.rs` prints it as "the Bite natural attack".

**Baselines:**

- The "before" package is F3c4's (`5bdf879c24`, the same package as F3c4b `07f02c6387`).
- The structural-diff baseline is tranche/16 (`cc21cac195`).
- Both were extracted read-only with `git archive` into scratch.

## 1. The oracle: what an Internal natural-attack helper is

The citations are in the pinned oracle checkout (`~/workspace/repos/pcgen` at `7f818006e3`).

- **The category.** `system/gameModes/Pathfinder/miscinfo.lst:303` declares
  `ABILITYCATEGORY:Internal VISIBLE:NO EDITABLE:NO EDITPOOL:NO`. An Internal ability is a hidden
  helper that the player never picks and PCGen never shows.
- **The row.** `Bite` (`core_essentials/ce_abilities_race.lst:249`) is `CATEGORY:Internal` with
  `TYPE:NaturalAttack.NaturalAttackPrimary.Primary`. It carries:
  - `DEFINE`s and `BONUS:VAR` raises for `BiteAttacks`, `NaturalAttacks` and `BiteOnlyAttack`;
  - `BONUS:WEAPONPROF=Bite|TOHIT|DAMAGE`, which adjusts the Bite weapon.
- **The object's other rows.**
  - Its `.MOD` rows set `BiteSize` (`:307`).
  - They grant the size helper `Bite 1 (<size>)` (`:327`, gated `PREVAREQ:BiteSize,n`).
  - That helper (`:354`) applies `TEMPLATE:Bite 1 (Medium)`.
  - The template (`ce_templates.lst:26`) carries
    `NATURALATTACKS:Bite,Weapon.Natural.NaturalPrimary...,*1,1d6`.
- **`NATURALATTACKS`** (`docs/listfilepages/globalfilestagpages/globalfilesother.html`) creates the
  natural weapon (name, types, count, damage). It "creates a proficiency for the attacks on the fly
  which allows the use of the BONUS:WEAPONPROF tag to effect it". This is the natural weapon's own
  bookkeeping, not a class weapon proficiency.

So holding `Internal|Bite` gives the character exactly one thing a sheet can name: the Bite natural
attack. **Dragon Disciple** (CRB p.380): "Dragon disciples gain no proficiency with any weapon or
armor." Its Dragon Bite (`cr_abilities_class.lst:2968`) carries `ABILITY:Internal|AUTOMATIC|Bite`.

**The brief's premise, corrected** (retro-logged). The brief said "the converter already emits
natural-attack facts from NATURALATTACKS; reuse that Fact shape". It does not.
`NATURALATTACKS` converts to `#natural<n>` Dice lines (`convert.rs`, the `NATURALATTACKS` arm), and
the `Fact` enum had no natural-attack variant.

- One variant was added: `Fact::NaturalAttack(String)`. It is additive, so every existing row
  deserializes unchanged.
- `Fact::Equipment` was considered and rejected. The equipment readers treat that fact as carried
  gear.
- `Fact::Proficiency` would be wrong. The class grants none, and the natural weapon's own
  on-the-fly proficiency is not a class weapon proficiency.

## 2. The rule (one mechanism, no per-attack case)

The rule is in `natural_attack.rs`, and its module doc states it in full.

**A natural-attack helper** is a row that meets all of these:

- it is an ability-family row outside `_pfs/`;
- it is plain;
- its category is `Internal` and its `TYPE:` carries `NaturalAttack`;
- no inventory unit stands for it: no unit owns the row, and no unit or pick option answers its
  `(Internal, KEY)` or `(Internal, name)` pair.

**Its object** is the row plus every `.MOD` row on its key.

**The object is natural-attack-only** when every token on every one of its rows is on the list
below. The check follows each `ABILITY:Internal|...|<t>` into another helper (recursively) and each
`TEMPLATE:<t>` into a template row outside `_pfs/`. The allowed tokens are:

- identity: `CATEGORY`, `KEY`, `TYPE`, `VISIBLE`, `OUTPUTNAME`, `SOURCE*`;
- bookkeeping: `DEFINE`, `BONUS:VAR`, `PRE*` / `!PRE*`;
- the attack: `NATURALATTACKS`, and `BONUS:WEAPONPROF=<w>` where `<w>` is one of the object's own
  attacks or the helper's name.

The attacks are the first fields of the object's `NATURALATTACKS` entries.

**What it converts to.** The rule that names the helper through
`ABILITY:Internal|<nature>|<helper>` gets `Fact::NaturalAttack(<attack>)`, one per attack. It is a
`GatedFactGrant` when that row is gated. The reference resolves, and no rule is written for the
helper.

**Named, never guessed.** A reference to any of these stays unresolved:

- `natural-attack-helper-carries-more`: any other token (a real ability);
- `natural-attack-helper-names-no-attack`: no `NATURALATTACKS` reached;
- `natural-attack-helper-pair-shared`: two helpers claim one pair with different attacks.

Unit tests (`natural_attack::tests`) read the Bite row's tokens as one attack, and name
`SAB`, `BONUS:SKILL`, `ABILITY:Special Ability|...`, `ABILITY:Internal|%LIST`, and a template's
`BONUS:WEAPONPROF` or `DR` as more than a natural attack.

## 3. Enumeration (denominators; `f3c5-helper-enumeration.log`)

The probe is `artifacts/epic-f/scripts/f3c5_helper_enumeration.rs`. It was copied to
`crates/codex-ingest/tests/zz_probe_f3c5.rs`, run with `cargo test --locked -j 8 -p codex-ingest
--test zz_probe_f3c5 -- --nocapture --test-threads=8`, then deleted.

| measure | count |
|---|---:|
| Internal ability rows whose `TYPE:` carries `NaturalAttack`, pinned tree | **794** (all plain; 0 `.COPY`, 0 in `_pfs/`) |
| ...that no inventory unit or pick option stands for (the scan's denominator) | **771** |
| ...classified natural-attack-only | **770 rows**, answering **771** `(Internal, key-or-name)` pairs |
| ...named `carries-more` / `names-no-attack` / `pair-shared` | **0 / 0 / 0** |

The one row that answers no pair of its own is the Core Essentials `Tail Slap` (`:258`). Its name
pair is claimed first by the Bestiary `Crocodile ~ Tail Slap` row (`b1_abilities_race.lst:248`),
which names the same attack, so no reference is lost.

The pairs by attack: Bite 64, Claw 57, Crush 9, Gore 63, Hair 29, Hoof 56, Pincers 64, Rake 8,
Ranged Slam 57, Slam 56, Sting 57, Tail Slap 65, Tail Sweep 9, Talons 56, Tentacle 65, Wing 56.

**Converted references (the package on disk):**

- **681** fact grants (645 `FactGrant`, 36 `GatedFactGrant`) on **546** rules.
- By kind: companion 254, monster 190, template 42, race_trait 22, class_feature 19, ability 11,
  monster_ability 3, feat 2, race 2, equipment 1.
- By attack: Bite 414, Claw 181, Tail Slap 38, Tentacle 18, Sting 15, Rake 8, Pincers 4,
  Ranged Slam 2, Hair 1.

## 4. Counts (before = F3c4 package; commands in `f3c5-verify.log`)

| measure | before | after |
|---|---:|---:|
| inventory records / converted / refused | 49,450 / 49,450 / 0 | 49,450 / 49,450 / 0 |
| `rules_written` | 73,360 | **73,363** (+3 `#weapon0` line siblings, §5) |
| `var_tables` | 6,209 | 6,211 (+2 tables a fact's gate reads, §5) |
| `_defects/unresolved-references.json` rows | 6,932 | **6,252** (680 cleared, 0 added: `f3c5-unresolved-cleared.txt`) |
| unres2 mechanism E / D / B / F | 3,444 / 2,646 / 63 / 779 | **2,764** / 2,646 / 63 / 779 |
| class principals attested `closure_complete` | 135 of 189 | **136 of 189** (+ dragon_disciple) |

**How the cleared rows split.** All 680 cleared rows name a helper, and all are mechanism E.
Before this step, 686 unresolved rows named a helper. The other **6** sit in a gate
(`PREABILITY:1,CATEGORY=Internal,Bite`):

- `horror_adventures:feat:blood_feast`
- `horror_adventures:feat:horrific_gorging`
- `inner_sea_races:feat:death_roll`
- `mythic_adventures:template:mythic_aspect_of_the_beast_has_claws`
- `..._doesn_t_have_claws`
- `advanced_players_guide:companion:evolution_rake`

A fact is not holdable, so these stay `MissingRule` terms. This is the named remainder of FS-20.
None of the six is in a census class closure.

## 5. What else moved, and why (all pinned)

**CONV-02 line split, 3 records.** These records are:

- `core_rulebook:class_feature:draconic_bloodline_claws`
- `..._abyssal_bloodline_claws`
- `..._rage_power_animal_fury`

Each converted to ONE line and kept it as its principal (SD-36 Epic E CONV-02). Now each also
carries its `Internal|Claw` fact, so the principal carries more than its line. The existing CONV-02
rule then does the rest:

- The line moves, with its value, target and gate intact, to a `#weapon0` sibling.
- The principal becomes Text with the record's own gates.

For Draconic and Abyssal Claws this closes **FS-18 for both bloodline lines**. The record is held
from sorcerer 1, as CRB p.75 states ("Starting at 1st level, you can grow claws"). The claw-size
line keeps its `Power1LVL >= 7` gate, which matches the book's one damage step at 7th.

- **Bloodline sweep** (`f3c5-bloodline-sweep.log`, the F3c4b sweep unchanged): lines not held at
  their own gate's level went 20 -> **18 of 287**. The remaining 18 are all Imperious and Kobold
  (FS-19).
- **Computed single-class:** still **30 of 32** at every level 1..=20.

**Two `_vars/` tables first written.** They are `Rake Size` (`v58ee7a23ec1c122a`) and
`Beast Totem Size Change` (`v5a75c897e1253933`). A `GatedFactGrant`'s `when` reads each of them.
Before, the same gate sat on a dropped grant edge.

## 6. FS-19 re-classified (no converter change; measured)

The brief said to confirm that `CharacterFacts` carries race and that the gate is decidable, and
if so to re-classify FS-19 as correct for a human. Both conditions hold:

- `CharacterFacts.race: Option<String>` exists and is seeded into the held set.
- `PREFACT:1,TEMPLATES,IsHuman=true` (`arg_abilities_class.lst:13`) converts to
  `Holds core_rulebook:template:ishuman`, which is correct. The gate is not mis-converted, so the
  brief allows no converter change.

**But the options are not correctly ungranted.** Imperious is the human bloodline. In the oracle, a
human holds the `IsHuman` fact through this chain:

1. the human race grants `ABILITY:Internal|AUTOMATIC|Racial Traits ~ Human`
   (`human_races.lst:6`);
2. that row grants `TEMPLATE:Race ~ Human` (`human_abilities_race.lst:10`);
3. that template grants `TEMPLATE:IsHuman`, which carries `FACT:IsHuman|True`
   (`human_templates.lst:4-5`).

The package's `core_rulebook:ability:racial_traits_human` is filed from the object's `.MOD` row
(`human_abilities_globalvar.lst:10`). Its closure omits the base row that carries the template
grant, so no human ever holds `template:ishuman`. Kobold has the same shape
(`kobold_abilities_race.lst:10`).

**The proof** is the sweep, which probes each race-gated option with the gated race: Imperious as a
human, Kobold as a kobold. Each holds **0 of 9** lines. So the gate never holds even for the
matching race, and "correct for a human" is false for Imperious.

**The mechanism is named FS-22 (M):** a record filed from its object's `.MOD` row omits the
object's base row.

- The proxy population is **143** records, measured with
  `python3 artifacts/epic-f/scripts/f3c5_mod_base_rows.py`, a Python walk that does not use the
  converter's index. 42 of them are `racial_traits_<race>`.
- The fix belongs in closure building, like F3c4b's `mod_fragments` rule for grants, and is out of
  this brief's allowance.
- Both bloodlines stay Blocked by name
  (`class_feature.sorcerer.bloodline.converted_option_not_held`). They are never Computed with an
  empty bloodline.

## 7. RED to GREEN (`f3c5-red.log`)

| test | RED (F3c4 package) | GREEN |
|---|---|---|
| `sheet_rule_convert_gate::an_internal_natural_attack_helper_converts_as_a_fact_on_the_rule_that_grants_it` | panicked: "Dragon Bite grants the Bite natural attack: []" | ok. Checks: Dragon Bite carries `FactGrant(NaturalAttack("Bite"))` and no proficiency; its `Internal\|Bite` row is gone; `class:dragon_disciple` is `closure_complete`; Draconic Claws carries `Claw`, its principal is not `>= 7`-gated, and its `#weapon0` is |
| `tests/sd36_multiclass_any_class.rs::sorcerer5_dragon_disciple3_computes_the_hand_worked_sheet` (new oracle row) | FAILED: Blocked on `combat.baseline_weapon_proficiency_unknown` ("class:dragon_disciple level 3: ... no closure-complete attestation") | ok: Computed; BAB 4; base saves 3/3/6; total saves 5/5/7; HP 59; class skill points 16 |
| `tests/sd36_bloodline_pick_option.rs::a_draconic_pick_reaches_its_lines_at_the_levels_the_book_states` (pin moved: Claws from 1) | FAILED at `:62` (Claws not held at level 1) | ok |
| `natural_attack::tests::{the_bite_helper_tokens_read_as_one_natural_attack, a_token_beyond_the_attack_is_named}` | -- | ok |

**The hand-worked oracle** is `f3c5-hand-worked.md`, written before the row first ran, for
Sorcerer 5 / Dragon Disciple 3 on the census fixture:

- BAB: 2 + 2 = **4**.
- Base saves use the fractional rule. Fort 5/3 + 2.0 = 3.667, giving **3**. Ref 5/3 + 4/3 = 3.0,
  giving **3**; CRB core rounding would give 2, and the row says so. Will 4.5 + 2.0 = 6.5, giving
  **6**.
- Total saves: **5 / 5 / 7**.
- HP: 8 + 4 x 6 + 3 x 9 = **59**.
- Class skill points: 5 x 2 + 3 x 2 = **16**.

**How the engine RED was run.** The F3c4 package was swapped in at the baked path and restored
afterwards. `git status` was checked before and after.

## 8. Census

The command is `cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3c5.json`,
and the output was copied to `artifacts/epic-f/census-f3c5.json`:

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=68 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

**Per-key diff against `census-f3c4.json`:** 7 keys changed.

- `generated_at`.
- `prestige_mix_computed` 67 -> 68.
- `prestige/18` (dragon_disciple): `status` and `mixes/0/status` Blocked -> Computed;
  `levels_computed` 0 -> 10; `levels_blocked` 10 -> 0; `blocking_diagnostics` 1 -> 0.
- No other row moves.

**Remainder, 6 of 74:** the six FS-15 save-formula rows (`multiclass.save_shape.unrecognized`):
Evangelist, Exalted, Mammoth Rider, Pure Legion Enforcer, Sentinel and Ulfen Guard.

**Baselines:**

- `BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` goes **67 -> 68**, with a dated 2026-09-25 reason in
  `scripts/verify-baselines.env`.
- `scripts/check_class_census_baselines.py` with the env floors (135/63/74/185/68): OK.
- `python3 scripts/gen_class_status_table.py --check --json .../census-f3c5.json`: the first run
  FAILED because the table had drifted, 67 -> 68. It was regenerated, and the re-check is OK.

**The reader remainder** (`every_census_class_has_a_known_proficiency_answer`, `--nocapture`):

- Result: `census classes: 137; ... walked by the reader: 95; Known at every level: 92; Unknown: 3`.
- The three Unknown classes are diabolist (G-U), exalted (G-O) and rivethun_emissary (G-K).
- dragon_disciple leaves the `reader-remainder.md` table, and G-N is CLOSED.
- `closure_defects.py dragon_disciple` gives "closure records 17; closure_complete [True]".

## 9. Regeneration protocol (F1c)

| gate | command | result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, refused 0; rules_written 73,360 -> **73,363** |
| freshness | `... -- --check` | exit 0: `records=49450 converted=49450 refused=0 rules=73363 var_tables=6211 verdict=PASS` |
| pins | `python3 .../scripts/f3c5_delta_pins.py <tranche/16> data/sheet_rules .../structural_diff_f3c5_deltas.json <F3c4>` | 0 unexplained against the F3c4 package; details below the table |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16>` | `verdict=PASS`, exit 0 (`f3c5-structural-diff.txt`): unexpected field deltas 0, removed edges 0, removed grants 0, added rule ids 347 (0 with no named cause) |
| planted mutations | `python3 .../scripts/f3c5_planted_mutations.py <scratch> <repo> <tranche/16 package>` | **10 of 10 FAIL, both controls PASS** (`f3c5-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 35 of 35 (+1: a dropped NaturalAttack grant and an unpinned `#weapon` sibling gate) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, shipped_scanned 70,045 (+2 `_vars/` tables), hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |
| unresolved before/after | `unres2.py` over each package; the row diff | 6,932 -> 6,252; E 3,444 -> 2,764; 0 rows added |

**Pinned by `f3c5_delta_pins.py`:**

- 681 NaturalAttack grants, by exact JSON;
- 3 `f3c5_line_sibling` rules, by sha256;
- 9 `f3c5_line_split` field deltas;
- 1 `f3c5_closure_complete` field delta (dragon_disciple);
- 2 added `_vars/` tables, by sha256;
- the `_defects/` counts: unresolved 6,252, and 0 for each natural-attack file.

The F3c5 unresolved-references count supersedes F3c4b's pinned 6,932, which was the one F3c4b pin
the new package failed before the F3c5 pins existed.

**The planted mutations:**

1. Dragon Bite loses its fact.
2. The fact names Claw.
3. The fact becomes a weapon proficiency.
4. The attestation is withdrawn.
5. The Claws `#weapon0` sibling is dropped.
6. The sibling loses its `>= 7` gate.
7. The principal regains the `>= 7` gate.
8. The `Internal|Bite` unresolved row reappears.
9. The Rake Size table loses its contribution.
10. An unpinned `#weapon` sibling is planted.

## 10. Moved pins (fixture protocol; `docs/retro/events/sd36-f3c5-executor.jsonl`, 5 corrections)

- **`class_census::tests::f3c_carriers_named_through_at_least_any_and_not_reach_the_engine`:**
  dragon_disciple goes from Blocked on 1 line to `computed` with no blocker.
- **`tests/sd36_bloodline_pick_option.rs`:** Claws is now asserted from level 1, where it was
  asserted only from 7 (FS-18).
- **`tests/sd36_multiclass_any_class.rs::a_class_with_no_proficiency_answer_cannot_undo_another_class_s_grant`:**
  the no-answer class is now Diabolist (G-U), because Dragon Disciple answers now.
- **Two brief premises were corrected:** the NATURALATTACKS "Fact shape" (§1) and FS-19 being
  "correct for a human" (§6).

**Docs:**

- `reader-remainder.md`: 4 -> 3, and G-N is CLOSED.
- `forward-scope-register.md`:
  - FS-20: CLOSED, with its remainder of 6 gate references.
  - FS-18: the 2 bloodline lines are closed.
  - FS-19: re-classified.
  - FS-22: new, mechanism M.
- `docs/architecture/rules-engine.md`: a natural-attack helper paragraph.
- `docs/architecture/status.md`: the class table was regenerated, and the prestige paragraph now
  says 68 of 74.
- `artifacts/epic-f/README.md`: a `census-f3c5.json` entry.

## 11. Verify (`f3c5-verify.log`, final tree)

| command | result |
|---|---|
| `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | 167 result lines, **1,763 passed**, 0 failed, 43 ignored. That is +3: the gate test and 2 unit tests. |
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (root) | 291 result lines, **6,370 passed, 0 failed**, 27 ignored. The lib alone is 2,723 passed. The +1 is the new oracle row. |
| `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 615 passed, 0 failed |
| `cargo clippy --locked -j 8 -p codex-ingest --tests -- -D warnings` | clean |
| `cargo clippy --locked --tests -j 8 -- -D warnings` (root) | clean |
| census baselines / status table | OK / OK (§8) |
| residue / frozen / bundle / structural diff | PASS / OK / unchanged / PASS |
| `git diff --quiet -- data/corpus site` | clean |

## 12. What stays open, by mechanism

- **FS-15:** 6 prestige save-formula rows. Prestige mixes are 68 of 74.
- **FS-19 / FS-22:** Imperious and Kobold. Mechanism M (a `.MOD`-anchored record omits its base
  row) affects 143 records by the Python proxy.
- **FS-20 remainder:** 6 gate references to a helper. A fact is not holdable.
- **FS-18:** the CONV-02 single-line principal shape, for records that carry no grant.
- **FS-21:** the generic pool-group pass on unlinked selections.
