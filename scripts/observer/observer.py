#!/usr/bin/env python3
"""Release-swarm observer renderer.

Reads four canonical sources; never writes back to the swarm:

  1. SWARM_STATUS.md          (lead authored state, doc/release/v0.6/)
  2. SWARM_TASKS.md           (per-task sidecar, format pinned 2026-07-23)
  3. risks-and-open-questions.md (operator-elevation register)
  4. /usage cache             (~/.cache file written by /usr/local/sbin/usage-wrapper.sh)

Stdlib only. Runs via hermes cron, watchdog, or manual tick.

Honors the §8.2 "what the observer is NOT allowed to do" guardrail:
no writes to SWARM_STATUS.md, no SendMessage, no task-list mutation, no spawning.
"""

from __future__ import annotations

import argparse
import datetime as dt
import glob
import html
import json
import os
import pathlib
import re
import sys

# ---------------------------------------------------------------------------
# Paths (operator-overridable via env vars)
# ---------------------------------------------------------------------------

DEFAULT_STATUS = "/home/ubuntu/workspace/repos/codex/docs/release/v0.6/SWARM_STATUS.md"
DEFAULT_MAILBOX_GLOB = "/home/ubuntu/.claude/teams/*/inboxes/*.json"
DEFAULT_TASK_GLOB = "/home/ubuntu/.claude/tasks/*/*.json"
DEFAULT_OUT = "/home/ubuntu/swarm-observer/dashboard.html"
DEFAULT_LIVE = "/home/ubuntu/swarm-observer/SWARM_LIVE.txt"
DEFAULT_TASKS_SIDECAR = "/home/ubuntu/workspace/repos/codex/docs/release/v0.6/SWARM_TASKS.md"
DEFAULT_RISKS_DOC = "/home/ubuntu/workspace/repos/codex/docs/release/v0.6/risks-and-open-questions.md"
DEFAULT_REPORT = "/home/ubuntu/workspace/repos/codex/docs/release/v0.6/SWARM_REPORT.md"
DEFAULT_USAGE_CACHE = "/home/ubuntu/swarm-observer/.usage-cache.txt"
DEFAULT_WEEKLY_CAP = 0

TASK_COLUMNS = ["task", "owner", "rulebook", "status"]
VALID_OWNERS = {"UI", "Backend", "QA"}
VALID_STATUSES = {"done", "in progress", "queued", "blocked"}


# ---------------------------------------------------------------------------
# SWARM_STATUS.md parser (free-form (a)/(b)/(c) sections)
# ---------------------------------------------------------------------------

STATUS_HAPPENING_RE = re.compile(r"(?im)^##\s+A\b|^\*\*\(a\)\s+Happening now|^###\s+A\b|^\(a\)\s+Happening now\b")
STATUS_HAPPENED_RE = re.compile(r"(?im)^##\s+B\b|^\*\*\(b\)\s+Happened|^###\s+B\b|^\(b\)\s+Happened\b")
STATUS_ONDECK_RE = re.compile(r"(?im)^##\s+C\b|^\*\*\(c\)\s+On deck|^###\s+C\b|^\(c\)\s+On deck\b")


def read_status(path):
    """Parse the lead's three-section SWARM_STATUS.md.

    Accepts both ATX (`## A` / `**(a) Happening now**`) and setext
    (`(a) Happening now` followed by `---`) heading shapes, because the
    lead uses plain-text-with-underline shape.
    """
    p = pathlib.Path(path)
    placeholder = "[Lead has not written SWARM_STATUS.md yet. Lead owns (a)/(b)/(c); see swarm-doc §4.1 step 7.]"
    if not p.exists():
        return {"happening": placeholder, "happened": placeholder, "ondeck": placeholder}
    raw = p.read_text(encoding="utf-8", errors="replace")
    lines = raw.splitlines()
    sections = {"happening": [], "happened": [], "ondeck": []}
    current = None
    for line in lines:
        s = line.strip()
        if STATUS_HAPPENING_RE.match(s):
            current = "happening"
            continue
        if STATUS_HAPPENED_RE.match(s):
            current = "happened"
            continue
        if STATUS_ONDECK_RE.match(s):
            current = "ondeck"
            continue
        if current is not None and re.match(r"^[-=*_]{3,}\s*$", line):
            continue
        if current is not None:
            sections[current].append(line)
    return {
        "happening": "\n".join(sections["happening"]).strip(),
        "happened":  "\n".join(sections["happened"]).strip(),
        "ondeck":    "\n".join(sections["ondeck"]).strip(),
    }


# ---------------------------------------------------------------------------
# Agent-status parser (extracts orchestrator/backend/frontend/qa from the
# most recent "## Agent Status (timestamp)" block in the lead's full
# SWARM_STATUS.md — not the (a) Happening-now prose, which is empty
# during autonomous-mode sessions. Falls back to (a) if no Agent Status
# block exists.
# ---------------------------------------------------------------------------

AGENT_NAMES = ("orchestrator", "backend", "frontend", "qa")

# Match "## Agent Status (...)" or "### Agent Status (...)" headers.
AGENT_STATUS_HEADER_RE = re.compile(
    r"(?im)^(?P<hashes>#{1,4})\s+Agent Status\s*(?:\([^)]*\))?\s*$"
)

# Phrases that suggest each bucket. Order: most specific to least.
RUNNING_PHRASES = (
    "FULLY AUTONOMOUS", "in flight", "mid-edit", "live-verifying", "live-verified",
    "delivering", "delivered", "wiring", "running", "executing", "deliver",
    "fresh-eyes review", "active", "publishing", "re-verifying", "re-verified",
    "fix landed", "landed (", "fix-verify", "fix verify",
)
IDLE_PHRASES = (
    "standing by", "idle", "watching for", "no commit", "no commits",
    "ScheduleWakeup", "not actively", "paused", "parked",
)
WAITING_PHRASES = (
    "waiting for", "waiting on", "blocked", "BLOCKED", "awaits",
    "queued",
)


def _classify_agent_status(agent: str, snippet: str) -> str:
    """Classify an agent's status based on the lead's prose snippet.

    Returns one of: 'Running', 'Idle', 'Waiting'. Orchestrator with a
    non-empty snippet is always 'Running' (the lead is alive and directing).
    """
    s_lower = snippet.lower()
    # waiting phrases win (blocked-agents, queued tasks, awaits)
    for p in WAITING_PHRASES:
        if p.lower() in s_lower:
            return "Waiting"
    for p in RUNNING_PHRASES:
        if p.lower() in s_lower:
            return "Running"
    for p in IDLE_PHRASES:
        if p.lower() in s_lower:
            return "Idle"
    # No phrase match: orchestrator defaults Running; others default Idle.
    if agent == "orchestrator":
        return "Running"
    return "Idle"


AGENT_STATUS_HEADER_RE = re.compile(
    r"(?im)^(?P<hashes>#{1,4})\s+Agent Status\s*(?:\([^)]*\))?\s*$"
)


def find_latest_agent_status_block(text: str) -> str:
    """Return the body of the most recent ## Agent Status (...) block, or ''.

    The lead writes Agent Status blocks throughout the document, not just in
    (a) Happening-now. The newest block is the authoritative view; older blocks
    are stale history. The block body runs to the next markdown header of
    equal-or-deeper depth, or end of text.
    """
    if not text:
        return ""
    matches = list(AGENT_STATUS_HEADER_RE.finditer(text))
    if not matches:
        return ""
    last = matches[-1]
    header_hash_count = len(last.group("hashes"))
    after = text[last.end():]
    stop_re = re.compile(r"(?im)^#{1," + str(header_hash_count) + r"}\s+\S")
    stop_m = stop_re.search(after)
    return after[: stop_m.start()].strip() if stop_m else after.strip()


def parse_agent_status_from_block(block: str) -> list:
    """Parse a single Agent Status block (markdown table or prose) into agent records.

    Supports two formats:
      - Markdown table: | Agent | Status | Detail | ... |
      - Prose lines:   "<agent-name>  <model>  <running|...>  <detail>"

    Returns a list of dicts: [{agent, status, snippet}] in canonical order.
    If an agent is missing, defaults to 'Idle' / empty snippet.
    """
    out = []
    found = {}

    # Pass 1: markdown table rows.
    for line in block.splitlines():
        line = line.strip()
        if not line.startswith("|"):
            continue
        cells = [c.strip() for c in line.strip("|").split("|")]
        if len(cells) < 2:
            continue
        first = cells[0].lower()
        if first in ("agent", "---", "agent status"):
            continue
        for name in AGENT_NAMES:
            if first == name or first.startswith(name + " ") or first.startswith(name + "("):
                snippet = " ".join(cells[2:]) if len(cells) >= 3 else cells[1]
                raw_status = cells[1].strip().lower() if len(cells) >= 2 else ""
                if raw_status in ("running", "working", "active", "delivering", "publishing"):
                    status = "Running"
                elif raw_status in ("idle", "standing by", "waiting for work"):
                    status = "Idle"
                elif raw_status in ("waiting", "blocked", "paused", "queued"):
                    status = "Waiting"
                else:
                    status = _classify_agent_status(name, snippet)
                found[name] = {"status": status, "snippet": snippet}
                break

    # Pass 2: prose lines (fallback if no table).
    if not found:
        for name in AGENT_NAMES:
            pat = re.compile(
                r"(?im)^\s*" + re.escape(name) + r"(?:\s*\([^)]*\))?\s+\w+[ \t]+(.+)$"
            )
            m = pat.search(block)
            if m:
                snippet = re.sub(r"\s+", " ", m.group(1).strip())
                found[name] = {"status": _classify_agent_status(name, snippet), "snippet": snippet}

    for name in AGENT_NAMES:
        if name in found:
            out.append({"agent": name, "status": found[name]["status"], "snippet": found[name]["snippet"]})
        else:
            out.append({"agent": name, "status": "Idle", "snippet": ""})
    return out


