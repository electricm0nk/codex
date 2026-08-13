#!/usr/bin/env bash
# On-screen verification harness for SD Definition-of-done item 8.
#
# reach_gate passing proves a code path exists; it does not prove a player
# sees the value — three separate compute twins have each passed that gate
# while rendering nothing (SD-29 decisions.md §29 / Decision 43 lineage).
# This script drives the REAL app, navigates to a record family's catalog
# screen, filters to one record, screenshots it, and then proves the
# expected strings are actually rendered by select-all/copy on the webview
# and reading the X clipboard back. A screenshot alone is evidence a human
# must re-check; the extraction makes the check machine-verdicted.
#
# Usage:
#   RUN_DESKTOP_AGENT=<unique> verify-on-screen.sh \
#     --family race_trait --record "Ironskinned" \
#     --expect "Duergar" --expect "+2" \
#     --out docs/release/<bundle>/artifacts/<cycle>/item8 [--slug duergar-ironskinned]
#
# Families: equipment | spell | race_trait | monster | companion  (hub bottom links).
#
# Outcomes (the ONLY two):
#   PASS — exit 0; writes <out>/<slug>.png and <out>/<slug>.verify.md
#   FAIL — exit nonzero; any artifacts are named <slug>.FAILED.* so they can
#          never be cited as passing evidence. Launch failure, navigation
#          landing on the wrong screen, empty clipboard, record not found,
#          or any missing --expect string all take this path. There is no
#          silent-success mode: a run with zero --expect strings is itself
#          an error.
#
# Concurrency: RUN_DESKTOP_AGENT MUST be set to a per-cycle unique value.
# This script refuses to run as `default` — the unset default is exactly the
# collision that bit tranche/7 twice (SKILL.md "Concurrent agents").
#
# App reuse: if this agent already has a live app (state file + process),
# it is reused; otherwise the script launches one. `--fresh` forces a
# relaunch; the app is LEFT RUNNING afterwards so a cycle can verify many
# records cheaply — run `driver.sh stop` once at cycle end.
#
# Memory: never run this concurrently with scripts/verify.sh on this box
# (22 GiB RAM, no swap — vite gets OOM-killed; see SKILL.md Troubleshooting).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DRIVER="$SCRIPT_DIR/driver.sh"

# ---------------------------------------------------------------- arguments
FAMILY="" RECORD="" OUT_DIR="" SLUG="" FRESH="" TAB="" NO_SEARCH="" SCROLL=0
declare -a EXPECTS=() NAV_CLICKS=()
usage() {
  sed -n '2,40p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//' >&2
  exit 2
}
while [ $# -gt 0 ]; do
  case "$1" in
    --family)  FAMILY="${2:?}"; shift 2 ;;
    --record)  RECORD="${2:?}"; shift 2 ;;
    --expect)  EXPECTS+=("${2:?}"); shift 2 ;;
    --out)     OUT_DIR="${2:?}"; shift 2 ;;
    --slug)    SLUG="${2:?}"; shift 2 ;;
    --fresh)   FRESH=1; shift ;;
    --tab)     TAB="${2:?}"; shift 2 ;;          # race_trait only: 'alternate'
    --nav-click) NAV_CLICKS+=("${2:?}"); shift 2 ;; # "x,y" extra clicks after tab (e.g. a race chip)
    --no-search) NO_SEARCH=1; shift ;;           # screen has no search box (alternate tab)
    --scroll)  SCROLL="${2:?}"; shift 2 ;;       # scroll ticks before screenshot
    -h|--help) usage ;;
    *) echo "verify-on-screen.sh: unknown argument '$1'" >&2; usage ;;
  esac
done

[ -n "$FAMILY" ] || { echo "FAIL: --family is required" >&2; exit 2; }
[ -n "$RECORD" ] || { echo "FAIL: --record is required" >&2; exit 2; }
[ -n "$OUT_DIR" ] || { echo "FAIL: --out is required" >&2; exit 2; }
[ "${#EXPECTS[@]}" -gt 0 ] || {
  echo "FAIL: at least one --expect string is required — a check that expects nothing verifies nothing" >&2
  exit 2
}
if [ -z "${RUN_DESKTOP_AGENT:-}" ] || [ "${RUN_DESKTOP_AGENT:-}" = "default" ]; then
  echo "FAIL: RUN_DESKTOP_AGENT must be exported to a per-cycle unique value (not unset, not 'default')." >&2
  echo "      Two agents sharing 'default' share a display and kill each other's apps." >&2
  exit 2
fi

