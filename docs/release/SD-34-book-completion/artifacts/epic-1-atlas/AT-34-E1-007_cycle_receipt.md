# Cycle 7 — Epic 1 Completion Atlas / AT-34-E1-007

- **Commit SHA:** `<filled at commit — see progress.md and kanban.md commit trailer>`
- **Files touched:** `scripts/verify.sh` (new `corpus-trap-audit` stage, own timeout wrapper, independent population count, wired into `ALL_STAGES` only — FULL scope, matching `corpus-sweep`'s own placement), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-007_cycle_receipt.md` (this file), `docs/release/SD-34-book-completion/progress.md` (updated, `## Open blockers` entry added), `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** "`v06_corpus_trap_report --audit` is a real `verify.sh` stage. Inherited unclosed through SD-31, SD-32 and SD-33 (each register carried it as C1.8). It is not carried a fourth time: Epic 1 already opens the `verify.sh` stage list (AT-34-E1-006), and the wiring pattern is established. **Evidence:** the stage listed in `verify.sh`'s `ALL_STAGES`; `scripts/verify.sh --only corpus-trap-audit` exits 0 and prints the population it examined; RED→GREEN by planting one trap the audit must catch, confirming the catch, removing the probe, and confirming the baseline returns to zero. The stage's own timeout wrapper is part of the deliverable — SD-33's register D1.2 records a sibling stage that could not bound its own runtime." (`epic-breakdown.md`, verbatim)

## Design note (read before the figures) — the mechanical wiring is done; the criterion is blocked on real content, not on the wiring

`v06_corpus_trap_report --audit` and its `--json` mode already existed (no `src/bin/`,
`src/pcgen_import/` change was needed or made — Epic 1's file-touch table names only
`scripts/verify.sh` for this criterion). The new `run_corpus_trap_audit` stage:

- Is placed in `ALL_STAGES` only (FULL scope), directly after `corpus-sweep`, same dependency
  (the real PCGen corpus via `PCGEN_CORPUS_ROOT`) and the same "fail loudly, never skip on an
  absent corpus" posture `corpus-sweep`'s own comment states.
- Wraps the invocation in `timeout "${CORPUS_TRAP_AUDIT_TIMEOUT_S:-300}s"` — the stage's own
  runtime bound, closing the exact gap `forward-scope-register.md D1.2` names (a sibling stage
  hung twice with no wrapper in either `verify.sh` or the script it called).
- Computes its **population independently of the binary's own output**: a `find -mindepth 3
  -maxdepth 3 -name '*.json'` count under `data/corpus/`, mirroring `audit_ingested_cache`'s own
  book/kind/record walk exactly. `27,638` (re-derive command below) — the same reasoning
  `corpus-sweep`'s own independent `examined`/`tokens` parse already uses, and the mechanism
  `workflow-instruction.md §12` row 15 requires ("a vacuous pass is not a pass").
- Parses `--json`'s `findings[].severity` tally via `python3` (already a dependency of this
  file, same pattern `denominator-gate`/`figure-provenance` use) into `defects=` / `traps=`,
  distinct exit-code handling for timeout (124), usage/IO error (1), and defect-bearing (2).

**Live run against the real corpus at HEAD is FAIL, not PASS** — this is the finding, not a
wiring defect. `scripts/verify.sh --only corpus-trap-audit` reports
`records_examined=27638 defects=10196 traps=407 (exit 2)`. Of those 10,196 `Severity::Defect`
findings:

- **3,181 are already-known, already out-of-scope inherited debt**, matching four tests in
  `tests/v06_corpus_trap_report.rs` (`no_ingested_record_is_sourced_from_a_disabled_line`,
  `no_two_ingested_records_share_a_record_key`,
  `every_mod_sourced_ingest_has_a_live_base_declaration`,
  `ingested_record_keys_match_their_cited_line`) that are **already** part of the
  `v06_corpus_trap_report` integration-test target SD-33's `forward-scope-register.md D1.1`
  verified as one of the 29 pre-existing failing suites at the `tranche/13` cut, ruled "genuinely
  outside SD-33's Definition of Done... a future SD-N's own suite-green epic. Not this bundle's."
  (165 `disabled-line` + 249 `shared-name-distinct-records` + 2,117 `mod-record` (defect subset)
  + 650 `key-differs-from-name` = 3,181.)
- **7,015 are `wiring-class-mismatch` — a brand-new discovery, not tracked anywhere before this
  cycle.** `git log -S WiringClassMismatch -- src/pcgen_import/corpus_traps.rs` shows this exact
  check was last driven to `0` by `b32926f2af` (`SD30-CARRY-001`, 2026-08-14: "Audit now exits
  0... 177 wiring-class-mismatch defects across 10 books... re-ran the canonical generator"). It
  has silently regressed to 7,015 defects across 34 of 37 books since, because nothing has run
  `--audit` in `verify.sh` between then and now — precisely the C1.8 gap this criterion exists to
  close, and precisely the failure mode `AGENTS.md` rule 8 names ("a warning is not a control").
  Retro event: `docs/retro/events/sd34-at-34-e1-007.jsonl` (`incident`,
  recurrence-key `unwired-standing-gate-decay`).

**Fixing either bucket is outside this criterion's write scope.** Epic 1's file-touch table
(`workflow-instruction.md §3`) lists no `data/corpus/**` path for Epic 1 at all — regeneration is
gated to Epic 3 (Core Rulebook, 798 of the 7,015 wiring-class-mismatch defects; 807 of 10,196
counting all four defect traps for that book) and Epic 4 (Ultimate Campaign, 152 of the 7,015;
163 of 10,196 all-trap) later in this same bundle, and even those two epics only cover 970 of
10,196 all-trap defects (9.5%). The
other 9,226 defects, across 32 books with no assigned epic in this bundle's plan, have no
sanctioned fix mechanism today. SD30-CARRY-001's own precedent (re-run `gen_book_cache` per
affected book, diff license/PI/`raw_tokens` survival, re-audit to confirm) scales to roughly 3.4×
its own 10-book/177-defect scope here — real, bounded, but a multi-cycle remediation wave in its
own right, not a one-cycle fix folded into a `verify.sh` wiring criterion.

## RED→GREEN — plant one real trap, confirm the catch, remove it, confirm the baseline returns

Performed against the live corpus with a single reversible mutation (never committed), the same
technique `AT-33-E1-004`'s denominator-gate receipt used to prove a stage fails without
permanently committing a violation:

```
$ git status --porcelain -- data/corpus/pathfinder_unchained/ability/sympathetic_rage.json
(clean)

# baseline (RED — real, pre-existing content, not the planted trap):
$ bash scripts/verify.sh --only corpus-trap-audit
FAIL  corpus-trap-audit  (records_examined=27638 defects=10196 traps=407 (exit 2) — ...)

# plant: flip this record's wiring_class to a value that disagrees with its fresh
# recomputation (was `display`, no existing finding; set to `computed`)
$ python3 -c "import json,p='...sympathetic_rage.json'; d=json.load(open(p)); d['wiring_class']='computed'; json.dump(d, open(p,'w'), indent=2)"

# catch — the stage's own +1, on exactly the planted record:
$ bash scripts/verify.sh --only corpus-trap-audit
FAIL  corpus-trap-audit  (records_examined=27638 defects=10197 traps=407 (exit 2) — ...)
$ grep -c sympathetic_rage <log>   # 1

# remove the probe:
$ git checkout -- data/corpus/pathfinder_unchained/ability/sympathetic_rage.json
$ diff -q <file> <pre-mutation backup>   # IDENTICAL

# confirm the baseline returns — to the STARTING baseline, not to zero (see Design
# note: the corpus itself carries 10,196 real, pre-existing defects):
$ bash scripts/verify.sh --only corpus-trap-audit
FAIL  corpus-trap-audit  (records_examined=27638 defects=10196 traps=407 (exit 2) — ...)
```

`records_examined` and `traps` are unchanged across all three runs (27638 / 407); `defects` moves
exactly `10196 → 10197 → 10196`, proving the stage's catch mechanism is correct: it detects
precisely the injected trap and nothing else, and cleanly returns to the true baseline on
removal. The **PASS** branch of the new bash function was verified separately, since the live
corpus cannot be made clean within this criterion's write scope: fed a synthetic `{"findings":
[]}` payload through the identical parse snippet, confirmed `defects=0 traps=0` — the shape that
drives `stage_pass` in the real function (exit 0 branch untaken here only because the real
corpus is not clean, not because the parse/exit-code logic is unexercised).

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Population examined | `27,638` | every `data/corpus/<book>/<kind>/*.json` (3-level walk, matching `audit_ingested_cache`) | `find data/corpus -mindepth 3 -maxdepth 3 -type f -name '*.json' \| wc -l` |
| Total findings | `10,603` | same population | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; print(len(json.load(sys.stdin)['findings']))"` |
| Defect findings | `10,196` | of `10,603` total findings | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(sum(1 for f in d if f['severity']=='DEFECT'))"` |
| Trap findings | `407` | of `10,603` total findings | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(sum(1 for f in d if f['severity']=='TRAP'))"` |
| Already-known inherited debt (4 traps) | `3,181` | of `10,196` defects (`165+249+2117+650`) | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(sum(1 for f in d if f['severity']=='DEFECT' and f['trap']!='wiring-class-mismatch'))"` |
| New discovery — `wiring-class-mismatch` | `7,015` | of `10,196` defects | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(sum(1 for f in d if f['trap']=='wiring-class-mismatch'))"` |
| Books carrying at least one defect | `34` | of `37` ingested books | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(len({f['file'].split('/data/corpus/')[1].split('/')[0] for f in d if f['severity']=='DEFECT'}))"` |
| Core Rulebook's `wiring-class-mismatch` share | `798` | of `7,015` wiring-class-mismatch defects (Epic 3's territory; `807` is Core Rulebook's all-trap defect total, of `10,196`) | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(sum(1 for f in d if f['trap']=='wiring-class-mismatch' and '/core_rulebook/' in f['file']))"` |
| Ultimate Campaign's `wiring-class-mismatch` share | `152` | of `7,015` wiring-class-mismatch defects (Epic 4's territory; `163` is Ultimate Campaign's all-trap defect total, of `10,196`) | `cargo run --locked --bin v06_corpus_trap_report -- --audit --json \| tail -1 \| python3 -c "import json,sys; d=json.load(sys.stdin)['findings']; print(sum(1 for f in d if f['trap']=='wiring-class-mismatch' and '/ultimate_campaign/' in f['file']))"` |
| Last time this check was 0 | `b32926f2af`, 2026-08-14 (`SD30-CARRY-001`) | N/A — commit reference | `git log -S WiringClassMismatch --oneline -- src/pcgen_import/corpus_traps.rs` |

## Row-count command output

```
$ bash scripts/verify.sh --only corpus-trap-audit
==> corpus-trap-audit — timeout 300s cargo run --locked --bin v06_corpus_trap_report -- --audit --json
    FAIL  corpus-trap-audit  (records_examined=27638 defects=10196 traps=407 (exit 2) — /tmp/codex-verify-.../corpus-trap-audit.log)
SUMMARY
  passed:  0
  FAILED:  1  corpus-trap-audit
RESULT: FAIL
```

The stage exists, is listed in `ALL_STAGES`, runs, bounds its own runtime, and prints its
population — every mechanical piece of the evidence bar except "`exits 0`", which depends on
real corpus content this criterion has no write scope to fix.

## Build scope verified

`cargo test --locked --no-run` exit 0, full workspace, run at commit
`782b146a3abb15c2ad58268079d9a3192c67a846` + this cycle's diff (`scripts/verify.sh` only, no Rust
source touched). `cd apps/desktop/src-tauri && cargo test --locked --no-run` exit 0, separate
`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-007-desktop`, same commit + diff. Both re-run after
the plant/revert mutation was fully reverted (`git status --porcelain -- data/corpus/` clean)
and after the last commit-affecting edit in this cycle, per `decisions.md §12` L7.

## Sweep population

N/A for the committed diff — the plant/revert mutation to
`data/corpus/pathfinder_unchained/ability/sympathetic_rage.json` was reverted via `git checkout
--` before commit and never lands; `git status --porcelain -- data/corpus/` is clean at commit
time. `docs/work-inventory.json` is untouched.

## Oracle pin

N/A — no figure in this receipt is drawn from the pinned oracle checkout; `--audit` reads the
operator's live `PCGEN_CORPUS_ROOT` clone directly (same convention `corpus-sweep` already uses),
not `$PCGEN_REPO_DIR`.

- **Status:** blocked-escalated
- **Movement, four buckets:** reachability — a previously-unreachable check (C1.8, unclosed
  through three prior bundles) is now wired, run, and its true population and defect count are
  visible for the first time since `SD30-CARRY-001`. No unit's bucket in `docs/work-inventory.json`
  moves this cycle (`docs/work-inventory.json` untouched, confirmed by `git status --porcelain`).
- **Notes:**
  - The mechanical deliverable (the stage exists, is listed in `ALL_STAGES`, bounds its own
    runtime, prints its population, and its catch/revert mechanism is proven correct by a live
    plant-and-remove) is complete and correct. The criterion's literal "exits 0" bar is not met,
    for a reason outside this cycle's authority to fix: `data/corpus/**` is not in Epic 1's
    file-touch table, and the true fix (re-running `gen_book_cache`-class regeneration across up
    to 34 books, verifying license/PI/`raw_tokens` survival by diff for each, per
    `SD30-CARRY-001`'s own precedent scaled ~3.4×) is a multi-cycle remediation wave, not
    something to fold into or defer around this criterion.
  - This is **not** a narrowed gate: the stage runs the full, real `--audit` against the full,
    real corpus, with no scope restriction added to make it pass. Reporting the true FAIL is the
    correct outcome per `acceptance-and-verification.md §5` ("do not manufacture a shortfall
    either... if the work is genuinely done, PASS it" — inverted here: the work genuinely is not
    clean, so it is not called clean).
  - `## Open blockers` entry filed in `progress.md`, naming the exact reproduction command,
    figures, and precedent. This pauses the bundle per `../../governance/blocker-closure-doctrine.md`
    until the operator rules on write-scope for the corpus fix (a dedicated remediation wave,
    folded into Epic 3/4 where the book overlaps, or a new epic for the remaining 32 books).
- **Next-cycle plan:** Awaiting operator ruling on the `## Open blockers` entry. Once cleared
  (either by authorizing a corpus-regeneration wave, or by a ruling that the criterion's
  acceptance bar is satisfied by "the stage exists, runs, and correctly reports the true state"
  without requiring the live corpus to be clean), Epic 1 closes and Epic 2 (build 8 of 9 tables)
  opens. Until then, per `workflow-instruction.md §11` step 1, no later epic proceeds past this
  blocked card.
