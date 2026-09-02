---
title: v0.8 — B-7 choice-pool resolver audit (bridge ticket vs engine gap)
status: complete
author: backend (Fable 5.1 teammate)
date: 2026-09-01
branch: tranche/14-ui
scope_read: repo-root src/rules_core/** (read-only per brief §4.2), scripts/census_class_feature_pool_group_names.py, apps/desktop/src-tauri/src/class_feature_pool_picker.rs
---

# B-7 — Which "no list command" choice slots are bridge tickets, and which are engine gaps?

Question per ticket: for each creation-time class-choice slot with no `list_*_options` command,
does the engine hold a **queryable option table** a Tauri command could enumerate (→ bridge
ticket), or only hard-coded match arms / scattered literals (→ §4.2 engine gap)?

Ten slots audited, in the lead's priority order. Every verdict is stated explicitly. "Undetermined"
is used where a firm answer needed more digging than the slot is worth this session.

## 0. The one mechanism that decides most of these

Six of the ten slots are served by a single generic engine pass, so the verdict for those six
is really one verdict with per-slot caveats. Read this section first.

`src/rules_core/pilot_compute/mod.rs`:

| Symbol | Line | Visibility | What it does |
|---|---|---|---|
| `push_generic_pool_group_selection_magnitude` | 41207 | private | "select ONE named group, inherit every one of its real corpus powers" — resolves the recorded `choice:<slot> -> <prefix>:<slug>` selection to a real corpus `class_feature` group and grounds every member's magnitude. |
| `real_pool_group_for_selection_slug(class, registered_name, slug)` | 41435 | private | The slug → group-name resolver. Scans every corpus `class_feature` record's `" ~ "`-qualified group, tallies each group's owner class by majority across its members, keeps groups owned by `class`, strips the `registered_name` suffix (or the `"<Name> of the "` prefix for Cavalier, or a trailing owner-class word for Bloodrager), slugs the remainder with `class_feature_id_slug`, and returns the group whose slug equals `slug`. `None` for anything else. |
| `class_feature_id_slug` | 20933 | private | lowercase, `' '`/`'-'` → `'_'`, drop everything non-alphanumeric. |
| `class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe` | `class_feature_grant_consumer.rs:944` | `pub(crate)` | The corpus record table the resolver scans (key, class, tokens). |

Callers of the generic pass (command: `grep -n -A7 "push_generic_pool_group_selection_" src/rules_core/pilot_compute/mod.rs`):

| choice set | class | registered name | id prefix |
|---|---|---|---|
| `choice:cleric_domain` | Cleric | Domain | `domain:` |
| `choice:sorcerer_bloodline` | Sorcerer | Bloodline | `bloodline:` |
| `choice:bloodrager_bloodline` | Bloodrager | Bloodline | `bloodline:` |
| `choice:cavalier_order` | Cavalier | Order | `order:` |
| `choice:shaman_spirit` | Shaman | Spirit | `spirit:` |
| `choice:warpriest_blessing` | Warpriest | Blessing | `blessing:` |

**What this means for a bridge.** The *option universe* for these six is queryable today from
`codex::rules_core::class_feature_pool_catalog::load_pool_catalog` (pub; the desktop already
calls it in `class_feature_pool_picker.rs`), whose `PoolCatalogEntry.pool_group` is the same
`" ~ "`-group name the resolver matches, and whose `name`/`description` give display text. The
group list per class is reproducible with
`python3 scripts/census_class_feature_pool_group_names.py`. So the raw table exists.

**The two things a bridge cannot get without touching the engine:**

1. **The slug rule and the ownership rule are private.** `real_pool_group_for_selection_slug`
   and `class_feature_id_slug` are `fn`, not `pub`. `PoolCatalogEntry` does not carry the
   owner class (its doc says `data.class` carries it but the struct drops it), so a src-tauri
   command cannot apply the majority-owner filter — it can only filter by suffix, which admits
   umbrella groups that are containers, not options (e.g. `"Core Domain"` 110 records,
   `"Cleric Domain"` 22, `"Sorcerer Bloodline"` 32, `"Shaman Spirit"` 12 — all real groups,
   none of them a pickable domain/bloodline/spirit). Re-implementing both rules in src-tauri
   is a second copy of engine logic that would drift silently; this file's own doctrine
   (AGENTS.md rule 8, decisions §29.2 "twin") argues against it.
2. **"Resolves" is not "computes".** For Cleric and Bloodrager (proven; see §1, and
   `an_unrecognized_bloodrager_bloodline_keeps_the_blocker` at mod.rs:76897) a selection the
   generic pass resolves but the hand-modelled catalog does not recognise still leaves the build
   **claim-blocked**. A picker listing 70 domains where 64 block the save is the exact
   "looks configured, computes as blocked" shape the no-stub doctrine forbids.

**Recommended shape (applies to all six):** one small engine change — make
`real_pool_group_for_selection_slug` + `class_feature_id_slug` `pub`, or better, add a
`pub fn list_pool_group_selections(class, registered_name) -> Vec<(slug, group_name)>` that
inverts the resolver over the same table — then each `list_*_options` command is a trivial
bridge over it, and the option list can never disagree with what the engine will accept. That
engine change is a §4.2 item for the concurrent engine session, but it is a ~30-line
visibility/inversion change, not a new capability. Until it lands, each of the six is
**bridge-blocked-on-visibility**, which I record below as "bridge ticket (gated)".

---

## 1. `choice:cleric_domain` — **bridge ticket (gated), with a hard claim-status caveat**

- **Resolver:** `mod.rs:46300-46420` (Cleric block). Two layers:
  - generic pass (`push_generic_pool_group_selection_magnitude`, "Cleric"/"Domain"/`domain:`)
    grounds powers for any resolvable domain slug;
  - hand-modelled `domain_power::DOMAIN_POWER_CATALOG` (`domain_power.rs:378`, `pub(super)`,
    **5 entries**; command: `grep -cE '^\s+DomainPowerSpec \{' src/rules_core/pilot_compute/domain_power.rs`)
    plus the `GOOD`/`HEALING` constants. Slugs named in `mod.rs` consts: good, healing, war,
    strength, + 2 more from SD-31 wave 26 (`sed -n 6480,6510p src/rules_core/pilot_compute/mod.rs`).
- **Queryable table:** yes for the universe — pool catalog groups with suffix `" Domain"` owned by
  Cleric; census reports **73 groups** (`python3 scripts/census_class_feature_pool_group_names.py | grep -c "'Domain' (Cleric)"`),
  of which 3 are umbrellas (`Core Domain`, `Cleric Domain`, `Forbidden Rites Domain`). Display
  names: yes (group name minus " Domain").
- **Claim status:** `mod.rs:46354-46362`: `unrecognized_other_domain_chosen` = any selection not
  Good/Healing and not in `DOMAIN_POWER_CATALOG` → the catch-all diagnostic fires and the
  build stays **Blocked** (pinned by `single_class_cleric_with_an_unrecognized_domain_stays_blocked_on_the_catch_all`,
  mod.rs:59875, which uses `domain:fire`). So today the *usable* list is ~6 domains, not 73.
- **Verdict: bridge ticket (gated).** The table exists and is enumerable; the option list must be
  restricted to what does not claim-block, and that set lives in a `pub(super)` catalog. Needs
  the engine to expose either the catalog or a "which domains compute" query. Listing all 73
  is an engine gap (64+ domains block).
- **Also note:** PF1 clerics pick TWO domains; the composer seeds one (`domain:good`) and the
  hand-modelled explanation at `mod.rs:46680` only fires for the exact Good+Healing pair. B-6's
  replace-wholesale semantics let a caller send two entries for the set.

## 2. `choice:sorcerer_bloodline` — **bridge ticket (gated)**

- **Resolver:** `mod.rs:42715-42740` (generic pass, "Sorcerer"/"Bloodline"/`bloodline:`) plus
  hand-modelled Arcane (`ARCANE_BLOODLINE_SELECTION_ID`, mod.rs:1951) and Draconic (:1952,
  Dragon Resistances at level 3). Recognition explanation at :43021 names only Arcane as
  "recognized".
- **Queryable table:** yes — census lists **52 groups** with suffix `" Bloodline"` owned by
  Sorcerer (one umbrella, `Sorcerer Bloodline`, 32 records). Display names: yes.
- **Claim status for non-Arcane/Draconic bloodlines:** **undetermined.** I found the
  Bloodrager analogue is blocked (§0 item 2) and the Sorcerer explanation text says "only the
  canonical Arcane bloodline selection is recognized", but I did not locate a Sorcerer
  claim-status test for an unrecognised bloodline. Needs one `build_pilot_headless_receipt`
  probe before a picker ships.
- **Verdict: bridge ticket (gated)** on the §0 visibility change, with the claim-status probe as
  its first red test.

## 3. `choice:wizard_school_specialization` + `choice:wizard_opposed_schools` — **bridge ticket for the list; engine gap for free opposed-school pairs**

- **Resolver:** five hand-coded gates, `mod.rs:44336-44460`, each an exact triple:

  | specialty | required opposed pair |
  |---|---|
  | evocation | necromancy + transmutation |
  | abjuration | necromancy + transmutation |
  | transmutation | necromancy + evocation |
  | conjuration | necromancy + abjuration |
  | universal | exactly zero opposed entries |

  Any other specialty (divination, enchantment, illusion, necromancy) or any other opposed pair
  returns `false` from every gate → no school power grounded, "universalist-shaped" silence.
- **Queryable table:** yes, and already `pub` — `class_feature_pool_catalog::WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER`
  (`class_feature_pool_catalog.rs:876`, 9 entries, `"<School> Wizard Spells"` keys; display name =
  key minus `" Wizard Spells"`). The 9 `school:<slug>` ids match the engine literals exactly
  (`grep -rhoE '"school:[a-z_]+"' src/rules_core | sort -u` → 9).
- **Verdict:** listing the 9 schools is a **bridge ticket, no engine change needed** (the const
  is pub). But the engine grounds only the 5 fixed triples above, so a picker that lets the
  player choose specialty × any 2 opposed (the real PF1 rule: any two except own school, and
  Divination may not be opposed) will silently ground nothing for ~90% of legal combinations.
  Free opposed-pair support is an **engine gap**. Honest interim: picker offers the 5 supported
  combos only, greyed-with-reason for the rest.

## 4. `choice:druid_nature_bond` — **engine gap**

- **Resolver:** `mod.rs:7006-7007`, `:47489`, `:47741`. Exactly one recognised id,
  `bond:animal_companion` (hard-coded const). `bond:domain` appears only in a test
  (`mod.rs:60357`). The doc comment at :7010-7020 records that the domain option is
  hard-restricted to Air/Animal/Earth/Fire/Plant/Water/Weather and was checked, not modelled.
- **Queryable table:** none (0 pool-catalog groups match "Nature Bond"). PF1 has exactly two
  options, so a "table" would be a two-row literal in src-tauri, and one of the two rows does
  nothing in the engine.
- **Verdict: engine gap.** Animal companion is already wired through `companionSpecies`
  (`list_companion_catalog`); the domain half needs an engine slot for the restricted domain
  sub-choice before a picker is honest. Nothing to bridge.

## 5. `choice:oracle_mystery` — **bridge ticket (gated); engine gap for 11 of 21 corpus mysteries**

- **Resolver:** `mod.rs:16496-16760`. Two layers, neither generic:
  - `ORACLE_MYSTERY_POOL` (`mod.rs:3570`, **private** `&[&str]`, 10 hand-transcribed ids:
    battle, bone, flame, heavens, life, lore, nature, stone, waves, wind) consumed via
    `archetype_resolver::chooser_option_selected` (pub, `archetype_resolver.rs:175`), which
    refuses any id not in the passed pool;
  - per-mystery hand-modelled tier-1 revelations (Life's Healing Hands etc.), each also
    requiring an explicit `choice:oracle_revelation` pick.
  There is **no** generic pool pass for Mystery (not in the §0 caller list).
- **Queryable table:** corpus has **21 `" Mystery"` groups** owned by Oracle (census), with names
  and member descriptions. The engine's accepted list is the private 10-id const.
- **Verdict: bridge ticket (gated)** for the 10 — needs `ORACLE_MYSTERY_POOL` made `pub` (one
  keyword) so the command can't drift from the engine. The other 11 corpus mysteries (Ancestor,
  Apocalypse, Dark Tapestry, Intrigue, Metal, Outer Rifts, Spellscar, Time, Wood, two
  "Speaker for the Past" variants) are an **engine gap**. `choice:oracle_curse` has **0** corpus
  groups and 5 literal ids; treat as engine-gap-until-shown-otherwise (not audited further).

## 6. `choice:ranger_favored_enemy` — **engine gap**

- **Resolver:** `mod.rs:34915-34928`. The selection is read and echoed verbatim into a `+0`
  recognition explanation. **No creature-type table exists**: the only `enemy:*` literals in the
  crate are `enemy:first` … `enemy:fifth` (ordinal *bump-target* ids for the level-5/10/15/20
  bonus-increase slot, `mod.rs:1055-1131`), not types. A comment at `mod.rs:8549` cites a
  `FAVORED_ENEMY_TYPES` table "(cycle 3)" but `grep -n "FAVORED_ENEMY_TYPES" src/rules_core/pilot_compute/mod.rs`
  finds only that comment — the table does not exist.
- **Queryable table:** none (0 pool-catalog groups; no const).
- **Verdict: engine gap.** Any string the UI sends "works" (it is never matched), which is worse
  than a missing list — a picker would be inventing the PF1 favored-enemy type table in TS
  (§3.3 forbids). The engine needs the type table before this is a bridge.

## 7. `choice:cavalier_order` — **bridge ticket (gated), with an omission caveat**

- **Resolver:** `mod.rs:14247-14290`. Generic pass ("Cavalier"/"Order"/`order:`) using the
  resolver's third naming shape (`"Order of the <X>"` prefix strip + chooser-header ownership
  proof) plus the `push_generic_pool_group_selection_description_magnitude` sibling for
  Order of the Beast, plus hand-modelled `order:sword` (`ORDER_OF_THE_SWORD_SELECTION`,
  mod.rs:2753).
- **Queryable table:** yes — my pool-catalog probe found **20 groups** matching `Order of the`
  (case-varied). Note the census script reports only the umbrella `Cavalier Order` (7 records)
  because it lacks the engine's header-record ownership fallback — so the census under-counts
  this pool; use the engine's resolver, not the script, for Cavalier.
- **Caveat:** `Order of the Sword` has **no** corpus chooser header (resolver doc, mod.rs:~41480:
  "correctly NOT picked up by this fallback"), so a purely catalog-derived list would omit the
  one order the engine hand-models best. The list must union the catalog with the hand-modelled
  ids.
- **Verdict: bridge ticket (gated)** on §0 visibility; claim status for orders other than Sword
  is **undetermined** (not probed).

## 8. `choice:witch_hex` — **engine gap**

- **Resolver:** `mod.rs:18684-18695` (`witch_has_hex`) and `:19029-19045`. Member-level pool:
  exactly three hand-coded ids compute — `hex:ward`, `hex:cauldron`, `hex:flight`
  (`hex:slumber` appears only in a test at :68924). **No generic member pass** for hexes. An
  unrecognised hex keeps the claim-blocker (the comment at :19038 names the pinning test
  `an_ungrounded_witch_hex_selection_still_claim_blocks`).
- **Queryable table:** yes for the universe — census: `Witch Hex` 65 + `Witch Major Hex` 16 +
  `Witch Grand Hex` 8 records, with names/descriptions. Member key → `hex:<slug>` id would go
  through the private `class_feature_id_slug`.
- **Verdict: engine gap.** Listing 65 hexes where 62 block the save is a stub-shaped picker. The
  three that work could ship as a bridge, but a "choose from 3 of 65" picker is not the item-13
  deliverable. Engine needs a generic member-level hex pass (the Rogue-Talent-style shape) first.

## 9. `choice:shaman_spirit` — **bridge ticket (gated)**

- **Resolver:** `mod.rs:22545-22600` + `ground_shaman_spirit_base_ability` (:22671). Generic pass
  ("Shaman"/"Spirit"/`spirit:`) **plus** hand-modelled base ability for all 10 primary spirits
  (`mod.rs:3931-3945`: life, battle, bones, flame, heavens, lore, nature, stone, waves, wind —
  the doc says the corpus was verified uniform: each spirit's one ungated base ability).
- **Queryable table:** yes — census: **14 groups** with suffix `" Spirit"` owned by Shaman
  (2 umbrellas: `Shaman Spirit`, `Shaman Wandering Spirit`; 1 extra real spirit, `Mammoth`, +
  `Wood` beyond the hand-modelled 10). Display names: yes.
- **Claim status:** the `spirit_recognized` flag at :22589 is set by the hand-modelled branches;
  Mammoth/Wood would resolve through the generic pass but likely stay blocked —
  **undetermined**, one probe needed.
- **Verdict: bridge ticket (gated)** — the closest to "just wire it" of the ten, because the
  hand-modelled set (10) nearly equals the corpus set (12 real).

## 10. `choice:bloodrager_bloodline`, `choice:warpriest_blessing` — **bridge ticket (gated)** (not in priority list; same mechanism)

Both are §0 generic-pass callers. Bloodrager: 12 real corpus groups (resolver doc), hand-modelled
Arcane only, unrecognised bloodline **keeps the blocker** (proven, mod.rs:76897). Warpriest: 36
`" Blessing"` groups (+ umbrella `Blessings` 37 records), hand-modelled Destruction/Strength. Same
gating and same claim-status caveat as Cleric.

---

## Summary table

| slot | table exists? | where | names? | verdict |
|---|---|---|---|---|
| cleric_domain | yes (73 groups) | pool catalog via private resolver; usable set = `DOMAIN_POWER_CATALOG` (5+2, `pub(super)`) | yes | **bridge (gated)**; all-73 is engine gap |
| sorcerer_bloodline | yes (52) | same | yes | **bridge (gated)**; claim status undetermined |
| wizard schools | yes (9, `pub` const) | `WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER` | derivable | **bridge** for list; **engine gap** for free opposed pairs |
| druid_nature_bond | no | one hard-coded id | — | **engine gap** |
| oracle_mystery | engine: private 10-id const; corpus: 21 groups | `ORACLE_MYSTERY_POOL` | yes (corpus) | **bridge (gated)** for 10; **engine gap** for 11 |
| ranger_favored_enemy | no | selection echoed, never matched | — | **engine gap** |
| cavalier_order | yes (20) | private resolver, 3rd shape | yes | **bridge (gated)**; must union hand-modelled Sword |
| witch_hex | universe yes (89 members); engine 3 ids | hand-coded | yes (corpus) | **engine gap** |
| shaman_spirit | yes (12 real) | private resolver + 10 hand-modelled | yes | **bridge (gated)** — best candidate |
| bloodrager / warpriest | yes | private resolver | yes | **bridge (gated)** |

**What "gated" needs from the engine session, once, for all seven bridge slots:** a `pub`
inversion of `real_pool_group_for_selection_slug` (or `pub` on it + `class_feature_id_slug`),
plus `pub` on `ORACLE_MYSTERY_POOL` and `DOMAIN_POWER_CATALOG` (or a "which selections
compute" query per pool). Estimated as visibility + one ~30-line function; no new rules logic.
Without it, every bridge is a second copy of the slug/ownership rule in src-tauri, and the
option list drifts from what the engine accepts — the exact silent-wrong shape that made the
B-6 replace-vs-append call necessary.

**Dispatchability of audit item 13 this session:** not as a whole. The honest interim that IS
dispatchable without any engine change is the Wizard specialty list (9 schools, `pub` const) with
the 5 supported opposed-pair combos, and — if the lead accepts a src-tauri copy of the slug rule
with a parity test against the engine — Shaman Spirit for the 10 hand-modelled spirits.
Everything else waits on the visibility change or is an engine gap.

## Commands behind every figure in this file

- generic-pass callers: `grep -n -A7 "push_generic_pool_group_selection_" src/rules_core/pilot_compute/mod.rs`
- per-class corpus groups: `python3 scripts/census_class_feature_pool_group_names.py`
- pool-catalog groups by suffix: probe test over `list_class_feature_pool_options()` grouped by `(book, pool_group)` — 1147 distinct groups (transient test, deleted; re-derive with the same loop)
- literal ids per prefix: `grep -rhoE '"<prefix>:[a-z_0-9]+"' src/rules_core --include=*.rs | sort -u`
- `DOMAIN_POWER_CATALOG` size: `grep -cE '^\s+DomainPowerSpec \{' src/rules_core/pilot_compute/domain_power.rs`
- claim-block pins: `grep -n "stays_blocked_on_the_catch_all\|keeps_the_blocker\|still_claim_blocks" src/rules_core/pilot_compute/mod.rs`
