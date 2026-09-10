# Cycle 2 — Epic 4, Resolve and verify / AT-35-E4-001

Re-dispatch of a criterion cycle 1 already closed (`AT-35-E4-001_cycle1_receipt.md`, `9bae2cfa1f`).
This cycle moves no unit — it re-derives all three Evidence clauses **at HEAD**, five epics of
work later, and corrects the dispatch's stale scope figures. It is a `PASS_WHOLE_REMAINDER`
cycle, not a floor exemption.

- **Commit SHA:** `a3c5d455b6` (this receipt landed in it; its own SHA row pinned by the follow-up commit below, the `AT-35-E3-004_cycle2` precedent)
- **Cycle start SHA:** `137658f31a8f87c0dc2af34ed9e3bdf79b4207b2`
- **Scope gate:**
  ```
  inventory=docs/work-inventory.json
  scope=bucket=M
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The dispatch mandated bundling other buckets to clear the 500 floor. There is nothing to
  bundle: `remaining_non_done=0` for **every** bucket at HEAD, so the bucket-M scope already
  *is* the whole remainder and the gate passes on that verdict. Re-run without the flag gives
  the identical line (`scope=(whole remainder)`). No other criterion's card changes state here —
  every one of them was already at `complete` before this cycle started.
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle2_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `kanban.md` (§6 step 7)
  - `docs/retro/events/at-35-e4-001.jsonl` (one `correction`, appended)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (the atlas
    instrument's own `derived_at` stamp, rewritten by `completion_atlas.py --check`)
  - `docs/retro/events/sd31-transcribe.jsonl` (one append left live on the shared checkout by
    another session; folded, not filtered away — precedent `c15e64bc3e`)

  No `src/`, no `data/`, no `scripts/`. `rust_lines_changed=0`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
  `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47` (`git merge-base HEAD origin/develop`);
  `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/pcgen_import/sheet_rule/ data/sheet_rules/ scripts/oracle_harness/ docs/work-inventory.json docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → **1 line**, and it is not a bundle tag in code: an earlier receipt's own prose quoting the
  audit command, whose pathspec contains the literal package directory
  `docs/release/SD-35-corpus-sheet-completion`. No identifier in any code or data file matches.
- **Wired-integration audit result:** OK_NO_TOKENS.
  Same pathspec, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → **17 lines** before this receipt landed and **20** after it (`git diff --unified=0 fe5ae6cd4a...HEAD -- <the same pathspec> | grep -cE '…'`); the delta of **3** is this receipt's own quotation of the audit grep, and the final-diff re-run bounded to this cycle alone
  (`git diff --unified=0 137658f31a..HEAD -- <the same pathspec>`) returns exactly those 3 and
  nothing else. None is in a shipping code path, and every pre-existing one is already recorded by
  `AT-35-E4-001_cycle1_receipt.md` / `AT-35-E3-002_cycle1_receipt.md`:
  generated Paizo prose inside `data/sheet_rules/` (*"creatures must hack or force a way
  through"*, `core_rulebook:spell:plant_growth`; *"Once swallowed by a tophet…"*), the bracketed
  upstream editorial note *"[Change to magical beast and stacking restriction not yet
  implemented]"*, receipt prose quoting this very grep, and **removed** (`-`) `"no selection"`
  placeholder rows deleted from `docs/work-inventory.json`. This cycle's own diff adds no match
  outside this receipt's quotation of the audit. **Identifier audit on this cycle's diff alone:
  0 matches.**
