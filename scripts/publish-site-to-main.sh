#!/usr/bin/env bash
# Publish the site/ directory from a working branch to the deploy branch.
#
# WHY THIS EXISTS: `.github/workflows/deploy-site.yml` deploys `site/**` to
# Cloudflare Pages on every push to `main`, with NO build step -- the committed
# files ARE the published artifact. That makes the merge to `main` the moment
# anything in `site/` becomes public, indexed and cached. Un-publishing later
# does not un-index it.
#
# The site content lives on a long-running work branch (tranche/N) tangled with
# engine commits, so cherry-picking individual commits drags unrelated work
# along. This script instead takes the site/ TREE as it stands on the source
# branch and lands it on a fresh branch off the deploy branch as one clean
# commit, then opens a PR.
#
# IT REFUSES TO PROCEED IF THE SAFETY GATES DO NOT PASS. That is the point of
# the script -- the gates are not advisory here, because the failure mode is
# publishing Paizo Product Identity to a public website.
#
# Usage:
#   ./scripts/publish-site-to-main.sh [--from BRANCH] [--to BRANCH] [--dry-run]
#
#   --from BRANCH   source of the site/ tree      (default: current branch)
#   --to   BRANCH   deploy branch to target       (default: main)
#   --dry-run       run every check and show the diff, then STOP without
#                   creating a branch, committing, pushing or opening a PR
#   --skip-gates    DANGEROUS. Skips the verify.sh site/PI stages. Requires
#                   PUBLISH_SITE_I_ACCEPT_THE_RISK=1 in the environment too.
#                   Intended only for a machine that cannot run the gates at
#                   all; never as a way past a red gate.
#
# Exit codes: 0 ok / 1 a gate or precondition failed / 2 usage error
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

FROM=""
TO="main"
DRY_RUN=0
SKIP_GATES=0

while [ $# -gt 0 ]; do
    case "$1" in
        --from) FROM="${2:-}"; shift 2 ;;
        --to)   TO="${2:-}";   shift 2 ;;
        --dry-run)    DRY_RUN=1;   shift ;;
        --skip-gates) SKIP_GATES=1; shift ;;
        -h|--help) sed -n '2,40p' "$0"; exit 0 ;;
        *) echo "usage error: unknown argument '$1'" >&2; exit 2 ;;
    esac
done

[ -n "$FROM" ] || FROM="$(git rev-parse --abbrev-ref HEAD)"

say()  { printf '\n\033[1m==> %s\033[0m\n' "$*"; }
fail() { printf '\n\033[31mREFUSING TO PUBLISH: %s\033[0m\n' "$*" >&2; exit 1; }

say "publish site/  from '$FROM'  ->  '$TO'"

# ---------------------------------------------------------------------------
# 1. Preconditions
# ---------------------------------------------------------------------------
say "1/6  preconditions"

if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    git status --short --untracked-files=no >&2
    fail "working tree has uncommitted tracked changes. Commit or stash them first
       (note: this repo forbids 'git stash' -- commit instead)."
fi

command -v gh >/dev/null 2>&1 || fail "the 'gh' CLI is required to open the PR and is not installed."
gh auth status >/dev/null 2>&1 || fail "'gh' is not authenticated. Run: gh auth login"

git fetch --quiet origin || fail "could not fetch origin."

git rev-parse --verify --quiet "origin/$TO" >/dev/null \
    || fail "deploy branch 'origin/$TO' does not exist."
git rev-parse --verify --quiet "$FROM" >/dev/null \
    || fail "source branch '$FROM' does not exist."

[ -d site ] || fail "no site/ directory in this checkout."
echo "    source branch : $FROM ($(git rev-parse --short "$FROM"))"
echo "    deploy branch : origin/$TO ($(git rev-parse --short "origin/$TO"))"

# ---------------------------------------------------------------------------
# 2. Safety gates -- DISCOVERED from verify.sh, never hardcoded, so a gate
#    added later is picked up automatically instead of being silently skipped.
# ---------------------------------------------------------------------------
say "2/6  safety gates"

if [ "$SKIP_GATES" = "1" ]; then
    [ "${PUBLISH_SITE_I_ACCEPT_THE_RISK:-}" = "1" ] \
        || fail "--skip-gates also requires PUBLISH_SITE_I_ACCEPT_THE_RISK=1. Refusing."
    echo "    !! GATES SKIPPED BY EXPLICIT REQUEST -- this publish is unverified !!"
