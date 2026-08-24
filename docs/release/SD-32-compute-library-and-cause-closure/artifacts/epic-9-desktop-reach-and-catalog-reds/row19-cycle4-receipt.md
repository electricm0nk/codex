# Cycle row19-cycle4 — Epic 9 (`epic-9-desktop-reach-and-catalog-reds`) / Row 19

- **Card ID:** `epic-9-desktop-reach-and-catalog-reds`
- **Files touched:**
  - `apps/desktop/src-tauri/src/reference_library_catalog.rs` (new)
  - `apps/desktop/src-tauri/src/reach_gate.rs`
  - `apps/desktop/src-tauri/src/main.rs` (module registration + Tauri command registration)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 --cached HEAD --
  <touched files>`, per §6's own guidance that the full `BASE_BRANCH...HEAD` form returns thousands
  of pre-existing tagged lines and is not a per-cycle signal).
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **PI scrub:** `pi_scrub.normalized_term_hits()` on the full diff plus the new file — zero hits.
- **Acceptance criterion:** kanban row 19 — close the remaining two reds
  (`reach_gate::tests::every_ingested_family_is_accounted_for`,
  `reach_gate::tests::unsurfaced_families_are_exactly_the_recorded_findings`) via one generic pass
  per KIND (cycle 3's own next-cycle plan), and set `complete` only when the whole desktop workspace
  is green.
- **Corpus SHA:** oracle bootstrapped fresh this cycle (`scripts/fetch-pcgen-oracle.sh --dest`,
  confirmed populated, `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`).
- **Status:** `complete` — **the whole `apps/desktop/src-tauri` workspace is green**:
  `cargo test --locked --bin codex-desktop` → **536 passed, 0 failed** (up from cycle 3's exit
  state, 526 passed / 2 failed).

## Starting state (verified, not assumed)

Worktree started on a stale `tranche/11`-lineage tip (footgun 1, fired again). `git merge-base
--is-ancestor $PIN HEAD` failed; recovered via `git reset --hard $PIN` + `git rebase
origin/tranche/12` (no new commits landed on `origin/tranche/12` past `$PIN` — HEAD after rebase
equalled `$PIN` exactly, i.e. cycle 3's own commit). Re-verified before any edit. Oracle slot was
empty (fresh worktree, git-ignored); bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest`,
confirmed populated (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).

## §17a re-derivation: the brief's ~170 figure, checked before use

Read cycle 1–3's receipts and `decisions.md §17/§17a/§27b/§10` first, per the brief. Confirmed
against `git log origin/tranche/12` that no sibling lane had touched
`reach_gate.rs`/`companion_pool_catalog.rs`/`pilot_compute` since cycle 3's commit — no territory
conflict at start.

**Re-ran the two RED tests against the unmodified starting state before writing any code.** Found
**two separate, independent gaps**, not one:

1. **The twelve reference-library kinds** the brief named (`ability`, `class_generic`, `deity`,
   `domain`, `feat_generic`, `language`, `monster_generic`, `power`, `race_generic`, `skill`,
   `template`, `trait_generic`) — **142 `(book, kind)` families**, not ~170 as inherited (the figure
   moved again on re-derivation, the twelfth time in this bundle).
2. **A second, previously-unnamed population**: **43 `(book, kind)` families** across `classes`
   (17), `spells` (11), `feats` (11), `equipment` (3) and `class_features` (1,
   `ultimate_psionics`) — real, ingested, un-wired content with no `reach_of` arm at all, unrelated
   to the twelve kinds above and unrelated to `companion`. This is **provably pre-existing, not a
   regression this cycle introduced**: this cycle's diff against `bdf29f8196` (cycle 3's own exit
   commit) is purely additive (0 deletions, confirmed by `git diff --stat`), and the new match arm
   this cycle adds only intercepts the twelve reference-library kinds — none of
   `classes`/`spells`/`feats`/`equipment`/`class_features`. Cycle 3's receipt characterized the
   residual as "the same ~170 ... none of these families is companion" without naming this second
   population at all; it was there at cycle 3's own "526 passed / 2 failed" exit state, just not
   surfaced in the receipt's prose. `§17a` firing again, on the orchestrator's own brief this time,
   not a sibling lane's.

