# SD-36 Epic F1 stage 5 -- merge-readiness blockers (4), root-caused and closed

Spec: `docs/release/SD-36-consolidation/epic-f-class-completion.md` 3b + Review log. Corrects
`stage3/blast-radius-receipt.md`, `stage4/dedup-receipt.md` §4 and `stage5/fixture-receipts.md`
(all three stated `changed_value`/`duplicate` figures true only for their own 70-build / 16-family
manifest, silent about the rest -- exactly the failure shape `every-figure-states-its-denominator`
exists to prevent). This receipt's denominator is the full class census population: **176 builds
(34 class families x levels 1/7/11/14/20, 3 race variants, 3 multiclass mixes)**, rendered with
`--sheet-dump <build> --with-sheet-rules` against this branch's own `data/sheet_rules`, diffed
against the SAME binary run against tranche/16's `data/sheet_rules` (a scratch checkout,
`git checkout tranche/16 -- data/sheet_rules`, since the tranche/16 `class_census` bin has no
`--with-sheet-rules` flag). Render script: `render_wide.sh` (34-family list + 3 race + 3 mix, in
this session's scratchpad). Classifier: `classify_final.py` (same scratchpad), parsing every
`LINE|` row into `(id, kind, label, printed, condition)` and diffing by id (added/removed/changed)
plus a same-build multiset duplicate scan.

## 1. Blocker 1 + Blocker 2 -- changed values

**Corrected figure: 33 changed LINE values across the 176-build population (denominator above),
every one of them a correctness fix matching the cited official PF1 text, via three root-caused,
mechanical (never per-class) fixes.** Zero unexplained/unclassified changed values remain.

### 1a. Root cause A -- comma-separated `BONUS:VAR` targets not indexed under either real name

`crates/codex-ingest/src/pcgen_import/sheet_rule/closure.rs`'s corpus-wide `bonus_var_index`
builder indexed a `BONUS:VAR|<target>|...` token's target field VERBATIM
(`"BloodrageStrBonus,BloodrageConBonus"` as one combined key) instead of splitting it on `,` the
way `bonus_chain_reader.rs`'s `var_contributions` and `mod.rs`'s per-record `own_var_contribs`
builder already correctly do. `variable_rows("BloodrageStrBonus")` and
`variable_rows("BloodrageConBonus")` could therefore never find this row under EITHER real name,
so `resolve_variable`'s same-record-only fold saw only the local `DEFINE` row for both names and
silently baked a genuinely multi-record, level-scaling variable to a flat, never-scaling constant
at CONVERT time -- invisible until a wide population diff renders every affected build.

Fixed by splitting the target field on `,` before indexing (RED-first test:
`variable_rows_finds_a_comma_separated_bonus_var_target_under_each_of_its_own_names`,
`closure.rs`). This alone fixes **Blocker 2** (`bloodrager_bloodrage`'s Str line was frozen at the
base `+4` while its Con/Will siblings correctly scaled to Greater/Mighty Bloodrage -- an internally
inconsistent, PF1-wrong read no table in the corpus produces) and 3 of blocker 1's four named
shapes.

### 1b. Root cause B -- a rule's own `applies` gate reading a var only that rule contributes to

Fixing 1a exposed a pre-existing circularity in `held_set`'s fixpoint
(`src/rules_core/sheet_rule.rs`): `var()` requires `held.holds(rule_id)` for a contribution to
count, and the fixpoint requires `rule.applies` to include before a candidate rule is ADDED to
`held` -- so a rule gating its own `applies` on a var only it (not-yet-held) contributes to could
never become held (real corpus shape: Fighter's `.MOD` `PREVARGTEQ:Fighter_CFP_Level,20` gates its
own Weapon Mastery capstone grant on a var the SAME "Fighter" record both writes and reads).
Regenerating `data/sheet_rules` under fix 1a alone broke Fighter's entire class chassis at level
20 (armor/weapon proficiencies, Bravery, Armor/Weapon Training all silently dropped) -- caught by
the 176-build wide sweep, not by any unit test, because nothing in the existing suite exercised a
self-referential `applies` gate.

Fixed with `Evaluator::evaluating_self` / `.evaluating(id)`: `held_set`'s fixpoint evaluates a
CANDIDATE rule's own `rule.applies` (never `grant.when`, the separate grant-path condition) with
that rule's own id counting as held for THAT evaluation only -- a general mechanism, not a
per-class one, since every fixpoint candidate goes through the same code path. RED-first test:
`a_rule_may_gate_its_own_applies_on_a_var_only_it_itself_contributes` (`sheet_rule.rs`), plus an
unrelated-character control (no fighter levels) asserting the var still reads 0. Verified: fighter
builds at every tested level are byte-identical to the pre-1a baseline after this fix.

### 1c. Root cause C -- an unrelated prestige class's mirror row poisoning the fold decision

Fixing 1a+1b still left a SECOND population regression: `class_chassis_sheet_rules.rs`'s
`row_at()` reads a class's own BAB/save progression with an EMPTY package/held set (its own doc
comment states "no `Var`, no held-set lookup" as a load-bearing assumption). Inner Sea Gods'
Evangelist prestige class carries, for MULTIPLE base classes, a `CATEGORY:Aligned Class` mirror
row that comma-targets BOTH that class's `_CFP_Level` convention name AND its bare `<Class>LVL`
name (`BONUS:VAR|Vigilante_CFP_Level,VigilanteLVL|EvangelistLVL-1`,
`BONUS:VAR|Fighter_CFP_Level,FighterLVL|EvangelistLVL-1`) -- fix 1a correctly makes this row
visible under `VigilanteLVL` too, but that row is a DIFFERENT class's (Evangelist's) own
progression view, never part of Vigilante's own record family, and letting it defeat Vigilante's
in-record fold turned `class_chassis_sheet_rules`'s read of Vigilante's own base attack bonus into
an unconditional 0 (caught by `generic_class_chassis::tests::vigilante_resolves_via_the_
toggle_off_baseab_row_matching_class_catalog_generic` and
`no_class_resolves_a_degenerate_all_zero_progression`, not by the population sweep, since the real
dispatcher does not route Vigilante through this module -- but other conventional classes do, and
would have hit the identical bug the next time an Evangelist-integration row happened to name
them).