def parse_agent_status(full_text: str, happening_text: str = "") -> list:
    """Find the four agents in the most recent Agent Status block.

    Loader contract: pass the full SWARM_STATUS.md contents. The function
    locates the newest `## Agent Status (...)` block, parses the table inside,
    and falls back to the (a) Happening-now prose if no Agent Status block
    exists (legacy behavior).
    """
    block = find_latest_agent_status_block(full_text)
    if block:
        return parse_agent_status_from_block(block)
    # Fallback: legacy (a) prose scan.
    if not happening_text:
        return [{"agent": name, "status": "Idle", "snippet": ""} for name in AGENT_NAMES]
    out = []
    for name in AGENT_NAMES:
        pat = re.compile(
            r"(?im)^\s*" + re.escape(name) + r"(?:\s*\([^)]*\))?\s+\w+[ \t]+(.+)$"
        )
        m = pat.search(happening_text)
        if m:
            snippet = re.sub(r"\s+", " ", m.group(1).strip())
            status = _classify_agent_status(name, snippet)
        else:
            snippet = ""
            status = "Idle" if name != "orchestrator" else "Running"
        out.append({"agent": name, "status": status, "snippet": snippet})
    return out


def read_status_raw(path: str) -> str:
    """Read the raw SWARM_STATUS.md text (not the parsed (a)/(b)/(c) sections)."""
    p = pathlib.Path(path)
    if not p.exists():
        return ""
    return p.read_text(encoding="utf-8", errors="replace")


# ---------------------------------------------------------------------------
# Class/race chassis breadth parsers
# ---------------------------------------------------------------------------

# Latest **Progress: <X> of <Y> classes ...** line — written by the lead bold-style
# in SWARM_STATUS.md. The newest occurrence is the authoritative view.
PROGRESS_LINE_RE = re.compile(
    r"\*\*Progress:[^\n]*?(\d+)\s+of\s+(\d+)\s+classes?[^*\n]*\*\*"
)

# Backstop: the "X of 27 classes. N remain." pattern that appears in plain
# prose (not bold) — the lead uses this in milestone summaries.
PROGRESS_FALLBACK_RE = re.compile(
    r"\(\s*(\d+)\s+of\s+(\d+)\s+classes?\s*\)\.?\s*(\d+)\s+remain"
)

# The lead's milestone framing: `<X> of 27 classes now genuinely reach Computed**:
# <list>`. The asterisks between "Computed" and ":" are the bold-marker artifact
# the lead uses. One or more asterisks are accepted.
MILESTONE_RE = re.compile(
    r"\b(\d+)\s+of\s+27\s+classes?\s+now\s+genuinely\s+reach\s+Computed\*+:?\s*"
    r"(?P<list>[^*\n]+)\.?"
)

# Full canonical roster — 27 classes per the source-side
# MARTIAL_CLASS_NAMES (10) + SPELLCASTING_CLASS_NAMES (17) in
# src/pcgen_import/lst_parser/{class,spellcasting_class}.rs. The lead's
# "<X> of 27 classes" reach-list denominator is this exact roster, NOT the
# 11-class CRB subset that the previous dashboard mistakenly showed.
FULL_CLASS_ROSTER = [
    # CRB martial (6)
    "barbarian", "fighter", "monk", "paladin", "ranger", "rogue",
    # CRB spellcasting (5)
    "bard", "cleric", "druid", "sorcerer", "wizard",
    # APG martial (1)
    "cavalier",
    # APG spellcasting (5)
    "alchemist", "inquisitor", "oracle", "summoner", "witch",
    # ACG martial (3)
    "brawler", "slayer", "swashbuckler",
    # ACG spellcasting (7)
    "arcanist", "bloodrager", "hunter", "investigator", "shaman", "skald", "warpriest",
]
# Total: 27. The lead's "of 27" is the source of truth.

# Maps each class to its source book so the dashboard can surface CRB/APG/ACG
# as a subhead on each class card.
CLASS_BOOK = {}
for _cid in ("barbarian", "fighter", "monk", "paladin", "ranger", "rogue",
             "bard", "cleric", "druid", "sorcerer", "wizard"):
    CLASS_BOOK[_cid] = "CRB"
for _cid in ("cavalier", "alchemist", "inquisitor", "oracle", "summoner", "witch"):
    CLASS_BOOK[_cid] = "APG"
for _cid in ("brawler", "slayer", "swashbuckler",
             "arcanist", "bloodrager", "hunter", "investigator", "shaman", "skald", "warpriest"):
    CLASS_BOOK[_cid] = "ACG"

# Classes with their own dedicated numbered entry in
# risks-and-open-questions.md (beyond the per-class detail already carried in
# SWARM_REPORT.md's own tables) — the dashboard's open_question field appends
# this citation so a viewer can jump straight to the fuller writeup.
KNOWN_RISK_ITEM_REFS = {
    "summoner": 38,
}

# Matches the concise blocker clause inside a class's SWARM_REPORT.md detail
# cell, preferring an explicit "still/stays Blocked on ..." or "Only ...
# remains ..." statement over the surrounding build history. Stops at the
# next literal "|" (the boundary the source row regex left in place between
# this table's own trailing columns, e.g. the CRB table's "Landed in" cell)
# rather than requiring a sentence-ending period — several real cells phrase
# the blocker with an em-dash instead of a period (Monk's "Only Deflect
# Arrows remains — ..."), and `Blocked**` (bold markdown closing right after
# the word) breaks a plain "Blocked on" match, so `\**\s*` tolerates it.
_OPEN_QUESTION_PHRASE_RE = re.compile(
    r"(dispatch-only,?\s*Blocked[^|]*"
    r"|Only\s+[^|]*?remains[^|]*"
    r"|stays?\s+(?:permanently\s+)?Blocked\**\s*on[^|]*"
    r"|still Blocked\**\s*on[^|]*)",
    re.IGNORECASE,
)

_OPEN_QUESTION_MAX_LEN = 280


def extract_open_question(class_id: str, detail: str, state: str) -> str:
    r"""Derive a concise blocker summary for one class from its raw
    SWARM_REPORT.md detail cell, for the dashboard's per-class open_question
    field (operator directive 2026-07-26: every non-full class must show a
    real, populated reason on the dashboard, not an empty field).

    A "full" class has no open question — returns "". Otherwise, prefers the
    detail cell's own explicit blocker clause ("stays Blocked on ...", "Only
    <feat> remains ...", "dispatch-only, Blocked ..."); falls back to a
    truncated raw cell if no such clause is present, so the field is never
    silently empty for a real blocker. Length-capped either way (a match can
    still run long when it's the table's last column, with no further "|" to
    stop at). Appends a risks-and-open-questions.md citation for classes with
    their own dedicated numbered entry there.

    Bug fixed 2026-07-27 (lead audit): `parse_class_report_table`'s row regex
    merges every non-name cell into one `detail` blob regardless of the
    APG/ACG table's real 4-column shape (Class | BAB | Hit die | Status), by
    design (see that function's own doc comment) -- it relies on THIS
    function's phrase regex to isolate the real content. When a row's Status
    cell doesn't happen to contain the literal "stays/still Blocked on ..."
    phrasing (e.g. it says "stay deferred" instead), the phrase regex found
    nothing and the fallback used the ENTIRE blob, leaking the leading
    "3/4 | d8 | " BAB/Hit-die columns into the dashboard (confirmed on
    Inquisitor and Hunter's real rows). Fixed by trimming the fallback to
    start at the row's own "**Blocked**"/"**Computed**" status marker (the
    one part of the blob every row shape is guaranteed to carry), not the
    raw blob from its start -- so the fallback degrades to a clean sentence
    instead of a column leak, independent of which phrasing convention the
    lead used that cycle.
    """
    if state == "full" or not detail:
        return ""
    detail = re.sub(r"\s+", " ", detail).strip()
    match = _OPEN_QUESTION_PHRASE_RE.search(detail)
    if match:
        text = match.group(0).strip()
    else:
        marker = re.search(r"\*\*(?:Blocked|Computed)\*\*", detail, re.IGNORECASE)
        text = detail[marker.start():] if marker else detail
    text = re.sub(r"\*+", "", text).strip(" -—")
    if len(text) > _OPEN_QUESTION_MAX_LEN:
        text = text[:_OPEN_QUESTION_MAX_LEN].rstrip() + "…"
    ref = KNOWN_RISK_ITEM_REFS.get(class_id)
    if ref is not None:
        text = f"{text} (see risks-and-open-questions.md item {ref})"
    return text


# Support-level labels per the live characterHubModel.classSupportLevel enum.
CLASS_SUPPORT_LEVELS = {
    "full": "Full (any race, any level)",
    "full-except-human-level-1": "Full (any race, any level except Human level 1)",
    "partial-human-only": "Partial (race/level flags still human-gated)",
    "human-diagnostics-only": "Human-only diagnostics (never reaches Computed)",
}


def parse_class_breadth_progress(status_text: str) -> dict:
    r"""Extract the latest class-chassis breadth progress statement.

    The lead writes bold `**Progress: ... <X> of <Y> classes ...**` lines.
    Newest occurrence wins. Falls back to a plain-prose
    `(\d+ of \d+ classes). \d+ remain` pattern if no bold-style line is found.

    Returns {reached, total, rest, raw_line} or {reached: None, total: None,
    rest: '', raw_line: ''} if no Progress line is found.
    """
    out = {"reached": None, "total": None, "rest": "", "raw_line": ""}
    if not status_text:
        return out

    # Pass 1: bold Progress line. Records the current best-effort count
    # but does NOT early-return — Pass 3 may override if a later milestone
    # has a higher count.
    matches = list(PROGRESS_LINE_RE.finditer(status_text))
    if matches:
        last = matches[-1]
        reached = int(last.group(1))
        total = int(last.group(2))
        raw = last.group(0)
        # Trim the bold markers and the leading "Progress:" prefix from the rest.
        rest = re.sub(r"^\*\*Progress:\s*", "", raw)
        rest = re.sub(r"\*\*\s*$", "", rest).strip()
        out.update({"reached": reached, "total": total, "rest": rest, "raw_line": raw})

    # Pass 2: plain-prose fallback.
    fb = list(PROGRESS_FALLBACK_RE.finditer(status_text))
    if fb and out["reached"] is None:
        last = fb[-1]
        out["reached"] = int(last.group(1))
        out["total"] = int(last.group(2))
        out["rest"] = f"{last.group(3)} remain"
        out["raw_line"] = last.group(0)

    # Pass 3: milestone pattern. Whichever count is higher wins.
    mile = list(MILESTONE_RE.finditer(status_text))
    if mile:
        last = mile[-1]
        m_reached = int(last.group(1))
        m_total = 27  # the regex hard-codes 27; total is invariant for the CRB roster
        m_list = last.group("list").strip()
        if out["reached"] is None or m_reached > out["reached"]:
            out["reached"] = m_reached
            out["total"] = m_total
            # Strip trailing backmatter (parens, etc.) from the list for the headline.
            out["rest"] = m_list.split("(")[0].rstrip()
            out["raw_line"] = last.group(0)
    return out