- **Acceptance criterion** (verbatim, `epic-breakdown.md § AT-35-E4-001`):
  > 4,334 units at authoring: ability 1,483, race_trait 697, spell 558, feat 518,
  > equipment_modifier 443, template 305, trait 123, equipment 99, domain 67, skill 29, deity 9,
  > race 3. Each cycle adds **converter mapping rows** for one token family (`BONUS:SKILL|…`,
  > `BONUS:SAVE|…`, `DR:`, `SR:`, `SPELLS:`, `TEMPBONUS:` → `Text` with its condition, `%CHOICE`
  > aliases → `Choice`), re-runs the converter, regenerates. Every new `Number` mapping is
  > oracle-checked on the fixture roster in the same cycle.
  >
  > **Evidence:** `completion_atlas.py --check` reports M at 0; `token-coverage.json` shows every
  > compute-bearing token type with a mapping row or a named refusal with count; the oracle
  > comparison per cycle with disagreements named.
  >
  > **Inherited from AT-35-E2-005 (2026-09-08, `decisions.md §16`, `### AT-35-E2-005-DISPOSITION`):**
  > the **659** converter-refused non-DONE units of 1,404 at `38b67db94e` … This criterion's bar
  > is therefore "M at 0 **and** the refused set at 0", not M alone.

  **The bar, clause by clause, re-derived at HEAD `137658f31a`:**
  1. **M at 0** — `completion_atlas.py --check` → `DONE: 49438`, `M: 0`, and every other bucket 0.
  2. **Every compute-bearing token type mapped or refused-with-a-count** —
     `token_coverage.py --check` → `unmapped_token_types=0` of `token_types=231`, `verdict=PASS`,
     all seven internal checks `ok=True`.
  3. **Oracle comparison with disagreements named** — re-run at HEAD; all 7 named below.
  4. **Inherited clause, the refused set at 0** — `refused=142`, `refused_non_done=0`; the atlas
     bucket of all 142 refused ids is `DONE` (they print their words under the sheet rule).
