# Cycle AT-35-E2-002_cycle2 — Epic 2 — Sheet rule / AT-35-E2-002

**This is a re-dispatch of an already-closed criterion, not new work.** AT-35-E2-002 landed at
`909bb0837c` (receipt `AT-35-E2-002_cycle1_receipt.md`; `kanban.md` row 8 `complete`). This cycle
re-verified every clause of the criterion's `Evidence:` sentence at HEAD `bb785e568d`, changed no
code, no data and no script, and closes zero units — the same shape as the re-dispatch
re-verifications already on this branch (`969d5b9402` for AT-35-E2-001, `986084c5a4`,
`06872eff73`, `3c43cf0531`). It **corrects one stale figure in the cycle-1 receipt**: the fixture
fighter's line count, which AT-35-E2-005 cycle 2 legitimately moved.

- **Commit SHA:** `<pinned by the follow-up commit — see Notes>`. The tree verified is HEAD
  `bb785e568df89d71ce6acff4fd798af1495649cf`, unchanged by this cycle apart from `docs/`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (live-evaluator + sheet-section cycle — closes zero units by design)`
  (`decisions.md §2`; the status `sheet-complete` that moves units is AT-35-E2-003, the pass that
  moves them AT-35-E2-005. The criterion is additionally already at zero, so the re-verification
  moves nothing by construction.) `python3 scripts/pcgen_residue_gate.py --check` at cycle start
  (`bb785e568d`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:** `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle2_receipt.md` (this file, new),
  `progress.md`, `kanban.md`, `docs/retro/events/at-35-e2-002.jsonl` (1 correction appended), plus
  the two live retro-log appends folded from the shared checkout (`docs/retro/events/root.jsonl`,
  `docs/retro/events/sd31-transcribe.jsonl` — other sessions' events, folded per the standing
  clean-tree rule, not this cycle's). **No file outside `docs/` changed** — no `src/`, no
  `scripts/`, no `data/`, no `apps/`. `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  was re-stamped by `completion_atlas.py --check` and **reverted** (`git checkout --`), outside this
  epic's file-touch set — the same disposition cycle 1 recorded.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — `git diff --unified=0 fe5ae6cd4a...bb785e568d --
  <Epic 2 file-touch set> ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  produced no output, at start and unchanged at the end (this cycle's diff is `docs/` only, which
  the scoped path list does not contain).
- **Wired-integration audit result:** **4 hits, every one attributed and none a stub in shipping
  code** — identical to the set AT-35-E2-001 cycle 2 recorded (`969d5b9402`), and the same
  correction the cycle-1 receipt already carries as `1788844812035-at-35-e2-002-7cbeb2`. The
  audit's keyword class greps English prose as well as code, and all four hits are prose:
  | Hits | File | Word | What it actually is |
  |---:|---|---|---|
  | 1 | `data/sheet_rules/bestiary_3/monster_ability/tophet_swallow_whole.json` | `hack` | Pathfinder rules text — "attempt to **hack** or smash its way out" |
  | 1 | `data/sheet_rules/core_rulebook/spell/plant_growth.json` | `hack` | Pathfinder rules text — "must **hack** or force a way through" |
  | 1 | `data/sheet_rules/ultimate_intrigue/class_feature/courtly_hunter_courtly_companion.json` | `not yet implemented` | the **source's own editorial bracket** inside the transcribed `Desc`, not a claim by our code |
  | 6 | `docs/work-inventory.json` | `placeholder` | `reason` fields describing the source's CHOOSE-menu "no selection" rows (pre-existing wording) |
  No `STUB`, `MOCK`, `todo`, `fixme` or `"Would …"` string appears in any code path.
  `workflow-instruction.md §8`'s non-self-healable "a stub, inline mock, or `\"Would …\"` string in
  shipping code" is **not** met.
