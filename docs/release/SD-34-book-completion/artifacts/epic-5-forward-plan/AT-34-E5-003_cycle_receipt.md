# Cycle AT-34-E5-003 — Epic 5 (Price the remaining 35 books) / AT-34-E5-003

- **Commit SHA:** PENDING (set at push time; see `git log -1` on `tranche/14` after this cycle's push)
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_power_table_cost.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_power_table_cost.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/power-table-cost.json` (new, generated)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (regenerated — `derived_at` bump only, `python3 scripts/completion_atlas.py --check`'s own write-back to reflect HEAD; no bucket count changed)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-003_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** 4 hits on `placeholder` — all pre-existing lines from the
  already-committed `capability-register.json` (AT-34-E5-002), quoting corpus-shape vocabulary
  ("menu placeholder", "vacuous_placeholder_row") describing content shapes, not stub code. Not
  introduced by this cycle (this cycle added no `.json`/`.py` line containing any of those
  tokens — confirmed by re-running the grep restricted to this cycle's own new files only, which
  returns `OK_NO_TOKENS`).
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "421 units, all inside
  `ultimate_psionics` — not built here, costed here, using the measured build rate from Epic 2's
  eight tables and the spread across them. **Evidence:** the projected cost, the rate it derives
  from, and the reason it was not built (`decisions.md §7`). Plus what `ultimate_psionics` would
  still need after it exists — that book has all eight non-DONE buckets occupied, so the table
  alone does not close it, and the plan must say so."

## What this cycle built

`build_power_table_cost.py` re-derives `power-table-cost.json` at HEAD, every run. This artifact
is read-only against the rest of the repo (`workflow-instruction.md §3`): it prices the `power`
table, it builds nothing.

**Population (421), cross-checked four independent ways, all agreeing:**

1. Live `docs/work-inventory.json`: `status==engine-does-not-hold AND evidence contains
   'has_no_engine_table' AND kind=='power'` → 421.
2. `artifacts/epic-1-atlas/missing-engine-tables.json`'s `kinds.power.count` → 421.
3. Live directory listing, `data/corpus/ultimate_psionics/power/*.json` → 421 files.
4. `artifacts/epic-5-forward-plan/capability-register.json`'s own `power_engine_table.population`
   (AT-34-E5-002) → 421.

**Rate derivation.** `table-build-rate.json` (AT-34-E2-003) states its own finding plainly: the
real cost differentiator across the 7 new-table kinds is whether the kind's corpus directory
name matches its kind name, not record count (`ability`, 4,337 records, and `domain`, 183
records, cost nearly the same — 7 vs 2 marginal lines — because both use the unmodified generic
loader; `trait`'s mismatched `trait_generic` directory is the one dearer case, needing an
override plus a dedicated regression test).

`power`'s corpus directory is `data/corpus/ultimate_psionics/power/` — matches its kind name
exactly, confirmed live. It is therefore priced against the **6 kinds whose directory also
matched** (`ability`, `template`, `deity`, `domain`, `skill`, `language`), **not** against
`trait`'s dearer, mismatched-directory tier:

| Comparator | marginal lines | est. wall seconds (ESTIMATE, per table-build-rate.json) |
|---|---|---|
| `ability`, `template`, `deity` | 7 | 172 |
| `domain`, `skill`, `language` | 2 | 49 |
| `trait` (excluded — mismatched dir) | 12 | 295 |
| `companion` (excluded — pre-existing table) | 0 | 0 (not a build) |

**Projected cost for `power`:** marginal lines **2–7**, wall time **49–172 seconds**, reported as
a **range**, not a point estimate. This is explicitly a **DOUBLE-ESTIMATE** (AGENTS.md rule 9):
`table-build-rate.json`'s own per-table wall times are already pro-rated ESTIMATEs (no kind was
independently stopwatched — all 7 new kinds landed in one shared-loader commit); this projects
`power` onto that same estimate, one level removed from a real timing. Collapsing the range to
one number would fabricate precision the underlying data does not carry.

**Reason not built (`decisions.md §7`):** `power`'s 421 units all sit inside a 3,498-unit book
(`ultimate_psionics`) that occupies 6 other non-DONE buckets besides A. Building the table would
clear bucket A but bank no closed book to prove it inside SD-34's own two-book scope
(`core_rulebook`, `ultimate_campaign`), so it is priced here for the successor bundle's cleanest
opening move.

**What `ultimate_psionics` still needs after `power` exists.** Live re-derivation (not the
number `decisions.md §7` originally cited — see instrument-correction below):

| Bucket | DONE | A | B | C | D | M | U | V |
|---|---|---|---|---|---|---|---|---|
| Live count | 803 | 421 | 711 | 289 | 465 | 427 | 10 | 372 |

Book total: 3,498 (= 803+421+711+289+465+427+10+372, checked). Neither bucket X nor bucket Z has
any unit in this book. **Before** power is built, 7 non-DONE buckets are occupied (A, B, C, D, M,
U, V). **After** power is built, bucket A goes to 0 and **6 non-DONE buckets remain occupied**
(B, C, D, M, U, V) — the table alone does not close the book, matching the criterion's own bar
exactly.

**Instrument-correction, named per `decisions.md §9`:** `decisions.md §7` (authored earlier in
this bundle) states `ultimate_psionics` has "all eight non-DONE buckets occupied" with
`A=852, B=769, C=304, D=356, M=168, V=322, U=10` (summing to 2,781 — not the book's 3,498 total
either, so that snapshot predates DONE's own current split too). Re-derived live at HEAD
(`decisions.md §12` L2 — never carry a number forward), this book occupies **7**, not 8,
non-DONE buckets (neither X nor Z has any unit here), and A/B/M's live counts differ substantially
from the ones cited. The book's total (3,498) and `power`'s own population (421) have **not**
moved — only the internal bucket split has, between §7's authoring and this cycle's HEAD. This
is reported as instrument-correction / bucket-movement elsewhere in the corpus's history, not a
defect in this cycle's own work — `docs/work-inventory.json` is untouched by this cycle
(confirmed by `git status --porcelain` showing no diff to that file).

## RED → GREEN (TDD, `workflow-instruction.md §6` step 3)

**RED, confirmed for the intended reason:** mutated the committed artifact directly — set
`population.count` to `999` (wrong; live re-derivation says 421), appended `trait` to
`matched_directory_kinds` (an illegal comparator — mismatched directory, dearer tier), and set
`occupied_non_done_buckets_after_power` to `[]` (an illegal claim — power alone would close the
book, contradicting the acceptance bar). Live run:

```
FAIL: 8 violation(s)
 - population.count=999 but live re-derivation says 421
 - population.count=999 disagrees with capability-register.json's power_engine_table population=421
 - directory file count 421 disagrees with population.count=999
 - trait (mismatched directory, dearer tier) must never be a matched-directory comparator
 - marginal_lines_range [2, 7] does not match the matched-kinds spread [7, 7, 7, 2, 2, 2, 12]
 - wall_time_range [49, 172] does not match the matched-kinds spread [172, 172, 172, 49, 49, 49, 295]
 - the acceptance bar requires power NOT to close the book alone -- 'after' must be non-empty (this book has other non-DONE buckets)
 - 'after' [] must equal 'before' ['A', 'B', 'C', 'D', 'M', 'U', 'V'] minus bucket A exactly
```

All 8 failures fired for the planted defects, no crash. Reverted by re-running
`build_power_table_cost.py` (re-derives every field from HEAD, discarding all three mutations).

**GREEN:**

```
$ python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_power_table_cost.py
PASS: power population=421 (4 cross-checks agree), directory-match confirmed, rate drawn from matched-directory tier only (6 kinds, trait/companion excluded), book still needs ['B', 'C', 'D', 'M', 'U', 'V'] after power exists (table alone does not close it)
```

## Figures + their re-derive commands

- `population=49438 buckets=10 unclassified=0 overlap=0` — `python3 scripts/completion_atlas.py --check` (whole-corpus denominator).
- `power` population: **421 of 49,438** corpus-wide — four independent cross-checks above, all agreeing.
- `ultimate_psionics` book total: **3,498** — `python3 -c "import json,sys; sys.path.insert(0,'scripts'); import completion_atlas as ca; d=json.load(open('docs/work-inventory.json')); print(ca.partition(d['units'], book='ultimate_psionics')['examined'])"`.
- `ultimate_psionics` live bucket counts (of 3,498): `DONE=803 A=421 B=711 C=289 D=465 M=427 U=10 V=372` — same command, `.counts`.
- Projected marginal lines for `power`: **2–7**, of the 6 matched-directory comparator kinds (`table-build-rate.json`'s own measured/estimated per-kind figures) — `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/power-table-cost.json`, `rate_derivation.projected_marginal_lines_range`.
- Projected wall time for `power`: **49–172 seconds** (ESTIMATE, one level removed from a real timing — see confidence note above), of the same 6-kind comparator set — same artifact, `rate_derivation.projected_wall_time_seconds_range`.
- `trait`'s excluded, dearer comparator: 12 marginal lines / 295s — `table-build-rate.json`, `tables[trait]`.
- Occupied non-DONE buckets before power: **7** (A,B,C,D,M,U,V); after: **6** (B,C,D,M,U,V) — `power-table-cost.json`, `book_still_needs_after_power_exists`.
- Denominator gate against this package: `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=4` — all 4 pre-existing in `progress.md` (lines 209, 252, 309, 315), all inside verbatim-quoted corpus prose ("75% chance..."), already flagged by the already-merged `AT-34-E3-004` cycle; none introduced by this cycle (this cycle touched no `.md` prose besides its own receipt, `progress.md`'s prepended entry, and `kanban.md`'s row — neither contains a bare percentage).

## Row-count command output

```
$ python3 -c "
import json
d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/power-table-cost.json'))
print('population_count:', d['population']['count'])
print('projected_marginal_lines_range:', d['rate_derivation']['projected_marginal_lines_range'])
print('projected_wall_time_seconds_range:', d['rate_derivation']['projected_wall_time_seconds_range'])
print('occupied_after_power:', d['book_still_needs_after_power_exists']['occupied_non_done_buckets_after_power'])
print('matched_directory_kinds:', d['rate_derivation']['matched_directory_kinds'])
"
population_count: 421
projected_marginal_lines_range: [2, 7]
projected_wall_time_seconds_range: [49, 172]
occupied_after_power: ['B', 'C', 'D', 'M', 'U', 'V']
matched_directory_kinds: ['ability', 'template', 'deity', 'domain', 'skill', 'language']
```

## Build scope verified

`cargo test --locked --no-run` exit 0 (workspace, at the SHA this cycle committed at — see the
commit-SHA line above). `apps/desktop/src-tauri` explicitly run: `cargo test --locked --no-run`
exit 0 (that crate's own binary target compiled clean). No Rust source touched — Python/JSON-only
change (plus the benign `completion-atlas.json` `derived_at` write-back).

## Sweep population

N/A — this cycle added no corpus records and regenerated none. `corpus_literal_sweep`'s
examined-population is unaffected (`git status --porcelain -- data/corpus/` shows no diff).

## Oracle pin

N/A — no figure in this cycle's artifact came from the pinned PCGen oracle corpus; all figures
are drawn from `docs/work-inventory.json` and prior SD-34 artifacts.

- **Status:** complete

## Movement, four buckets

**Instrument-correction / naming, zero unit movement.** This cycle moves no unit on any bucket
board (`docs/work-inventory.json` untouched, confirmed above). It is a **naming/pricing**
artifact matching the criterion's own bar exactly: cost `power`, do not build it. It also
surfaces and corrects a stale figure in `decisions.md §7` (bucket split, not population or book
total) — filed as instrument-correction, not closure, per `decisions.md §9`.

## Notes

- The projected rate is deliberately reported as a **range** (2–7 marginal lines / 49–172s), not
  collapsed to a single number. `table-build-rate.json` itself already found that per-kind
  marginal cost inside the matched-directory tier varies 2–7x depending on macro-invocation
  formatting (short vs long book/key argument strings), a real but shallow driver that this
  cycle does not attempt to resolve further for `power` specifically, since doing so would
  require picking `power`'s actual sample book/key ahead of building the table — out of this
  cycle's read-only scope.
- `decisions.md §7`'s stated bucket split for `ultimate_psionics` (`A=852, B=769, C=304, D=356,
  M=168, V=322, U=10`, summing to 2,781) does not match this cycle's live re-derivation (summing
  to 3,498 with the book's DONE count included, or 2,695 excluding DONE — 6 buckets, not 7,
  because §7 also omits `C` from its own list despite listing "eight" occupied). This is reported
  plainly above and is **not** corrected in `decisions.md` itself — that file is not in this
  epic's file-touch set (`workflow-instruction.md §3`: `artifacts/epic-5-forward-plan/` only,
  read-only against the rest of the repo). The correction lives in this artifact and this
  receipt; a future cycle with write scope to `decisions.md` should fold it in.

## Next-cycle plan

1. `AT-34-E5-004` (plan ordered by real cost, single-bucket books flagged) can cite this cycle's
   `power-table-cost.json` directly for `ultimate_psionics`'s remaining-after-power shape rather
   than re-deriving it a second time.
2. A future bundle that actually builds `power` should re-time the build for real (not pro-rated)
   and correct this cycle's DOUBLE-ESTIMATE range with a single measured figure, following the
   same discipline `table-build-rate.json` used for the other 8 tables.
