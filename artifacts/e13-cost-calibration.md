# SD28-E13 Cost Calibration Receipt

**Card:** `epic-13-calibration` (SD28-E13-F1-001 / SD28-E13-F2-001)
**Book:** `ultimate_campaign` — 23 units, all `kind:feat` (Story Feats), the smallest book in the corpus by an order of magnitude.
**Actor:** `epic-13-calibration`
**Cycle date:** 2026-08-03

## Result

`ultimate_campaign`: `proven` 0 → **23 of 23**, `not-started` 23 → 0. Every unit is `text-complete` or `deferred-with-reason` carrying the engine's verbatim diagnostic. Zero `unknown`, zero `not-ingested`, zero `not-started`.

**Honest split: 20 text-complete + 3 deferred-with-reason = 23 accounted** — not the cycle brief's own stated target of 22+1. Re-derivation (a 10-word shingle comparison across all 23 `BENEFIT:` rows, `python3` script) found two more corpus splices beyond the one the brief named (`Fearless Zeal`): `Magnum Opus` and `Stronghold` share an identical, misattributed trailing sentence, and `Magnum Opus`'s own sentence is separately truncated. Recorded as a correction in `decisions.md` Decision 33 (`--verified-by` the shingle-comparison script) rather than silently absorbed. This finding **narrows** the delivered scope relative to the brief, not widens it — recorded per the anti-gaming instruction (`decisions.md §32`) as evidence the honest number moved because more corruption was found, not because the classifier was relaxed.

```
python3 -c "
import json
b=[x for x in json.load(open('docs/work-inventory.json'))['books'] if x['id']=='ultimate_campaign'][0]
print(b['id'], b['kinds'])
"
# -> ultimate_campaign {'feat': {'units': 23, 'by_status': {'deferred-with-reason': 3, 'text-complete': 20}}}
```

## By status bucket: units, evidence

| Status | Units | Evidence |
|---|---|---|
| `text-complete` | 20 | Real corpus `.MOD BENEFIT:` mechanical text, joined with the flavor `DESC:` text, reaches `list_feat_catalog` (`feats_all::map_uca_entry`, `reach_gate::feats_reach`) |
| `deferred-with-reason` | 3 | `Fearless Zeal`, `Magnum Opus`, `Stronghold` — confirmed upstream corpus splices, engine emits a verbatim file:line diagnostic (`ultimate_campaign::feat_tables::DEFERRED_WITH_REASON`), consumed by both `v06_work_inventory` status classification and the player-facing joined description |
| `grounded` | 0 | None of UCA's 23 records carry a `BONUS:`-family token the `feat_effect` probe can observe a computed delta from — every mechanical effect here is narrative/GM-adjudicated prose (temp HP, saving-throw rerolls tied to story completion, etc.), not an automatable numeric bonus. This is a real property of this book's content, not a shortfall of this cycle's work. |

## Measured cost, wall-clock and cycles

