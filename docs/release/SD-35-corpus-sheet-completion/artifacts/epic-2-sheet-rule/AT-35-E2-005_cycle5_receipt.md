# Cycle AT-35-E2-005_cycle5 — Epic 2 — Sheet rule / AT-35-E2-005

**This is a re-dispatch of an already-closed criterion, not new work.** AT-35-E2-005 was closed
against its **amended bar** on 2026-09-08 (`epic-breakdown.md` `### AT-35-E2-005` amendment;
`decisions.md §16`; `kanban.md` row 11 `complete` at `cd3d64e578`), after four cycles and a
disposition cycle. The dispatch that produced this receipt carried `CYCLE NUMBER FOR THIS
CRITERION: 1` and the scope flag `--min 500` (whole remainder) — both stale relative to the
branch. This cycle therefore did what the four preceding Epic-2 re-dispatches did
(`969d5b9402` AT-35-E2-001, `ca14f2363f` AT-35-E2-002, `9f1b27dcdf` AT-35-E2-003, `8a5b19e7bb`
AT-35-E2-004): it **re-verified every clause of the criterion's bar at HEAD**, changed no code,
no data and no script, and closed zero units. It did **not** start a fifth grinding cycle —
`workflow-instruction.md §8` forbids it and the orchestrator's re-scope already ruled on it.

- **Commit SHA:** `ad6da1bbf24709121ec9f9c8844d2a92f4d9e057` is the tree verified — unchanged by
  this cycle apart from `docs/`. Cycle start `ad6da1bbf2`. The docs-only commit carrying this
  receipt, `progress.md`, `kanban.md` and the two retro events is
  `77fa8a0d31` on `tranche/15`, pinned here by the follow-up commit (the pattern `ca14f2363f`
  set on this branch).
- **Scope gate:** `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS` — the literal last
  line of `python3 scripts/cycle_scope_gate.py --min 500` (no flags = the whole remainder), run at
  cycle start on the rebased tree. Full output:
  `scoped_by_bucket=A:1 B:437 C:79 D:43 M:63 U:202 V:392 X:168 Z:19`;
  `scoped_by_kind=ability:91 class:144 class_feature:641 companion:13 equipment:188
  equipment_modifier:40 feat:93 monster:2 power:1 race_trait:168 skill:7 spell:6 template:9 trait:1`.
  The gate passes; the **criterion** does not scope those 1,404 units under its amended bar —
  they are owned, unit for unit, by `### AT-35-E2-005-DISPOSITION`, re-proved below.
  `python3 scripts/pcgen_residue_gate.py --check` at cycle start: `live_files=260 live_hits=12736
  baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:** `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle5_receipt.md` (this file, new),
  `progress.md`, `kanban.md`, `docs/retro/events/at-35-e2-005.jsonl` (1 `correction` + 1 `deferral`
  appended), plus the live `docs/retro/events/sd31-transcribe.jsonl` append folded from the shared
  checkout (the standing "clean tree = unfiltered `git status` empty" rule), and three reflowed
  prose lines in `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle2_receipt.md` and
  `AT-35-E2-004_cycle2_receipt.md` that turned `verify.sh --only figure-provenance` from red to
  green (**no figure changed** — the re-derive commands were already there, one line below the
  figure they source, and the gate is per-line; see Discoveries). **No `src/`, no
  `scripts/`, no `data/`, no `apps/`, no `tests/`, no `docs/work-inventory.json`.** Two runs
  re-stamped `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`'s
  `derived_at` (one line, `942c8d3ae5` → `ad6da1bbf2`, outside this epic's file-touch set); reverted
  with `git checkout --` both times, the same disposition AT-35-E2-001/002/003/004 recorded.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS —
  `git diff --unified=0 fe5ae6cd4a...HEAD -- <Epic 2 file-touch set> scripts/verify.sh tests/sheet_rule_convert_gate.rs ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  over the whole Epic 2 diff since `develop` prints nothing. Re-run unchanged on the final diff.
