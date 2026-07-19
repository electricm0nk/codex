---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-15; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; override flags A–D defaulted; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
---

# SD-22 — Risks and Open Questions

This file enumerates the risks, blockers, and open questions specific to SD-22. Structured to mirror SD-21's risks docs.

## Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start | `git status --porcelain \| wc -l` returns non-zero | Run `git stash` (if unfinished) or `git checkout -- .` (stray edit noise); re-verify clean; retry |
| A merge conflict on a structured-data file in `rules_tables/apg/` / `acg/` / `beastiary1/` when two cycles touch adjacent files | Merge conflict on the structured-data file | Resolve inline if mechanical (ordering, an extra row); escalate to operator if semantic |
| A cycle's RED test fails because the `RuleSetId` enum's variant isn't yet wired into the resolver | `tests/sd22_<book>_class_table_resolve` fails for a book that hasn't been ingested yet | Route to Open Blockers; operator decides whether the cycle is the right time to ingest that book |
| A cycle's RED test fails because the parser is mis-parsing a publisher's structured-data file | Per-file parse error on the cycle's `<book>/class_<class>.rs` load | Surface the file path and parse error in the load result; route to Open Blockers if the parse is unfixable per-cycle |
| Markdown file on disk has a stale `nonce` (from a Drive sync edge case, if SD-22 surfaces this) | `CampaignSnapshot.nonce != saved_nonce` on load | Engine surfaces "stale nonce, please save again"; doesn't trigger conflict log unless the *content* also differs |
| Per-class Epic 3+4 ordering collision | Two cycles both try to land the same APG/ACG class table | Defer the second cycle until the first cycle's tests are green; surface as `## Open blockers` if the conflict indicates a real per-book issue |

## Per-content-unit failure-mode inventory (operator directive 2026-07-19)

Per Epic 9's evaluator working from `corpus-source-inventory.md`: failure modes below are concrete tables that tell a cycle's debugger exactly which column / row / rule-table cell is suspicious when a test fails or a resolver returns the wrong value. Self-heal and non-self-heal decisions are made against the inventory. Generic "rules-engine correctness violation" rows in the next section are diagnostic-aid only; the inventory below is the load-bearing surface.

### Epic 3 — APG content-source ingest failure modes