- **Acceptance criterion:** `src/rules_core/sheet_rule.rs` evaluates `Expr` against the computed
  character — a match over the enum, no parser, no strings (`technical-design.md §2`). One generic
  "Rules and features" section in `apps/desktop/src/characterHub/CharacterSheet.tsx` lists every
  held rule's line, grouped by kind, on the existing sheet IPC. The `Not computed` lane keeps only
  records with no rule. **Evidence:** `cargo test --locked --lib sheet_rule` — one test per value
  form (the racial SLA rule evaluating to `Resolved(15)` at Charisma 14 and spell level 3; the
  weapon to `Dice("1d8+2")`; the choice trait to `Words`). 19 frontend tests, one per kind,
  asserting a held record's label and value are in the DOM for a fixture character.
  `pcgen_residue_gate.py --check` unchanged from baseline (this cycle adds zero live PCGen).
  Desktop crate tested explicitly.
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260`
  — the literal last line of `python3 scripts/cycle_scope_gate.py --receipt --since bb785e568df89d71ce6acff4fd798af1495649cf
  --before /tmp/wi-before-AT-35-E2-002.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-002`
  (full output also: `closed_by_kind=` empty, `relabeled_moves=` empty, `regressed=0 added=0 dropped=0`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  at start and at the end — unchanged, and identical to cycle 1's and to AT-35-E2-001 cycle 2's.
  Zero live PCGen added; this cycle added no code at all.
- **Oracle parity:** N/A — no `Number` mapping row was added (the converter is untouched). The
  criterion's own parity obligation was discharged by AT-35-E2-005 (`oracle-parity/sheet-parity.json`,
  `compared=8 agree=8 disagree=0` at `33deab007b`).
- **Movement, four buckets:** closure 0 / relabel 0 / reachability 0 / instrument-correction 1
  (retro correction `1788883047652-at-35-e2-002-8f4a35` — the cycle-1 receipt's fixture line count).
- **Refused tokens:** none (no converter run changed anything; `data/sheet_rules/_refused.json`
  unchanged at 1,810 records — `cargo run --locked --release --bin sheet_rule_convert -- --check` →
  `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS (19.4s)`,
  and 47,628 + 1,810 = 49,438 exactly).
- **Discoveries:** **one, and it is a correction to this criterion's own cycle-1 receipt.** The
  cycle-1 figure "the fixture Human Fighter 1 … renders **45 lines**" now reads **13 lines**, across
  the *same* five kinds `{class_feature, equipment, feat, race_trait, skill}`. The cause is named
  and benign: AT-35-E2-005 cycle 2 (`33deab007b`) changed `render_sheet` so a `#bonusN` sibling
  prints **only when its own `applies` includes**, which removed the 32 unconditionally-printed
  siblings the cycle-1 figure counted (cycle 1 explicitly listed `#bonus1` (Fly) → `+2` among them,
  and its Discovery (4) flagged the always-printed-sibling shape as an open question). This is a
  downstream improvement to the criterion's own clause, not a regression: "lists every held rule's
  line" still holds, the per-kind evaluation census is unchanged for all 19 kinds, and every
  per-value-form and per-kind proof is green at HEAD. Recorded as correction
  `1788883047652-at-35-e2-002-8f4a35`. No other clause of the criterion drifted.