**Cycles consumed: 1** (a single continuous claim-to-commit cycle; the card was claimed once, per `kanban.md`'s IN-FLIGHT protocol, and closes in this same cycle).

**Wall-clock: not instrumented precisely.** This cycle ran as one continuous interactive session with no external wall-clock timer captured at claim time; the honest statement is that it is a single-session, single-sitting cost, on the order of the elapsed time between the previous branch-tip commit (`fc5f1fab`, 2026-08-03T17:55:56-04:00) and this cycle's own commit timestamp (below), which is the best available proxy — not a precise measurement. **Future calibration receipts should record `date -u` at kanban claim time and at commit time explicitly**, which this cycle's own kanban edit did not do (recorded as a process gap, not smoothed over).

**What the wall-clock actually contained**, as a proxy for where cost went:

1. Diagnosis verification (root-cause confirmation, corpus re-derivation, splice detection) — approximately half the cycle's total effort.
2. Rust implementation: 1 new module (`feat_tables.rs`, ~380 lines incl. doc comments/tests), 1 new `mod.rs`, 8 wiring edits across `mod.rs`/`v06_work_inventory.rs`/`feats_all.rs`/`v06_content_state_dump.rs`/`feat_identity.rs`/`feat_prereqs.rs`/`corpus_ingest_diagnostic.rs`/`reach_gate.rs`.
3. Verification: targeted `cargo test` passes, `v06_work_inventory` regeneration + idempotency check, `v06_corpus_trap_report --audit`, reach-gate suite, four-check wired-integration audit, full `verify.sh`.
4. Documentation: 2 `decisions.md` entries' worth of content (one dated ruling), this receipt, `progress.md` receipt, kanban claim/close.

## Fixed vs. variable cost — the point of this receipt

**Per-book FIXED cost (paid once, does not repeat per unit):**

- New `RuleSetId::Uca` variant + exhaustive-match updates at **6 separate call sites** across 5 files (`mod.rs`, `v06_work_inventory.rs` ×2 arms, `v06_content_state_dump.rs`, `feats_all.rs` join wiring, `corpus_ingest_diagnostic.rs`, `reach_gate.rs` ×2: `RECORD_TYPE_KINDS` registration + `reach_of` claim). **This is the single largest cost driver for a small book**: a 23-unit book pays the exact same number of wiring sites as a 2,854-unit book would.
- New `rules_tables/<book>/` module + `mod.rs` scaffold (required before a single unit can be classified anything but `not-started`).
- New per-book record type (`StoryFeatEntry`, distinct from `crb::feats::FeatTableEntry` because this book's corpus shape — `PRETEXT:` prose prerequisites, `.MOD BENEFIT:` split from `DESC:` — does not fit the shared type, exactly the same reason ARG and PU each needed their own types).
- Two catalog-wide test-count updates that had nothing to do with UCA's content and everything to do with UCA existing at all: `feat_identity.rs` and `feat_prereqs.rs` both hardcode the full catalog's total record count (690 → 713) and a full-catalog eligibility count (211 → 234) that had to be **re-derived by actually running the evaluator**, not computed by hand, because `PRETEXT:`-only records interact with the prerequisite engine in a way that is only knowable by executing it.
- Desktop `every_book_landed_in_rules_tables_is_reported` diagnostic registration (drift guard that fails hard on a landed-but-unreported book).
- The corpus splice re-derivation (the shingle-comparison script) — this is genuinely re-derivation work that scales with the number of `BENEFIT:`-bearing records in the *book*, not with the total unit count, so it is closer to fixed-per-book than variable-per-unit for a book this small.

**PER UNIT cost (scales with the 23 records, and would scale roughly linearly with a larger book's record count):**

- Corpus field extraction (`DESC:`/`PRETEXT:`/`SOURCEPAGE:`/`BENEFIT:` per record) — mechanical once the extraction script exists.
- Rust literal authorship for each of the 23 `StoryFeatEntry` values.
- The 23-row `UCA_FEAT_PREREQUISITES` gather table.
- Per-record splice/defect triage (only 3 of 23 needed it here).

**Why this matters for extrapolation.** A 23-unit book's fixed cost (6+ wiring sites, a new module, a new type, two catalog-wide re-derivations, a diagnostic registration) is amortized over only 23 units — an unusually bad fixed-to-variable ratio. A 2,854-unit book (e.g. Ultimate Combat) pays the **same fixed cost once** and then 2,854× the per-unit cost, which is dramatically cheaper per unit than this book's blended rate would suggest. **Do not extrapolate this cycle's blended per-unit time to a large book without dividing out the fixed component first** — see `decisions.md §32`'s own caution against exactly this kind of blended-average error, restated here as this receipt's central finding.

**Separate cost-per-unit, not blended (qualitative, since wall-clock was not instrumented per-record):** the marginal cost of `text-complete` records (20 of them) was materially lower than `deferred-with-reason` records (3 of them) — a `text-complete` unit is one field-extraction + one literal; a `deferred-with-reason` unit additionally required corruption detection, byte-for-byte cross-record comparison, a decisions.md ruling, and a diagnostic wired through 2 additional call sites (`v06_work_inventory.rs`'s new per-feat diagnostic lookup — itself a small fixed addition, since `Kind::Feat` had no per-feat diagnostic path before this cycle — and `feats_all::map_uca_entry`'s joined-description branch). No `grounded` units exist in this book to measure against.

## Structural-unreachability finding

Per `decisions.md §32`'s own realism framing: this cycle's fixed-cost enumeration above (6+ wiring call sites touched for a single small book) suggests the 29,161-unit gap's per-book fixed overhead, summed across ~13-19 remaining books, is itself a nontrivial and currently-unmeasured cost the program has not yet budgeted separately from per-unit costs. This is flagged in `risks-and-open-questions.md` rather than smoothed over — **this receipt is an input to that estimate, not a verdict** that the full directive is or is not reachable.

## Commands behind every published figure

```sh
# 23-unit count, re-derived directly against the corpus (not transcribed):
grep -c '^[A-Za-z]' <(grep -vE '^#|^CATEGORY=FEAT\|' \
  ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign/uca_feats.lst)
# -> 23

# Splice detection (10-word shingle comparison):
python3 - <<'PY'
import json, re
d = json.load(open('/tmp/.../uca_feats.json'))
ngrams = {}
for e in d:
    if not e['benefit']: continue
    words = re.findall(r"\S+", e['benefit'])
    for i in range(len(words)-9):
        gram = " ".join(words[i:i+10])
        ngrams.setdefault(gram, []).append(e['name'])
for gram, names in ngrams.items():
    if len(set(names)) > 1:
        print(set(names), "|", gram)
PY
# -> {'Champion','Town Tamer'} (benign, generic phrase)
# -> {'Damned','Fearless Zeal'} (confirmed splice, ~80 overlapping 10-grams)
# -> {'Stronghold','Magnum Opus'} (confirmed splice, 19 overlapping 10-grams)

cargo run --locked --bin v06_work_inventory   # -> ultimate_campaign: 23 units, deferred-with-reason: 3, text-complete: 20
cargo run --locked --bin v06_corpus_trap_report -- --audit   # -> exit 0
cd apps/desktop/src-tauri && cargo test --locked -- reach_gate   # -> 16 passed (including the new ultimate_campaign/feats claim)
```