| Content unit | Failure mode | Where to look (column / row / rule-table cell) | Bucket |
|---|---|---|---|
| Alchemist | Bomb damage scale wrong | `class_alchemist::BOMB_DAMAGE_BY_LEVEL_BY_DIE` (column: damage-die per level) | self-healable (read the published book's Alchemy/Bombs table; fix the cell) |
| Alchemist | Mutagen-bomb mutual exclusion | `class_alchemist::MUTAGEN_AND_BOMB_MUTEX_AT_LEVELS_1_THRU_20` (column: "Mutagen and Bomb active simultaneously? boolean per level") | self-healable |
| Alchemist | Discovery choices | `class_alchemist::DISCOVERIES_BY_LEVEL: HashMap<u8, DiscoverySet>` (column: level → discoveries-known count) | self-healable |
| Cavalier | Order choice at level 1 | `class_cavalier::ORDERS: Vec<OrderId>` (column: order list; verify every choice is mechanically distinct) | self-healable |
| Cavalier | Challenge uses | `class_cavalier::CHALLENGE_USES_BY_LEVEL: HashMap<u8, u8>` (column: 1/day + 1/3 levels) | self-healable |
| Gunslinger | Deeds list per level | `class_gunslinger::DEEDS_BY_LEVEL: HashMap<u8, Vec<DeedRef>>` (column: deeds-known at level, requires updating source) | self-healable |
| Gunslinger | Grit per-day + per-encounter rule | `class_gunslinger::GRIT_POOL: HashMap<u8, (daily_max, encounter_max)>` (column: two tuples per level) | self-healable |
| Inquisitor | Judgment uses | `class_inquisitor::JUDGMENT_USES_BY_LEVEL: HashMap<u8, u8>` (column: 1 + 1/4 levels) | self-healable |
| Inquisitor | Inquisition domain choice | `class_inquisitor::INQUISITION_DOMAINS: Vec<DomainId>` (column: domain list) | self-healable |
| Magus | Spell Combat legal attacks per round | `class_magus::SPELL_COMBAT_ATTACKS_PER_ROUND: u8` (column: 1 attack + spell; verify the rule, not the table) | self-healable (read PF1 Magus §Spell Combat) |
| Magus | Arcane Pool points per level | `class_magus::ARCANE_POOL_POINTS_BY_LEVEL: HashMap<u8, u8>` | self-healable |
| Oracle | Mystery / Curse / Revelation columns | `class_oracle::MYSTERIES`, `CURSES`, `REVELATIONS_BY_LEVEL` (three columns per row) | self-healable |
| Summoner | Eidolon stat-block shape | `class_summoner::EIDOLON_BASE_FORM` + linked-life rule (`class_summoner::LIFE_LINK_HP_PCT`) | self-healable |
| Summoner | Spell-known progression | `class_summoner::SPELLS_KNOWN_BY_LEVEL: HashMap<u8, u8>` (column: 1 + 1/2 levels) | self-healable |
| Witch | Hex-per-day count | `class_witch::HEXES_PER_DAY_BY_LEVEL: HashMap<u8, u8>` (column: 1 + 1/2 levels to 8, +1 at 10, 12, 14, 16, 18, 20) | self-healable |
| Witch | Patron spells augmented | `class_witch::PATRON_SPELLS` cross-ref table (column: witch-list spell level 1 matches patron-list spell slot X) | self-healable |
| APG shared spell | Spell-level lookup misses | `spell_list::APG_SPELL_BY_NAME` returns `None` for a known APG spell | self-healable (re-parse the publisher's spell list) |
| APG shared equipment | Equipment row indexing misses | `equipment_tables::APG_EQUIPMENT_BY_KEY` returns `None` for a key | self-healable |
| Cross-book APG vs CRB | APG-only item resolves via CRB | Resolver chain `RuleSetId::Apg::resolve` returning the CRB record instead | self-healable (re-check the priority chain) |
| Cross-book APG fail | APG-only item returns `Some` for `RuleSetId::Crb` | Resolver chain | self-healable |

### Epic 4 — ACG content-source ingest failure modes

| Content unit | Failure mode | Where to look | Bucket |
|---|---|---|---|
| Arcanist | Exploit per 2 levels | `class_arcanist::EXPLOITS_KNOWN_BY_LEVEL: HashMap<u8, u8>` | self-healable |
| Arcanist | Spell Blending count | `class_arcanist::SPELL_BLENDING_CAPACITY_BY_LEVEL` | self-healable |
| Bloodrager | Bloodline + per-level powers | `class_bloodrager::BLOODLINES` + `BLOODLINE_POWERS_BY_LEVEL_BY_BLOODLINE` | self-healable |
| Brawler | Flurry attack pair | `class_brawler::FLURRY_ATTACK_PROGRESSION_BY_LEVEL` | self-healable |
| Hunter | Animal companion shape | `class_hunter::ANIMAL_COMPANION_LINK_TO_LEVEL` | self-healable |
| Investigator | Inspiration pool | `class_investigator::INSPIRATION_POOL: HashMap<u8, u8>` (1 + INT + level/2) | self-healable |
| Investigator | Formula book spell list | `class_investigator::FORMULA_BOOK` (separate from arcane spell list) | self-healable |
| Shaman | Spirit companions | `class_shaman::SPIRIT_LINK_BY_LEVEL_BY_SPIRIT` | self-healable |
| Shaman | Wandering Spirit by level | `class_shaman::WANDERING_SPIRIT_BY_LEVEL: HashMap<u8, u8>` | self-healable |
| Skald | Spell Kenning | `class_skald::SPELL_KENNING_SPELL_PER_DAY_BY_LEVEL` | self-healable |
| Skald | Raging Song uses | `class_skald::RAGING_SONG_USES_BY_LEVEL: HashMap<u8, u8>` (3 + INT + level/2) | self-healable |
| Swashbuckler | Panache uses | `class_swashbuckler::PANACHE_USES_BY_LEVEL: HashMap<u8, u8>` (1 + INT + level/2) | self-healable |
| Swashbuckler | Deeds per level | `class_swashbuckler::DEEDS_BY_LEVEL: HashMap<u8, Vec<DeedRef>>` | self-healable |
| Warpriest | Blessings (level 1: 2; +1 every 4 levels) | `class_warpriest::BLESSINGS_KNOWN_BY_LEVEL: HashMap<u8, u8>` | self-healable |
| Warpriest | Sacred armor proficiencies | `class_warpriest::SACRED_ARMOR_PROFICIENCIES_BY_ITEM_KIND` | self-healable |
| ACG shared spell | Same as APG | `spell_list::ACG_SPELL_BY_NAME` | self-healable |
| ACG shared equipment | Same as APG | `equipment_tables::ACG_EQUIPMENT_BY_KEY` | self-healable |
| Cross-book ACG vs APG/CRB | ACG-only item resolves via APG/CRB | Resolver chain | self-healable |

### Epic 5 — Bestiary 1 content-source ingest failure modes

| Monster class | Failure mode | Where to look | Bucket |
|---|---|---|---|
| Goblin | CR returned as 1.0 not 0.333 | `monster_subset_01::GOBLIN.cr` (column: numeric CR value) | self-healable |
| Goblin | Initiative Dex+other = -1, +0 instead of standard | `monster_subset_01::GOBLIN.initiative` (column: derived) | self-healable |
| Goblin | HP d6+1 wrong | `monster_subset_01::GOBLIN.hp_max` (column: derived from dice + CON) | self-healable |
| Goblin | Attacks list misses `short sword +0 (1d6-2)` | `monster_subset_01::GOBLIN.attack_damage_die` (column: attack profile) | self-healable |
| Kobold | Same shape, smaller CR (0.25) | `monster_subset_01::KOBOLD.*` | self-healable |
| Subset > 1 | Missing monsters from sub-list | `monster_subset_<NN>::MONSTERS` (column: subset list) | self-healable |
| Cross-book Bestiary 1 vs class/spell | Bestiary monster returns `Some` for `RuleSetId::Apg` | Resolver chain (monsters aren't spells or equipment) | non-self-healable (genuine bug; cycle gets a fresh PR via Epic 6's happy-path integration) |
| Tarrasque | Encounter-difficulty Math throws on extreme CR | `dm_toolkit::encounter_difficulty` overflow handling | non-self-healable (boundary bug; row goes to `## Open blockers`) |

### Epic 6 — DM Toolkit failure modes

| Surface | Failure mode | Where to look | Bucket |
|---|---|---|---|
| encounters.rs | XP-multiplier table wrong (1 monster × 1.0, 2 monsters × 1.5, ..., 13+ × 4.0) | `encounter_difficulty::XP_MULTIPLIER_BY_MONSTER_COUNT` (column: monster count → multiplier) | self-healable |
| encounters.rs | Party-of-N-thresholds table wrong | `encounter_difficulty::PARTY_CR_THRESHOLD_BY_PARTY_SIZE_BY_LEVEL` (column: party size × level → CR threshold) | self-healable |
| party_cr.rs | Class-difficulty modifier | `party_challenge_rating::CLASS_DIFFICULTY_MOD_BY_CLASS` (column: class → modifier, e.g. Fighter +2, Wizard -1) | self-healable |
| party_cr.rs | Average-PF1 averaging-vs-best-fractional-progression | `party_challenge_rating::STRATEGY_BY_PARTY` (column: party strategy → averaging rule) | non-self-healable (this is the SD-22/SD-21 Epic 7 collision; if Epic 6 ships with one rule but Epic 21 ships with the other, the operator mediates) |
| Happy path | PartySnapshot from ingested Epic 3 class + Epic 5 monster → invalid encounter | `encounter_difficulty(<party>, <monsters>)` returns Err (no CR/level/size) | non-self-healable (Epic 6's happy-path requires Epic 3+4+5 ingestion to be correct first) |

### Epic 1 — identifier cleanup failure modes

| Pattern | Failure mode | Where to look | Bucket |
|---|---|---|---|
| `sd22_*` Tauri command names | grep finds remaining dirty identifier after cleanup | `grep -rE "sd22_\|SD22_\|Sd22" apps/desktop/src apps/desktop/src-tauri/src src/rules_core` | self-healable (residual identifier cleanup cycle) |
| `sd22_*` test-IDs | Same as above for `data-testid` and `AV-PAY-N` | Same grep | self-healable |
| Doc-comment `SD-22-ExN` audit IDs | Same as above in `// SD-22-Ex1: ...` style comments | Same grep | self-healable |

## Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| The DM-toolkit encounter-math (`Encounter::new`) returns a result inconsistent with the canonical Paizo encounter table | DM-toolkit deterministic test (criterion 20) fails | Rules-engine correctness violation; a PF1 player will catch it within 30 seconds; cycle can't fix the algorithm alone |
| A published source-book content record is loaded but its `RuleSetId` variant doesn't match the cycle's expected book | Per-class test fails with `RuleSetId` mismatch | Wrong-book attribution; the resolver changes; cycle can't fix this alone |
| The `codex-tranche-5` kanban board doesn't exist at cycle-1 launch | Epic 2 criterion 3 cycle fails on board lookup | Operator-side setup; bundle can't proceed until the board is created (or in this case, until the prior dead-state board is repurposed) |
| Two `claude` processes both touch `src/rules_core/rules_tables/<book>/` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule |
| Cargo test regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | Cycle's expected vs. actual differ | Manual operator reconciliation required |
| The DM-toolkit encounters.rs requires Paizo-published PF1 encounter math that requires licensing compliance | Operator-side legal review | The bundle can't ship with non-PF1-compatible rules; operator calls legal |

## Override flags (durable; patched when operator accepts a default)

### Flag A — APG 9-class per-cycle ordering

**Default chosen**: alphabetical by class name (Alchemist → Cavalier → Gunslinger → Inquisitor → Magus → Oracle → Summoner → Witch). One cycle per class.

**Override alternatives:**
- *Bestiary-style ordering* (by level progression: low-level classes first, high-level last)
- *PF1-publication ordering* (the order Paizo published APG classes in the book)
- *Tier-of-evidence ordering* (classes with strongest published evidence first)

**Override cost**: ~30 minutes; affects the cycle-ordering block of `epic-breakdown.md`.

### Flag B — ACG class per-cycle ordering

**Default chosen**: alphabetical by class name (Alchemist → Arcanist → Bloodrager → Brawler → Hunter → Investigator → Shaman → Skald → Swashbuckler → Warpriest).

**Override alternatives:**
- *Hybrid ordering* (intersperse APG and ACG classes by shared-class-priority, e.g. Alchemist appears twice across the two books)
- *Theme-grouped ordering* (casters, martial, hybrid)
- *PF1-publication ordering*

**Override cost**: ~30 minutes; affects the cycle-ordering block of `epic-breakdown.md`.

### Flag C — Bestiary 1 monster-block ordering

**Default chosen**: alphabetical by monster name within each CR band (CR 1/8 → CR 30, alphabetical within each band).

**Override alternatives:**
- *Alphabetical by environment* (group by terrain: forest / desert / urban / etc., then alphabetical)
- *PF1-publication ordering* (Paizo's Bestiary 1 page order)
- *By monster-type* (humanoid, beast, dragon, outsider, etc.)

**Override cost**: ~30 minutes.

### Flag D — DM-toolkit math surface scope

**Default chosen**: `Encounter::new` (encounter difficulty) + `party_challenge_rating` (party CR). Two functions, two modules (`encounters.rs`, `party_cr.rs`).

**Override alternatives:**
- *Single-module with both functions* (`dm_toolkit.rs` containing both `Encounter::new` and `party_challenge_rating`; simpler, but harder to test in isolation)
- *Three-module split* (`encounter_difficulty`, `party_cr`, `monster_role` adding monster-role-attack-bonus logic; richer surface but more work)
- *Full DM-toolkit GUI* (out of scope; that's `SD-23`)

**Override cost**: ~1 hour (re-doing the modular split); affects `technical-design.md` and `epic-breakdown.md`.

## Architectural questions (Q1–Q5 OPEN, defer to operator review)

### Q1 — APG/ACG class table schema: per-class Rust module or per-feature table?

**Default chosen**: per-class Rust module (`rules_tables/apg/class_<class>.rs`) with structured data entries per level per the APG's class table.

**Override alternatives:**
- *Per-feature tables* (separate `feat_table.rs`, `spell_table.rs`, `equipment_table.rs` per book) — more reusable across classes but harder to keep per-class in one place
- *Single combined module per book* (`rules_tables/apg.rs` containing all classes) — simpler but doesn't scale

**Override cost**: ~1 hour (re-doing the table layout).

### Q2 — DM-toolkit's encounter-math precision

**Default chosen**: PF1's "Encounter Building" rules with a per-monster CR-weight table (per the canonical Paizo example).

**Override alternatives:**
- *Linear scaling* (simple per-monster CR sum)
- *Logarithmic scaling* (handles extreme party-vs-monster mismatches)

**Override cost**: ~4 hours (re-deriving the math + tests).

### Q3 — Cross-book resolver priority order

**Default chosen**: per SD-21 §12 doctrine: APG → CRB → ACG → Bestiary1 (priority for cross-book fallback; independent per `RuleSetId` for content reads).

**Override alternatives:**
- *CRB-first* (legacy CRB is the canonical source)
- *Per-class priority* (e.g. Wizard-class from any book → CRB first, then APG)

**Override cost**: ~30 minutes (changing the priority order in `equipment_id_resolve` / `spell_id_resolve`).

### Q4 — Bestiary 1: full 300+ monsters or first 100?

**Default chosen**: 300+ monsters (full ingest); one cycle per monster-block subset.

**Override alternatives:**
- *100-monster minimum* (1 cycle per monster for 100 cycles) — faster to closure but incomplete
- *CR-band-by-CR-band* (low CR first, high CR later) — player-facing value incremental

**Override cost**: ~30 minutes (changing the per-cycle work unit).

### Q5 — Build version number for SD-22's first release

**Default chosen**: `0.5.<current_build>` (per the operator's 2026-07-17 `<major>.<tranche-base>.<build>` amendment, applied symmetrically). Major stays `0` until first main-publish; tranche is `5` because `tranche/5` is the active branch base; build is the next monotonic counter value after the last committed build on `tranche/5`.

**Override alternatives:**
- *Reset build to 0* (treat SD-22's launch as a fresh counter) — but operator's amendment said "monotonic across all builds across all branches — never resets"
- *Use SD-21's last build value* — operator-pinned at cycle launch

**Override cost**: 0; the operator pins the value at SD-22 cycle launch.

No remaining open architectural questions for SD-22. Future-class concerns (DM-toolkit GUI, Ultimate-line book ingest, multi-DM campaigns) are deferred to future bundles (SD-23+, SD-22 expansions).

## Open judgments deferred to next SD (Epic 9 evaluator's parking lot)

Per operator directive 2026-07-19 (Epic 9 — Closure Readiness doctrine): when Epic 9's evaluator encounters a state that looks suspicious but isn't a clean shortfall (e.g., "a rule-table entry looks wrong but a unit test passes," "an ingested book is technically correct but the cross-book resolver returns a counter-intuitive fall-through," "Epic 6's deterministic test passes against a Paizo canonical example but the example chosen is ambiguous in the source book"), Epic 9 does **not** self-heal it. Epic 9 logs the judgment here for the *next* SD's audit (e.g. SD-23) to pick up — the operator-judgment-call rule is the doctrine boundary that separates "fix in-bundle" from "defer to next bundle."

Each log entry has this shape:

```
### Judgment-<n> — <one-line summary>
- Surfaced by: Epic 9 cycle on `<YYYY-MM-DDTHH:MM:SSZ>` during eval cycle #<N>
- Criterion nearest to the observation: <criterion-NN>
- Why Epic 9 didn't self-heal: <one-line reason — typically "technically correct but counter-intuitive" or "passes unit test but reads suspect">
- Recommended next-SD action: <one-line — typically a new capability-slice in SD-23 Epic 1 (Identifier Cleanup if identifier-shaped), Epic 9 (Closure Readiness if audit-shaped), or another Epic per scope>
- Status: deferred
```

(Epic 9 adds entries to this section as it runs; the next bundle's audit reads them on first cycle.)

## Cross-reference

- `acceptance-and-verification.md` — closure gates (gates 1-16).
- `decisions.md` — the 3-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 8 epics.
- `technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md` — canonical handoff.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md` — loop body.
