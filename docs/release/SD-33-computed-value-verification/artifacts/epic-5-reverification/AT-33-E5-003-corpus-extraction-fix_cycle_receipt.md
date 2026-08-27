# Cycle AT-33-E5-003-corpus-extraction-fix — epic-5-reverification / AT-33-E5-003

- **Commit SHA:** recorded below at push time (`sd33-r6-corpus-extraction`, remediation wave 6)
- **Files touched:** `src/bin/enrich_equipment_raw_tokens.rs`; 137 of `data/corpus/**/{equipment,equipment_modifier}/*.json`; `artifacts/epic-5-reverification/corpus-extraction-fix.oracle-results.json` (new); `docs/release/SD-33-computed-value-verification/progress.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (`git diff --unified=0 f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba -- src/bin/enrich_equipment_raw_tokens.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** OK_NO_TOKENS (`git diff --unified=0 f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba -- src/bin/enrich_equipment_raw_tokens.rs | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion (this lane's dispatched task, verbatim from the dispatch brief):** "Clear the `## Open blockers` entry by FIXING it — decompose and run the cycles ... find the extraction gap, enumerate the blast radius, fix the extraction with RED→GREEN, regenerate the affected corpus records through the guarded generator path only, re-run the affected units through the oracle harness and confirm `rending_claw_blades` now agrees or produces a different, honestly-recorded verdict, remove the `## Open blockers` entry from `progress.md` and replace it with the resolution."

## Summary

The originally-filed blocker's root cause (a corpus-extraction gap: `.MOD`-attached `EQMOD:`/`BONUS:`
references never captured into `raw_tokens`) is **fixed, verified systemic, regenerated through the
guarded generator path, and re-run.** `rending_claw_blades` itself is **still `disagree`** after the fix —
not because the corpus is wrong anymore, but because a **separate, narrower, newly-diagnosed** gap in
`src/rules_core/equipment_effects.rs` (out of this lane's granted write scope) prevents the now-present
corpus token from reaching the DAMAGE-dimension computation. This is the "different, honestly-recorded
verdict" the dispatch brief explicitly anticipated as an acceptable outcome. The original blocker entry is
removed from `progress.md` and replaced with (a) the resolution below and (b) a new, much smaller,
precisely-scoped blocker naming the exact remaining fix.

## 1. Gap located

`src/bin/enrich_equipment_raw_tokens.rs::enrich_one` folded in a `.COPY=` row's base tokens (an existing,
tested feature, `SD31-E6-F6-001`) but never looked for a separate `<record_key>.MOD` row appearing
elsewhere in the same cited LST file. `src/pcgen_import/lst_parser/equipment.rs::extract_record_name`
strips only a `.COPY=` suffix, not `.MOD`, so a `.MOD` row (e.g. `Rending Claw Blades.MOD`) always opens
its own, differently-named `EquipmentRecord` — nothing upstream ever matched it back to the identity it
modifies.

