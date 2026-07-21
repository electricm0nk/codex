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

- **Decision (SUPERSEDED 2026-07-20, see correction below):** All SD-23 kanban cards minted with explicit `--assignee` profile.
- **Operator directive:** Standing memory (`default-assignee footgun` Honcho duracon).
- **Original application:**
  - CODE lanes (Epic 5, Epic 6) → `--assignee tech-priest`.
  - OPS lanes (Epic 7 closure epilogue) → `--assignee god-emporer`.
  - Never `--assignee default`. Never `--assignee vanderspeigle`.
- **Correction (2026-07-20, cycle 5 incident):** `tech-priest` (and every other named hermes profile — confirmed via `ps aux`: ruby, servitor, default, god-emporer, gunny, shepherd, tech-priest all run a standing `gateway run` daemon) has a live, always-on daemon that auto-claims any `ready`-status card assigned to it and spawns an independent worker to execute it — twice, on the same card, racing the orchestrating session's own already-completed implementation. `operator` is the one assignee value with `ON DISK: no` in `hermes kanban assignees` — no daemon, matching SD-22's precedent (all 27+ of its cards used `--assignee operator`). **All SD-23 cards from cycle 5 onward use `--assignee operator`.** The "never default, never vanderspeigle" guard still applies; `operator` is not `default`.
- **Card lifecycle correction (2026-07-20):** Cards are receipts of already-completed work, not work orders. The corrected sequence per cycle is: implement + verify + commit FIRST, then `create` the card, then immediately `comment` (the receipt) and `complete` it — no `claim` step, no window left in `ready` status where any daemon (even under `operator`, defensively) could act on it.

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

## 14. Four-check audit "placeholder" false positive — `ItemPickerModal.tsx`

- **Finding (2026-07-21, cycle 9):** The audit's Check 1 (`grep -nE '\b(STUB|MOCK|placeholder|...)\b'`) flags `apps/desktop/src/characterHub/ItemPickerModal.tsx:127`'s `placeholder={props.searchPlaceholder}` — a standard HTML `<input>` attribute, not a claim of unfinished work. This is a genuine false positive: `\bplaceholder\b` matches the JSX attribute name regardless of context.
- **Discovered:** the branch-wide audit (base `f36c211...HEAD`) surfaced this hit during cycle 9's (Rust-only) verification. Confirmed via `git diff 885bbf9^..885bbf9` that the hit was already present in cycle 8's own commit (Epic 5 closure) — cycle 8's audit run that cycle reported `OK_NO_TOKENS`, which was a verification miss on my part, not a real absence of the string.
- **Disposition:** not a doctrine violation — `ItemPickerModal.tsx` has no stub, mock, or fake-success behavior (independently re-verified in cycle 8's review: real fetch, real filter, real select/confirm). Not registered in the Stubs Registry because it isn't a stub; this is a check-calibration note, not an exception grant.
- **Going forward:** every subsequent four-check audit run in this bundle will re-surface this same line. Treat a Check-1 hit that is *only* `apps/desktop/src/characterHub/ItemPickerModal.tsx:127: placeholder={props.searchPlaceholder}` (or its line-number-shifted equivalent as the file changes) as this known false positive — verify no *other* hits are present, don't block a cycle on this one alone. Any other Check-1 hit is real and blocking as normal.

## 15. Promotion PR merge: auto-merge on green CI (operator-confirmed 2026-07-21)

- **Decision:** Criterion 29's promotion PR (`tranche/5-1 → develop`) auto-merges once CI passes — no separate manual approval gate for the merge itself.
- **Operator directive:** 2026-07-21, confirmed via AskUserQuestion at the Epic 7 pre-PR checkpoint (choice: "Auto-merge on green CI" over "Open PR, then stop and wait for you").
- **Rationale offered:** matches `loop-instruction.md`'s own designed Epic 7 pipeline (sub-step 6, "CI passes; merge is clean") and the SD-22 precedent, which merged its own promotion PR autonomously under the same loop pattern with no operator objection.
- **Scope:** applies to this bundle's promotion PR only. Does not create a standing "always auto-merge" rule for other bundles or other repos without asking again.