# ------------------------------------------------- family navigation table
# Coordinates are window-relative on the driver's 1920x1200 Xvfb screen and
# were calibrated against the live app (see git log of this file). The
# SCREEN_MARKER guard below turns any drift into a loud failure, never a
# screenshot of the wrong surface.
#   HUB_X/HUB_Y    — the "Browse …" link on the character-hub landing screen
#   SEARCH_Y       — the catalog's full-width search input (x is arbitrary)
#   SCREEN_MARKER  — text that MUST be in the extraction to prove we are on
#                    the intended catalog screen at all
# SEARCH_Y differs per screen because the facet-chip rows above the search
# box differ in count — verified live per family; MATCH_WORD is the noun in
# the screen's own "N matching <noun>." line, used to prove the filter
# actually applied (see the filtered-count gate below).
case "$FAMILY" in
  equipment)  HUB_X=578;  HUB_Y=930; SEARCH_Y=285; SCREEN_MARKER="Equipment Catalog"; MATCH_WORD="matching" ;;
  # SEARCH_Y CALIBRATED LIVE 2026-08-13 (SD-32 `ground-spell-units`), 285 -> 311.
  # 285 was set by analogy with `race_trait` and never driven — the deferral
  # recorded at `docs/retro/events/item8-harness.jsonl` ("equipment and spell
  # SEARCH_Y coordinates ... are set to 285 by analogy, not live-verified;
  # revisit: first lane cycle that verifies an equipment or spell record on
  # screen"). This is that cycle, and 285 was WRONG: the run was refused with
  # "search for 'Shield' still shows 1286 rows — filter did not apply".
  #
  # The reason is the same one the `companion` note below records: this screen
  # has TWO chip rows, not one. A book row (All/CRB/APG/ACG/ARG/UI) AND a school
  # row that itself wraps to two lines (10 schools) sit above the box, putting
  # it at y=311 — the same value `monster` uses, for the same structural reason.
  # Verified against a live screenshot of the running app, not by analogy.
  spell)      HUB_X=781;  HUB_Y=930; SEARCH_Y=311; SCREEN_MARKER="Spell Catalog";     MATCH_WORD="matching" ;;
  race_trait) HUB_X=1166; HUB_Y=930; SEARCH_Y=285; SCREEN_MARKER="Race Traits";       MATCH_WORD="matching" ;;
  monster)    HUB_X=1350; HUB_Y=930; SEARCH_Y=311; SCREEN_MARKER="Monster Catalog";   MATCH_WORD="matching" ;;
  # SD-29 Epic 7 (companion lane). The hub's bottom link row WRAPPED when this
  # link was added -- "Browse Companion Catalog" sits on a second line at y=971,
  # and every coordinate above is unchanged, verified on a live screenshot
  # rather than assumed.
  #
  # SEARCH_Y for this family is CALIBRATED LIVE every time the lane registers
  # books, not copied by analogy, because THIS SCREEN'S CHIP ROW GROWS WITH THE
  # CORPUS and the box moves with it. Two live calibrations so far:
  #
  #   247 — SD-29 Epic 7 round 1, 4 books, chips on ONE line. 285 (the value
  #         every other single-chip-row family uses) landed BELOW the box; the
  #         run was refused with "still shows 15 rows — filter did not apply".
  #   285 — SD-29 Epic 7 round 2, 7 books. The chip row WRAPPED to two lines
  #         (Bestiary 2's chip starts a second row), pushing the box down ~38px.
  #         247 then landed ABOVE it and the run was refused with "still shows
  #         77 rows". Same gate, opposite direction, one round apart.
  #
  #   323 — SD-29 Epic 7 round 9, 17 books. The chip row wrapped to a THIRD
  #         line and 285 landed ON the "Advanced Player's Guide (1)" chip: the
  #         run came back with "1 matching companion" and the Eidolon on screen
  #         instead of the searched record. The refusal was the RECORD gate
  #         rather than the filtered-count gate this time — a chip click is a
  #         real filter, so the count moved and only the record check caught it.
  #         Worth recording as its own shape: an off-by-one-row search click
  #         does not always look like an unfiltered list.
  #
  # Both refusals are the filtered-count gate doing its job rather than
  # screenshotting an unfiltered list and finding the record name in it. **The
  # constant is a function of registered book count and WILL move again** — the
  # next companion round should expect to recalibrate, and the gate will say so.
  companion)  HUB_X=855;  HUB_Y=971; SEARCH_Y=323; SCREEN_MARKER="Companion Catalog"; MATCH_WORD="matching" ;;
  *) echo "FAIL: unknown --family '$FAMILY' (known: equipment spell race_trait monster companion)" >&2; exit 2 ;;
esac
SEARCH_X=960
# A neutral left-margin point OUTSIDE every interactive element: clicking it
# blurs the search input so ctrl+a selects the DOCUMENT, not the input —
# with an input focused, select-all/copy yields only the input's own text
# (observed live: clipboard contained just the search query).
BLUR_X=150; BLUR_Y=600