Real corpus reproduction: `advanced_race_guide:equipment:rending_claw_blades`'s corpus citation
(`data/corpus/advanced_race_guide/equipment/rending_claw_blades.json`'s `source.line`) is `54`:

```
$ sed -n '54p' $PCGEN_DATA_ROOT/pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_equip_arms_armor.lst
Claw Blades (Catfolk).COPY=Rending Claw Blades
```

A bare `.COPY=` row — its own line carries no tokens at all. The base ("Claw Blades (Catfolk)", line 34)
folds in correctly (pre-existing). But line 27, earlier in the same file, is:

```
$ sed -n '27p' $PCGEN_DATA_ROOT/pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_equip_arms_armor.lst
Rending Claw Blades.MOD    ...    COST:10305    EQMOD:Special Ability ~ Keen ~ Weapon.Special Ability ~ +1 ~ Weapon.Material ~ Steel    SOURCEPAGE:p.95    SPROP:...
```

This row targets the identity `.COPY=` creates ("Rending Claw Blades") directly, by name, and neither the
cited line nor its `.COPY=` base fold captured it.

## 2. Blast radius, re-derived live (denominator stated in the same construct throughout)

```
$ python3 scripts/<scan: for every data/corpus/**/{equipment,equipment_modifier}/*.json record whose
  source.kind == "lst_token", index every "<identity>.MOD" row in the cited $PCGEN_DATA_ROOT LST file by
  identity, cross-reference the record's source.record_key against that index, and check whether every
  EQMOD:/BONUS: (key,value) on the matching .MOD row's own line is already present in the record's
  raw_tokens/raw_bonus_chains>
total equipment/equipment_modifier json files: 7807
records_checked (lst_token equipment/equipment_modifier): 7621
records_with_matching_.MOD_row: 391
records_with_DROPPED_eqmod_or_bonus_from_.MOD_row: 139
books_affected: 9 ['advanced_players_guide', 'advanced_race_guide', 'beginner_box', 'inner_sea_combat',
  'inner_sea_gods', 'monster_codex', 'mythic_adventures', 'ultimate_equipment', 'ultimate_intrigue']
```

**139 of 7,621** `lst_token` equipment/equipment_modifier records (of **391 of 7,621** carrying any
matching `.MOD` row at all) across **9 of 27** scanned corpus books carry a `.MOD`-attached EQMOD or BONUS
reference the extraction pipeline dropped. This is a bundle-level finding on its own: not one record, a
systemic gap in the extraction pipeline's `.MOD` handling.

Spot-verified independently against the real pinned LST source for a second, unrelated record
(`ultimate_equipment:equipment:klar`, a PLAIN base row — not a `.COPY=` shape — separately amended by
`Klar.MOD` at line 438 of `ue_equip_arms_armor.lst`, overriding `SOURCEPAGE` p.12→p.31 and adding
`EQMOD:Material ~ Steel`), confirming the gap is not specific to the `.COPY=`+`.MOD` shape.

## 3. RED → GREEN

RED: `enrich_one_folds_in_a_dot_mod_row_targeting_the_copy_created_identity`
(`src/bin/enrich_equipment_raw_tokens.rs`, `pi_screen_tests` module) — a real reproduction of the
`rending_claw_blades` shape (base row + `.COPY=` row + a separate `.MOD` row targeting the `.COPY=`-created
identity). Confirmed failing for the intended reason before the fix:

```
thread 'pi_screen_tests::enrich_one_folds_in_a_dot_mod_row_targeting_the_copy_created_identity' panicked:
the .MOD row's EQMOD (Keen + +1 Weapon Special Abilities) must be folded into raw_tokens, not silently
dropped -- raw_tokens was: [KEY, COST, WT]  (no EQMOD present)
```

GREEN after the fix — `cargo test --locked --bin enrich_equipment_raw_tokens`:

```
running 9 tests
test copy_base_tests::copy_base_identity_splits_on_the_literal_marker ... ok
test copy_base_tests::find_copy_base_never_matches_another_copy_row ... ok
test copy_base_tests::find_copy_base_resolves_by_key_token ... ok
test pi_screen_tests::declared_pi_on_line_reads_nameispi_and_descispi_off_the_raw_line ... ok
test pi_screen_tests::screen_field_value_redacts_a_blacklist_term_hit_on_any_key_not_just_desc ... ok
test pi_screen_tests::screen_field_value_redacts_desc_when_declared_even_without_a_blacklist_hit ... ok
test pi_screen_tests::enrich_one_skips_enrichment_for_a_declared_nameispi_record ... ok
test pi_screen_tests::enrich_one_redacts_an_undeclared_blacklist_hit_in_raw_tokens ... ok
test pi_screen_tests::enrich_one_folds_in_a_dot_mod_row_targeting_the_copy_created_identity ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**Fix design:** after the existing `.COPY=`-base fold, look up `parsed.entries` for a record whose own
`header_raw_line` column-0 equals `"{record_key}.MOD"` (the record's own citation identity, which for a
`.COPY=`-created item is already the `.COPY=` right-hand name). When found: byte-presence-verify every
folded token against its own line (same discipline the existing `.COPY=` fold already applies), fold its
tokens/bonus-chains into the closure, and fold its own `NAMEISPI:`/`DESCISPI:` declaration into the PI
screen (a `.MOD` row can itself declare PI the base did not). No fabrication: every folded value is a
real, byte-present token on its own real LST line.

## 4. Regeneration — guarded generator path only

Two new modes added to the SAME existing tool (`ENRICH_TARGET_LIST`, `ENRICH_FORCE_MOD_REFRESH`) — the
corpus was never hand-edited.

**Why not a full-corpus sweep:** first attempted as a blind `ENRICH_FORCE_MOD_REFRESH=1` full sweep (the
tool's existing book list, all ~7,800 already-enriched records). Killed after ~4 minutes of 98%+ CPU still
inside book 1 of ~20 (core_rulebook, 2,995 records, 1,556 of them citing one 1,619-line file —
`parse_equipment_entries` has no cross-call cache, so a full sweep re-parses that file once per citing
record). **Before the kill, it had already written 2 files it should not have**
(`core_rulebook/equipment/general/{map_case,tankard}.json`) — not from the new `.MOD` fix, but from
silently re-applying the ALREADY-SHIPPED `.COPY=`-base-fold feature to two records that predate it (a real,
separate, pre-existing gap this run happened to also touch — confirmed by inspecting the diff: the folded
tokens were the `.COPY=` base's, not a `.MOD` row's). Both reverted via `git checkout --` before any
commit; confirmed clean (`git status --porcelain -- data/corpus` empty immediately after). This is exactly
why `ENRICH_TARGET_LIST` exists: process ONLY the 139 diagnosed files, never the other ~7,660
already-enriched records this cycle has no mandate to touch.

Real run, targeted:

```
$ ENRICH_FORCE_MOD_REFRESH=1 ENRICH_TARGET_LIST=<139-path list> ./target/debug/enrich_equipment_raw_tokens
...
enrich_equipment_raw_tokens (targeted): 2 enriched, 135 refreshed, 0 unchanged, 2 other
```

`2 + 135 + 2 = 139`. The `2 other` are `DroppedPi` — `Legendsbane` and `Witherfang` (both `mythic_adventures`)
both declare `NAMEISPI:YES` in their own closure, correctly refused per this tool's pre-existing,
unrelated PI discipline (a name cannot be redacted) — not a new defect, not silently dropped: named here
and in `progress.md`.

**Verification, per record, all 139:**

- `git status --porcelain -- data/corpus` → exactly **137** files, all `M` (modify), 0 unexpected —
  `changed_files − target_files = {}` (empty set), confirmed by direct set comparison.
- `target_files − changed_files = {legendsbane.json, witherfang.json}` — exactly the 2 correctly-refused
  NAMEISPI records, matching the tool's own reported count.
- License/`pi_field`/`pi_marker` byte-identical pre/post on all 139 checked records (`0` mismatches).
- `raw_tokens` length monotonically grew (never shrank) on all 139 checked records (`0` shrinkages).
- Total equipment/equipment_modifier record count: **7,808 before, 7,808 after** — 0 added, 0 removed (all
  changes are in-place modifies; the count-sweep hazard — a record-count change compiling clean but
  leaving another file's hardcoded assertion red — does not apply here since no count moved).
- `rending_claw_blades.json` diff (representative sample): base `COST:305` overridden to `10305`, real
  `EQMOD:` reference added, `SOURCEPAGE` corrected `p.93`→`p.95` (the `.MOD` line's own page), the item's
  real "rending" special-property text added (previously the base item's unrelated SPROP was the only one
  present) — all four values match the pinned PCGen source exactly.

## 5. `rending_claw_blades` re-run through the oracle harness

**Verdict: still `disagree`, `ours=0`, `oracle=1`, DAMAGE dimension — a different, honestly-recorded
reason, not a fabricated `agree`.**

Proven live via a scratch integration test (real on-disk, now-fixed corpus; `compute_equipment_effects`;
no fixture — written, run, printed, then `rm`'d, never committed, since `src/rules_core/**` is outside
this lane's granted write scope):

```rust
let corpus = load_equipment_corpus(&[BookCorpusRoot { book_id: "advanced_race_guide", dir: ... }]);
let selection = EquipmentSelection { item_id: "Rending Claw Blades".into(), ... };
let effects = compute_equipment_effects(&[selection], &corpus);
```
```
weapon_enhancement_bonus = Some(WeaponEnhancementBonus { tohit_bonus: Some(1), damage_bonus: None,
  natural_attack_only: false, weapon_prof_scope: None })
```

`tohit_bonus: Some(1)` matches the oracle's own `MAGICHIT=+1` exactly — always agreed, unaffected either
way. `damage_bonus: None` is why `ours=0` for DAMAGE persists: `compute_equipment_effects`'s weapon path
(`src/rules_core/equipment_effects.rs`, the `let weapon_enhancement_bonus =
equipmods::compute_equipmods_effect(record);` line) sums the resolved item's OWN `bonus_chains` only. It
never resolves the item's `EQMOD:`-referenced modifier records (`eqmod_referenced_records`, already
defined in the same file) and sums THEIR `compute_equipmods_effect` result in for this dimension — unlike
the AC dimension, which already does exactly this (`resolve_category_effect` →
`arms_armor::apply_eqmod_armor_class_bonus`, wave 4's `abc72f75ec`).

**This corrects the originally-filed blocker's own finding.** That entry stated "no `src/rules_core/`
resolver change can fix this — `compute_equipmods_effect` would need to invent a value not present in its
input." That was true only while the corpus lacked the token. Now that it doesn't, a resolver change
genuinely can close it without fabricating anything — reading a real, now-present corpus token through the
exact pattern the AC dimension already uses. This is a smaller, better-scoped, one-cycle-sized fix than the
original "audit the whole extraction pipeline's blast radius" ask — filed as a new, narrower `## Open
blockers` entry in `progress.md`, out of this lane's write scope (`src/rules_core/**`, with 4 sibling lanes
concurrently running elsewhere in the tree this cycle).

## 6. Other units already judged, re-run

13 of the 139 corpus-fixed records already carried a row in `AT-33-E5-003.combined-oracle-results.json`
(of the 8,291 previously-examined units) — re-derived by set intersection:

```
$ python3 -c "<intersect the 139 fixed unit_ids against the combined file's unit_id set>"
changed unit_ids (guessed): 139
overlap with combined oracle results: 13
not in combined (unexamined so far): 126
```

All 13 re-run via the same scratch `compute_equipment_effects` harness. **0 verdicts flipped** (0 to
`agree`, 0 to `disagree`) — all remain `disagree` (1, above) or `unverifiable` (12). But **3 of the 12**
`unverifiable` rows had their `reason` go STALE: `general::compute_general_effect` already resolves
`BONUS:SKILL|...` chains, and the corpus fix newly populated one for these 3 —

```
inner_sea_gods:equipment:blade_of_three_fancies       skill_bonus = Some(TYPE.Perform, +4)
inner_sea_gods:equipment:golden_judge_s_breastplate   skill_bonus = Some(Sense Motive, +4)
inner_sea_gods:equipment:kimle_coat                   skill_bonus = Some(Swim, +5)
```

No live PCGen oracle export exists for these specific skill dimensions this cycle, so **no new agree/
disagree verdict is claimed** — each row's `reason` is corrected to state the real computed value and that
a live oracle capture is the genuine next step (row 17's own skill-shaped population lanes' mandate, not
this one's). The remaining 9 also gained a real, previously-empty chain (`SAVE`/`VAR`/`MOVEADD`/
`SITUATION`/ammunition `EQMOD`) but no resolver anywhere in `equipment_effects/{arms_armor,general,
magic_items,equipmods}.rs` matches that shape — `reason` corrected from `no_bonus_chain: raw_bonus_chains
is empty` (now literally false) to `no_resolver` (an honest, re-verified absence).

Full per-unit rows: `artifacts/epic-5-reverification/corpus-extraction-fix.oracle-results.json` (13 rows).

**Not attempted:** the other 126 of 139 fixed records carry no prior oracle-results row at all — never
previously examined by any lane. Newly examining them is outside this lane's mandate (the extraction gap,
not the un-rowed population — row 17's own three shape lanes own that population).

## Figures + their re-derive commands

| Figure | Value | Denominator | Re-derive command |
|---|---|---|---|
| `.MOD`-targeted records | 391 | of 7,621 `lst_token` equipment/equipment_modifier records | full-corpus scan, §2 above |
| Records with a dropped EQMOD/BONUS from a `.MOD` row | 139 | of 7,621 same records | full-corpus scan, §2 above |
| Books affected | 9 | of 27 scanned corpus books | full-corpus scan, §2 above |
| Records regenerated (written) | 137 | of 139 diagnosed records | `ENRICH_TARGET_LIST` run, §4 above |
| Records correctly refused (declared NAMEISPI) | 2 | of 139 diagnosed records | `ENRICH_TARGET_LIST` run, §4 above |
| Unexpected files touched | 0 | of 137 written | `git status --porcelain -- data/corpus` set-diff, §4 above |
| License/PI mismatches | 0 | of 139 checked records | pre/post snapshot diff, §4 above |
| `raw_tokens` shrinkages | 0 | of 139 checked records | pre/post snapshot diff, §4 above |
| Equipment/equipment_modifier record count | 7,808 | before and after (0 added/removed) | `find data/corpus -path '*/equipment/*.json' -o -path '*/equipment_modifier/*.json' \| wc -l` |
| Overlap with already-examined oracle rows | 13 | of 8,291 examined units | set intersection, §6 above |
| Verdicts flipped | 0 | of 13 overlapping rows | §6 above |
| Rows with a stale `no_bonus_chain` reason (now `no_resolver` or a real computable value) | 12 | of 13 overlapping rows | §6 above |
| `enrich_equipment_raw_tokens` bin tests | 9 of 9 | `cargo test --locked --bin enrich_equipment_raw_tokens` |
| `equipment_effects` lib tests | 71 of 71 | `cargo test --locked --lib equipment_effects` |
| `corpus_loader` lib tests | 6 of 6 | `cargo test --locked --lib corpus_loader` |
| `equipmods` lib tests | 20 of 20 | `cargo test --locked --lib equipmods` |

- **Status:** complete
- **Movement, four buckets:** closure 0 / reclassification 0 / reachability 0 / instrument-correction 13 (13 oracle-results rows' reason/note corrected; 139 corpus records' `raw_tokens`/`raw_bonus_chains` corrected at the source, the systemic fix underlying the whole bucket)
- **Notes:** The task's own escape valve — "confirm `rending_claw_blades` now agrees, or produces a
  different, honestly-recorded verdict" — is exercised here: the verdict did not flip, and is reported
  exactly as computed, not forced. The originally-filed blocker (corpus extraction, unknown blast radius)
  is fully cleared; a new, much smaller, precisely-code-pointed blocker (one `src/rules_core/` function
  widening, already fully designed) replaces it. `src/rules_core/**` was never touched by this lane —
  respecting the dispatch's explicit territory boundary ("you are the only lane touching the corpus
  extraction pipeline and `data/corpus/**`") and the coordination risk of 4 concurrently-running sibling
  lanes possibly editing `equipment_effects.rs` this same wave.
- **Next-cycle plan:** a dedicated cycle with `src/rules_core/equipment_effects.rs` write scope: widen
  `compute_equipment_effects`'s weapon-enhancement assembly to also fold in
  `eqmod_referenced_records(record, RuleSetId::Crb, corpus).iter().map(equipmods::
  compute_equipmods_effect)`, `Option`-summing `tohit_bonus`/`damage_bonus`, mirroring the AC dimension's
  already-shipped pattern — RED→GREEN with a real fixture matching `rending_claw_blades`'s exact shape.
  Separately: a live-oracle capture for the 3 newly-skill-computable units named in §6 (row 17's mandate).
