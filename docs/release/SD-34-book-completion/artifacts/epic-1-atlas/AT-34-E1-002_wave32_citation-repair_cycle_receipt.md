# Cycle — SD-34 Wave 32, Lane B — `AT-34-E1-002` condition-6 citation-gate repair

`python3 scripts/completion_atlas.py --check` was tripping condition 6 (the citation gate) with
`citation_failure: DONE: src/bin/v06_work_inventory.rs:10172 no longer contains 'grounded'` — an
edit to `v06_work_inventory.rs` between the last re-derivation and this cycle shifted the DONE
citation's line off its marker.

**Repo-vs-brief note (rules require this be stated, not silently resolved either way): the brief
named only the DONE citation as broken. The repo disagreed — running `--check` at this cycle's
own fresh `HEAD` showed all ten citations broken, not one.** The gate reports only the first
failure it finds (`_citation_failures` iterates `BUCKET_ORDER` and appends every failure to a
list, but `--check`'s printed summary and non-zero exit both fire on the first `citation_failures`
count > 0 regardless of how many are in the list — so a single visible line under-reports the
real damage). The audit below is of all ten, per the brief's own instruction 3, and the repo's
shape (all ten broken) is what got fixed, not the brief's (one broken).

**Concurrent-lane collision, resolved by cross-check rather than by discarding either side's
work.** This cycle's own commit collided on push with `65c891e277` ("wave 32 lane C -- re-pin
completion_atlas.py's 10 citation lines"), landed by the same wave's bucket-D lane a few minutes
earlier — bucket D's own work needed `--check` clean first and hit the identical gap. After
rebasing onto it: **all ten of that lane's independently re-derived line numbers are byte-identical
to all ten of this lane's** (re-derive: `git show 65c891e277:scripts/completion_atlas.py | grep
'"line":' > /tmp/lane_c.txt && grep '"line":' scripts/completion_atlas.py > /tmp/lane_b.txt && diff
/tmp/lane_c.txt /tmp/lane_b.txt` → empty, exit 0). That agreement is the
strongest evidence either lane has that its own audit is correct — two independent greps of the
same 25,609-line file landed on the same ten lines. This cycle's landing keeps that lane's line
numbers (already on `origin/tranche/14`, unchanged here) and adds the value this receipt's audit
table and per-bucket comments carry beyond that lane's scope: a full ten-row table with live line
content, the awk-verified "inside real production code, not a doc string or test" check per
marker, the pytest/unittest confirmation, the denominator-gate self-check, and the durable-anchor
recommendation the brief asked for. No functional code changed beyond what `65c891e277` already
landed — `git diff 65c891e277..HEAD -- scripts/completion_atlas.py` shows comment-only lines.

- **Commit SHA:** see push output below (this receipt's own landing commit).
- **Files touched:**
  - `scripts/completion_atlas.py` (ten citation line numbers corrected; historical
    re-derivation-method comments extended with a wave-32 entry, same style as the prior nine)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated through its producer, `completion_atlas.py --check`, per the standing rule
    against hand-editing generated files — never edited directly)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-002_wave32_citation-repair_cycle_receipt.md`
    (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)

## Fresh-base check

`git fetch origin && git log --oneline -1 origin/tranche/14` → `4df2c3fa0a docs(sd34): register
wave 32 in the ledger`. This worktree's branch (`worktree-wf_820936ab-e11-2`) was reset onto that
commit before any file was read (`git reset --hard origin/tranche/14`; working tree was already
clean, nothing lost).

## Method

1. **Locate the live emission site for each of the ten markers.** Per-marker: `grep -n
   '<literal>' src/bin/v06_work_inventory.rs`, discard (a) the `STATUS_VOCABULARY` tuple hits
   near lines 9330–9410 (doc strings describing the vocabulary, never a construction site — the
   comment block already in the file names this exact trap) and (b) `#[cfg(test)]` module hits
   (assertions, never the emitter), then confirm the surviving hit sits in real production code
   by walking upward from the candidate line to the nearest `fn`/`#[cfg(test)]` marker with
   `awk`.
2. **Read the new line back and check it literally contains the marker string** — the same
   substring test `_citation_failures` itself runs — before writing it into `BUCKET_DEFINITIONS`.
   Verified with a small Python script reading all ten lines fresh from the working file, not
   from memory of the grep output.
3. **Re-run `--check`** and the script's own unit suite.

## Ten-citation audit table