- **Wired-integration audit result:** **9 diff lines / 4 files**, every one rulebook prose or the
  source's own editorial wording — **none in `src/`, `scripts/`, `apps/` or `tests/`**. Attributed
  by `+++` header: `data/sheet_rules/bestiary_3/monster_ability/tophet_swallow_whole.json` 1
  ("hack or smash its way out"), `data/sheet_rules/core_rulebook/spell/plant_growth.json` 1
  ("hack or force a way through"),
  `data/sheet_rules/ultimate_intrigue/class_feature/courtly_hunter_courtly_companion.json` 1
  (PCGen's own bracketed "[… not yet implemented]" editorial note in the rules text),
  `docs/work-inventory.json` 6 (the three `empty_selection_standard_*` records whose `reason`
  says PCGen's row is a CHOOSE-menu **placeholder** — 3 removed + 3 added lines of the same three
  records). **Identical, line for line and file for file, to AT-35-E2-004 cycle 2's accounting.**
  Re-run unchanged on the final diff.
- **Acceptance criterion:** Run the converter over every record, regenerate `data/sheet_rules/`,
  regenerate the inventory once. Report: units moved into DONE by bucket and kind (id-set diff),
  the refused report by token type, the projected remainder per token type, and the pass's wall
  time. **No mapping row is added in this cycle.** Then run the oracle harness over the fixture
  roster and compare every `Number` value the evaluator produces against PCGen's exported total
  for the same character — this is the "use PCGen to test the rewrite" check, and it runs here
  first so a wrong mapping is caught before Epics 3–4 build on it. **Evidence:** the receipt with
  `cycle_scope_gate.py --receipt` rows; `token-coverage.json` re-derived; `completion_atlas.py
  --check` before and after; the oracle comparison with `PCGEN_ORACLE_SHA`, `compared=<n>
  agree=<n> disagree=<n>` and every disagreement named with its `Expr` and PCGen's value.
  **Amended bar (2026-09-08, `decisions.md §16`)** — the four clauses this cycle re-proved at HEAD:
  (1) the corpus-wide pass ran and was **measured**; (2) the report and the ledger were
  **re-derived**; (3) the oracle harness ran at the pin and **agrees**; (4) **zero mapping rows
  were added**.
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a
  builds_recorded=1 pcgen_live_files=260` — the literal last line of
  `python3 scripts/cycle_scope_gate.py --receipt --since ad6da1bbf24709121ec9f9c8844d2a92f4d9e057 --before /tmp/wi-before-AT-35-E2-005.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005`
  (full output also `closed_by_kind=` empty, `relabeled_moves=` empty, `regressed=0 added=0
  dropped=0`, `residue_gate=present`). `rust_lines_changed=0` is the point of this receipt:
  nothing outside `docs/` was written.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736
  verdict=PASS` — identical at cycle start, at the end of the chain, to cycles 1–4's, and to all
  four preceding Epic-2 re-dispatches'. `identifier_files=68 identifier_hits=514`.
  **No live-side PCGen read was added; `pcgen_live_files` did not rise.**
- **Oracle parity:** **`compared=42 agree=41 disagree=1`** (`unverifiable=5`) over the evaluator's
  `Number` values, **`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`** — re-run at
  HEAD on an **isolated worktree that pushed nothing** (`/home/ubuntu/workspace/wt-sd35-e2005-oracle`,
  detached at `ad6da1bbf2`, its own `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E2-005-oracle`;
  `workflow-instruction.md §2` worker split). Engine side: `cargo build --locked --release -j 6
  --bin sheet_rule_parity` then `$CARGO_TARGET_DIR/release/sheet_rule_parity --roster
  <oracle-parity>/roster --output /tmp/oracle-ours.json` → `characters=29 lines=270 wall=5.7s`
  (`/usr/bin/time -v`: elapsed 0:05.99, max RSS 441,728 kB). Compare side:
  `python3 scripts/oracle_harness/sheet_parity.py compare --ours /tmp/oracle-ours.json --exports
  <oracle-parity>/exports --output /tmp/oracle-sheet-parity.json` →
  `lines compared=42 agree=41 disagree=1 unverifiable=5; chassis compared=382 agree=376
  disagree=6 unverifiable=140; characters=29 exports_missing=0`. The produced
  `sheet-parity.json` is **byte-identical** to the committed one (`cmp` → identical), so the
  committed artifact is re-proved at HEAD and **not rewritten**. The exports were **not**
  re-exported: no live-side, converter or template change since cycle 3, and the pin is unchanged.
  **The one `Number` disagreement, named (unchanged from cycles 3–4):**
  `deterministic_human_fighter_l1`, unit
  `target:WeaponAttack:{"Chosen": "core_rulebook:feat:weapon_focus"}`, **ours 0, PCGen 1**
  (`oracle_key` `WEAPON.0.TOTALHIT-ATTACK.MELEE.TOTAL`) — `Expr`
  `{"Number": {"Var": "vb1e14268d73c2def"}}`, whose var table
  `data/sheet_rules/_vars/vb1e14268d73c2def.json` carries one contribution, `Const(1)` (`Base`)
  declared by `core_rulebook:class_feature:default`, a rule the held set does not hold — a
  **holdings gap owned by AT-35-E3-001**, not a mapping or evaluator defect. The 5 unverifiable:
  `SPELL-dc-not-numeric` 4 (PCGen prints a blank `DC` for a no-save spell), `no-component-export`
  1 (`Other:accheck`). The 6 chassis disagreements are Halfling Luck's +1 and Divine Grace's +3 on
  the three saves (`halfling_fighter_l1` `CHECK.{0,1,2}.TOTAL` ours 1/1/2 vs 2/2/3;
  `human_paladin_l10` ours 6/3/9 vs 9/6/12) — Epic 3 / Epic 5 holdings, as cycles 2–4 recorded.
  **Blocker B1 (this criterion's assigned owner) is satisfied and stays satisfied:** the oracle
  export tokens for skill, speed, DR, DC and spells-per-day exist in
  `oracle-parity/exports/_template/sheet-totals.txt.ftl` and are populated in all 29 exports —
  `exports_missing=0`, `chassis compared=382`, and the only `unverifiable` reasons are
  `engine-posture-absent` 140, `SPELL-dc-not-numeric` 4 and `no-component-export` 1, none of which
  is a missing export token.
- **Movement, four buckets:** closure 0 / relabel 0 / reachability 0 / instrument-correction 1
  (retro `correction` `1788894275228-at-35-e2-005-1d1792`).
- **Refused tokens:** the ledger's remainder at HEAD, by the token type each refusal arose under,
  **non-DONE units** (a unit refused under k token types counts under each; **49 token types, sum
  with multiplicity 850, over 659 distinct non-DONE refused units of 1,404 non-DONE**):
  `ABILITY=200, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66, BONUS:[redacted PI]=62,
  BONUS:VAR=60, DEFINE (PI-redacted token)=40, DESC=40, unmapped:MODTOSKILLS=37,
  unmapped:SPELLSTAT=23, BONUS:COMBAT=19, unmapped:MEMORIZE=19, BONUS:SKILL=15,
  ASPECT:<display sub-key>=13, unmapped:SPELLLIST=12, BONUS:EQM=11, BONUS:STAT=11,
  BONUS:ITEMCOST=10, BONUS:MOVEADD=9, BONUS:SITUATION=9, BONUS:MISC=5, token-less=5,
  unmapped:KNOWNSPELLS=5, PREVARGTEQ=4, PREVARNEQ=4, TEMPBONUS=4, [redacted PI] token=4,
  unmapped:NUMPAGES=4, unmapped:SPELLBOOK=4, BENEFIT=3, BONUS:HP=3, BONUS:WEAPONPROF=<name>=3,
  HITDIE (%-step)=3, unmapped:BONUSSPELLSTAT=3, ASPECT:CheckCount / ASPECT:CheckType=2,
  BONUS:ABILITYPOOL=2, BONUS:SKILLRANK=2, unmapped:DOMAIN=2, unmapped:PRESPELLSCHOOL=2, ADD=1,
  ASPECT:NAME=1, BONUS:DR=1, BONUS:EQMWEAPON=1, BONUS:PCLEVEL=1, BONUS:SAVE=1, DR=1,
  NATURALATTACKS=1, PREVAREQ=1, SIZE (formula)=1, unmapped:ITEMCREATE=1`.
  **Identical, type for type and count for count, to cycles 1–4's and to AT-35-E2-004 cycle 2's.**
  All statuses: 1,810 refused of 49,438. The scoped population (1,404) is larger than the closed
  count (0), so a `deferral` event is owed and was emitted:
  `1788894965735-at-35-e2-005-1f0f22`.
- **Discoveries:** **one**, and it is an instrument discovery, not a mechanism or an atlas one.
  `oracle-parity/ours.json` **embeds the absolute `--roster` path it was run with**, so the
  engine-side artifact is not byte-stable across trees even when the engine is: this cycle's
  worktree run differs from the committed file at byte 195592 **only** in that key
  (`"roster": ".../wt-sd35-e2005-oracle/..."` vs `".../repos/codex/..."`), while
  `characters` (n=29) and `generated_by` are byte-identical and the derived `sheet-parity.json`
  is byte-identical. Cycle 4 used `cmp` on `ours.json` as its engine-stability test; that test is
  path-sensitive and would read as an engine regression for any cycle running the harness from a
  worktree — which `workflow-instruction.md §2`'s worker split now mandates. Emitted as a
  `correction` (`1788894275228-at-35-e2-005-1d1792`) with the re-derive command; the committed
  `ours.json` was **left as it is** rather than overwritten with a worktree path. The
  right stability test is the semantic one on `characters`, or `cmp` on `sheet-parity.json`.
  **A second, smaller one:** `scripts/verify.sh --only figure-provenance` was **already red at
  HEAD**, `violations=3 of figures_examined=189`, on three lines of AT-35-E2-003 cycle 2 and
  AT-35-E2-004 cycle 2 where the wrapped bullet put the figure on one line and its re-derive
  command on the next — the gate matches per line. It is not in `§6` step 3's chain, so four
  cycles ran past it. Reflowed (no figure touched); `RESULT: PASS files_checked=162
  figures_examined=189 violations=0` now, this cycle's own receipt included.
- **Figures + their re-derive commands:**
  - **Measure-first sample, before the population run** (standing lesson: project wall time from a
    small sample). `n=3` single-unit runs,
    `/usr/bin/time -f "%e" $CARGO_TARGET_DIR/debug/sheet_rule_convert --one <id>` for
    `core_rulebook:feat:weapon_focus` **33.93 s**, `core_rulebook:spell:plant_growth` **32.19 s**,
    `core_rulebook:class_feature:default` **31.79 s** (mean **32.6 s**, spread 2.1 s).
    `convert_one` converts the whole repo and then selects one record, so **the marginal
    per-record cost is below this run's noise floor (`src/bin/sheet_rule_convert.rs`: < 2.1 s over
    49,437 further records, < 0.04 ms each; `cargo run --locked --bin sheet_rule_convert -- --check`) and the pass is entirely fixed-cost**. **Projection
    stated before the full run: the corpus-wide conversion approx 33 s, plus the `--check`
    freshness comparison of the 66,514 on-disk rule files / 245 MB (`cargo run --locked --bin sheet_rule_convert -- --check`),
    measured at 110.6 s by AT-35-E2-004 cycle 2 → projected `--check` wall approx 145 s.**
    **Actual: 117.7 s** (`/usr/bin/time -f "TOTAL_WALL=%e s MAXRSS=%M kB"` →
    `TOTAL_WALL=117.74 s MAXRSS=767764 kB`) — 27.3 s under the 145 s projection, which was
    conservative because it added the two costs that in fact overlap. Denominator: 49,438 records (`cargo run --locked --bin sheet_rule_convert -- --check`),
    the population `cargo run --locked --bin sheet_rule_convert -- --check` (`src/bin/sheet_rule_convert.rs`) prints.
  - **The corpus-wide pass, measured (amended-bar clause 1):**
    `$CARGO_TARGET_DIR/debug/sheet_rule_convert --check` →
    `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS (116.2s)`,
    exit 0. Denominator 49,438 = the atlas population (`python3 scripts/completion_atlas.py --check`).
    **Identical record/converted/refused/rules/
    var_tables figures to cycles 1–4 and to AT-35-E2-004 cycle 2** — the conversion is stable at
    HEAD. (This is the debug binary; cycle 4's 25.64 s figure was the release binary's conversion
    phase alone, not the `--check` comparison.)
  - **Per-kind conversion**, same command's tail: `ability 4337/4246/91, class 185/3/182,
    class_feature 18043/17683/360, companion 1696/1682/14, deity 459/458/1, domain 183/183/0,
    equipment 6223/6044/179, equipment_modifier 1532/1504/28, feat 2764/2030/734, language
    136/136/0, monster 1270/1257/13, monster_ability 3806/3795/11, power 421/420/1, race 95/68/27,
    race_trait 2561/2431/130, skill 149/141/8, spell 2843/2829/14, template 2248/2234/14, trait
    487/484/3` (records/converted/refused).
  - **No source-format literal survives the conversion:**
    `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**.
    Denominator: all 66,514 rule files under `data/sheet_rules/` (`cargo run --locked --bin sheet_rule_convert -- --check`),
    the `rules=` figure of `cargo run --locked --bin sheet_rule_convert -- --check`.
  - **The ledger re-derived (amended-bar clause 2):** `python3 scripts/token_coverage.py --check` →
    `non_done=1404 tokened=1399 token_less=5 refused=1810 refused_non_done=659 token_types=231
    shapes=81 verdict=PASS`, exit 0, all six internal checks `ok=True`
    (`population census_entries=49438 inventory_units=49438 report_records=49438`;
    `double_count duplicate_records=0`; `coverage uncovered=0`; `refused_set census_refused=1810
    refused_json=1810 union_over_token_types=1810`; `shape_totals shapes=81`;
    `partition token_types=231`). `token-coverage.json` was rewritten byte-identically — the file
    does not appear in `git status --porcelain`.
  - **The atlas, before and after (amended-bar clause 2):** `python3 scripts/completion_atlas.py
    --check` → **identical both times**, exit 0: `population=49438 buckets=10 unclassified=0
    overlap=0`, `DONE 48034 / A 1 / B 437 / C 79 / D 43 / M 63 / V 392 / U 202 / X 168 / Z 19`,
    `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
    `citation_failures=0`. Denominator 49,438 and 48,034 DONE of 49,438 = 97.16 percent (`python3 scripts/completion_atlas.py --check`) — both from
    that same `python3 scripts/completion_atlas.py --check` run.
  - **Zero mapping rows added (amended-bar clause 4):** `rust_lines_changed=0` from the receipt
    command above, and `git status --porcelain` lists no path under `src/`, `scripts/` or
    `data/` at any point in this cycle.
  - **No unit orphaned:** `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py`
    → `non_done=1404 atlas_non_done=1404 refused_non_done=659 not_refused_non_done=745
    owned_sum=1404 unowned=0 duplicate_ids=0 verdict=PASS`, exit 0;
    `by_owner AT-35-E4-001=659 AT-35-E4-002=391 AT-35-E5-003=217 AT-35-E5-004=137`
    (659+391+217+137 = 1,404 — the `by_owner` line of that same `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py` run,
    run from the repo root). Re-derived at HEAD from the live inventory, `_refused.json` and
    `token-coverage.json`, not copied from a receipt.
  - **The other two inherited instruments:** `python3 scripts/shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=363 citation_ok=True`, exit 0;
    `python3 scripts/missing_engine_tables.py --check` → `population=1 kinds=1 (power 1)
    citation_failures=0`, exit 0.
  - **Package prose:** `python3 scripts/denominator_gate.py --check
    'docs/release/SD-35-corpus-sheet-completion/*.md'
    'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` →
    `files_checked=44 violations=0`, exit 0. `scripts/verify.sh --only pi-sweep` →
    `PASS pi-sweep (11 hits over src/rules_core/rules_tables, 11 baseline rows)`, `RESULT: PASS`.
- **Build scope verified:** `cargo test --locked --no-run -j 6` → **exit 0**, `NORUN_WALL=1.75 s`
  (warm target dir; the cold figure for AT-35-E1-003's baseline is the launch checklist's
  2 min 45 s and this cycle did not disturb it). `cargo test --locked --lib -j 6` → **3217 passed;
  0 failed; 14 ignored**, exit 0, 52.43 s. `cargo test --locked --test sheet_rule_convert_gate -j 6`
  → **27 passed; 0 failed**, 177.66 s — the per-kind converter gates that read the live corpus
  directory. `cargo clippy --locked --tests -j 6 --bin sheet_rule_convert --bin v06_work_inventory`
  → **0 warning/error lines**, exit 0. **`cargo test --locked --no-fail-fast -j 6` was not run:
  `workflow-instruction.md §6` step 3 requires it "when `src/` or the classifier changed", and this
  cycle changed neither (`rust_lines_changed=0`); the workspace run is the epic wrap-up's (§10),
  and it was last green over 590 targets / 8,656 tests at the cut.** The desktop crate and the
  frontend did not run: `apps/` was not touched, so §6 defers them to the epic wrap-up. All run at
  `ad6da1bbf24709121ec9f9c8844d2a92f4d9e057`.
- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` was not run
  (§6 step 3 runs it "only when corpus records changed"). Its last population is the launch
  checklist's 48,706 of 51,476 records examined, 0 findings.
  **The inventory regeneration the criterion names was attempted and correctly refused:**
  `$CARGO_TARGET_DIR/debug/v06_work_inventory` (723.07 s) exits 1 with
  *"refusing to write docs/work-inventory.json: this run would drop 7395 of the 31605 verification
  stamp(s) it currently carries"* unless `CORPUS_LITERAL_SWEEP_REPORT` and
  `DERIVED_FIXTURE_CHECK_REPORT` are set to the sweep's and the fixture check's `--json-out`
  reports. The named offenders are SD-34 `oracle-agree` stamps
  (`advanced_class_guide:class_feature:slayer_sneak_attack`,
  `advanced_class_guide:class_feature:bloodrager_damage_reduction`, …), **not** `sheet-complete`
  stamps — `data/sheet_rules/` is present and fresh (`--check verdict=PASS` above). This is the
  guard working as designed; `--allow-stamp-loss` was **not** passed (§6 step 3) and
  `docs/work-inventory.json` is **byte-unchanged**, which is the correct outcome for a cycle whose
  corpus, converter and classifier are all unchanged. The token census the run rebuilt on the way
  (`49438 record(s) in data/sheet_rules/_tokens.json; 48599 of 49438 unit(s) carry a token list`)
  matches the ledger's.
- **Oracle pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; the
  29 exports in `oracle-parity/exports/` are cycle 3's, run against `$PCGEN_REPO_DIR/build/install/pcgen`
  at that pin; this cycle joined the HEAD engine output to them from the isolated worktree).
- **Status:** complete — against the amended bar (`epic-breakdown.md` `### AT-35-E2-005`
  amendment 2026-09-08; `decisions.md §16`), every clause of which is re-proved at
  `ad6da1bbf24709121ec9f9c8844d2a92f4d9e057` above: the pass measured (117.7 s, projected 145 s
  from an `n=3` sample stated first), the report and ledger re-derived
  (`non_done=1404 refused_non_done=659 shapes=81 verdict=PASS`), the atlas checked before and
  after (identical, `unclassified=0 overlap=0 done_evidence_violations=0`), the oracle harness run
  at the pin and agreeing (`compared=42 agree=41 disagree=1 unverifiable=5`, the one disagreement
  named with its `Expr` and PCGen's value and owned by AT-35-E3-001), and zero mapping rows added
  (`rust_lines_changed=0`). The 1,404 non-DONE units are **not** dropped: `owned_sum=1404
  unowned=0 verdict=PASS` from the disposition hand-off, re-derived at HEAD.
- **Notes:** the dispatch's "cycle 1" and its `--min 500` whole-remainder scope were stale; this
  cycle is #5 and is a re-verification. Starting a fifth grinding cycle would have been
  byte-identical to cycles 3 and 4 and is what `workflow-instruction.md §8`'s ">10 distinct
  refused token types — re-scope, do not grind" and the orchestrator's own re-scope both forbid.
- **Next-cycle scope:** criterion at zero against its amended bar. The remainder is dispatched by
  owner, not by this criterion: `AT-35-E4-001` 659 (the refused set, any bucket), `AT-35-E4-002`
  391 (V, non-refused), `AT-35-E5-003` 217 (U 198 + Z 19), `AT-35-E5-004` 137 (X, non-refused).
