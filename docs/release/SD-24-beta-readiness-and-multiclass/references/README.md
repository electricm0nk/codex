# SD-24 — References Index

> **Per `./scope-draft.md §4 Files in this folder`.** Doctrine pointers, skill pointers, sibling-bundle pointers.

## 1. Repo-local doctrine (REPO-LOCAL CANONICAL)

- `../../governance/no-stub-mvp-doctrine.md` — wired-integration parent doctrine (skill: `wired-integration-discipline`)
- `../../governance/identifier-discipline.md` — identifier-discipline sibling doctrine (skill: `identifier-discipline`)
- `../../governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions registry (entry #0001: browser-preview fallback at `apps/desktop/src/characterHub/characterHubRuntime.ts:17-18`)
- `../../governance/spec-domain-lifecycle.md` — spec-domain lifecycle routing (governs which bundle owns which code path)

## 2. Repo-local architecture

- `../../architecture/` — repo-local architecture docs (topic-by-topic; the closure epilogue §6 obligation re-verifies every touched topic)

## 3. Skills

Per the loop's Step 0 (skill list):

- `wired-integration-discipline` — per the four-check audit recipe
- `identifier-discipline` — per the per-cycle review checklist
- `kanban-claude-code-execution-receipt` — per the cycle-receipt schema

## 4. Sibling bundles

- `../SD-22/` — predecessor bundle, content-source ingest (APG + ACG + Bestiary 1) + DM toolkit; data source for Epic 4 + Epic 5
- `../SD-23-character-mutation-and-wired-integration/` — active bundle on `tranche/5-1`; Tier-1 launch-gate dependency (SD-23 closure PR → develop)
- `../SD-21-campaign-manager-and-persistence/decisions.md §18` — operator's 2026-07-17 `<major>.<tranche-base>.<build>` build-version amendment

## 5. Cross-cutting operator directives (Honcho duracons)

- **2026-07-21 09:24:59** — SD-23 closure PR SD-23 → develop is SD-24 launch-gate dependency
- **2026-07-20 11:30:56** — operator's count of designed stubs is 2-3; anything beyond is accidental debt
- **2026-07-20 11:38:02** — storage-tier convergence: Option A (file-based fix) for this bundle; structural convergence deferred
- **2026-07-20 12:37:07** — operator expects the app to add a lot more data in the future (more rules systems, character classes, races, beasts)
- **2026-07-17 19:15:05** — operator's tranche-related build-version amendment (`<major>.<tranche-base>.<build>`); SD-24's first concrete value is `0.5.<build>`
- **2026-07-18 18:20:41** — operator identified that no command exists to mutate / re-save an existing character; `compose_character_input` loadout hardcoding needs repair
- **2026-07-04 12:41:37** — respawn-guard footgun: kanban dispatcher's `active_pr` rule emits `respawn_guarded` on every ready cycle once a CODE slice lands a PR; remedy is verifying work on disk + `--result` with named receipt comment id + leaving an audit comment
- **2026-07-18 20:17:51** — loop-instruction doctrine: full per-cycle mechanics internalized; cross-bundle refs are doctrinal reads only

## 6. External

- `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — PCGen LST corpus (per Epic 6's equipment coverage audit)
