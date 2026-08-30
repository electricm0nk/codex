# Cycle — Epic 3 Core Rulebook / AT-34-E3-005 (bucket-v-widen, second dispatched lane — independent re-verification)

- **Commit SHA:** `ab65a090efb26e4427125aa49bbc421ce8f0f346` (this lane's own two commits:
  `d2bcf80b1b` progress.md re-verification entry + `completion-atlas.json` restamp,
  `ab65a090ef` retro incident event). The criterion's substantive work — the ledger itself — was
  already landed by `cfd9c6d3d9` (data) and `3cc878de05` (retro/atlas restamp), both already on
  `origin/tranche/14` before this worktree's first rebase.
- **Files touched (this lane):**
  - `docs/release/SD-34-book-completion/progress.md` (prepended entry, independent re-verification)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` restamped to this lane's own HEAD by a `--check` re-run; no bucket count moved)
  - `docs/retro/events/sd34-at-34-e3-005.jsonl` (one `incident` event, `duplicate-dispatch-same-criterion`)
  - This receipt (new)
  - **Zero data/ledger/Rust files touched** — the ledger this criterion is about was already
    committed by a prior lane before this worktree rebased onto `origin/tranche/14`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (own staged diff, `git diff --cached --unified=0`
  at each of this lane's two commits — the wider epic-scoped diff since
  `merge-base HEAD origin/develop` carries many pre-existing `sd27_`/`sd28_`/... matches from
  already-merged prior cycles, none introduced by this lane)
- **Wired-integration audit result:** OK_NO_TOKENS (same scoping as above)
- **Acceptance criterion (verbatim, this dispatch's brief):** "AT-34-E3-005 — rebuild the
  corpus-wide bucket-V ledger AND COMMIT THE DATA FILE... Cross-reference the 6,846 still-open
  bucket-V units corpus-wide against SD-33's own committed `oracle-results.json` files... A
  `disagree` is never dispositioned... Freshness-check a sample and state its size. Prove the
  remainder by SET, not count. Name `PCGEN_ORACLE_SHA` on every corpus-derived figure."
- **Provenance — why this receipt exists instead of a new ledger:** This lane's worktree was cut
  at `ea2b3396f2` (the tranche/14 base) before `cfd9c6d3d9`/`3cc878de05` were pushed. On
  `git fetch origin tranche/14 && git rebase origin/tranche/14` (the first action taken, before
  any write), both commits were already present, tracked, and correct. Rebuilding a second,
  competing ledger from scratch would have either produced byte-different-but-equivalent output
  (wasted work) or risked a collision with the already-landed one. Instead this lane independently
  **re-derived** every headline figure from the already-committed artifacts, treating the landed
  work exactly as a fresh reader must: verify it, don't just cite it.
- **Figures + their re-derive commands (re-run fresh at this lane's own HEAD, all agree with the
  landed receipt's figures):**
  - Corpus-wide widen ledger row count + verdicts: `python3 -c "import json,collections; d=json.load(open('docs/release/SD-34-book-completion/artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"`
    → `6590`, `Counter({'unverifiable': 6164, 'agree': 426})`. Denominator: 6,765 non-`core_rulebook`
    bucket-V units (the ledger's own declared population).
  - `disagree` count in the widen ledger: **0** — same command, key absent entirely.
  - `core_rulebook`'s own sibling ledger row count + verdicts:
    `python3 -c "import json,collections; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/bucket-v-consolidated.oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"`
    → `2712`, `Counter({'unverifiable': 2327, 'agree': 385})`, 0 `disagree`. Denominator: 2,793
    `core_rulebook` bucket-V units.
  - Disjointness of the two ledgers' `unit_id` sets:
    `python3 -c "import json; a=set(r['unit_id'] for r in json.load(open('docs/release/SD-34-book-completion/artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json'))['results']); b=set(r['unit_id'] for r in json.load(open('docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/bucket-v-consolidated.oracle-results.json'))['results']); print(len(a),len(b),len(a&b))"`
    → `6590 2712 0`. Denominator: the union of both ledgers' unit ids.
  - Corpus-wide bucket state: `python3 scripts/completion_atlas.py --check` →
    `population=49438 unclassified=0 overlap=0`, `V: 256`, `DONE: 24166`. Denominator: 49,438
    corpus-wide units.
  - `core_rulebook` bucket state: `python3 scripts/completion_atlas.py --book core_rulebook --check`
    → `population=6701 unclassified=0 overlap=0`, `V: 81`, `DONE: 4254`. Denominator: 6,701
    `core_rulebook` units (unchanged by this criterion's bucket-V sub-scope — confirms
    `core_rulebook`'s own 81-unit V population is untouched by the corpus-wide widen).
  - Independent freshness re-sample, **different seed (42) and larger n (50)** than the landed
    receipt's own (seed 20260830, n=30):
    `python3 -c "import json,random; inv=json.load(open('docs/work-inventory.json')); m={u['id']:u for u in inv['units']}; led=json.load(open('docs/release/SD-34-book-completion/artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json'))['results']; random.seed(42); s=random.sample(led,50); bad=[r['unit_id'] for r in s if m.get(r['unit_id'],{}).get('status') not in ('oracle-agree','oracle-unverifiable')]; print('checked',len(s),'bad',len(bad))"`
    → `checked 50 bad 0`. Denominator: 6,590-row ledger, this lane's own 50-row sample (0.76%).
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
    `files_checked=15 violations=6` — unchanged from the pre-existing `FRT_HVY`-quote baseline
    (the landed receipt itself records `violations=6` post-cycle); this lane's own two docs-only
    commits added no new bare-percentage prose.
  - `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — `grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env`.
    Named because every reused verdict traces back to a `.pcg`/oracle-export round-trip SD-33 ran
    against this pin; **zero new oracle runs performed this lane** (pure re-verification).