- **Receipt rows (mechanical):**
  ```
  since=137658f31a8f87c0dc2af34ed9e3bdf79b4207b2 residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=253
  ```
  `closed=0` / `relabeled=0` is the correct and only possible reading: the population was
  **already 0 non-DONE** at the cycle start. `ratio` is `n/a` — a division by zero, never `0.0`.
  `rust_lines_changed=0` because this cycle writes no Rust. `pcgen_live_files=253`, seven
  **below** the 260 baseline (Epic 6's fall, never a rise).
- **PCGen residue:**
  ```
  live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Identical at cycle start and at cycle end. Nothing on the live side was touched.
- **Oracle parity:** `lines compared=146 agree=145 disagree=1 unverifiable=16`;
  `chassis compared=382 agree=376 disagree=6 unverifiable=140`; `characters=29 exports_missing=0`,
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
  **The re-run at HEAD reproduced `AT-35-E4-001_cycle1_sheet-parity.json` byte for byte**
  (`sha256 5202e278dd49c017ddf1fe60a0458e7c962d1ea74f677a89b95f9665f7a082fb` for both), so no
  duplicate artifact is committed — the cycle-1 file **is** this cycle's evidence, now shown
  reproducible five epics later. **0 new mapping rows, 0 of them `Number`**, so no new Number
  mapping was owed a check; the run was made because the Evidence sentence asks for one per
  cycle. All 7 disagreements, unchanged and all pre-existing (**0 introduced, 0 fixed**):
  - `deterministic_human_fighter_l1` `target:WeaponAttack:{"Chosen":"core_rulebook:feat:weapon_focus"}` ours=0 oracle=1 (`WEAPON.0.TOTALHIT-ATTACK.MELEE.TOTAL`)
  - `halfling_fighter_l1` `save.fortitude.total` / `save.reflex.total` / `save.will.total` ours=1/1/2 oracle=2/2/3 (`CHECK.{0,1,2}.TOTAL`)
  - `human_paladin_l10` `save.fortitude.total` / `save.reflex.total` / `save.will.total` ours=6/3/9 oracle=9/6/12 (`CHECK.{0,1,2}.TOTAL`)

  These are chassis rows (the halfling's racial save bonus, the paladin's divine grace), not
  sheet-rule lines: AT-35-E4-002 booked them as Epic 6's parity baseline
  (`1788955474431-at-35-e4-002-0f136c`). This cycle neither caused nor cleared any of them.
- **Movement, four buckets:**
  - **closure (into DONE):** 0 — the population was 0 non-DONE before the cycle started.
  - **relabel (bucket to bucket):** 0. No unit's bucket, status or evidence changed.
  - **reachability:** unchanged — `degraded_records=603`, `refused=142` (one shape,
    `no_corpus_record`), `converted=49296` of `records=49438`.
  - **instrument-correction:** one — the dispatch prompt's scope figures (below), emitted as
    `correction` `1789048470735-at-35-e4-001-a8c709`. No instrument's own output changed.
- **Refused tokens:** **none.** No refusal was added and none cleared. `_refused.json` holds
  142 entries under the single shape `no_corpus_record`, `refused_non_done=0`, and
  `unmapped_token_types=0`, so the criterion's population is zero by token family as well as by
  bucket. **Criteria emptied by this cycle: none** — all were already `complete`.
- **Discoveries:** none of the mechanism kind. One instrument-shaped finding, recorded as the
  correction above: **the dispatch's re-scope figures were five epics stale** — it stated
  bucket M at 63 and a ~1,404-unit non-DONE remainder across nine buckets, and mandated
  bundling to clear the floor. Live at HEAD every one of those buckets is 0. The bundling
  mandate had nothing to bind to; the cycle ran as `PASS_WHOLE_REMAINDER` exactly as
  `workflow-instruction.md §6 step 1` provides.
- **Figures + their re-derive commands** (denominator: **49,438 units** = **49,438 corpus records**, all books, unless stated — `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"`):
  | figure | value | command |
  |---|---|---|
  | population, the denominator of every ratio below | **49,438** units | `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"` |
  | bucket M | **0** of 49,438 | `python3 scripts/completion_atlas.py --check` |
  | every non-DONE bucket (A B C D M V U X Z) | **0** each; `DONE: 49438` | `python3 scripts/completion_atlas.py --check` |
  | scoped population for this cycle | **0**, `verdict=PASS_WHOLE_REMAINDER` | `python3 scripts/cycle_scope_gate.py --min 500 --bucket M` |
  | unmapped token types | **0** of 231 | `python3 scripts/token_coverage.py --check` |
  | converter-refused records | **142**, one shape `no_corpus_record` | `python3 scripts/token_coverage.py --check` |
  | refused records that are non-DONE | **0** of 142 | `python3 -c "import json,sys;sys.path.insert(0,'scripts');import completion_atlas as a;inv={u['id']:u for u in json.load(open('docs/work-inventory.json'))['units']};print(sum(1 for e in json.load(open('data/sheet_rules/_refused.json'))['entries'] if a._bucket_of(inv[e['id']])!='DONE'))"` |
  | degraded records | **603** of 49,438 | `python3 -c "import json;print(json.load(open('data/sheet_rules/_report.json'))['degraded_records'])"` |
  | converter census | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | mapping-table rows | **273** (269 distinct token types) | `python3 -c "import json;r=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json'))['rows'];print(len(r),len({x['token_type'] for x in r}))"` |
  | mapping rows added this cycle, and how many are `Number` | **0** and **0** | `git diff --stat 137658f31a..HEAD -- docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json src/pcgen_import/sheet_rule/table.rs` |
  | `data/sheet_rules/` source-format leaks | **0** files | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | PCGen live-side files | **253** (baseline 260) | `python3 scripts/pcgen_residue_gate.py --check` |
  | magnitude-bearing units not held by the engine | **0** of 26,396 | `python3 scripts/shape_engine_boundary.py --check` |
  | missing engine tables | **0** | `python3 scripts/missing_engine_tables.py --check` |
  | oracle lines compared / agree / disagree | **146 / 145 / 1** | `python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> --exports docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/oracle-parity/exports --output <out.json>` |
  | oracle chassis compared / agree / disagree | **382 / 376 / 6** | same command |
  | cycle-2 parity output vs cycle 1's | **byte-identical**, sha256 `5202e278dd…a082fb` | `sha256sum docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle1_sheet-parity.json` |
  | denominator gate, package + artifacts | `files_checked=77 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | figure-provenance gate | `files_checked=194 figures_examined=353 violations=0` | `python3 scripts/denominator_gate.py --check-provenance` |
  | dashboard feed pin | matches, `5a0a0787312b…e36f` | `./scripts/publish-site-dashboard.sh --check-pin` |

  Wall times paid: release build of `sheet_rule_parity` **1 m 55 s**; `sheet_rule_parity` over the
  29-character roster **3.81 s** (504 lines); `compare` under a second; debug build **1 m 08 s**;
  `sheet_rule_convert --check` **114.2 s**; `cargo test --locked --lib -j 6` **41.15 s**.
- **Build scope verified**, run at `137658f31a` (the tree this cycle commits is docs-only above it):
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`
  - `cargo test --locked --lib -j 6` → `ok. 3261 passed; 0 failed; 15 ignored` (41.15 s)
  - `cargo test --locked --no-fail-fast -j 6` → **not run, and not owed**: `workflow-instruction.md
    §6 step 3` requires it "when `src/` or the classifier changed", and this cycle changes neither
    (`rust_lines_changed=0`). The full workspace ran green at the Epic 4 wrap-up
    (`EPIC-4_wrapup_correction_cycle_receipt.md`, 48 of 48 stages PASS).
  - `cargo clippy --locked --tests -j 6` → **0** lines matching `^(warning|error)`
  - `cargo run --locked --bin sheet_rule_convert -- --check` → `verdict=PASS`, exit 0
  - `cargo run --locked --bin corpus_literal_sweep` → **not run**: no corpus record changed
    (`git diff --stat 137658f31a..HEAD -- data/corpus/` is empty), which is the guard
    `§6 step 3` names.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, 253 files (baseline 260)
  - `python3 scripts/completion_atlas.py --check` → `DONE: 49438`, every other bucket 0,
    `citation_failures=0`
  - `python3 scripts/token_coverage.py --check` → `verdict=PASS`, `unmapped_token_types=0`
  - `python3 scripts/shape_engine_boundary.py --check` → `not_held_by_engine=0`
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 citation_failures=0`
  - `python3 scripts/denominator_gate.py --check …` → `files_checked=77 violations=0`
  - `python3 scripts/denominator_gate.py --check-provenance` → `violations=0` of 353 figures
  - `./scripts/publish-site-dashboard.sh --check-pin` → pin matches
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`
  - the desktop crate and the frontend did **not** run: this cycle touched no path under
    `apps/`. They ran at the Epic 4 wrap-up.
- **Sweep population:** N/A — no corpus record was written this cycle; `data/corpus/**` is
  untouched, so `corpus_literal_sweep` is guarded off. Its last reading stands at 48,706 records
  examined of 51,476 read, 0 findings, `CLEAN` (`AT-35-E4-001_cycle1_receipt.md`).
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`). The parity re-run read the pinned exports only, read-only.
- **Status:** complete
- **Notes:** Blockers `B3`, `B4`, `B6`, `B8` are unchanged from cycle 1's disposition and need no
  re-work: `B6`'s two PI residue paths were closed there; `B8`'s equipment/encumbrance shapes
  degrade because a term is genuinely unreadable, which under the sheet rule prints its words and
  is DONE; `B3`'s `ABILITYCATEGORY` reader and `B4`'s character facts touch no unit that is not
  already DONE and no token type left unmapped. Two files dirty on the shared checkout at cycle
  start were folded rather than filtered away (the atlas `derived_at` stamp, and one
  `sd31-transcribe.jsonl` append from another live session) — precedent `c15e64bc3e`.
- **Next-cycle scope:** criterion at zero. Bucket M 0, every other bucket 0,
  `unmapped_token_types` 0, `refused_non_done` 0. No further cycle is owed.
