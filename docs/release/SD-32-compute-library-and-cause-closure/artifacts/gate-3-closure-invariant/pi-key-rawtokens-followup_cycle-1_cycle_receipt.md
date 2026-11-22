# Cycle pi-key-rawtokens-followup — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `pi-key-rawtokens-followup`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `src/rules_core/pi_screening.rs` — `PI_BLACKLIST_TERMS` 60 → 61 (one new
    per-book term, the oracle's own lowercase-possessive typo of an
    already-blacklisted deity; see the array's own trailing-entry comment
    and `ogl-pi-blacklist.md`'s new per-book-override section — never
    spelled the term itself in this receipt), plus two new tests
    (mutation-proved) and a `term_list_len` assertion update.
  - `scripts/ingest_simple_filename_kinds.py` — (a) `scrub_blacklist_pi_tokens`
    (imported from `ingest_ability.py`, reused not reinvented) now runs
    unconditionally on every record's `raw_tokens`, not only description
    tokens; (b) the `always_pi`-gated branch split removed — every
    `name_is_pi` record across all six kinds this script serves (`template`,
    `power`, `domain`, `language`, `skill`, `deity`) now takes the
    `decisions.md §24` Codex-generated-neutral-name path; the legacy
    in-place-marker-substitution branch is gone.
  - `scripts/ingest_generic_kind.py` — same `scrub_blacklist_pi_tokens`
    reuse, applied unconditionally before the `name_is_pi` branch (covers
    `feat_generic`/`monster_generic`/every other `<kind>_generic` this
    script serves, going forward).
  - `docs/governance/ogl-pi-blacklist.md` — new per-book-override section
    (Inner Sea Gods, equipment) documenting the new term's provenance and
    verification, per the existing template two prior additions used.
  - `scripts/tests/test_pi_key_rawtokens_defect1_regen.py` (new) — Defect 1
    regression: the 3 real fixed records carry no blacklist hit (mutation-
    proved against `git show HEAD`, which still carries the leak), plus the
    4th originally-reported record (`spell`) proved a false positive.
  - `scripts/tests/test_declared_pi_shipping_defect2_regen.py` (new) —
    Defect 2 regression: the 28 originally-violating file paths no longer
    exist (moved to `codex_named_unit_*` siblings), plus an end-to-end
    `cargo run --bin declared_pi_shipping_audit` zero-`NAME-PI-SHIPPED` proof
    (scoped to this defect's own violation shape, not the audit's overall
    verdict — see the Defect 2 notes below for why).
  - `data/corpus/inner_sea_gods/equipment/wayfinder_of_zephyrs.json` —
    regenerated (deleted + `gen_cache_equipment_gap` + `enrich_equipment_
    raw_tokens`, both guarded generator paths) with the description and
    `raw_tokens[DESC]` now `[redacted PI]`.
  - `data/corpus/**/domain/**`, `data/corpus/**/language/**`,
    `data/corpus/**/template/**` — full regen via
    `scripts/ingest_simple_filename_kinds.py --kind domain --kind language
    --kind template` (2,567 records: 183 + 136 + 2,248). 60 records whose
    declared/blacklisted name previously shipped as the in-place
    `[redacted PI]` marker are now `codex_named_unit_*` siblings under a
    §24 neutral name; the 60 old marker-shaped files were removed (`git rm`)
    as orphans of the rename, not hand-edited. All other regenerated files
    in this population changed only `ingested_at` and picked up the
    `codex_generated_name`/`rename` schema fields already landed by the
    pinned base commit but never regenerated onto them — confirmed by
    diffing every changed file's content, not assumed.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to my own working-
  tree diff of the 6 touched source/governance/test files above, not the
  full `BASE_BRANCH...HEAD` form). The handful of raw matches are all
  pre-existing shared-module names / wiring-class-signal strings already
  used identically elsewhere in the repo before this cycle.