- **Row-count command output (this lane's own artifact — the re-verified ledger, unchanged):**
  ```
  $ python3 -c "
  import json
  d=json.load(open('docs/release/SD-34-book-completion/artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json'))
  print('rows:', len(d['results']))
  "
  rows: 6590
  ```
- **Build scope verified:** `cargo test --locked --no-run` (full workspace,
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-005`) → **exit 0**, run at this lane's own HEAD
  `ab65a090efb26e4427125aa49bbc421ce8f0f346` (no Rust source changed by this lane or the landed
  ledger cycle, so the compile check is valid at the current tip). `cargo test --locked --no-run
  --manifest-path apps/desktop/src-tauri/Cargo.toml` (separate cargo workspace, tested explicitly,
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-005-desktop`, `3m48s`) → **exit 0**.
- **Sweep population:** N/A — this lane added or changed zero `data/corpus/**` records
  (`git status --porcelain data/corpus` empty throughout this lane's work). The landed ledger
  cycle's own receipt already recorded `corpus_literal_sweep` unchanged at `records_examined:48708`.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (see Figures above;
  zero new oracle runs this lane).
- **Status:** complete
- **Movement, four buckets:** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — this lane moved **zero** units between buckets. The 6,590-unit
  V→DONE reclassification this criterion is about was performed by the already-landed
  `cfd9c6d3d9` cycle; this lane only re-verified it independently.
- **Notes:** This dispatch handed this lane the identical criterion text as a prior lane that had
  already landed and pushed the work (`cfd9c6d3d9`/`3cc878de05`) before this worktree's first
  `git fetch`/`rebase`. Per `git status --porcelain` (clean before and after every write this
  lane made) and the figures above, the criterion is genuinely satisfied at
  `origin/tranche/14`'s current tip. This lane did not rebuild the ledger — doing so would have
  been wasted, redundant work risking a spurious diff against a correct, already-committed
  artifact — and instead spent its cycle on adversarial-style independent re-derivation (fresh
  commands, a different freshness-sample seed/size, an explicit ledger-disjointness check the
  landed receipt asserted but did not show the command for) to confirm the criterion holds rather
  than merely trusting the landed receipt's own prose. A retro `incident` event
  (`duplicate-dispatch-same-criterion`) was filed so the dispatch-orchestration layer has a
  record of this collision if it recurs.
- **Next-cycle plan:** Unchanged from the landed cycle's own plan — the 175-unit corpus-wide
  remainder and `core_rulebook`'s own 81-unit remainder both need real oracle-harness
  (`.pcg`/`.ftl` round-trip) work, scoped together per the landed receipt's own recommendation.
  The whole-book `AT-34-E3-005` gate (`docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-005_cycle_receipt.md`,
  Cycle 1) remains gated on `AT-34-E3-001`/`002`/`003` reaching zero in their own buckets for
  `core_rulebook` — untouched by this lane or the bucket-V widen work, both of which are scoped
  to bucket V only.
