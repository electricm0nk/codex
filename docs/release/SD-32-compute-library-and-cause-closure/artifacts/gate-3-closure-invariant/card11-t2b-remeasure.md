# Card 11, shape T2b — re-measurement after the classifier fix + Adoptive Parentage cycle (measurement only, 0 units banked)

- **Card ID:** `epic-2-cause-closure` (row 11; scope: T2b only, measurement per `decisions.md §13`,
  triggered by the Opus adversarial verdict `NOT_SOUND` on the classifier-fix + Adoptive Parentage
  wave)
- **Actor:** `t2b-remeasure`
- **Base:** worktree started on a stray `site-publish` merge commit (footgun 1, fired again) —
  `git reset --hard e2bbff32ca328fa3a0a76f0286b2f479f1ae0bc2`, re-verified, then
  `git fetch origin tranche/12 && git rebase origin/tranche/12`. Landed at `b4192a712`
  (`docs(sd32): Decision 18 …`) — one commit past the reviewed tip `57780b5bc`, itself one commit
  past `70695f35c` (the classifier-fix + Adoptive Parentage tip the verdict reviewed). `git log
  57780b5bc..b4192a712` shows exactly one commit, a T9 PI-review ruling unrelated to T2b/card 11 —
  confirmed before treating the verdict as still current.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`), fetched fresh via `scripts/fetch-pcgen-oracle.sh --dest <repo-local slot>`
  after `scripts/verify.sh --only preflight-oracle` FAILed on this fresh worktree — matches the
  pin exactly.
- **Environment note:** the sandbox refused writes under `/home/ubuntu/.cache/...` (both `mkdir`
  and the reclaim-claim file) as "too complex to verify stays inside the worktree" — not a
  git-operation issue, a path-outside-worktree refusal. Used an in-worktree
  `CARGO_TARGET_DIR=.cargo-target-scratch` per the brief's documented fallback; noted here as the
  brief instructed.
- **Status:** measurement cycle only — **0 units banked, no engine code changed, no corpus data
  changed, no pinned count changed.** kanban row 11 left `in-progress`.
- **Re-derive script (new, committed):** `scripts/t2b_remeasure_other_bucket_probe.py`

## 0. The adversarial verdict was `NOT_SOUND` — re-verified fresh, all seven findings still stand

The dispatch brief's verdict (reviewed at `57780b5bc`) is **not stale**. Every finding was
independently re-derived at this cycle's actual tip (`b4192a712`) before anything else, per the
brief's explicit instruction not to build a tidy re-measurement on an unsound base:

| # | Finding | Still true at `b4192a712`? | Re-derive command (this cycle, fresh) |
|---|---|---|---|
| 1 | 112 Ultimate Psionics units reclassified to `monster_ability` on a false discriminator (corpus's own `TYPE:...PC` + `CR:` marker contradicts it) | **Yes.** `up_abilities_race.lst` rows for Blue/Dromite/Elan/Forgeborn/Half-Giant/Maenad/Noral/Ophiduan/Xeph still all carry both `TYPE:Humanoid.Base.PC` and `CR:1/2`; the 112 units are still on `monster_ability_absent_from_ultimate_psionics_monster_abilities`, UP's T9 sub-total still 176 (was 64). | `grep -P "^(Blue\|Dromite\|Elan\|Forgeborn\|Half-Giant\|Maenad\|Noral\|Ophiduan\|Xeph)\t" "$PCGEN_CORPUS_ROOT/pathfinder/dreamscarred_press/ultimate_psionics/up_races.lst"` |
| 2 | Stress test's "0 false positives" is true by construction — 10 hardcoded dirs, `dreamscarred_press` never in scope | **Yes.** `scripts/t2b_refine_kind_key_prefix_stress_test.py`'s `KNOWN_RACE_BOOKS_DIRS` (line 48) still lists exactly the same 10 Paizo paths; Dreamscarred Press absent. | `grep -n "KNOWN_RACE_BOOKS_DIRS" -A20 scripts/t2b_refine_kind_key_prefix_stress_test.py` |
| 3 | `origin/tranche/12` red on `--tests` (`inner_sea_faiths` mis-registered, from unrelated card-15 commit) | **Yes, unchanged.** | `cargo test --locked --test v06_work_inventory sd30_campaign_setting_books` → `FAILED … got String("in_scope")` |
| 4 | Templates guard passes by construction, doesn't fail under the realistic widening | Not re-derived this cycle (measurement-only scope; no code touched). Named as still open. | — |
| 5 | 7 Adoptive Parentage closures never reached `docs/work-inventory.json` | **Yes.** The 7 rows (`advanced_race_guide`, `arg_abilities_race.lst:291-297`) still read `status: not-ingested`, `evidence: race_trait_race_not_modelled` in the ledger **even though the real corpus records exist** (`data/corpus/advanced_race_guide/race_trait/{dwarf,elf,gnome,halfling,orc,drow,grippli}/*.json`, `ingested_at: "2026-08-23T04:02:31Z"`, real `raw_tokens`, resolver code present). This directly inflates this memo's own headline T2b count by 7 — see §2. | `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print([(u['source_line'],u['status']) for u in d['units'] if u['book']=='advanced_race_guide' and u['source_file']=='arg_abilities_race.lst' and 291<=u['source_line']<=297])"` |
| 6 | `adoptiveParentageOptions` reaches IPC but no player-facing UI | **Yes.** `grep -rn adoptiveParentageOptions apps/desktop/src/` → 0 hits; `AlternateTraitPicker.tsx` only renders `.alternates`. | `grep -rn "adoptiveParentageOptions" apps/desktop/src/`; `grep -n "\.alternates" apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx` |
| 7 | `kanban.md` row 11 never records the classifier movement (T2b/T9 totals stale) | **Yes.** Row 11 still reads "T2b 2,472; T9 2,712" in its four-shape summary line; only the Adoptive Parentage sub-entry (prepended) mentions the real numbers. | `grep -n "epic-2-cause-closure" docs/release/SD-32-compute-library-and-cause-closure/kanban.md` |

**Suites, re-run fresh at `b4192a712` (not quoted from the reviewed cycles):**
```
cargo test --locked --lib                                          -> 2390 passed, 0 failed, 13 ignored
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml -> 518 passed, 0 failed
cargo test --locked --test v06_work_inventory                       -> 16 passed, 1 FAILED (finding 3)
scripts/verify.sh --only reach                                      -> PASS (31 passed)
```
`cargo test --locked --tests` (finding 3's own recommendation, "add it to the standing suite") was
**not** added this cycle — that is a code change, out of this memo's measurement-only scope; named
here as still-needed.

**None of this is fixed in this cycle.** This memo reports the honest current state on top of that
unsound base, as instructed, and the decomposition below explicitly carries the +7
(stale-ledger) and the ±112 (disputed UP reclassification) as named adjustments rather than folding
them silently into a clean number.

## 1. T2b population, re-derived, against the history

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
print(len(u))"
```

| Point in time | T2b population | Source |
|---|---:|---|
| Original (`decisions.md §13`) | 2,472 | operator ruling table, 2026-08-22 |
| After census memo (header rows named, not subtracted from the raw ledger) | 2,472 (2,325 "real work" carve-out for dispatch sizing) | `card11-t2b-census-census.md §4` |
| After `decisions.md §16` item 1 (refine_kind classifier fix) + item 2 (Adoptive Parentage, 7 units) | **1,578** | `epic-2-t2b-refine-kind-fix_cycle-1_cycle_receipt.md §5` |
| **Now, re-derived fresh at `b4192a712`** | **1,578** (unchanged since the fix — no code has touched T2b's classifier or evidence family since) | this cycle |

**No drift since the last code change** — 1,578 is exact, matches the receipt, and this cycle
changed nothing that could move it.

**But 1,578 is not the true current backlog by the ledger's own criterion**, because of finding 5:
7 of these 1,578 rows are units whose real corpus record already exists and already resolves with
zero unresolved grants — they are closed in substance, just not in the ledger. **True current
backlog if the ledger were regenerated: 1,571.** This memo decomposes the as-published 1,578 below
and calls out the 7 explicitly rather than quietly subtracting them, because "the ledger says X" and
"the corpus says X" have diverged and that divergence is itself a finding, not a rounding error.

## 2. Exact decomposition — no residual term

### 2a. From 2,472 to 1,578 (Δ −894)

```
python3 scripts/t2b_refine_kind_fix_movement_report.py <before.json> docs/work-inventory.json
```
(command and full output already in `epic-2-t2b-refine-kind-fix_cycle-1_cycle_receipt.md §4`;
re-read, not re-run, since it requires a pre-fix snapshot this worktree no longer has — the
**result** it reports is corroborated independently in §1 above by re-deriving the population at
both ends of the fix from the two committed inventories' own history)

| Component | Units | Disposition |
|---|---:|---|
| Reclassified `race_trait → monster_ability` (`decisions.md §16` item 1, KEY-prefix discriminator) | 864 | **Disputed for 112 of the 864** — see finding 1. The other 752 are corroborated: 0 hit any of the 39 races this project models, 0 hit any Paizo playable-race book, all land on `monster_ability_*` evidence and stay open work (moved, not closed) |
| Genuinely closed (18 Dhampir chassis + Ratfolk alternates, bestiary_2; 12 `inner_sea_races` stale-regen catch-up) | 30 | Closed by class, corpus-wide, wave-1 book lanes |
| **Total Δ** | **894** | 864 + 30 = 894 = 2,472 − 1,578 ✓ no residual |

### 2b. Inside the current 1,578 — full partition, zero residual

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
def classify(ck):
    if ck.startswith('Adopted Race ~ '): return 'adopted_race'
    for p in ('Racial SLA ~ ','Unchained Evolution ~ ','Favored Class Bonus ~ ','Race Subtype ~ '):
        if ck.startswith(p): return 'header'
    return 'other'
c = collections.Counter(classify(x.get('corpus_key') or x.get('name') or '') for x in u)
print(c)"
```
→ `{'other': 1305, 'header': 236, 'adopted_race': 37}` (sum 1,578 ✓)

| Bucket | Units | What it is | Disposition |
|---|---:|---|---|
| **Category-header rows** (`Racial SLA ~`, `Unchained Evolution ~`, `Favored Class Bonus ~`, `Race Subtype ~` prefixes) | 236 | No race named at all — by-design matcher exclusion | **Not work.** Confirmed corpus-wide this cycle (was 147 for the registered-book pile alone in the census memo; the same 4-prefix rule applied to the *whole* current 1,578 gives 236 — 147 registered + 89 unregistered, not previously totalled together) |
| **`Adopted Race ~ <X>` selector rows — proven empty** | 2 | Rougarou (bestiary_6, known precedent) **+ bestiary_4's Changeling (new finding this cycle)** — corpus-wide grep for `"Changeling Race Trait"` returns only the book's own file, same proof shape as Rougarou | **Not work.** See §3 for the corpus-wide join that found this |
| **`Adopted Race ~ <X>` selector rows — real, corpus-wide content exists, no ingestable target in this project** | 35 | See §3 — **corrected from 14 to 35**, see the retro correction below | **Real, open, blocked** on an operator ruling (new `kind: trait` surface + `player_companion` book onboarding), per the Adoptive Parentage receipt's own escalation, now sized correctly |
| **`arg_flat_grant` "Adoptive Parentage" rows (advanced_race_guide, lines 291-297)** | 7 | Already built, already ingested, already resolving with 0 unresolved grants, already reachable via `reach_gate` (421 records) | **Stale-ledger.** Genuinely closed in substance; `docs/work-inventory.json` was never regenerated to reflect it (finding 5). Counted here because the ledger still counts it, but it is not open work |
| **"Other" — ordinary per-record/per-book content** | 1,298 (1,305 − 7 stale) | The real per-book backlog — see §4 for the full breakdown and §5 for a major sub-finding (a large slice of this is itself still classifier noise, not race content) | **Real, open** — sized in §4 |

**Check:** 236 + 2 + 35 + 7 + 1,298 = 1,578 ✓ no residual.

**Headline: genuinely-open T2b work right now = 35 (adopted_race) + 1,298 (other) = 1,333 units**,
against a **238-unit not-work total** (236 header + 2 proven-empty) and a **7-unit stale-ledger
gap** that should already read closed.

## 3. Correction to `decisions.md §16` item 2 / the Adoptive Parentage receipt: 37 units, not 21; 35 real, not 13

The Adoptive Parentage cycle scoped its census to 5 books (bestiary_2, bestiary_3,
advanced_race_guide, bestiary_5, bestiary_6) and found 21 units (7 `arg_flat_grant` + 14
`adopted_race_choose_selector`). **The `Adopted Race ~ <X>` selector shape is not limited to those
5 books.** A corpus-key prefix scan over the *current, full* T2b population finds it in **9 books**:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
c = {}
for x in u:
    ck = x.get('corpus_key') or x.get('name') or ''
    if ck.startswith('Adopted Race ~ '):
        c[x['book']] = c.get(x['book'],0)+1
print(c, sum(c.values()))"
```
→ `{'bestiary_2': 7, 'bestiary_5': 1, 'bestiary_6': 1, 'core_rulebook'...}` — **37 total**:
bestiary 11, bestiary_2 7, bestiary_3 5, bestiary_4 9, bestiary_5 1, bestiary_6 1,
inner_sea_world_guide 2, ultimate_wilderness 1.

Re-proving each race corpus-wide with the **same method** the Adoptive Parentage receipt used
(count of distinct files corpus-wide containing `"<Race> Race Trait"`):

```
grep -rl --fixed-strings "<Race> Race Trait" "$PCGEN_CORPUS_ROOT"
```

Result (full table in `scripts/t2b_remeasure_other_bucket_probe.py`'s sibling check, re-run
live this cycle): **35 of 37 are REAL** (2+ files corpus-wide — mostly unregistered
`player_companion` PF1e "Trait" books, exactly the pattern the receipt already proved for its
original 14), **2 are PROVEN EMPTY** (1 file corpus-wide, matching only the selector's own file):
Rougarou (bestiary_6, the receipt's own finding, unchanged) and **bestiary_4's Changeling** (new
this cycle — `grep -rl "Changeling Race Trait" "$PCGEN_CORPUS_ROOT"` returns exactly one file,
`core_essentials/races/changeling/changeling_abilities_race.lst`, the selector's own row).

**Retro correction logged:**
```
scripts/retro.py correction --subject "decisions.md §16 item 2 / epic-2-t2b-adoptive-parentage_cycle-1_cycle_receipt.md" \
  --claimed "21 units across 5 books (7 arg_flat_grant + 14 adopted_race_choose_selector)" \
  --actual "37 units across 9 books; 35 real, 2 proven empty" \
  --verified-by "python3 -c \"...\" -> 37; corpus-wide grep per race name + ' Race Trait'"
```
Event: `docs/retro/events/t2b-remeasure.jsonl`.

**Consequence:** the "new `kind: trait` content surface + `player_companion` book onboarding"
epic the Adoptive Parentage receipt escalated is **35 units, not 13** — 2.7× larger than reported.
This does not change the ruling needed (still an operator call on a new content kind), only its
size.

## 4. Per-book "still genuinely open" work list — the next wave's dispatch list

Current, live counts (not the census memo's pre-fix figures), with status:

| Book | Units (now) | Registered? | Status / what it needs | Files (est.) |
|---|---:|---|---|---|
| `bestiary_3` | 194 | No | **Already fully sub-classified by w1-d** (unchanged by this cycle): 9 header (not-work), 5 adopted_race (real, blocked — new-kind epic), 58 `monster_or_template_owned` (needs a template-vs-subrace discriminator — genuinely new work, not a widening of `decisions.md §16` item 1), 122 `unresolved` (needs fuzzy/prefix matching, deliberately not attempted — risk of reopening the Favored-Enemy trap) | new discriminator + onboarding |
| `bestiary_2` | 178 | Yes | 7 adopted_race (real). **171 "other" — §5 finds only 6 of 171 correspond to any name in the book's own `*races*.lst` at all; spot-checked rows (`Adamantine Golem`, `Agathion`, `Aeon Traits`) are monster stat-block content.** SUSPECT, not proven — needs the same per-row proof bestiary_3 got before any dispatch | classifier pass first, not ingest extension |
| `bestiary_5` | 137 | Yes | **Fully characterized by w1-b** (unchanged): 8 new race chassis (61 units: Shabti 12, Reptoid 10, Deep One Hybrid 9, Orang-Pendak 9, Astomoi 8, Caligni 7, Clockwork Familiar 5, Esipil 1), Skinwalker heritage-selector (72 units, `ingest_races.rs`'s own doc comment calls it "genuinely new… deferred, not stubbed"), 1 adopted_race (real), 1 header, ~2 unaccounted | 3 chassis builds + 1 selector mechanism |
| `core_rulebook` | 132 | Yes | 118 header (not-work). 14 "other" — census memo flagged these as sentinel-suspect (`"Region ~ None"`, `"No Race Trait Available"`); §5's probe finds 0/14 match any core_rulebook races-file name, consistent with sentinel/bookkeeping, **not re-confirmed row-by-row this cycle** | confirm sentinel status, then small ingest extension for any real residual |
| `pathfinder_unchained` | 127 | No | 68 header (not-work). **59 "other" — the book has NO `*races*.lst` file at all** (only `pu_abilities_race.lst`); spot-checked rows are Unchained Summoner "Base Form" entries (Agathion/Angel/Archon/Azata/Daemon Base Form) — **not race content**, same defect class as `decisions.md §16`, book-level proof (no races file exists to name a race against) | **do not onboard — needs the classifier fix extended, not a book cycle** |
| `mythic_adventures` | 118 | No | **The book has NO `*races*.lst` file at all** (only `ma_abilities_race.lst`). Spot-checked rows: `Mythic Aboleth`, `Mythic Barghest (Greater)`, `Damage Reduction` — mythic monster template content, not race content | **do not onboard — same as above, book-level proof** |
| `ultimate_wilderness` | 103 | No | 2 header, 1 adopted_race (real). 100 "other" — 33/100 match a books-own race name (real per-record backlog, e.g. Ghoran), 67/100 do not (`Enhanced Gnome Magic`, `Favored Class Bonus Output` — plumbing, not proven noise or real) | mixed; needs row-level split before dispatch |
| `bestiary_4` | 99 | No | 5 header, 9 adopted_race (8 real + 1 proven-empty, Changeling — §3). **85 "other" — 21/85 match a books-own race name (Kasatha/Kitsune/Nagaji/Samsaran/Trox/Wayang/Wyrwood/Wyvaran = real per-record backlog, same shape as the ARG's deferred races), 64/85 do not** (`Demon Lord`, `Dragon`, `Empyreal Lord` — SUSPECT noise) | real backlog (≤21) + suspect noise (≤64), needs split |
| `bestiary` | 96 | No | 11 adopted_race (real). **85 "other" — only 3/85 match a books-own race name; spot-checked rows (`Aberration Traits`, `Air Traits`, `Angel Traits`, `Aquatic Traits`) are monster-type descriptor headers, not race content.** SUSPECT, high-confidence noise, not proven row-by-row | classifier pass first |
| `occult_adventures` | 71 | No | **The book's own `oa_races.lst` is a 15-line "Phantom Companion" pool stub, not player races.** Spot-checked rows: `Emotional Focus / Anger`, `Emotional Focus / Dedication` — these are **Occultist class-feature mental-focus options**, not race content | **do not onboard — book carries no playable races at all, same shape as bestiary_3** |
| `advanced_players_guide` | 58 | Yes | 21 header (not-work). **37 "other" — `apg_races_companion.lst` (an animal-companion pool file) carries only 2 names; spot-checked rows (`Alchemist`, `Bard Spell Level 0-5`, `Cavalier`) are class names and spell-level headers**, matching w1-c's own established finding for this book | **do not onboard — matches w1-c's "0 real" finding, extend the classifier fix instead** |
| `advanced_race_guide` | 57 | Yes | 7 stale-ledger (already closed, needs ledger regen — finding 5). 3 Changeling + 3 Dhampir (real content, blocked — new-race-onboarding epic per w1-c, `Changeling`/`Dhampir` explicitly excluded from `IN_SCOPE_RACES`). 1 Samsaran header (blocked, same reason) + 28 Mystic Past Life primitives — **the formula-interpreter blocker w1-c cited is STALE, see §6**. 9 "Heart of the X"/"Fins to Feet" rows — spot-checked as genuine Human alternate racial traits (`ABILITY:Human Racial Trait\|AUTOMATIC\|Human ~ Heart of the Mountain`), real, in-scope, not previously flagged as open by w1-c's earlier pass | ledger regen (0 files); formula-interpreter wiring (blocked-on-ruling, see §6); 9-unit standard ingest extension |
| `inner_sea_races` | 47 | Yes | 0 header, 0 adopted_race. All 47 "other" — census memo's own worked example (`Kasatha ~ Stealthy`) confirms real, never-transcribed per-record content for already-in-scope races | standard ingest-tool extension |
| `ultimate_psionics` | 47 | No | 0 header/adopted. All "other" — spot-checked rows (`Aegis`, `Cryptic`, `Dread`, `Dromite Crafter/Psionics`, `DuergarDSP`) are **psionic class names and subtype-tagged plumbing**, not race content — SUSPECT noise, consistent with this book already being ground zero for finding 1's UP-race defect | classifier pass, not book onboarding |
| `inner_sea_world_guide` | 34 | No | 2 adopted_race (real). 32 "other" — 19/32 match a books-own race name (real, e.g. Gillman/Strix per §3), 13/32 do not | mixed, needs split |
| `advanced_class_guide` | 31 | No | **The book has NO `*races*.lst` file at all.** Spot-checked rows (`Arcanist Exploit`, `Arcanist`, `Bloodrager`) are class names | **do not onboard — same book-level shape as mythic_adventures/pathfinder_unchained** |
| `inner_sea_gods` | 25 | No | 21/25 match a books-own race name (real per-record backlog), 4 do not (small, unexamined) | mostly real, standard extension |
| `horror_adventures` | 6 | Yes | 2 header, 4 "other" (small, matches census memo's characterization, not re-examined) | standard extension |
| `ultimate_combat` / `ultimate_intrigue` / `ultimate_magic` / `inner_sea_bestiary` / `monster_codex` / `book_of_the_damned_volume_1/2` | 4/3/3/2/3/1/1 | mixed | Small populations, not individually re-examined this cycle (out of budget); census memo's per-book byte counts still the best available estimate | small, needs a fresh look each |
| `bestiary_6` | 1 | Yes | 1 adopted_race, **proven empty (Rougarou)** | **not work** |

**Sum check:** every book row above sums to the live per-book counts derived in §2, which sum to
1,578 exactly (re-run: `python3 -c "...Counter(x['book'] for x in u)..."` → matches).

## 5. The major discovery this cycle adds: a further, unquantified classifier-noise residual

`decisions.md §16`'s classifier fix (item 1) only reclassifies a row when its `KEY:` prefix
**exactly matches a `CR:`-bearing entry in that same book's own `*_races.lst`**. That is
deliberately narrow (the Favored-Enemy trap requires it to be). But it means the fix is **blind**
to two shapes of the identical root defect:

1. **A book whose `abilities_race.lst` exists but whose `*_races.lst` does not, or is a near-empty
   stub.** `mythic_adventures` and `pathfinder_unchained` have **no `*races*.lst` file at all** —
   confirmed by direct directory listing, not inference. `occult_adventures`'s `oa_races.lst` is
   15 lines, a "Phantom Companion" pool stub. `advanced_class_guide` has no races file either.
   These four books cannot possibly have any row correctly reclassified by the KEY-prefix rule,
   because there is nothing to match against — yet their `abilities_race.lst` rows still get typed
   `race_trait` by `refine_kind`'s default. Spot-checked content in every one of the four is
   monster-template or class-feature material, not race content (`Mythic Aboleth`, `Agathion Base
   Form`, `Emotional Focus / Anger`, `Arcanist Exploit`). `advanced_players_guide`'s own races file
   is a 2-name animal-companion stub with the same shape.
2. **A book with a real, populated `*_races.lst` where a monster's KEY doesn't happen to match
   the exact string** (a different name, a punctuation difference, or the monster lives outside
   that book's own `*_races.lst` entirely — golems, for instance, are commonly not listed with a
   `CR:` token in `*_races.lst` at all). `bestiary_2`'s wave-1 stress test itself found **296**
   corpus-wide hits under a looser matching pass, but the exact-match discriminator only moved
   **69** — the `epic-2-t2b-refine-kind-fix_cycle-1_cycle_receipt.md §9` `## DISCOVERED` entry
   named this exact residual as unexamined for `bestiary_2`/`bestiary_4`/`bestiary`/
   `inner_sea_gods`/`inner_sea_bestiary`/`occult_adventures`/`ultimate_psionics`. This memo's own
   probe (`scripts/t2b_remeasure_other_bucket_probe.py`) is the first follow-up look: for
   `bestiary_2`, only 6 of 171 remaining "other" rows' KEY prefix matches *any* name in the book's
   own races files at all (the other 165 include `Adamantine Golem`, `Agathion`, `Aeon Traits`,
   `Crystal Dragon`, `Nightshade`, `Protean` — none are races this project models or could model).

**This is a measurement, not a proof at the rigor bestiary_3 got.** The "key not in any races
file" heuristic has real false positives for books whose genuine race content lives in a
subdirectory the probe's glob doesn't reach (`advanced_race_guide`'s Drow/Dwarf/Elf/etc. — those
are the 7 already-closed stale-ledger rows, correctly real, and the probe's crude check would have
called them "not found" too if they hadn't already been excluded by the `arg_flat_grant` shape
check). So the counts in §4's "SUSPECT" column are a **starting point for a dedicated
classification pass**, not a corrected total — they are not subtracted from §2's headline 1,333.

**High-confidence (book-level, not row-heuristic) noise, safe to state as a lower bound:**
`mythic_adventures` (118) + `pathfinder_unchained` (59, excl. its 68 already-not-work header) +
`advanced_class_guide` (31) + `occult_adventures` (71) + `advanced_players_guide` (37) = **316
units in 5 entire books that plausibly carry zero playable-race content**, the same shape
`decisions.md §16`'s own w1-d finding proved for `bestiary_3` (819 units, zero playable races) —
just not yet proven to that same row-by-row rigor.

**Recommendation for the next wave, not performed here:** extend `decisions.md §16` item 1's
classifier fix with a second discriminator — *does this book's `abilities_race.lst` file have a
sibling `*_races.lst` with any content at all, and does the row's KEY correspond to anything in
it* — proven safe against the same Favored-Enemy trap, **before** dispatching any per-book
onboarding cycle against `bestiary_2`, `bestiary`, `bestiary_4`, `mythic_adventures`,
`pathfinder_unchained`, `occult_adventures`, `advanced_class_guide`, `advanced_players_guide`, or
`ultimate_psionics`. Dispatching book-onboarding cycles against these now, before that pass, risks
repeating exactly the fabrication `decisions.md §1a` forbids — building race chassis for monster
and class-feature content.

## 6. The formula-interpreter blocker the ARG lane raised is stale — the ruling's condition has been met

`epic-2-t2b-w1-c_cycle-1_cycle_receipt.md` named 29 `advanced_race_guide` units (Samsaran's
`Mystic Past Life` trait + its 28 per-class CHOOSE-target primitives) as blocked on **two**
conditions: Samsaran not being in `IN_SCOPE_RACES`, **and** "`decisions.md §24`'s formula-
interpreter ban" forbidding resolution of `BONUS:VAR|MysticPastLifeINTBonus|(MysticPastLifeScoreINT-10)/2`-shaped
formulas even if the race were in scope.

**That ban has been overturned.** `§24` is not in this bundle's `decisions.md` — it is
`SD-27 decisions.md §24.1`, and `SD-31 decisions.md` Decision 20 (2026-08-21, operator-pinned,
titled *"§24.1 is overturned for this package: build the formula interpreter, gated by fixtures"*)
explicitly lifted it, **before this SD-32 bundle even started**. The capability exists in this
repo right now:

```
wc -l src/rules_core/pilot_compute/formula_interpreter.rs
```
→ 1,345 lines, fixture-gated (this bundle's own `decisions.md §3` restates the fixture-discipline
condition Decision 20 attached), already used by `bonus_stack_reader.rs` and
`derived_evaluator_fixture_check.rs`, with its own corpus-wide coverage tests passing in the
suite run at §0.

**It is not yet wired into `ingest_race_traits.rs` or `race_resolver.rs`** —
`grep -rn "formula_interpreter" src/bin/ingest_race_traits.rs src/rules_core/race_resolver.rs`
returns nothing — so the 29 units are still genuinely open, just not for the reason last stated.
Per `AGENTS.md`'s standing lesson 7 (a deferral's revisit condition must be **checked, not
remembered**): **checked, and the condition is met.** The remaining blocker on these 29 units is
purely `Samsaran` not being in `IN_SCOPE_RACES` (a new-race-onboarding scope question, same as
Changeling/Dhampir), not the formula interpreter. Wiring the already-built, already-fixture-gated
interpreter into race-trait ingestion is real, buildable, in-scope-by-precedent work for whichever
cycle is granted the Samsaran chassis.

## 7. Mechanism-shaped clusters — build once, not per-book

Confirmed still current at this cycle's live counts (§4):

1. **8 new race chassis, `bestiary_5`, 61 units** — Shabti 12, Reptoid 10, Deep One Hybrid 9,
   Orang-Pendak 9, Astomoi 8, Caligni 7, Clockwork Familiar 5, Esipil 1.
2. **Skinwalker heritage-selector, `bestiary_5`, 72 units** — `ingest_races.rs`'s own doc comment:
   "genuinely new… deferred (not stubbed)."
3. **`Adopted Race` / "Adoptive Parentage" selector, 35 real units across 9 books** — corrected
   this cycle from 14/5 books to 37/9 books (35 real + 2 proven empty), §3. Needs a new `kind:
   trait` content surface plus `player_companion` book onboarding; escalated, needs an operator
   ruling, unchanged in kind from the Adoptive Parentage receipt's own escalation — only the size
   changed.
4. **Changeling / Dhampir / Samsaran new-race chassis, spanning `advanced_race_guide` (6-7 units)
   + `bestiary_4` (Kasatha/Kitsune/Nagaji/Samsaran/Trox/Wayang/Wyrwood/Wyvaran, ≤21 units per §4)
   + `inner_sea_races` (6 Changeling + 7 Dhampir + 1 Samsaran units, confirmed present in the
   ledger this cycle)** — one chassis-building epic serving at least 3 books, not per-book work.
   Dhampir's *original* chassis (bestiary_2) is already built (wave 1's 18-unit closure); these
   are each race's *additional* heritage/regional variants in other books, the same "chassis
   exists, heritage rows don't" shape Skinwalker already demonstrated.
5. **The book-level classifier-noise cluster named in §5** — not a content mechanism, but a single
   tooling fix (extend `refine_kind`'s discriminator) that would retire work from at least 5, and
   likely 9, books at once, versus dispatching per-book cycles that would misfire into fabricating
   race chassis for monster/class content.

## 8. Sweep for pinned counts (no code changed, but the brief's discipline still applies to this memo's own figures)

```
grep -rn "2472\|2,472\|1578\|1,578\|2325\|2,325" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v "/target/" | grep -v "/artifacts/corpus/"
```
No hardcoded `assert`/`assert_eq` pins any of T2b's totals in `src/`, `tests/`, or `apps/` — every
hit is prose (doc comments, `docs/release/` receipts, `scripts/t2b_*.py` docstrings). Confirmed for
`apps/desktop/src-tauri` explicitly (separate Cargo workspace) via the full suite run in §0.

## 9. What this memo did NOT do (measurement-only scope, explicitly)

- Did not fix findings 1-4 or 6-7 from the adversarial verdict.
- Did not regenerate `docs/work-inventory.json` (would close finding 5's 7-unit gap, but is a
  corpus/ledger-regenerating action, out of this memo's granted scope).
- Did not build the template-vs-subrace discriminator `bestiary_3`'s 58 `monster_or_template_owned`
  units need, nor the fuzzy-match layer its 122 `unresolved` units need.
- Did not extend the classifier for §5's book-level noise cluster.
- Did not perform the row-level proof `bestiary_2`/`bestiary`/`bestiary_4`'s SUSPECT buckets need
  before being trusted as either real work or not-work.
- Did not wire the formula interpreter into race-trait ingestion (§6).
- Did not touch `kanban.md` row 11's status (stays `in-progress` per the brief) — did not even
  update its stale T2b/T9 totals in the summary line, since that is itself a doc edit outside a
  strict read of "measurement only, no engine code, no corpus data, no pinned count"; named as
  outstanding (finding 7) rather than silently fixed mid-measurement.

## 10. Next-cycle plan

In priority order, per this memo's own findings:

1. Close the adversarial verdict's findings 1-4, 6, 7 (§0) — these block calling *any* prior T2b
   cycle SOUND, and finding 3 blocks the branch's own green claim.
2. Regenerate `docs/work-inventory.json` against the current corpus (closes finding 5's 7-unit gap,
   corrects the published T2b total to 1,571).
3. Extend `refine_kind`'s discriminator for §5's book-level cluster (proven-safe, TDD'd, adversarially
   checked per `decisions.md §16`'s own guard rail) — likely retires a large fraction of the 316+
   units in `mythic_adventures`/`pathfinder_unchained`/`occult_adventures`/`advanced_class_guide`/
   `advanced_players_guide`, and a currently-unquantified slice of `bestiary_2`/`bestiary`/
   `bestiary_4`.
4. Re-measure T2b again after (3) — only then is the per-book dispatch list in §4 safe to act on
   without risking fabricated chassis for monster/class content.
5. In parallel (no dependency): dispatch the two proven, ready-to-build mechanism clusters (§7
   items 1-2, bestiary_5's 133 units) and escalate for an operator ruling on the new `kind: trait`
   epic (§7 item 3, 35 units) and the Changeling/Dhampir/Samsaran new-race-chassis epic (§7 item 4).
6. Wire the formula interpreter into race-trait ingestion once Samsaran (or any race needing it)
   gets a scope ruling (§6) — no longer blocked on the interpreter itself.

## Disk usage

```
df -h /
```
→ `/dev/sda1  968G  308G  660G  32% /` — no pressure, no cleanup needed.
