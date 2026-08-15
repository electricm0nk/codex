# SD-31 Progress Log

**Note (2026-08-15, `decisions.md §6`): every `tranche/10` / `origin/tranche/10` reference below this
point is a historical receipt, correct at the time it was written — this package's cycles committed to
`tranche/10` before that date.** From SD31-S7-VERSION-001 forward, this package operates on
`tranche/11` (cut from `tranche/10`'s tip `1980d6b95`, operator ruling 2026-08-15) at release
`0.11.<build>`. Do not edit the historical entries below to say `tranche/11` — they describe what was
true when recorded.

## Split from SD-30 (2026-08-14)

Package created by operator ruling 2026-08-14 ("split phase 3 and phase 4 into their own SD's; SD-31
and SD-32"). Epics 4, 5, 6, 10, 11, and the grind-lane half of 14 moved from
`SD-30-class-feature-archetype-bundle/` (renumbered 1-6 here — see `epic-breakdown.md`'s map). No card
was `IN-FLIGHT` at split time (verified against SD-30's `kanban.md` immediately before the move); every
moved card carries forward as `READY`/`READY (gated on ...)`, state unchanged.

SD-30's own per-cycle receipts for this scope, recorded before the split, remain in
`SD-30-class-feature-archetype-bundle/progress.md` as history — not copied here. Cycles claiming a card
in this package's `kanban.md` from this point forward append their receipts below.

No cycles claimed yet.

## 2026-08-15 — SD-32 absorbed; epics re-sequenced (`decisions.md §2`/§3/§4)

**Added retroactively 2026-08-15 (launch-readiness remediation Step 5, drift D14): this package had no
merge receipt for its own SD-32 absorption before this correction, even though the absorption is the
single largest structural change this package underwent after its creation.** Commit `fae30fc12`
(2026-08-15 09:00:41 -0400), pushed to `origin/tranche/10` before the remediation pass began. Summary
of `decisions.md §2`-§4, the decisions that commit landed:

- **§2 — SD-32 absorbed, epics re-sequenced.** Operator ruling 2026-08-15 (verbatim in `decisions.md`):
  *"SD-31 will be next, if there are prereqs in SD-32, then they need to be moved into SD-31... merge
  SD-31 and SD-32 and... reshuffle the epic order."* `SD-30 decisions.md §51` had split SD-30 into a
  grind package (SD-31) and a capability package (SD-32), scheduled to run in that order — but the
  dependency ran the other way: 22.1 % of the board (8,524 units — `ambiguous` 2,109, `unmeasurable`
  3,989, `race`+`race_trait` not-done 3,284, minus 119 overlap) could not reach `done` without the
  capability builds SD-32 was going to deliver *after* SD-31 would have already tried to close.
  `docs/release/SD-32-engine-capability-builds/` was deleted; its Epics 1-2 became this package's own
  Epics 1-2; its Epic 3 (cloud fan-out) merged into this package's Epic 8. Directory name kept
  unchanged (cited from a shipped Rust test and a production Python script). Full renumber map:
  `README.md`.
- **§3 — Structural Exclusion Register.** Replaces the struck "or named a successor for the remainder"
  deferral hatch. A unit leaves the 100 % denominator only via an entry carrying the proving command,
  the named missing capability (cost is never a valid reason), an Epic 0 audit run, and **operator
  sign-off**. A cycle may propose; only the operator grants.
- **§4 — Epic 0, the reachability audit, becomes a standing gate.** `scripts/reachability_audit.py`
  answers, mechanically, whether a path to `done` exists for every unit on the board; runs before the
  first cycle, at every epic closure, and before any closure receipt. Not yet built at this writing —
  it is Epic 0's own first-cycle deliverable (see `technical-requirements.md`'s pre-loop-prerequisites
  clarification, drift D3).

No `verify.sh` run required for this receipt — commit `fae30fc12` was doc-only (package restructuring:
`decisions.md`, `epic-breakdown.md`, `kanban.md`, `README.md`, `scope-draft.md`,
`state-goals-and-lessons.md`, `progress.md`, `forward-scope-register.md`,
`risks-and-open-questions.md`, deletion of `SD-32-engine-capability-builds/`), confirmed by
`git show --stat fae30fc12`.

## 2026-08-15 — Launch-readiness remediation Step 1: S1-doneness-bar (RETRO_ACTOR sd31-ready-s1)

**Scope.** Plan Step 1 of
`~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md` — blockers B1/B3. Landed Decision 5 (the
mandate denominator, operator ruling 2026-08-15) in `decisions.md`; the doneness bar in
`epic-breakdown.md` Epic 9-F1 and the Completion gate; `AT-31-103` in
`acceptance-and-verification.md`; relabelled `AT-31-005`'s `done+held` floors as progress floors, not
closure criteria; updated the exit gate checklist; updated `README.md` Purpose/Exit statement to name
the doneness bar. Doc-only step — no Rust/Python/shell production code touched.

**Started from HEAD** `56512485cbd89594c832e976c18e47adf1820987` on `tranche/10`. Tree was clean at
start (`git status --porcelain` empty); confirmed before any write.

**Figures re-derived this cycle (every one reproduced, not transcribed):**

1. Strict mandate denominator/doneness (command in `AT-31-103`, `decisions.md §5`):
   ```
   python3 -c "import json,sys,collections; sys.path.insert(0,'scripts/observer'); import
   pf1e_dashboard_producer as P; U=[u for u in json.load(open('docs/work-inventory.json'))['units'] if
   u.get('book') not in P.EXCLUDED_BOOKS]; c=collections.Counter(P.doneness_verdict(u.get('wiring_class'),
   u.get('status'),u.get('kind')) for u in U); print(c, len(U))"
   ```
   → `Counter({'not-started': 20895, 'held': 6916, 'done': 5837, 'unmeasurable': 3989,
   'in-progress': 848, 'deferred': 36}) 38521`. **done=5,837, denominator=38,521, 15.15 %** — matches
   the plan's expected figure exactly.
2. Old/secondary headline (in-scope books only, minus `unmeasurable`/`deferred`, replaying the live
   dashboard's `inScopeUnits()`/`usableDenom()` logic from `~/swarm-observer/PF1e-dashboard.html` over
   `docs/work-inventory.json`'s `books[].scope == "in_scope"`):
   ```
   python3 -c "
   import json, sys, collections
   sys.path.insert(0,'scripts/observer')
   import pf1e_dashboard_producer as P
   d = json.load(open('docs/work-inventory.json'))
   in_scope_ids = {b['id'] for b in d['books'] if b.get('scope') == 'in_scope'}
   U = [u for u in d['units'] if u.get('book') in in_scope_ids]
   c = collections.Counter(P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind')) for u in U)
   denom = len(U) - c['unmeasurable'] - c['deferred']
   print(c, 'denom', denom, 'pct', round(c['done']/denom*100,2))
   "
   ```
   → `done=5,837, denom=30,402, pct=19.20` — matches the plan's cited B3 figure exactly, and confirms
   the source of the 30,402/19.20% figure is the book-level `scope` field (30 `in_scope` of 38 books;
   7 `future_state`, 1 `out_of_scope` = `beginner_box`).
3. Book-scope roster: `Counter({'in_scope': 30, 'future_state': 7, 'out_of_scope': 1})` — 38 total
   books, 37 non-`beginner_box`, matching the operator ruling's "37 non-`beginner_box` books" text
   exactly.
4. Invariance check: the 7 `future_state` books already carry **4,094 units** inside
   `docs/work-inventory.json` today (`adventurers_guide`, `inner_sea_faiths`, `inner_sea_magic`,
   `inner_sea_taverns`, `inner_sea_temples`, `mythic_adventures`, `occult_adventures`) — confirms
   Epic 7 onboarding those books moves units toward `done` inside the existing 38,521 denominator,
   never widens it.

**Not independently re-derived this cycle:** the reachable-ceiling figure (36,412/38,521 = 94.53 %)
cited in `decisions.md §5` from the readiness plan's blocker B1 — `scripts/reachability_audit.py` is
Epic 0's own not-yet-built deliverable (explicitly out of scope for this pass, per the plan's
"Explicitly NOT in this pass"), so that figure is carried by source citation, not reproduced by me this
cycle. Flagged, not silently presented as re-derived.

**Retro.** One correction emitted for the denominator ambiguity
(`docs/retro/events/sd31-ready-s1.jsonl`, event id `1786802682843-sd31-ready-s1-51d39b`):
subject "SD-31 exit gate / AT-31-102 (pre-remediation)", claimed 30,402 vs actual 38,521 as the binding
mandate denominator, `--verified-by` the two commands above. No standalone `retro.py decision` event —
`decisions.md §5` itself is that record, dated and attributed.

**Verification.** Doc-only step; ran `./scripts/verify.sh --only preflight-disk` only, per the step
brief (no Rust/Python/shell production code changed):
```
==> preflight-disk — disk budget check before any build starts
    repo filesystem (/home/ubuntu/workspace/repos/codex, mounted at /): 40% used, 580G available
    scratch-log filesystem (/tmp/codex-verify-DsNt8Z, mounted at /): 40% used, 580G available
    PASS  preflight-disk  (disk budget OK)
SUMMARY
  passed:  1  preflight-disk
RESULT: PASS
```
`VERIFY_EXIT=0`.

**Files changed:** `decisions.md` (+Decision 5), `epic-breakdown.md` (Epic 9-F1 + Completion gate gain
the doneness bar), `acceptance-and-verification.md` (+AT-31-103, AT-31-005 relabelled, exit checklist
updated), `README.md` (Purpose/Exit statement name the doneness bar), `docs/retro/events/sd31-ready-s1.jsonl`
(+1 correction event), this file.

**Status:** complete for this step's stated scope. Steps 2-6 of the plan (cards for unowned kinds,
oracle pin, dashboard import, drift sweep, pre-launch checklist) are separate steps, not owned by this
receipt.

## 2026-08-15 — Launch-readiness remediation Step 2: S2-cards (RETRO_ACTOR sd31-ready-s2)

**Scope.** Plan Step 2 of `~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md` — blocker B2
(six not-done kinds, 9,894 units, no card) and the coupled parts of B4 (the 5,273-unit held
static/derived residual, the 1,243-unit `display|grounded` widening of AT-31-010) and the 36
`deferred-with-reason` units carried with zero sign-off. Doc-only step — no Rust/Python/shell
production code touched.

**Started from HEAD** `85aaa0c77ad91e027105000b295634e18df1885f` on `tranche/10`. Tree was clean at
start (`git status --porcelain` empty); confirmed before any write.

**Figures re-derived this cycle (every one reproduced, not transcribed — commands and outputs also
recorded in the package files themselves, repeated here for the receipt):**

1. **The six-kind ladder** (`epic-breakdown.md` Epic 6 F5-F10 preamble):
   ```
   python3 -c "
   import json, sys, collections
   sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
   d = json.load(open('docs/work-inventory.json'))
   U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
   for k in ('equipment','monster_ability','feat','companion','equipment_modifier','class'):
       units = [u for u in U if u.get('kind')==k]
       verdict = lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),k)
       c = collections.Counter(verdict(u) for u in units)
       print(k, 'total', len(units), 'done', c.get('done',0), 'not_done', len(units)-c.get('done',0))
   "
   ```
   → `equipment 6,208/2,626 done/3,582 not-done` · `monster_ability 3,107/334/2,773` ·
   `feat 2,610/1,178/1,432` · `companion 1,696/416/1,280` · `equipment_modifier 1,580/911/669` ·
   `class 185/27/158`. **Sum 9,894 — matches the plan's expected figure exactly**, per-kind and in
   total. Per-kind `(wiring_class, status)` cell breakdowns for the not-done population also
   re-derived and recorded in `epic-breakdown.md` F5-F10 (per-cell command in `s2_ladder.py`,
   scratchpad, not committed — the per-kind commands embedded in the package files are the
   reproducible record).
2. **Epic 6-F1 `monster` rewrite** (fixture-coverage lane, not ingest): `monster` total 1,270,
   `{'held': 1235, 'done': 7, 'not-started': 28}`, of which **1,229 is the single cell
   `derived|grounded`**. Command recorded in `epic-breakdown.md` F1.
3. **Epic 6-F11 held static/derived residual**: **5,273** held `static`/`derived` units corpus-wide
   (`equipment 2,284`, `monster 1,232`, `spell 1,061`, `companion 322`, `monster_ability 304`,
   `class_feature 33`, `equipment_modifier 19`, `feat 17`, `race_trait 1`) — matches the plan's B4
   figure exactly. Fixture coverage: **94 entries** (`tests/fixtures/rules_core/derived-evaluator-
   fixtures.json`, `entries` array length). Held-derived eligible population re-derived at **2,792**,
   not the **2,879** the code comments at `src/bin/v06_work_inventory.rs:4585` and
   `derived_evaluator_fixture_check.rs:14` state (stale, corpus drift since authored) — correction
   emitted (below).
4. **AT-31-010 widening**: `display|grounded` corpus-wide = **1,243** units, all `held`
   (`monster_ability 981`, `companion 182`, `class_feature 54`, `race_trait 23`, `feat 3`) — matches
   the plan figure exactly. Command in `acceptance-and-verification.md` AT-31-010.
5. **Epic 5-F4 the 36 deferred units**: re-derived at **36** (34 `class_feature`, 2 `feat`), full
   per-book breakdown and per-unit `id` list recorded in `epic-breakdown.md` Epic 5-F4 — matches the
   plan's "34 class_feature, 2 feat" exactly.

**Package changes:**

- `epic-breakdown.md` Epic 6: F1 (`monster`) rewritten as a fixture-coverage lane with the
  re-derivation and a correction note; F5 `equipment`, F6 `equipment_modifier`, F7 `companion`, F8
  `feat` (routes the SD-30 E0-F3 217-unit probe-fixture residue — `docs/release/SD-30-class-feature-
  archetype-bundle/artifacts/sd30-e0-f3-unknown-residue/`, cited by its own bucket breakdown: 194
  `PREABILITY` chooser-prereq + 23 `PRESTAT`/`PRESKILL` = 217, remedy = widen `PROBE_CLASSES`/
  `PROBE_SELECTIONS` and `feat_probe_input`'s stripped-fixture shape in `src/bin/v06_work_inventory.
  rs:128/138/1560`), F9 `monster_ability`, F10 `class` added, each with its ladder, shape, moving
  instrument/lane, and acceptance; F11 held static/derived residual added, owner named (extends
  `corpus_literal_sweep` + `derived-evaluator-fixtures.json`). Epic 5 gains F4 (the 36 deferred
  units, full list, disposition per unit: 6 to a named "build the missing consumer or propose
  exclusion" path, 27 to the existing option-pool/Tier-1 disposition already satisfied, 1 (`brawler`)
  to a **PROPOSED** (unsigned) Structural Exclusion Register entry, 2 to an ordinary transcription
  fix). Epic 2-F3 and the Completion gate gain the AT-31-010 widening cross-reference. Epic 6/Epic 5
  bullets in the Completion gate updated to name the new seeds.
- `acceptance-and-verification.md`: AT-31-010 widened to also bind the 1,243-unit `display|grounded`
  population (Decision 1(e)'s other named target), with its own Given/When/Then and re-derivation
  command; exit-gate checklist line updated.
- `kanban.md`: `epic-6-ingest-lanes` row lists F1-F11 with the rewrite/new-seed notes;
  `epic-5-chassis-sweep` row gains F4.
- `forward-scope-register.md` G1.3: struck through in place (original text preserved), superseded
  with a dated note pointing to Epic 6 F5-F11 and Epic 5-F4 — doc convention observed, not silently
  rewritten.
- `risks-and-open-questions.md` open question 1: struck through in place, resolved with the same
  dated cross-reference.
- `README.md` In-scope: six-kind bullet added, each kind's not-done figure cited.

**PROPOSED Structural Exclusion Register entry (not signed — a cycle may only propose, per
`decisions.md §3`):** `advanced_class_guide:class_feature:brawler` — missing capability: transient
combat-state representation (helpless/immobilized flags feeding AC-bonus suppression). Recorded in
`epic-breakdown.md` Epic 5-F4 with all four `AT-31-100` fields except operator sign-off; **not**
copied into `acceptance-and-verification.md`'s live register table (that table's own convention is
signed entries only — this cycle's proposal lives with its full reasoning in Epic 5-F4 and awaits
either a real consumer or operator sign-off before promotion to the register proper).

**Retro.** Two corrections emitted (`docs/retro/events/sd31-ready-s2.jsonl`):
- `1786803206624-sd31-ready-s2-19eed7` — Epic 6-F1's pre-remediation "1,242 grounded / 7 done, 0.6 %"
  ingest framing → actual: 1,229 of 1,235 held units are the single cell `derived|grounded`, a
  fixture-coverage gap.
- `1786803206746-sd31-ready-s2-919f72` — the "94 of 2,879" held-derived comment in
  `v06_work_inventory.rs:4585` / `derived_evaluator_fixture_check.rs:14` → actual 2,792 (corpus
  drift), fixture entries confirmed at 94 (unchanged).

**Verification.** Doc-only step; ran `./scripts/verify.sh --only preflight-disk` only (no Rust/
Python/shell production code changed):
```
==> preflight-disk — disk budget check before any build starts
    repo filesystem (/home/ubuntu/workspace/repos/codex, mounted at /): 40% used, 580G available
    scratch-log filesystem (/tmp/codex-verify-J4Red8, mounted at /): 40% used, 580G available
    PASS  preflight-disk  (disk budget OK)
SUMMARY
  passed:  1  preflight-disk
RESULT: PASS
```
`VERIFY_EXIT=0`.

**Not independently re-derived this cycle:** the SD-30 E0-F3 artifact's own 217/100/50-unit
`feat`-residue bucket split (194 `PREABILITY` + 23 `PRESTAT`/`PRESKILL` = 217; 68+16+16=100
option-pool; 50 unclustered) is cited from that artifact's own committed JSON/README, not
independently re-run this cycle — the artifact's own classifier script
(`characterize_feat_unknown.py`) was read but not re-executed. The corpus-wide `feat` `unknown` total
it depends on (367) *was* independently re-derived this cycle (§ figures above, feat ladder) and
matches the artifact's own count exactly, which is the cross-check available without re-running a
script against the external PCGen oracle tree.

**Files changed:** `epic-breakdown.md`, `acceptance-and-verification.md`, `kanban.md`,
`forward-scope-register.md`, `risks-and-open-questions.md`, `README.md`,
`docs/retro/events/sd31-ready-s2.jsonl` (+2 correction events), this file.

**Status:** complete for this step's stated scope. Steps 3-6 of the plan (oracle pin, dashboard
import, drift sweep D1-D14, pre-launch checklist + adversarial verify) are separate steps, not owned
by this receipt.

---

## 2026-08-15 — Step 3 (S3-oracle): PCGen oracle pin + bootstrap + preflight stages

Actor `sd31-ready-s3`. Started at HEAD `8b76b25628fcdbd2bae17605072a82ea8a206340`
(`docs(sd31): open cards for six unowned kinds...` — Step 2's commit); tree clean at start.
Implemented against `~/.claude/plans/conduct-a-launch-readines-zesty-ripple-agent-aplan-code-shapes-409903fa5aadd92c.md`
§1 (1a-1e).

**Deliverables (commit A):**
- `scripts/pcgen-oracle-pin.env` — sourced pin file (`PCGEN_ORACLE_REPO`, `PCGEN_ORACLE_SHA`,
  `PCGEN_ORACLE_SHA_DATE`, `PCGEN_ORACLE_SPARSE_PATHS`), same convention as `verify-baselines.env`.
- `scripts/fetch-pcgen-oracle.sh` — bootstrap/verify script (`--dest`/`--force`/`--check`/`--quiet`),
  no `set -e`, no piped exit-status checks; sparse-cone + depth-1 + `--filter=blob:none` fresh fetch,
  `--force` to move an existing clean off-pin checkout, hard refusal (regardless of `--force`) on any
  dirty tracked cone file, prints `pcgen-oracle: OK <sha> <dest>` then the two `export` lines on
  success.
- `scripts/tests/test_fetch_pcgen_oracle.sh` — 10-scenario detection self-test (11 `pass()` calls;
  case 6 asserts twice: `--check` and `--force` both refuse a dirty file) against a synthetic local
  bare "upstream" (two commits, in-cone + out-of-cone files, `uploadpack.allowFilter`/
  `allowAnySHA1InWant` set). Harness modeled on `scripts/tests/test_corpus_literal_sweep.sh`.
- `scripts/verify.sh`: sources `scripts/pcgen-oracle-pin.env` next to `BASELINES_FILE` (exit 2 if
  missing); two new stages, **`preflight-oracle`** and **`oracle-pin-selftest`**, added to both
  `ALL_STAGES` and `QUICK_STAGES` immediately after `preflight-disk`; dispatch `case` entries added;
  `corpus-sweep`'s comment block gets a pointer to `preflight-oracle`.
- `scripts/verify-baselines.env`: one-line pointer to the new pin file.
- Docs: `AGENTS.md` §Concurrency and Measurement (new bullet — oracle pinned, resolve via
  `$PCGEN_CORPUS_ROOT`/`$PCGEN_REPO_DIR`, never a literal path); SD-30
  `loop-instruction.md` step 1b + its worked `awk` example → `$PCGEN_CORPUS_ROOT`; SD-31
  `loop-instruction.md` override 8 (pin-check is the first command every cycle, pin SHA quoted in
  every re-derive receipt); SD-31 `epic-breakdown.md` Epic 8 rule 4 (cloud bootstrap); SD-31
  `technical-requirements.md` prerequisites; `docs/governance/license-matrix.md:32`;
  `docs/architecture/testing.md` §verify.sh.

**Pin verification (re-derived, not transcribed):**
```
git -C ~/workspace/repos/pcgen rev-parse HEAD
  -> 7f818006e371188e5717fd18d74d18a420747fc6   (matches the brief's pin exactly)
git -C ~/workspace/repos/pcgen status --porcelain --untracked-files=no | wc -l
  -> 0   (clean)
git -C ~/workspace/repos/pcgen remote get-url origin
  -> https://github.com/PCGen/pcgen.git
```

**Proofs (all recorded):**
1. `bash scripts/tests/test_fetch_pcgen_oracle.sh` → `passed: 11  failed: 0`, `SELF-TEST PASSED.`
   **Mutation-proven, not just green:** temporarily removed the dirty-tracked-cone-file guard from
   `do_check()` in `fetch-pcgen-oracle.sh`, re-ran the self-test → exactly the two dirty-file
   assertions (case 6's `--check` and `--force` halves) went RED, everything else stayed PASS;
   reverted (`diff` against the pre-mutation copy confirmed byte-identical), re-ran → green again.
   This is the "prove new gates can fail" requirement satisfied for both the fetch script and its
   self-test in one pass.
2. `./scripts/verify.sh --only preflight-oracle --only oracle-pin-selftest` → both **PASS**
   (`oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6`; `11 passed, 0 failed`).
3. RED proofs. `PCGEN_CORPUS_ROOT=/nonexistent ./scripts/verify.sh --only preflight-oracle` **stayed
   PASS** — `fetch-pcgen-oracle.sh` resolves its checkout location via `--dest`/`$PCGEN_REPO_DIR`/
   `$HOME/workspace/repos/pcgen` only (mirrors `src/oracle_validation/pcgen_runner.rs::default_pcgen_repo_dir()`
   exactly, per design §1b and confirmed by `grep -n PCGEN_REPO_DIR src/oracle_validation/pcgen_runner.rs`);
   `PCGEN_CORPUS_ROOT` is the script's *derived output* (`$DEST/data`), never an input, so setting it
   alone changes nothing. Logged as a correction (`docs/retro/events/sd31-ready-s3.jsonl`,
   `--subject "S3-oracle dispatch brief"`) rather than silently substituted. The command that actually
   produces the brief's intended RED: `PCGEN_REPO_DIR=/nonexistent ./scripts/verify.sh --only
   preflight-oracle` → **FAIL**, console shows `no checkout at /nonexistent -- run
   scripts/fetch-pcgen-oracle.sh --dest /nonexistent to bootstrap it`. Then a scratch clone one commit
   off the pin: `git clone --depth 50 https://github.com/PCGen/pcgen.git <scratch>`, fetched the pin
   commit at depth 2 and checked out its **parent** (`6adec3855e1eca54f55e04b3ee67589bcb0e4ec5`) —
   `PCGEN_REPO_DIR=<scratch> ./scripts/verify.sh --only preflight-oracle` → **FAIL**, naming both SHAs:
   `HEAD=6adec3855e1eca54f55e04b3ee67589bcb0e4ec5 pinned=7f818006e371188e5717fd18d74d18a420747fc6`.
4. Real network fetch into the scratchpad (NOT the operator's clone):
   `bash scripts/fetch-pcgen-oracle.sh --dest <scratchpad>/pcgen-oracle-test/fresh-fetch` →
   `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 <dest>` + both export lines, exit 0.
   `du -sh <dest>` → **108M** (`.git` 22M, `data/` 86M, `system/` 612K — the sparse cone materialized
   `system/gameModes/Pathfinder` only, not all of `system/gameModes`). Then
   `PCGEN_CORPUS_ROOT=<dest>/data cargo run --locked --bin corpus_literal_sweep` →
   `corpus-literal-sweep: 3516 records examined of 9328 read, 36105 tokens compared (9 synthesized),
   8903 digests checked, 0 findings` / `CLEAN` — **identical** record count, token count, and digest
   count to the same binary run against the operator's real full clone (re-run for comparison,
   same output). Proves the sparse cone is sufficient. Scratch clone (`108M`) deleted afterward
   (`rm -rf <scratchpad>/pcgen-oracle-test`); operator's `~/workspace/repos/pcgen` re-confirmed
   unchanged at the pin, clean, 0 status lines.
5. Full `./scripts/verify.sh` at the tip: **started, not yet complete when this receipt was written.**
   Observed so far, in order: `preflight-disk` PASS, `preflight-oracle` PASS (oracle at pin),
   `oracle-pin-selftest` PASS (11/0), `pi-sweep` PASS, `audit-selftest` PASS (28/0), `reclaim-selftest`
   PASS (13/0), `driver-selftest` PASS (7/0), `corpus-sweep-selftest` PASS (15/0), `root-lib` PASS
   (1777 passed) — then `root-full` (building ~490 test binaries) was still running when this turn
   ended. `corpus-sweep` itself was independently confirmed CLEAN with the exact record count
   (`3516`) in proof 4 above, against both the scratch fetch and the real oracle, so the remaining
   risk is narrow (root-full/desktop/reach/frontend/clippy/class-dump, none of which this step's
   diff touches except the two new stage functions already proven PASS above). Log:
   `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d9c38510-724f-408f-b3c9-273134333e9d/scratchpad/verify-s3-full.log`.
   **This is the one open item — see "Remaining" below.**

**Retro events emitted:** one `correction` (`sd31-ready-s3.jsonl`) — the dispatch brief's proof (3)
named `PCGEN_CORPUS_ROOT` as the env var that forces `preflight-oracle` RED on an absent oracle; the
actual governing var is `PCGEN_REPO_DIR` (verified against `pcgen_runner.rs`'s own resolution logic
and by direct command, both cited above).

**Files changed:** `scripts/pcgen-oracle-pin.env` (new), `scripts/fetch-pcgen-oracle.sh` (new),
`scripts/tests/test_fetch_pcgen_oracle.sh` (new), `scripts/verify.sh`, `scripts/verify-baselines.env`,
`AGENTS.md`, `docs/architecture/testing.md`, `docs/governance/license-matrix.md`,
`docs/release/SD-30-class-feature-archetype-bundle/loop-instruction.md`,
`docs/release/SD-31-corpus-closure-grind/{loop-instruction.md,epic-breakdown.md,technical-requirements.md}`,
`docs/retro/events/sd31-ready-s3.jsonl` (new, +1 correction event), this file.

**Wired-integration self-check:** `fetch-pcgen-oracle.sh` performs real `git` operations throughout
(init, remote add, sparse-checkout, fetch, checkout, status, rev-parse) — no stub branches, no
fixture-only paths, no `exit 0` without the checks that justify it. The self-test builds a real local
git repo and drives the real script against it; no mocked git.

**Status: incomplete.** Everything above landed and is independently proven except item 5: the FULL
`./scripts/verify.sh` run at this commit had not finished (was mid-`root-full`) when this turn's
budget ran out. **Remaining:** let the backgrounded full run finish
(`/tmp/claude-1000/.../scratchpad/verify-s3-full.log`, watch for `VERIFY_EXIT=`), confirm exit 0, and
if it is not 0, diagnose and fix before this step is truly closed — do not treat the strong partial
evidence above (root-lib 1777/1777, all 8 selftest/preflight stages green, corpus-sweep independently
CLEAN at the exact expected count) as a substitute for the real full-gate exit code.

---

**2026-08-15 (same day, resumption) — proof (5) closed, S3-oracle done.** Actor `sd31-ready-s3`
resumed at HEAD `acee7f092` (this step's own commit A, already pushed — tree clean at start, `git
status --porcelain` empty). Verified everything the prior turn landed BY CONTENT before proceeding
(file listing, `--list` output, doc-touchpoint greps against §1e, `fetch-pcgen-oracle.sh`'s full
`do_check`/`do_fetch_to_pin`/`do_fresh_clone`/`main()` logic read end to end against design §1b,
`preflight_oracle`'s stage function read against design §1c, plan Step-3 text cross-checked against
every deliverable) — no discrepancies found beyond the already-logged `PCGEN_REPO_DIR` correction.

**Housekeeping found and fixed before re-running the gate:** a `ps -ef` check turned up a SECOND,
orphaned `./scripts/verify.sh` process tree (pid 1252000, `PPID=1`, started 10:26:01 — the prior
turn's own backgrounded full run, `verify-s3-full.log`, apparently never killed when that turn ended)
still alive and building against the SAME `$CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ready-s3`
as a freshly-launched run started this turn (pid 1290529, `verify-s3-full-2.log`). Two concurrent
`cargo test` invocations sharing one target dir is wasteful (cargo's own target-dir lock would have
serialized them, not corrupted anything, but doubled wall-clock and confused which log was
authoritative). Killed the orphaned tree (`kill -TERM` on 1264220/1252002/1252000, confirmed dead)
and let the single remaining, actively-monitored run (`verify-s3-full-2.log`) proceed alone.

**Proof (5), completed this cycle:**
```
RETRO_ACTOR=sd31-ready-s3 CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ready-s3 ./scripts/verify.sh
  -> RESULT: PASS
  -> VERIFY_EXIT=0
  -> passed: 18  preflight-disk preflight-oracle oracle-pin-selftest pi-sweep audit-selftest
     reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop reach
     corpus-sweep frontend-install frontend-test frontend-typecheck clippy class-dump
```
Full stage-by-stage detail (log: `/tmp/claude-1000/.../scratchpad/verify-s3-full-2.log`,
`duration_seconds: 1169`, auto-emitted retro event
`docs/retro/events/sd31-ready-s3.jsonl` id `1786805423520-sd31-ready-s3-67e124`, `head: acee7f092`):
`preflight-disk` PASS; `preflight-oracle` PASS (`oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6`);
`oracle-pin-selftest` PASS (11/0); `pi-sweep` PASS (10 hits/10 baseline rows); `audit-selftest` PASS
(28/0); `reclaim-selftest` PASS (13/0); `driver-selftest` PASS (7/0); `corpus-sweep-selftest` PASS
(15/0); `root-lib` PASS (1777 passed); `root-full` PASS (**6410 passed across 548 suites, all 527
`tests/*.rs` suites executed**); `desktop` PASS (445 passed); `reach` PASS (27 passed); `corpus-sweep`
PASS (**3516 records examined of 9328 read, 36105 tokens compared (9 synthesized), 8903 digests
checked, 0 findings**) — this is the third independent confirmation of the identical 3516/36105/8903
figures (scratch fetch in proof 4, real-oracle re-run in proof 4, and now the full-gate run itself,
all matching); `frontend-install` PASS; `frontend-test` PASS (99/99 files); `frontend-typecheck` PASS
(clean); `clippy` PASS (root:46 desktop:7 warnings, 0 errors — pre-existing warning counts, this
step's diff introduces no new clippy findings, since `fetch-pcgen-oracle.sh`/the test script are
bash, not Rust); `class-dump` PASS (31/31 computing).

**Pre-existing baseline drift observed, NOT this step's diff, NOT fixed here (out of S3-oracle's
write scope — flagging for whichever cycle owns `verify-baselines.env` maintenance next):**
`scripts/verify.sh`'s own SUMMARY block printed three "not failures — update deliberately" notes:
`BASELINE_ROOT_LIB_TESTS` stale (1776 recorded, 1777 measured), `BASELINE_ROOT_FULL_TESTS` stale
(6398 recorded, 6410 measured), `BASELINE_ROOT_TEST_BINARIES` stale (547 recorded, 548 measured).
These three counts moved between whenever `verify-baselines.env` was last hand-updated and this
commit, from work unrelated to S3-oracle (this step added zero Rust tests). Left as-is per "stay
inside the granted write scope" — not a correction to a figure this step stated, just an honest
observation for the next baseline-maintenance pass.

**Status: COMPLETE.** All five PROOFS from the dispatch are now closed: (1) selftest 11/0 green,
mutation-proven; (2) `--only preflight-oracle`/`oracle-pin-selftest` PASS at the tip; (3) RED proofs
for absent oracle and one-commit-off-pin, naming both SHAs (via `PCGEN_REPO_DIR`, logged correction
above); (4) real network fetch into the scratchpad, OK token, 108M, `corpus_literal_sweep` CLEAN with
the identical record count, scratch clone deleted, operator's real clone reconfirmed unchanged; (5)
**FULL `./scripts/verify.sh` at the tip: PASS, `VERIFY_EXIT=0`, all 18 stages green.** Commit A
(`acee7f092`) already pushed to `origin/tranche/10` from the prior cycle; no code change was needed
this cycle, only completing the verification this step's own protocol requires before calling it
done. `scripts/reclaim.sh --apply` re-run this cycle (see below).

---

## 2026-08-15 — S4-dashboard (Plan Step 4, commits B->C, D)

Actor `sd31-ready-s4`. Started at HEAD `62b5dc995`, tree clean (`git status --porcelain` empty).
Read the design file's sections 2/3 and the commit plan
(`~/.claude/plans/conduct-a-launch-readines-zesty-ripple-agent-aplan-code-shapes-409903fa5aadd92c.md`)
before touching anything. THIS IS LOCAL-ONLY WORK under SD-30 state-goals hazard 4: the dashboard
producer runs from cron every 5 minutes under `flock`, so both `~/swarm-observer/PF1e-dashboard.html`
and `scripts/observer/pf1e_dashboard_producer.py` were backed up to
`~/swarm-observer/.backups/*.pre-sd31-20260815-105442` **before any edit**, and every new field
(`mandate_headline`) landed in the SAME commit as its reader (`renderCompletion()`).

**Commit B — `2b232fe1d`.** Imported `~/swarm-observer/PF1e-dashboard.html` byte-identical as
`scripts/observer/PF1e-dashboard.html` (`cmp` confirmed identical, 182,023 bytes). Then, operationally
(not a commit): removed the standalone served file, symlinked
`~/swarm-observer/PF1e-dashboard.html -> ~/workspace/repos/codex/scripts/observer/PF1e-dashboard.html`
(mode 644 on the repo file), matching the producer's own precedent (the cron path already symlinks to
`scripts/observer/pf1e_dashboard_producer.py`). Proved nginx still serves it:
`curl -s -H "Host: hermes.trantor.internal" http://10.0.0.134/swarm/PF1e-dashboard.html` ->
`HTTP_CODE=200 SIZE=182023`, and `cmp`'d the curled body against the repo file: byte-identical.

**Commit C — `d636c922d`.** Producer `work_inventory_panel()` gained `_mandate_headline()` (new
helper next to `_exclude_books_from_flat_counts`) emitting `mandate_headline: {done, denominator
(== total_units, the strict 38,521-unit mandate), denominator_rule, unmapped_units,
secondary_in_scope_measurable}` per the design's exact spec, with a stderr-only sanity assert
(`ladder sum + unmapped == denominator`) that logs, never raises (the cron must keep publishing).
HTML `renderCompletion()` now reads `wi.mandate_headline` (fallback to `by_doneness`/`total_units` for
an older JSON), caption reads "Fully usable, against the whole mandate ... 100% is the bar", a
secondary `.pct-small` line shows the pre-2026-08-15 in-scope figure, a visible warning line appears
when `unmapped_units > 0`, and `assertLadderSums("mandate headline", ...)` runs at render time.
`renderLaneStrip()`'s sticky-header chips switched from `usableDenom()` to the strict per-lane
denominator so header and headline agree (every other `usableDenom()` call site left as-is, per the
design's explicit scope cut). Stale comments at `inScopeUnits()` and the F1-fallout note updated
in place (superseded, dated, original text kept per this program's doc convention). JS syntax verified
with `node --check` on the extracted `<script>` body.

Re-derived, not assumed, run under the SAME `flock` the cron uses:
```
/usr/bin/flock -n /home/ubuntu/swarm-observer/PF1e-dashboard.lock /usr/bin/python3 \
  /home/ubuntu/workspace/repos/codex/scripts/observer/pf1e_dashboard_producer.py
jq .work_inventory.mandate_headline /home/ubuntu/swarm-observer/PF1e-dashboard.json
  -> done 5837, denominator 38521, secondary 5837/30402
```
Matches `decisions.md` Decision 5's 15.15% ruling exactly (5837/38521 = 15.148...%). No sanity-check
warning emitted in `pf1e-dashboard-producer.log`. **Confirmed the NEXT AUTOMATIC cron tick** (not just
the manual run above) produced a fresh JSON: polled `PF1e-dashboard.json`'s mtime until it advanced
past the manual-run timestamp, caught the 11:00:02 tick, and re-checked: same `mandate_headline`
figures, `doneness_unmapped_seen: false` (found via `jq '.. | objects | select(has("doneness_unmapped_seen"))'`
since the field lives nested under a shard-index object, not top-level), zero sanity warnings.
`docs/work-inventory.json` untouched throughout (confirmed via `git diff --stat`) — no inventory
regeneration was needed or performed.

**Commit D — `195b237d3`.** `_doneness_verdict_uncapped`'s `ambiguous` branch now maps
`literal-verified`/`fixture-verified` to `held` (design's option (ii), with its full rationale comment
verbatim) — blocker B6's two previously-unmapped cells. Generator: extracted the two inline
`static`/`derived` stamp loops in `main()` (`src/bin/v06_work_inventory.rs`, was ~4562-4595) into
`apply_done_rung_stamps(inventory, sweep_verified, derived_fixture_verified)`, combined into one loop
with an exhaustive `match` on `wiring_class` (behaviourally identical to the original two separate
loops — Static/Derived are mutually exclusive per unit — but makes the "every other class is a no-op"
invariant a single visible arm rather than two loops' worth of implicit omission). New `#[test]`
`apply_done_rung_stamps_tests::ambiguous_display_computed_items_in_both_verified_sets_stay_unstamped`
puts an Ambiguous/Display/Computed item's own `(book, file, line)`/`id` in BOTH verified sets and
confirms none get stamped, plus a Static control item in the same run that DOES get stamped (proves
the verified sets themselves are wired correctly, not an empty-set false negative). `status_vocabulary`
entries for both `literal-verified` and `fixture-verified` gained the one-sentence addendum from the
design: meaningful only on a static/derived unit; elsewhere the producer reads `held` and the next
regen re-derives the status from the class.

New `scripts/tests/test_pf1e_dashboard_producer.py` (unittest, imports the producer module the same
`importlib.util.spec_from_file_location` way the producer imports its own `observer.py` sibling):
grids the full `WIRING_CLASS_VALUES x` a 9-word status vocabulary (kept in sync by hand with the
Rust `STATUS_VOCABULARY` list, commented as such) = 45 fabricated units, one per cell, `kind="spell"`
(outside `NO_GROUNDING_PROBE`, so the kind-cap never masks a raising cell) through
`compute_wiring_class_summary(doc_path=<tmp fabricated doc>, cache_path=<tmp scratch>)` — asserts
`doneness_unmapped == {}`; plus `ambiguous+literal-verified == held`, `ambiguous+fixture-verified ==
held`, `static+literal-verified == done` (control), and `ambiguous+"bogus-status-word"` still raises.
5/5 pass. Wired as the `producer-selftest` stage in BOTH `verify.sh` stage sets (`ALL_STAGES` and
`QUICK_STAGES`), right after `oracle-pin-selftest`, following that stage's own tally-parsing
convention (`Ran N tests` from unittest's own summary line, 0-cases guard, exit-code gate).
`epic-breakdown.md` Epic 0-F1 gained the acceptance sentence from design §3 verbatim (full-grid
evaluation, `ValueError` cells reported as dead-end/unmapped with counts, fail outright if any
count > 0), cross-referenced to this remediation and to the new producer test.

**PROVE NEW GATES CAN FAIL, both new tests, both restored after:**
- Python: reverted the `ambiguous` tuple to its pre-fix 3-status form
  (`grounded`/`text-complete`/`ingested-magnitude` only) in a copy-and-restore round-trip; re-ran
  `python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py -v` -> 2 errors + 1 failure
  (`ValueError: doneness: unmapped 'ambiguous' + 'literal-verified'`, and the full-grid assertion
  showing the exact unmapped dict); also re-ran through the wired stage,
  `./scripts/verify.sh --only producer-selftest` -> `RESULT: FAIL`. Restored the file byte-for-byte
  from a pre-break copy; `git status --porcelain` confirmed the diff returned to exactly the intended
  D-commit state; re-ran both -> green again.
- Rust: temporarily made `WiringClass::Ambiguous` share `Static`'s stamping condition (a guarded match
  arm inserted ahead of the catch-all); `cargo test --bin v06_work_inventory apply_done_rung_stamps`
  -> `FAILED`, `assertion `left == right` failed: Ambiguous must stay unstamped, left: "literal-verified"
  right: "grounded"`. Restored from a pre-break copy; re-ran -> `ok`, 1 passed.

**Full gate at the tip (`195b237d3`), launched in background early and polled inline this same turn:**
```
RETRO_ACTOR=sd31-ready-s4 CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ready-s4 ./scripts/verify.sh
  -> RESULT: PASS
  -> VERIFY_EXIT=0
  -> passed: 19  preflight-disk preflight-oracle oracle-pin-selftest producer-selftest pi-sweep
     audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop
     reach corpus-sweep frontend-install frontend-test frontend-typecheck clippy class-dump
```
(log `/tmp/claude-1000/.../scratchpad/verify-s4d-full.log`, `duration_seconds: 1389`, retro event
`docs/retro/events/sd31-ready-s4.jsonl` id `1786807831706-sd31-ready-s4-74f426`). `producer-selftest`
itself: `PASS (5 cases passed)`. `cargo test --bin v06_work_inventory` (separately, isolating the new
test group): 84 passed, 0 failed, including the new `apply_done_rung_stamps_tests` module.

**Correction, logged via `retro.py correction` (event `1786807982394-sd31-ready-s4-757924`):** commit
`195b237d3`'s own message mis-stated the `BASELINE_ROOT_LIB_TESTS` stale note (1776 recorded / 1777
measured) as "this commit's own +1 test, expected". That is wrong: the `root-lib` stage runs
`cargo test --locked --lib` only (the crate's `--lib` target), and this commit's new `#[test]` lives
inside `src/bin/v06_work_inventory.rs`'s own `#[cfg(test)]` module — compiled into that BINARY's
separate test target, which `root-full` (not `root-lib`) exercises. The `root-lib` 1776/1777 drift is
**100% pre-existing**: S3-oracle's own receipt (above) already measured `root-lib` at 1777 against the
same 1776 baseline with zero new tests of its own. This commit's actual +1 shows up in
`BASELINE_ROOT_FULL_TESTS`: S3-oracle measured `root-full` at 6410 (12 pre-existing over the 6398
baseline, none of it S3's); this commit measured 6411 = 6410 + this commit's one new test.
`BASELINE_ROOT_TEST_BINARIES` (547 recorded / 548 measured) is unchanged by this commit — no new test
**file** was added, only a test module inside an existing one — and was already 548 at S3's
measurement. Per "stay inside the granted write scope" (S3's own precedent), `verify-baselines.env`
itself is left untouched; this is a correction to what THIS step's own commit message claimed, not a
fix to the baseline file.

**Status: COMPLETE.** Commits B (`2b232fe1d`), C (`d636c922d`), D (`195b237d3`) all landed and pushed
to `origin/tranche/10`. HTML under version control and served via symlink (proven live). Strict
mandate headline live on both a manual and an automatic cron-driven run (proven twice). Blocker B6
closed (both unmapped cells now map to `held`, never `done`, never raise). Both new self-tests proven
able to fail and restored. `producer-selftest` wired into both `verify.sh` stage sets. Epic 0-F1
acceptance sentence added. Full `verify.sh` green at the tip, 19/19 stages. `docs/work-inventory.json`
untouched throughout, as instructed (no inventory change needed).

## 2026-08-15 — Launch-readiness remediation Step 5: S5-drift (RETRO_ACTOR sd31-ready-s5)

**Scope.** Plan Step 5 of `~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md` — drift findings
D1-D14, plus a full re-scan of the package for remaining SD-32-as-live text and stale (pre-merge or
pre-Step-2) epic numbers. **Doc-only step** — no Rust/Python/shell production code touched; ran
`./scripts/verify.sh --only preflight-disk` only, per the dispatch, not the full gate.

**Started from HEAD** `d4693f9b68269b33d07c4f43864bc400339a3022` on `tranche/10`. Tree was clean at
start (`git status --porcelain` empty); confirmed before any write.

**Figures re-derived this cycle, every one reproduced not transcribed (commands and outputs also
recorded inline at each fix site, and via `retro.py correction` for the five that moved a package
figure):**

1. Per-kind `doneness_verdict()` replay (`class_feature`/`monster`/`spell`/`race`/`race_trait`):
   ```
   python3 -c "
   import json, importlib.util, collections
   spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
   mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
   d = json.load(open('docs/work-inventory.json'))['units']
   by_kind = collections.defaultdict(collections.Counter)
   for u in d:
       if u.get('book') == 'beginner_box': continue
       by_kind[u.get('kind')][mod.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
   for k in ('class_feature','monster','spell','race','race_trait'):
       print(k, dict(by_kind[k]))
   "
   ```
   → `spell {'held': 1103, 'in-progress': 132, 'done': 47, 'not-started': 1561}`; `race_trait
   {'not-started': 2934, 'done': 266, 'held': 247}`. Confirms `spell` held/floor is **1,103/1,150**
   (not the 1,235/1,282 `scope-draft.md`'s own copy of this table still carried — D5) and
   `race_trait` done is **266** (not the 264 `epic-breakdown.md SD31-E6-F4`'s header still carried —
   D6). `acceptance-and-verification.md AT-31-005` already had the correct numbers (a prior SD-30
   correction, `SD30-E0-F4-001`); this cycle's fix brings `scope-draft.md`'s independent copy into
   agreement — the disagreement itself was the STOP-condition drift D5 named.
2. PCC-gate exclusion count: `python3 scripts/screen_pcc_load_gates.py` → `TOTAL remaining units
   excluded by a PCC load gate: 682` (was 719 in `AT-31-004` — D7).
3. `wiring_class` distribution, corpus-wide: `python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json'))['units']; print(collections.Counter(u.get('wiring_class') for u in d if u.get('book')!='beginner_box'))"` →
   `Counter({'display': 14366, 'computed': 8477, 'static': 7394, 'derived': 6175, 'ambiguous': 2109})`.
   Confirms `ambiguous` is **2,109**, not the historic 360 `decisions.md` Decision 1(e)'s reproduced
   SD-30 §50(c) quote still carried (D9); `display`+`grounded` re-derived at **1,243** (not that
   quote's 1,416), matching `AT-31-010`'s already-corrected figure elsewhere in this package.
4. Epic 7 premise (D8): re-derived the 7 `future_state` books are already in
   `docs/work-inventory.json` (4,094 units total), and that 4 of them
   (`occult_adventures`/`adventurers_guide`/`inner_sea_magic`/`inner_sea_taverns`) overlap the 23-book
   `class_feature` roster, carrying 1,908 `class_feature` units — command and output recorded in
   `epic-breakdown.md`'s own Epic 7 correction. Named the carrier: **Epic 5** (chassis sweep), not
   Epic 7.
5. Confirmed `scripts/reachability_audit.py` does not exist (`ls` → not found) and carries no stage in
   `./scripts/verify.sh --list` (grep for `reachability` in the stage list → no hits) — grounds D3's
   fix to `README.md`/`technical-requirements.md`.
6. SD-30 hardware live block, re-confirmed by reading `SD-30-.../loop-instruction.md:74-98` directly
   (not re-measured this cycle — a doc-only step, and the figures are already this package's own
   `loop-instruction.md` override 4's citation): 24 cores / 167 GiB RAM / 968 GB disk at 19% used, cap
   8 — grounds D2's fix to `decisions.md` 1(d)'s stale 8-core/45GB/cap-3 capture.

**D1-D14 disposition:**

- **D1** — `state-goals-and-lessons.md` Goals section and `technical-design.md`'s closing
  file-disjointness paragraph both described a three-package split ("joint SD-30→SD-31→SD-32" / "SD-32
  touches race-chassis and classifier engine code") that `decisions.md §2` had already ended.
  Corrected in place, dated, original text struck-through/superseded per this program's convention.
  Also corrected `technical-design.md`'s "What this package does not touch" list, which claimed the
  dashboard producer untouched — false since this package's own Step 4 (`195b237d3`, `2b232fe1d`)
  touched it narrowly.
- **D2** — `acceptance-and-verification.md` frontmatter (`status`/`date`) updated to the post-merge
  2026-08-15 state; `AT-31-001` (Epic 1→3), `AT-31-002` (Epic 3/4/5→5/6/7), `AT-31-004` (Epic 4→6)
  epic numbers corrected; `decisions.md` 1(c) (Epic 3/4/5→5/6/7) and 1(d) ("Governs...Epic 6"→Epic 8,
  plus the stale-hardware supersession) corrected — all as dated in-place corrections, not silent
  rewrites.
- **D3** — `README.md`'s claim "`scripts/verify.sh` runs [`reachability_audit.py`]" was false
  (confirmed by `--list`); corrected to name it Epic 0's own not-yet-built deliverable.
  `technical-requirements.md:11`'s "pre-loop prerequisite" framing was self-contradictory (the script
  is built BY the loop's first epic) — clarified as "before any card other than Epic 0's own first
  cycle."
- **D4** — `forward-scope-register.md` gained `C1.8`/`C1.9` rows, reproduced from
  `SD-30-.../forward-scope-register.md` lines 188/215, matching `README.md`'s existing claim that they
  are tracked here (previously false — no rows existed).
- **D5** — `scope-draft.md`'s `spell` row (1,235/1,282, a copy-paste artifact of `monster`'s
  held + `feat`'s floor) corrected to 1,103/1,150, matching `AT-31-005`'s already-correct copy.
  `retro.py correction` emitted.
- **D6** — `epic-breakdown.md SD31-E6-F4` header's "264 done" corrected to 266. `retro.py correction`
  emitted. (The "1,242 grounded / 7 done" conflation for the `monster` lane, F1, was already fixed by
  Step 2's rewrite — confirmed still correct, no further action.)
- **D7** — `AT-31-004`'s 719-unit PCC-gate figure corrected to 682 (re-run this cycle). `retro.py
  correction` emitted.
- **D8** — `epic-breakdown.md` Epic 7's premise corrected: the 7 books are already in
  `work-inventory.json` (4,094 units), and the 1,908 `class_feature` units in the 4 overlapping books
  are named as Epic 5's, not silently stripped. Mirrored (lighter touch) in
  `state-goals-and-lessons.md`'s "State at package creation" bullet for the same books.
- **D9** — `decisions.md` Decision 1(e)'s reproduced SD-30 §50(c) quote corrected in place
  (360→2,109 `ambiguous`; 1,416→1,243 `display`+`grounded`), and "Reproduced exactly as it stands"
  changed to "Reproduced with edits noted" with a correction paragraph explaining both moved figures.
  `retro.py correction` emitted.
- **D10** — `SD-30-class-feature-archetype-bundle/state-goals-and-lessons.md:116`'s "dashboard
  producer is not under version control" corrected in place (struck-through, dated) — false as of this
  package's own commit `2b232fe1d`. Touched outside this package's own directory because the dispatch
  named this exact file/line explicitly; no other SD-30 file touched.
- **D11** — `docs/governance/loop-instruction-template.md` lines 37/42/127 (Hermes kanban references)
  each gained a "retired 2026-08-01, see SD-30 decisions §14a" note, without rewriting the template's
  own checklist structure.
- **D12** — Added **Epic 9-F3 — Bundle code review of SD-31's own diff** to `epic-breakdown.md`,
  shaped on `SD-30-.../epic-8-code-review` (three parallel read-only dimensions: correctness/no-stub/
  reach, test quality, doc-fact accuracy) plus an adversarial-verify refutation pass and per-finding
  disposition (`fixed-in-bundle`/`deferred`). Epic 9-F1's exit-gate acceptance now requires F3
  `COMPLETE`. `kanban.md`'s `epic-9-closure` row updated to mention it. Closed
  `risks-and-open-questions.md` open question 2 in place (struck-through, dated, resolution recorded).
- **D13** — `loop-instruction.md` gained override 9: a quoted live-dashboard figure must name its
  source (`by_status` vs. `by_doneness`/`cross_tab`, with stamps) whenever
  `work_inventory.status_sources_agree` is `false` — verified the field's real meaning by reading
  `pf1e_dashboard_producer.py`'s `_cross_tab_status_margin`/`work_inventory_panel()` directly.
- **D14** — Added a dated merge receipt to this file (above, "2026-08-15 — SD-32 absorbed; epics
  re-sequenced"), summarizing `decisions.md §2`-§4, retroactively covering commit `fae30fc12` which had
  none. Confirmed `fae30fc12` was doc-only via `git show --stat`.

**Re-scan for remaining SD-32-as-live text / stale epic numbers** (beyond D1-D14's named sites): ran
`grep -n "SD-32"` and epic-number patterns (`Epic N-FM`, `epic-N-<name>` card IDs) across every `.md`
in the package. Every remaining `SD-32` hit is a historical quote (operator ruling verbatim, provenance
citation, or `progress.md`/`decisions.md` narrative) — none treat SD-32 as a live sibling. Found and
fixed one additional drift beyond the named D-items: `release-notes.md`'s "Split provenance" section
still stated the split-time-only Epic 1-6 ↔ SD30-E4/5/6/10/11/14 mapping as current; corrected in
place, dated, pointing at `README.md`'s live renumber map.

**Retro corrections emitted this cycle** (`docs/retro/events/sd31-ready-s5.jsonl`, `python3
scripts/retro.py validate` → clean, 1101 events all valid):
1. `1786808823998-sd31-ready-s5-653f06` — spell held/floor 1,235/1,282 → 1,103/1,150.
2. `1786808824115-sd31-ready-s5-427396` — race_trait done 264 → 266.
3. `1786808824228-sd31-ready-s5-5d24dc` — PCC-gate exclusion 719 → 682.
4. `1786808824341-sd31-ready-s5-1ab920` — Decision 1(e) ambiguous 360 → 2,109 (and display+grounded
   1,416 → 1,243, folded into the same correction since both live in the same reproduced quote).

**What was NOT re-derived this cycle** (explicitly out of this step's scope, per the dispatch): the
`monster`/`spell`/`race`/`race_trait` D6 "1,242 grounded/7 done" conflation fix (already Step 2's,
verified still correct, not re-done); anything requiring a fresh `docs/work-inventory.json` regen
(none of this cycle's fixes needed one — the file was read-only throughout, confirmed by `git diff
--stat docs/work-inventory.json` showing no change).

**Verify.** Doc-only step. Ran the disk-preflight stage only, per the dispatch instruction
("Doc-only unless you touch code: `--only preflight-disk`"):
```
RETRO_ACTOR=sd31-ready-s5 CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ready-s5 \
  ./scripts/verify.sh --only preflight-disk > "$LOG" 2>&1; echo VERIFY_EXIT=$? >> "$LOG"
```
→ `repo filesystem: 46% used, 525G available; scratch-log filesystem: 46% used, 525G available; PASS
preflight-disk; RESULT: PASS; VERIFY_EXIT=0`. This step touched no Rust/Python/shell production code,
so the full gate was not required; `preflight-disk` alone confirms the shared checkout has headroom
for this and the next cycle.

**Status: COMPLETE.** All 14 named drift items (D1-D14) fixed in place with dated corrections; four
figure corrections emitted via `retro.py correction`; one additional drift found and fixed by the
re-scan (`release-notes.md`); full package re-scan for SD-32-as-live and stale epic numbers came back
clean beyond the named items. 16 files touched, all `docs/`, all doc-only — no Rust/Python/shell
production code changed this cycle.

---

## 2026-08-15 — S6-prelaunch: SD-30 seven-item pre-launch checklist, run for SD-31

`RETRO_ACTOR=sd31-ready-s6 CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ready-s6`. Doc-only step
(no Rust/Python/shell production code touched) — the checklist is `SD-30-class-feature-archetype-bundle/loop-instruction.md`
"Pre-launch checklist" section (lines 31-45), run against this package's own `kanban.md`/branch/box per
the plan's Step 6 part 1 instruction. Started at HEAD `583aaecdf05c84e9ffec7946ac5782e2ae0457f` (matches
the tip left by S5); tree was clean at start (`git status --porcelain` empty) and stayed clean through
every read-only command below.

**1. `kanban.md` exists and lists a ready queue, epic-0 first.**
`docs/release/SD-31-corpus-closure-grind/kanban.md`'s Cards table: `epic-0-reachability-audit` is row
1, status `READY`, Order 1 ("Reachability Audit (standing gate)"). "Claim-priority order is the table
order, top-down" (kanban.md, above the table) — epic-0 is dispatched first by construction. **PASS.**

**2. Branch pushed, `tranche/10` == origin.**
```
git fetch origin tranche/10 && git rev-parse HEAD && git rev-parse origin/tranche/10
```
→ both `583aaecdf05c84e9ffec7946ac5782e2ae0457f`. **PASS.** (Fetch printed a harmless
`/tmp/codex-cred-helper.sh store: 1: /tmp/codex-cred-helper.sh: not found` line — a stale credential
helper hook, not a fetch/push failure; the fetch itself succeeded and the SHA compare is exact.)

**3. OAuth credentials valid.**
```
gh auth status
```
→ `Logged in to github.com account electricm0nk ... Active account: true ... Token scopes: 'project',
'repo', 'workflow', 'write:packages'`. **PASS for push** (repo scope present, matches this cycle's own
push at the end of this receipt). One gap noted and defaulted-past per unattended-mode protocol: `!
Missing required token scopes: 'read:org'` — not required by any push/PR-open operation this package's
epics perform (`gh pr create` needs `repo`, which is present); flagged here, not blocking.

**4. Working tree clean.**
```
git status --porcelain
```
→ empty, before and after every command in this receipt except this file's own edit and the
`retro.py`/git commands the closing steps run. **PASS.**

**5. Wave disk budget, re-derived (not carried from S3/S4/S5's captures), SD-30 loop-instruction
"Concurrency and resource budget" method:**
```
nproc                    # 24
free -h                  # 167Gi total / 7.6Gi used / 1.8Gi free / 160Gi available
df -B1G /                # 968 total / 443 used / 526 avail / 46%
grep -n 'PREFLIGHT_DISK_MIN_FREE_GB=\|PREFLIGHT_DISK_MAX_PERCENT=' scripts/verify.sh
#   :251 PREFLIGHT_DISK_MIN_FREE_GB=${PREFLIGHT_DISK_MIN_FREE_GB:-20}
#   :252 PREFLIGHT_DISK_MAX_PERCENT=${PREFLIGHT_DISK_MAX_PERCENT:-90}
du -sh target                                    # 83G  (primary checkout's accumulated tree)
du -sh /home/ubuntu/cargo-targets/*              # 14 other agents' dirs, 3.3G-30G each, none
                                                  #   this cycle's to reclaim (see item 9 below)
```
| quantity | value | how |
|---|---:|---|
| cores | 24 | `nproc` |
| RAM | 167 Gi total, 160 Gi available | `free -h` (raw `free` col reads 1.8 Gi — page cache, not a real constraint; `available` is the operative figure, matching SD-30's own method) |
| filesystem | 968 G | `df -B1G /` |
| currently used | 443 G (46 %) | same |
| `preflight-disk` refuses at | 90 % used or < 20 G free | `verify.sh:251-252` |
| headroom to the 90 % floor | **428.2 G** | `0.90 × 968 − 443` |
| headroom to the 20 G-free floor | 506 G | `526 − 20` |
| binding headroom | **428.2 G** | the smaller (90 % floor binds) |
| a full-gate `CARGO_TARGET_DIR`, measured | 83 G | `du -sh target` (conservative: today's accumulated primary, the SD-30 method's own choice over a fresher/smaller sibling) |
| concurrent full-gate agents (disk) | **5** | `428.2 ÷ 83 = 5.16` → floor 5 |
| concurrent full-gate agents (CPU) | 12 | `24 ÷ 2` default `-j`; not binding |
| RAM headroom at 5 agents × `-j 2` | ample | ~10 concurrent `rustc` jobs × 2-4 G ≈ 20-40 G against 160 Gi available; not binding |
| binding constraint | **disk** | 5 < 12 |
| **CAP: concurrent full-gate agents this box can carry today** | **5** | smaller of disk/CPU/RAM bounds |

This is a fresh re-derivation, not a carry-forward: the box's disk usage moved from the 178 G/19 %
figure SD-30's own loop-instruction last captured (2026-08-14) to 443 G/46 % now (SD-31's own S3/S4/S5
cycles each left an 27-30 G `CARGO_TARGET_DIR` under `/home/ubuntu/cargo-targets/` — see item 9), and
the cap moved with it, 8 → **5**. **PASS** (budget computed and recorded before any wave fires; no wave
has fired yet under this receipt).

**6. Pilot/scope validation — N/A by scope, stated.**
SD-30 loop-instruction's "Pilot and scope validation" section (lines 155-203) is **"REQUIRED before a
first cycle pins a book or class"** — i.e. it is performed by the first Epic 3/5/6 cycle that actually
claims a specific book, using the three-command at-source procedure against
`$PCGEN_CORPUS_ROOT`/`docs/work-inventory.json`, not by the pre-launch checklist itself. This S6 cycle
opens no kanban card, claims no book, and pins no class — it is a doc-only checklist-and-receipt step.
**N/A for this cycle; not satisfied-in-advance.** The obligation carries forward unchanged to whichever
cycle first claims `epic-3-measurement`, `epic-5-chassis-sweep`, or `epic-6-ingest-lanes` for a named
book/class — that cycle must still run and record all three commands per the loop-instruction text
before pinning, exactly as SD-30 required. Recorded here so the launch declaration below does not
silently treat this item as done.

**7. `epic-0-reachability-audit` card status known.**
```
grep -n "epic-0" docs/release/SD-31-corpus-closure-grind/kanban.md
```
→ `epic-0-reachability-audit | READY | Order 1 — Reachability Audit (standing gate) | ...`. Per
`kanban.md`'s own claim-priority note, epic-0 is claimed *first* (top-down table order) rather than
running fully decoupled the way SD-30's `epic-0-instrument-apply` did — a real difference from the
SD-30 checklist's item 7 wording, noted rather than silently equated. The only downstream gate
naming epic-0 explicitly is `epic-9-closure` ("`epic-0` audit at closing tip"), which is expected
(closure needs the final audit) and not a mid-run block on any other epic. **PASS** (status known:
`READY`, unclaimed, first in priority order, gates only closure).

**Plus: `./scripts/verify.sh --only preflight-oracle` green, SHA recorded.**
```
RETRO_ACTOR=sd31-ready-s6 CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ready-s6 \
  ./scripts/verify.sh --only preflight-oracle > "$LOG" 2>&1; echo VERIFY_EXIT=$? >> "$LOG"
```
→
```
==> preflight-oracle — scripts/fetch-pcgen-oracle.sh --check
    PASS  preflight-oracle  (oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6)
SUMMARY
  passed:  1  preflight-oracle
RESULT: PASS
VERIFY_EXIT=0
```
Pin SHA `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`, dated 2026-06-17)
matches the live oracle at `/home/ubuntu/workspace/repos/pcgen`, confirmed independently via
`./scripts/fetch-pcgen-oracle.sh --check` → `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6
/home/ubuntu/workspace/repos/pcgen`. **PASS.**

**Plus: `scripts/reclaim.sh` dry run, then `--apply`.**
```
./scripts/reclaim.sh            # dry run
./scripts/reclaim.sh --apply
```
→ dry run: `would reclaim: 0 item(s), 0.0B total; skipped: 36 item(s)`. `--apply`: `reclaimed: 0
item(s), 0.0B total; skipped: 36 item(s)`. Both runs skip the same 36 items — 1 stray cargo-target dir
outside its scanned roots, 13 `/tmp/codex-verify-*` logs (too young), 8 forbidden-path worktrees, 14
branches (unmerged-with-upstream or checked-out-in-a-worktree). **Observed, not a blocker, but flagged
per SD-30's own "0.0 B reclaimed means the box is structurally full, not that it is clean" doctrine —
here the doctrine's premise doesn't hold as stated: `reclaim.sh`'s `cargo-target` sweep only scans
`$RECLAIM_SCRATCHPAD_ROOT` (default `/tmp/claude-1000`) and `$RECLAIM_CACHE_ROOT` (default
`$HOME/.cache`) — it never scans `/home/ubuntu/cargo-targets/`, which is where this package's own
per-agent `CARGO_TARGET_DIR`s (S3/S4/S5/S6 and 10+ sibling SD-30 agents, 3.3G-30G each, ~230G total)
actually live. The 443G/46% disk figure in item 5 is real and the cap of 5 stands on it, but the
0.0B reclaim result is not evidence the box is clean or full — it is evidence `reclaim.sh` cannot see
that directory at all. This is a gap in `reclaim.sh`'s scanned-roots list, not this cycle's to fix
(doc-only step, no code write authorized); default-and-flag for the operator / a future code-touching
cycle.**

### Launch declaration

**LAUNCH-READY**, with two named, non-blocking gaps carried forward (not silently marked done):

1. Item 6 (pilot/scope validation) is genuinely N/A-by-scope for this checklist-only cycle and remains
   an open obligation on the first cycle that claims a book/class — that cycle must run it, not assume
   this receipt covers it.
2. `scripts/reclaim.sh` does not scan `/home/ubuntu/cargo-targets/`, so its `0.0B reclaimed` result
   under-reports true reclaimable space and should not be read as "box is full" or "box is clean" —
   either reading is unsupported by what the tool actually checked. Disk budget (item 5, cap **5**
   concurrent full-gate agents) was computed from `df` directly, not from the reclaim result, so this
   gap does not invalidate the cap.

All seven SD-30-shaped checklist items, plus `preflight-oracle` (green, pin `7f818006e37...`) and the
reclaim dry-run/`--apply` pair, are satisfied or explicitly accounted for. `git status --porcelain`
clean throughout; `origin/tranche/10 == HEAD` (`583aaecdf...`) before this receipt's own commit.

**Verify.** Doc-only step; only `--only preflight-oracle` run (see above), `VERIFY_EXIT=0`. No
Rust/Python/shell production code changed this cycle — full `./scripts/verify.sh` not required and not
run.

**Status: COMPLETE.**

## 2026-08-15 — SD31-S7-VERSION-001: version bump 0.10.0 -> 0.11.0 for the `tranche/11` cut (`decisions.md §6`, Epic 10)

**Started from HEAD** `1980d6b95` on `tranche/11` (freshly cut, verified `git rev-parse --abbrev-ref
HEAD`/`git rev-parse HEAD`/`git status --porcelain` clean before any write). Ended at HEAD `0b557ac43`
before this receipt's own commit.

**Commits:**

- `147f1c2b7` — `feat(sd31): version bump 0.11.0 for tranche/11`.
- `0b557ac43` — `docs(sd31): SD-31 operates on tranche/11 as release 0.11.<build>`.

**Surface enumerated fresh** (not trusted from the dispatch brief or the prior SD-30 bump), via
`grep -rn '0\.10\.0'` / `'0\.10\.\${'` across `apps/desktop/package.json`,
`apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`, `apps/desktop/src/**`,
`.github/workflows/publish-tester-release.yml`, and a repo-wide unfiltered pass — 15 files bumped:

- `apps/desktop/package.json:4` — `"version": "0.10.0"` -> `"0.11.0"`.
- `apps/desktop/src-tauri/tauri.conf.json:4` — same.
- `apps/desktop/src-tauri/Cargo.toml:3` — `version = "0.10.0"` -> `"0.11.0"`.
- `apps/desktop/src-tauri/Cargo.lock:479` — the `codex-desktop` package entry only (grep-verified no
  other crate in the lock carries `0.10.0`); **`cargo metadata --locked` confirmed the lock stayed
  consistent with the hand-edit — no regeneration was required**, because the root package's own
  `version` field does not participate in dependency-resolution constraints for its dependencies. This
  differs from `SD30-E7-F1-001`'s precedent (which did need a lock regeneration); noted here so the
  next bump doesn't assume regeneration is always required.
- `.github/workflows/publish-tester-release.yml:105` — `VERSION="0.10.${GITHUB_RUN_NUMBER}"` ->
  `"0.11.${GITHUB_RUN_NUMBER}"`; comment lines 74 and 92 (now ~80/99) updated to record `v0.11.0` and
  the `tranche/11` bump.
- 8 files under `apps/desktop/src` carrying the literal `'Codex 0.10.0-test'` -> `'Codex
  0.11.0-test'` (and `makeSurface.ts`'s `'0.10.0-test'` version field): `testSupport/makeSurface.ts`,
  `testerWorkbench/loadTesterWorkbenchSurface.test.ts`,
  `testerWorkbench/status/createWorkbenchStatus.test.ts`, `operatorTriage/buildOperatorTriageDraft.test.ts`,
  `testerWorkbench/feedback/evidence/captureFeedbackEvidence.test.ts`,
  `testerWorkbench/feedback/enhancement/composeEnhancementRequest.test.ts`,
  `testerWorkbench/feedback/bug/composeBugReport.test.ts` — **this is the 7 entries in
  `releaseChecks/buildLabelFixtureFreshness.test.ts`'s `FIXTURE_FILES` list plus `makeSurface.ts`
  itself (8 total); the dispatch brief guessed 7, missing `buildOperatorTriageDraft.test.ts`** —
  caught by the fresh grep, not by trusting the brief's count.
- `releaseChecks/buildLabelFixtureFreshness.test.ts` — `STALE_LABEL` constant `'Codex 0.9.0-test'` ->
  `'Codex 0.10.0-test'` (the one-bump-behind literal this check is designed to name, per its own doc
  comment) plus the surrounding comment block; `FIXTURE_FILES` itself needed no additions.
- `releaseChecks/buildVersionTriple.test.ts` and `release/buildVersionTriple.test.ts` (two separate
  copies of the same check) — hardcoded tranche anchor `pkg.startsWith('0.10.')` -> `'0.11.'`, plus the
  comment blocks naming `tranche/10` -> `tranche/11`. Neither file was named in the dispatch brief;
  found via the `grep -rn '0\.10\.0'` sweep and confirmed load-bearing (a self-verifying test with one
  literal anchor, exactly the shape SD-30's Epic 7 lesson warns about).

**Publish counter.** `gh run list --workflow publish-tester-release.yml -L 3 --json
databaseId,number,conclusion,headBranch,createdAt` — last completed run `#123` (success, `develop`,
2026-08-14T14:19:28Z, the `PR #360` merge). Next publish from this lineage stamps `0.11.124`.

**Package docs.** `decisions.md` Decision 6 records the ruling, cut point, and closing-PR increment
rule. `canonical_branch`/`build_version_target` frontmatter updated in README.md, kanban.md,
epic-breakdown.md, scope-draft.md; `technical-requirements.md`'s checkout line; `decisions.md`
subsection (d) and `epic-breakdown.md`/`kanban.md`'s Epic 8 rows (owns merges to `tranche/11`, not
`tranche/10`); `risks-and-open-questions.md` risk 5 restated (separate branch now, not a same-branch
collision). Epic 10 (`SD31-E10`) added to `epic-breakdown.md` mirroring SD-30 Epic 7, plus its kanban
row, a `Recommended sequencing` line, and a `Completion gate` line. `loop-instruction.md` grep-checked
clean, no branch/version literals present, no change needed. Historical `tranche/10` receipts earlier
in this file left as written, with the dated note now at the top of this file.

**Gate.** Launched `./scripts/verify.sh` immediately after the version-bump commit; ran inline while
docs edits proceeded. Log: `artifacts/sd31-s7-version-verify.log`.

```
SUMMARY
  passed:  19  preflight-disk preflight-oracle oracle-pin-selftest producer-selftest pi-sweep
  audit-selftest reclaim-selftest driver-selftest corpus-sweep-selftest root-lib root-full desktop
  reach corpus-sweep frontend-install frontend-test frontend-typecheck clippy class-dump

BASELINE NOTES (not failures — left as-is per instruction, verify-baselines.env not touched):
  - BASELINE_ROOT_LIB_TESTS: 1776 recorded, 1777 measured.
  - BASELINE_ROOT_FULL_TESTS: 6398 recorded, 6411 measured.
  - BASELINE_ROOT_TEST_BINARIES: 547 recorded, 548 measured.

RESULT: PASS
VERIFY_EXIT=0
```

19/19 stages passed, including `root-lib` (1777 tests), `root-full` (6411 tests / 548 suites, all 527
`tests/*.rs` suites executed), `desktop` (445), `reach` (27), `corpus-sweep` (3516/9328 records, 0
findings), `frontend-test` (99/99 files), `frontend-typecheck` clean, `clippy` (root:46 desktop:7
warnings, 0 errors, no new errors), `class-dump` (31/31 computing). `verify.sh` itself emitted a
`derived` verification retro event
(`docs/retro/events/sd31-ready-s7-version.jsonl`, `id`
`1786811691317-sd31-ready-s7-version-8aaa1b`).

**Status: COMPLETE.** `HEAD` `1980d6b95` -> `0b557ac43` (+ this receipt's own commit). Epic 10 flipped
to `COMPLETE` in `kanban.md` on this gate result.

---

## Cycle SD31-W1-PREFLIGHT-001 (sd31-w1-preflight, 2026-08-15)

**Wave-admission cycle. No production code touched — markdown only, so no full gate run for this
cycle** (stated per instruction; the substantive full-gate figures below are re-derived from the live
board and box state, not from a fresh `verify.sh` run).

**Checkout state.** Started at `HEAD 3f756b4b9` (`git log -1`: "docs(sd31): unattended-mode
open-issues log + orchestrator dispatch log with re-derived baseline"), branch `tranche/11`. Package
dir present; no recovery needed. `git status --porcelain` showed only `docs/retro/events/codex.jsonl`
modified (another agent's retro append, left untouched).

**Oracle pin.** `./scripts/verify.sh --only preflight-oracle` -> `PASS (oracle at pin
7f818006e371188e5717fd18d74d18a420747fc6)`. `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`).

### 1. Box measurement

```
nproc                        # 24
free -h                      # 167Gi total / 6.9Gi used / 154Gi free / 161Gi available
df -B1G /                    # 968 total / 151 used / 818 avail / 16%
du -sh /home/ubuntu/cargo-targets/* target 2>/dev/null
#   (cargo-targets/ is empty — no prior agent left a target dir there)
#   83G  target               (primary checkout's accumulated tree)
```

### 2. `reclaim.sh`

`./scripts/reclaim.sh` (dry run) then `./scripts/reclaim.sh --apply`: **0.0 B reclaimed**. All 25
candidates correctly skipped — 23 `verify-logs` under `/tmp/codex-verify-*` too young (<6h), 2
branches not merged into `origin/develop`/upstream present, and one stray non-cargo-target dir
(`/home/ubuntu/workspace/codex-target-wiring-classifier`, skipped "not a cargo target dir"). Zero
findable stale garbage on this box right now, not an unclean box.

**Known hardening gap, confirmed not fixed here:** `scripts/reclaim.sh`'s `cargo-target` category
scans only two roots — `SCRATCHPAD_ROOT="${RECLAIM_SCRATCHPAD_ROOT:-/tmp/claude-1000}"` and
`CACHE_ROOT="${RECLAIM_CACHE_ROOT:-$HOME/.cache}"` (`scripts/reclaim.sh:122-123`, `roots=("$SCRATCHPAD_ROOT"
"$CACHE_ROOT")` at line 515). **`/home/ubuntu/cargo-targets/` — the directory this package's own
loop-instruction mandates every agent's `CARGO_TARGET_DIR` live under — is never in that scan.** The
dry-run output confirms this empirically: it enumerated nothing under `cargo-targets/` even to skip
it. Any orphaned per-agent target dir left there (SD-30's `sd29-e2-prelaunch`-style leak) would sit
forever unreclaimed by this tool. Per instruction, not fixed in this cycle — noted for a future
hardening card.

### 3. Wave disk budget (this wave: 2 concurrent full-gate agents)

Following SD-30 `loop-instruction.md` "Concurrency and resource budget" arithmetic exactly, re-measured
today:

| quantity | value | how |
|---|---:|---|
| cores | 24 | `nproc` |
| RAM | 167 Gi total, 154 Gi free | `free -h` |
| filesystem | 968 G | `df -B1G /` |
| currently used | 151 G (16 %) | `df -B1G /`, this cycle |
| `preflight-disk` refuses at | 90 % used or < 20 G free | `verify.sh:243-244` (`PREFLIGHT_DISK_MIN_FREE_GB=20`, `PREFLIGHT_DISK_MAX_PERCENT=90`) |
| headroom to the 90 % floor | **720.2 G** | `0.90 × 968 − 151` |
| headroom to the 20 G-free floor | 798 G | `818 − 20` |
| **binding headroom** | **720.2 G** (90 % floor binds) | `min(720.2, 798)` |
| a full-gate `CARGO_TARGET_DIR`, measured | 83 G | `du -sh target` above (primary checkout's accumulated footprint; `cargo-targets/` empty right now, no fresh-footprint sample available this cycle) |
| **concurrent full-gate agents (disk)** | **8** | `720.2 ÷ 83 = 8.68` -> floor 8 |
| **concurrent full-gate agents (CPU)** | **12** | `24 ÷ 2 (default -j per agent) = 12`; not binding |
| RAM headroom at 8 agents × -j 2 | ample | ~16 concurrent rustc jobs × ~2-4G each ≈ 32-64G against 154 Gi free; not binding |
| **binding constraint** | disk | 8 < 12 |
| **CAP: concurrent full-gate agents today** | **8** | smaller of the disk/CPU/RAM bounds |

**This wave dispatches 2 concurrent full-gate agents. 2 ≤ 8 — the budget admits 2**, with substantial
headroom to spare (would admit up to 8 before the disk floor binds).

### 4. Board baseline re-derive

Re-derived by replaying the dashboard producer's own `doneness_verdict()` over the live
`docs/work-inventory.json`, not transcribed from any prior receipt or from `ORCHESTRATOR-LOG.md`:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
c = collections.Counter(P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind')) for u in U)
print(d.get('generated_at'), len(U), dict(c), round(100*c['done']/len(U),2))
"
```

Result: `generated_at 2026-08-15T01:34:18Z`, denominator **38,521**, `done` **5,837** = **15.15 %**.
`not-started` 20,895 · `held` 6,916 · `unmeasurable` 3,989 · `in-progress` 848 · `deferred` 36.

Per-kind (same command, grouped by `kind`):

| kind | total | done | done % | held | not-started | other |
|---|---:|---:|---:|---:|---:|---|
| class_feature | 15,472 | 25 | 0.16 % | 88 | 11,703 | unmeasurable 3,622; deferred 34 |
| equipment | 6,208 | 2,626 | 42.30 % | 2,327 | 962 | in-progress 293 |
| race_trait | 3,447 | 266 | 7.72 % | 247 | 2,934 | — |
| monster_ability | 3,107 | 334 | 10.75 % | 1,295 | 1,478 | — |
| spell | 2,843 | 47 | 1.65 % | 1,103 | 1,561 | in-progress 132 |
| feat | 2,610 | 1,178 | 45.13 % | 89 | 973 | unmeasurable 367; deferred 2; in-progress 1 |
| companion | 1,696 | 416 | 24.53 % | 506 | 774 | — |
| equipment_modifier | 1,580 | 911 | 57.66 % | 19 | 228 | in-progress 422 |
| monster | 1,270 | 7 | 0.55 % | 1,235 | 28 | — |
| class | 185 | 27 | 14.59 % | 0 | 158 | — |
| race | 103 | 0 | 0.00 % | 7 | 96 | — |

**Cross-check against `docs/release/SD-31-corpus-closure-grind/artifacts/ORCHESTRATOR-LOG.md`'s
"Baseline at orchestration start" table: exact match on every figure** — denominator 38,521, `done`
5,837/15.15 %, the `not-started`/`held`/`unmeasurable`/`in-progress`/`deferred` breakdown, and all 11
per-kind rows. **No hard stop.** Both re-derivations resolve to the identical `docs/work-inventory.json`
snapshot (`generated_at 2026-08-15T01:34:18Z`), unchanged since the orchestrator's own baseline capture
earlier the same day — no board mutation occurred between the two reads.

**Status: COMPLETE.** No code change; docs-only commit. `HEAD 3f756b4b9` -> (this receipt's commit).

## SD31-E2-F1-001 — Epic 2 F1: hand-labelled ground-truth sample (the gate that runs first)

**Role:** sd31-e2-groundtruth. **Worktree:** `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_09fec605-4d0-3`,
branch `sd31/e2-groundtruth`. **HEAD start:** `65decb5e0` (`docs(sd31): W1-preflight`), recovered onto
via `git fetch origin && git reset --hard origin/tranche/11` — the worktree was cut before the SD-31
package directory existed in it; `git status --porcelain` was clean, package dir absent, recovered per
protocol and recorded here. **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`), confirmed via `./scripts/verify.sh --only preflight-oracle` -> PASS.

**Card:** `epic-2-verdict-paths`, feature seed **SD31-E2-F1 only**. No classifier code written this
cycle (the acceptance-mandated gate). Full acceptance read from `epic-breakdown.md` "## Epic 2
(SD31-E2)" and `decisions.md` Decision 1(e).

### What landed

- `artifacts/SD31-E2-F1-ground-truth-sample-v1.json` — 150 hand-labelled units, one object per unit:
  `id`, `kind`, `book`, `name`, `population` (`null` / `ambiguous_target` / `display_grounded_target`),
  `engine_wiring_class` + `engine_wiring_class_reason` + `engine_status` (as of this cycle's
  `docs/work-inventory.json`), `hand_wiring_class`, `confidence` (`high`/`medium`), `agrees_with_engine`
  (bool), `token_evidence` (the specific corpus tokens deciding the label), `source_file`/`source_line`.
- `artifacts/SD31-E2-F1-ground-truth-methodology.md` — sampling method, stratification table, judgement
  calls, and three named findings (A: `no_corpus_line` path-resolution bug, load-bearing; B: `BONUS:STAT`
  selector-name + `DR:`/`CR:` "/" notation false positives for `derived`; C: case-sensitive scalar-match
  false negative).

### Sampling — re-derived, commands in the methodology note

150 units total: 70 general-stratified (14 per wiring class × proportional-by-kind), 40 oversampled from
`wiring_class == 'ambiguous'` (re-derived **2,109**, matches `decisions.md` Decision 1(e) exactly), 40
oversampled from `wiring_class == 'display' AND status == 'grounded'` (re-derived **1,243**, matches
AT-31-010). Fixed seed `random.seed(31)`. All 5 wiring classes and all 11 corpus kinds represented
(floor was 5 classes / 4 kinds). `beginner_box` excluded (matches the live dashboard producer).

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
units = [u for u in d['units'] if u.get('book') != 'beginner_box']
print(collections.Counter(u.get('wiring_class') for u in units))
print('ambiguous', len([u for u in units if u.get('wiring_class')=='ambiguous']))
print('display+grounded', len([u for u in units if u.get('wiring_class')=='display' and u.get('status')=='grounded']))
"
# -> Counter({'display': 14366, 'computed': 8477, 'static': 7394, 'derived': 6175, 'ambiguous': 2109})
# -> ambiguous 2109
# -> display+grounded 1243
```

### Labelling — read the whole corpus record, not a filtered field

For every sampled unit: resolved the book directory under `$PCGEN_CORPUS_ROOT`
(`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/...`), read the base `.lst` row at
`source_file`/`source_line` **searched recursively** under the book directory (not assumed to sit at
book root — see Finding A below, which exists precisely because the production code does NOT search
recursively), then searched every `.lst` file in that book tree for `.MOD` rows targeting the unit's
`name`/`corpus_key` (the GE-01 token closure). Applied D0-D6 from
`docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`
directly to the raw tokens — independently of running the production determinator — so the sample can
serve as ground truth for it, per Decision 1(e) item 1.

### Result: 150 sampled, 107 agree with the engine's current `wiring_class`, 43 disagree

**Finding A (load-bearing).** `wiring_class::CorpusLines::line()` (`src/rules_core/wiring_class.rs:758`)
resolves a unit's row via a single-level `dir.join(file)`. Several books' `.lst` files live nested
(`core_essentials/races/<race>/`, `ultimate_combat/support/`, `horror_adventures/support/`,
`inner_sea_world_guide/_pfs/`, etc.) — the join silently misses and the unit falls to
`ambiguous:no_corpus_line`. Re-derived corpus-wide:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
units = [u for u in d['units'] if u.get('book') != 'beginner_box']
print(len([u for u in units if u.get('wiring_class_reason') == 'no_corpus_line']))
"
# -> 1707
```

**1,707 units** (was documented as 47 in the GE-01 doc, now stale — `retro.py correction` emitted,
`--verified-by` the command above plus a recursive-glob confirmation that all 1,707 are findable).
That is **80.9% of the whole 2,109-unit `ambiguous` population.** Recursive search confirms **100%
(1,707/1,707)** are resolvable — none are genuinely provenance-free. Within this sample, 40 units carry
this reason; I hand-resolved all 40 from their real rows: 19 `display`, 12 `computed`, 4 `derived`, 3
`static`, 2 `ambiguous` (for real reasons, not the bug). Not a blocker to this card (the sample itself
is unaffected — hand-labelled from the real row, not the engine's `ambiguous` verdict) but load-bearing
for Epic 2-F2 (classifier build) and F3 (`ambiguous` dead-end closure): F3's Structural Exclusion
Register review must not run against the current 97%-inflated bucket. Logged
`artifacts/OPEN-ISSUES.md` row 1 (`NOTE`).

**Finding B.** `BONUS:STAT|<ABILITY>|<value>` fields trip the scalar scanner via the STAT *selector*
token (`STR`/`DEX`/.../`CHA` collide with `SCALARS_WORD`) regardless of whether `<value>` is itself
scalar-dependent; `DR:`/`CR:` "value/type" notation (e.g. `DR:10/Cold Iron`, `CR:1/3`) trips
`has_arith`'s unconditional `/` check. 3 units hit this as a clean single-cause misclassification
(`core_rulebook:race_trait:2_dexterity`, `ultimate_equipment:equipment:staff_of_mithral_might`,
`bestiary:monster:neothelid` — all engine=`derived`, true=`static`). Logged `OPEN-ISSUES.md` row 2
(`NOTE`); `retro.py note` emitted.

**Finding C (minor).** `has_scalar`'s substring check is case-sensitive; lowercase PCGen function
calls like `classlevel("Druid")` don't match `"CLASSLEVEL"`. 1 unit
(`ultimate_magic:class_feature:dragon_shaman_totem_transformation`, engine=`static`, true=`derived`).
`retro.py note` emitted.

**Excluding the `no_corpus_line` population** (a single fixable root cause): 110 units, 105 agree
(95.5%), 5 disagree (Findings B, C, and one bare-variable-reference judgement call). Per Decision 1(e)
item 4, this sample does **not** show "substantially correct, contradiction rare" across the whole
board (the `no_corpus_line` bug alone misclassifies 80.9% of `ambiguous`), so **SD31-E2-F2 (classifier
build) is in scope** — but its first task per this evidence should be fixing `CorpusLines::line()`'s
path join before evaluating any new logic against ground truth, or F2's accuracy numbers will measure
the path bug rather than classification quality.

### Judgement calls (methodology note has full detail)

1. Bare cross-referenced non-literal/non-scalar `BONUS:VAR` values (e.g. `...|FavoredBaseBonus`,
   `...|MonkLVL`) hand-labelled `ambiguous` (determination failure) rather than the production
   fallback's `static` — medium confidence, 2 units.
2. A `PRE*` guard scoped to record eligibility rather than to the magnitude itself
   (`core_essentials:race:changeling`'s `PREGENDER:F`) — labelled `computed` for consistency with the
   rest of the guard-bearing sample, medium confidence.
3. A prose ability-score grant the mechanical grant/refer word-list doesn't cleanly match
   (`exciter_rapture`'s "gained"/"bonus to" wording) — labelled `ambiguous` at medium confidence,
   coincidentally class-matching the engine via the unrelated `no_corpus_line` bug.

### Verification run this cycle

No production code changed — docs/artifacts only. Per the card brief, a full `./scripts/verify.sh` is
not warranted; ran the two applicable stages:

```
./scripts/verify.sh --only preflight-oracle   # PASS -- oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6
./scripts/verify.sh --only preflight-disk     # PASS -- 19% used, 791G available
```

`scripts/verify.sh` has no dedicated JSON/doc-lint stage (`ALL_STAGES` is entirely Rust/frontend
build/test plus the two preflight checks above and several `*-selftest` stages that test the gate's own
tooling, not artifact docs) — confirmed by reading `ALL_STAGES=(...)` in `scripts/verify.sh:110`
directly rather than assuming. The JSON file's own validity was checked directly:
`python3 -c "import json; d=json.load(open('...SD31-E2-F1-ground-truth-sample-v1.json')); assert len(d)==150; assert len(d)==len(set(r['id'] for r in d))"` -> passes, 150 unique ids, no schema violations.

### Retro events emitted

- `correction` (`sd31-e2-groundtruth` shard): GE-01 doc's stale 47-unit `no_corpus_line` figure ->
  1,707, `--verified-by` the re-derivation command + recursive-glob confirmation.
- `note` × 2: Findings B and C (classifier-accuracy gaps distinct from Finding A).

### Status: COMPLETE at feature-seed scope (SD31-E2-F1 only)

Per the card brief: *"If you find yourself writing [a classifier], you have exceeded the card."* No
classifier code was written, tuned, or sketched. SD31-E2-F2 (classifier build/accept) and F3
(`ambiguous` dead-end closure) remain open, gated on this F1 deliverable per
`epic-breakdown.md`/`decisions.md` Decision 1(e) — both should read Finding A before starting.

`HEAD` before this receipt: `65decb5e0`. Committed on branch `sd31/e2-groundtruth`, pushed to
`origin/sd31/e2-groundtruth` — **not merged by this cycle**; a later integration cycle merges it onto
`tranche/11` per the workflow's worktree-isolation instructions.
