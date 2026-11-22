# SD-19 Spell/Equipment Reachability — Operator-Driven Loop Instruction

This file is the body of the goal the `/loop 60m /goal` invocation runs.
It is **self-sufficient**: no interactive prompts, no mid-loop questions to the
operator, no shared state with anything other than the on-disk files
named here. The loop runs it; the loop restarts every 60 minutes; the loop
dies when the operator stops it.

This file is **fully self-contained**. It does not read from, look up, or
inherit procedural mechanics from any prior SD's loop-instruction, progress
doc, or source pattern. The SD-18 chassis loop (now archived from the
workspace root) was the historical reference for SD-19's cycle mechanics;
that procedural model is captured below in full. If a future session needs
to recover SD-19's cycle mechanics from a clean checkout, this file is
sufficient on its own.

## What this loop does

**Operator directive 2026-07-16 (mandate expansion):** "we need to reach
support/product-visible on all items except for the non-human race
issue — which isn't possible until we do a later book." This
supersedes SD-19's original charter of "9 spell schools + 4 equipment
categories" and folds in every other row of the seeded `SupportStateMatrix`
(all 7 races and all 12 classes and interactions, inherited from SD-13's
already-accepted chassis/spell-baseline work) as this loop's target,
too. See `## Full-matrix closure` below for the complete row-by-row
target list, the one named exception, and the concrete UI-surfacing
deliverables this expansion requires.