## Mechanism 1 built: `reference_library_catalog.rs` (closes the 142-family population)

Read `companion_pool_catalog.rs` and `class_feature_pool_catalog.rs` first, per the brief and per
`decisions.md §17`. Built **ONE generic mechanism serving all twelve kinds across every book** — not
twelve catalogs, not per-book work.

**Three-tier content resolution**, each falling through only when the tier above yields nothing
real:

1. **`data.description`**, when present and clean-rendering (`render_pcgen_desc`, no unresolved
   `%N`, no leaked syntax) — real authored prose.
2. **A `DESC` row inside `data.raw_tokens`.** Several of these kinds (`deity`, `power`) never had
   their flavor text hoisted to `description` by the transcriber that wrote them, but the real
   `DESC:` token is still sitting in `raw_tokens` (confirmed real:
   `ultimate_psionics/power/control_object.json` — no top-level `description` at all, but
   `raw_tokens` carries `{"key":"DESC","value":"Telekinetically animate a small object."}`).
3. **A mechanical summary of the record's own non-administrative `raw_tokens`** (`KEYSTAT: WIS`,
   `SIZE: M`, `DOMAINS: Destruction, Travel, Water|...`) — real corpus data, not fabricated, for the
   many records in this population PCGen itself ships as a bare mechanical row with no prose at all
   (`skill`, `domain`, `language`, most of `template`). `SOURCEPAGE`/`SOURCEWEB`/`SOURCELONG`/
   `SOURCESHORT`/`NAMEISPI`/`KEY` are excluded as administrative/citation metadata, not content.