# Matches a markdown table row naming one of the roster's classes as its
# first cell: `| ClassName | ... |`. The class-name cell is matched
# case-insensitively against FULL_CLASS_ROSTER; the rest of the row (every
# other cell, to end of line) is returned as `detail` so the caller can
# classify state from its own status vocabulary rather than a second regex.
_REPORT_ROW_RE = re.compile(r"^\|\s*([A-Za-z][A-Za-z ]*?)\s*\|(?P<detail>.*)\|\s*$", re.MULTILINE)


def parse_class_report_table(report_text: str) -> dict:
    r"""Parse per-class state directly from SWARM_REPORT.md's own CRB/APG/ACG
    tables — the lead's canonical, structured per-cycle status record (see
    SWARM_REPORT.md's own "Full class/race chassis breadth" section).

    This is a more robust source than `parse_class_chassis_table`'s own
    prose-scraping of SWARM_STATUS.md: the lead maintains one table row per
    class every cycle, each with a status cell that always starts with one
    of three literal markers:
      "**Computed**"                    -> "full"
      "**Blocked** ... real progress"    -> "in-progress"
      "dispatch-only, Blocked ... untouched" -> "untouched"
    Whereas the prose-scraper depends on the lead writing a specific
    ALL-CAPS "**CLASS'S FEATURE COMMITTED**" bold-header shape for every
    single closure — a real, silent failure mode when the lead's own
    narrative style varies even slightly (confirmed 2026-07-26: Alchemist,
    Inquisitor, and Oracle all have substantial real, documented progress
    but were misclassified "untouched" because their SWARM_STATUS.md prose
    used ordinary Title Case rather than the exact ALL-CAPS pattern; Arcanist
    was stuck at "in-progress" instead of "full" because the separate
    reach-list regex's exact phrase hadn't been repeated since Bard).

    Returns {class_id: {"state": state, "detail": raw_detail}} for every
    class row found. Classes not mentioned in the report text at all are
    omitted (caller should fall back to another source or "untouched").
    `detail` is the row's own raw cell text (the CRB table's "Race/level
    support" cell, or the APG/ACG tables' own "Status" cell — whichever this
    row's table shape used), kept verbatim so the caller can derive an
    open-question summary from it without a second parse pass.
    """
    out = {}
    if not report_text:
        return out
    for m in _REPORT_ROW_RE.finditer(report_text):
        name = m.group(1).strip().lower()
        if name not in FULL_CLASS_ROSTER:
            continue
        detail = m.group("detail")
        if "**computed**" in detail.lower():
            out[name] = {"state": "full", "detail": detail}
        elif "untouched" in detail.lower():
            out[name] = {"state": "untouched", "detail": detail}
        elif "real progress" in detail.lower() or "**blocked**" in detail.lower():
            out[name] = {"state": "in-progress", "detail": detail}
        # Anything else (a row shape not yet seen) is left unclassified
        # rather than guessed — the caller's fallback logic handles it.
    return out


def parse_class_chassis_table(status_text: str, report_text: str = "") -> list:
    r"""Parse the full-roster class state from SWARM_STATUS.md.

    The argument is misleadingly named `risks_text` in the previous signature
    but the function ALWAYS read SWARM_STATUS.md (the milestone lines live
    there, not in the risks doc). Updated to `status_text` for honesty.

    Returns a list of 27 class entries in canonical roster order, each
    {class_id, book, state, source, open_question}. `open_question` is a
    concise blocker summary (empty string for "full" classes) — see
    `extract_open_question`'s own doc comment; only populated when this
    class came from the report-table source, since that's the only source
    carrying real detail text. The state is one of:
      "full"        — class is in the latest "<X> of 27 classes ... Computed"
                      reach-list. Source: "reach-list".
      "in-progress" — class has real partial engine progress committed but
                      is NOT at full reach. Source: "milestone-pattern".
                      Detected from lines like:
                        "<class> COMMITTED (`abc123`)"
                        "MILESTONE ... -- <class> IS THE FIRST/SECOND/... APG/ACG CLASS TO GET REAL PILLAR-INTEGRATION WORK"
                        "MILESTONE ... -- <class>'S ... BUILT / LANDED / COMMITTED"
      "untouched"   — no class-specific work yet. Source: "roster-default".

    The previous (CRB-only) parser returned 11 entries and labeled the
    other 16 as "unknown" — operator flagged this as missing the actual
    scope. Full 27-class roster is correct.
    """
    ROSTER = FULL_CLASS_ROSTER

    # 0) Authoritative source: SWARM_REPORT.md's own structured per-class
    #    tables (see `parse_class_report_table`'s own doc comment for why
    #    this takes priority over the prose-scraping below — the reach-list/
    #    milestone-pattern heuristics silently misclassify any class whose
    #    SWARM_STATUS.md prose doesn't happen to match their exact phrase
    #    shapes, confirmed 2026-07-26 for Alchemist/Inquisitor/Oracle/
    #    Arcanist). Classes not covered by the report (or when no report
    #    text is supplied, e.g. an older caller) fall through to the
    #    original reach-list/milestone-pattern detection below unchanged.
    report_states = parse_class_report_table(report_text)

    # 1) Reach-list: extract the latest "X of 27 classes now genuinely reach
    #    Computed: <list>" milestone.
    reach_set = set()
    if status_text:
        matches = list(MILESTONE_RE.finditer(status_text))
        if matches:
            raw = matches[-1].group("list")
            head = raw.split("**")[0].split("(")[0]
            for tok in head.split(","):
                clean = re.sub(r"\([^)]*\)", "", tok).rstrip(". ;").strip().lower()
                if clean in ROSTER:
                    reach_set.add(clean)

    # 2) In-progress: detect classes named in commit/MILESTONE lines that are
    #    NOT in the reach-list. The lead writes three patterns:
    #       "**Skald IS THE FIRST APG/ACG CLASS TO GET REAL PILLAR-INTEGRATION WORK"
    #       "**CLASS COMMITTED (\`abc123\`)"
    #       "**CLASS'S <feature> BUILT AND LEAD-VERIFIED PRE-COMMIT"
    #       "**CLASS'S <feature> LANDED (\`abc123\`)"
    in_progress = set()
    if status_text:
        # Pattern: "**CLASS COMMITTED (`abc123`)" or "**CLASS LANDED (`abc123`)"
        # Class names are written ALL CAPS in the lead's commit lines.
        for m in re.finditer(
            r"\*\*([A-Z][A-Z]+)\s+(COMMITTED|LANDED)\s*\(",
            status_text,
        ):
            cls = m.group(1).lower()
            if cls in ROSTER and cls not in reach_set:
                in_progress.add(cls)
        # Pattern: "**MILESTONE ... -- CLASS IS THE FIRST/SECOND/THIRD/FOURTH/FIFTH"
        for m in re.finditer(
            r"\*\*MILESTONE[^\n]*--\s+([A-Z][A-Z]+)\s+IS\s+(?:THE\s+)?(?:FIRST|SECOND|THIRD|FOURTH|FIFTH)",
            status_text,
        ):
            cls = m.group(1).lower()
            if cls in ROSTER and cls not in reach_set:
                in_progress.add(cls)
        # Pattern: bold "**CLASS'S <feature> BUILT" or "**CLASS'S <feature>
        # COMMITTED" — the lead writes these with or without a MILESTONE
        # prefix, and the apostrophe-S after the class is uppercase
        # ("BRAWLER'S", not "Brawler's"). Examples:
        #   "BRAWLER'S AC BONUS BUILT AND LEAD-VERIFIED PRE-COMMIT" (no MILESTONE)
        #   "MONK'S 6TH AND 7TH FEATS COMMITTED" (with MILESTONE)
        #   "HUNTER'S ANIMAL COMPANION BUILT" (with MILESTONE)
        #   "CAVALIER'S MOUNT BUILT" (with MILESTONE).
        for m in re.finditer(
            r"\*\*(?:MILESTONE[^\n]*--\s+)?([A-Z][A-Z]+)[’']S\s+[A-Z][A-Z0-9 ]+\s+(?:BUILT|COMMITTED|LANDED)",
            status_text,
        ):
            cls = m.group(1).lower()
            if cls in ROSTER and cls not in reach_set:
                in_progress.add(cls)
        # Pattern: standalone "**CLASS'S <feature> COMMITTED" with feature
        # in Title case (e.g. "**MONK'S DODGE FIX COMMITTED").
        for m in re.finditer(
            r"\*\*([A-Z][A-Z]+)[’']S\s+[A-Z][a-z]+[\w\s]*\s+COMMITTED",
            status_text,
        ):
            cls = m.group(1).lower()
            if cls in ROSTER and cls not in reach_set:
                in_progress.add(cls)

    # 3) Build the roster. Each class is exactly one of {full, in_progress,
    #    untouched}. The source field names the lead's prose that classified
    #    it.
    out = []
    for cid in ROSTER:
        if cid in report_states:
            state = report_states[cid]["state"]
            out.append({
                "class_id": cid,
                "book": CLASS_BOOK[cid],
                "state": state,
                "source": "swarm-report-table",
                "open_question": extract_open_question(cid, report_states[cid]["detail"], state),
            })
        elif cid in reach_set:
            out.append({
                "class_id": cid,
                "book": CLASS_BOOK[cid],
                "state": "full",
                "source": "reach-list",
                "open_question": "",
            })
        elif cid in in_progress:
            fallback = "In progress — see SWARM_STATUS.md for detail (not yet in SWARM_REPORT.md's own table)."
            if cid in KNOWN_RISK_ITEM_REFS:
                fallback += f" See risks-and-open-questions.md item {KNOWN_RISK_ITEM_REFS[cid]}."
            out.append({
                "class_id": cid,
                "book": CLASS_BOOK[cid],
                "state": "in-progress",
                "source": "milestone-pattern",
                "open_question": fallback,
            })
        else:
            out.append({
                "class_id": cid,
                "book": CLASS_BOOK[cid],
                "state": "untouched",
                "open_question": (
                    f"Untouched — see risks-and-open-questions.md item {KNOWN_RISK_ITEM_REFS[cid]}."
                    if cid in KNOWN_RISK_ITEM_REFS
                    else "Untouched — no class-specific work started yet."
                ),
                "source": "roster-default",
            })
    return out


