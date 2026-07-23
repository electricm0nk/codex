# Cycle ultimate_combat_stub_manifest — Epic 4 / Criterion 4.18

- **Card ID:** (see kanban step, below)
- **Commit SHA:** `a371fc3` (manifest + registry entry #0019) — see `progress.md` for the final landed SHA after rebase/push.
- **Files touched:**
  - `data/stubs/ultimate_combat.json` (new — per-book stub manifest for `ultimate_combat`)
  - `docs/governance/wired-integration-stubs-registry.md` (added registry entry #0019 for `ultimate_combat`; updated the reserved-entries footer note to reflect 4 remaining books/#0020-000n)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.18 — Book stub manifest for `ultimate_combat`. Replicate criterion 4.1's landed `book_stub` pattern exactly (not redesigned): write `data/stubs/ultimate_combat.json` in the pilot's exact shape and add the next-numbered registry entry following #0003's template.
- **Status:** complete
- **Notes:**
  - **Replicated, did not redesign.** Used `content-unit-inventory.md §2.1`'s shape exactly:
    `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null,
    registered_at: <ISO-8601>}`. Verified by direct key-order/type comparison against the pilot
    `data/stubs/advanced_race_guide.json` (`python3 -c "import json; ..."` — see Verification
    below).
  - **Book identity verified against real source, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat/_ultimate_combat.pcc`:
    `CAMPAIGN:Ultimate Combat`, `SOURCELONG:Ultimate Combat`, `SOURCESHORT:UC`,
    `PUBNAMELONG:Paizo Inc.`, `SOURCEDATE:2011-01` — a real Paizo PF1 hardcover supplement
    (per the `.pcc` file's own `PRECAMPAIGN` dependency on the Advanced Player's Guide, and the
    real Paizo product page URL embedded in `SOURCEWEB`). The directory also has real LST content
    files confirming coverage: `uc_abilities.lst`, `uc_abilities_class.lst`,
    `uc_abilities_globalvar.lst`, `uc_abilities_race.lst`, `uc_abilitycategories.lst`,
    `uc_classes.lst`, `uc_companionmods.lst`, `uc_equip_arms_armor.lst`, `uc_equip_general.lst`,
    `uc_equip_magic_items.lst`, `uc_equipmods.lst`, `uc_feats.lst`, `uc_kits.lst`,
    `uc_profs_armor.lst`, `uc_profs_weapon.lst`, `uc_races.lst`, `uc_skills.lst`, `uc_spells.lst`,
    `uc_templates.lst`, plus a `support/` subdirectory. `SOURCEDATE:2011-01` used verbatim as
    "January 2011" for consistency with how prior `book_stub` entries sourced their publish
    month/year directly from each book's own `.pcc` `SOURCEDATE` field (e.g. `bestiary_2` used
    `SOURCEDATE:2010-12` → "December 2010"; `ultimate_campaign` used `SOURCEDATE:2013-05` → "May
    2013" — confirmed by re-checking both `.pcc` files this cycle, not assumed). `book_name` set
    to `"Ultimate Combat"` (the `SOURCELONG`/`CAMPAIGN` value, matching the convention every prior
    `book_stub` entry used). None of this content is ingested into `data/corpus/ultimate_combat/`
    (directory does not exist), confirming the stub is honest.
  - **Registry entry number (#0019).** At the pre-implementation rebase, the highest registry
    heading on disk was `#0018` (`ultimate_campaign`, criterion 4.17) and `ultimate_combat` was
    unclaimed, so this cycle used `#0019`. Re-confirmed immediately before editing the shared
    registry file (re-fetch + rebase per step 6/§5) that `#0018` was still the highest heading and
    no sibling cycle had already claimed `ultimate_combat` or `#0019` — no renumbering was needed
    this cycle (unlike the `pathfinder_unchained`/4.16 cycle, which hit a genuine concurrent-write
    collision and had to renumber #0016→#0017).
  - **`planned_resolution_bundle` value.** Used `"SD-27+ (unscheduled)"`, matching the pilot's
    (#0003's) corrected value and every subsequent `book_stub` entry (#0004-#0018), not
    `decisions.md §10`'s originally-pinned `"SD-27"` literal — consistent with the discrepancy
    resolution already recorded by criterion 4.1's receipt.
- **Verification:**
  - `python3 -c "import json; ..."` — confirmed `data/stubs/ultimate_combat.json` is valid JSON
    with the exact same key set and key order as the pilot `advanced_race_guide.json` (`book_id`,
    `book_name`, `planned_resolution_bundle`, `content_kind_counts`, `registered_at`),
    `content_kind_counts` is JSON `null` (not `0` or a string), `registered_at` parses as a valid
    ISO-8601 UTC timestamp, and `book_id` / `book_name` / `planned_resolution_bundle` hold the
    expected literal values.
  - `grep -n "ultimate_combat" docs/governance/wired-integration-stubs-registry.md` — confirms
    the new #0019 entry landed with the `book_stub` structural pattern (all seven fields: Book /
    manifest path, What's missing, Justification, Audit-grep impact, Bundle-of-record,
    Remediation cycle, Status) matching #0003-#0018's template field-by-field.
  - Dual-audit gate (§6 identifier + wired-integration greps) re-run against
    `${BASE_BRANCH}...HEAD` after committing: both `OK_*` (`OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`).
- **Discovery forwards:** None new this cycle — the `decisions.md §10` `planned_resolution_bundle`
  discrepancy was already forwarded by criterion 4.1's receipt.
- **Next-cycle plan:** Criteria covering the remaining 4 unclaimed books (`core_essentials` done
  by 4.10 — already landed; remaining are tracked in the registry footer note) continue the same
  mechanical replication: pick the next unclaimed book from `content-unit-inventory.md §2.2`,
  write `data/stubs/<book_id>.json` in this exact shape, add the next-numbered registry entry
  (re-verifying the highest entry number on disk immediately before editing, per the
  concurrent-write protocol), and update the reserved-entries footer note.