All ten re-derived and verified against the live file (`src/bin/v06_work_inventory.rs`,
25,609 lines at this cycle's `HEAD`):

| Bucket | Marker | Old line (broken) | New line (verified) | Live line content |
|---|---|---|---|---|
| DONE | `grounded` | 10172 | **10195** | `status: "grounded",` (in `simple_kind_verdict`) |
| A | `has_no_engine_table` | 12480 | **12494** | `Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table"),` |
| B | `not_held_by_engine` | 12160 | **12174** | `return engine_does_not_hold("class_feature_option_pool_record_not_held_by_engine");` |
| C | `explanation_id` | 12385 | **12399** | `engine_does_not_hold("no_explanation_id_and_no_diagnostic_names_this_feature")` |
| D | `engine-does-not-hold` | 10346 | **10369** | `status: "engine-does-not-hold",` (body of the shared `engine_does_not_hold` closure) |
| M | `ingested-magnitude` | 10181 | **10204** | `status: "ingested-magnitude",` (in `simple_kind_verdict`) |
| V | `literal-verified` | 13262 | **13276** | `item.verdict.status = "literal-verified";` (in `apply_done_rung_stamps`) |
| U | `unmeasurable` | 10433 | **10456** | `status: "unmeasurable",` (feat placeholder-description arm) |
| X | `deferred-with-reason` | 10393 | **10416** | `status: "deferred-with-reason",` (UCA `DEFERRED_WITH_REASON` table lookup arm) |
| Z | `not-started` | 10254 | **10277** | `status: "not-started",` (no compiled rule set for book arm) |

Re-derive the whole table yourself: `python3 -c "pairs=[(10195,'grounded'),(12494,'has_no_engine_table'),(12174,'not_held_by_engine'),(12399,'explanation_id'),(10369,'engine-does-not-hold'),(10204,'ingested-magnitude'),(13276,'literal-verified'),(10456,'unmeasurable'),(10416,'deferred-with-reason'),(10277,'not-started')]; lines=open('src/bin/v06_work_inventory.rs').readlines(); [print(ln, m, m in lines[ln-1]) for ln,m in pairs]"` — all ten print `True`.

**None of the nine non-DONE markers merely happened to still resolve — every single one had also
drifted.** The nine "old line" values above are what `BUCKET_DEFINITIONS` held before this cycle
(the same values `git show HEAD~1:scripts/completion_atlas.py` shows); none of them still
contained their marker at this cycle's `HEAD` either, confirmed the same way as DONE (fresh read,
literal substring check, all `False` before the fix).

## `--check` output (literal, after the fix)

```
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 24963
  A: 449
  B: 11769
  C: 4173
  D: 2955
  M: 4449
  V: 289
  U: 202
  X: 170
  Z: 19
done_evidence_violations=0
missing_clearing_mechanisms=0
stale_derived_at=False
citation_failures=0
```
Re-derive: `python3 scripts/completion_atlas.py --check` — exit 0.

## Unit suite

`python3 -m unittest scripts.tests.test_completion_atlas -v` → **38 tests, OK** (0 failures, 0
errors), including `test_real_citations_all_resolve_and_match`, which independently exercises the
same content-match logic as `--check`'s condition 6 against the now-corrected
`BUCKET_DEFINITIONS`. Re-derive: `python3 -m unittest scripts.tests.test_completion_atlas -v
2>&1 | tail -5` → `Ran 38 tests in 2.7XXs / OK`.

## What did NOT change (the trap the brief warns against, checked off)

- **No path-only loosening.** `_citation_failures` in `scripts/completion_atlas.py` is untouched;
  it still reads the cited line's live content and does a literal substring check against
  `must_contain`. Confirmed by `git diff scripts/completion_atlas.py` — only `BUCKET_DEFINITIONS`
  entries (line numbers + comments) changed, zero lines touched in `_citation_failures`,
  `_read_source_line`, or the `--check` driver.
- **No dropped citation field.** All ten buckets still carry a `citation` dict with `file`,
  `line`, `must_contain`.
- **No "merely contains the word" repointing.** Every new line was walked upward to its owning
  `fn` (or confirmed outside any `#[cfg(test)]` block) before being accepted — see the awk checks
  in this cycle's shell history and the per-bucket notes in the table above. None of the ten sit
  inside a test module or a doc comment.
