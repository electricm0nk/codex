# Cycle 10 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 9 costed the remaining reachable work and named its largest item: **47
hits of unframed prose citation, hand work, record by record** — a token printed
mid-sentence inside a string the player or a developer actually reads, wearing
no frame any of the three automatic rules built in cycles 4–6 could find (two of
those three were *removed* for producing ungrammatical sheet lines). This cycle
is that hand work, plus the two files carrying the same shape in
assertion-failure text. **458 → 413 code hits, 45 cleared, 3 files to zero, none
risen, and every one of the 45 came out of the reachable side** —
`hits_inside_cfg_test` did not move by a single hit.

- **Commit SHA:** `5e0329cc63` carries the cycle's work; this line is written
  into the immediately following docs commit (a receipt cannot name the commit
  that carries it). Cycle start `bc97c92776ed2a903f882adba8edc7815cf12742`.

- **Cycle number:** this is cycle **10**, not the 9 the dispatch prompt named.
  Cycle 9 was already committed at `f4583db504` with its receipt tracked at HEAD
  (`bc97c92776`), so writing a `cycle9` receipt would have overwritten a landed
  one. Recorded as `correction 1789175573947-at-35-e6-003-sweep-0d383e`. The
  *remainder* the prompt named (`BONUS:=128; TYPE==107; PRE[A-Z]+:=71;
  DESC:=66; render_pcgen_desc=39; %LIST=28; %CHOICE=13; raw_tokens=5;
  DEFINE:=1`) is cycle 9's **end** state, and `pcgen_residue_gate.py --check`
  reproduced `live_files=59 live_hits=458` exactly at cycle start.

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units
  by design, decisions.md §2)`. Run anyway, for the record —
  `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  This criterion's **own** floor — the 500-code-hit currency cycle 1
  established — is **missed**: **45 code hits cleared of the 500 the floor asks
  for** (458 → 413). The scope taken was the **whole remainder** (458 hits in 59
  files, examined mechanism by mechanism), which is what the floor rule requires
  of a cycle that cannot reach 500 any other way. The shortfall is named with
  its cause under **Refused tokens**: **352 of the 413 that remain are behind an
  operator ruling asked for by cycles 5, 6 and 9 and still open**, and no code
  work of any size reaches them. The *reachable* population at cycle start was
  **106**; this cycle took **45 of it**, the whole prose mechanism.

- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — 32 rendered strings. Every
    `ComputationExplanation.detail` and `ComputationDiagnostic.message` that was
    naming a PCGen token mid-sentence now says the same thing in the rule's own
    words, with the token re-emitted verbatim in a `//` provenance comment
    beside the value where it carried information a reader wants. Two
    `format!` argument lists shortened by the two placeholders their deleted
    `(BONUS:SITUATION|{}=...|{})` parenthetical consumed.
  - `src/rules_core/derived_evaluator_fixture_check.rs` — 6
    assertion-failure strings. `carries no BONUS:VAR|{}| token at all` becomes
    `carries no bonus-variable magnitude named {} at all`; the failing
    developer still learns exactly which variable is missing.
  - `src/rules_core/support_state_matrix.rs` — 5 `next_required_uplift`
    strings, one Dwarf-Hatred row plus the four byte-identical equipment rows.
  - `src/rules_core/pilot_compute/class_slayer.rs`,
    `.../class_ultimate_combat.rs` — one shipped identity-record sentence each.
  - `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs` — **new**. The
    standing gate: it re-derives the count from the five source files rather
    than pinning a recorded figure.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle10_prose_citation_handwork.py`
    — **new**, the hand table with `--apply` / `--check`.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`,
    `kanban.md`; `docs/retro/events/at-35-e6-003-sweep.jsonl`.
  - Folded from the shared checkout, not this cycle's own work:
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (the `derived_at` stamp `completion_atlas.py --check` moves).

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** Run on the cycle's own
  diff (`workflow-instruction.md §6` steps 2 and 4):
  ```bash
  git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD" \
      -- src/rules_core src/pcgen_import src/bin apps/desktop/src-tauri/src tests \
         docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → three matches, **all `tests/…` citations**, none a bundle-tagged identifier in
  shipping code: two are doc-comment prose naming the existing pin
  `tests/sd27_pu_class_feature_descriptions_carry_the_characters_numbers.rs`, and
  the third is the diff's own `+++ b/tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs`
  header. This is the documented citation-exclusion class
  (`scripts/identifier-discipline-audit.sh`, `strip_test_citations`), which
  scans shipping paths only and reads **`OK_NO_BUNDLE_TAGS`** at HEAD — the same
  disposition AT-35-E1-003's receipt recorded for the same shape.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → no match, before and after.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's own closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md §17`). […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` →
  > `live_files=0 live_hits=0 verdict=PASS` […]

  **Not met.** `live_files=56 live_hits=413` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=bc97c92776ed2a903f882adba8edc7815cf12742 target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=360 ratio=n/a builds_recorded=2 pcgen_live_files=56
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit.
  `pcgen_live_files` falls **59 → 56** — three files reached zero.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at end of
  cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=14 hits=92
  pattern DEFINE: files=0 hits=0
  pattern PRE[A-Z]+: files=20 hits=66
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=63
  pattern %CHOICE files=3 hits=13
  pattern %LIST files=6 hits=28
  pattern TYPE= files=23 hits=107
  root src/rules_core files=55 hits=406
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=56 live_hits=413 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=59 live_hits=458`. The count
  fell **only** because tokens left the live side:
  `scripts/pcgen-residue-baseline.env` was **not** edited, `--rebaseline` was
  **not** run, and no pattern, root or exclusion was touched. `DEFINE:` reaches
  **zero across the whole live side** for the first time.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converted rule
  moved.** `data/corpus/` was not written and `data/sheet_rules/` was not
  written; `sheet_rule_convert --check` reproduces cycles 3–9's line field for
  field (below), which is the standing proof. `PCGEN_ORACLE_SHA` unchanged and
  `scripts/pcgen-oracle-pin.env` untouched.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's
    own currency: **45 code hits** across 5 files, 3 of them to zero. The
    test-region census separates code work from reclassification:
    `hits_outside` falls **106 → 61** (45) and `hits_inside_cfg_test` is
    **unmoved at 352**. Every hit cleared came out of executable code; **nothing
    was banked by reclassification**, and the invariance of the 352 is itself
    the standing evidence that that mechanism is a ruling and not a backlog.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched.

- **Per-file movement — every file that moved, summing to 45.** Re-derived by
  applying the gate's own regexes (comment lines excluded) to
  `git show bc97c92776:<file>` and to the file at HEAD, over the union of every
  live-root source file at either tree. **No file rose.**

  | file | before | after | cleared |
  |---|---:|---:|---:|
  | `src/rules_core/pilot_compute/mod.rs` | 56 | 24 | 32 |
  | `src/rules_core/derived_evaluator_fixture_check.rs` | 18 | 12 | 6 |
  | `src/rules_core/support_state_matrix.rs` | 5 | **0** | 5 |
  | `src/rules_core/pilot_compute/class_slayer.rs` | 1 | **0** | 1 |
  | `src/rules_core/pilot_compute/class_ultimate_combat.rs` | 1 | **0** | 1 |
  | | | | **45** |

- **What the player's sheet says now.**

  | was, shipped verbatim | is |
  |---|---|
  | `…suppresses the standard racial trait whose !PREFACT:1,ABILITIES,<flag>=True gate names it` | `…suppresses the standard racial trait its own exclusion gate names` |
  | `…its reach here comes from apg_feats.lst's own \|CATEGORY=FEAT\|Improved Channel.MOD\|, which adds BONUS:VAR\|OracleChannelDC\|2` | `…its reach here comes from the Advanced Player's Guide's own modification of that feat, which adds +2 to the Oracle's channel DC` |
  | `Gated at investigator level 9 by the talent's own PREVARGTEQ:InvestigatorTalentLVL,9` | `Gated at investigator level 9 by the talent's own prerequisite, which requires nine investigator talent levels` |
  | `…the corpus formula token BONUS:VAR\|CombatReflexesAttacks\|DEX resolves to max(Dexterity modifier, 0)` | `…the corpus formula for its extra attacks of opportunity resolves to max(Dexterity modifier, 0)` |
  | `…grants a +2 bonus on Perception checks to notice unusual stonework (BONUS:SITUATION\|Perception=…\|2), transcribed from its corpus token` | `…grants a +2 bonus on Perception checks to notice unusual stonework, transcribed from its corpus situational-bonus entry` |
  | `Corpus: BONUS:ABILITYPOOL\|Sorcerer Bloodline Feat\|BloodlineFeatCount with BONUS:VAR\|BloodlineFeatCount\|(BloodlineFeatProgression-1)/6, which is 0 below 7th` | `The corpus sizes the bloodline feat pool as one per six bloodline levels after the first, which is 0 below 7th` |
  | `…the class block carries no \`CAST:\`/\`KNOWN:\` row at all below level 4, and its caster-level token is itself gated PRECLASS:1,Bloodrager=4` | `…the class block carries no spells-per-day or spells-known row at all below level 4, and its caster level is itself gated on reaching bloodrager level 4` |
  | *(developer-facing)* `corpus row states X but carries no BONUS:VAR\|Y\| token at all` | `corpus row states X but carries no bonus-variable magnitude named Y at all` |

- **Refused tokens:** `BONUS:=92; TYPE==107; PRE[A-Z]+:=66; DESC:=63;
  render_pcgen_desc=39; %LIST=28; %CHOICE=13; raw_tokens=5` — **413 code hits in
  56 files** (the per-pattern lines double-count a line matching two patterns;
  the gate's `live_hits` is 413). **Eight** token types, one fewer than every
  cycle since 6 — `DEFINE:` is gone — and under
  `workflow-instruction.md §8`'s limit of ten. By mechanism, largest first —
  re-derived at HEAD, not carried forward:

  | mechanism | files | code hits | shape |
  |---|---|---:|---|
  | hits inside a `#[cfg(test)]` module of a live file | 38 files are test-only | **352** | **Unmoved by any cycle that does not first get a ruling**, and unmoved by this one: it stood at exactly 352 before and after. The residue gate scans whole files, so an assertion inside a live file counts while the identical assertion in `tests/` does not. Asked for by cycles 5, 6 and 9, still open — see `progress.md`'s `## Open blockers`. **85% of what remains.** |
  | `bestiary/monster_data.rs` `%CHOICE`/`%LIST` description slots | 1 | 20 | `description_variables: &["%CHOICE"]`. Measured and refused for a mechanism by cycle 8 and the measurement still holds: `&'static [&'static str]` with **4,378 literals repo-wide**, serialised into the book cache by `gen_book_cache.rs:1610` and `:1926`. A wire-format cycle. |
  | `FeatEffectBonus.qualifiers` selection targets (`WEAPONPROF=%LIST`, `SCHOOL.%LIST`, `TYPE=`) | 4 | 12 | The bonus *target* is the player's chosen weapon/school. Cycle 7 typed the qualifier **tail**; this is the target the bonus engine matches on, so it moves `damage_total` / `skill_allocation` / `pilot_compute` matching, not a literal. |
  | `render_pcgen_desc` + the `PU_*_DESC_TOKEN` corpus transcriptions it consumes | 4 | 11 | The live prose renderer `AT-35-E6-003` says to delete, plus the four verbatim Pathfinder Unchained `DESC:` strings it substitutes `%N` into — pinned byte-for-byte against the `.lst` files by `sd27_pu_class_feature_descriptions_carry_the_characters_numbers`, so editing them would be a lie about what the book says. Replacement for the renderer exists and is proven (`sheet_rule_catalog::catalog_description_or_fields`); blocked on `class_feature_pool_catalog.rs` reading the sheet-rule package instead of the ingest cache. Both halves leave together. |
  | `race_trait_picker.rs` `raw_tokens` on the desktop side | 1 | 7 | Real `PREVAREQ`/`PREMULT`/`PREABILITY`/`!PREFACT` parsing deciding alternate-trait exclusion. Needs `SheetRule.applies` to carry the exclusion-guard relation; relocating the parse would launder the read, not end it. |
  | `bestiary_3/monster_data.rs` `DESC:&nl;` markers | 1 | 4 | Ingest line-break markers inside a shipped monster description, regenerable only through `transcribe_monster_tables.py`. |
  | `external_ability_refs` `!PRETEMPLATE:` tails | 1 | 3 | The conversion cycle 9 made for `conditions`, **deliberately left again**: `companion_catalog.rs:585` clones this field onto the desktop wire, so it pulls `apps/` in and with it the desktop crate + frontend suites, which this epic runs at wrap-up cadence. 3 hits and one `.map()`. |
  | live parses of the ingest format, scattered | 5 | 6 | `race_resolver.rs`'s `ABILITYSELECTION\|…\|TYPE=` selector prefix, `skinwalker_change_shape.rs`'s `TYPE=` pool prefix, `equipment_effects/{arms_armor,equipmods}.rs` matching `TYPE=Circumstance`/`TYPE=Enhancement`, `corpus_loader.rs` rebuilding a `BONUS:` string, `inner_sea_world_guide/monster_data.rs`'s two `TYPE=Base` slots. Each is a genuine live read needing its own converter-side home; no shared frame, so no single rule reaches them. |

- **Discoveries:** **one, and it is about the gate this cycle wrote rather than
  about the corpus.** A first draft of
  `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs` matched `BONUS:`
  by plain substring and reported **63** findings instead of 45 — it was
  flagging ordinary Rust, `const SLAYER_QUARRY_ATTACK_BONUS: i16 = 2;` and the
  format spec `{MONK_IMPROVED_GRAPPLE_BONUS:+}`, and also `TEMPBONUS:`, none of
  which the residue gate's own `\bBONUS:` matches. **A gate that re-expresses
  another gate's rule must re-express its word boundaries too**; an 18-hit
  overcount would have sent this cycle rewriting correct code. Caught because
  the RED run was read rather than merely observed to be red. No `correction`
  event: the defect never left the working tree and was in a file written this
  cycle.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=56 live_hits=413` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=59 live_hits=458` (start) | same, at `bc97c92776` | same command, at that tree |
  | `45` cleared, `3` files to zero, `0` risen | of the 458 the gate counted at cycle start, over the 59 files carrying them | `458 - 413`; per-file from the gate's own regexes applied to `git show bc97c92776:<file>` against the working file, over the union of live-root source files at both trees |
  | `hits_outside` 106 → 61, `hits_inside_cfg_test` 352 → 352 | the live files carrying hits | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` (unchanged tool, run at both trees) |
  | `42` hand rows applied, covering 45 hits | every rendered string in the five files that the residue gate hit outside a `#[cfg(test)]` region and outside the corpus transcriptions | `python3 …/AT-35-E6-003-SWEEP_cycle10_prose_citation_handwork.py --check` → `rows=42 rows_changed=0` (a non-zero `rows_changed` means work remains) |
  | `45` findings in the RED state, `0` in GREEN | the five scanned files | `cargo test --locked --test sd35_rendered_prose_carries_no_ingest_vocabulary` at `bc97c92776` vs at HEAD |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |
  | `rust_lines_changed`, `pcgen_live_files=56` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since bc97c92776… --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json` |

- **Build scope verified:** run **once**, at the end, after the last
  figure-moving edit.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `test result: ok. 3322 passed; 0 failed;
    15 ignored` (`LIB_EXIT=0`) — unmoved from cycle 9, correctly: the cycle's one
    new test is an integration test, not a lib test.
  - `cargo test --locked --no-fail-fast -j 6` → **415 targets (414 test binaries
    + 1 doc-test target), 8,841 passed, 0 failed, 68 ignored, 0 `test result:
    FAILED` lines**, `FULL_EXIT=0`. 414 → 415 targets and 8,840 → 8,841 tests is
    exactly the one new test binary.
  - `cargo clippy --locked --tests -j 6` → `CLIPPY_EXIT=0`, 0 warnings.
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `records=49438
    converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS
    (114.7s)` — identical to cycles 3–9 on every field; the 142 refusals are all
    `no_corpus_record`. This is the standing proof that no converted rule moved.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
    (fell by 45).
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → `0`.
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`,
    `EXIT=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233
    shapes=1 verdict=PASS`, `EXIT=0`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, `EXIT=0`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, `EXIT=0`.
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-…/*.md'
    'docs/release/SD-35-…/artifacts/**/*.md'` → `files_checked=118
    violations=0`.
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched
    **no** file under `apps/`; `git diff --stat bc97c92776..HEAD -- apps/` is
    empty. That is also why the 3 `external_ability_refs` hits were left again.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus
    record and no classifier changed; `docs/work-inventory.json` is
    byte-identical to `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`diff -q` →
    identical; `regressed=0 added=0 dropped=0` agrees).

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA` unchanged; no figure in this receipt came
  from the pinned corpus. Every string edited was read out of the shipped
  tables, which carry their own source-file and source-line provenance.

- **RED→GREEN preserved, recorded.** The gate test was written **before** a
  single string was edited and run against the untouched tree:
  ```
  ---- rendered_prose_carries_no_pcgen_ingest_vocabulary ----
  45 line(s) of prose this engine writes still print PCGen ingest vocabulary on
  a player's sheet (sheet rule, decisions.md §1). …
  src/rules_core/pilot_compute/mod.rs:14699: `BONUS:` in rendered prose -- …
  src/rules_core/support_state_matrix.rs:7466: `BONUS:` in rendered prose -- …
  test result: FAILED. 0 passed; 1 failed
  ```
  and green after the table was applied. The 45 it listed and the 45 the residue
  gate lost are the same 45, checked file by file in the per-file table above.

- **Retro events emitted:**
  `correction 1789175573947-at-35-e6-003-sweep-0d383e` (the dispatch's cycle
  number), `deferral 1789175574167-at-35-e6-003-sweep-5134fc` (the 413-hit
  remainder, mechanism by mechanism).

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** the one judgment call was declining to edit the four
  `PU_*_DESC_TOKEN` constants. They are the book's own words held verbatim and
  pinned against the `.lst` files on disk; rewording them to satisfy a gate
  would have been the masking cycles 4–5 were burned by. They are counted in the
  refused remainder under the `render_pcgen_desc` mechanism that owns them, not
  exempted from it.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole
  remainder, **413 code hits in 56 files**. The 352 behind the `#[cfg(test)]`
  ruling cannot move without it, so the reachable batch is **61 hits in 18
  files** — and after this cycle **there is no hand work left in it**. What
  remains is four converter-side jobs, each of which is a whole cycle: the
  `bestiary` `description_variables` wire-format regeneration (20), the
  `FeatEffectBonus` selection-target typing that moves the bonus engine's
  matching (12), the `pcgen_desc.rs` deletion with its `PU_*_DESC_TOKEN`
  transcriptions (11, blocked on the catalog rewire), and `SheetRule.applies`
  carrying the exclusion-guard relation for `race_trait_picker.rs` (7). The 3
  `external_ability_refs` hits plus the desktop suites remain worth folding into
  whichever cycle next touches `apps/`. **The single highest-value action for
  this criterion is not a cycle at all — it is the `#[cfg(test)]` ruling**,
  which now stands between the gate and 352 of the 413 code hits that remain.
