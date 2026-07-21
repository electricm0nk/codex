# SD-23 Decisions — Character Mutation and Wired Integration

Append-only decision log. Each entry: number, decision, operator directive that drove it, and the recorded-by date.

---

## 1. SD-23 slug is `character-mutation-and-wired-integration`

- **Decision:** Slug drives the directory name, branch description, and Epic 1-7 titles.
- **Operator directive:** 2026-07-20 (operator-confirmed in this session).
- **Rationale:** Captures both load-bearing themes — the typed mutation surface (concrete code) and the Wired Integration doctrine (governance). Naming only the code side understates the bundle.

## 2. Branch is `tranche/5-1`; board is reused `codex-tranche-5`

- **Decision:** Distinct branch, reused board.
- **Operator directive:** 2026-07-20 (operator-confirmed override of the convention slug).
- **Rationale:** Convention `tranche/X-Y → codex-tranche-X-Y` only applies to fresh boards. The reused board keeps its prior slug. SD-22 itself followed this pattern (`codex-tranche-5` was the dead-state board from a prior 2026-07-16 SD-21 launch attempt).

## 3. Build counter inheritance at SD-23 launch

- **Decision:** Tranche-base 5 (same as SD-22); build counter inherits from develop at SD-22 closure PR merge.
- **Operator directive:** 2026-07-17 build-version amendment (per Honcho duracon "Operator directive 2026-07-17 (build version amendment)").
- **Capture:** At pre-launch checklist step 7, fill `progress.md` §"Build counter inheritance" with the develop HEAD's `Cargo.toml` workspace version. The first concrete value is `0.5.<captured_build>`.
- **Closure increment:** On `tranche/5-1 → develop` promotion (Epic 7), tranche-base advances to 6 and build resets to 0. Resulting version after SD-23 closure: `0.6.0`.
- **Captured 2026-07-20:** `0.5.96`, from `apps/desktop/src-tauri/Cargo.toml:3` at `origin/develop` HEAD `f36c211`. Root `Cargo.toml` has no `[workspace]` section (standalone `0.1.0` package) — "develop's Cargo.toml workspace version" resolves to the desktop app's Cargo.toml, which is where SD-22 also stamped `0.5.96` at closure. Applies to Criterion 6 (progress.md capture, done) and is the base Criterion 30 increments from at closure.

## 4. Google OAuth dropped from scope; local folder only

- **Decision:** Campaign Manager does not authenticate to Google. Drive API not called. "Drive folder" is a local folder the user configures via OS folder picker.
- **Operator directive:** 2026-07-20 ("we are going to use a shared drive and leave it at that for our initial release - the campaign just needs to create and manage files on a local path for now. The user will handle the Google setup.").
- **Rationale:** Simplifies the storage contract. The user's Google Drive desktop sync client handles cross-device sync, not the app. Removes three planned stubs (OAuth flow, Drive API call, member invite flow).

## 5. Member invites deleted entirely

- **Decision:** `CampaignMember.invited` field removed from the `Campaign` type. `createCampaign` no longer hardcodes `invited: true`. The data model becomes `{email}` only.
- **Operator directive:** 2026-07-20 (implied by the "shared drive only" simplification).
- **Rationale:** The Wired Integration doctrine forbids "Would invite: a@b.com" return strings. Shipping a fake-invite field would violate the doctrine from day one. If member invites are added later, they will be a different feature with a different shape.

## 6. Storage tier fix is Option A (minimal file-store)

- **Decision:** Add `delete_character` and `import_character` Tauri commands on the existing file-based `SavedCharacterStore`. No database. No migration.
- **Operator directive:** 2026-07-20 ("option a, confirmed").
- **Rationale:** The doctrine does not require fixing latent architectural debt that hasn't surfaced as a bug yet — it requires that shipped code is fully wired. Option A wired fully is doctrine-compliant. The latent referential-integrity risk is captured at `programs/codex/research/storage-tiers-convergence-2026-07-20.md` for a future bundle.

