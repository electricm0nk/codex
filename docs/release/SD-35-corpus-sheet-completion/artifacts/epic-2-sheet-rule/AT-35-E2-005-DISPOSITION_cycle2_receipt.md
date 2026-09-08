# Cycle AT-35-E2-005-DISPOSITION_cycle2 — Epic 2 — Sheet rule / AT-35-E2-005-DISPOSITION

**Re-dispatch.** The criterion was already closed by cycle 1 at `8cc4ea1516` (the disposition commit
carrying the amendment, `decisions.md §16`, the hand-off script and table, `kanban.md` rows 11 and
30, and `progress.md`'s entry). This cycle re-derives the hand-off and every gate at HEAD and
confirms the amended bar still holds. **No unit moved and no file outside `docs/` changed.**

- **Commit SHA:** `5b97482b779912c9a5331de788cd33b89f9ab911` — the docs commit carrying this receipt,
  its `progress.md` entry, the `kanban.md` row-30 pointer and the retro deferral, on `tranche/15`
  (this line's own SHA pinned by the immediately following commit; cycle start
  `ca976bf31da9e34376f5beb0b6e684de46df4570`; the cycle moved no file under `src/**`, `scripts/**`,
  `tests/**`, `data/**` or `apps/**`, and `docs/work-inventory.json` is untouched —
  `cmp docs/work-inventory.json /tmp/wi-before-AT-35-E2-005-DISPOSITION.json` → identical).
- **Scope gate:** `SCOPE_GATE: EXEMPT (disposition cycle — it moves no unit; it records where every
  remaining unit is owned)` — `decisions.md §2`'s zero-units-by-design exemption. The gate was run
  anyway and is quoted for the record: `python3 scripts/cycle_scope_gate.py --min 500` →
  `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS`
  (`scoped_by_bucket=A:1 B:437 C:79 D:43 M:63 U:202 V:392 X:168 Z:19`;
  `scoped_by_kind=ability:91 class:144 class_feature:641 companion:13 equipment:188
  equipment_modifier:40 feat:93 monster:2 power:1 race_trait:168 skill:7 spell:6 template:9
  trait:1`). `python3 scripts/pcgen_residue_gate.py --check` at start:
  `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Files touched:** this receipt (new),
  `docs/release/SD-35-corpus-sheet-completion/progress.md` (prepended entry),
  `docs/release/SD-35-corpus-sheet-completion/kanban.md` (row 30 Notes — cycle-2 pointer appended;
  no status change, the row was already `complete`),
  `docs/retro/events/at-35-e2-005-disposition.jsonl` (1 deferral appended). No change to
  `epic-breakdown.md`, `decisions.md`, `AT-35-E2-005-DISPOSITION_handoff.py` or `_handoff.json` —
  cycle 1's text and table are correct at HEAD and re-derivation reproduces them exactly, so
  rewriting them would be churn. The atlas `--check` re-stamped
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` `derived_at`;
  reverted (`git checkout --`), outside this cycle's set — as every Epic 2 cycle did.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own diff (no line added outside
  this receipt, `progress.md`, `kanban.md` and the retro event). Over the whole Epic 2 docs
  file-touch set since `BASE_BRANCH=fe5ae6cd4a5f3c65d5d10f4d523f00e33b04ac47`
  (`git diff --unified=0 fe5ae6cd4a...HEAD -- docs/release/SD-35-corpus-sheet-completion/{epic-breakdown,decisions,kanban,progress}.md docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`)
  the grep finds 3 pre-existing hits, none a bundle tag: AT-35-E1-003's `progress.md` /
  receipt lines naming the test directories `tests/sd18_widening/` and `tests/sd13_progression/`
  — file paths that pre-date this bundle.
- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own diff. Over the whole set
  since `fe5ae6cd4a` (same command, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`)
  the grep finds only pre-existing hits, **none in code**: prior receipts and `progress.md` entries
  quoting the audit pattern itself, the 3 rulebook-prose hits AT-35-E2-002 recorded
  (`bestiary_3:monster_ability:tophet_swallow_whole` "hack",
  `core_rulebook:spell:plant_growth` "hack",
  `ultimate_intrigue:class_feature:courtly_hunter_courtly_companion` "[… not yet implemented]" —
  the rulebook's own bracketed editorial note; correction `1788844812035-at-35-e2-002-7cbeb2`),
  and `token-mapping/*.json` rule texts describing the archetype "no spellcasting" **hack** row.
- **Acceptance criterion:** `epic-breakdown.md` `### AT-35-E2-005-DISPOSITION`, verbatim. Its five
  obligations, each re-verified at HEAD:
  1. **The amendment exists**, dated, reasoned, with the original text kept —
     `### AT-35-E2-005`'s "**Amendment, 2026-09-08 — orchestrator re-scope**" paragraph follows the
     original bar unaltered. ✅
  2. **The hand-off is re-derived at HEAD, not copied** —
     `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py`
     exits **0** with
     `non_done=1404 atlas_non_done=1404 refused_non_done=659 not_refused_non_done=745
     owned_sum=1404 unowned=0 duplicate_ids=0 verdict=PASS`. ✅
  3. **`decisions.md §16` exists and cites the four receipts** — `decisions.md:439`,
     "§16 — Orchestrator re-scope, 2026-09-08: AT-35-E2-005's bar is amended and its remainder
     handed on unit for unit". ✅
  4. **`kanban.md` row 11 reads `complete`** with a pointer to the amendment and the hand-off table
     (`kanban.md:41`); row 30 carries this disposition cycle (`kanban.md:42`). ✅
  5. **Each successor criterion states what it inherited** — an "Inherited from AT-35-E2-005
     (2026-09-08, `decisions.md §16`, `### AT-35-E2-005-DISPOSITION`)" line on `### AT-35-E3-001`
     (the 659, carried under AT-35-E4-001's ownership rule), `### AT-35-E4-002` (391),
     `### AT-35-E5-003` (217) and `### AT-35-E5-004` (137), each with the re-derive command. ✅
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a
  builds_recorded=0 pcgen_live_files=260` — the literal last line of
  `python3 scripts/cycle_scope_gate.py --receipt --since ca976bf31da9e34376f5beb0b6e684de46df4570 --before /tmp/wi-before-AT-35-E2-005-DISPOSITION.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-005-DISPOSITION`
  (`since=ca976bf31d… residue_gate=present`, `closed_by_kind=` empty, `relabeled_moves=` empty,
  `regressed=0 added=0 dropped=0`). `builds_recorded=0` reported honestly — no build was paid,
  nothing outside `docs/` changed.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736
  verdict=PASS` at start and after — unchanged (no live-side file touched). Nothing new on the live
  side reads a PCGen token; the converter, parser, generators and oracle harness are untouched
  (`decisions.md §11`).
- **Oracle parity:** N/A for this cycle (no `Number` mapping added, no live path touched, no
  converter run). The standing figure the amended bar cites is unchanged at
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`:
  `compared=42 agree=41 disagree=1 unverifiable=5`
  (`artifacts/epic-2-sheet-rule/oracle-parity/sheet-parity.json`), the one disagreement Weapon Focus
  on `deterministic_human_fighter_l1` (ours 0 vs PCGen 1, `Var vb1e14268d73c2def`) — a holdings gap
  owned by AT-35-E3-001.
- **Movement, four buckets:** closure **0** / relabel **0** / reachability **0** /
  instrument-correction **0**. Buckets at HEAD `A 1 B 437 C 79 D 43 M 63 V 392 U 202 X 168 Z 19`
  (non-DONE **1,404** of **49,438**, DONE 48,034) — byte-identical to cycle 1's row.
- **Refused tokens:** none refused *by this cycle* (no converter run). **The hand-off, re-derived at
  HEAD:** `by_owner AT-35-E4-001=659 AT-35-E4-002=391 AT-35-E5-003=217 AT-35-E5-004=137`
  (sum **1,404** = the live non-DONE total). Cells — E4-001: A 1, B 437, C 79, D 43,
  M 63 (`engine-does-not-hold` / `ingested-magnitude`), U 4, V 1, X 31, **all converter-refused**;
  E4-002: V `literal-verified` 388 + `fixture-verified` 3, not refused; E5-003: U `unmeasurable` 198
  + Z `not-started` 19, not refused; E5-004: X `deferred-with-reason` 137, not refused.
  `refused_class_records all=182 non_done=144` — the 144 go first. The 659 by refusal string
  (69 strings with a non-DONE count, multiplicity 851 over 659 units):
  `FORMULA:var(COUNT)=210, unmapped:STARTSKILLPTS=119, SPELLS (PI-redacted token)=66,
  BONUS:[redacted PI]=62, FORMULA:malformed (parser refusals)=62, DEFINE (PI-redacted token)=40,
  unmapped:MODTOSKILLS=37, unmapped:SPELLSTAT=23, unmapped:MEMORIZE=19,
  FORMULA:var(<export token>) (ENCUMBERANCE)=17, FORMULA:identifier DEFINEd nowhere
  (Bloodrager_CF_BloodlinePowers …)=12, unmapped:SPELLLIST=12, BONUS:EQM=11,
  FORMULA:CL-no-owner=11, FORMULA:identifier DEFINEd nowhere (Bloodrager_CF_BloodlineSpells …)=11,
  FORMULA:var(SKILL.<name>.MISC)=11, BONUS:ITEMCOST=10,
  BONUS:STAT (target BASESPELLKNOWNSTAT;Class)=7, BONUS:STAT (target BASESPELLSTAT;Class)=7,
  FORMULA:var(STAT)=7, BONUS:SITUATION (target shape)=6, FORMULA:var(SPELLFAILURE)=5,
  unmapped:KNOWNSPELLS=5, and 46 strings at 1–4 each` — full list in
  `AT-35-E2-005-DISPOSITION_handoff.json` `refused_non_done_by_shape`. Deferral
  `1788895582394-at-35-e2-005-disposition-dde6f4` names all of it.
- **Discoveries:** none. Every figure cycle 1 wrote re-derives identically at HEAD — the amendment's
  counts, the hand-off table's four owner rows and thirteen cells, the 69 refusal strings, the
  `class` 182/144 split, and the bucket row. Cycle 1's own correction
  (`1788878644075-at-35-e2-005-disposition-6224d1`: the four AT-35-E2-005 receipts wrote the
  non-refused split as "V 389 + 3, U 202, X 137, Z 19" summing to **750**; the true split is
  **V 391 + U 198 + X 137 + Z 19 = 745**, because 1 V and 4 U units are converter-refused and belong
  to the 659) stands and needs no re-issue. No new token type, kind, or remaining-step category
  outside `token-coverage.json` and the atlas.
- **Figures + their re-derive commands:**
  - non-DONE **1,404 of 49,438**, DONE **48,034**, bucket row `A 1 B 437 C 79 D 43 M 63 V 392
    U 202 X 168 Z 19` — `python3 scripts/completion_atlas.py --check` →
    `population=49438 buckets=10 unclassified=0 overlap=0 done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`
  - refused non-DONE **659 of 1,404**, shapes **81**, token types **231** —
    `python3 scripts/token_coverage.py --check` → `non_done=1404 tokened=1399 token_less=5
    refused=1810 refused_non_done=659 token_types=231 shapes=81 verdict=PASS`
    (all six sub-checks `ok=True`: population, double_count, coverage, refused_set, shape_totals,
    partition)
  - the hand-off partition (owners 659 / 391 / 217 / 137, sum 1,404; 13 cells; by kind; the 69
    refusal strings; `class` 182 refused / 144 non-DONE) —
    `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py`
    → exit 0, `owned_sum=1404 unowned=0 duplicate_ids=0 verdict=PASS` (denominators: 49,438 units,
    1,404 non-DONE, 1,810 refused records; it imports `scripts/completion_atlas._bucket_of`, so DONE
    is the atlas partition and not a second list)
  - receipt rows — the `cycle_scope_gate.py --receipt` invocation above
  - scope-gate line — `python3 scripts/cycle_scope_gate.py --min 500` →
    `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS`
  - PCGen residue — `python3 scripts/pcgen_residue_gate.py --check` →
    `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  - denominator gate — `python3 scripts/denominator_gate.py --check
    'docs/release/SD-35-corpus-sheet-completion/*.md'
    'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` → `files_checked=46 violations=0`
    (45 before this receipt existed)
  - shape/engine boundary — `python3 scripts/shape_engine_boundary.py --check` →
    `magnitude_bearing=26396 not_held_by_engine=363 citation_ok=True`
  - missing engine tables — `python3 scripts/missing_engine_tables.py --check` →
    `population=1 kinds=1  power: count=1 books=1 zero_bucket_a_books=1  citation_failures=0`
- **Build scope verified:** no build — nothing outside `docs/` changed
  (`git diff --stat ca976bf31d..HEAD -- src scripts tests data apps` empty), so
  `cargo test --locked --no-run` / `--lib` / `--no-fail-fast`, `cargo clippy`,
  `sheet_rule_convert -- --check` and `corpus_literal_sweep` are **not run** (`§6`: the build runs
  after a figure-moving change; the desktop crate and frontend at epic cadence). Docs gates run and
  green, all at SHA `ca976bf31da9e34376f5beb0b6e684de46df4570`: `completion_atlas.py --check`,
  `token_coverage.py --check` (`PASS`), `pcgen_residue_gate.py --check` (`PASS`),
  `denominator_gate.py --check` (`violations=0`), `shape_engine_boundary.py --check`,
  `missing_engine_tables.py --check`, and the hand-off script (`PASS`).
- **Sweep population:** N/A (no corpus record changed; no regen).
- **Oracle pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (quoted only for the standing parity
  figure; no oracle run in this cycle).
- **Status:** `complete` — the amended bar (`epic-breakdown.md` `### AT-35-E2-005`, 2026-09-08) is
  met at HEAD by cycles 1–4's committed evidence, and every one of the **1,404** non-DONE units of
  **49,438** is owned by a named later criterion (`owned_sum=1404 unowned=0 duplicate_ids=0`). This
  criterion's own population is zero by construction — it moves no unit. **No carve-out exists:**
  the 1,404 remain inside the bundle's Definition of Done (AT-35-E5-005 — `DONE=49438 of 49438`),
  owned by AT-35-E4-001 (659), AT-35-E4-002 (391), AT-35-E5-003 (217) and AT-35-E5-004 (137).
- **Notes:** (a) This is a re-dispatch of an already-closed criterion; the correct outcome is
  re-verification, not a rewrite. `epic-breakdown.md`, `decisions.md §16`, the hand-off script and
  its JSON are unchanged because re-derivation reproduces them exactly. (b) The 5 refused V/U units
  are E4-001's by the owner rule ("a non-DONE unit the converter refused needs a mapping row →
  AT-35-E4-001, whatever its bucket"), which is why E4-002 inherits **391**, not 392, and E5-003
  inherits **217**, not 221. (c) `builds_recorded=0` is honest, not a skipped obligation: the
  dispatch states no build is needed because nothing outside `docs/` changes.
- **Next-cycle scope:** Epic 2 wrap-up (`workflow-instruction.md §10`) if not already run, then
  Epic 3 opens. `python3 scripts/cycle_scope_gate.py --min 500` over the whole remainder →
  `scoped=1404 remaining_non_done=1404 floor=500 verdict=PASS`. Every B/C/D unit at HEAD is
  converter-refused (E4-001's by the owner rule), so Epic 3's first mechanism is the holdings gap
  the parity names (Weapon Focus's `class_feature:default` declarer, the 19 unprinted pools,
  Halfling Luck / Divine Grace save consumers), and AT-35-E4-001's first cycle takes the 659 by
  refusal string, the **144 non-DONE `class` records** first.