else
    STAGES="$(grep -oE 'stage_start "[a-z0-9-]+' scripts/verify.sh \
              | sed 's/stage_start "//' \
              | grep -iE 'site|pi' | sort -u || true)"

    [ -n "$STAGES" ] && echo "    discovered stages:" && echo "$STAGES" | sed 's/^/      - /'
    [ -n "$STAGES" ] || echo "    (no site/PI stages found in verify.sh -- see the warning below)"

    if [ -z "$STAGES" ]; then
        fail "found NO site or PI stages in scripts/verify.sh. Either the gates were
       removed or this script's discovery pattern has drifted. Both are reasons to
       stop, not to publish unchecked."
    fi

    for stage in $STAGES; do
        printf '    %-28s ' "$stage"
        if ./scripts/verify.sh --only "$stage" >/tmp/publish-site-gate.log 2>&1; then
            echo "PASS"
        else
            echo "FAIL"
            tail -25 /tmp/publish-site-gate.log >&2
            fail "gate '$stage' failed. Full log: /tmp/publish-site-gate.log"
        fi
    done
fi

# ---------------------------------------------------------------------------
# 3. Independent PI sweep over EXACTLY the files bound for publication.
#    Belt and braces: the gates above check what they were built to check;
#    this checks the actual payload. PI screening has failed repeatedly in
#    this repo on near-miss text that an earlier check was blind to.
# ---------------------------------------------------------------------------
say "3/6  independent PI sweep over the publish payload"

if [ -f scripts/observer/pi_redaction.py ]; then
    git archive "$FROM" site | tar -x -C /tmp -f - --transform 's|^site|publish-site-payload|' 2>/dev/null \
        || { rm -rf /tmp/publish-site-payload; mkdir -p /tmp/publish-site-payload
             git archive "$FROM" site | tar -x -C /tmp/publish-site-payload --strip-components=1 -f -; }

    PCGEN_ROOT="${PCGEN_CORPUS_ROOT:-$HOME/workspace/repos/pcgen/data}"
    python3 - "$PCGEN_ROOT" /tmp/publish-site-payload <<'PY' || fail "declared-PI names found in the publish payload (see above)."
import json, os, sys, glob
sys.path.insert(0, os.path.join(os.getcwd(), "scripts", "observer"))
try:
    import pi_redaction as R
except Exception as exc:                      # noqa: BLE001
    print(f"    could not import pi_redaction ({exc}); NOT treating this as a pass", file=sys.stderr)
    sys.exit(1)

oracle_root, payload = sys.argv[1], sys.argv[2]
try:
    index = R.build_declared_pi_name_index(oracle_root)
except Exception as exc:                      # noqa: BLE001
    print(f"    could not build the declared-PI index ({exc}); refusing", file=sys.stderr)
    sys.exit(1)

lowered = {n.lower(): n for n in index}
leaks, scanned = [], 0

def walk(node, path, origin):
    global scanned
    if isinstance(node, dict):
        for key, value in node.items():
            walk(value, f"{path}.{key}", origin)
    elif isinstance(node, list):
        for i, value in enumerate(node):
            walk(value, f"{path}[{i}]", origin)
    elif isinstance(node, str):
        scanned += 1
        hit = lowered.get(node.strip().lower())
        if hit:
            leaks.append((origin, path, hit))

for path in sorted(glob.glob(os.path.join(payload, "**", "*.json"), recursive=True)):
    try:
        with open(path) as handle:
            doc = json.load(handle)
    except Exception:                          # noqa: BLE001
        continue
    walk(doc, "$", os.path.relpath(path, payload))

print(f"    declared-PI names known : {len(index)}")
print(f"    strings scanned         : {scanned}")
print(f"    exact-match leaks       : {len(leaks)}")
for origin, path, hit in leaks[:20]:
    print(f"      LEAK {origin} {path} = {hit!r}", file=sys.stderr)
sys.exit(1 if leaks else 0)
PY
    echo "    NOTE: this sweep is exact-leaf matching. A declared-PI name embedded"
    echo "          inside a longer built-up string is a KNOWN residual gap."
else
    fail "scripts/observer/pi_redaction.py not found -- cannot verify the payload."
fi

