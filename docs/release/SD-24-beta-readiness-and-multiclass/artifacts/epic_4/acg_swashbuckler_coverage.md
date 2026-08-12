# ACG Swashbuckler — Per-Class Coverage (SD-24 Epic 4, criterion 4.3)

**Updated (v0.6 alpha swarm, risks item 8, Swashbuckler full-build
closure, 9th ACG/APG class-specific closure, 2026-07-25):** Panache's
flat daily maximum, Charmed Life (level-gated from 2nd level, activation-
gated with a real per-day budget), and Nimble's flat AC dodge bonus are
now genuinely wired, plus the class-skill-bonus widening fix (Climb/
Intimidate/Swim, the fourth class needing it). This closure was mis-
scoped as "harder" alongside Investigator/Shaman/Summoner/Witch in the
second comparative pass — corrected in the third pass after checking the
real corpus directly (a non-caster, zero spellcasting scope, no new
subsystem required). Swashbuckler stays permanently `Blocked` on its own
`other_features_deferred` diagnostic (Deeds, Swashbuckler Finesse, and
every other named feature remain deferred). See
`docs/release/v0.6/third-full-class-build-comparative-scoping.md` for
the full record.

| Field | Value |
|---|---|
| `class_name` | Swashbuckler |
| `book` | ACG |
| `feature_table_path` | `rules_core::rules_tables::acg::class_swashbuckler` (chassis); `pilot_compute::ground_or_block_swashbuckler_class_features` (Panache/Charmed Life/Nimble) |
| `feature_table_sha` | PCGen corpus `7f818006e371188e5717fd18d74d18a420747fc6` (2026-06-17), `advanced_class_guide/acg_classes.lst` (chassis) + `acg_abilities_class.lst` (named features) |
| `class_features_expected` | 29 (distinct `KEY:Swashbuckler ~ ...` records in `acg_abilities_class.lst`) |
| `class_features_wired` | 3 (Panache, Charmed Life, Nimble — three structurally independent mechanisms, no shared table linking them) |
| `chassis_rows_wired` / `chassis_rows_expected` | 20 / 20 |
| `pilot_compute_integrated` | Yes — `compute_class_chassis`/`compute_acg_class_chassis` in `pilot_compute.rs` recognizes all 10 ACG classes; Swashbuckler additionally gets its own Panache/Charmed Life/Nimble dispatch via `ground_or_block_swashbuckler_class_features` |
| `level_up_wired` | No — no `level_up::swashbuckler` module exists |
| `gap_priority` | P2 (real, verified partial coverage; permanently `Blocked` on the remaining named features) |

## Gap features (from `acg_abilities_class.lst`, corpus commit above)

Deeds (+ its many individually named Deeds — Dodging Panache, Kip-Up, Menacing Swordplay, Opportune Parry and Riposte, Precise Strike, Swashbuckler Initiative, Swashbuckler Weapon Training, ...), Swashbuckler Finesse, Bonus Feats, Swashbuckler Weapon Mastery, Swashbuckler's Grace, Swashbuckler's Edge, and every other named feature beyond Panache/Charmed Life/Nimble.

## Verification

- Chassis math cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens (see `class_swashbuckler.rs`'s own doc comment for the exact transcribed tokens) — matches, no defect.
- Charmed Life's real grant level (2nd, not 1st) and its formula (`((SwashbucklerLVL-2)/4)+3`) both verified: the corpus record itself carries no level-gate token, so a real web search confirmed "at 2nd level... three times per day... at 6th level and every 4 levels thereafter... increases by one" — matches the formula exactly at levels 2/6/10.
- Nimble's `(SwashbucklerLVL+1)/4` dodge bonus verified directly against `acg_abilities_class.lst`'s own `BONUS:VAR|SwashbucklerDodgeBonus|(SwashbucklerLVL+1)/4` tag.
- Panache's `max(1, Charisma modifier)` formula is sourced from the ability's own DESC text, not a literal `BONUS:VAR` token — confirmed the base Swashbuckler's own Panache record defines no `Panache_Cap`/`PanachePoints` formula anywhere in this corpus checkout (only the Inspired Blade archetype and the Extra Panache feat set/adjust these variables). Cross-validated against the Inspired Blade archetype's own explicit `BONUS:VAR|Panache_Cap|MAX(1,CHA)+MAX(1,INT)`, which adds an Intelligence term on top of a base `MAX(1,CHA)` — confirming the base formula. A different evidentiary path than every other formula this segment has grounded (all had a literal `BONUS:VAR` token), named honestly rather than treated as equivalent-confidence.
- `tests/sd24_acg_class_coverage_audit.rs` exercises this row via `rules_tables::acg::class_coverage(AcgClassId::Swashbuckler)` and the live `compute_pilot_base_chassis` seam; `pilot_compute::swashbuckler_dispatch_widening_safety_tests` (6 tests) exercises the full Panache/Charmed Life/Nimble dispatch directly.