Fixed by excluding `CATEGORY:Aligned Class` rows from `bonus_var_index` -- a structural PCGen
category (scoped the same mechanical way `.MOD` rows are already scoped to their own file family
two lines above it in the same function), never a per-class name. RED-first test:
`variable_rows_excludes_an_aligned_class_minus_one_mirror_row` (`closure.rs`). Verified: Vigilante
(and every other class) is byte-identical to the pre-1a baseline after this fix, and `cargo test
--locked -j 8 --lib` is 2668/2668 green (0 failures) on the full crate, and every `codex-ingest`
suite (incl. `package_on_disk_is_fresh_and_clean`) is green after the final `--write` regen.

### 1d. The 33 changed values, named and cited

All 33 are `changed-value-accepted` under Ruling 1's own logic (a level-scaling sibling record
that only the repaired join/fold can now reach, matching the cited official text) -- extending
Ruling 1's citation to the shapes it did not enumerate:

- **Standard Rage `#bonus1/2/3` (Will/Str/Con), 12 lines** -- `barbarian_L11`, `barbarian_L14`,
  `barbarian_L20`, `mix_barb12_ftr1` (Ruling 1 already named L14/L20/the mix; L11 is the IDENTICAL
  mechanism at the level Greater Rage first activates and was simply missing from that
  enumeration): `+2/+4/+4` -> `+3/+6/+6` at L11/L14 (Greater Rage, L11: PF1 Core Rulebook "+6
  Str/+6 Con/+3 Will"), `+2/+4/+4` -> `+4/+8/+8` at L20 (Mighty Rage: "+8/+8/+4").
- **Bloodrage main/`#bonus2`/`#bonus3` (Will/Str/Con), 9 lines** -- `bloodrager_L11/L14/L20`:
  `+2/+4/+4` -> `+3/+6/+6` at L11/L14, `+2/+4/+4` -> `+4/+8/+8` at L20. Matches PF1 Advanced Class
  Guide Greater Bloodrage (11th: Str/Con/Will morale bonus increases, +2/+2/+1 over base) and
  Mighty Bloodrage (20th: another +2/+2/+1) exactly -- verified against the pinned PCGen oracle's
  own `acg_abilities_class.lst` rows 333/342/345 (`BONUS:VAR|BloodrageStrBonus,BloodrageConBonus|4`
  base, `|2` at Greater, `|2` at Mighty; `BONUS:VAR|BloodrageSaveBonus|2` base, `|1`/`|1`).
- **Track Bonus, 4 lines** -- `inquisitor_L7/L11/L14/L20`: `+1` -> `+3/+5/+7/+10`. PF1 Advanced
  Player's Guide Inquisitor Track: "+1/2 class level" (`max(TrackLVL/2, 1)`,
  `apg_abilities_class.lst`'s `Track Bonus` internal record) -- 7/2=3, 11/2=5, 14/2=7, 20/2=10,
  all floor(level/2), matching exactly.
- **Precise Strike `_1h`/`_light`, 8 lines** -- `swashbuckler_L7/L11/L14/L20`: `+0` -> `+7/+11/
  +14/+20`. PF1 Advanced Class Guide Swashbuckler Precise Strike: "+class level" damage
  (`BONUS:VAR|SwashbucklerPreciseStrikeBonus|SwashbucklerDeedsLVL`, and
  `SwashbucklerDeedsLVL` = swashbuckler class level) -- matches exactly.

No OTHER changed value exists anywhere in the 176-build population. Re-derive:
`python3 classify_final.py` (this session's scratchpad) against `wide_final2` (this branch, both
fixes) vs `wide_swap` (tranche/16), `changed=33`.

## 2. Blocker 3 -- new duplicate visible lines

**Root cause:** `render_sheet` (`sheet_rule.rs`) builds its `Vec<SheetLine>` from `held.rules`,
which is keyed by RULE ID -- so two DIFFERENT ids, each held and each `print: true`, can render
byte-identical visible content. Two real shapes, both pre-existing in the corpus and both newly
REACHABLE once fixes 1a/1b correctly propagate `held`/join membership: (a) a bare, class-agnostic
record PF1 writes once, explicitly `granted_by`-aliased ("counts as") by a class-scoped record
with the identical label (`wild_empathy` + `druid_wild_empathy`/`ranger_wild_empathy`,
`aura_of_good` + `paladin_aura_of_good`, `channel_positive_energy` + `paladin_channel_positive_
energy`, `timeless_body` + `monk_timeless_body`, `evasion_output` + `rogue_evasion`); (b) a
`#bonusN` sibling family whose unfilled pick slots all fall back to the identical generic
placeholder text (`hunter_teamwork_feats#bonus1..11`, unfilled teamwork-feat picks; a swashbuckler
weapon-training tier repeating an earlier tier's own weapon-group row). `held.rules` can never
legitimately hold the SAME visible content under two ids for a genuine repeat (a real repeat count
folds into ONE line's own value via `HeldCount`, never N separately-held ids), so any second (and
later) id producing the same `(kind, label, printed, also, condition)` as an earlier one is a pure
display artifact.

**Fix:** `render_sheet` retains lines in its existing `(kind, label, id)` sort order, dropping any
line whose `(kind, label, printed, also, condition)` tuple already appeared -- deterministic (the
existing sort decides which of several identical rules "wins", never a HashMap iteration order),
general (content-based, not a per-record or per-class list), and it applies everywhere
`render_sheet` is called (the live desktop app included, not just this dump tool). RED-first test:
`render_sheet_folds_two_different_rule_ids_with_identical_visible_content_into_one_line`
(`sheet_rule.rs`), reproducing the `wild_empathy` shape directly against a synthetic package.

**Corrected figures (176-build population, same-build multiset scan, `classify_final.py`):**
tranche/16 baseline already carries 130 "extra copy" duplicate lines (a pre-existing defect, not
this branch's scope, whose own `stage3` proxy also missed it for the same "new-vs-already-printed
only" reason blocker 3 diagnosed). This branch: 199 extra copies BEFORE the `render_sheet` dedup
fix (69 of them newly introduced by fixes 1a/1b -- exactly Blocker 3's own count, across the same
29 builds it named), **16 extra copies AFTER** the fix -- a net improvement over tranche/16's own
130, and, checked per build, **zero builds anywhere in the population have more duplicate-extra
copies on this branch than on tranche/16** (`dup_new_rows` in `classify_final.py`'s output is
empty). No NEW duplicate shape remains.

## 3. Invariants re-checked after all four fixes

- `data/sheet_rules/_report.json`: `records=49450 converted=49450 refused=0` (frozen count
  unchanged).
- `git status --porcelain -- data/corpus site`: empty.
- `python3 scripts/pcgen_residue_gate.py --check --closure`: `verdict=PASS`
  (`live_hits=0`, `identifier_hits=0`).
- `cargo test --locked -j 8 --lib` (root crate): 2668 passed, 0 failed, 7 ignored.
- `cargo test --locked -j 8 -p codex-ingest` (incl. `package_on_disk_is_fresh_and_clean`,
  `conversion_is_deterministic`, all 19 `gate_*` kind gates): all green, `EXIT=0`.

## 4. Correction to prior receipts

`stage3/blast-radius-receipt.md` line 22 (`changed-value=0`), `stage4/dedup-receipt.md` §4
(`'changed_value': 9`, "All 9 are core_rulebook:class_feature:standard_rage#bonus1/2/3") and
`stage5/fixture-receipts.md` (`STOPs: none`, x3) are each true only for the 70-build / 16-family
manifest `stage4/render_variant.sh` enumerates (`fixture-receipts.md`'s claim is specifically
about root-crate unit-test pass/fail, a narrower and still-accurate statement on its own terms --
the population-wide census sweep this receipt covers was simply never run by any prior stage).
This receipt supersedes all of them for `changed_value` and `duplicate` figures at the corrected,
full-population denominator (176 builds / 34 families) stated in §0 above.

## 5. Blocker 1c owner-exclusion correction

**Problem.** §1c's fix excluded every `CATEGORY:Aligned Class` `BONUS:VAR` row from
`bonus_var_index` outright. That over-reaches on the one shape where the Aligned Class row IS the
owning record for the name it targets: Inner Sea Gods' Evangelist prestige class's own `Winter
Witch` row (`pathfinder/paizo/campaign_setting/inner_sea_gods/support/abilities_rowpg.lst:24`)
declares `BONUS:VAR|WinterWitchLVL|EvangelistLVL-1` and then consumes that SAME name on the SAME
row (`BONUS:CASTERLEVEL|Witch|WinterWitchLVL-2`) -- no other row anywhere in the pinned tree ever
claims `WinterWitchLVL` (the Reign of Winter Player's Guide `CLASS:Winter Witch` row that would is
an Adventure Path book, outside both `BOOKS_RELATIVE` and `EXTRA_BOOK_DIRS`, so it is never loaded
into `PinnedTree`). Excluding the row left `WinterWitchLVL` DEFINEd nowhere in the tree, which
`resolve_variable` (`ctx.rs`) reads as C1(c) (undefined -> `Const(0)` + an `undefined-variables`
defect) and silently baked `inner_sea_gods:ability:winter_witch#bonus1`'s caster-level bonus to a
flat, never-scaling constant -- the same "level-scaling var baked to a constant" defect shape §1a
(Blocker 2) closed, reintroduced by §1c's own fix, and named by no receipt on the branch until now.
Not reachable on paper today (`Winter Witch`'s `applies` is `Holds{MissingRule{class, "Winter
Witch"}}`, held in 0 of the 176-build population, no `--sheet-dump` line affected) -- a silent
data/defect-register regression, not a wrong printed row, but an unnamed one until this section.

**Before (tranche/16, and this branch before the correction):**
```json
{"id":"inner_sea_gods:ability:winter_witch#bonus1", "value":{"Number":{"Sum":[{"Var":"vd80705684a527b14"},{"Const":-1},{"Const":-2}]}}, ...}
```
**After 1c's original fix (HEAD `1ed2ac9132`, the regression):**
```json
{"id":"inner_sea_gods:ability:winter_witch#bonus1", "value":{"Number":{"Const":-2}}, ...}
```
`data/sheet_rules/_defects/undefined-variables.json` also gained
`inner_sea_gods:ability:winter_witch: WinterWitchLVL` (absent on tranche/16, absent again after
this correction).

**Root-caused fix, at the root (`closure.rs`'s `build_indexes`).** A `CATEGORY:Aligned Class`
`BONUS:VAR` contribution is no longer decided inline: it is buffered
(`aligned_class_pending: Vec<(name, RowRef, row's own id.key)>`) while the single forward scan
builds `class_rows` (every file's genuine `CLASS:<name>` headers, tree-wide), then resolved in one
second pass once the scan completes: **indexed only when no genuine `CLASS:<key>` row exists
anywhere else in the pinned tree under this row's own identity key** -- otherwise it is a mirror of
an externally-owned base class and stays excluded, exactly as §1c intended. One mechanical,
class-name-free predicate: `Vigilante`/`Fighter`/every other base class's own `CLASS:` header
exists elsewhere in the tree (a real class an Aligned Class row merely mirrors), so those stay
excluded; `Winter Witch` has none in scope, so its row -- the only declarer of its own target name
-- is kept. The second pass is necessary (not optional) because a row's owning `CLASS:` header can
live in a file the forward scan has not reached yet; file order is not name order.

**RED-first test:** `variable_rows_keeps_an_aligned_class_row_that_owns_its_own_target_var`
(`closure.rs`), reproducing the real `Winter Witch` row verbatim and asserting
`variable_rows("WinterWitchLVL")` stays non-empty. Confirmed RED against the pre-fix code
(`assertion left == right failed ... left: 0 right: 1`), GREEN after. The stage-5 fix's own guard,
`variable_rows_excludes_an_aligned_class_minus_one_mirror_row` (Vigilante), stays green unchanged.

**Regenerate + verify:**
- `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` before the
  correction: `verdict=FAIL`, `stale on disk: _defects/undefined-variables.json`,
  `stale on disk: inner_sea_gods/ability/winter_witch.json` (exactly the two files this section
  predicts, nothing else).
- `-- --write`: `records=49450 converted=49450 refused=0` (frozen count unchanged), then
  `-- --check` again: `verdict=PASS`.
- `structural_diff.py` (a `--dump` of this branch vs. a read-only worktree of this branch's
  PREVIOUS commit, `1ed2ac9132`): file set unchanged (`+0 -0` everywhere), zero added/removed
  `granted_by` edges or grants, zero var/defect FILE additions, and exactly **one** unexpected
  field delta: `inner_sea_gods:ability:winter_witch#bonus1: value` -- the script's own gate treats
  any unexpected field delta as a script-level FAIL by design (it only names growth classes from
  the ORIGINAL F1 diff as expected), so its printed `verdict=FAIL` here is the script correctly
  reporting the scope of THIS content-correcting change, not a defect: it is the one delta this
  fix intends, and the only one that exists.
- `grep -c WinterWitchLVL data/sheet_rules/_defects/undefined-variables.json` -> `0`.
- **Changed files, this correction, full list:** `crates/codex-ingest/src/pcgen_import/sheet_rule/
  closure.rs` (source), `data/sheet_rules/inner_sea_gods/ability/winter_witch.json`,
  `data/sheet_rules/_defects/undefined-variables.json` (one line removed). No other file in
  `data/sheet_rules/` differs from the previous commit.
- `cargo test --locked -j 8 -p codex-ingest --lib`: 645 passed, 0 failed, 11 ignored.
- `cargo test --locked -j 8 --lib sheet_rule`: 60 passed, 0 failed, 1 ignored.
- `cargo clippy --locked --tests -j 8 --no-deps` (`crates/codex-ingest`, `-D warnings`): clean.
- `cargo clippy --locked --tests -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml
  -- -D warnings`: clean. (A workspace-wide, dependency-inclusive `cargo clippy -- -D warnings`
  from the repo root, or from `crates/codex-ingest` without `--no-deps`, fails on a
  `clippy::type_complexity` lint at `src/rules_core/sheet_rule.rs:2179` -- confirmed byte-identical
  on the previous commit `1ed2ac9132` before this correction touched anything, so it is pre-existing
  and unrelated to this fix, not introduced by it.)
- `python3 scripts/pcgen_residue_gate.py --check --closure`: `verdict=PASS`.
- `git status --porcelain -- data/corpus site`: empty.