def parse_session_prose(status_text: str) -> dict:
    """Extract the lead's own session-progress prose.

    Three signals:
      1. session_total: the latest "**Session total: ...**" line — the lead's
         own accounting of mechanisms/closures/bug-fixes built this session.
      2. uncommitted_milestone: the latest "MILESTONE (uncommitted at check time)
         -- ..." line — work that's verified but not yet committed.
      3. in_progress_lines: the latest "in progress, uncommitted" / "in flight"
         prose snippet showing what's actively being worked.

    Returns {"session_total": str, "uncommitted_milestone": str,
             "in_progress_lines": [str, ...]} with empty strings / lists
    when no signal is found.
    """
    out = {"session_total": "", "uncommitted_milestone": "", "in_progress_lines": []}
    if not status_text:
        return out

    # Session total: the lead writes this as a single bold line.
    st_m = list(re.finditer(r"\*\*Session total:[^\n]+", status_text))
    if st_m:
        line = st_m[-1].group(0)
        # Strip the bold markers and the leading "Session total:" prefix.
        line = re.sub(r"^\*\*Session total:\s*", "", line)
        line = line.rstrip("*").rstrip()
        out["session_total"] = line.strip()

    # Uncommitted milestone: the lead writes "MILESTONE (uncommitted at check time)".
    um_m = list(re.finditer(r"\*\*MILESTONE \(uncommitted[^\n]+", status_text))
    if um_m:
        line = um_m[-1].group(0)
        # Pull out the headline class name + a short summary.
        line = re.sub(r"^\*\*", "", line)
        line = line.rstrip("*").rstrip()
        out["uncommitted_milestone"] = line.strip()

    # In-progress prose: find the last few "in progress, uncommitted" or
    # "in flight" / "in progress" lines. The lead writes these in narrative
    # form, often inside a paragraph. We look for whole paragraphs that
    # contain the trigger phrase and grab the surrounding context.
    paragraphs = re.split(r"\n\n+", status_text)
    for para in reversed(paragraphs):
        if "in progress, uncommitted" in para or "in flight" in para:
            # Extract the headline sentence (the first sentence with the
            # class name or mechanism name).
            sentences = re.split(r"(?<=\.)\s+", para)
            head = ""
            for s in sentences:
                if "in progress" in s or "in flight" in s:
                    head = s.strip()
                    break
            if head and len(out["in_progress_lines"]) < 3:
                out["in_progress_lines"].append(head)
    return out


def parse_class_reach_list(risks_text: str) -> str:
    """Return the most recent milestone's reach-list as a comma-separated string.

    Used by the chassis progress card to print the *list* of classes that
    reach Computed, not just the count. Empty string if no milestone found.
    """
    if not risks_text:
        return ""
    matches = list(re.finditer(
        r"\b(\d+)\s+of\s+27\s+classes?\s+now\s+genuinely\s+reach\s+Computed\*+:?\s*"
        r"(?P<list>[^*\n]+)\.?",
        risks_text,
    ))
    if not matches:
        return ""
    raw = matches[-1].group("list")
    # Truncate at the first bold-marker or paren — the 9-of-27 line has
    # backmatter in parens that should not appear in the headline.
    head = raw.split("**")[0].split("(")[0]
    return head.rstrip(". ;").strip()


def parse_race_chassis_table(risks_text: str) -> list:
    """Parse the per-race support table from the risks doc.

    Returns a list of race entries {race_id, status} where status is one of
    "fully-supported" | "human-only" | "human-diagnostics-only" | "unknown".
    """
    RACES = [
        "human", "dwarf", "elf", "gnome", "halfling", "half-elf", "half-orc",
    ]
    out = []
    if not risks_text:
        return out
    for rid in RACES:
        idx = risks_text.lower().find(rid)
        if idx < 0:
            continue
        window = risks_text[max(0, idx - 200): idx + 600]
        # Cheap heuristic: if the window has "human-diagnostics-only" or
        # "human-only", that's the status. Otherwise, full.
        if "human-diagnostics-only" in window.lower():
            status = "human-diagnostics-only"
        elif "human-only" in window.lower():
            status = "human-only"
        else:
            status = "fully-supported"
        out.append({"race_id": rid, "status": status})
    return out


def render_chassis_breadth_html(progress: dict, classes: list, races: list,
                              reach_list: str = "",
                              session_prose: dict = None) -> str:
    """Render the class/race chassis breadth cards.

    Layout: progress card on top, optional session-prose callout (lead's
    Session total / uncommitted MILESTONE / in-progress prose), then a
    visible section header between the progress card and the class cards,
    then a class grid, then another section header for races, then a race
    grid. The double-header layout reads as 3 distinct sub-sections
    ("headline" -> "classes" -> "races") so the eye never fuses the
    headline number with the race count.
    """
    if session_prose is None:
        session_prose = {}
    parts = ['<h2>Class &amp; race chassis breadth</h2>']
    if progress.get("reached") is not None:
        reached = progress["reached"]
        total = progress["total"]
        rest = html.escape(progress.get("rest", ""))
        pct = (reached / total * 100) if total else 0
        parts.append(
            f'<div class="chassis-card chassis-progress">'
            f'<div class="chassis-num">{reached} / {total}</div>'
            f'<div class="chassis-label">classes reach Computed ({pct:.0f}%)</div>'
            f'<div class="chassis-rest">{rest}</div>'
            f'</div>'
        )
    else:
        parts.append(
            '<div class="chassis-card chassis-progress chassis-empty">'
            '<div class="chassis-label">no Progress line in SWARM_STATUS.md</div>'
            '</div>'
        )

    # Session-prose callout: the lead's own progress accounting, surfaced
    # verbatim. This is the dashboard's way of saying "10 of 27 reach
    # Computed, but there's also in-flight work the headline doesn't
    # capture." When session_prose is empty, the section is omitted.
    if session_prose and any(session_prose.values()):
        parts.append('<div class="chassis-card chassis-session">')
        parts.append('<div class="chassis-session-title">Lead session prose '
                     '(canonical source: SWARM_STATUS.md)</div>')
        if session_prose.get("session_total"):
            parts.append(
                f'<div class="chassis-session-line">'
                f'<strong>Session total:</strong> '
                f'{html.escape(session_prose["session_total"])}'
                f'</div>'
            )
        if session_prose.get("uncommitted_milestone"):
            val = session_prose["uncommitted_milestone"]
            if len(val) > 400:
                val = val[:400].rstrip() + "..."
            parts.append(
                f'<div class="chassis-session-line">'
                f'<strong>Uncommitted:</strong> {html.escape(val)}'
                f'</div>'
            )
        for line in session_prose.get("in_progress_lines", []):
            if len(line) > 400:
                line = line[:400].rstrip() + "..."
            parts.append(
                f'<div class="chassis-session-line">'
                f'<strong>In progress:</strong> {html.escape(line)}'
                f'</div>'
            )
        parts.append('</div>')

    # Three-lane per-class render: full reach, in progress, untouched.
    # The headline progress card enumerates the FULL-REACH count; the in-progress
    # and untouched lanes enumerate the actual remaining scope so the operator
    # can see the full v0.6 work at a glance, not just the done part.
    full_classes = [c for c in classes if c["state"] == "full"]
    in_progress_lane = [c for c in classes if c["state"] == "in-progress"]
    untouched_lane = [c for c in classes if c["state"] == "untouched"]

    def render_class_card(cid, book, state_label, css_class):
        return (
            f'<div class="chassis-card {css_class}">'
            f'<div class="chassis-class">{html.escape(cid)} '
            f'<span class="chassis-source"> ({html.escape(book)})</span></div>'
            f'<div class="chassis-support">{html.escape(state_label)}</div>'
            f'</div>'
        )

    parts.append(
        '<h3 class="chassis-subhead">Full reach (reach Computed) '
        f'<span class="chassis-subhead-meta">{len(full_classes)} of {len(classes)} classes '
        '(canonical source: latest milestone in SWARM_STATUS.md)</span></h3>'
    )
    parts.append('<div class="chassis-grid">')
    for c in full_classes:
        parts.append(render_class_card(c["class_id"], c.get("book", "?"), "Full reach", "support-full"))
    parts.append('</div>')

    parts.append(
        '<h3 class="chassis-subhead">In progress (committed, not at full reach) '
        f'<span class="chassis-subhead-meta">{len(in_progress_lane)} of {len(classes)} classes '
        '(sourced from <class> COMMITTED / MILESTONE (uncommitted) lines)</span></h3>'
    )
    if in_progress_lane:
        parts.append('<div class="chassis-grid">')
        for c in in_progress_lane:
            parts.append(render_class_card(c["class_id"], c.get("book", "?"), "Partial engine progress", "support-partial"))
        parts.append('</div>')
    else:
        parts.append('<div class="chassis-grid"><div class="chassis-card">'
                     '<div class="chassis-support">no in-progress classes surfaced</div>'
                     '</div></div>')

    parts.append(
        '<h3 class="chassis-subhead">Untouched (no class-specific work yet) '
        f'<span class="chassis-subhead-meta">{len(untouched_lane)} of {len(classes)} classes '
        '(canonical source: canonical roster minus full-reach minus in-progress)</span></h3>'
    )
    if untouched_lane:
        parts.append('<div class="chassis-grid">')
        for c in untouched_lane:
            parts.append(render_class_card(c["class_id"], c.get("book", "?"), "Untouched", "support-unknown"))
        parts.append('</div>')
    else:
        parts.append('<div class="chassis-grid"><div class="chassis-card">'
                     '<div class="chassis-support">all classes accounted for</div>'
                     '</div></div>')

    parts.append(
        '<h3 class="chassis-subhead">Race coverage '
        '<span class="chassis-subhead-meta">(7 races; race-level support is '
        'not milestone-graded — the CRB Core 7 chassis covers all 7 by '
        'class-level reach)</span></h3>'
    )
    parts.append('<div class="chassis-grid">')
    for r in races:
        rid = r["race_id"]
        status = r["status"]
        cls = "support-full" if status == "fully-supported" else \
              "support-partial" if status == "human-only" else \
              "support-diagnostics-only"
        parts.append(
            f'<div class="chassis-card {cls}">'
            f'<div class="chassis-class">{html.escape(rid)}</div>'
            f'<div class="chassis-support">{html.escape(status)}</div>'
            f'</div>'
        )
    parts.append('</div>')

    return "\n".join(parts)


