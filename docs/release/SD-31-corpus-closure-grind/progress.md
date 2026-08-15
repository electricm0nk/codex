# SD-31 Progress Log

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
