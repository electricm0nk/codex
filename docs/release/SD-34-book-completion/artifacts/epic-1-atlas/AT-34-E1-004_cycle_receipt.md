# Cycle 4 — Epic 1 Completion Atlas / AT-34-E1-004

- **Commit SHA:** `4d69afd6e4`
- **Files touched:** `scripts/shape_engine_boundary.py` (new), `scripts/tests/test_shape_engine_boundary.py` (new), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/shape-engine-boundary.md` (new, generated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** "A committed statement, proven by execution, of what a shape engine does and where its output stops -- so no future bundle re-learns it. **Evidence:** `artifacts/epic-1-atlas/shape-engine-boundary.md`, carrying the count of magnitude-bearing units (**26,396**), how many of those the engine still does not hold (**13,119 of 26,396**), and the four-condition promotion ladder quoted from `src/bin/v06_work_inventory.rs` with its line number re-verified at HEAD." (`epic-breakdown.md`, verbatim)
- **Figures + their re-derive commands:**
  - `magnitude_bearing=26396` — `python3 scripts/shape_engine_boundary.py --check` (denominator: 26,396 of `docs/work-inventory.json`'s full 49,438-unit population; independently cross-checked: `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1 for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0))"` → `26396`)
  - `not_held_by_engine=13119` — same command (denominator: **13,119 of 26,396** magnitude-bearing units; independently cross-checked: `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); m=[u for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0]; print(sum(1 for u in m if u.get('status') == 'not-ingested'))"` → `13119`)
  - Promotion-ladder citation, re-verified at HEAD by content (not just path/line): `src/bin/v06_work_inventory.rs:9592-9595` — `python3 scripts/shape_engine_boundary.py --check` (exits 0 only if all four lines' live content still matches; verified directly: `sed -n '9592,9595p' src/bin/v06_work_inventory.rs` reproduces the exact four-condition block quoted in `technical-design.md §3` and `decisions.md §2a`, anchored at line `9595` as those documents cite)
  - `citation_ok=True` — same command; a RED->GREEN mutation proof (in-process, no file mutation) confirmed the check fails for the intended reason when a line's expected content is altered, and passes again once restored (see Notes)
- **Row-count command output:**
  ```
  $ python3 -c "
  content = open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/shape-engine-boundary.md').read()
  reqs = ['26396', '13119', '9595', 'has_real_description', 'is_display_wiring_class_for_promotion', 'universal_sheet_modifier', 'class_feature_pool_catalog_holds', 'python3 scripts/shape_engine_boundary.py --check']
  present = [r for r in reqs if r in content]
  missing = [r for r in reqs if r not in content]
  print(f'required_elements_present={len(present)} of {len(reqs)}; missing={missing}')
  "
  required_elements_present=8 of 8; missing=[]
  ```
  All eight required elements — both counts, the anchor line number, all four promotion-ladder
  condition fragments, and the re-derive command itself — are present in the committed artifact.
- **Build scope verified:** `cargo test --locked --no-run` exit 0, run at the tip of this cycle (no Rust source touched — Python + one generated markdown artifact only); `cargo test --locked --lib` not re-run (no Rust change to verify; inherited from AT-34-E1-003's last-verified state); `apps/desktop/src-tauri` not touched, not run.
- **Sweep population:** N/A — this cycle adds no corpus records and regenerates none; `docs/work-inventory.json` is read-only this cycle (read by both the new script and its tests).
- **Oracle pin:** N/A — no figure in this cycle came from the pinned PCGen corpus; every figure is derived from `docs/work-inventory.json` and `src/bin/v06_work_inventory.rs`.
- **Status:** complete
- **Movement, four buckets:** reclassification — no unit moved bucket; this cycle commits an existing, already-true fact (the shape-engine/promotion-ladder boundary) as a self-verifying artifact so no later cycle re-derives or re-asserts it from memory.
- **Notes:**
  - Both headline counts (`26,396` magnitude-bearing, `13,119` not held by the engine) matched `technical-design.md §3` / `decisions.md §2a`'s stated figures exactly on the first live run against the current `docs/work-inventory.json` — no drift since those documents were authored.
  - The promotion-ladder citation at `src/bin/v06_work_inventory.rs:9592-9595` was independently re-read with `sed -n` before writing the instrument, confirming the exact four-condition block technical-design.md quotes, anchored at line `9595` (the `facts.class_feature_pool_catalog_holds(...)` line) as both `technical-design.md §3` and `decisions.md §2a` cite it.
  - **RED->GREEN mutation proof**, run in-process against the live module (no file on disk mutated): temporarily overwrote `shape_engine_boundary.PROMOTION_LADDER_LINES[9595]` with a string guaranteed absent from the real line, confirmed `citation_failures()` returns exactly one failure naming line `9595` and the wrong-content substring, and confirmed `build_report()` raises `StaleCitationError` for that reason (RED). Restored the original mapping and confirmed `python3 scripts/shape_engine_boundary.py --check` exits 0 again with `citation_ok=True` (GREEN). Also exercised as unit tests `TestCitationFailsClosedForTheIntendedReason.*` in `scripts/tests/test_shape_engine_boundary.py` — 12/12 tests green, including two dedicated to this mutation proof and one asserting the live counts against the real corpus (not a fixture).
  - The instrument deliberately checks the ladder's four lines by **content**, not merely path/line existence, per `risks-and-open-questions.md §10` and the same posture `completion_atlas.py` condition 6 and `missing_engine_tables.py`'s `citation_failures()` already established — a future refactor that shifts this code without changing line counts is caught rather than silently trusted.
  - This cycle does not touch `docs/work-inventory.json`, `completion_atlas.py`, or any bucket definition — it is a read-only, self-verifying statement of an already-established fact, matching the shape of the criterion's evidence requirement exactly (a committed `.md` artifact, not a code change to the engine or the atlas).
- **Next-cycle plan:** AT-34-E1-005 — rename the `not-ingested` status field to state what it means (e.g. `engine-does-not-hold`) across `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`, and every consumer, including `completion_atlas.py`'s bucket-A/B/C/D arms and this cycle's own `shape_engine_boundary.not_held_by_engine()`, which currently keys on the literal string `"not-ingested"` and must be updated in the same rename cycle or it will silently report `not_held_by_engine=0`.