Ground **every row in the seeded support-state matrix** — all 7 races,
all 12 classes, both interaction rows (with one named exception), all 9
PF1 strict spell schools, and all 4 core-rulebook equipment categories —
toward `SupportState::Supported` AND evidence tier `Product-visible`.
The single exception: `interaction.non_human_any_class.progression_pressure`
stays `Unverified/Observed` — reaching any other state for that row is
not possible against the Core Rulebook corpus alone (see `## Full-matrix
closure` for why). Working in bounded cycles against the integration
branch `tranche/3` (SD-19 shares tranche-3 with the SD-18 chassis lane;
SD-19 will not begin until the chassis lane's loop completes). Each
cycle commits directly to `tranche/3` — no ephemeral feature branch, no
auto-merge, no PR.

## Required reading (every cycle)

### 1. Canonical handoff doc

```
cat /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
```

This is the canonical scope doc. The 15 acceptance criteria live there by
section number (§1.1 capability slice, §2.4 spell schools, §2.5 equipment
categories). Each criterion's acceptance criterion prose and concrete
corpus/code pointers live there. **As of 2026-07-16, this loop's target
is broader than this doc's own 15 criteria** — read `## Full-matrix
closure` in this file for the current complete target (33 of 34 seeded
matrix rows), and read `src/rules_core/rules_tables/crb/`-adjacent
`src/rules_core/support_state_matrix.rs` directly (`grep -n "row_id:"`)
for the live row list rather than assuming the scope-draft's 15 is
still the full picture.

### 2. Progress doc (SD-19's own; loop's working memory)

```
cat /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-progress.md
```

This is SD-19's own progress doc — separate from any other bundle's progress
doc. Created on first run if missing; frontmatter mirrors the canonical
shape (`title`, `mirrors` pointing at the scope draft, `created`,
`snapshot_as_of`). Each section in SD-19's progress doc corresponds 1:1
with §2.x in the scope doc. Under each section, the loop maintains `done` /
`in-flight` / `open` status rows with cycle-id, commit SHA, and card id.

### 3. Required reading from the corpus and the table store

```
ls /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/
# and
ls /home/ubuntu/workspace/repos/codex/src/rules_core/rules_tables/crb/
# and
grep -A 9999 "## 9. " /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
```

The PCGen corpus at `/home/ubuntu/workspace/repos/pcgen/data/` is the
source of truth for spell and equipment records. The CRB table store at
`/home/ubuntu/workspace/repos/codex/src/rules_core/rules_tables/crb/`
(populated by SD-19's foundation slice) is the load-bearing authority
surface for spell/equipment reachability. SD-19's `decisions.md` §9
documents the source-book subdirectory pattern (CRB → `crb/`, APG →
`apg/`, ACG → `acg/`); SD-21 reads from sibling directories per that
pattern. SD-19 cycles ground eligibility directly against the corpus and
the table store — not against any prior SD's progress doc.

### 4. Live git state

```
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git log origin/tranche/3 --oneline -5
git worktree list --porcelain
```

(No `git ls-remote origin | grep -E 'loop/tranche3-cycle-'` check — SD-19
has no feature branches.)

### 5. In-flight detection

```
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep
```

If any `claude` process is running with a prompt that names a specific
SD-19 acceptance criterion, do NOT pick that criterion. Cycle exits with
`CLAIM-EXISTS` status; loop restarts.

## Concurrency rules (read first, obey always)

These rules are structural. Two concurrent cycles that touch the same
file are guaranteed to collide; the loser will be Tech-Priest (or the
operator) having to reconcile.

### File-touch partition (the hard rule)

The SD-19 cycle surface is concentrated in these files:

| File | Purpose | Cycles that may touch it |
|---|---|---|
| `src/rules_core/pilot_compute.rs` | The compute seam — every spell/equipment cycle extends the corpus-aware seam function here | **One cycle at a time, full stop** |
| `src/rules_core/pilot_compute_corpus.rs` | The corpus-derived contribution function — cycles extend `compute_pilot_with_corpus` per the §Step 5 contribution shape | **One cycle at a time** |
| `src/rules_core/support_state_matrix.rs` | The matrix carrier — every cycle updates a row's evidence_tier | **One cycle at a time** |
| `src/rules_core/spell_resolver.rs` | The spell-key resolver — touched only by spell cycles that need to extend normalization/key handling | One cycle at a time |
| `src/rules_core/equipment_resolver.rs` | The equipment-key resolver — touched only by equipment cycles that need to extend normalization/key handling | One cycle at a time |
| `tests/sd19_<criterion>.rs` | Per-cycle test file | One cycle per file (its owning criterion) |
| `tests/fixtures/rules_core/sd19_<criterion>_*.txt` | Per-cycle fixture | One cycle per fixture |
| `apps/desktop/src-tauri/src/sd19_<domain>_catalog.rs` | UI-surfacing Tauri command per domain (spell/class/race — added per `## Full-matrix closure`) | One cycle per domain file |
| `apps/desktop/src/<domain>Catalog/*.tsx`, `apps/desktop/src/boundary/load<Domain>Catalog.ts` | UI-surfacing React screen + boundary wrapper per domain | One cycle per domain's file set |
| `apps/desktop/src/characterHub/{LandingScreen,CharacterHubPage}.tsx` | Hub navigation wiring — every new browser adds one nav link + one mode branch | **One cycle at a time** (shared file, same collision risk as the seam-matrix pair, just lower-traffic) |

The first three rows are the choke point for compute-grounding cycles.
The three new rows are the choke point for UI-surfacing cycles (added
2026-07-16 per `## Full-matrix closure`) — the hub nav files in
particular are shared across every domain's browser, so **at most one
UI-surfacing cycle may be active at a time**, same rule as the
compute-grounding side. **At most one cycle may be active across any of
these files.**

### Per-cycle spawn budget (the default)

Default: **1 cycle at a time.** Reason: the file-touch partition collapses
any parallel attempt into a serial one for the first three files. Two
cycles in parallel means two cycles racing on the matrix carrier, two
cycles serializing on `pilot_compute.rs` rebase, and zero speedup.

To run more than one cycle in parallel you must show that the second
cycle touches a disjoint file set. That is only possible when the second
cycle is doing **documentation-only work** (e.g. updating the progress
doc, writing a future-cycle handoff doc, refreshing the matrix markdown).
For code-bearing cycles, **1 cycle at a time is the rule**.

This is not a recommendation; it is a structural property of the cycle
surface.

### Branch control protocol (not applicable to SD-19)

SD-19 has no feature branches (per `decisions.md` §6). Concurrency is
controlled by the file-touch partition above and the in-flight check in
§5. If two `claude` processes were to start cycles touching the same
choke-point file, the file-touch partition's "one cycle at a time" rule
is the enforcement mechanism — whichever cycle starts second will see
the choke-point file dirty and exit `CLAIM-EXISTS`.

## Per-cycle procedure (the steps, in order)

### Step 1 — Pick a criterion

From the SD-19 progress doc's `open` list, pick the smallest unclaimed
acceptance criterion. Priority order (operator directive 2026-07-14,
amended 2026-07-16 per `## Full-matrix closure`):

1. **§2.4 spell schools** — DONE, no cycles remain.
2. **§2.5 equipment categories** — DONE, no cycles remain.
3. **Spell Catalog Browser** (build, then promote all 9 `school.*` rows).
4. **Class Progression Browser** (build, then promote all 12 `class.*` rows).
5. **Race Trait Browser** (build, then promote all 7 `race.*` rows).
6. **Human interaction-row judgment call** (`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) — last, per `## Full-matrix closure`'s explicit instruction.

**Never pick:** `interaction.non_human_any_class.progression_pressure`.
This row is permanently excluded from this loop's target — see
`## Full-matrix closure`. A cycle that picks this row is a mistake; it
should stop immediately, revert any change, and pick the next-priority
item instead.

**Eligibility check.** A criterion is eligible when:

1. The criterion has not yet reached `supported/Product-visible` (per the progress doc's `done`/`open` status), AND it is not the excluded non-Human interaction row.
2. No live `claude` process is working on that criterion (in-flight detection above).
3. For compute-grounding work (items 1-2 above, both already done): the chosen burden or family is **actually exercisable** from the corpus-aware compute seam — i.e. the seam function and the relevant resolver exist and are green.
4. For UI-surfacing work (items 3-5 above): the domain's compute grounding must already be `Partial/Computed` or better for every row it would promote (true for all of races/classes/remaining-schools today — this is SD-13/SD-19 grounding already landed, not new work). The cycle does NOT re-derive or re-verify the underlying compute grounding; it builds the missing UI surface and promotes the row's `evidence_tier`/`support_state` per `## Full-matrix closure`'s bar.
5. The cycle's RED test (for compute-grounding cycles) must verify that the corpus record it asserts exists in the real PCGen corpus, against `CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data`, before writing the test. If the asserted corpus record does not exist, the criterion is **not eligible** and the cycle writes to `## Open blockers` with the specific corpus-side defect and exits FAIL. (Not applicable to UI-surfacing cycles — see Step 4's UI-surfacing branch.)

When several criteria tie on priority above, prefer the one that has
not had a cycle attempted in the last 3 cycles (read the progress doc's
cycle log to check). The loop's job is to advance the **frontier**, not
to retry the same criterion forever.

### Step 2 — Pick the criterion's work-unit

Each acceptance criterion has more granular work-units within it. The
loop must NOT try to land a whole criterion in one cycle. Instead:

- For §2.4 spell schools and §2.5 equipment categories: DONE, no further work-units.
- For the **Spell/Class/Race Catalog Browsers**: the work-unit is the whole browser build (Tauri command + DTOs + React screen + nav wiring), mirroring the Equipment Catalog Browser's own build, which closed all 4 equipment categories in one build rather than one category per cycle. Do not try to build a browser incrementally across multiple cycles unless it turns out to be genuinely too large for one cycle — if so, split by sub-step (command+DTOs first, screen second, nav wiring third), not by data domain.
- For the **row promotions** that follow a browser build: one cycle may promote every row in that browser's domain at once (again mirroring the equipment precedent: one commit promoted all 4 equipment rows together), since the promotion is a `support_state_matrix.rs` edit plus updating whichever tests assert the row's prior state — not independent per-row work.
- For the **Human interaction-row judgment call**: the work-unit is the decision itself (option (a) or (b) from `## Full-matrix closure`) plus whatever `support_state_matrix.rs`/test edits that decision implies.

### Step 3 — Verify the working tree is on tranche/3 (no feature branch)

```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git checkout tranche/3
git pull origin tranche/3
git status --porcelain | wc -l   # expect 0; if non-zero, exit CLAIM-EXISTS
```

Cycle-id is the ISO-8601 timestamp the cycle started. Criterion is the
one chosen in Step 1 with work-unit from Step 2. Example cycle-id:
`2026-07-15T0900` (the cycle-id is recorded in the kanban card body and
the progress doc).

### Step 4 — Write the failing test first

**For compute-grounding cycles (§2.4/§2.5 — both done, kept for historical reference):**

Add `tests/sd19_<criterion>.rs`. Mirror the shape of the most recent
sibling cycle's test file. The test must fail for the intended reason
when run against `origin/tranche/3` as the base.

The RED test must include a corpus-existence assertion. Specifically:
the cycle's chosen fixture (one spell's `KEY:` for a school cycle, one
item's `KEY:` for an equipment cycle) must be verified to exist in the
real PCGen corpus before the test is written. A grep against
`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/<file>.lst`
is the verification.

```bash
cargo test --locked --test sd19_<criterion> 2>&1 | tail -40
```

Capture the failing output. It is the RED evidence.

**For UI-surfacing cycles (Spell/Class/Race Catalog Browsers — the live directive):**

There is no corpus-existence RED test in the compute-grounding sense —
the corpus data is already grounded. Instead, write a backend unit test
first, in the new `apps/desktop/src-tauri/src/sd19_<domain>_catalog.rs`
module, mirroring `sd19_equipment_catalog.rs`'s own tests
(`catalog_contains_all_four_categories_and_expected_counts`,
`every_entry_has_a_non_empty_key_and_name`): assert the new Tauri
command's builder function returns the exact expected count against
the real table store (e.g. 652 spells across 9 schools; 12 classes ×
20 levels; 7 races), and that every entry has non-empty required
fields. This test fails (module doesn't exist yet) — that is the RED
evidence for a UI-surfacing cycle.

```bash
cd apps/desktop/src-tauri && cargo test sd19_<domain>_catalog 2>&1 | tail -40
```

### Step 5 — Implement the smallest change that makes the test pass

**For compute-grounding cycles (§2.4/§2.5 — both done):**

- **Extend the seam's school-coverage block** for §2.4 cycles. Add the school's per-school corpus-derived contribution to `compute_pilot_with_corpus`'s output, keyed by `school:<school>.coverage` in the `corpus_derived` section.
- **Extend the seam's equipped-items block** for §2.5 cycles. Add the category's per-category corpus-derived contribution to `compute_pilot_with_corpus`'s output, keyed by `category:<category>.equipped_items` in the `corpus_derived` section.
- **Update `support_state_matrix.rs`** with the corresponding `MatrixSubjectType::School(...)` or `MatrixSubjectType::Equipment(...)` row's `support_state`, `evidence_tier`, `grounding_ref`, `blocker_or_lossiness_note`, and `next_required_uplift` fields.

For all paths, the change must be in `pilot_compute.rs` for the seam-extension work, in the resolver file for normalization/key edge cases, and the matrix carrier for state transitions. The forbidden write scopes are documented in `decisions.md` §1.3 (slot math, spellbook engine, equipment-effect breadth) — do not invent these from a cycle.

```bash
cargo test --locked --test sd19_<criterion> 2>&1 | tail -40
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

All three must be green. Capture the output. It is the GREEN evidence.

**For UI-surfacing cycles (the live directive):**

1. Add `apps/desktop/src-tauri/src/sd19_<domain>_catalog.rs`: a `build_<domain>_catalog()` pure function over the relevant `rules_tables::crb::*` store, DTOs with `#[serde(rename_all = "camelCase")]`, and a thin `#[tauri::command]` wrapper — mirror `sd19_equipment_catalog.rs` exactly.
2. Register the new module and command in `apps/desktop/src-tauri/src/main.rs` (`mod` + `use` + add to `generate_handler!`).
3. Add `apps/desktop/src/boundary/load<Domain>Catalog.ts` (raw invoke wrapper) and `apps/desktop/src/<domain>Catalog/<domain>CatalogRuntime.ts` (adds the preview-mode fallback via `hasTauriRuntime()`), mirroring `loadEquipmentCatalog.ts` / `equipmentCatalogRuntime.ts`.
4. Add `apps/desktop/src/<domain>Catalog/<Domain>CatalogScreen.tsx`: filter chips for the domain's natural grouping (schools for spells, classes for classes, races for races) + name search + a render cap with an explicit "showing first N of X" message — mirror `EquipmentCatalogScreen.tsx`.
5. Wire into `apps/desktop/src/characterHub/LandingScreen.tsx` (new link) and `CharacterHubPage.tsx` (new mode branch).
6. Update `support_state_matrix.rs`: promote every row in the domain to `Supported`/`ProductVisible`, rewriting `blocker_or_lossiness_note` to name the new browser (path + Tauri command + what it shows), mirroring how the equipment rows' notes were rewritten in commit `e9845f2`.
7. Update whichever "closed-world no unexpected Supported row" tests assert the prior state (the 24 files updated in commit `e9845f2` plus the master `tests/sd13_support_state_matrix.rs` — add the newly-promoted row ids to those allowlists the same way).

```bash
cd apps/desktop/src-tauri && cargo test 2>&1 | tail -40
cd /home/ubuntu/workspace/repos/codex && cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
cd apps/desktop && npm test && npm run typecheck && npm run build
```

All must be green. Capture the output. It is the GREEN evidence. **At
least once per new browser** (not necessarily every cycle after that),
live-verify via the `run-desktop` skill: launch the app, navigate to
the new browser, screenshot the full list + a category filter + a
search result, confirming real corpus-derived data renders — exactly
as was done for the Equipment Catalog Browser. Record the screenshot
evidence (or a description of what was verified) in the progress doc.

### Step 6 — Commit, push directly to tranche/3

**Compute-grounding cycles:**

```bash
git add src/rules_core/pilot_compute.rs \
        src/rules_core/support_state_matrix.rs \
        tests/sd19_<criterion>.rs \
        tests/fixtures/rules_core/sd19_<criterion>_*.txt
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd19): <criterion> (<row transition>)"
git push origin tranche/3
```

**UI-surfacing cycles** (mirror the two-commit equipment precedent —
browser build, then row promotion, as separate commits when both land
in the same cycle):

```bash
git add apps/desktop/src-tauri/src/main.rs \
        apps/desktop/src-tauri/src/sd19_<domain>_catalog.rs \
        apps/desktop/src/boundary/load<Domain>Catalog.ts \
        apps/desktop/src/<domain>Catalog/ \
        apps/desktop/src/characterHub/LandingScreen.tsx \
        apps/desktop/src/characterHub/CharacterHubPage.tsx
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd19): <domain> catalog browser (desktop app)"

# then, once live-verified:
git add src/rules_core/support_state_matrix.rs tests/sd13_*.rs tests/sd19_*.rs
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd19): promote all <N> <domain> rows to Supported/ProductVisible"
git push origin tranche/3
```

The commit lands directly on `tranche/3`. Capture the commit SHA — it is
the durable receipt (recorded as `merge_receipt_sha` in the card body
and progress doc).

### Step 7 — Open the PR (NOT APPLICABLE to SD-19)

SD-19 has no PRs. Per `decisions.md` §6, every cycle commits directly to
`tranche/3`. The `tranche/3 → develop` promotion PR is operator-driven
only and happens once at SD-19 closure, not per cycle.

### Step 8 — Auto-merge to tranche/3 (NOT APPLICABLE to SD-19)

SD-19 has no auto-merge. The commit is already on `tranche/3` by
construction.

### Step 9 — Cleanup (NOT APPLICABLE to SD-19)

SD-19 has no ephemeral branch to clean up. The next cycle's Step 3
checkout handles any stale working-tree state.

### Step 10 — Mint the kanban card (post-mortem record)

```bash
hermes kanban --board codex-tranche-3 create \
  "SD19 <criterion> (<criterion-section>) [cycle <cycle-id>]" \
  --assignee operator \
  --workspace scratch \
  --initial-status done \
  --created-by operator \
  --priority 3 \
  --body "<card body per schema below>"
```

Card body schema:

```
epic: SD-19
criterion_section: <scope doc section reference, e.g. "§2.4 Spell school cards: Abjuration">
row_or_kind: school:abjuration | school:conjuration | ... | category:arms_armor | category:general | category:magic_items | category:equipmods
evidence_tier_before: <previous matrix row state>
evidence_tier_after: <new matrix row state after this commit>
merge_receipt_sha: <commit SHA on tranche/3>
cycle_id: <ISO-8601 timestamp>
cargo_test_summary: <test summary string>
clippy_signal: clean | dirty
cycle_timing_seconds: <N>
self_heals_applied: <list, empty if none>
next_required_uplift: <recommendation for next iteration>
ui_surface: <operator-provided surface name, empty if none>
corpus_existence_verified: yes — <corpus path> :: <KEY: or item_id used>
```

### Step 11 — Update the progress doc

SD-19 cycles append to SD-19's own progress file
`./progress.md`.

1. Update the `snapshot_as_of` line in the frontmatter to the current `tranche/3` HEAD short SHA. (SD-19's own snapshot; not shared with any other bundle.)
2. Append a new entry to the cycle log under `## SD-19 cycles` (or the section that corresponds to the scope-doc §2.x the criterion lives in):

```
### cycle-<cycle-id> | <criterion> | <commit sha> | <card id> | <evidence transition> | cargo test <N>/<N> green | clippy clean | <timing>
```

3. If the cycle did not produce a landed commit (test could not be made green, corpus record missing, in-flight process blocked the criterion, etc.), add an `## Open blockers` entry under SD-19's section with the specific reason so the next cycle routes around it.

Do NOT rewrite the doc from scratch. Edit in place so the diff is small
and auditable.

### Step 12 — Exit the cycle

Print a final 7-line report and exit:

```
cycle: <cycle-id>
criterion touched: <criterion>
row_or_kind: <row_or_kind>
commit: <commit sha on tranche/3, or 'no commit: <reason>'>
card: <hermes kanban card id, or 'no card: <reason>'>
verify: cargo test <X>/<X> green; clippy clean
status: GREEN | FAIL | NO-OP | CLAIM-EXISTS
```

`/loop` restarts the cycle 60 minutes later. The next cycle re-reads the
progress doc and picks the next criterion.

## Coverage gap closure: representative-sample → every-item

**Operator directive 2026-07-16:** "i want to make sure that we brought in ALL spells, ALL armor, ALL weapons, ALL equipment, not just samples."

**The current state** (snapshot at `origin/tranche/3 @ c46c9b6`, per SD-19's progress doc):

- **§2.4 spell schools: full coverage.** All 9 schools landed at `Partial/Computed` against the **full** corpus-coverage criterion — every `cr_spells.lst` record for the school resolves via `spell_id_resolve` and reaches `corpus_derived.school_coverage[<school>]`. Total spell coverage: 73 Abjuration + 116 Conjuration + 50 Divination + 60 Enchantment + 87 Evocation + 47 Illusion + 62 Necromancy + 152 Transmutation + 5 Universal = **652 spells, all covered**.
- **§2.5 equipment categories: sample coverage.** All 4 categories landed at `Partial/Computed` against the **representative-sample** criterion — each cycle picked a 3-item sample and verified `equipment_id_resolve` returns `Some` for it. Coverage rate per cycle: arms_armor 1 of 3 (Longsword grounds; Banded Mail, Armor Spikes don't ground through `EQUIPMENT_TABLES`' bootstrap cell), general 1 of 3 (Backpack grounds), magic_items 0 of 3 (deliberately drawn from non-`.COPY=` records; the alphabetically-first item under each `.COPY=`-merged name would ground but the sample didn't include those), equipmods 0 of 3 (deliberately drawn from names verified unique across the file; `EQUIPMENT_TABLES`' single equipmods entry `"Material ~ Cloth"` is not in the sample by design).
- **The asymmetry is a contract bug, not a coverage gap on the spell side.** The scope doc's §2.5 acceptance criterion says "a representative sample of items"; the operator's true requirement is "every item." §2.4's criterion already says "every spell" and is satisfied. §2.5 needs to expand from sample to full coverage, symmetric with §2.4.

**The structural pre-condition blocking §2.5 full coverage.**

The cycle at `commit 1689b16` (magic_items, cycle-2026-07-16T1858) discovered a category-wide parser-level defect:

- `src/pcgen_import/lst_parser/equipment.rs:383-389` strips a `.COPY=<source>` suffix from `column_zero` (e.g. `Bolt (Base).COPY=Bolt, Crossbow` → `Bolt (Base)`), making `.COPY=` records indistinguishable from base-name records at the merge layer.
- `src/pcgen_import/lst_parser/equipment.rs:544-553` then merges records by name within the same source: every record sharing a base name collapses into a single `EquipmentRecord` carrying multiple `KEY:` tokens. This merge is intentional for `cr_equip_general.lst`'s genuine same-name continuation rows but is wrong for `.COPY=` records (which are distinct items that share a base name as a syntactic device).
- `equipment_id_resolve`'s `equipment_key_token` helper then returns only the first such token, so only the alphabetically-first item under each merged name resolves. The cycle at `commit c46c9b6` (equipmods) discovered the same defect for plain (non-`.COPY=`) duplicate names: two records share the name `"Cloth"` (`KEY:Material ~ Cloth` and `KEY:Artisan's Tools (Cloth)`); only the first-inserted token under the merged name resolves.
- **Net effect:** equipment_id_resolve is name-and-order dependent. Only one item per merged-name group can resolve today. **Full coverage is structurally blocked until the parser-merge defect is fixed.**

**The parser-merge defect is SD-17 lane work, not SD-19.** The defect lives at `src/pcgen_import/lst_parser/equipment.rs:544-553` (and the `.COPY=` strip at lines 383-389). SD-17 owns the corpus-side parser work; SD-19 owns the corpus-aware compute seam that consumes the parser's output. SD-19 cannot fix the parser without expanding lane scope past the handoff. The right move is to record the dependency in SD-19's doctrine and let SD-17's loop close it.

**What this loop fires against (the next-loop directive).**

When the operator fires this loop document at the next session, the loop's job is to:

1. **Check the parser-merge defect status first.** `git log origin/tranche/3 --oneline -- src/pcgen_import/lst_parser/equipment.rs` to see if SD-17 has landed a fix. If yes, proceed to step 2. If no, the cycle writes to `## Open blockers` with a "BLOCKER: SD-17 parser-merge fix required before §2.5 full coverage can run" entry and exits `FAIL`. The loop does NOT spawn §2.5 cycles against a known-blocked criterion.
2. **If the parser-merge fix has landed, re-run each §2.5 cycle at full coverage.** One cycle per category. The cycle's per-item fixture set is **every item** in the category's `cr_equip_<category>.lst`, not a 3-item sample. The cycle asserts:
   - `equipment_id_resolve` returns `Some((&EquipmentRecord, Some(table_cell_ref)), _)` for **every** item in the category (not just a sample).
   - The receipt's `corpus_derived.equipped_items` lists **every** resolved item when the cycle's `CharacterInput.equipment_selections` carries the full category's `item_id` set.
   - Every cycle's fixture set is unique (no `.COPY=`-merged-name collisions; no plain duplicate names; the same uniqueness check the equipmods cycle ran for its 3-item sample runs for the full category).
3. **§2.4 cycles do NOT re-run.** Spells are at full coverage. Re-running them is redundant and wastes cycles.
4. **The progress doc reframes.** When §2.5 cycles re-run at full coverage, the progress doc's `## §2.5 Equipment categories (4)` section is rewritten to record the full-coverage result, not the sample coverage. Each category's row reads: "All N real-corpus [category] items (cr_equip_[category].lst) resolve via `equipment_id_resolve` and reach `corpus_derived.equipped_items` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; `derived_stats` population is a documented capability-slice non-goal, not this cycle's job)." — symmetric with the §2.4 progress doc entries.
5. **UI-surfacing gate is downstream.** Once §2.5 reaches full coverage, the `Partial/Computed` ceiling is correct: the remaining work to `Supported/Product-visible` is the operator's UI surfacing (per `decisions.md` §1.3 and the loop's own `## What "supported / Product-visible" actually means for SD-19` section). Full-coverage `Partial/Computed` + UI surfacing = `Supported/Product-visible`. Full-coverage `Partial/Computed` is the loop's exit condition.

**New acceptance criterion (added to scope doc §2.5):**

> For each of the four `core_rulebook/cr_equip_*.lst` files (`arms_armor`, `general`, `magic_items`, `equipmods`), **every item** in the category is reachable from a chosen `CharacterInput.equipment_selections`, resolved via `equipment_id_resolve`, and produces corpus-derived stat contributions in the receipt's `equipped_items` list. Per category, the corresponding `MatrixSubjectType::Equipment(EquipmentCategory)` row reads `support_state=Supported` and `evidence_tier=Product-visible`. Each cycle's `ResolvedEquipment` carries a `TableCellRef` pointing at the item's row of the relevant CRB equipment table. **Pre-condition:** the SD-17 parser-merge defect at `src/pcgen_import/lst_parser/equipment.rs:544-553` (and the `.COPY=` strip at lines 383-389) must be fixed before this criterion can be met; otherwise full coverage is structurally blocked. Per category, the count of items that must ground = the line count of the relevant `cr_equip_*.lst` file (verified by `wc -l` against the live corpus).

**Eligibility check (Step 1) — amended for the next loop run.**

Step 1's eligibility rule 3 currently says the chosen burden must be exercisable from the seam. For §2.5 cycles fired under the new criterion, rule 3 is amended to: "the chosen category must have **every** item in `cr_equip_<category>.lst` exercisable through `equipment_id_resolve` against the post-parser-merge-fix code. If any item fails to resolve (e.g. parser-merge defect still present, or a corpus record is malformed), the cycle does NOT land at sample coverage; it writes to `## Open blockers` with the failing item's `KEY:` token and the specific parse/resolution error, and exits FAIL." This closes the loophole that allowed the original §2.5 cycles to land at sample coverage despite the parser-merge defect.

**Acceptance verdict for §2.5 from the prior loop run (re-affirmed):**

The prior loop's 15/15 `Partial/Computed` count is correct **against the prior scope doc's representative-sample criterion** — every row landed its sample. It is **not correct** against the new "every item" criterion. The prior loop's 15/15 is re-grounded to whatever the next loop run lands against the new criterion. If all 4 §2.5 cycles re-run at full coverage with the parser-merge fix in place, the new count is 15/15 `Partial/Computed` (full coverage, awaiting UI surfacing). If the parser-merge fix is not yet in place, the next loop run's §2.5 cycles write to `## Open blockers` and the count stays at the prior-loop's sample-coverage state until SD-17 closes the parser defect.

**Status: this section is now historical/resolved.** Both structural
pre-conditions closed (SD-17 parser-merge fix cherry-picked to
`tranche/3` at `22eeed9`; full per-item coverage landed at `5fef69c` /
`de88434` / `513d8a6`), and the equipment UI-surfacing gap closed too
(Equipment Catalog browser, `c19b9be` / `e9845f2`) — all 4
`equipment.*.equipment_reachability` rows are `Supported/ProductVisible`
as of 2026-07-16. This section is kept for its historical record of how
the defect was found and fixed; it is not a live directive for future
cycles. The live directive is `## Full-matrix closure` below.

## Full-matrix closure (operator directive 2026-07-16): every row except the non-Human interaction row

**What changed.** When asked whether all classes/races/equipment/spells
were both `Supported` and `Product-visible`, the honest answer was: no
— only the 4 equipment rows and 2 spell schools (Abjuration, Illusion)
had reached that bar; all 7 races, all 12 classes, the remaining 7
spell schools, and both interaction rows were still short. The operator
responded by expanding this loop's mandate to close that gap
everywhere except one named, currently-unclosable row. This section is
the authoritative target state for every future cycle until the loop
reports full closure.

**The complete row inventory and target (34 seeded rows total):**

| Group | Rows | Current state (2026-07-16) | Target |
|---|---|---|---|
| Equipment categories (4) | `equipment.{arms_armor,general,magic_items,equipmods}.equipment_reachability` | `Supported/ProductVisible` | **Done** — no further work |
| Spell schools, done (2) | `school.{abjuration,illusion}.spell_reachability` | `Supported/ProductVisible` | **Done** — no further work |
| Spell schools, remaining (7) | `school.{conjuration,divination,enchantment,evocation,necromancy,transmutation,universal}.spell_reachability` | `Partial/Computed` (full per-school corpus coverage already landed — every spell in each school resolves and reaches `corpus_derived.school_coverage`) | `Supported/ProductVisible` — **UI-surfacing only; no further compute-grounding work needed** |
| Races (7) | `race.{human,dwarf,elf,gnome,half_elf,half_orc,halfling}.*_semantics` | `Partial/Computed` (SD-13 grounding; each race's bounded ability-modifier/size/speed/senses recognition bundle already computed) | `Supported/ProductVisible` — **UI-surfacing only** |
| Classes (12) | `class.{fighter.level_1_pilot,fighter.levels_2_10,rogue,barbarian,bard,cleric,druid,monk,paladin,ranger,sorcerer,wizard}.*` | `Partial/Computed` (SD-13 grounding; each class's bounded chassis/spell-burden recognition already computed, several classes carry named claim-blocking diagnostics for out-of-scope burdens — those diagnostics are correct honesty, not gaps to close) | `Supported/ProductVisible` — **UI-surfacing only, for whatever each row's own `blocker_or_lossiness_note` already names as grounded**; do NOT invent new compute (slot math, spellbook engine, spell save DCs, equipment-effect breadth) to chase a promotion — those stay named, honest gaps per `decisions.md` §1.3 |
| Interaction row, Human (1) | `interaction.human_bonus_feat_ability_bonus.pilot_pressure` | `Partial/ProductVisible` (evidence tier already Product-visible; support_state is Partial because its own `next_required_uplift` ties promotion to generalizing beyond the single named Human sample) | `Supported/ProductVisible` — **see the explicit judgment-call instruction below; this row is the trickiest one** |
| Interaction row, non-Human (1) | `interaction.non_human_any_class.progression_pressure` | `Unverified/Observed` | **Excluded — see below. Stays Unverified/Observed. Do not attempt to promote this row.** |

**Total: 33 of 34 rows targeted at `Supported/ProductVisible`; 1 row explicitly and permanently excluded from this loop's target set.**

**Why the non-Human interaction row is excluded.** Read the row's own
`blocker_or_lossiness_note` and `next_required_uplift` in
`support_state_matrix.rs` before touching this — the SD13-E2-F15
verdict already reasoned through this carefully: every non-Human race
row grounds a class-independent recognition bundle (no race seam
branches on class identity), and every class row with Computed evidence
either gates to `race:human` specifically or is Blocked on an
out-of-scope burden — so **no class row's compute path currently
branches on a specific non-Human race identity**, meaning there is no
real non-Human race×class interaction pressure for this row to ground
without inventing one. That branching behavior (e.g. a race-specific
favored-class bonus, a racial paragon archetype, a race-gated
prestige-class prerequisite) is real PF1 content, but it lives in
sourcebooks beyond the Core Rulebook (APG, ACG, race-specific
supplements) that this repo has not yet ingested — `decisions.md` §9's
`crb/`/`apg/`/`acg/` sibling-directory pattern exists precisely so a
future SD can add that content without restructuring the table store.
**Do not attempt to promote this row.** Do not invent a non-Human
interaction pressure to satisfy it. A cycle that finds itself tempted
to do so should stop and re-read this paragraph instead.

**The judgment call on the Human interaction row.** Unlike every other
targeted row, this one's own documented uplift path
(`next_required_uplift`: "generalize the named Human pilot pressure into
the interaction-row model once a second computed interaction pressure
exists") is now structurally unreachable — the "second computed
interaction pressure" it names is the very non-Human row this loop is
explicitly excluding. Whichever cycle picks this row up must make an
explicit, written judgment call between two options, and must record
the choice and its reasoning in the progress doc (not silently pick
one):

- **(a)** Treat the row's own named sample (the `human_bonus_feat ->
  feat:dodge` and `human_ability_bonus -> ability:strength` selections)
  as sufficient to satisfy this loop's generic Supported bar — both are
  already grounded as real compute explanations and already
  Product-visible per `tests/sd18_preloop_consumer_compose.rs` — and
  promote to `Supported/ProductVisible` with a note explicitly stating
  that the broader "generalize beyond the named sample" ambition is
  decoupled from this row's own Supported bar (i.e., "Supported" here
  means "this row's own named claim is fully grounded and surfaced,"
  not "the interaction-row model itself is generalized").
- **(b)** Determine that decoupling is not honest — that this row's own
  documented bar for Supported genuinely requires the generalization it
  names, which is now permanently blocked by the paired row's
  exclusion — and record this as a **second** named, deliberate
  exception alongside the non-Human row, updating this document's
  target count from "33 of 34" to "32 of 34."

Either answer is acceptable; silently picking one without writing the
reasoning to the progress doc's Open Blockers (or cycle log) is not.

**The concrete UI-surfacing deliverables this expansion requires.**
Every remaining `Partial/Computed` row above needs the same two things
the equipment rows needed: (1) already-done compute grounding — true
for all of them today — and (2) a live, operator-reachable UI surface
in the desktop app showing the full grounded data set, not a
per-character sample. Follow the Equipment Catalog Browser precedent
exactly (`apps/desktop/src-tauri/src/sd19_equipment_catalog.rs` +
`apps/desktop/src/equipmentCatalog/EquipmentCatalogScreen.tsx`,
commits `c19b9be` / `e9845f2`): a thin `#[tauri::command]` wrapper
returning every real entry from the relevant table store, a React
screen with filter chips + name search + an explicit "showing first N
of X" cap (no silent truncation), wired into hub navigation via a link
on `LandingScreen.tsx`. One browser build closes every row in its
domain at once (the equipment browser closed all 4 categories in one
build, not one per category) — build these the same way:

- **Spell Catalog Browser** — one new Tauri command over
  `rules_tables::crb::spell_list` (all ~652 real `cr_spells.lst`
  records), one new React screen with school filter chips (9 schools)
  + name search, wired into hub nav. Closes all 7 remaining
  `school.*.spell_reachability` rows in one build (Abjuration/Illusion
  can be folded into the same browser for consistency, though they're
  already Supported).
- **Class Progression Browser** — one new Tauri command over
  `rules_tables::crb::class_tables` (every class's full level-1-20
  table: BAB, saves, named features), one new React screen filterable
  by class. Closes all 12 `class.*` rows in one build.
- **Race Trait Browser** — one new Tauri command surfacing each race's
  bounded ability-modifier/size/speed/senses recognition bundle (the
  same data each race's own seam in `pilot_compute.rs` already
  computes), one new React screen listing all 7 races. Closes all 7
  `race.*` rows in one build.

Each browser build is its own cycle (or two, mirroring the equipment
precedent's browser-commit + promotion-commit split); promoting the
rows in that domain follows in the same or the very next cycle once the
browser is live-verified (via the `run-desktop` skill, at minimum once
per new browser — screenshot proof that the full data set renders and
that filtering/search work against real corpus-derived data, exactly as
was done for the Equipment Catalog Browser).

**Priority order for these new cycles** (added to Step 1's existing
priority list, after the already-complete §2.4/§2.5 items):

1. Spell Catalog Browser build, then promote all 9 `school.*` rows.
2. Class Progression Browser build, then promote all 12 `class.*` rows.
3. Race Trait Browser build, then promote all 7 `race.*` rows.
4. The Human interaction row's judgment call (see above) — tackled
   last, since it is the one row whose promotion path is genuinely
   ambiguous and needs a written decision, not a mechanical build.

**This section supersedes the "operator-driven, not loop-driven" UI
gate** in `## What "supported / Product-visible" actually means for
SD-19` below for these specific rows — see that section's own updated
text for the current framing. The compute-grounding side of the bar
(every named item/spell/class/race sample resolves and reaches the
corpus-aware seam or its SD-13-era equivalent) is unchanged and remains
non-negotiable.

## Self-healing posture

The loop self-heals wherever the failure is mechanically resolvable. The
operator returns from a multi-day run to a list of problems — not a
stopped loop.

### Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start (an in-flight cycle left uncommitted work) | `git status --porcelain \| wc -l` returns non-zero at Step 3 | Run `git stash` (if the dirty state is from a previous unfinished attempt) or `git checkout -- .` (if it's stray edit noise); re-verify clean; retry |
| Resolver normalization edge case for equipment_id_resolve | RED test asserts `equipment_id_resolve` returns `None` for a known-good fixture (existing fixture namespace `"item:<name>"` that should resolve to a corpus record) | Extend the normalization rule in `equipment_resolver.rs`; add the fixture's normalization pattern to the test suite |
| Spell resolver key edge case for spell_id_resolve | RED test asserts `spell_id_resolve` returns `None` for a corpus `KEY:` token that the corpus-side asserts is present | Extend the lookup in `spell_resolver.rs` (e.g. add a normalized fallback index, or document the cycle's finding as a corpus-side defect and route to Open Blockers instead — see non-self-healable) |
| Progress doc snapshot drift | Progress doc > 5 commits behind `tranche/3` | Read live matrix, refresh progress doc snapshot, retry |
| Cargo build cache corruption | `cargo build` reports stale state | `cargo clean`, rebuild, re-run tests |

### Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| Corpus-derived contribution cannot be grounded for a school or category because the corpus record is missing or malformed | RED test fails after corpus-existence verification confirms the record is absent or broken | Corpus-side work is SD-17's lane; cycle routes to Open Blockers, operator decides |
| Slice commit needs to be amended or reverted mid-cycle (e.g. clippy signal dirty post-commit, sibling regression surfaces after push) | `git log origin/tranche/3 -1` shows the cycle's commit is the tip; full suite or sibling-row check fails | `git reset --soft HEAD~1`, fix, recommit, force-push `tranche/3` (operator confirms before force-push; linear commit-to-tranche-3 means force-push is acceptable when the previous commit was seconds old and only on `tranche/3`, not `develop`). If the commit has been on `tranche/3` for any non-trivial time, escalate to operator — `tranche/3` may have downstream observers. |
| Two live `claude` processes would both touch `pilot_compute.rs` or `support_state_matrix.rs` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule |
| A cycle's RED test depends on a corpus record that does not exist in the real PCGen corpus | Corpus-existence grep returns no match before the test is written | Corpus-side defect; cycle routes to Open Blockers |
| Same criterion has failed twice already in this run | Progress doc cycle log shows two cycles for the criterion with FAIL status | Operator pause; consider scoping or seeding down the criterion |
| Conflict requires a domain decision (which side wins on a school-spell or equipment-stats semantics question) | Merge conflict has overlap on a question with no mechanical resolution | Operator must decide which semantics are canonical |
| The progress doc and the live matrix disagree on a row's `evidence_tier` | Cycle's expected vs. actual differ (not just stale snapshot) | Manual operator reconciliation required |

## Hard stops (refuse, exit FAIL)

The cycle refuses to advance when any of the following is true. In every
case the cycle writes the reason to `## Open blockers` in the progress
doc and exits with `FAIL`.

- The progress doc and the live matrix disagree on a row's `evidence_tier` and the disagreement is not just a stale snapshot.
- `cargo test --tests` regresses on a row other than the one the cycle touched. Sibling-preservation is a hard rule.
- Two live `claude` processes are working on cycles that would both touch `pilot_compute.rs` or `support_state_matrix.rs` or the shared hub-navigation files.
- **SD-19-specific:** a cycle's RED test depends on a corpus record that does not exist in the real PCGen corpus, verified against `CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data`.
- **A cycle attempts to promote `interaction.non_human_any_class.progression_pressure`.** This row is permanently excluded — see `## Full-matrix closure`. Revert the change and pick the next-priority item.

## What "supported / Product-visible" actually means for SD-19

A row or criterion reaches `supported/Product-visible` only when **both** of these are true:

1. The row's evidence_tier is `Product-visible` — a live, operator-reachable UI surfaces the row's full grounded data set. **Updated 2026-07-16:** for the rows targeted by `## Full-matrix closure` (all races, all classes, the remaining spell schools), building this UI surface is now explicitly loop-driven cycle work — the Catalog Browser build steps in Step 4/Step 5's UI-surfacing branch — not something that waits on a separate operator-authorized session. (The original "operator-driven, not loop-driven" framing described how the equipment and Abjuration/Illusion rows were actually promoted historically — through operator-authorized follow-up sessions, not autonomous loop cycles. The operator's 2026-07-16 directive changes this going forward: the loop itself now builds the remaining browsers.)
2. **Every** named spell / equipment / class / race sample listed in the row's `blocker_or_lossiness_note` is grounded as a real corpus-derived contribution (not a recognition record, not a diagnostic string). This bar is unchanged and non-negotiable.

Until both are true, the row is `Partial/Computed` or `Blocked/Computed`,
or higher tiers. The honest promotion per cycle is usually
`Blocked→Partial` or `Partial→Partial` widening, EXCEPT for the
Catalog Browser cycles under `## Full-matrix closure`, where a
`Partial→Supported` jump in the same or immediately-following cycle is
correct and expected — because condition 2 above is already satisfied
for every race/class/remaining-school row (SD-13/SD-19 grounding
already landed); only condition 1 (the UI surface) is missing, and
building it is exactly what that cycle does. Do NOT promise a
`→Supported` jump for any row where condition 2 is not already
satisfied; do NOT ship a card whose body claims a promotion the diff
does not support.

The remaining structural gaps that **no SD-19 cycle can close** are the
spellbook engine, slot math, spell save DCs, and broad equipment-effect
computation named in `decisions.md` §1.3, plus the one permanently
excluded row (`interaction.non_human_any_class.progression_pressure`,
per `## Full-matrix closure`). When every other row is `Supported`,
those gaps become named blockers for a future SD-N, not excuses for
fake completion.

## How the loop will end

The `/loop` form exits when the operator stops it. There is no automatic
stopping condition. The loop keeps picking the next-best criterion until
**every row is `Supported/Product-visible` except the one permanently
excluded row** (`interaction.non_human_any_class.progression_pressure`),
or every remaining row has a real blocker in `## Open blockers`. Per
`## Full-matrix closure`, that means 33 of 34 rows at `Supported/ProductVisible`
— or 32 of 34 if the Human interaction-row judgment call lands on
option (b) (a second permanent exception, explicitly recorded).

The operator can stop the loop at any time; a stopped loop leaves the
progress doc in the state of the last completed cycle, with all open
claims expired, and the operator can resume by relaunching `/loop 60m
/goal <this file>`.

## Operating posture (for the operator launching the loop)

1. **Launch with `/loop 60m /goal <this file>`.** SD-19 is serial
   (single criterion per cycle, single cycle at a time); `/batch` is not
   used. The 60-minute cadence is one cycle long enough to land a small
   criterion, short enough that a stuck cycle doesn't waste a long block.

2. **Default ceiling: 1 cycle at a time.** The file-touch partition
   collapses any parallel attempt.

3. **Watch the progress doc, not the loop output.** The cycle log is
   the durable truth. If the log shows three cycles in a row with no
   landed commit, the loop is stuck on a structural problem and the
   operator should investigate before letting it run another cycle.

4. **Tranche-branch direct-commits mean PRs to develop are operator's
   call.** The loop commits directly to `tranche/3` per cycle. The
   operator opens the `tranche/3 → develop` promotion PR once at SD-19
   closure, not per cycle.

5. **Post-mortem record is the kanban board.** Each cycle mints a card
   on `codex-tranche-3` with the §Step 10 schema. A 3-day-later operator
   reads the board to reconstruct what happened.

6. **The 5-hour window applies here too.** A 60-minute cycle × 5 hours
   = up to 5 landed criteria per 5-hour window, with each criterion on
   a distinct SD-19 acceptance criterion. Realistic target: 3-5
   criteria per window with 1 cycle each. The progress doc accumulates
   the progress; the operator reviews the merge history and the kanban
   board on the same cadence.

7. **The SD-18 chassis loop must be complete before SD-19 begins.** SD-19
   starts after the chassis lane's loop finishes its lane (or is
   otherwise paused by the operator). The two loops do not run
   concurrently. This is a sequencing constraint, not a workload
   constraint — once the chassis lane is closed, SD-19 begins without
   further gating.

8. **Force-push discipline on `tranche/3` is conservative.** Because
   SD-19 commits directly to `tranche/3`, a mid-cycle correction (clippy
   dirty post-commit, sibling regression) requires a `git reset --soft
   HEAD~1` + force-push. This is acceptable only when the previous
   commit was seconds old and no downstream observer has fetched. If
   the commit has been on `tranche/3` for any non-trivial time,
   escalate to operator before force-pushing — see §Self-healing
   posture.

## Cross-reference

- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md` (canonical handoff doc; 15 acceptance criteria with corpus/code pointers)
- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-progress.md` (SD-19's own progress doc; loop's working memory; created on first run)
- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/` (spec-domain STC bundle; doctrine + decisions + technical-design + risks + acceptance)
- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md` §6 (linear commit-to-tranche/3, no feature branches)
- `/home/ubuntu/workspace/repos/pcgen/data/` (PCGen corpus; source of truth for spell and equipment records)
- `/home/ubuntu/workspace/repos/codex/src/rules_core/rules_tables/crb/` (CRB table store; populated by SD-19's foundation slice)
- **Equipment Catalog Browser — the reference pattern for every `## Full-matrix closure` UI-surfacing cycle**: `apps/desktop/src-tauri/src/sd19_equipment_catalog.rs` (Tauri command + DTOs), `apps/desktop/src/boundary/loadEquipmentCatalog.ts` + `apps/desktop/src/equipmentCatalog/equipmentCatalogRuntime.ts` (boundary/runtime split), `apps/desktop/src/equipmentCatalog/EquipmentCatalogScreen.tsx` (React screen), `apps/desktop/src/characterHub/{LandingScreen,CharacterHubPage}.tsx` (nav wiring) — commits `c19b9be` (browser) and `e9845f2` (row promotion + closed-world test updates) on `tranche/3`.