def render_chassis_breadth_live(progress: dict, classes: list, races: list,
                              session_prose: dict = None) -> str:
    """Render the chassis breadth as live-text lines."""
    if session_prose is None:
        session_prose = {}
    lines = ["[CHASSIS BREADTH]"]
    if progress.get("reached") is not None:
        lines.append(f"  classes reach Computed: {progress['reached']} / {progress['total']}")
    if session_prose.get("session_total"):
        st = session_prose["session_total"]
        if len(st) > 200:
            st = st[:200].rstrip() + "..."
        lines.append(f"  session total: {st}")
    if session_prose.get("uncommitted_milestone"):
        um = session_prose["uncommitted_milestone"]
        if len(um) > 200:
            um = um[:200].rstrip() + "..."
        lines.append(f"  uncommitted: {um}")
    for line in session_prose.get("in_progress_lines", []):
        if len(line) > 200:
            line = line[:200].rstrip() + "..."
        lines.append(f"  in progress: {line}")
    for c in classes:
        book = c.get("book", "?")
        state = c.get("state", "?")
        lines.append(f"  class {c['class_id']:<11} {book:<3} {state}")
    for r in races:
        lines.append(f"  race {r['race_id']:<11} {r['status']}")
    return "\n".join(lines)


def render_agent_strip_html(agents):
    """Render the 4-agent status strip as a flex of cards at the top."""
    cards = []
    status_class = {"Running": "agent-running",
                    "Idle": "agent-idle",
                    "Waiting": "agent-waiting"}
    for a in agents:
        snippet = a.get("snippet", "")
        if snippet:
            snippet_short = (snippet[:160] + "...") if len(snippet) > 160 else snippet
        else:
            snippet_short = "(no recent snippet in (a) Happening now)"
        cards.append(
            f'<div class="agent-card {status_class.get(a["status"], "")}">'
            f'<div class="agent-name">{html.escape(a["agent"])}</div>'
            f'<div class="agent-status">{html.escape(a["status"])}</div>'
            f'<div class="agent-snippet">{html.escape(snippet_short)}</div>'
            "</div>"
        )
    return (
        '<h2>Agents</h2>'
        '<div class="agent-strip">'
        + "".join(cards) +
        "</div>"
    )


def render_agent_strip_live(agents):
    out = ["[AGENTS]"]
    for a in agents:
        snippet = a.get("snippet", "")
        if snippet:
            snippet = (snippet[:200] + "...") if len(snippet) > 200 else snippet
        out.append(f"  {a['agent']:<13} {a['status']:<9} {snippet}")
    return "\n".join(out)


# ---------------------------------------------------------------------------
# Mailbox / task-list readers (kept for compatibility)
# ---------------------------------------------------------------------------


def safe_read_json(path):
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
        if not text.strip():
            return None
        return json.loads(text)
    except (json.JSONDecodeError, OSError):
        return None


def read_tasks(pattern):
    counts = {"pending": 0, "in_progress": 0, "completed": 0, "blocked": 0, "other": 0}
    for path in glob.glob(pattern):
        data = safe_read_json(pathlib.Path(path))
        if not isinstance(data, list):
            continue
        for task in data:
            if not isinstance(task, dict):
                continue
            status = str(task.get("status", "other")).lower().replace("-", "_").replace(" ", "_")
            if status in counts:
                counts[status] += 1
            else:
                counts["other"] += 1
    return counts


# ---------------------------------------------------------------------------
# Claude Code /usage cache reader (ground-truth subscription usage)
# ---------------------------------------------------------------------------


def read_usage(cache_path):
    """Read the /usage cache produced by /usr/local/sbin/usage-wrapper.sh."""
    p = pathlib.Path(cache_path)
    if not p.exists() or not p.is_file():
        return {}
    out = {}
    try:
        for line in p.read_text(encoding="utf-8", errors="replace").splitlines():
            line = line.strip()
            if not line or "=" not in line:
                continue
            k, _, v = line.partition("=")
            out[k.strip()] = v.strip()
    except OSError:
        return {}
    return out


def render_usage_html(usage):
    """Render the Claude Code /usage block: three gauges only."""
    if not usage:
        return (
            "<h2>Claude Code usage (ground truth)</h2>"
            f"<p class=\"meta\">[No /usage cache found at "
            f"<code>{html.escape(DEFAULT_USAGE_CACHE)}</code>. The wrapper at "
            f"<code>/usr/local/sbin/usage-wrapper.sh</code> runs every 15 min via "
            f"crontab; capture completes on next tick. Run manually with "
            f"<code>claude -p \"/usage\"</code>.]</p>"
        )
    cap_at = usage.get("captured_at", "?")
    return (
        "<h2>Claude Code usage (ground truth)</h2>"
        "<table class=\"usage-table\">"
        "<tr><th>gauge</th><th>value</th></tr>"
        "<tr>"
        f"<td><strong>Current session</strong></td>"
        f"<td><strong>{html.escape(usage.get('session_used_pct', '?'))}%</strong> &middot; reset {html.escape(usage.get('session_reset', '?').strip())}</td>"
        "</tr>"
        "<tr>"
        f"<td><strong>Current week (all models)</strong></td>"
        f"<td><strong>{html.escape(usage.get('week_all_models_used_pct', '?'))}%</strong> &middot; reset {html.escape(usage.get('week_all_models_reset', '?').strip())}</td>"
        "</tr>"
        "<tr>"
        f"<td><strong>Current week (Fable)</strong></td>"
        f"<td><strong>{html.escape(usage.get('fable_used_pct', '?'))}%</strong></td>"
        "</tr>"
        "</table>"
        f"<p class=\"meta\"><strong>Source:</strong> <code>claude -p \"/usage\"</code> via "
        f"<code>/usr/local/sbin/usage-wrapper.sh</code> (15-min cadence). "
        f"<strong>Captured:</strong> {html.escape(cap_at)}.</p>"
    )


def render_usage_live(usage):
    if not usage:
        return (
            "[CLAUDE CODE USAGE]\n"
            f"(no /usage cache at {DEFAULT_USAGE_CACHE})"
        )
    cap_at = usage.get("captured_at", "?")
    return (
        "== CLAUDE CODE USAGE (ground truth) ==\n"
        f"  captured_at         : {cap_at}\n"
        f"  current session     : {usage.get('session_used_pct', '?')}% used &middot; reset {usage.get('session_reset', '?')}\n"
        f"  current week (all)  : {usage.get('week_all_models_used_pct', '?')}% used &middot; reset {usage.get('week_all_models_reset', '?')}\n"
        f"  current week (Fable): {usage.get('fable_used_pct', '?')}% used\n"
    )


# ---------------------------------------------------------------------------
# SWARM_TASKS.md sidecar parser
# ---------------------------------------------------------------------------


def parse_tasks(sidecar_path):
    p = pathlib.Path(sidecar_path)
    if not p.exists():
        return []
    tasks = []
    header_seen = False
    in_table = False
    for raw_line in p.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw_line.strip()
        if not line:
            continue
        if line.startswith("#"):
            in_table = False
            header_seen = False
            continue
        if "|" not in line:
            continue
        cells = [c.strip() for c in line.split("|")]
        if cells and cells[0] == "":
            cells = cells[1:]
        if cells and cells[-1] == "":
            cells = cells[:-1]
        if len(cells) < 4:
            continue
        lowered = [c.lower() for c in cells[:4]]
        if not header_seen and lowered[:4] == TASK_COLUMNS:
            header_seen = True
            in_table = True
            continue
        if all(set(c) <= set("-: ") and len(c) > 0 for c in cells[:4]):
            continue
        if not in_table:
            continue
        task = cells[0]
        su = (cells[1] or "?").strip().upper()
        owner = su if su in VALID_OWNERS else cells[1] or "?"
        rulebook = (cells[2] or "-").strip() or "-"
        s = (cells[3] or "?").strip().lower().replace("-", " ").replace("_", " ")
        if s == "in progress":
            status = "in progress"
        elif s in VALID_STATUSES:
            status = s
        else:
            status = cells[3] or "?"
        last_update = cells[4].strip() if len(cells) >= 5 else "unknown"
        tasks.append({
            "task": task,
            "owner": owner,
            "rulebook": rulebook,
            "status": status,
            "last_update": last_update,
        })
    return tasks


# ---------------------------------------------------------------------------
# Operator-decision queue
# ---------------------------------------------------------------------------


# Fix 2 (2026-07-28): the old status-label rule was
# `re.match(r'^([^-]+?)\s*[\-:]', title)` with a fallback of the WHOLE title.
# Two things went wrong with it. Titles in this document separate their status
# label from their description with an em dash (U+2014), not an ASCII hyphen,
# so the pattern never matched the real separator -- it either found nothing
# (and fell back to the entire title, which the HTML then rendered a second
# time as the status badge, so every such row read its own title twice), or it
# matched an incidental hyphen inside a word and produced a fragment
# ("Alpha-bar distance — SHARPENED" yielded the label "Alpha").
#
# The document's actual convention is an ALL-CAPS status phrase at the very
# start of the title ("RESOLVED — ...", "RULING REVERSED (...) — ...",
# "CORRECTNESS BUG, live on a Computed class — ..."). Taking exactly that
# leading run gives a real badge when there is one and an empty badge when
# there is not, and can never echo the title back. Any parenthetical
# qualifier (attribution, date) is deliberately left out: it is noise in a
# badge, and the category already carries the autonomous/resolved
# distinction. The 4-character floor keeps an incidental leading acronym
# ("AC Bonus — ...") from being mistaken for a status.
_STATUS_LABEL_RE = re.compile(r'^(?P<label>[A-Z][A-Z0-9]*(?:[ ,\-/][A-Z0-9]+)*)')


