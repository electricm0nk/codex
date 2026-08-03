---
title: Book Ingestion Playbook
stc_id: GOV-BOOK-INGESTION-PLAYBOOK
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-30
canonical_source: ~/workspace/repos/codex/docs/governance/book-ingestion-playbook.md (this file)
supersedes: (none — first issuance)
related_artifacts:
  - ./no-stub-mvp-doctrine.md (sibling doctrine — shipped code must do what it claims)
  - ../architecture/corpus-ingest.md (the parse pipeline this playbook sits on top of)
  - ../architecture/rules-data-tables.md (what a book's `rules_tables` module looks like)
  - ../release/v0.6/execution-engine-scoping.md (the "build no execution engines" verdict)
date: 2026-07-30
---

# Book Ingestion Playbook

Four of the twenty-five PCGen book directories are ingested. This file is the
procedure for the rest, written so a session with no memory of the first four
can run a cycle correctly on the first attempt.

## How to read this file

Every expensive lesson from the first four books now has a **tool that
enforces it**. This playbook's job is to tell you which tool, and when to run
it — not to re-argue the lesson. Prose drifts; that is this project's central
finding. Where a paragraph here and a tool disagree, **the tool is right and
this file is stale** — fix the file.

| The failure | What now catches it | Your obligation |
|---|---|---|
| Content ingested but reaching no player surface | `apps/desktop/src-tauri/src/reach_gate.rs` (`verify.sh` stage `reach`) | Land the surface in the same cycle as the ingest |
| Ad-hoc verification letting red ship | `scripts/verify.sh` + `scripts/verify-baselines.env` | Run it; never compose your own; never pipe it |
| Counts wrong on first pass | `src/bin/v06_corpus_trap_report.rs`, `src/bin/v06_work_inventory.rs` | Derive every number; quote none from memory |
| Status claims drifting from code | `src/bin/v06_class_state_dump.rs` (`verify.sh` stage `class-dump`), `docs/work-inventory.json` | Cite the generator, don't hand-maintain a tally |
| Corpus shapes rediscovered by hand each book | `src/pcgen_import/corpus_traps.rs` | Run the trap report **before** writing ingest code |

Four things below are *not* tool-enforced and are therefore the parts you have
to carry yourself: the shape assessment (§1), the counting discipline (§6),
independent re-verification (§7.1), and the standing authority to correct the
brief (§7.2).

## The cycle in one screen

```sh
# 0. Shape the book. Read books[<book>] in the output: kinds,
#    files_not_enumerated, trap_hits, reconciliation.            -> §1
cargo run --locked --bin v06_work_inventory

# 1. Trap-report the book. BEFORE writing any ingest code.       -> §2
cargo run --locked --bin v06_corpus_trap_report -- <book_dir>

# 2. Ingest AND surface, in the same cycle. Add a reach_of arm
#    that executes the real IPC builder.                         -> §3

# 3. Verify. Exit code captured directly, never piped.           -> §4
./scripts/verify.sh

# 4. Definition of done.                                         -> §5
cargo run --locked --bin v06_corpus_trap_report -- --audit
cargo run --locked --bin v06_work_inventory        # units leave not-started
```

If you do only one thing differently from how you would have done it anyway,
make it step 1 — it is the step every previous cycle skipped, and the step that
made every previous cycle's counts wrong.

---

## 1. Step 0 — assess the book's shape, before anything else

Books differ enormously and the cycle shape follows from the book, not from a
template. The Core Rulebook is a spell/feat/equipment book with a large class
chassis. Bestiary 1 is monsters and almost nothing else. Ultimate Equipment is
one gigantic equipment table. Bestiary 5's PCGen dataset contains **no monster
file at all** — it is a player-options dataset (races, companions, feats).
A per-monster-block cycle plan for that book would produce zero cycles.

Note the framing trap in that sentence: "Bestiary 1 is monsters" is *almost*
right, and the generator disagrees in a way that matters. Its largest unit kind
is not `monster` (330) but `race_trait` (620) — the monsters' racial abilities,
which the generator counts separately and which a per-monster-block cycle plan
does not name. Nothing about the book is surprising; the *count* is, and only
because it was derived rather than assumed.

The shape is generated, not guessed:

```sh
cargo run --locked --bin v06_work_inventory     # regenerates docs/work-inventory.json
```

Then read the book's entry out of `docs/work-inventory.json`:

- `books[].kinds` — units per kind for that book, each with a `by_status` map.
- `books[].scope` — `in_scope` / `future_state` / `shared_library` / `out_of_scope`.
- `books[].pcc_includes` and `books[].included_by` — the include graph. A book
  that includes `core_essentials` inherits its records; a "book" that only
  appears in `included_by` is a shared library, not an ingest target.
- `books[].files_not_enumerated` — files the generator deliberately skipped.
  **Read this list before planning cycles.** A kind you expected to find and
  did not is either genuinely absent or sitting in a skipped file, and those
  are different findings.
- `books[].trap_hits` — how many times each trap fires in this book. A book
  with a five-figure `mod_record` count needs a different ingest than one with
  none.
- `books[].reconciliation` — per-kind `corpus_units_total` vs `engine_records`
  vs `delta`, for books the engine already knows.

The inventory is idempotent by contract: *"two consecutive runs over an
unchanged corpus and engine differ only in `generated_at`."* If a rerun moves a
number, the corpus or the engine moved — investigate, do not re-baseline.

**Do not copy figures out of this playbook.** The illustrative contrasts below
were read from `docs/work-inventory.json` generated 2026-07-30 and are here
only to show the *span* of shapes you will encounter. Re-derive before use.

| Book | Dominant kinds (2026-07-30 generation) |
|---|---|
| `core_rulebook` | 2,319 equipment + 676 equipment modifiers, 1,512 class features, 664 spells, 185 feats, 28 classes |
| `bestiary` (B1) | 330 monsters, 620 race traits, 59 companions, 4 equipment |
| `ultimate_equipment` | 1,424 equipment + 190 equipment modifiers, 1 spell — nothing else |
| `bestiary_5` | 6 races, 119 race traits, 63 companions — **zero monsters** |
| `ultimate_campaign` | 23 feats — nothing else |

Whatever the shape says, write it down in the cycle's own scoping note with
the command that produced it. That is the artifact the next session reads.

### The shape decides the cycle

- **Monster-dominant** (`bestiary*`, `bonus_bestiary`) — per-monster-block
  cycles, and a monster surface. Note that as of this writing **no monster
  record reaches any player surface at all**; see `OPEN_FINDINGS` in
  `reach_gate.rs`. Ingesting a second bestiary without a monster browser
  reproduces the defect at four times the scale.
- **Equipment-dominant** (`ultimate_equipment`) — the equipment catalog is
  currently CRB-only in `equipment_catalog.rs`; widening it is a prerequisite,
  not a follow-on. Two entries in `OPEN_FINDINGS` already say so.
- **Class/feature-dominant** (`ultimate_combat`, `ultimate_magic`,
  `occult_adventures`, `advanced_race_guide`) — expect chooser-shaped content.
  Apply canonical narrowing (§7.5) rather than attempting the whole family.
- **Spell-dominant** — the spell catalog is already book-generic; the work is
  ingest plus a reach claim, and the `.COPY=` delta-row trap (§2, trap 11)
  is the one that bites, as the twelve pinned `BARE_RECORD_FINDINGS` show.
- **Thin books** (`ultimate_campaign`, `bonus_bestiary`) — a single cycle.
  Confirm with the inventory rather than assuming a book is large because it
  is a hardcover.

---

## 2. Step 1 — run the pre-ingest trap report

**Before you write a line of ingest code.** The report runs against a book
that has never been touched; it does not need the book to be ingested.

```sh
cargo run --locked --bin v06_corpus_trap_report -- <book_dir>            # human-readable
cargo run --locked --bin v06_corpus_trap_report -- <book_dir> --json     # for a diff between runs
cargo run --locked --bin v06_corpus_trap_report -- <book_dir> --examples 0  # every finding, not a sample
cargo run --locked --bin v06_corpus_trap_report -- --census <SearchString> # per-book counts, never a bare total
cargo run --locked --bin v06_corpus_trap_report -- --audit                 # cross-check already-ingested caches
```

`<book_dir>` is a directory name under `pathfinder/paizo/roleplaying_game`, or
an absolute path. `PCGEN_CORPUS_ROOT` overrides the corpus location.

Exit codes: `0` clean, `1` usage/IO failure, `2` when `--audit` finds a
`Severity::Defect` in already-ingested content. **A corpus scan never exits
non-zero for findings** — upstream corpus shape is data to handle, not a defect
to fail on. That distinction is the whole design: a `.MOD` row, a `#`-disabled
row, an archetype-qualified `KEY:` and a namespaced key are all *legitimate
data*. The defect is counting a `.MOD` as a declaration.

The trap catalogue and the corpus evidence for each entry live on
`src/pcgen_import/corpus_traps.rs`'s module doc. Read it once per book rather
than reading a summary of it here — several incident-report descriptions of
these traps were wrong and were corrected in that module, not in prose.

What the report gives you that a grep cannot:

- Per-file, per-line findings, so each is actionable rather than a count.
- The specific **miscount each trap produces** if handled naively —
  `Trap::miscount_risk()` is the sentence you need to act on the finding.
- `KEY:` namespaces per book with leaf counts, so a grep for a bare leaf name
  that returns zero does not read as "this content does not exist".

Record the report in the cycle's scoping note. It is the evidence that the
counts you later publish were derived rather than recalled.

---

## 3. Step 2 — ingest and surface are ONE unit of work

This is the dominant defect of the project. Six instances landed in a single
session — feats rendered as raw ids, 441 APG/ACG spells reaching no surface,
equipment computed then discarded at IPC, AC-by-source, the Pets tab, the
Weapons tab. Each was found by accident, patched individually, and the next one
appeared anyway. On its very first run the reach gate found four more.

`reach_gate.rs` makes it structural. You cannot ingest a book and defer its
surface, because the gate builds its inventory from two independent live
sources and a new book must defeat both:

1. **The app's own ingest diagnostic** — every `(book, kind)` pair
   `build_corpus_ingest_diagnostic()` reports with a non-zero count needs a
   reach claim.
2. **The record slices on disk** — every `pub const <NAME>: &[<RecordType>]`
   under `src/rules_core/rules_tables/`, scanned straight off the filesystem,
   so a family ingested but never wired into the diagnostic still shows up.

### What you must add, per book

- **A `reach_of` arm** for each `(book, kind)`, whose body calls the *real* IPC
  builder the Tauri command returns. A claim is executed, not documented — a
  doc comment cannot rot into a lie here, because the comment is not what the
  test reads.
- **A `RECORD_TYPE_KINDS` entry** if the book introduces a record type the gate
  does not know, mapping it to the surface that renders it; or a
  `SUPPORTING_RECORD_TYPES` entry with the reason it is a facet of an existing
  family. An unrecognized record type is a hard failure by design: a genuinely
  new kind of content needs a *decision* about where it reaches, not a default.

### What does not count as reach

- **A count does not.** `corpus_ingest_diagnostic` carries every book's record
  count to the player and renders none of the records. Treating that as reach
  would make the gate pass on all six historical defects, so it is disqualified
  by construction and `a_count_does_not_satisfy_the_gate` pins it.
- **An identifier alone does not.** A record that crosses the boundary carrying
  only its own key is the Feats-tab defect — the player saw
  `feat:deflect_arrows` where a name and description belonged. Every claim
  names the field(s) the render path actually reads, and `assess` rejects
  identity-only arrivals for *every* record, not most of them.

### If a family genuinely cannot be surfaced this cycle

Add an `OPEN_FINDINGS` entry stating the **remedy**, not an excuse. That list is
pinned in both directions: an unsurfaced family that is *not* listed fails, and
a listed family that someone *does* surface also fails until the entry is
deleted. It can only shrink without a deliberate, reviewable edit. Same rule
for individual bare records via `BARE_RECORD_FINDINGS`, pinned by exact key so
one record cannot silently swap for another.

An `OPEN_FINDINGS` entry is a work queue item and a bad cycle outcome. Prefer
shipping the surface.

### Scope boundary

The gate answers *"do this book's records reach a surface at all, carrying
something a player can read"*. It deliberately does not check that a surface is
correct, that every field crosses, or that a React component is mounted. Those
are other tests' jobs. Do not extend it into them — a gate that tried to prove
everything would be argued down to proving nothing.

---

## 4. Step 3 — verify with `scripts/verify.sh`, and nothing else

```sh
./scripts/verify.sh --help      # read this first, every time; it is the rationale
./scripts/verify.sh --list      # stages and which set each belongs to
./scripts/verify.sh             # full gate — slow, builds ~490 test binaries
./scripts/verify.sh --quick     # fast subset; NOT sufficient to close a cycle
./scripts/verify.sh --only reach
./scripts/verify.sh -j 2        # cargo parallelism; default 2
```

**Do not compose your own verification run.** Four distinct structural ways
this repo shipped broken while a hand-composed run reported green are
documented in `verify.sh --help`, and each has a corresponding guard in the
script. The short version of why hand-composition keeps failing:

- `apps/desktop/src-tauri` is a **separate cargo crate** — the repo root has no
  `[workspace]` table, so a root-level sweep never touches it, and it shipped
  un-compilable twice. It is also bin-only: `cargo test --lib` fails there
  outright. **`cargo test --workspace` from the root is not a whole-repo run.**
- `cargo test` fail-fasts. One failure meant 124 of 488 suites ran and the
  output still looked complete. `--no-fail-fast` is mandatory and the *number
  of suites executed* is checked, not just the summary line.
- Piping to `grep`/`tail` yields the **pipe's** exit status. That produced a
  false green on a full sweep that had failed. Capture exit codes directly:
  `cmd >log 2>&1; status=$?` on the very next line.
- The frontend runner reports `0/0 test files passed.` and exits `0` when
  `node_modules` is absent.

Per-class integration suites were once absent from the checklist and 34 failing
tests merged under sign-off. They are in `root-full` now. That is exactly why
the answer is "run the script", not "run the parts you think matter".

### Baselines

`scripts/verify-baselines.env` holds the recorded truth of what a green tree
produces. Test counts are **floors** (adding tests never fails the gate; losing
them does). Clippy warnings are a **ceiling**. When an actual exceeds a floor
the run still passes but prints a STALE notice naming the new number; updating
the file is then a deliberate one-line commit.

> **Never lower a floor to make a red tree green.** A floor that dropped means
> tests were deleted, which is the finding, not the fix.

Re-measure with `scripts/verify.sh --full --show-actuals`.

A book ingest normally raises `BASELINE_ROOT_LIB_TESTS`,
`BASELINE_ROOT_FULL_TESTS` and `BASELINE_ROOT_TEST_BINARIES`, and may raise
`BASELINE_DESKTOP_TESTS` when it lands a reach claim. Move those in their own
commit, with the `--show-actuals` output in the message.

---

## 5. Step 4 — definition of done

A book-ingest cycle is done when **all** of these hold. Each is checkable by
someone who was not there.

1. `./scripts/verify.sh` (full, not `--quick`) exits `0`. Exit code captured
   directly, never through a pipe.
2. The `reach` stage passes **with a claim for every one of this book's
   families**, not by the family being absent from the inventory. `reach_gate`
   failing with "0 tests matched the filter" is a hard failure — a gate running
   zero tests asserts nothing.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits `0`: no
   ingested record cites a corpus line that does not resolve.
4. `cargo run --locked --bin v06_work_inventory` regenerates
   `docs/work-inventory.json` and the book's units move out of `not-started`.
   Re-running it a second time changes only `generated_at`.
5. Every published figure in the cycle's receipt names the command that
   produced it (§6).
6. The four-check wired-integration audit in `./no-stub-mvp-doctrine.md`
   §"Per-cycle audit" is clean on the cycle's diff.
7. Any family that could not be surfaced has an `OPEN_FINDINGS` entry naming
   its remedy — and that is recorded as a cycle shortfall, not a pass.
8. Baseline movements in `scripts/verify-baselines.env`, if any, are a separate
   reviewable commit.
9. **If the cycle regenerated (or wrote for the first time) any book's
   `equipment/*.json` records, `cargo run --locked --bin
   enrich_equipment_raw_tokens` was re-run afterward, over the whole
   corpus, as a mandatory post-step — not a maybe.** Every book-specific
   equipment codegen pipeline diverged independently (`enrich_equipment_raw_tokens.rs`'s
   own module doc comment) and `raw_tokens`/`raw_bonus_chains` are not
   fields any generator populates itself; they exist on disk only because
   that tool adds them afterward, operating on raw `serde_json::Value` so
   it never drops a book-specific field a typed struct wouldn't recognize
   (a real, once-reverted defect: see that file's own history for why a
   typed-struct convergence was tried and abandoned). A generator run that
   is not followed by this step silently reverts every equipment record's
   `raw_tokens`/`raw_bonus_chains` to absent — this was found and fixed
   for `wiring_class`/license (GE-01, 2026-08-03) after regenerating
   through the generator dropped fields the generator itself has never
   known about; the same failure mode applies here and this item exists
   so it is never rediscovered the hard way for this field pair too.

---

## 6. Counting discipline

**Nearly every count in this project's history was wrong on first pass — the
lead's included.** The corrections are recorded in `corpus_traps.rs` and
`verify-baselines.env` because they are load-bearing:

| Claimed | Actual | Why |
|---|---|---|
| 396 missing feats | 301 | `.MOD` rows counted as declarations |
| 207 bonus-bearing feats | 166 | disabled `#` twins counted as live |
| 180 `BONUS:VAR` records | 86 | one record carried 66 tokens — tokens are not records |
| 186 CRB feats | 185 | off-by-one from a hand tally |
| 83 clippy warnings | 66 | summary lines counted as diagnostics |

Rules:

1. **Derive mechanically.** Every number in a receipt, scoping note or status
   surface names the command that produced it. A number without a command is a
   number nobody can re-check.
2. **Never quote a remembered count**, including one from this file, a prior
   receipt, or a dashboard. Re-derive at time of use — on a shared checkout an
   inventory decays silently.
3. **Count records, never tokens.** `magnitude_token_count` in the work
   inventory is reported per unit precisely so the two can never be conflated.
4. **A per-book subtotal is never a corpus total.** The inventory emits every
   count per book *and* aggregated, and `--census` exists for the same reason.
5. **2-of-3 corroboration for any published value.** Three independent
   derivations, or two plus the generator. This is the bar that caught most of
   the errors above.
6. **Read the whole corpus record.** A grep filtered to `BONUS:`/`PRE:` hides
   `STACK:`/`MULT:` and other application-governing fields. Trap 10
   (`GoverningTokenHiddenByFilter`) exists for exactly this.
7. **A shared name never implies a shared thing.** Join on `KEY:`, never on the
   display name (traps 3, 5, 7).

---

## 7. Practices that worked and must be preserved

These are not tool-enforced. They are what the tools were built out of, and
dropping them is how the next class of defect gets in.

### 7.1 Independent re-verification before every merge

A second agent re-derives the cycle's load-bearing claims from source, without
reading the first agent's working notes. In the session that produced this
playbook it caught a real error nearly every time it ran. Budget for it; it is
cheaper than the defect.

### 7.2 Agents are empowered — and expected — to correct the brief

Roughly ten briefs were corrected mid-flight in one session, and the correction
was right every time. If the brief's premise disagrees with the corpus or the
code, **the corpus and the code win**. Say so, in the cycle receipt, with the
evidence. Do not silently comply and do not silently deviate.

The corollary: re-check the finding that looks *good*. A diagnostic correctly
keeping something blocked is not the same as the feature working.

### 7.3 Honest deferral over fabrication

`work-inventory.json`'s own contract: *"It never invents a unit and never
invents a status. A record it cannot classify is emitted as `unknown` with the
reason attached, because an honest unknown beats a confident wrong entry."*
Apply the same rule to receipts and status surfaces. `deferred-with-reason`
carries the engine's own diagnostic message **verbatim**, never a re-narration.

Note the deliberate gap between `grounded` and `ingested-magnitude` in the
status vocabulary: the engine holding a record with its real numeric fields is
*strictly weaker* than a magnitude observed reaching a consumer. Calling the
first one "grounded" is the exact over-claim the inventory exists to prevent.

### 7.4 Mutation-test every new gate

A gate nobody has seen fail is a gate with no evidence it bites. `reach_gate.rs`
carries `a_count_does_not_satisfy_the_gate`, `identity_without_payload_is_not_reach`
and `records_missing_from_the_response_are_not_reach` — each drives the
assessment core with a synthetic input that *must* fail. Copy that pattern:
when you add a gate, add the test that proves it rejects the thing it exists to
reject.

### 7.5 Canonical narrowing for chooser-shaped content

For a large family of player-chosen options (rage powers, rogue talents,
bloodlines, hexes, exploits), ground **one representative record's real
magnitude end to end**, then name and defer the rest with a count. Precedents
are the `*-canonical-narrowing-scoping.md` documents under
`docs/release/v0.6/`. Pick the representative for the simplest formula that
still lands on a computed total, and check the family for selection
constraints before claiming leverage across classes.

### 7.6 Status comes from code, never prose

A dashboard claimed 12 finished classes when 5 was true. A coverage matrix read
1 wired feature where the code had 6. Four shipped deferral strings still claim
engines do not exist that do. Every one of those was a hand-maintained surface.

- Class/level state: `v06_class_state_dump` (`verify.sh` stage `class-dump`).
- Content state: `v06_content_state_dump`.
- Whole-corpus work state: `v06_work_inventory` → `docs/work-inventory.json`.

If a planning or reporting artifact needs a figure one of these generates,
**point at the generator** rather than transcribing the figure. A transcribed
figure is a figure that will be wrong later.

Shipped prose is not a source of truth either: diagnostic messages and doc
comments drift roster-wide. Derive remaining scope from live functions.

---

## 8. Scope boundaries this playbook assumes

- **Build no execution engines.** `docs/release/v0.6/execution-engine-scoping.md`
  is the verdict, with the evidence: not one of the 252 "no `<X>` engine exists"
  deferrals requires an execution engine for the correct number to reach the
  player. A book ingest never needs one. If a cycle's plan calls for RNG,
  opponent state or turn sequencing, the plan is wrong.
- **No stubs.** `./no-stub-mvp-doctrine.md` applies in full. An ingest that
  lands a picker with no handler, or fixture data in a production path, fails
  regardless of how much corpus it moved.
- **The corpus is never vendored.** It is an external checkout located by
  `PCGEN_CORPUS_ROOT`; corpus-gated tests skip gracefully when it is absent.
  See `../architecture/testing.md` §"Corpus-gated tests".

---

## 9. Where the mechanics live

This playbook deliberately does not restate the pipeline. Read the source of
truth for the layer you are touching:

| You are… | Read |
|---|---|
| adding a new LST record kind | `../architecture/corpus-ingest.md` §"Adding support for a new record kind" — the six files, in order |
| writing a book's `rules_tables` module | `../architecture/rules-data-tables.md` |
| generating a book cache | `src/bin/gen_cache_acg.rs` and siblings — the established shape |
| wiring compute | `../architecture/rules-engine.md` |
| adding a surface | `apps/desktop/src-tauri/src/spell_catalog.rs` + `apps/desktop/src/spellCatalog/` — the pattern `reach_gate.rs`'s open findings name as the remedy |
| writing a test | `../architecture/testing.md` |

## 10. Cross-reference

- `./no-stub-mvp-doctrine.md` — sibling doctrine; shipped code does what it claims.
- `./wired-integration-stubs-registry.md` — operator-granted stub exceptions.
- `../architecture/corpus-ingest.md` — the parse pipeline.
- `../architecture/rules-data-tables.md` — the downstream book modules.
- `../architecture/testing.md` — corpus-gated test conventions.
- `../release/v0.6/execution-engine-scoping.md` — the closed question on execution engines.
- `scripts/verify.sh`, `scripts/verify-baselines.env` — the verification contract.
- `src/pcgen_import/corpus_traps.rs`, `src/bin/v06_corpus_trap_report.rs` — the trap catalogue and its report.
- `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json` — the generated work inventory.
- `apps/desktop/src-tauri/src/reach_gate.rs` — the content-reach gate.
