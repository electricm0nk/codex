# Cycle 1 — Epic 2 (T9 lane) / Card 11 `epic-2-cause-closure`

- **Card ID:** `epic-2-cause-closure`
- **Commit SHA:** (this cycle's commit — see push log)
- **Files touched:** `docs/retro/events/epic-2-t9.jsonl` (new — 1 correction, 1 deferral),
  `kanban.md` (card 11 Notes appended, status left `in-progress` — **not** set `complete`, per
  dispatch scope), `progress.md` (this entry). No production source file changed: investigation
  found no closeable defect this cycle (see Notes).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`BASE_BRANCH=$(git merge-base HEAD origin/develop); git diff --unified=0
  "${BASE_BRANCH}...HEAD" -- docs/retro/events/epic-2-t9.jsonl kanban.md progress.md
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no matches).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens).
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — "Cause closure closes
  by class, not by instance." This lane's scope is **T9 only** (per dispatch brief): "Per-record
  onboarding backlog in registered books ... spell, companion, feat, monster_ability, equipment,
  and monster kinds."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`), read from the repo-local slot; re-derived this cycle via
  `scripts/verify.sh --only preflight-oracle` (empty slot in fresh worktree, self-healed per §8
  with `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; PASS after fetch).
- **Status:** returned-to-backlog is explicitly NOT the disposition here — per `decisions.md §10`
  that is not a closure path. This is an honest **measurement-only cycle that banks zero units**
  (standing lesson 6 permits this as a legitimately closed cycle when the count is real and
  re-derivable, which this one is). Card 11's row status stays `in-progress`.

- **Notes:**

  **Step 1 — re-derived the population, per the anti-gaming method (§1a/§3).**
  `epic-breakdown.md`'s T9 row states 2,651 units (spell 726, companion 726, feat 480,
  monster_ability 517, equipment 174, monster 28), sourced from card 11's own cycle-1 receipt,
  which did not investigate T9 (T9 was named-and-deferred there, not measured that cycle — see
  its Notes).

  Fresh build + run, this cycle:
  ```
  cargo build --locked --release --bin v06_work_inventory
  ./target/release/v06_work_inventory --stdout-only > fresh_inventory.json
  ```
  (`CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-2-t9`,
  `PCGEN_CORPUS_ROOT=<repo-local pcgen slot>/data`). Filtered by the exact evidence-code families
  `epic-breakdown.md`/`THE-BOX.md` name for T9 (`spell_key_absent_from_spell_list`,
  `companion_absent_from_*`, `feat_key_absent_from_catalog`, `monster_ability_absent_from_*`,
  `equipment_key_absent_from_equipment_tables`, `monster_absent_from_*`):

  | Kind | Filed (2026-08-22, cycle 1) | Re-derived (this cycle) | Delta |
  |---|---:|---:|---:|
  | spell | 726 | **732** | +6 |
  | companion | 726 | 726 | 0 |
  | feat | 480 | **487** | +7 |
  | monster_ability | 517 | 517 | 0 |
  | equipment | 174 | **222** | +48 |
  | monster | 28 | 28 | 0 |
  | **Total** | **2,651** | **2,712** | **+61** |

  Logged as `scripts/retro.py correction` (`docs/retro/events/epic-2-t9.jsonl`,
  subject `epic-breakdown.md` Epic 2 T9 row). **Using the re-derived 2,712 figure from here on**,
  per the anti-gaming method's own instruction.

  **Step 2 — identify the cause, not just the count.** Investigated the mechanism behind
  `<kind>_absent_from_<book>_<table>` before touching anything. Every kind resolves through a
  per-book, per-kind compiled Rust data table (`facts.holds_key(&engine_book, &unit.kind,
  &unit.key, &unit.name)`, `src/bin/v06_work_inventory.rs`); "absent" means the corpus carries the
  key but the compiled table for that already-registered book does not. Two of six kinds
  (`monster`, `companion`) have a committed, book-generic transcription tool
  (`scripts/transcribe_monster_tables.py`, `scripts/transcribe_companion_tables.py`) that derives
  its unit set from `docs/work-inventory.json` itself, so its own refusals are diagnostic, not
  guesswork. `spell`/`feat`/`equipment` have **no such tool at all** — grepped
  `scripts/transcribe_*.py`, only the two above exist.

  **Full forensic pass on the `monster` family (28 units, all 6 residual books, dry-run only —
  zero writes committed):**
  ```
  PCGEN_CORPUS_ROOT=<repo-local pcgen slot>/data python3 scripts/transcribe_monster_tables.py <book>
  ```
  for `bestiary_4`, `bestiary` (=bestiary_1), `bestiary_2`, `inner_sea_world_guide`,
  `inner_sea_bestiary`, plus a direct inventory read for the `MonsterId_ALL` fallback unit. Every
  book's freshly-generated `monster_data.rs` diffed **byte-identical** to the committed file
  (`git diff --stat` empty in all 5 cases) — the compiled tables are already exactly what the
  corpus + tool produce; nothing is stale. The 28 "absent" units break down as:

  - **21 units — Product-Identity-excluded, by the tool's own PI screen**
    (`bestiary_4` 14: 3 Demon Lords, 3 Empyreal Lords, 3 Great Old Ones, 3 Kaiju, Spawn of
    Yog-Sothoth, Star-Spawn of Cthulhu, all `NAMEISPI:YES`; `inner_sea_world_guide` 5: Boar
    (Sargavan), Daughter of Urgathoa, Herd Animal (Storval Aurochs), Sandpoint Devil, Treerazer,
    all `NAMEISPI:YES`; `inner_sea_bestiary` 2: Chemnosit, Volnagur, both hit
    `PI_BLACKLIST_TERMS`). Transcribing these would republish Paizo Product Identity —
    `docs/governance/ogl-pi-blacklist.md` is explicitly **DRAFT, operator-review-gated**
    ("nothing in this repository treats this file as authoritative until an operator has reviewed
    and accepted"). Closing these 21 is not a code fix; it needs an operator PI ruling.
  - **6 units — structurally non-standalone overlay rows, correctly excluded by the tool's own
    logic** (`bestiary` 4: `.MOD`-only rows stating a delta on a record defined elsewhere;
    `bestiary_2` 2: `.COPY=` derived rows). Not a defect — a `.MOD`/`.COPY` row is not itself a
    creature.
  - **1 unit — a genuine, real onboarding gap**: `occult_adventures:monster:kami_shikigami`
    (`oa_races_b3.lst:6`). `engine_book: null` in the fresh inventory — `occult_adventures` has no
    `RuleSetId` match arm for the `monster` kind's chassis registry at all (only 1 monster record
    in that book). Identified, not yet closed this cycle (wiring a whole new book-kind registry
    entry for a single record is real work; see Next-cycle plan).

  **Spot-check on `companion` (86 of 726 units, `core_rulebook` only, dry-run):** same command
  against `transcribe_companion_tables.py core_rulebook` — output byte-identical to the committed
  `crb/companion_data.rs`. All 86 residual `core_rulebook` units read as orphan
  companion-ability rows with no owning companion record (e.g. `Animal Companion Feat ~
  Acrobatic`, `Animal Companion Feat ~ Armor Proficiency (Heavy)` — feat-grant rows, not
  standalone companions). This is the **same honest-not-ingested shape**
  `monster_chassis.rs`'s own module doc names for orphan `monster_ability` rows ("Only ability
  rows WITH an owner are registered... those rows stay `not-ingested`, which is their honest
  status") — not verified here to be a defect, and not closed.

  **What this leaves unproven (AGENTS.md rule 7 — proof width, stated explicitly):**
  - The other **7 companion books' residual 640 units** (ultimate_wilderness 248,
    advanced_players_guide 203, ultimate_magic 138, book_of_the_damned_volume_1 29,
    advanced_race_guide 18, bestiary_4 2, bestiary_5 2) were **not** individually forensically
    checked — `core_rulebook`'s all-orphan finding is not assumed to generalise.
  - `monster_ability` (517, 9 books), `spell` (732, single evidence code, book breakdown not
    pulled this cycle), `feat` (487, single evidence code), `equipment` (222, single evidence
    code) received **no forensic pass at all** this cycle. `spell`/`feat`/`equipment` additionally
    have no transcription tool to run a dry-run through — building one is itself new-tool work,
    not a re-run of an existing one.
  - Nothing here proves the `monster`/`core_rulebook-companion` split (PI / structural-overlay /
    genuine-gap) is the SAME split that applies to the other 5 kinds' residuals. It is a real,
    verified finding for the population actually checked (28 + 86 = 114 of 2,712 units, 4.2%),
    not a corpus-wide claim.

  **Why zero units are banked this cycle, and why that is the honest cycle rather than a stall:**
  closing "by class" requires knowing WHICH of the three causes (PI-excluded / structurally
  non-standalone / genuine gap) applies before writing a single table row — fabricating PI content
  is a licensing violation, "fixing" a `.MOD`/`.COPY` row would corrupt the transcriber's own
  correctness invariant, and the one genuine gap found (`occult_adventures` monster, 1 unit) needs
  a new book-kind registry wiring, not a data edit, so closing it alone this cycle would be a
  single-instance fix for a class-shaped card — exactly what AT-32-E2-001 prohibits ("a cycle that
  closes T2a for a single class and stops is out of protocol").

- **Discovery forwards:** none requiring a new card — this is scoped investigation and
  measurement against the existing T9 line of card 11.
- **Next-cycle plan (named, not attempted this cycle):**
  1. **Operator ruling** on the 21+ PI-flagged monster records (and however many the same PI
     screen would flag in the other 5 kinds once checked) — `docs/governance/ogl-pi-blacklist.md`
     itself requires this before any cycle can transcribe them; without it they stay
     `not-ingested` permanently and correctly.
  2. **`occult_adventures` monster onboarding** (1 record) — add the `RuleSetId` match arm +
     `MONSTER_BOOKS` entry + run `transcribe_monster_tables.py occult_adventures`; smallest
     concrete unit of real, closeable T9 work identified this cycle.
  3. **Forensic pass, book-by-book, on the remaining `companion` (640), `monster_ability` (517)**
     residuals using the existing transcription tools' own dry-run diagnostics (same method as
     this cycle's `monster`/`core_rulebook-companion` pass) — will separate genuine gaps from
     PI-exclusion/orphan-structural noise before any content is written.
  4. **Build `transcribe_spell_tables.py` / `transcribe_feat_tables.py` /
     `transcribe_equipment_tables.py`** (no such tool exists yet) before attempting spell (732)
     / feat (487) / equipment (222) — building blind without the same diagnostic screen these two
     kinds already have risks re-transcribing PI content by hand.