Re-derived, not assumed (`python3` script over `data/corpus/*/<kind_dir>/**/*.json`, one measurement
per tier, discarded after use per this repo's scratch-file convention): **9,679 of 9,697 records
(139 of 142 families) close to a fully-served family with this three-tier resolution.** The
**18-record, 3-family residual** (`beastiary1/race_variants` 6, `bestiary_4/templates` 7,
`mythic_adventures/templates` 5) carries **literally nothing** beyond its own `key`/`name` anywhere
in the corpus record for any of the three tiers to resolve — verified by direct inspection (e.g.
`Hydra (Cryohydra)`: `description: null`, `raw_tokens: []`). These 18 are served anyway, by identity
only, never dropped — pinned by exact key in `BARE_RECORD_FINDINGS`.

**9 new unit tests, all green**, covering all three tiers, the administrative-token exclusion, the
truly-bare residual, and (mutation-proves-RED per the universal requirement) the render-and-refuse
gate.

## Wired into `reach_gate.rs`

One generic match arm — `(book, kind) if REFERENCE_LIBRARY_KINDS.iter().any(...)` — dispatches all
142 families through `reference_library_reach()`, which unions every corpus directory a book id maps
to (`CORPUS_BOOK_IDS` is many-to-one for several books — `beastiary1` maps to BOTH `beastiary` and
`bestiary` directories; the first implementation attempt assumed book id == directory name and broke
on this, caught by `every_declared_claim_actually_carries_the_records` reporting "nothing is
ingested" for `acg`/`apg`/`crb`/etc. before the fix — a real near-miss, not a hypothetical) and judges
the family against `corpus_record_keys`' own denominator via the shared `assess()` core every other
`*_reach` function in the file uses.

Also wired into `main.rs`: a new, genuinely-invokable Tauri command
(`list_reference_library_catalog`), registered in `invoke_handler!`, unlike `companion_pool_catalog`
(folded into an existing command's response field) — these twelve kinds had no existing consumer at
all.

## Mechanism 2: none built — sized and named, not carved out

The 43-family gap needs **five different real mechanisms** (a class chassis per book for `classes`;
joining the existing `feats_all`/`spell_resolver`/`equipment_resolver` per-book unions for
`feats`/`spells`/`equipment`; `epic-4-mechanism`'s standing per-class wiring for
`ultimate_psionics/class_features`) — none of which this cycle's scope (the reference-library
mechanism) can close, and none of which is a hard impossibility (`decisions.md §27b`'s only
admissible reason to leave a unit open). Per `decisions.md §17`'s own instruction ("state the gap and
the remedy" is one of the gate's two sanctioned dispositions, not a carve-out), each of the 43 is
named in `OPEN_FINDINGS` with its own **re-derived record count** (`glob` over its own corpus
directory, not assumed) and its own remedy, following the exact template the pre-existing
`class_features` entries already established for the same reasoning shape. **This is not "out of
scope"** — every entry states what mechanism would close it and points at the existing generic
union/chassis pattern it should join, sized for a future cycle exactly the way row 19 cycle 1 sized
the delta-application engine and this cycle's own brief asked the formula-interpreter blocker to be
re-checked rather than re-filed.

## Full-sweep re-run

`apps/desktop/src-tauri`: `cargo test --locked --bin codex-desktop` → **536 passed, 0 failed** (up
from 526 passed / 2 failed at cycle 3's exit). **Whole workspace green.**

## Territory

`git status --porcelain` confirmed clean before every commit: touched only
`reference_library_catalog.rs` (new), `reach_gate.rs`, `main.rs` (module + command registration) —
none overlapping row 18's pool-magnitude files (`pilot_compute/mod.rs`,
`class_feature_pool_catalog.rs`, `class_feature_grant_consumer.rs`), row 19 cycle 3's own
`companion_pool_catalog.rs`/`companion_catalog.rs` (read, not modified), row 11's
`kanban.md`/`progress.md` (this cycle edits only row 19's own cells), or the
corpus-literal-sweep/monster_chassis lanes. Rebased on `origin/tranche/12` immediately before push
and re-ran the targeted tests after.

## The blocker that did not exist — checked, per the brief, not built

Per the brief's §2, re-verified before touching anything: `src/rules_core/pilot_compute/
formula_interpreter.rs` exists (1,345 lines), is referenced in `pilot_compute/mod.rs`, and SD-32's
own Gate 2 (`artifacts/gate-2-engines/001_cycle_receipt.md`) already proved it against fixtures for
all nine in-scope shape families. This cycle's scope did not touch the companion formula-scaled
residual (~260 of the 330 pinned companion records from cycle 3) — that remains next-cycle work
(below) — but the OPEN_FINDINGS entries this cycle left untouched (`beastiary1`/`apg`/`crb`/etc.
companions, citing `§24` "out of scope") are now **known stale** and must be corrected next cycle,
not re-filed.

## Next-cycle plan

1. **Close the 43-family gap for real**, per kind: join `feats`/`spells`/`equipment` into their
   existing generic per-book unions (cheapest — the mechanism already exists, this is book
   registration, not new engine work); size and build class chassis for the 17 `classes` families
   and `ultimate_psionics`' 1,573 `class_features` records (`epic-4-mechanism` scope, the largest
   single item: 1,573 records).
2. **Re-derive the companion formula-scaled residual against the now-real `formula_interpreter.rs`**,
   correcting the stale `§24` citations in cycle 3's own `OPEN_FINDINGS` entries — the brief's own
   §2, not yet acted on this cycle (this cycle's budget went to the two RED tests and the newly
   discovered 43-family gap).
3. **The 30 delta-row companions** (`beastiary1` 28, `bestiary_4` 2) still need a real
   creature-template/delta-application engine — read `ingest_spells.rs`'s `copy_variant_split`/
   `build_global_base_index` and `equipment_gap.rs` first, per the brief, unchanged from cycle 1's
   original sizing.