- **Figures + their re-derive commands:**
  - `sheet_rules package: 47628 rule files, 5081 var files, 66147 rules, loaded in 3.026603511s`
    (debug, parallel parse) — `cargo test --locked --lib -j 6 sheet_rule -- --nocapture`, the
    `package()` `OnceLock` line
  - **the corrected fixture figure: 13 lines** across kinds `{class_feature, equipment, feat,
    race_trait, skill}` for the fixture Human Fighter 1 (+ `feat:acrobatic`, 10 ranks Acrobatics,
    race traits `Human ~ Bonus Feat` / `Human ~ Skilled`) — same command, the `fixture fighter
    lines:` line. Cycle 1 read 45; see Discoveries.
  - per-kind evaluation of every rule against the deterministic Human Fighter 1 — `cargo test --locked --lib -j 6 sheet_rule -- --nocapture`, the `kind=<k> number=<n> dice=<d> words=<w>` lines (denominator: 66,147 rules in the package, a rule's `#suffix` siblings counting as rules) — **all 19 identical to cycle 1's**:
    `ability 2036/3/3138`, `class 12/0/0`, `class_feature 5205/134/14953`, `companion 3687/104/917`,
    `deity 0/0/458`, `domain 5/0/180`, `equipment 888/552/5143`, `equipment_modifier 7/0/1498`,
    `feat 422/3/1734`, `language 0/0/136`, `monster 8483/1010/625`, `monster_ability 958/16/3612`,
    `power 17/0/426`, `race 46/6/55`, `race_trait 701/28/1945`, `skill 325/0/38`, `spell 268/0/2805`,
    `template 1093/847/1085`, `trait 230/0/313` — 19 kinds
  - value-form proofs, all green — `cargo test --locked --lib -j 6 sheet_rule` → **31 passed; 0
    failed; 0 ignored; 3200 filtered out** (17.36 s), including
    `choice_trait_evaluates_to_words`, the racial-SLA `Resolved(15)` case and the
    `Dice("1d8+2")` weapon case named in the criterion's `Evidence:` sentence
  - live-side purity: `grep -cE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|raw_tokens|PcgenFormulaEvaluator|render_pcgen_desc' src/rules_core/sheet_rule.rs` → **0**, over its 2,359 lines (`wc -l src/rules_core/sheet_rule.rs`). The section, the loader and the DTO read `SheetRule` / `VarTable` JSON and `PilotBaseChassisComputation` only.
  - the section and its grouping helpers exist at HEAD — `grep -n 'RulesAndFeaturesSection\|groupSheetLinesByKind\|sheetLineKindLabel' apps/desktop/src/characterHub/CharacterSheet.tsx` → `:2041 :2051 :2064 :2084 :2087` plus the mount at `:2324`
  - the `Not computed` lane keeps only records with no rule — `grep -n sheetLines apps/desktop/src/characterHub/classFeaturesModel.ts` → `noticeHasSheetRule` at `:321` and `:353`
  - the loader — `grep -n 'fn load_sheet_rules' src/rules_core/corpus_loader.rs` → `:246` (`load_sheet_rules`) and `:252` (`load_sheet_rules_filtered`)
  - the `SheetLine` on the computation result — `grep -n 'sheet_lines\|with_sheet_rules' src/rules_core/pilot_compute/mod.rs` → `:261 :263 :270 :280`
  - the IPC reach test — `grep -n sheet_rule_lines_cross_the_ipc apps/desktop/src-tauri/src/reach_gate.rs` → `:8195` (`sheet_rule_lines_cross_the_ipc_carrying_label_and_value`)
  - **frontend, the criterion's "19 frontend tests, one per kind" clause:** `node scripts/run-tests.mjs`
    in `apps/desktop` → `101/101 test files passed`, exit 0, with the line
    `rulesAndFeaturesSection: 19 per-kind tests + 5 section tests passed` and
    `PASS src/characterHub/classFeaturesModel.test.ts`; `npx tsc --noEmit` exit 0
  - **desktop crate, tested explicitly** (this cycle re-verifies `apps/`): `cargo test --locked -j 6`
    in `apps/desktop/src-tauri` (`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E2-002-desktop`) →
    `574 passed; 0 failed; 0 ignored` (99.60 s); `cargo clippy --locked --tests -j 6` there → **0
    warnings** (`grep -cE '^warning'` = 0), exit 0
  - literal scan **0** files — `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
  - gates: `completion_atlas.py --check` → `done_evidence_violations=0 missing_clearing_mechanisms=0
    stale_derived_at=False citation_failures=0`, exit 0; `token_coverage.py --check` →
    `non_done=1404 tokened=1399 token_less=5 refused=1810 refused_non_done=659 token_types=231
    shapes=81 verdict=PASS`, exit 0; `shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=363 citation_ok=True`, exit 0; `missing_engine_tables.py --check` →
    `population=1 kinds=1 (power 1) citation_failures=0`, exit 0; `denominator_gate.py --check`
    over the two package globs → `files_checked=41 violations=0`, exit 0; `scripts/verify.sh --only
    pi-sweep` → `RESULT: PASS`
- **Build scope verified:** `cargo test --locked --no-run -j 6` exit 0
  (`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E2-002`, `CARGO_INCREMENTAL=0`, warm target dir carried
  from cycle 1 — 22 crates recompiled, **2 min 47 s** wall, max RSS 2,383,532 kB, `/usr/bin/time -v`);
  `cargo test --locked --lib -j 6` → **3217 passed; 0 failed; 14 ignored** (63.78 s; cycle 1 read
  3214 — the +3 are AT-35-E2-005 cycle 2's three RED→GREEN tests in `33deab007b`);
  `cargo test --locked --no-fail-fast -j 6` → **412 test binaries, 412 `test result: ok`, 0 FAILED;
  8,721 passed, 0 failed, 67 ignored**, `FULL_EXIT=0` (cycle 1 read 411 / 8,710;
  `grep -c '^test result'` and an `awk` sum over the result lines, two independent derivations
  agreeing); `cargo clippy --locked --lib -j 6` → **0 warnings**. Desktop crate and frontend run in
  THIS cycle (`apps/` is in the criterion's scope) — figures above. All run at SHA `bb785e568d`,
  the tree this cycle verified and did not change.
- **Sweep population:** N/A — no corpus record changed and `data/sheet_rules/` is byte-unchanged
  (`sheet_rule_convert -- --check` `verdict=PASS`), so `corpus_literal_sweep` is not triggered by
  `§6` step 3. `docs/work-inventory.json` was not regenerated; it is byte-identical to
  `/tmp/wi-before-AT-35-E2-002.json` (`closed=0 relabeled=0 regressed=0 added=0 dropped=0`).
- **Oracle pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  unchanged; no figure in this receipt came from the pinned corpus).
- **Status:** complete
- **Notes:** This line's own commit SHA is pinned by a follow-up commit, the pattern `942c8d3ae5`
  and `bb785e568d` set for the two earlier cycle-2 receipts. Judgment calls: (a) the criterion was
  already `complete` on arrival and the dispatch named "cycle 1", so this receipt is written as
  **cycle 2**, following the AT-35-E2-001 precedent on this branch — a second cycle-1 receipt would
  overwrite evidence. (b) `completion_atlas.py --check` re-stamps SD-34's atlas `derived_at`; the
  stamp is **reverted**, as in cycle 1, because that file is outside Epic 2's file-touch set. (c)
  Two live retro-log appends from other sessions (`root.jsonl`, `sd31-transcribe.jsonl`) were on the
  shared checkout and are folded into this cycle's docs commit per the standing clean-tree rule.
  (d) `CARGO_TARGET_DIR` is under `/tmp` as the dispatch mandates, against `AGENTS.md`'s
  "never place one under `/tmp`" — 604 G free at start, no bus errors; flagged, not silently
  followed.
- **Next-cycle scope:** criterion at zero — the evaluator, the loader, the IPC field, the section,
  the `Not computed` lane and every per-kind and per-value-form proof exist and are green at HEAD;
  residue unchanged at baseline. Epic 2 is closed on the board (rows 7–11 and 30 all `complete`).
  The open work is Epic 3: row 12 AT-35-E3-001 is `blocked-escalated` (`scoped=214 …
  verdict=FAIL_UNDER_FLOOR`, 24 refused token types — awaiting the orchestrator's re-scope into
  AT-35-E4-001's 623-unit refused bundle), rows 13–15 `not-started`.
