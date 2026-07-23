# Cycle ultimate_campaign_stub_manifest — Epic 4 / Criterion 4.17

- **Card ID:** (see kanban step, below)
- **Commit SHA:** a7df999 (manifest + registry entry), 8b39041 (this receipt) — both landed on
  `origin/tranche/5-4` after the push-time rebase
- **Files touched:**
  - `data/stubs/ultimate_campaign.json` (new — per-book stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0018; updated the
    reserved-entries footer note)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.17 — Book stub manifest for `ultimate_campaign`.
  Write `data/stubs/ultimate_campaign.json` following the pilot's exact shape and add the next
  numbered `book_stub` entry to the Stubs Registry, replicating #0003's pattern exactly.
- **Status:** complete
- **Notes:**
  - **Replicated the established pattern, no redesign.** Both the manifest shape
    (`{book_id, book_name, planned_resolution_bundle, content_kind_counts: null,
    registered_at}`) and the registry entry's seven-field `book_stub` template were copied
    field-by-field from #0003 (pilot) / #0017 (`pathfinder_unchained`, the most recent prior
    cycle), substituting only `book_id`/`book_name`/entry number/criterion number/timestamp.
  - **Registry entry number resolved at edit time, not assumed in advance.** Re-fetched and
    rebased onto `origin/tranche/5-4` immediately before editing the registry file (per the
    shared-state protocol). At that point the highest landed heading was `#0017`
    (`pathfinder_unchained`, criterion 4.16), so this cycle claimed `#0018`. Also caught and
    corrected a path error mid-cycle: the manifest was first accidentally written into the main
    repo checkout (`/home/ubuntu/workspace/repos/codex/data/stubs/...`, a sibling process's live
    working tree) instead of this cycle's own worktree; removed the stray untracked file from
    the main checkout before redoing the write at the correct worktree path, so no cross-cycle
    interference occurred.
  - **Book identity verified against real source, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign/_ultimate_campaign.pcc`:
    `CAMPAIGN:Ultimate Campaign`, `SOURCELONG:Ultimate Campaign`, `SOURCESHORT:UCA`,
    `PUBNAMELONG:Paizo Publishing`, `SOURCEDATE:2013-05`, `BOOKTYPE:Supplement` — a real Paizo
    PF1 sourcebook (May 2013) described in its own `DESC:` field as covering downtime/kingdom-
    building/army rules, character background generation, and other "between adventures" systems
    (leadership, businesses, magic item crafting). The directory has real LST content files
    confirming coverage: `uca_abilities_drawbacks.lst`, `uca_abilities_traits.lst`,
    `uca_abilities_retraining*.lst` (plus APG/UC/UM retraining cross-tie-ins),
    `uca_abilitycategories.lst`, `uca_companionmods.lst`, `uca_feats.lst`. `book_name` set to
    `"Ultimate Campaign"` (the `SOURCELONG`/`CAMPAIGN` value), matching the convention every
    prior `book_stub` entry used (e.g. `"Horror Adventures"`, `"Pathfinder Unchained"`). None of
    this content is ingested into `data/corpus/ultimate_campaign/` (directory does not exist),
    confirming the stub is honest — a genuine data-completeness gap, not a fabricated one.
- **Verification:**
  - `python3 -c "import json; ..."` — confirmed `data/stubs/ultimate_campaign.json` is valid
    JSON with the exact same key set and key order as the pilot `advanced_race_guide.json`
    (`book_id`, `book_name`, `planned_resolution_bundle`, `content_kind_counts`, `registered_at`),
    `content_kind_counts` is JSON `null` (not `0` or a string), and `book_id` /
    `planned_resolution_bundle` hold the expected literal values (`"ultimate_campaign"` /
    `"SD-27+ (unscheduled)"`).
  - `grep -n "ultimate_campaign" docs/governance/wired-integration-stubs-registry.md` —
    confirms the new #0018 entry landed with the `book_stub` structural pattern (all seven fields:
    Book / manifest path, What's missing, Justification, Audit-grep impact, Bundle-of-record,
    Remediation cycle, Status) matching #0003-#0017's template field-by-field.
  - `cargo test --locked --test sd26_identifier_discipline_audit` — 1/1 pass
    (`no_bundle_tag_identifier_leaks_in_scripts_and_data`).
  - Dual-audit gate re-run against the new (committed) diff: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`.
- **Discovery forwards:** none new this cycle (the `decisions.md §10` / `"SD-27"` vs.
  `"SD-27+ (unscheduled)"` discrepancy was already forwarded by criterion 4.1's receipt).
- **Next-cycle plan:** as of this cycle's rebase, remaining unclaimed future-state books were
  `ultimate_combat`, `ultimate_equipment`, `ultimate_intrigue`, `ultimate_magic`,
  `ultimate_wilderness`, plus criterion 4.12's book — re-derive the actual unclaimed set from
  `data/stubs/` + the registry on disk at cycle start, per the established pivot protocol from
  prior cycles' receipts, since sibling cycles run concurrently and may claim books between this
  receipt being written and the next cycle starting.