- **`completion-atlas.json` was regenerated through its producer, never hand-edited.** `git diff`
  on that file shows exactly the ten `"line"` values plus `derived_at` (stamped to this cycle's
  own `HEAD`, `4df2c3fa0ad16b40b31893d61f7084ed47758f86`, by the producer's own staleness-gate
  bookkeeping) — no bucket population changed (`DONE`/`A`/`B`/`C`/`D`/`M`/`V`/`U`/`X`/`Z` all
  identical before and after). Re-derive: `git diff --stat
  docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` → `1 file
  changed, 11 insertions(+), 11 deletions(-)` (ten `"line"` fields + one `"derived_at"`).

## Durable-anchor recommendation (not implemented — see rationale)

The brief invited considering whether the citation should anchor to something more durable than a
raw line number, since line numbers will drift again on the next edit above any citation. **This
cycle recommends but does not implement** an anchor change, for two reasons:

1. **The mechanism's whole value is a human-verified pointer, not a self-discovering one.** If
   `_citation_failures` were changed to *search* for the marker's construction site itself (e.g.
   "find the line matching this regex within this function") rather than check a named line, the
   gate would stop verifying that a *person* re-confirmed the citation after an edit — it would
   verify only that the marker string exists somewhere plausible, which is close to the
   path-only-match shortcut this brief explicitly forbids. An anchor durable enough to survive
   edits automatically is, by construction, too loose to catch the class of defect condition 6
   exists to catch (a refactor that moves code without changing the marker string itself, cited
   in the module docstring at `scripts/completion_atlas.py:38-41`).
2. **A less drastic durable anchor — anchoring to a unique preceding/following literal instead of
   a raw line number (e.g. "the line containing `<marker>` nearest below the line containing
   `<unique preceding literal>`") — is worth building, but it is a `completion_atlas.py` logic
   change with its own test surface**, and this cycle's brief scope is the citation *data*, not
   the check's *mechanism* ("implement only if it keeps the content check honest; otherwise write
   it up as a recommendation and leave the mechanism alone"). Given this wave already found all
   ten citations broken (not the one the brief named), a mechanism change landed in the same
   cycle as a full data repair would make it harder for the next reviewer to tell which category
   fixed the gate. Recommend a dedicated future cycle: keep the exact-line content check as the
   ultimate arbiter, but let each `citation` optionally carry an `anchor` (a short unique literal
   string, e.g. the enclosing `fn` name) that a helper resolves to a line number via `grep -n`
   at check time, falling back to the stored `line` for buckets that never opt in. That keeps
   condition 6 honest (content is still checked at the resolved line, not assumed) while removing
   the need for a human to re-count line-number drift by hand every time unrelated code is
   inserted above a citation.

## Movement (four buckets)

- **Closure (`-> DONE`):** 0. No unit changed bucket this cycle.
- **Reclassification (bucket-to-bucket, non-DONE):** 0.
- **Reachability:** 0. Population, per-bucket counts, `unclassified`, and `overlap` are byte-
  identical before and after (`population=49438 buckets=10 unclassified=0 overlap=0`, same ten
  bucket counts in the `--check` output above and in the pre-fix run at the top of this receipt).
- **Instrument-correction:** **10** — every one of the ten `BUCKET_DEFINITIONS` citations
  repointed from a stale line to the live construction-site line, restoring condition 6 (the
  citation gate) from `citation_failures=10` (this cycle's own fresh measurement, not the
  brief's `1`) to `citation_failures=0`. This closes the *instrument's* own fail-closed gate; it
  closes zero content units.

## Denominator-gate self-check (run on this receipt and `progress.md` before commit)

Scoped to this cycle's own two new/changed files:
`python3 scripts/denominator_gate.py --check
docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-002_wave32_citation-repair_cycle_receipt.md`
→ `files_checked=1 violations=0`; `--check-provenance` on the same path →
`files_checked=1 figures_examined=0 violations=0` (no percentage or comma-grouped 4+ digit
figure appears in this receipt — the bucket counts and line numbers above are all bare
sub-4-digit or non-comma-grouped integers, outside both gates' scan pattern).

Full-repo `python3 scripts/denominator_gate.py --check` at this cycle's own working tree →
`files_checked=150 violations=3` — the same three pre-existing bare-percentage figures the wave-32
dispatch brief assigns to **Lane A** (`AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md`, lines
`138` and `153`; `progress.md` line `33`'s bare margin figure — unrelated content this lane did
not touch; `files_checked` rose from 149 to 150 only because this cycle's own new receipt file
entered the scan, at `violations=0`). `--check-provenance` full-repo → `files_checked=80
figures_examined=126 violations=0`. Neither run shows a violation introduced by this cycle's own
diff.