[ -n "$SLUG" ] || SLUG="$(printf '%s--%s' "$FAMILY" "$RECORD" \
  | tr '[:upper:]' '[:lower:]' | tr -cs 'a-z0-9' '-' | sed 's/^-//;s/-$//')"
mkdir -p "$OUT_DIR"
SHOT="$OUT_DIR/$SLUG.png"
REPORT="$OUT_DIR/$SLUG.verify.md"
EXTRACT_FILE="$(mktemp)"
trap 'rm -f "$EXTRACT_FILE"' EXIT

# On any failure: rename artifacts so they cannot be cited as passing
# evidence, write the failure report, and exit nonzero.
fail() {
  local reason="$1"
  echo "FAIL: $reason" >&2
  [ -f "$SHOT" ] && mv "$SHOT" "$OUT_DIR/$SLUG.FAILED.png"
  {
    echo "# item-8 on-screen verification — FAILED"
    echo
    echo "- verdict: **FAILED** — $reason"
    report_common
    if [ -s "$EXTRACT_FILE" ]; then
      echo "- extraction (first 40 lines of what WAS on screen):"
      echo '```'
      head -n 40 "$EXTRACT_FILE"
      echo '```'
    else
      echo "- extraction: EMPTY (clipboard read failed or nothing rendered)"
    fi
  } > "$OUT_DIR/$SLUG.FAILED.verify.md"
  rm -f "$REPORT"
  exit 1
}