def _status_label_for_title(title):
    """The ALL-CAPS status phrase a title opens with, or "" if it has none."""
    m = _STATUS_LABEL_RE.match(title.strip())
    if not m:
        return ""
    label = m.group("label").strip().rstrip(",-/")
    return label if len(label) >= 4 else ""


def parse_operator_decisions(risks_doc_path):
    """Pull items from risks-and-open-questions.md and categorize each.

    Categories: decision, autonomous, context, resolved.

    Items in `decision` and `context` lanes are marked `open=True` and
    rendered with their full body. Items in `autonomous` and `resolved`
    lanes get a truncated excerpt (the full body is preserved on disk in
    the dict for the user to inspect).
    """
    p = pathlib.Path(risks_doc_path)
    if not p.exists():
        return []

    DECISION_TITLE_PREFIXES = (
        "MAJOR ARCHITECTURE FINDING",
        "STILL UNRESOLVED",
        # A question addressed TO the operator is a pending operator call, so
        # it belongs in the decisions lane, not the general questions lane.
        # ("OPERATOR DECISION", below, is the already-made one.)
        "OPERATOR QUESTION",
        "operator call, not an engineering one",
        "deferred to operator",
        "delegated to operator",
    )
    AUTONOMOUS_TITLE_PREFIXES = (
        "STANDING GUIDANCE",
        "RESOLVED (autonomous",
    )
    # Fix 3 (2026-07-28): this used to be ("RESOLVED", "PARTIALLY RESOLVED")
    # only, so roughly 25 items that this document genuinely settles -- with
    # RULING, OPERATOR DECISION, GREENLIT, INCIDENT and friends -- fell
    # through to the `context` default and rendered as still-open. The list
    # below is the vocabulary the document is actually written in, read off
    # risks-and-open-questions.md itself rather than assumed.
    RESOLVED_TITLE_PREFIXES = (
        "RESOLVED",
        "PARTIALLY RESOLVED",
        "RULING REVERSED",
        "RULING",
        "OPERATOR DECISION",
        "OPERATOR-EQUIVALENT DECISION",
        "GREENLIT",
        "INCIDENT",
        "CORRECTED",
        "CORRECTED SCOPE",
        "METHODOLOGY CORRECTION",
        "CHECKPOINT FINDING",
    )
    # Titles that carry one of these labels are still genuinely open, and must
    # never be swallowed by a prefix above (none currently collide, but this
    # keeps the intent explicit if the vocabulary grows).
    OPEN_TITLE_PREFIXES = (
        "OPEN",
        "NEW FINDING",
        "ENVIRONMENT NOTE",
        "CORRECTNESS BUG",
    )

    BODY_OPERATOR_PATTERNS = (
        "Operator-only",
        "operator call",
        "not an engineering call",
        "Genuinely",
    )

    text = p.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines()

    items_by_id = {}
    section = None
    cur = None
    para = []

    def flush_para():
        if not para or cur is None:
            para.clear()
            return
        body = " ".join(para).strip()
        if cur.get("body"):
            cur["body"] = cur["body"] + " " + body
        else:
            cur["body"] = body
        para.clear()

    item_re = re.compile(r'^(?P<id>\d+)\.\s+\*\*(?P<title>.*?)\*\*')

    for line in lines:
        s = line.strip()
        if s.startswith("## "):
            flush_para()
            section = s[3:].strip()
            if cur is not None:
                items_by_id.setdefault((section, cur["item_id"]), cur)
                cur = None
            continue
        if section not in ("Risks", "Open questions"):
            continue
        m = item_re.match(s)
        if m:
            flush_para()
            if cur is not None:
                items_by_id.setdefault((section, cur["item_id"]), cur)
            title = m.group("title").strip()
            cur = {
                "section": section,
                "item_id": int(m.group("id")),
                "title": title,
                "status_label": _status_label_for_title(title),
                "body": "",
                "call_text": "",
            }
            # Fix 1 (2026-07-28): this used to `continue` straight past the
            # rest of the line, silently dropping the body of every item
            # written on ONE line (e.g. risks item 4, "Unsigned Windows
            # installers.", whose entire body -- the SmartScreen/code-signing
            # rationale -- lives after the closing `**` on the same line and
            # never reached the JSON at all). Whatever trails the title is
            # this item's first paragraph, exactly as if it had been on the
            # following line.
            trailing = s[m.end():].strip()
            if trailing:
                para.append(trailing)
            continue
        if cur is None:
            continue
        if not s:
            flush_para()
            continue
        para.append(s)
    flush_para()
    if cur is not None:
        items_by_id.setdefault((section, cur["item_id"]), cur)

    out = []
    for key, d in items_by_id.items():
        title_low = d["title"].lower()
        category = "context"
        # AUTONOMOUS is tested before RESOLVED: "RESOLVED" is a prefix of
        # "RESOLVED (autonomous", so the old RESOLVED-first ordering made the
        # AUTONOMOUS_TITLE_PREFIXES entry dead code that could never match.
        if any(title_low.startswith(p.lower()) for p in AUTONOMOUS_TITLE_PREFIXES):
            category = "autonomous"
        elif any(title_low.startswith(p.lower()) for p in DECISION_TITLE_PREFIXES):
            category = "decision"
        elif any(title_low.startswith(p.lower()) for p in OPEN_TITLE_PREFIXES):
            category = "context"
        elif any(title_low.startswith(p.lower()) for p in RESOLVED_TITLE_PREFIXES):
            category = "resolved"

        # An item is "open" if it's a real decision or context. The full
        # body is shown for open items; resolved/autonomous get excerpts.
        is_open = category in ("decision", "context")

        call_text = ""
        if d["body"]:
            sentences = re.split(r'(?<=\.)\s+', d["body"])
            for sent in sentences:
                sl = sent.lower()
                if any(p.lower() in sl for p in BODY_OPERATOR_PATTERNS):
                    call_text = sent.strip()
                    break
            if not call_text:
                for sent in sentences:
                    if "operator" in sent.lower():
                        call_text = sent.strip()
                        break

        out.append({
            "section": d["section"],
            "item_id": d["item_id"],
            "title": d["title"],
            "status_label": d["status_label"],
            "category": category,
            "body": d["body"],
            "call_text": call_text,
            "is_open": is_open,
        })

    section_order = {"Risks": 0, "Open questions": 1}
    category_order = {"decision": 0, "context": 1, "autonomous": 2, "resolved": 3}
    out.sort(key=lambda d: (
        category_order.get(d["category"], 9),
        section_order.get(d["section"], 9),
        d["item_id"],
    ))
    return out


def render_operator_decisions_html(decisions, source_path):
    """Render operator-decision queue grouped by category.

    Open items (decision / context) show their full body verbatim.
    Resolved / autonomous items show a truncated excerpt.
    """
    if not decisions:
        return (
            '<h2>Operator decisions pending</h2>'
            f'<p class="meta">[No items surfaced from '
            f'<code>{html.escape(source_path)}</code>.]</p>'
        )

    by_cat = {"decision": [], "context": [], "autonomous": [], "resolved": []}
    for d in decisions:
        by_cat.setdefault(d.get("category", "context"), []).append(d)

    headers = {
        "decision": ("Decisions awaiting operator sign-off",
                     "Titles or body text contain the explicit operator-call signal."),
        "context": ("Related context (no standalone decision)",
                    "Body mentions operator-handling but does not itself demand a call."),
        "autonomous": ("Lead-made autonomous decisions",
                       "Lead acted on operator's behalf under autonomous-operations directive."),
        "resolved": ("Resolved this session",
                     "For context only."),
    }

    def render_item(d, dim=False, full=False):
        section_slug = d["section"].lower().replace(" ", "-")
        id_label = f"{section_slug}-#{d['item_id']}"
        body_text = d.get("body", "")
        if not full and body_text:
            body_text = body_text[:500] + ("..." if len(d.get("body", "")) > 500 else "")
        if not body_text:
            body_text = "[no body]"
        cls = "decision-block"
        if dim:
            cls += " decision-block-dim"
        if full:
            cls += " decision-block-open"
        body_html = "<br>".join(html.escape(body_text).split("\n"))
        return (
            f'<div class="{cls}">'
            f'<div class="decision-header">'
            f'<span class="decision-id">{html.escape(id_label)}</span>'
            f'<span class="decision-title">{html.escape(d["title"])}</span>'
            f'<span class="decision-status decision-status-{html.escape(section_slug)}">{html.escape(d["status_label"])}</span>'
            "</div>"
            f'<div class="decision-body">{body_html}</div>'
            "</div>"
        )

    parts = ['<h2>Operator decisions pending</h2>']
    open_count = sum(1 for d in decisions if d.get("is_open"))
    parts.append(
        f'<p class="meta"><strong>Source:</strong> <code>{html.escape(source_path)}</code> '
        f'&middot; <strong>{open_count}</strong> open item(s) shown in full below. '
        f'Categories: '
        f'<strong>{len(by_cat["decision"])}</strong> awaiting decision, '
        f'<strong>{len(by_cat["context"])}</strong> context, '
        f'<strong>{len(by_cat["autonomous"])}</strong> autonomous, '
        f'<strong>{len(by_cat["resolved"])}</strong> resolved. '
        f'Categorized from title prefix and body-level operator-call patterns.</p>'
    )

    for cat in ("decision", "context"):
        items = by_cat.get(cat, [])
        if not items:
            continue
        title, subtitle = headers[cat]
        parts.append(f'<h3 class="decisions-section-title" data-category="{cat}">{html.escape(title)} &middot; {len(items)}</h3>')
        parts.append(f'<p class="meta">{html.escape(subtitle)}</p>')
        for d in items:
            parts.append(render_item(d, dim=False, full=True))

    return "\n".join(parts)


