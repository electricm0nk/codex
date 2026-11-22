---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — front half
---

# SD-43 Technical Requirements

> `decisions.md` wins where this file differs. Every prerequisite has a check; every requirement names the card
> or gate that enforces it.

## 1. Pre-launch prerequisites

| ID | Prerequisite | Check | Closes at |
|---|---|---|---|
| P-1 | `tranche-43` contains `origin/develop` @ `fb94a85049` | `git merge-base --is-ancestor fb94a85049 HEAD && echo ok` | C1 |
| P-2 | Working tree clean before every card's first write; no file the card did not touch is listed | `git status --porcelain \| awk 'END{print NR}'` → 0 (a `docs/retro/events/root.jsonl` line written by tooling is left alone and named in the receipt) | every card |
| P-3 | `0.43.0` on the version surfaces | `awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock` → `version = "0.43.0"`; `node -p "require('./apps/desktop/package.json').version"` → `0.43.0` | done (`147a3f71eb`) |
| P-4 | The release pointer resolves | `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0 (exit 2 on 2026-11-22) | C1 |
| P-5 | Doctrine present | `test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/governance/blocker-closure-doctrine.md && echo ok` | C1 |
| P-6 | Disk ≥ 24 G free before a full sweep; cargo cards in flight < 3 | `df -h /` (1.4T free on 2026-11-22); `pgrep -x cargo \| awk 'END{print NR}'` | every cargo card |
| P-7 | The PCGen oracle at the pin (E0.1 reads its UI strings and `ABILITYCATEGORY` lines) | `source <(bash scripts/fetch-pcgen-oracle.sh --print-env 2>/dev/null \|\| true); git -C "$PCGEN_REPO_DIR" rev-parse HEAD` = `PCGEN_ORACLE_SHA` in `scripts/pcgen-oracle-pin.env` (`7f818006e3…`); `$PCGEN_REPO_DIR` is unset in a plain shell on 2026-11-22 | E0.1 |
| P-8 | The report API key exists as a CI secret (operator creates it; never in the repo) | `gh secret list -R electricm0nk/codex \| grep -c CODEX_REPORT_KEY` → 1 | E5.1 release proof (the card lands without it; Send is disabled with a reason) |
| P-9 | The isolated app-data root and screenshot path work (`run` skill / `run-desktop`) | one screenshot under `artifacts/epic_0/gear-repro/` | E0.4 |
| P-10 | `pdftotext` present for print checks | `which pdftotext` | E2.MC, E3.11 |
| P-11 | The class feature pool census script runs | `python3 scripts/census_class_feature_pool_population.py \| awk '/distinct/'` → `1913` | E0.1 |

## 2. Normative requirements

| ID | Requirement | Enforced by |
|---|---|---|
| NR-1 | **The engine owns every rules number.** No TS sums a weight, a price, a grant or a prerequisite. Ability budgets are `abilities.budget.<subtab>.<pool>.{granted,spent,remaining}` rows; feat budgets stay `feats.budget.<pool>.*`; sell values come from `sellNegotiateGp`, which mirrors `refund_copper`; stats come from an engine command | E3.3's "no number not from rows" test; E1.8's two-uses grep; MC reviews |
| NR-2 | **Refuse with a reason.** An ability pick not granted names the pool; one not qualified for names each unmet prerequisite; a refused drop or double-click names why | E3.2, E3.5, E1.4 tests |
| NR-3 | **Old saves load unchanged.** Every new saved field is `#[serde(default)]`; `custom.json` without the new flags loads with them off | `apps/desktop/src-tauri/tests/old_save_loads_unchanged.rs` (E3.2 adds a case) |
| NR-4 | **One write path per action.** Double-click reuses Move-to; the sell window reuses the Sell write; House Rules edit the existing `Overrides` | E1.4, E1.6, E6.2 reviews |
| NR-5 | **Print the rule.** Abilities, rest text and pool members print as text; only sheet totals are computed (`decisions.md §27`) | E3.11, E2.4 reviews |
| NR-6 | **No stub ships.** Four-check audit on every code-bearing card; no on-screen engine-gap notice at closure; data-gap notices equal E0.1's list | the audit in each receipt; E3.MC |
| NR-7 | **Packaged data.** Any new runtime data read (rest table, name table) goes through `repo_root()`, has a `bundle.resources` entry in `apps/desktop/src-tauri/tauri.conf.json` and an assertion in `apps/desktop/src-tauri/tests/packaged_resources.rs` | `cd apps/desktop/src-tauri && cargo test --test packaged_resources`; root `cargo test --test no_baked_manifest_dir_in_production` |
| NR-8 | **Secrets never in the repo.** The report key is never written to the repo, logs, receipts or the web view's JS | `git grep -n "x-codex-key" -- apps/desktop/src` → 0; `git log -p tranche-43 \| grep -c "$(cat ~/workspace/.report_key 2>/dev/null \|\| echo NO_KEY_FILE)"` → 0 (run locally only) |
| NR-9 | **Every row reachable.** No Market row limit; filter value counts + `Other` cover every row per list and column | E1.10, E1.11 |
| NR-10 | **Accessibility.** Double-click has an Enter equivalent; the sell window traps and returns focus; Escape cancels | E1.4, E1.6 tests |
| NR-11 | **Layout fits the real viewport.** `100dvh`, no fixed pixel height, no page scroll on the character screen | E2.1 test + screenshots |
| NR-12 | **One writer per shared file.** `character_hub.rs`, `main.rs`, `CharacterSheet.tsx` under the `hub` token; `ui-smoke/spec.json` only by E7.1 | `epic-breakdown.md §0` |
| NR-13 | **Proof width stated.** Every acceptance names what it does not cover (`AGENTS.md` rule 7); sampling tests (Market stats, filter counts, pool prerequisites) print per-shape counts | each card's receipt |
| NR-14 | **Widest build scope.** Each MC runs root `cargo test` and `apps/desktop/src-tauri` `cargo test`, plus `npm test && npm run typecheck`; every `test result: FAILED` attributed to its `Running` line | E*.MC, E7.2 |

## 3. Out of scope

See `scope-draft.md §4` (non-goals) and `§5` (exclusions). Nothing in the Definition of Done is out of scope.
