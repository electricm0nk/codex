# Branch Protection Rulesets (Codex program)

Codex branches whose protection belongs to **the operator's GitHub admin authority**, not the repo's CI. The JSON files in this directory are **reviewable source-of-truth** artifacts — they document the intended protection state and can be applied via the GitHub Rulesets API.

This file is owned by god-emporer (doctrine + admin scaffold). Updates require the operator's review.

---

## Files

| File | Branch | Notes |
|---|---|---|
| `tranche-3.json` | `tranche/3` | Source of truth for Tranche 3 protection. No-PR shape per operator decision 2026-07-06. |

To add a new tranche:

1. Cut the tranche branch per `devops/tranche-branch-governance` (§"Tranche start").
2. Copy `tranche-3.json` to a new `<tranche-name>.json`, update `_meta.branch`, `_meta.baseline_sha`, `_meta.mirrors_doctrine`, the `enforcement` block, and any tranche-specific status checks.
3. Commit on the new tranche branch (so the ruleset artifact lives next to the branch it protects).
4. Apply via one of the methods below.

---

## Apply path (GitHub UI)

1. `https://github.com/electricm0nk/codex/settings/rulesets` → **New branch ruleset** → **Import** the JSON.
2. Or, **New branch ruleset** → fill manually with the same fields.

Each file in this directory is a single ruleset payload. There is no automatic application — the artifact is documentation for what must be applied via the admin console (or via the API below).

## Apply path (GitHub Rulesets API)

```bash
# Validate locally first:
python3 -c "import json; json.load(open('.github/branch-protection-rulesets/tranche-3.json'))"

# Apply (requires a token with `repo:admin` scope on electricm0nk/codex):
curl -fsSL -X POST \
  -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/electricm0nk/codex/rulesets \
  -d @.github/branch-protection-rulesets/tranche-3.json
```

On success, GitHub returns the created ruleset id; record it back into the file's `_meta.applied_ruleset_id` field for future audits.

## Update path

When the operator wants to change protection rules:

1. Edit the JSON file in this directory.
2. Open a PR against the tranche branch — even when working in the direct-push shape, an admin-only review on protection changes is the audit trail.
3. Apply the updated ruleset via the same path as above (PUT on existing id, or POST if recreating).

---

## Why this lives in-repo and not just in the GitHub console

Three reasons:

1. **Audit visibility** — every change to branch protection has a commit and a review on the same branch that the protection governs. The console changes have no native audit log.
2. **Doctrine discoverability** — future agents on this repo can read the protection policy without admin console access. The skill `devops/tranche-branch-governance` points operators here for the "what should tranche/3 protect" question.
3. **Multi-tranche consistency** — the next tranche (tranche/4) starts by copying `tranche-3.json`, not by re-deriving every field. The shape stays consistent.

The trade-off: this directory is documentation until applied. If the ruleset file diverges from the live GitHub ruleset, the file is wrong (or the live ruleset is). Audits should check both surfaces.

---

## Files owned elsewhere

- `allow-only-develop-into-test.yml` workflow — branch-source CI guard, lives in `.github/workflows/` (not here).
- `allow-only-test-into-main.yml` — same.
- `promotion-gates.yml` — release-evidence gate at the develop→test→main boundary, owns the rules for those branches from inside the workflow itself.

The directory here is for human-administered rulesets only.