- **Wired-integration audit result:** `OK_NO_TOKENS` on the same scoped diff.
- **Acceptance criterion:** dispatch brief's two named defects — (1) the 4
  cross-kind PI leaks under the signed-off 60-term blacklist
  (`pi-key-rawtokens-screen`'s own deferred finding); (2) the 28
  pre-existing `NAME-PI-SHIPPED` violations in `language`/`template`
  (`cargo run --bin declared_pi_shipping_audit`).
- **Corpus SHA:** PCGen oracle pin `7f818006e371188e5717fd18d74d18a420747fc6`
  (bootstrapped fresh this cycle — worktree's oracle slot was empty).
- **Status:** complete
- **Notes:**

  ## Defect 1 — re-derived per `§17a`: 3 of the 4 named leaks were real; 1 was a false positive; 9 more (new kinds) discovered

  Re-ran `python3 scripts/pi_key_rawtokens_audit.py` fresh (25,653 records
  scanned, up from the prior report's 24,051 — more kinds/books landed by
  sibling lanes since). Result before any fix: **confirmed=13**, not 4:

  | Kind | Book | Record | Disposition |
  |---|---|---|---|
  | `domain` | `core_rulebook` | `death.json` | **real leak — FIXED** (`PREDEITY:1,<deity>` inside a `SPELLLEVEL` raw_token, not screened before this cycle) |
  | `equipment` | `inner_sea_gods` | `wayfinder_of_zephyrs.json` | **real leak — FIXED** (oracle's own lowercase-possessive deity typo, case-sensitive scan missed it) |
  | `language` | `inner_sea_temples` | `nightsong.json` | **real leak — FIXED** (`PREDEITY:1,<deity>` raw_token, same shape as `domain`) |
  | `spell` | `advanced_players_guide` | `bard_s_escape.json` | **FALSE POSITIVE, not fixed, not a real leak** — see below |
  | `feat_generic` (7), `monster_generic` (2) | `adventurers_guide`, `inner_sea_bestiary` | 9 records, named in the committed test/audit output | **real leaks, confirmed by direct literal grep against the pinned oracle — NOT fixed this cycle, reported for a follow-up** (see "Discovery forwards") |

  **`spell/bard_s_escape.json` is a false positive of the audit instrument
  itself, not a real PI leak** — `§17a` in direct action. The flagged
  blacklist term does not appear anywhere in the record's actual bytes. The
  hit is an artifact of `normalized_term_hit`'s OCR-confusion fold (the
  `rn`→`m` substitution `decisions.md §19a`/`ogl-pi-blacklist.md §2.3a`
  mandates) canonicalizing the flagged term to the same string as an
  ordinary English word that appears in this record's genuine OGL prose
  (an idiom about being "in a tight [that word]"). Confirmed live:
  `canonicalize(term)` and the record's own description collide under the
  fold; the term is provably absent from the record byte-for-byte. Not
  fixed (nothing to fix — the record is correctly `OGL`), not touched.
  **This is a real defect in the shared `rn`→`m` fold itself** (it can
  produce false REDACTIONS on unrelated content elsewhere, not just false
  audit alarms) — out of this cycle's scope to change (the fold is
  `decisions.md §19a`'s own approved, tested scheme, used across every kind
  that scans `normalized_term_hit`), reported as a discovery forward for an
  operator ruling or a follow-up cycle, per `AGENTS.md`'s instrument-
  validation rule.

  **9 new confirmed leaks discovered, not fixed this cycle**:
  `feat_generic` (7, all `adventurers_guide`) and `monster_generic` (2, both
  `inner_sea_bestiary`) — real, literal, exact-case substring matches
  against the pinned oracle (verified by direct `grep`, not just the
  normalized scanner). These are NEW kinds relative to the original 4-record
  report (ingested by sibling `no_record`-closure lanes between that report
  and this cycle). `scripts/ingest_generic_kind.py`'s cause-level fix
  (`scrub_blacklist_pi_tokens` now runs unconditionally) is landed and will
  prevent this shape in every FUTURE `ingest_generic_kind.py` run, but
  `ingest_generic_kind.py`'s writer is gated on `no_record` status via a
  ledger join — it cannot re-touch an ALREADY-ingested record the way
  `gen_cache_equipment_gap`'s simpler "skip if file exists" guard let this
  cycle force a targeted re-ingest for the single equipment leak. Forcing a
  re-ingest of these 9 would need either a fresh `shape_ledger.py` run
  reflecting a deletion of the 9 files, or a small standalone script — both
  judged out of this cycle's time budget and better done as their own
  focused follow-up (`decisions.md §15`: land everything else, report by
  name, never silently skip). Named in full in the committed test file's
  own `ORIGINAL_28`-style enumeration is NOT how these are recorded (that
  list is Defect 2's); they are named here and in the discovery-forward
  event below.

  **Zero leaks remain for `domain`/`equipment`/`language`** (re-run,
  `scanned_records: 25653`, `confirmed.by_kind`: no `domain`/`equipment`/
  `language` key at all — command: `python3 scripts/pi_key_rawtokens_audit.py
  --json-out <path>`).

  ## Defect 2 — root-caused and closed, all 28 (zero `NAME-PI-SHIPPED` violations remain)

  Root cause: `scripts/ingest_simple_filename_kinds.py` served six kinds
  but only `deity` (`NAME_ALWAYS_PI_KINDS`) went through `decisions.md §24`'s
  neutral-name path for a declared/blacklisted name. The other five kinds
  (`template`, `power`, `domain`, `language`, `skill`) fell through a legacy
  pre-`§24` branch that replaced `name`/`key` with the literal
  `REDACTED_PI_MARKER` string **in place** — a shape
  `declared_pi_shipping_audit.rs`'s own check rejects outright (a key/name's
  mere presence on disk, even marker-redacted, is still the violation;
  `§24b`-3's own reasoning). **Not** a gap in `ingest_simple_filename_
  kinds.py`'s PI *detection* (the coordination note's "deity exclusion
  lifted recently" hypothesis) — detection was already correct; the
  *remediation shape* for 5 of 6 kinds was simply the wrong one for current
  policy.

  Fix: removed the `always_pi`-gated branch split. Every `name_is_pi`
  record, across all six kinds, now takes the single `§24` neutral-name
  path — reusing `scripts/codex_neutral_name.py` and `scrub_name_pi_tokens`
  exactly as `deity`/`ability`/`class_feature` already do, per the dispatch
  brief's explicit instruction not to invent a second scheme.

  Regenerated `domain`+`language`+`template` (full population, 2,567
  records). 60 records were declared/blacklisted-name PI; each now ships as
  a `codex_named_unit_*` sibling with `codex_generated_name: true`. The 60
  OLD marker-shaped files (different filename — the neutral name changes
  the output slug) were left as orphans by the regen; found and removed via
  `git rm` (never hand-edited — this is cleanup of files the SAME guarded
  generator run superseded, not a manual data edit) once discovered by a
  first-pass `cargo run --bin declared_pi_shipping_audit` still failing
  after the regen.

  **Verified: `cargo run --bin declared_pi_shipping_audit` reports zero
  `NAME-PI-SHIPPED` violations** (all 28 originally-named + a re-scan for
  any new ones — none). **Re-derived after this cycle's post-launch rebase
  onto `origin/tranche/12`, not assumed stable across it**: the rebase
  picked up sibling-lane commits that landed a DIFFERENT, unrelated
  violation shape (`DESC-PI-SHIPPED-IN-RAW-TOKENS`, 82 instances across
  `ability`/`feat_generic`/`race_trait_generic` — a record's `data.
  description` is correctly redacted but `data.raw_tokens`' own `DESC`
  entry still carries the real prose). **Confirmed pre-existing and out of
  this cycle's scope**: `git show bd6e0b6968:<one flagged file>` (the
  `origin/tranche/12` tip immediately before this cycle's rebase) shows the
  identical leak already present, and that commit's own message
  (`e5c53a6ab0`) already records this as a known, separately-discovered
  defect. This cycle never touches `ability`/`feat_generic`/
  `race_trait_generic`'s generators. `scripts/tests/test_declared_pi_
  shipping_defect2_regen.py`'s audit test asserts `NAME-PI-SHIPPED` absence
  specifically, not overall `CLEAN`, so it does not go red on this
  unrelated, concurrently-landed defect.

  ## `no_record` — unaffected, by kind, before and after

  This cycle's own count command:
  `find data/corpus -type d -name domain -exec find {} -name '*.json' \; | wc -l`
  (and the `language`/`template` equivalents) — 183 / 136 / 2,248 after,
  matching the `seen`/`written` totals the regen itself reported (both
  before and after: same 183/136/2,248 population, `citation_mismatches: []`,
  `unresolved: []`). No unit moved kind, no unit was created or deleted —
  every touched record kept its `(book, source_file, source_line)`
  coordinate, which is `shape_ledger.py`'s join key, not the filename. The
  60 §24 renames change only the on-disk slug, never the coordinate.
  **`no_record` is unmoved by this cycle for `domain`/`language`/`template`/
  `equipment`.**

  ## Regeneration safety

  `data/corpus/**` only — `CORPUS_LITERAL_SWEEP_REPORT`/
  `DERIVED_FIXTURE_CHECK_REPORT` are consumed only by `v06_work_inventory`,
  which this cycle never ran (matches the precedent this cycle's own
  predecessor receipt recorded). No `--allow-stamp-loss` flag exists on any
  generator this cycle ran. Diffed `git status --porcelain data/corpus`
  before/after every regen step (0 → 1 for the equipment fix, then a full
  domain+language+template population diff for Defect 2 — every changed
  file's content was inspected, not assumed, and the git-rm'd files were
  independently confirmed to be exact-count orphans of the 60 renames, not
  a guess).

  ## Own-diff PI scrub

  Grepped every new/modified source, test, and governance-doc line (working-
  tree diff, `+`-added lines only) against all 60(+1) `PI_BLACKLIST_TERMS`
  before pushing. Found and fixed 2 real leaks in my OWN prose (this
  receipt's own predecessor pattern, repeated): a deity name and a 3-letter
  term written out literally in explanatory comments in
  `src/rules_core/pi_screening.rs` and `docs/governance/ogl-pi-blacklist.md`
  — both rewritten to reference the array by index/coordinate instead of
  spelling the term, before committing. Re-ran the grep after the fix:
  `CLEAN`. This receipt itself and the two new test files were grepped and
  are clean.

  ## Gate 3's budget constants — untouched.
- **Discovery forwards:**
  - 9 new confirmed leaks (`feat_generic` 7 in `adventurers_guide`,
    `monster_generic` 2 in `inner_sea_bestiary`) discovered by this cycle's
    corpus-wide re-scan, real per direct literal grep, **not fixed** —
    `scripts/ingest_generic_kind.py`'s writer is `no_record`-ledger-gated
    and cannot re-touch already-shipped records the way this cycle's
    equipment fix did; a follow-up needs either a `shape_ledger.py` re-run
    reflecting a deletion of these 9 files or a small standalone re-ingest
    script.
  - The `normalized_term_hit` `rn`→`m` OCR-confusion fold (`decisions.md
    §19a`, `ogl-pi-blacklist.md §2.3a`) has a proven false-positive/false-
    redaction risk: it can canonicalize a blacklist term to the same string
    as an ordinary English word (confirmed live against `spell/bard_s_
    escape.json`). This is the SAME shared scan used by every kind's
    `normalized_term_hit` caller — the false-positive direction was caught
    here (an audit alarm with no real leak), but the SAME collision could
    fire as a false REDACTION somewhere the fold happens to match real,
    innocent prose. Reported for an operator ruling on whether the fold
    needs a narrower confusion table or a different disambiguation; not
    touched this cycle (`§19a`'s scheme is operator-approved and used
    corpus-wide — changing it is a bigger blast radius than this cycle's
    named scope).
  - **Pre-existing, unrelated `DESC-PI-SHIPPED-IN-RAW-TOKENS` (82 instances,
    `ability`/`feat_generic`/`race_trait_generic`)** surfaced by the post-
    dispatch rebase onto `origin/tranche/12` — already present at that
    tip's `bd6e0b6968` (confirmed via `git show`) and already recorded by
    that lineage's own `e5c53a6ab0` commit as a known, separately-discovered
    defect. Not this cycle's defect shape (`data.description` is correctly
    redacted; `data.raw_tokens`'s own `DESC` entry is not), not touched, not
    double-logged here beyond this cross-reference — the owning lane's own
    commit is the record of it.
- **Next-cycle plan:** a follow-up lane picks up the 9 named `feat_generic`/
  `monster_generic` leaks (needs a `no_record`-ledger-aware re-ingest path,
  not just the generator fix already landed) and separately an operator
  ruling on the `normalized_term_hit` OCR-fold false-positive risk.
