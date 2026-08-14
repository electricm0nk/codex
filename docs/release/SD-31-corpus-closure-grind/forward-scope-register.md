# SD-31 Forward-Scope Register

Successor work depending on this package's output. Format follows
`SD-30-class-feature-archetype-bundle/forward-scope-register.md`'s convention (one row per item, named
owner, no unowned tidiness entries).

| ID | Item | Owner |
|---|---|---|
| G1.1 | The ~2,894-unit chassis-blocked `race_trait` remainder, and `race`'s 100 unfunded units, stay outside this package's own ceiling until `SD-32-engine-capability-builds`'s race-chassis epic lands. Not this package's work to fund. | `SD-32-engine-capability-builds` |
| G1.2 | Any `class_feature` `unknown`-bucket unit Epic 1-F4 characterizes as "genuinely unreachable" (needs net-new engine work, no chooser code at all) is a named finding, not silently deferred. | `SD-32-engine-capability-builds` (verdict-path epic), pending operator funding per-item |
| G1.3 | `equipment`/`equipment_modifier`/`companion`/`feat`/`monster_ability` ingest lanes — real `not-started` residue exists (per `SD-30-.../decisions.md §44`'s own "not in this epic" note) but was not prioritized into a card by SD-30's original Epic 10 fold, and this package inherited that same scoping. A future card can open under this package's Epic 4 without a new operator ruling. | This package, unclaimed card |
| G1.4 | The per-class PI-blacklist sweep this package's Epic 3 (`class_feature` chassis sweep, ex-SD-30 Epic 6) must call before writing any generated table: `codex::rules_core::pi_table_sweep::screen_generated_table` (shared `pi_screening::PI_BLACKLIST_TERMS`), already built, already production-wired (two live callers, `gen_feat_gap_tables.rs`/`gen_equipment_gap_tables.rs`), proven against real `class_feature` content by `SD30-E3-F1-001`. Six-step invocation contract: `SD-30-.../decisions.md §52.3`; do not re-derive or fork the term list. | This package's Epic 3 (chassis-sweep ingest binary) |

No item above is left without a stated home, per this program's standing "unowned deferral is not a
valid disposition" discipline (`SD-30-.../decisions.md §27`).
