# Cycle epic-2-t2b-w1b-inner_sea_races — Gate 3 closure invariant / Card 11 T2b (Epic 2)

- **Card ID:** `epic-2-cause-closure` (row 11; shape T2b, book `inner_sea_races`)
- **Actor:** `t2b-w1-b`
- **Commit SHA:** (this cycle's commit — see push log)
- **Files touched:**
  - `src/bin/ingest_race_traits.rs` (count-pinning test: `inner_sea_races` 82 -> 94, total 544 -> 556)
  - `apps/desktop/src-tauri/src/reach_gate.rs` (reach test + `UNREACHED_RECORD_FINDINGS` +
    `OPEN_FINDINGS` updated for the 12 newly-ingested records and the 2 of them that are
    genuinely unreached)
  - `data/corpus/inner_sea_races/race_trait/{catfolk,gillman,kitsune,nagaji,ratfolk,strix,suli,
    vanara,vishkanya,wayang}/*.json` (12 new records, generated via
    `cargo run --bin ingest_race_traits -- inner_sea_races` against the pinned oracle, never
    hand-edited)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own-diff scope; see Notes on the base-branch
  command's false positive)
- **Wired-integration audit result:** `OK_NO_TOKENS` (own-diff scope; see Notes)
- **Acceptance criterion:** AT-32-E2-001 (cause closure closes by class) / AT-32-E4-001 (book
  onboarding) — `acceptance-and-verification.md`. This book was already in `RACE_CORPUS_BOOKS`;
  scope was ingest-tool extension only, per the dispatch brief.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`)
- **Status:** complete (partial closure — see Notes for the honest remainder)
- **Notes:**

  **Real root cause (correction to the census memo, `decisions.md §13`'s table and
  `card11-t2b-census-census.md §2`):** the memo characterized `inner_sea_races`'s 59 open units
  as "never-transcribed per-record content... same shape as T9" needing per-record onboarding.
  Re-derivation shows the true shape for most of them is different: `ingest_race_traits.rs`'s
  `IN_SCOPE_RACES` roster grew from 18 to 34 races across three SD-31 waves (2026-08-15 to
  2026-08-17), but nobody re-ran `cargo run --bin ingest_race_traits -- inner_sea_races` after
  those widenings, so 10 races that had *already* become in-scope (Catfolk, Gillman, Kitsune,
  Nagaji, Ratfolk, Strix, Suli, Vanara, Vishkanya, Wayang) sat un-transcribed on disk — a **stale
  regen**, not a content-onboarding gap and not a new mechanism. Logged:
  `scripts/retro.py correction --subject t2b-census --claimed "inner_sea_races: 59 never-
  transcribed per-record units, ingest-tool extension" --actual "13 of the 59 close by simply
  re-running the existing ingest binary (stale regen); the other 46 need a race chassis this
  project has not built or a heritage-selector mechanism it has deliberately deferred" --verified-by
  "ls data/corpus/inner_sea_races/race_trait/ before/after cargo run --bin ingest_race_traits --
  inner_sea_races"`.

  **Units closed this cycle: 12** (of the census's 13-unit prediction for these 10 races — see
  below). Re-running the existing, unmodified ingest binary against the pinned oracle
  (`cargo run --bin ingest_race_traits -- inner_sea_races`) wrote 12 new records:
  Catfolk 1, Gillman 1, Kitsune 1, Nagaji 1, Ratfolk 1, Strix 1, Suli 2, Vanara 1, Vishkanya 2,
  Wayang 1. **Re-derive:**
  `find data/corpus/inner_sea_races/race_trait -name '*.json' | wc -l` -> 94 (was 82).
  The 83 other on-disk records the same run touched only had their `ingested_at` timestamp
  refreshed (byte-identical `data`); those timestamp-only diffs were reverted
  (`git checkout -- data/corpus/inner_sea_races`) before the 12 new race directories were
  re-copied back in, to keep this commit's diff to real content only.

  **1 unit correctly NOT work** (confirms the census's own header-exclusion discipline extends
  one step further than it documented): `Svirfneblin ~ Stalwart Watcher Output`
  (`isr_abilities_race.lst:1297`) is PCGen's own internal `ABILITY:...|AUTOMATIC|...` companion
  token for the real, already-ingested trait `Stalwart Watcher` — its row's `TYPE:Special Attack`
  never matches `parse_row`'s `<Race> Racial Trait`/`Racial Default` suffix gate, by design, the
  same way the 147 category-header rows across the corpus are excluded. Not a second
  player-facing object; zero code change needed or made.

  **2 of the 12 newly-ingested records are themselves genuinely unreached** (not a defect in this
  cycle's work — a pre-existing upstream/mechanism gap the reach gate is built to surface, not
  paper over, per `AGENTS.md` "a magnitude is not wired until it moves on the twin the player
  reads"):
  - `Mostly Human ~ Suli ~ Languages` — same unmodelled-Geneiekin-heritage gap as its
    already-open Ifrit/Sylph/Undine siblings (`isr_abilities_race.lst:654`'s
    `Geneiekin ~ Mostly Human.MOD` granter carries a race-scoped TYPE this project's `is_mod_row`
    guard correctly excludes from ingestion).
  - `Suli ~ Trusted Mediator` — a genuine upstream PCGen data omission: unlike its structurally
    identical siblings (`Ifrit ~ Brazen Flame`, `Oread ~ Isolated`, `Sylph ~ Secretive`, each of
    which sets a matching `FACT:<Race>_Replace...|True` pair), this row
    (`isr_abilities_race.lst:1266`) sets no `FACT:` token at all, despite its own DESC prose
    claiming it replaces energy resistance and low-light vision. Verified against the pinned
    oracle, not assumed.

  Both are recorded by name in `reach_gate.rs`'s `UNREACHED_RECORD_FINDINGS`/`OPEN_FINDINGS`
  (same discipline as the pre-existing `Human ~ Tribalistic Languages` finding), not silently
  dropped or force-reached.

  **Honest remainder — 45 of the census's 59 units are NOT closed by this cycle, and the
  count is corrected from the memo's "same shape as T9" characterization.** They belong to races
  with no chassis this project has built at all (Android, Changeling, Dhampir + its 4
  hag-mother/vampire-progenitor subrace families — Jiang-shi-Born, Moroi-Born, Nosferatu-Born,
  Vetala-Born — Gathlain, Geneiekin, Ghoran, Kasatha, Lashunta, Samsaran, Skinwalker (in this
  book's context), Syrinx, Triaxian, Trox, Wyrwood, Wyvaran) or to a heritage-selector mechanism
  this project has explicitly and repeatedly deferred elsewhere (Changeling/Dhampir/Skinwalker
  set their `Replace*` FACT flags directly on subrace-selector rows via a `PREMULT` gate, not
  through a `_globalvar_subrace.lst` file `subrace_grants()` can read — the identical shape
  `ingest_races.rs`'s own `skinwalker` doc comment names as "a genuinely new mechanism, deferred
  (not stubbed) to a follow-on batch"). Building either is **chassis-load wiring**, which the
  dispatch brief explicitly places out of this lane's scope ("no registration, no chassis-load
  wiring needed"). Doing it here would also violate `decisions.md §1a`/no-stub doctrine in
  spirit: a race added to `IN_SCOPE_RACES` with no chassis produces records
  `RaceCorpus::chassis()` never populates — "loaded but permanently unreachable," per this same
  file's own module doc.
  **This is a blocker per `AGENTS.md` Blocker Discipline, not a deferral**: it was inside
  AT-32-E2-001's Definition of Done at launch. Escalating: closing these 45 units needs either
  (a) an operator ruling widening this lane's scope to include chassis wiring for the 15 races
  named above, sequenced as its own cycle(s), or (b) a dedicated follow-on cycle scoped
  specifically to the Dhampir/Changeling/Skinwalker heritage-selector mechanism (which alone
  would close most of the Dhampir-family units: Dhampir Default/Heir to Undying Nobility/Vampire
  Hunter + the 4 subrace families' 4 units each = 7 + 16 = 23 of the 45).

- **Discovery forwards:** the stale-regen finding above likely explains part of `bestiary_2`'s and
  `advanced_race_guide`'s open T2b units too (both books' `IN_SCOPE_RACES` widened across the same
  SD-31 waves) — worth a sibling lane checking `git log --follow` on those books' `race_trait/`
  directories against their own `IN_SCOPE_RACES` widening dates before assuming their remainder is
  all net-new onboarding.
- **Next-cycle plan:** `bestiary_5` next (this lane), then `horror_adventures` (receipt-only, see
  its own cycle receipt — 0 code units, a census correction).