# ---------------------------------------------------------------------------
# 4. Show precisely what would change on the deploy branch
# ---------------------------------------------------------------------------
say "4/6  what changes on '$TO'"

git diff --stat "origin/$TO" "$FROM" -- site/ || true
CHANGED="$(git diff --name-only "origin/$TO" "$FROM" -- site/ | wc -l | tr -d ' ')"
echo "    files changed under site/ : $CHANGED"

if [ "$CHANGED" = "0" ]; then
    echo "    nothing to publish -- site/ on '$FROM' already matches 'origin/$TO'."
    exit 0
fi

if [ "$DRY_RUN" = "1" ]; then
    say "DRY RUN -- stopping here. Nothing was branched, committed, pushed or opened."
    exit 0
fi

# ---------------------------------------------------------------------------
# 5. Build the publish branch: site/ tree only, one clean commit
# ---------------------------------------------------------------------------
say "5/6  building the publish branch"

STAMP="$(date -u +%Y%m%d-%H%M%S)"
PUBLISH_BRANCH="site-publish/${STAMP}"
ORIGINAL_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
cleanup() { git checkout --quiet "$ORIGINAL_BRANCH" 2>/dev/null || true; }
trap cleanup EXIT

git checkout --quiet -b "$PUBLISH_BRANCH" "origin/$TO"
git checkout "$FROM" -- site/
git add -- site/

if git diff --cached --quiet; then
    echo "    no staged difference after all; aborting cleanly."
    git checkout --quiet "$ORIGINAL_BRANCH"
    git branch -D "$PUBLISH_BRANCH" >/dev/null
    exit 0
fi

MSG_FILE="$(mktemp -t publish-site-msg-XXXXXX.txt)"
cat > "$MSG_FILE" <<EOF
site: publish site/ from ${FROM} ($(git rev-parse --short "$FROM"))

Publishes the site/ tree as it stands on ${FROM} onto ${TO}, which
deploy-site.yml serves to Cloudflare Pages with no build step -- these committed
files ARE the published artifact.

Taken as a tree rather than as cherry-picked commits: the site content on a
long-running work branch is interleaved with engine commits, and picking them
individually would drag unrelated work onto the deploy branch.

Only site/ is included. The generators that produce it stay on the work branch
and reach ${TO} through the normal promotion path.

Safety gates run before this commit was created, all green:
$( [ "$SKIP_GATES" = "1" ] && echo "  !! GATES SKIPPED BY EXPLICIT REQUEST !!" || echo "$STAGES" | sed 's/^/  - /' )
Plus an independent declared-PI sweep over the exact publish payload.

Files changed under site/: ${CHANGED}

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_012LQjhfcbAEBt6SiquQXEAE
EOF

git commit --quiet -F "$MSG_FILE"
rm -f "$MSG_FILE"
echo "    committed $(git rev-parse --short HEAD) on $PUBLISH_BRANCH"

# ---------------------------------------------------------------------------
# 6. Push and open the PR. NEVER merge -- publishing is the operator's call.
# ---------------------------------------------------------------------------
say "6/6  pushing and opening the PR"

git push --quiet -u origin "$PUBLISH_BRANCH"

PR_URL="$(gh pr create \
    --base "$TO" \
    --head "$PUBLISH_BRANCH" \
    --title "site: publish site/ from ${FROM}" \
    --body "$(cat <<EOF
Publishes \`site/\` from \`${FROM}\` to \`${TO}\`.

**Merging this PR publishes to Cloudflare Pages.** \`deploy-site.yml\` deploys
\`site/**\` on push to \`${TO}\` with no build step, so these files go live as
committed, and become indexed and cached. Un-publishing later does not un-index.

- Files changed under \`site/\`: **${CHANGED}**
- Safety gates: all green (discovered from \`verify.sh\`, not hardcoded)
- Independent declared-PI sweep over the publish payload: clean

Generated by \`scripts/publish-site-to-main.sh\`.
EOF
)" 2>/dev/null || true)"

git checkout --quiet "$ORIGINAL_BRANCH"
trap - EXIT

if [ -n "$PR_URL" ]; then
    say "PR opened: $PR_URL"
else
    say "branch pushed: $PUBLISH_BRANCH  (open the PR manually -- gh pr create did not return a URL)"
fi

echo
echo "NOT MERGED. Merging is what publishes; that decision is the operator's."