def render_operator_decisions_live(decisions, source_path):
    if not decisions:
        return (
            "[OPERATOR DECISIONS PENDING]\n"
            f"(no items surfaced from {source_path})"
        )
    out = [
        "[OPERATOR DECISIONS PENDING]",
        f"source: {source_path}",
        "",
    ]
    open_count = sum(1 for d in decisions if d.get("is_open"))
    out.append(f"  open={open_count}  total={len(decisions)}")
    for cat in ("decision", "context", "autonomous", "resolved"):
        items = [d for d in decisions if d.get("category") == cat]
        if not items:
            continue
        out.append(f"--- {cat.upper()} ({len(items)}) ---")
        for d in items:
            section_slug = d["section"].lower().replace(" ", "-")
            id_label = f"{section_slug}-#{d['item_id']}"
            body = d.get("body", "")
            full = d.get("is_open")
            if not full and body and len(body) > 500:
                body = body[:497] + "..."
            out.append(f"  {id_label}  [{d['status_label']}]  {d['title']}")
            if body:
                # One line per body sentence
                for sent in re.split(r'(?<=\.)\s+', body):
                    sent = sent.strip()
                    if not sent:
                        continue
                    if len(sent) > 220:
                        sent = sent[:217] + "..."
                    out.append(f"    {sent}")
        out.append("")
    return "\n".join(out)


# ---------------------------------------------------------------------------
# Task-table renderer
# ---------------------------------------------------------------------------

STATUS_CLASS = {
    "done": "status-done",
    "in progress": "status-in-progress",
    "queued": "status-queued",
    "blocked": "status-blocked",
}


def render_task_table_html(tasks, sidecar_path):
    if not tasks:
        return (
            '<h2>Task table</h2>'
            f'<p class="meta">[No tasks found in sidecar '
            f'<code>{html.escape(sidecar_path)}</code>. '
            f'Author a markdown table with columns '
            f'<code>task | owner | rulebook | status</code> to populate.]</p>'
        )
    rows = []
    for t in tasks:
        status = t.get("status", "?")
        cls = STATUS_CLASS.get(status, "")
        rows.append(
            '<tr class="task-row">'
            f'<td class="cell-task">{html.escape(t.get("task", ""))}</td>'
            f'<td class="cell-owner">{html.escape(t.get("owner", "?"))}</td>'
            f'<td class="cell-rulebook">{html.escape(t.get("rulebook", "-"))}</td>'
            f'<td class="cell-status {cls}">{html.escape(status)}</td>'
            "</tr>"
        )
    return (
        '<h2>Task table</h2>'
        '<p class="meta">Source: <code>'
        f'{html.escape(sidecar_path)}</code> &middot; '
        f'{len(tasks)} task rows.</p>'
        '<table class="task-table">'
        '<thead><tr>'
        '<th class="cell-task">task</th>'
        '<th class="cell-owner">owner</th>'
        '<th class="cell-rulebook">rulebook</th>'
        '<th class="cell-status">status</th>'
        '</tr></thead>'
        '<tbody>' + "".join(rows) + '</tbody>'
        '</table>'
    )


def render_task_table_live(tasks, sidecar_path):
    if not tasks:
        return (
            "[TASK TABLE]\n"
            f"(no tasks found in {sidecar_path})"
        )
    out = ["[TASK TABLE]", ""]
    for t in tasks:
        out.append(
            f"  [{t.get('status', '?'):<11}] "
            f"{t.get('owner', '?'):<8} "
            f"{t.get('rulebook', '-'):<4} "
            f"{t.get('task', '?')}"
        )
    out.append("")
    out.append(f"({len(tasks)} rows; source: {sidecar_path})")
    return "\n".join(out)


# ---------------------------------------------------------------------------
# Resolved-decisions renderer (collapsed by default)
# ---------------------------------------------------------------------------


def render_resolved_decisions_html(decisions, source_path):
    by_cat = {"decision": [], "context": [], "autonomous": [], "resolved": []}
    for d in decisions:
        by_cat.setdefault(d.get("category", "context"), []).append(d)
    rows = []
    rows.append('<details open="false">')
    rows.append('<summary><strong>Resolved this session</strong> &middot; '
                f'{len(by_cat["resolved"])} item(s) &middot; click to expand</summary>')
    rows.append('<table class="decisions-table">')
    rows.append('<thead><tr><th class="cell-decision-id">id</th>'
                '<th class="cell-decision-title">title</th>'
                '<th class="cell-decision-status">status</th></tr></thead>')
    rows.append('<tbody>')
    for d in by_cat["resolved"]:
        section_slug = d["section"].lower().replace(" ", "-")
        id_label = f"{section_slug}-#{d['item_id']}"
        rows.append('<tr class="decision-row decision-row-dim">'
                    f'<td class="cell-decision-id">{html.escape(id_label)}</td>'
                    f'<td class="cell-decision-title">{html.escape(d["title"])}</td>'
                    f'<td class="cell-decision-status decision-status-{html.escape(section_slug)}">{html.escape(d["status_label"])}</td>'
                    "</tr>")
    rows.append('</tbody></table>')
    rows.append('</details>')
    return "\n".join(rows)


# ---------------------------------------------------------------------------
# HTML template (single source of truth)
# ---------------------------------------------------------------------------