## 7. Wired Integration doctrine active 2026-07-20

- **Decision:** New base-level governance doctrine. Codifies "no stubs in shipping code; stubs are the exception requiring explicit operator approval." Pairs with identifier-discipline doctrine.
- **Operator directive:** 2026-07-20 ("No more stub work. No more mock data. I expect everything from this point forward to be fully wired.").
- **Doctrine-of-record:** `../../governance/no-stub-mvp-doctrine.md` (active, accepted, dated 2026-07-20).
- **Companion skill:** `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` (v1.0.0).
- **Stubs Registry:** `../../governance/wired-integration-stubs-registry.md`.

## 8. Stubs Registry entry #0001: browser-preview fallback

- **Decision:** The `characterHubRuntime.ts:17-18` browser-preview fallback (`return buildPreviewListSurface()` when `!hasTauriRuntime()`) is an operator-granted permanent exception.
- **Operator directive:** 2026-07-20 (per the doctrine's operator-grant mechanism; rationale: browser preview needs a sample character so the Load → sheet flow stays walkable without the desktop backend).
- **Registry entry:** `../../governance/wired-integration-stubs-registry.md` §0001.

## 9. Epic structure: 7 epics / 33 acceptance criteria / 16 closure gates

- **Decision:** Epic layout per `epic-breakdown.md`.
- **Operator directive:** 2026-07-20 (implicit; operator-confirmed the high-level shape across the session).
- **Rationale:** Identifier Cleanup first (doctrine); Operator Pre-Launch second (gating); Wired Integration Cleanup third (doctrine + stub remediation); Campaign Manager Simplification fourth (carries the OAuth collapse); Character Mutation Surface fifth (the load-bearing new functionality); Storage Tier Minimal Fix sixth (closes the Load Character stubs); Closure Epilogue seventh (tranche promotion + build counter advance).

## 10. Default-assignee rule for SD-23 cards

- **Decision:** All SD-23 kanban cards minted with explicit `--assignee` profile.
- **Operator directive:** Standing memory (`default-assignee footgun` Honcho duracon).
- **Application:**
  - CODE lanes (Epic 5, Epic 6) → `--assignee tech-priest`.
  - OPS lanes (Epic 7 closure epilogue) → `--assignee god-emporer`.
  - Never `--assignee default`. Never `--assignee vanderspeigle`.

## 11. Workspace-routing for SD-23 artifacts

- **Decision:** All SD-23 files live under `programs/codex/requirements/SD-23-character-mutation-and-wired-integration/` per Honcho duracon 2026-06-17 (research/governance routing rule) and operator directive 2026-07-20 ("keep everything inside the requirements directory going forward, no more creation at the root of the workspace").
- **Exceptions:**
  - `../../governance/no-stub-mvp-doctrine.md` (governance, lives under `governance/`).
  - `../../governance/wired-integration-stubs-registry.md` (governance, lives under `governance/`).
  - `../../research/storage-tiers-convergence-2026-07-20.md` (program research, lives under `programs/codex/research/`).

## 12. Operator cross-cutting rules inherited from prior bundles

- **TDD mandatory** per repo `AGENTS.md` §"Non-Negotiable Rules" — write failing test before production code, confirm test fails for the intended reason, implement smallest change, run relevant tests, refactor only after green.
- **Respawn-guard footgun** per Honcho duracon — CODE slices that land a PR and write the PR-URL into the card's comment stream trigger `respawn_guarded` on subsequent ready cycles; the remedy is to verify the work on disk via the receipt comment, then complete with the receipt id and merge tip.
- **Patch-tool multi-line corruption footgun** per Honcho duracon — multi-line `old_string`/`new_string` with literal `\n` can lose paragraph breaks; verify with `grep -nE "^#"` and `grep -nE "^$"` after every multi-line patch.
- **Honcho memory doctrine** per Honcho duracon — pointers in Hermes memory, content in Honcho or skills-on-disk; do not duplicate durable content into memory.