report_common() {
  echo "- family: \`$FAMILY\` · record: \`$RECORD\`"
  local e
  for e in "${EXPECTS[@]}"; do echo "- expected on screen: \`$e\`"; done
  echo "- agent: \`$RUN_DESKTOP_AGENT\` · date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "- HEAD: \`$(git -C "$SCRIPT_DIR" rev-parse --short HEAD 2>/dev/null || echo unknown)\`"
  echo "- harness: \`apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh\`"
}

# ------------------------------------------------------------ launch/reuse
app_alive() { [ -n "$("$DRIVER" _app_pid 2>/dev/null)" ]; }

# driver.sh's "Ready" means the WINDOW exists; on a cold start WebKitGTK can
# take a further ~100s (observed live, 4 contended cores) to paint the React
# app — first all-black, then blank-white, then content. Clicking before
# paint silently does nothing, so wait until a screenshot shows real content
# (standard deviation of pixel values; black and blank-white are both ~0).
wait_for_paint() {
  local probe stddev
  probe="$(mktemp --suffix=.png)"
  for _ in $(seq 1 36); do
    "$DRIVER" screenshot "$probe" >/dev/null 2>&1 || true
    stddev="$(identify -format "%[standard-deviation]" "$probe" 2>/dev/null || echo 0)"
    if awk -v s="$stddev" 'BEGIN{exit !(s>3000)}'; then rm -f "$probe"; return 0; fi
    sleep 5
  done
  rm -f "$probe"
  return 1
}

if [ -n "$FRESH" ] || ! app_alive; then
  "$DRIVER" launch || fail "driver.sh launch failed for agent '$RUN_DESKTOP_AGENT' — see its diagnose output above"
  app_alive || fail "launch reported success but no app process is alive on this agent's display"
  wait_for_paint || fail "window exists but the webview never painted content within 180s of launch"
else
  echo "Reusing live app for agent '$RUN_DESKTOP_AGENT'."
fi

# --------------------------------------------------- extraction primitive
# Prove text is really rendered: blur to the document (click a neutral
# margin point), select-all + copy in the webview, read the X clipboard.
STATE_FILE="/tmp/run-desktop-driver-${RUN_DESKTOP_AGENT}.state"
# shellcheck disable=SC1090
DISPLAY_NUM="$(source "$STATE_FILE" && echo "$DISPLAY_NUM")"
extract_screen_text() {
  "$DRIVER" click "$BLUR_X" "$BLUR_Y"
  "$DRIVER" key ctrl+a
  "$DRIVER" key ctrl+c
  sleep 1
  DISPLAY=":$DISPLAY_NUM" python3 "$SCRIPT_DIR/read-clipboard.py" 2>/dev/null > "$EXTRACT_FILE"
}
# Screen renders lag clicks by a variable amount on contended cores
# (observed >3s live) — poll for expected text instead of fixed sleeps.
wait_for_text() { # $1=needle, $2=attempts (3s apart)
  local i
  for i in $(seq 1 "$2"); do
    extract_screen_text || true
    grep -qF "$1" "$EXTRACT_FILE" && return 0
    sleep 3
  done
  return 1
}

# --------------------------------------------------------------- navigate
# Return to the hub first so the click lands on the landing screen even if a
# previous record left the app inside a catalog: every catalog screen has a
# top-right Back button (1476,80); on the hub itself that point is inert.
"$DRIVER" click 1476 80; sleep 2
"$DRIVER" click "$HUB_X" "$HUB_Y"
wait_for_text "$SCREEN_MARKER" 10 \
  || fail "navigation landed on the wrong screen — marker '$SCREEN_MARKER' not in rendered text (coordinates drifted?)"

# race_trait only: the Alternate racial traits tab (monster_codex records
# are all alternates and never match the standard tab's search).
if [ "$TAB" = "alternate" ]; then
  "$DRIVER" click 628 127; sleep 2
fi
# Extra clicks supplied by the caller (e.g. the race chip on the alternate
# tab, whose x depends on the race name). Wrong coordinates cannot produce
# a false pass — the record/expect checks below still gate the verdict.
for nc in "${NAV_CLICKS[@]:-}"; do
  [ -n "$nc" ] || continue
  "$DRIVER" click "${nc%,*}" "${nc#*,}"; sleep 2
done

# Filter to the record via the catalog search box (skipped for screens
# without one, e.g. the alternate-traits tab).
if [ -z "$NO_SEARCH" ]; then
  "$DRIVER" click "$SEARCH_X" "$SEARCH_Y"
  "$DRIVER" key ctrl+a
  "$DRIVER" type "$RECORD"
  sleep 2

  # The filter MUST demonstrably have applied. Without this gate the check
  # can pass on the UNFILTERED list: select-all extraction covers the whole
  # DOM including rows below the fold, so the record and expect strings can
  # all be "present" while the screenshot shows none of them (observed live:
  # first Ankheg run passed with the search click having missed the box and
  # 60 unfiltered rows on screen). Require the screen's own "N matching"
  # counter to be small enough that the record row is inside the screenshot
  # viewport — that is what makes the .png actual evidence.
  MATCH_N=""
  for _ in $(seq 1 4); do
    extract_screen_text || true
    MATCH_N="$(grep -oE "[0-9]+ $MATCH_WORD" "$EXTRACT_FILE" | head -1 | grep -oE "^[0-9]+" || true)"
    [ -n "$MATCH_N" ] && [ "$MATCH_N" -le 8 ] && break
    sleep 3
  done
  [ -n "$MATCH_N" ] || fail "no 'N $MATCH_WORD' counter in rendered text — either the search matched NOTHING (the screen swaps the counter for a 'No … match.' notice; record '$RECORD' is absent) or the filter never applied (search-box coordinates drifted)"
  [ "$MATCH_N" -le 8 ] || fail "search for '$RECORD' still shows $MATCH_N rows — filter did not apply (search click missed the box) or the query is too broad; the record cannot be proven in the screenshot viewport"
fi

wait_for_text "$RECORD" 4 \
  || fail "record '$RECORD' is NOT rendered on the $FAMILY screen — this is exactly the green-gate/empty-screen defect item 8 exists to catch"

# ------------------------------------------------------ capture evidence
[ "$SCROLL" -gt 0 ] && "$DRIVER" scroll 960 600 "$SCROLL"
"$DRIVER" click "$BLUR_X" "$BLUR_Y"   # drop any select-all highlight
"$DRIVER" screenshot "$SHOT" >/dev/null || fail "screenshot capture failed"
extract_screen_text \
  || fail "could not extract rendered text (empty clipboard) — cannot verify anything appeared"

grep -qF "$SCREEN_MARKER" "$EXTRACT_FILE" \
  || fail "marker '$SCREEN_MARKER' vanished from rendered text between navigation and capture"
grep -qF "$RECORD" "$EXTRACT_FILE" \
  || fail "record '$RECORD' is NOT rendered on the $FAMILY screen — this is exactly the green-gate/empty-screen defect item 8 exists to catch"
MISSING=()
for e in "${EXPECTS[@]}"; do
  grep -qF "$e" "$EXTRACT_FILE" || MISSING+=("$e")
done
[ "${#MISSING[@]}" -eq 0 ] \
  || fail "record is rendered but expected value(s) missing from screen: $(printf "'%s' " "${MISSING[@]}")"

# ------------------------------------------------------------------ report
{
  echo "# item-8 on-screen verification — PASS"
  echo
  echo "- verdict: **PASS** — record and all expected strings rendered on the live app screen"
  report_common
  echo "- screenshot: \`$SHOT\`"
  echo "- rendered lines containing the record/expectations:"
  echo '```'
  { grep -nF "$RECORD" "$EXTRACT_FILE" || true
    for e in "${EXPECTS[@]}"; do grep -nF "$e" "$EXTRACT_FILE" || true; done
  } | sort -t: -k1,1n -u | head -n 30
  echo '```'
} > "$REPORT"

echo "PASS: $FAMILY / $RECORD — evidence: $SHOT + $REPORT"