PAGE_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Release-Swarm Dashboard &mdash; v0.6 alpha</title>
<style>
  :root {{
    --fg: #e8e8ea;
    --bg: #11131a;
    --muted: #8a8f9c;
    --accent: #6ab7ff;
    --border: #2a2e3a;
  }}
  body {{ font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
        background: var(--bg); color: var(--fg); margin: 1.5em auto;
        max-width: 1100px; line-height: 1.45; }}
  h1 {{ color: var(--accent); font-size: 1.4em; margin-bottom: .25em; }}
  h2 {{ color: var(--accent); border-bottom: 1px solid var(--border);
       padding-bottom: .25em; margin-top: 1.5em; font-size: 1.1em; }}
  h3 {{ color: var(--muted); font-size: 1em; margin-top: 1em; }}
  pre, .body {{ background: #181a23; border: 1px solid var(--border);
              padding: .75em; border-radius: 6px; white-space: pre-wrap;
              word-break: break-word; }}
  table {{ border-collapse: collapse; width: 100%; margin: .5em 0; }}
  th, td {{ text-align: left; padding: 4px 8px; border-bottom: 1px solid var(--border);
          vertical-align: top; }}
  th {{ color: var(--accent); font-weight: 600; }}
  .meta {{ color: var(--muted); font-size: .9em; }}
  .pill {{ display: inline-block; padding: 1px 6px; border-radius: 8px;
          background: #233; color: var(--accent); font-size: .85em; }}
  .footer {{ color: var(--muted); font-size: .85em; margin-top: 3em;
            border-top: 1px solid var(--border); padding-top: .75em; }}

  /* Agent strip */
  .agent-strip {{ display: flex; flex-wrap: wrap; gap: 0.75em; margin: 0.5em 0 1em; }}
  .agent-card {{ flex: 1 1 200px; min-width: 200px;
                border: 1px solid var(--border); border-radius: 6px;
                padding: 0.75em; background: #181a23; }}
  .agent-card.agent-running {{ border-left: 3px solid #6abf7b; }}
  .agent-card.agent-idle    {{ border-left: 3px solid #8a8f9c; }}
  .agent-card.agent-waiting {{ border-left: 3px solid #d2a64a; }}
  .agent-name {{ color: var(--accent); font-weight: 600; }}
  .agent-status {{ font-size: .9em; margin: .25em 0; }}
  .agent-snippet {{ color: var(--muted); font-size: .85em; }}

  /* Task table */
  table.task-table {{ table-layout: fixed; }}
  .task-table .cell-task     {{ width: auto; }}
  .task-table .cell-owner    {{ width: 90px; }}
  .task-table .cell-rulebook {{ width: 90px; text-align: center; }}
  .task-table .cell-status   {{ width: 110px; }}
  .task-table .status-done       {{ color: #6abf7b; }}
  .task-table .status-in-progress{{ color: #d2a64a; }}
  .task-table .status-queued     {{ color: var(--muted); }}
  .task-table .status-blocked    {{ color: #d36a6a; }}

  /* Operator-decision blocks (open items show full body) */
  .decision-block {{ border: 1px solid var(--border); border-radius: 6px;
                    margin: .5em 0; padding: .75em; background: #181a23; }}
  .decision-block-open {{ border-left: 3px solid #d2a64a; }}
  .decision-block-dim {{ opacity: 0.7; }}
  .decision-header {{ display: flex; gap: 1em; flex-wrap: wrap;
                     margin-bottom: .5em; font-weight: 600; }}
  .decision-id {{ color: var(--muted); font-family: monospace; min-width: 130px; }}
  .decision-status {{ padding: 1px 6px; border-radius: 8px;
                     background: #233; font-size: .85em; font-weight: 600; }}
  .decision-status-risks         {{ color: #d2a64a; }}
  .decision-status-open-questions {{ color: var(--muted); }}
  .decision-body {{ color: var(--fg); white-space: pre-wrap;
                   font-size: .92em; line-height: 1.45; }}

  /* Resolved-decisions table inside <details> */
  .decisions-table {{ table-layout: fixed; }}
  .decisions-table .cell-decision-id      {{ width: 130px; }}
  .decisions-table .cell-decision-title   {{ width: auto; }}
  .decisions-table .cell-decision-status  {{ width: 140px; }}
  .decisions-table .decision-row-dim td   {{ color: var(--muted); }}

  /* <details> styling */
  details {{ margin: 0.5em 0; }}
  details summary {{ cursor: pointer; padding: .25em 0; color: var(--accent); }}
  details summary::marker {{ color: var(--muted); }}
  details[open] summary {{ border-bottom: 1px solid var(--border); margin-bottom: .5em; }}

  /* Chassis breadth */
  .chassis-card {{ border: 1px solid var(--border); border-radius: 6px;
                 padding: 0.5em 0.75em; background: #181a23; }}
  .chassis-progress {{ margin: 0.5em 0 1em; padding: 1em 1.25em;
                      border-left: 3px solid #6abf7b; }}
  .chassis-progress.chassis-empty {{ border-left-color: var(--muted); opacity: 0.7; }}
  .chassis-num {{ font-size: 1.6em; font-weight: 700; color: var(--accent); }}
  .chassis-label {{ color: var(--fg); font-size: 0.95em; margin-top: 0.15em; }}
  .chassis-rest {{ color: var(--muted); font-size: 0.85em; margin-top: 0.35em;
                   white-space: pre-wrap; }}
  .chassis-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
                  gap: 0.5em; margin: 0.5em 0 1em; }}
  .chassis-card.support-full {{ border-left: 3px solid #6abf7b; }}
  .chassis-card.support-partial {{ border-left: 3px solid #d2a64a; }}
  .chassis-card.support-diagnostics-only {{ border-left: 3px solid #d36a6a; }}
  .chassis-card.support-unknown {{ border-left-color: var(--muted); opacity: 0.7; }}
  .chassis-class {{ color: var(--accent); font-weight: 600; text-transform: capitalize; }}
  .chassis-source {{ color: var(--muted); font-size: 0.75em; font-weight: normal;
                      margin-left: 0.4em; font-style: italic; }}
  .chassis-support {{ color: var(--muted); font-size: 0.85em; margin-top: 0.15em; }}
  .chassis-subhead {{ color: var(--accent); font-size: 0.9em; margin-top: 1.5em;
                       margin-bottom: 0.5em; border-bottom: 1px solid var(--border);
                       padding-bottom: 0.25em; font-weight: 600;
                       text-transform: uppercase; letter-spacing: 0.05em; }}
  .chassis-subhead-meta {{ color: var(--muted); font-size: 0.7em; font-weight: normal;
                            text-transform: none; letter-spacing: 0; margin-left: 0.5em; }}
  .chassis-session {{ margin: 0.5em 0 1em; padding: 1em 1.25em;
                       border-left: 3px solid #d4a04a;
                       background: #2a2218; border-radius: 4px; }}
  .chassis-session-title {{ color: var(--accent); font-size: 0.85em;
                              font-weight: 600; text-transform: uppercase;
                              letter-spacing: 0.05em; margin-bottom: 0.5em; }}
  .chassis-session-line {{ color: var(--fg); font-size: 0.9em;
                            margin: 0.4em 0; line-height: 1.4; }}
  .chassis-session-line strong {{ color: #d4a04a; font-weight: 600;
                                    margin-right: 0.4em; }}

  @media (max-width: 720px) {{
    .agent-strip {{ flex-direction: column; }}
    table.task-table thead, table.decisions-table thead {{ display: none; }}
    table.task-table, table.task-table tbody, table.task-table tr, table.task-table td {{
      display: block; width: 100%;
    }}
    table.task-table tr.task-row {{
      border: 1px solid var(--border); border-radius: 6px;
      margin-bottom: .5em; padding: .5em;
    }}
    table.task-table td {{ border-bottom: none; padding: 2px 0; }}
    table.task-table td.cell-task::before     {{ content: "task: "; color: var(--muted); }}
    table.task-table td.cell-owner::before    {{ content: "owner: "; color: var(--muted); }}
    table.task-table td.cell-rulebook::before {{ content: "book: "; color: var(--muted); }}
    table.task-table td.cell-status::before   {{ content: "status: "; color: var(--muted); }}
  }}
</style>
</head>
<body>
<h1>Release-Swarm Dashboard &mdash; v0.6 alpha</h1>
<p class="meta">Last refreshed <strong>{refreshed}</strong> &middot; manual refresh &middot;
   observer reads only; canonical source is <span class="pill">SWARM_STATUS.md</span> written by the lead.</p>

{usage_section}

{agent_strip}

{chassis_breadth}

{task_table}

{decisions_section}

{resolved_section}

<details open="false">
<summary><strong>Lead-authored free-form</strong> &middot; (a) Happening now / (b) Happened / (c) On deck &middot; click to expand</summary>

<h3>(a) Happening now</h3>
<pre class="body">{happening}</pre>
<h3>(b) Happened</h3>
<pre class="body">{happened}</pre>
<h3>(c) On deck</h3>
<pre class="body">{ondeck}</pre>

</details>

<p class="footer">
  Operator-side observer, off the swarm budget.
  Canonical source: <code>{status_path}</code> &mdash; written by the lead.
  Observer does NOT write to <code>SWARM_STATUS.md</code>, does NOT SendMessage any teammate,
  does NOT spawn.
</p>
</body>
</html>"""


# ---------------------------------------------------------------------------
# Renderers and entry point
# ---------------------------------------------------------------------------


def render_html(status, task_counts, status_path, refreshed,
                task_table, decisions_section, usage_section,
                agent_strip, resolved_section, chassis_breadth):
    # task_counts deliberately unused in the rendered HTML per the v0.6 dashboard
    # trimming directive: the canonical task list is the table itself; the
    # aggregate counts duplicated a derived view that drifted whenever the
    # sidecar's statuses disagreed on casing. Keep the parameter for the
    # signature contract but do not project it into the page.
    return PAGE_TEMPLATE.format(
        refreshed=refreshed.strftime("%Y-%m-%d %H:%M:%S UTC"),
        usage_section=usage_section,
        agent_strip=agent_strip,
        chassis_breadth=chassis_breadth,
        task_table=task_table,
        decisions_section=decisions_section,
        resolved_section=resolved_section,
        happening=html.escape(status.get("happening", "")),
        happened=html.escape(status.get("happened", "")),
        ondeck=html.escape(status.get("ondeck", "")),
        status_path=html.escape(status_path),
    )


def render_live_text(status, task_counts, refreshed,
                     task_live, decisions_live, usage_live, agent_live,
                     chassis_live):
    # task_counts intentionally unused in the live-text view for the same
    # reason documented in render_html: the per-row table is the canonical
    # view; aggregate counts are a derived projection that drifts.
    lines = []
    if usage_live:
        lines.append(usage_live)
        lines.append("")
    if agent_live:
        lines.append(agent_live)
        lines.append("")
    if chassis_live:
        lines.append(chassis_live)
        lines.append("")
    if task_live:
        lines.append(task_live)
        lines.append("")
    if decisions_live:
        lines.append(decisions_live)
        lines.append("")
    lines.append("[release-swarm live]")
    lines.append(f"refreshed: {refreshed.strftime('%Y-%m-%d %H:%M:%S UTC')}")
    return "\n".join(lines) + "\n"


def main() -> int:
    p = argparse.ArgumentParser(description="Release-swarm observer renderer.")
    p.add_argument("--status", default=os.environ.get("SWARM_STATUS", DEFAULT_STATUS))
    p.add_argument("--mailbox-glob", default=os.environ.get("SWARM_MAILBOX_GLOB", DEFAULT_MAILBOX_GLOB))
    p.add_argument("--task-glob", default=os.environ.get("SWARM_TASK_GLOB", DEFAULT_TASK_GLOB))
    p.add_argument("--out", default=os.environ.get("SWARM_OUT", DEFAULT_OUT))
    p.add_argument("--live", default=os.environ.get("SWARM_LIVE", DEFAULT_LIVE))
    p.add_argument("--tasks-sidecar", default=os.environ.get("SWARM_TASKS_SIDECAR", DEFAULT_TASKS_SIDECAR))
    p.add_argument("--risks-doc", default=os.environ.get("SWARM_RISKS_DOC", DEFAULT_RISKS_DOC))
    p.add_argument("--report", default=os.environ.get("SWARM_REPORT", DEFAULT_REPORT))
    p.add_argument("--usage-cache", default=os.environ.get("SWARM_USAGE_CACHE", DEFAULT_USAGE_CACHE))
    args = p.parse_args()

    refreshed = dt.datetime.now(dt.timezone.utc)
    status = read_status(args.status)
    task_counts = read_tasks(args.task_glob)
    usage = read_usage(args.usage_cache)
    decisions = parse_operator_decisions(args.risks_doc)
    tasks = parse_tasks(args.tasks_sidecar)
    agents = parse_agent_status(read_status_raw(args.status), status.get("happening", ""))
    status_raw = read_status_raw(args.status)
    report_raw = read_status_raw(args.report)
    chassis_progress = parse_class_breadth_progress(status_raw)
    # The live class support-level is sourced from SWARM_REPORT.md's own
    # structured per-class tables (authoritative; see
    # parse_class_report_table's own doc comment), falling back to
    # SWARM_STATUS.md's milestone prose only for classes the report doesn't
    # cover. The risks doc is only the fallback for races (race-level
    # support is not milestone-graded — that lives in the risks doc and
    # stays there).
    chassis_classes = parse_class_chassis_table(status_raw, report_raw)
    chassis_races = parse_race_chassis_table(read_status_raw(args.risks_doc))
    session_prose = parse_session_prose(status_raw)

    decisions_html = render_operator_decisions_html(decisions, args.risks_doc)
    decisions_live = render_operator_decisions_live(decisions, args.risks_doc)
    resolved_html = render_resolved_decisions_html(decisions, args.risks_doc)
    tasks_html = render_task_table_html(tasks, args.tasks_sidecar)
    tasks_live = render_task_table_live(tasks, args.tasks_sidecar)
    usage_html = render_usage_html(usage)
    usage_live = render_usage_live(usage)
    agents_html = render_agent_strip_html(agents)
    agents_live = render_agent_strip_live(agents)
    chassis_html = render_chassis_breadth_html(chassis_progress, chassis_classes, chassis_races,
                                         session_prose=session_prose)
    chassis_live = render_chassis_breadth_live(chassis_progress, chassis_classes, chassis_races,
                                         session_prose=session_prose)

    out_html = render_html(status, task_counts, args.status, refreshed,
                           tasks_html, decisions_html, usage_html,
                           agents_html, resolved_html, chassis_html)
    out_text = render_live_text(status, task_counts, refreshed,
                                tasks_live, decisions_live, usage_live, agents_live,
                                chassis_live)

    try:
        pathlib.Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        pathlib.Path(args.out).write_text(out_html, encoding="utf-8")
        pathlib.Path(args.live).parent.mkdir(parents=True, exist_ok=True)
        pathlib.Path(args.live).write_text(out_text, encoding="utf-8")
    except OSError as exc:
        print(f"observer: write failed: {exc}", file=sys.stderr)
        return 1
    print(f"observer: rendered {args.out} and {args.live}")
    return 0


if __name__ == "__main__":
    sys.exit(main())