#!/usr/bin/env python3
"""PF1e dashboard JSON producer.

Reads the same canonical sources as the v0.6 observer (SWARM_STATUS.md,
risks-and-open-questions.md, /usage cache, the wired-integration stubs
registry) and produces a single JSON file at ~/swarm-observer/
PF1e-dashboard.json. The HTML at PF1e-dashboard.html fetch()'s this file at
page-load.

The JSON is the canonical record. The HTML is a thin viewer.

Status vocabulary (4 states, per operator choice 2026-07-25):
  full        — class/race is in the latest reach-list
  in-progress — class has real partial engine progress committed but
                is NOT at full reach
  untouched   — no class-specific work yet
  unassigned  — work is known to be needed but no SD-N owns it yet

Workchannel vocabulary (auto-discovered from the wired-integration stubs
registry's `Remediation cycle: SD-27+ (unscheduled)` annotations and from
SWARM_STATUS.md's lead prose):
  v0.6   — v0.6 alpha (active)
  SD-27  — SD-27 (planning)
  SD-28, SD-29, ... — successor bundles, surfaced as their handoffs exist
  unassigned — items known to be needed but no owner

Stdlib only. Runs via hermes cron, watchdog, or manual tick.
"""
from __future__ import annotations

import datetime as dt
import json
import os
import pathlib
import re
import shutil
import sys
import tempfile

# Reuse observer.py parsers.
import importlib.util
# observer.py is always this file's sibling, so resolve it relative to __file__
# rather than an absolute home path — the hermes tree is checked out under a
# different home on some hosts and the literal path made the import unusable
# there. Every other input is already --flag/env overridable; this one was not.
_OBSERVER = pathlib.Path(__file__).resolve().parent / "observer.py"
_spec = importlib.util.spec_from_file_location("observer", _OBSERVER)
_observer = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_observer)

# Decision 12 (2026-08-17, `decisions.md`): the declared-PI oracle reader
# every public-feed name must pass through before it ships. Same
# resolve-relative-to-__file__ pattern as observer.py above.
_PI_REDACTION = pathlib.Path(__file__).resolve().parent / "pi_redaction.py"
_pi_spec = importlib.util.spec_from_file_location("pi_redaction", _PI_REDACTION)
pi_redaction = importlib.util.module_from_spec(_pi_spec)
_pi_spec.loader.exec_module(pi_redaction)

# FIX-DASHBOARD-PI (2026-08-17): the same shared, reviewed allow-list
# `scripts/site/build_public_status.py` already uses for the public status
# projection -- see `_PiScreen` below for why the dashboard feed needs it
# too (word-boundary matching is deliberately over-inclusive; this is the
# one place a mundane homonym is cleared, one entry at a time).
_PI_ALLOWLIST = pathlib.Path(__file__).resolve().parent.parent / "site" / "pi_substring_allowlist.py"
_pi_allowlist_spec = importlib.util.spec_from_file_location("pi_substring_allowlist", _PI_ALLOWLIST)
pi_substring_allowlist = importlib.util.module_from_spec(_pi_allowlist_spec)
_pi_allowlist_spec.loader.exec_module(pi_substring_allowlist)


# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

DEFAULT_STATUS = os.path.expanduser("~/workspace/repos/codex/docs/release/v0.6/SWARM_STATUS.md")
DEFAULT_RISKS_DOC = os.path.expanduser("~/workspace/repos/codex/docs/release/v0.6/risks-and-open-questions.md")
# Authoritative per-class state source (see observer.py's
# parse_class_report_table doc comment for why this takes priority over
# SWARM_STATUS.md's own prose-scraping heuristics).
DEFAULT_REPORT = os.path.expanduser("~/workspace/repos/codex/docs/release/v0.6/SWARM_REPORT.md")
DEFAULT_USAGE_CACHE = os.path.expanduser("~/swarm-observer/.usage-cache.txt")
DEFAULT_STUBS_REGISTRY = os.path.expanduser("~/workspace/repos/codex/docs/governance/wired-integration-stubs-registry.md")
# PF1E_JSON_PATH is the toolchain-wide override (the orchestrator helper reads
# the same var), so one env var retargets both the producer and the writer.
# PF1E_JSON_OUT stays supported and still wins for producer-only redirection.
DEFAULT_OUT = os.environ.get(
    "PF1E_JSON_PATH", os.path.expanduser("~/swarm-observer/PF1e-dashboard.json")
)

# ---------------------------------------------------------------------------
# Engine-derived class state (2026-07-28)
# ---------------------------------------------------------------------------
#
# The 27-class matrix used to be produced by regex-scraping hand-written
# English prose out of SWARM_REPORT.md (observer.parse_class_report_table and
# friends). That is the root cause of every complaint about this panel: the
# matrix went stale the moment somebody forgot to edit the prose, the
# per-class "open question" was chopped blind at 280 characters (routinely
# mid-commit-hash), and nothing about it could be trusted.
#
# The engine already knows the truth. `cargo run --bin v06_class_state_dump`
# builds a real headless receipt for all 27 classes through the actual
# compute pipeline and reports, per class, whether it reaches
# HeadlessReceiptStatus::Computed and which claim-blocking diagnostics are
# stopping it. Those diagnostics already name the specific remaining
# features, which is exactly the "what's left" this column wants.
#
# The prose scrape is kept only as a fallback for when the engine dump cannot
# be produced at all (no repo, no toolchain, build failure) -- so the panel
# degrades to its old behaviour instead of going blank.
DEFAULT_REPO_ROOT = os.environ.get(
    "CODEX_REPO_ROOT", os.path.expanduser("~/workspace/repos/codex")
)
DEFAULT_CLASS_STATE_CACHE = os.environ.get(
    "PF1E_CLASS_STATE_CACHE", os.path.expanduser("~/swarm-observer/class-state-dump.json")
)
# This producer is driven by a once-a-minute cron renderer. Shelling out to
# cargo every minute would contend with the swarm's own builds on a shared
# checkout, so the dump is cached and only rebuilt when it ages out.
CLASS_STATE_MAX_AGE_SECONDS = int(
    os.environ.get("PF1E_CLASS_STATE_MAX_AGE", "1800")
)
CLASS_STATE_BUILD_TIMEOUT_SECONDS = int(
    os.environ.get("PF1E_CLASS_STATE_TIMEOUT", "600")
)
# A private target dir keeps this refresh from fighting the swarm's agents
# over the shared checkout's target/ lock.
DEFAULT_CLASS_STATE_TARGET_DIR = os.environ.get(
    "PF1E_CLASS_STATE_TARGET_DIR", os.path.expanduser("~/swarm-observer/.class-state-target")
)

# ---------------------------------------------------------------------------
# Engine-derived content state (2026-07-29)
# ---------------------------------------------------------------------------
#
# Same disease, second outbreak. The operator looked at the dashboard on
# 2026-07-29 and reported "Bestiary looks to be not touched -- all 41 beasts
# show not started". That was false: `src/rules_core/rules_tables/beastiary1/`
# holds 41 real, resolvable `MonsterStatBlock` records. The dashboard's own
# payload was the broken thing -- it emitted 41 bare monster NAME STRINGS with
# no state field at all, so the viewer's per-item state lookup missed on every
# one of them and rendered the "no manifest entry" default, which reads as
# "Not started". A blank is not a measurement.
#
# The per-book 5-doneness matrix had the same shape of problem: `spells` and
# `equipment` were hand-set literals, so Bestiary 1 -- a book with no spell
# concept at all -- claimed a full green tick on spells, and every book
# inherited the CRB race aggregate whether or not it ingests a single race.
# The feats column carried a hand-typed "16 of 185" that had gone stale by
# more than a factor of three against a catalog that is now 486 records.
#
# `cargo run --bin v06_content_state_dump` is the cure, and it is the same
# cure `v06_class_state_dump` already applied to the 27-class matrix: it
# counts the real compiled tables, resolves every monster through the real
# `monster_resolve` entry point, runs every race through the real compute
# pipeline, and probes which catalog feats genuinely change a computed number.
# Nothing in this file re-types any of those numbers.
DEFAULT_CONTENT_STATE_CACHE = os.environ.get(
    "PF1E_CONTENT_STATE_CACHE", os.path.expanduser("~/swarm-observer/content-state-dump.json")
)
# Same cost discipline as the class dump: cron runs this producer often and the
# checkout is shared, so the dump is cached and only rebuilt when it ages out.
# Its own env var (rather than reusing the class one) so the two can be tuned
# apart -- the content dump's feat probe is ~10s of pure compute on top of the
# build, where the class dump's is ~0.
CONTENT_STATE_MAX_AGE_SECONDS = int(
    os.environ.get("PF1E_CONTENT_STATE_MAX_AGE", "1800")
)

# ---------------------------------------------------------------------------
# Engine-derived work inventory (2026-07-30)
# ---------------------------------------------------------------------------
#
# Third instance of the same disease, and the one that covers the other 21
# books. The class dump answers "which of the 27 classes work"; the content
# dump answers "what has each of the 4 ingested books got". Neither answers
# the operator's actual question during a multi-day run: **how much work is
# there in total, and how much of it is done?** Twenty-one of the twenty-five
# corpus books have never been read by any code, so nothing in this producer
# could see them at all -- they rendered as a roster of titles with no
# denominator behind them.
#
# `cargo run --bin v06_work_inventory` walks EVERY book in the corpus,
# including the ones the engine knows nothing about, enumerates each real
# record as a unit of work, and cross-references the engine for a status. A
# book nobody has started still contributes real, named units at
# `not-started`, which is what turns "21 books remaining" from a headline into
# a measured number.
#
# `--summary` is deliberate: the full document is ~19 MB of per-unit rows and
# lands at `docs/work-inventory.json` in the repo. Piping that through a
# subprocess on every cron tick would cost more than the compute it reports
# on, so the dashboard takes the aggregates and leaves the rows on disk.
DEFAULT_WORK_INVENTORY_CACHE = os.environ.get(
    "PF1E_WORK_INVENTORY_CACHE", os.path.expanduser("~/swarm-observer/work-inventory-summary.json")
)
# The inventory reads ~1,000 corpus files and runs the feat probe, so it is the
# most expensive of the three dumps. Its own env var, defaulting to an hour --
# corpus content changes when a book is ingested, which is a per-day event, not
# a per-minute one.
WORK_INVENTORY_MAX_AGE_SECONDS = int(
    os.environ.get("PF1E_WORK_INVENTORY_MAX_AGE", "3600")
)


# ---------------------------------------------------------------------------
# Canonical 25-book roster
# ---------------------------------------------------------------------------
# 4 in-scope books (SD-22 + Bestiary 1) with real JSON cache builds per
# SD-26 Epic 3; 21 future-state books registered as honest stubs per
# SD-26 Epic 4. The 21-book list is the lead's `wired-integration-stubs-registry`
# entries 0003-0023. Source: docs/governance/wired-integration-stubs-registry.md.

IN_SCOPE_BOOKS = [
    {"id": "core_rulebook", "title": "Core Rulebook", "channel": "v0.6"},
    {"id": "advanced_players_guide", "title": "Advanced Player's Guide", "channel": "v0.6"},
    {"id": "advanced_class_guide", "title": "Advanced Class Guide", "channel": "v0.6"},
    {"id": "bestiary_1", "title": "Bestiary 1", "channel": "v0.6"},
]

FUTURE_STATE_BOOKS = [
    # Each: id, title, channel (always "SD-27+ (unscheduled)" until operator
    # names a specific SD-N), 5-doneness markers (Races/Classes/Spells/Equipment/Feats).
    # The "X" / "O" / "✓" markers come from the lead's wired-integration registry
    # description for each book. The default state for future-state books is
    # "O" (out-of-scope, no ingest) on every kind.
    # `core_essentials` (2026-08-10): un-excluded from `work_inventory_panel`
    # above, so it must have a real book-roster entry too, or it would
    # report real content while appearing in no book list at all. No SD-N
    # channel -- it predates the SD numbering and is not owned by any
    # SD-bundle epic, so `channel: ""` is the honest value, not a guess at
    # one, and correctly falls to `status: "unassigned"` under the same
    # three-state logic every other FUTURE_STATE_BOOKS entry uses (no SD-N
    # channel at all is exactly what that status means). This may read as
    # underselling real, substantial landed content -- flagged rather than
    # silently accepted, since the vocabulary's own "unassigned" wording
    # ("work is known to be needed") does not fit a book with this much
    # already done. A future pass may want a fourth status or a channel
    # label for pre-SD foundational work; not invented here.
    # (`SD31-ATTRIB-001`, 2026-08-16: the "unique to this book" framing above
    # is now stale -- most of what was unique to `core_essentials` was a
    # mislabelled true-book attribution, not genuinely core_essentials-only
    # content; see `work_inventory_panel()`'s own doc comment. The row stays
    # here for the administrative/roadmap panel, but as of operator ruling
    # §16 (2026-08-19, wave 16) the residual is **0**: every remaining
    # `core_essentials`-labelled unit was either re-attributed to its true
    # book (12, Ghoran's own `ultimate_wilderness` declaration) or deleted
    # outright as a hallucination not found in print (116 -- the file's own
    # 23 pre-directive rows, 6 `SOURCELONG:Universal Rules` rows, Ghoran's
    # held-back duplicate `race` chassis row, and 86 units across the 7
    # remaining ambiguous/unattributable races). `decisions.md §9`'s
    # condition is discharged: `core_essentials` no longer appears as a key
    # in `docs/work-inventory.json`'s `books` map at all. This row is kept
    # here, at 0, as the historical record of the label's dissolution
    # (1,610 -> 644 -> 129 -> 128 -> 0) rather than deleted, so a future
    # regression that reintroduces the label is visible against a known-zero
    # baseline. The attribution contract gate
    # (`core_essentials_book_attribution_tests::
    # core_essentials_real_corpus_residual_never_grows_past_its_pinned_baseline`,
    # `v06_work_inventory.rs`) and `main()`'s own
    # `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING` assertion both still ratchet
    # the pre-deletion residual, so a regression is caught before this panel
    # would ever need to report a non-zero figure again. SD-32 card 15
    # (`decisions.md §12b`) raised the ratchet from 117 to 138 when
    # `Kind::Skill` made `core_essentials/ce_skills.lst`'s 21
    # previously-unenumerated, unattributable rows visible for the first
    # time -- see that constant's own doc comment for the re-derive command.)
    {"id": "core_essentials", "title": "Core Essentials", "channel": ""},
    {"id": "advanced_race_guide", "title": "Advanced Race Guide", "channel": "SD-27"},
    {"id": "pathfinder_unchained", "title": "Pathfinder Unchained", "channel": "SD-27"},
    {"id": "adventurers_guide", "title": "Adventurer's Guide", "channel": "SD-30"},
    {"id": "bestiary_2", "title": "Bestiary 2", "channel": "SD-29"},
    {"id": "bestiary_3", "title": "Bestiary 3", "channel": "SD-29"},
    {"id": "bestiary_4", "title": "Bestiary 4", "channel": "SD-29"},
    {"id": "bestiary_5", "title": "Bestiary 5", "channel": "SD-29"},
    {"id": "bestiary_6", "title": "Bestiary 6", "channel": "SD-29"},
    {"id": "bonus_bestiary", "title": "Bonus Bestiary", "channel": "SD-29"},
    {"id": "horror_adventures", "title": "Horror Adventures", "channel": "SD-30"},
    {"id": "monster_codex", "title": "Monster Codex", "channel": "SD-29"},
    {"id": "mythic_adventures", "title": "Mythic Adventures", "channel": "SD-30"},
    {"id": "occult_adventures", "title": "Occult Adventures", "channel": "SD-30"},
    {"id": "ultimate_campaign", "title": "Ultimate Campaign", "channel": "SD-28"},
    {"id": "ultimate_combat", "title": "Ultimate Combat", "channel": "SD-28"},
    {"id": "ultimate_equipment", "title": "Ultimate Equipment", "channel": "SD-28"},
    {"id": "ultimate_intrigue", "title": "Ultimate Intrigue", "channel": "SD-28"},
    {"id": "ultimate_magic", "title": "Ultimate Magic", "channel": "SD-28"},
    {"id": "ultimate_wilderness", "title": "Ultimate Wilderness", "channel": "SD-28"},
]


# ---------------------------------------------------------------------------
# Matrix doneness markers
# ---------------------------------------------------------------------------
# The matrix uses 3 markers:
#   "O" — out of scope, no work yet
#   "X" — in progress
#   "✓" — fully done
# For races and classes, the canonical state is read from
# parse_class_chassis_table / parse_race_chassis_table (3-state + 4th unassigned).
# For books, the state is read from the wired-integration registry.

BOOK_ID_TO_CLASS_BOOK = {
    "core_rulebook": "CRB",
    "advanced_players_guide": "APG",
    "advanced_class_guide": "ACG",
    # bestiary_1 has no PC classes at all -- deliberately absent, aggregation
    # returns "unassigned" for it below.
}


def aggregate_book_classes_state(classes: list, class_book_key: str) -> str:
    """Real per-book 'classes' doneness, aggregated from the same per-class
    states `parse_class_chassis_table` already computes (see that function's
    own doc comment for the authoritative-source discipline this mirrors).

    Fixes a real bug (operator report, 2026-07-26): the book matrix
    previously hardcoded every in-scope book's 'classes' column to
    "in-progress" unconditionally, regardless of any class's real state --
    meaning this column could never show "done" or "untouched" no matter
    how much real progress landed underneath. Aggregation rule: "full" only
    if every class in the book reaches "full"; "in-progress" if any class
    has real engine progress (full or in-progress); "untouched" if the book
    has classes but none have started; "unassigned" if the book (Bestiary 1)
    has no PC classes at all.
    """
    book_classes = [c for c in classes if _observer.CLASS_BOOK.get(c.get("class_id")) == class_book_key]
    if not book_classes:
        return "unassigned"
    states = {c["state"] for c in book_classes}
    if states == {"full"}:
        return "full"
    if "full" in states or "in-progress" in states:
        return "in-progress"
    return "untouched"


# `aggregate_races_state` used to live here. It aggregated
# `parse_race_chassis_table`'s per-race statuses -- a regex scrape of
# hand-written prose in `risks-and-open-questions.md` -- and then applied that
# single aggregate uniformly to all four in-scope books, including the three
# that ingest no race records at all. `derive_book_matrix` replaced it: races
# are now run through the real compute pipeline by `v06_content_state_dump`'s
# race probe, and the marker is scoped to books that genuinely ingest races.


def status_to_marker(state: str) -> str:
    """Map the parser's state vocabulary to the matrix's 3-marker display."""
    if state == "full":
        return "\u2713"  # ✓
    if state == "in-progress":
        return "X"
    if state == "unassigned":
        return "O"
    return "O"  # untouched also reads as O on the matrix


# ---------------------------------------------------------------------------
# SD-N auto-discovery
# ---------------------------------------------------------------------------

# Channel assignments for the 4 in-scope books come from IN_SCOPE_BOOKS above.
# The 21 future-state books all share `channel: "SD-27+"` per the registry.
# When the operator's prose names a specific SD-N (e.g. "SD-27" not "SD-27+"),
# we surface it as a workchannel button. The auto-discovery is: any SD-N
# mentioned by the lead's prose is a button. This is a deliberately simple
# approach — it doesn't fabricate bundles, just surfaces ones the lead named.

SD_N_RE = re.compile(r"\bSD-\d+\b")


def discover_workchannels(status_text: str) -> list:
    """Return the ordered list of workchannels surfaced by the lead's prose.

    The 4 in-scope books default to `v0.6`. ARG + AG (the operator-pinned
    SD-27 books) default to `SD-27`. The remaining 19 future-state books
    default to `SD-28+` (deferred). Any explicit SD-N named in SWARM_STATUS.md
    prose is added as a workchannel button.
    """
    channels = [
        {"id": "v0.6", "label": "v0.6 alpha", "kind": "active"},
        {"id": "SD-27", "label": "SD-27 (ARG + PU)", "kind": "active"},
        {"id": "SD-28", "label": "SD-28 (Ultimate)", "kind": "deferred"},
        {"id": "SD-29", "label": "SD-29 (Bestiary)", "kind": "deferred"},
        {"id": "SD-30", "label": "SD-30 (Adventure+)", "kind": "deferred"},
    ]
    # Auto-discover: any SD-N mentioned in the lead's prose becomes a
    # workchannel. Order is the order of first appearance. SD-20/21/24/26
    # are closed prior-tranche work-efforts; they don't get surfaced.
    DROPPED_WORKCHANNELS = {"SD-20", "SD-21", "SD-24", "SD-26"}
    seen = {"v0.6", "SD-27", "SD-28", "SD-29", "SD-30"}
    if status_text:
        for m in SD_N_RE.finditer(status_text):
            cid = m.group(0)
            if cid in DROPPED_WORKCHANNELS:
                continue
            if cid not in seen:
                # Infer kind from id shape: bare "SD-N" is planning, "SD-N+" is deferred.
                kind = "deferred" if cid.endswith("+") else "planning"
                channels.append({"id": cid, "label": cid, "kind": kind})
                seen.add(cid)
    return channels


# ---------------------------------------------------------------------------
# Channel-scoped data
# ---------------------------------------------------------------------------

def build_v0_6_channel_data(status_text: str, risks_text: str, risks_path: str, report_text: str = "") -> dict:
    """Bundle the v0.6 channel's HUD data: agents, full-reach, decisions, questions."""
    agents = _observer.parse_agent_status(status_text, "")
    progress = _observer.parse_class_breadth_progress(status_text)
    # Same engine-first, prose-fallback discipline as build_pf1e_dashboard's
    # own matrix: the channel bundle must not disagree with the matrix it
    # sits next to.
    engine_states = load_engine_class_states()
    classes = (
        engine_class_rows(engine_states)
        if engine_states
        else _observer.parse_class_chassis_table(status_text, report_text)
    )
    races = _observer.parse_race_chassis_table(risks_text)
    # parse_operator_decisions takes a path, not a string. Pass the path through.
    decisions = _observer.parse_operator_decisions(risks_path)
    session_prose = _observer.parse_session_prose(status_text)
    reach_list = _observer.parse_class_reach_list(status_text)
    # Engine-first for `reach` too (2026-08-12). It was the last figure in this
    # bundle still sourced ONLY from prose, and it had drifted the furthest:
    # `parse_class_reach_list()` regex-scrapes an exact
    # "<N> of 27 classes now genuinely reach Computed**: <list>" phrase out of
    # SWARM_STATUS.md, and its last real match in the whole document is the
    # 10-class "...Bard" line written before Arcanist existed. Every later
    # closure was invisible to it, silently, because the lead's prose stopped
    # repeating that exact sentence -- the same failure the operator diagnosed
    # in SWARM_STATUS.md itself (see its 2026-07-2x root-cause entry).
    #
    # Scoped to the 27 v0.6 roster classes on purpose. The dump carries 31,
    # the extra four being Pathfinder Unchained variants that are not on this
    # channel's roster, so reporting 31/27 would be worse than the stale
    # figure: a ratio above 1.0 that looks like a bug in the denominator.
    reach = engine_reach(engine_states) or {
        "reached": progress.get("reached"),
        "total": progress.get("total"),
        "rest": progress.get("rest", ""),
        "list": reach_list,
        "source": "swarm-status-prose",
    }

    return {
        "schema_version": 2,
        "managed_by": "producer",
        "agents": [
            {"agent": a["agent"], "status": a["status"], "snippet": a.get("snippet", "")}
            for a in agents
        ],
        "reach": reach,
        "session_prose": session_prose,
        "decisions": {
            "open": [d for d in decisions if d.get("is_open")],
            "context": [d for d in decisions if not d.get("is_open") and d.get("category") == "context"],
            "autonomous": [d for d in decisions if d.get("category") == "autonomous"],
            "resolved": [d for d in decisions if d.get("category") == "resolved"],
        },
        "classes": classes,
        "races": races,
    }


# ---------------------------------------------------------------------------
# Main producer
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# Engine-derived class state
# ---------------------------------------------------------------------------

def _resolve_cargo() -> str | None:
    """Absolute path to cargo.

    cron runs this producer with a near-empty PATH, so a bare "cargo" is
    ENOENT there even though it resolves fine in an interactive shell -- the
    refresh would silently never happen and the panel would freeze at
    whatever cache it last had.
    """
    import shutil

    found = shutil.which("cargo")
    if found:
        return found
    for candidate in (
        os.environ.get("CARGO_BIN"),
        os.path.expanduser("~/.cargo/bin/cargo"),
        "/usr/local/bin/cargo",
        "/usr/bin/cargo",
    ):
        if candidate and os.path.exists(candidate):
            return candidate
    return None


def _run_state_dump(
    bin_name: str, repo_root: str, bin_args: list[str] | None = None
) -> dict | None:
    """Run one of the engine's own state-dump binaries and return its JSON.

    Shared by `v06_class_state_dump`, `v06_content_state_dump` and
    `v06_work_inventory`: all three are plain `cargo run --bin <name>` targets
    in the same crate that print JSON on stdout, and all three need the
    identical cron-hostile-PATH handling and private target dir, so there is
    one implementation rather than three that can drift.

    `bin_args` are passed after `--` to the binary itself; the work inventory
    uses it for `--summary`.
    """
    import subprocess

    root = pathlib.Path(repo_root)
    if not (root / "Cargo.toml").exists():
        print(f"pf1e-producer: no cargo project at {root}", file=sys.stderr)
        return None

    cargo = _resolve_cargo()
    if not cargo:
        print(
            f"pf1e-producer: cargo not found, cannot refresh {bin_name}",
            file=sys.stderr,
        )
        return None

    env = dict(os.environ)
    env["CARGO_TARGET_DIR"] = DEFAULT_CLASS_STATE_TARGET_DIR
    # rustup's shim needs its own bin dir reachable even when cron hands us a
    # minimal PATH.
    env["PATH"] = os.pathsep.join(
        filter(None, [os.path.dirname(cargo), env.get("PATH", "")])
    )
    try:
        proc = subprocess.run(
            [cargo, "run", "--quiet", "--bin", bin_name] + (["--"] + bin_args if bin_args else []),
            cwd=str(root),
            env=env,
            capture_output=True,
            text=True,
            timeout=CLASS_STATE_BUILD_TIMEOUT_SECONDS,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        print(f"pf1e-producer: {bin_name} failed to run: {exc}", file=sys.stderr)
        return None

    if proc.returncode != 0:
        print(
            f"pf1e-producer: {bin_name} exited {proc.returncode}: "
            f"{proc.stderr[-400:]}",
            file=sys.stderr,
        )
        return None
    try:
        return json.loads(proc.stdout)
    except ValueError as exc:
        print(f"pf1e-producer: {bin_name} emitted non-JSON: {exc}", file=sys.stderr)
        return None


def _load_cached_dump(
    bin_name: str,
    cache_path: str,
    repo_root: str,
    max_age_seconds: int,
    bin_args: list[str] | None = None,
) -> dict | None:
    """The engine's own truth for `bin_name`, refreshed when it ages out.

    Returns None only when there is neither a usable cache nor a runnable
    engine. A STALE cache is deliberately preferred over None: a blank panel
    renders as "not started", which is the exact failure this whole mechanism
    exists to prevent.
    """
    cache = pathlib.Path(cache_path)
    cached = None
    fresh_enough = False
    if cache.exists():
        try:
            cached = json.loads(cache.read_text(encoding="utf-8"))
            age = dt.datetime.now().timestamp() - cache.stat().st_mtime
            fresh_enough = age < max_age_seconds
        except (OSError, ValueError):
            cached = None

    if cached is not None and fresh_enough:
        return cached

    produced = _run_state_dump(bin_name, repo_root, bin_args)
    if produced is None:
        return cached
    try:
        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_text(json.dumps(produced, indent=2), encoding="utf-8")
    except OSError as exc:
        print(f"pf1e-producer: could not cache {bin_name}: {exc}", file=sys.stderr)
    return produced


def load_engine_content_state(
    cache_path: str = DEFAULT_CONTENT_STATE_CACHE,
    repo_root: str = DEFAULT_REPO_ROOT,
    max_age_seconds: int = CONTENT_STATE_MAX_AGE_SECONDS,
) -> dict | None:
    """Per-book ingested content, the Bestiary roster, and feat-effect wiring,
    all counted out of the real engine by `v06_content_state_dump`."""
    return _load_cached_dump(
        "v06_content_state_dump", cache_path, repo_root, max_age_seconds
    )


def load_work_inventory(
    cache_path: str = DEFAULT_WORK_INVENTORY_CACHE,
    repo_root: str = DEFAULT_REPO_ROOT,
    max_age_seconds: int = WORK_INVENTORY_MAX_AGE_SECONDS,
) -> dict | None:
    """The corpus-wide work inventory summary, generated by
    `v06_work_inventory --summary`.

    Covers all 25 corpus books -- the 4 ingested ones and the 21 nobody has
    started -- so the dashboard has a real denominator behind "how much is
    left" instead of a roster of book titles."""
    return _load_cached_dump(
        "v06_work_inventory",
        cache_path,
        repo_root,
        max_age_seconds,
        bin_args=["--summary"],
    )


def _cross_tab_status_margin(wiring: dict | None) -> dict:
    """Project the wiring cache's cross_tab onto its status axis.

    The result is comparable to the summary's UNADJUSTED `by_status` (both
    cover every book including excluded ones), which makes it the arithmetic
    skew test between the two work-inventory sources -- see the
    `status_sources_agree` field in work_inventory_panel()."""
    margin: dict[str, int] = {}
    for cell, count in ((wiring or {}).get("cross_tab") or {}).items():
        status = cell.split("|", 1)[1]
        margin[status] = margin.get(status, 0) + count
    return margin


def work_inventory_panel(inventory: dict | None, wiring: dict | None = None) -> dict:
    """The dashboard's work-inventory panel, built entirely out of the
    generator's own aggregates.

    Nothing is recomputed here, and nothing is defaulted to a plausible-looking
    number: when the inventory cannot be produced at all the panel says so in
    its own `available` field rather than rendering zeros, because a zero in a
    doneness panel reads as "not started" and that misreading is the exact
    failure this whole mechanism exists to prevent.
    """
    if not inventory:
        return {
            "available": False,
            "note": (
                "v06_work_inventory could not be produced and no cached summary "
                "exists; totals below are absent, not zero"
            ),
        }

    # Operator directive 2026-08-02: `core_essentials` and `beginner_box`
    # ruled "redundant to other tomes, never coming into scope" and excluded
    # from this dashboard's book list.
    #
    # REVERSED 2026-08-10 for `core_essentials` specifically, on the
    # reasoning that hiding it hid 1,595 units that "exist nowhere else."
    # That reasoning is now PARTIALLY SUPERSEDED, not overturned:
    # `core_essentials` genuinely is PCGen's shared packaging directory
    # (physical storage), never a book (attribution) -- the operator's own
    # 2026-08-16 dashboard read caught the resulting defect directly
    # (`OPEN-ISSUES.md` row 68: "race is at 0%, I don't see the core rules
    # book listed under race, and advanced race guide reports as nearly
    # untouched"), and `SD31-ATTRIB-001` fixed `v06_work_inventory`'s own
    # `book` field to attribute each `core_essentials`-sourced unit to its
    # TRUE source book wherever that is provable one record deep (44 of 51
    # races; every companion/spell/class/feat/equipment unit whose file
    # carries a `SOURCELONG:` header; every race_trait row nested under a
    # resolved race's own directory). `core_essentials`'s own residual
    # dropped from 1,610 to **128** as of `SD31-D9-DISSOLVE-001`
    # (2026-08-16, further detail below): 9 `monster_ability` + 111
    # `race_trait` (29 of the two from the single file
    # `core_essentials/ce_abilities_race.lst`; the rest from the 8 ambiguous
    # races' own per-race trait files, see below) + 8 `race` (races two or
    # more in-scope books natively declare, so no single true book is
    # provable -- Android, Aquatic Elf, Gathlain, Ghoran, Goblin
    # (Monkey), Lashunta, Syrinx, Triaxian; see `v06_work_inventory.rs`'s
    # `RACE_TRUE_BOOK` doc comment for the full derivation).
    #
    # **`SD31-D9-DISSOLVE-001` (2026-08-16) fixed the 516-unit gap
    # `SD31-ATTRIB-002` found and left open.** `resolve_true_book_for_core_essentials`
    # (`v06_work_inventory.rs`) is now source-line-aware: it tracks which of
    # `ce_abilities_race.lst`'s 11 mid-file `SOURCELONG:<Book>` directive
    # lines (1273/1624/1794/2221/2275/2342/2361/2406/2420/2432/2441) most
    # recently preceded each row, rather than only checking a file's first 5
    # lines (which found none for this file, since its own header is a plain
    # comment). Verified against the pinned oracle: line 1273's
    # `SOURCELONG:Bestiary` precedes the file's own `###Block: *** Universal
    # Monster Rules, pages 297-306 ***` header, and the "Ability Damage"/
    # "Ability Drain" rows immediately under it are exactly Bestiary 1's own
    # Universal Monster Rules appendix. Resolved 516 of 545: `bestiary` 263,
    # `bestiary_2` 206, `bestiary_3` 41, `bestiary_4` 2, `bestiary_5` 1,
    # `bestiary_6` 3 -- `corpus_literal_sweep.rs`'s own `short_book_of`
    # synced in the same commit (its own doc comment previously disclaimed
    # root-level `ce_*.lst` resolution entirely), so neither file's join key
    # diverges from the other. **A first draft of this fix shipped a real
    # regression** (an unrecognized directive, `SOURCELONG:Universal Rules`,
    # silently inherited the PRECEDING recognized directive's book instead of
    # resetting to unattributed) -- caught by re-deriving the real corpus
    # effect before commit, fixed, and mutation-proven both directions
    # (`core_essentials_book_attribution_tests::
    # an_unrecognized_directive_resets_tracking_rather_than_inheriting_the_prior_book`).
    # The remaining 129-unit residual (down from 644) is genuinely
    # unattributable: 23 rows precede the file's first `SOURCELONG:` line
    # (lines 1-1272, the file's own top-of-file comment confirms this
    # stretch is genuinely PCGen's book-agnostic "Default Internal Ability"
    # content), 6 carry `SOURCELONG:Universal Rules` (PCGen's own internal
    # designation, not a Paizo book this program tracks), and the 8 races
    # two or more in-scope books natively declare (below) contribute the
    # rest. Zero doneness impact -- `book` is a pure reporting field, per
    # the same 0-transition proof `SD31-ATTRIB-001`/`SD31-W5-INTEGRATE-001`
    # already established for every prior relabel in this program. Full
    # derivation: `OPEN-ISSUES.md` row 98/`progress.md`'s `SD31-D9-DISSOLVE-001`
    # receipt.
    #
    # **`decisions.md §10`'s "newest publish wins" ruling, applied to
    # `kind == race` specifically**, same cycle: 32 of the 43 unambiguously-
    # attributed races (7 Core Rulebook + 11 Bestiary 1 + 7 Bestiary 2 + 5
    # Bestiary 3 + 2 Inner Sea World Guide -- every one currently on a book
    # STRICTLY OLDER than Advanced Race Guide's own `SOURCEDATE:2012-06`,
    # `advanced_race_guide/advanced_race_guide.pcc`, that ARG's own `.lst`
    # files independently carry rows for) now attribute to
    # `advanced_race_guide` instead -- ARG the newer of their two printings,
    # the ruling's own worked example (Catfolk, Bestiary 3 2012-01 -> ARG
    # 2012-06). `core_rulebook` race now reads 0 (all 7 moved), and
    # `advanced_race_guide` race reads 33 (32 moved + its own pre-existing
    # `Race Builder` scaffold unit). **Bestiary 4's own 5 ARG-reprinted races
    # (Changeling/Kitsune/Nagaji/Samsaran/Wayang) are deliberately EXCLUDED**
    # from this move, correcting `decisions.md §10`'s own worked-example list
    # (which named Changeling as needing to move): Bestiary 4's own
    # `SOURCEDATE:2013-10` postdates ARG's `2012-06`, so under strict
    # SOURCEDATE ordering -- `§10`'s own binding rule -- Bestiary 4 is
    # already the newer printing there. Scoped to `kind == race` only:
    # `race_trait` rows from the SAME race directories are unaffected, still
    # on their true FIRST-printing book (`§10`'s own text answers the
    # `advanced_race_guide` "nearly untouched" observation via `race_trait`,
    # not by moving it too). Zero doneness impact, same 0-transition proof.
    #
    # This panel keeps `core_essentials` UN-excluded rather than
    # re-hiding it: that residual is real, genuinely un-attributable content,
    # and the 2026-08-10 directive's underlying worry -- a shrinking
    # denominator with nobody told -- applies exactly as much to it as it
    # ever did. Only the SIZE of what core_essentials legitimately owns has
    # changed, not the decision to show it.
    #
    # `beginner_box` is NO LONGER excluded (closed 2026-08-24, `decisions.md
    # §27b`) -- see `EXCLUDED_BOOKS`'s own declaration for the full
    # derivation. Its 19 equipment units now flow into every figure below
    # like any other not-started population, which is the honest state:
    # real records this dashboard was previously hiding from its own
    # denominators, not a book with no source data to show.
    #
    # Filtered HERE rather than in `v06_work_inventory` on purpose (unchanged
    # from the original directive). `core_essentials`'s directories remain
    # load-bearing inside the generator regardless of this dashboard-level
    # inclusion change: all seven Core Rulebook races are `.MOD` rows over
    # bases that `core_rulebook.pcc` pulls in from `core_essentials`, and the
    # generator's own `mod_only_rescue` trap rule notes that dropping them
    # blindly "would report the Core Rulebook as having ZERO playable races".
    excluded = EXCLUDED_BOOKS

    totals = inventory.get("totals") or {}
    by_status = totals.get("by_status") or {}
    total_units = totals.get("units", 0)
    # "Done" is deliberately narrow: only the statuses whose definition is a
    # positive proof. `ingested-magnitude` is NOT counted as done -- the engine
    # holds the record's numbers but no consumer delta was observed, and
    # rolling it into a done figure would be exactly the over-claim the
    # generator's own status vocabulary refuses to make.
    proven = by_status.get("grounded", 0) + by_status.get("text-complete", 0)
    books = []
    # Totals arrive already summed over every book, so dropping the excluded
    # rows from the list is not enough -- their units would keep inflating
    # every denominator on the dashboard. Subtract what we drop.
    dropped_units = 0
    dropped_status: dict[str, int] = {}
    dropped_kind: dict[str, int] = {}
    for book in inventory.get("books") or []:
        kinds = book.get("kinds") or {}
        units = sum((k.get("units") or 0) for k in kinds.values())
        statuses: dict[str, int] = {}
        for k in kinds.values():
            for status, count in (k.get("by_status") or {}).items():
                statuses[status] = statuses.get(status, 0) + count
        if book.get("id") in excluded:
            dropped_units += units
            for status, count in statuses.items():
                dropped_status[status] = dropped_status.get(status, 0) + count
            for kind_name, k in kinds.items():
                dropped_kind[kind_name] = dropped_kind.get(kind_name, 0) + (k.get("units") or 0)
            continue
        books.append({
            "id": book.get("id"),
            "scope": book.get("scope"),
            "engine_rule_set": book.get("engine_rule_set"),
            "units": units,
            "by_status": statuses,
            "proven": statuses.get("grounded", 0) + statuses.get("text-complete", 0),
            # by_wiring_class is carried per book row the same way by_status
            # is, immediately above. Not present under "proven": wiring_class
            # is an orthogonal axis and never feeds proven (GE-09).
            "by_wiring_class": (wiring or {}).get("by_book", {}).get(book.get("id"), {}),
            # Doneness per book, from the cross-tab -- the per-book figure the
            # margins above cannot produce. Same source, same exclusion.
            "by_doneness": (wiring or {}).get(
                "doneness_by_book", {}).get(book.get("id"), {}),
            # Kept verbatim: these rows are how a reader tells "the corpus and
            # the engine agree" from "they do not", per kind.
            "reconciliation": book.get("reconciliation") or [],
            "trap_hits": book.get("trap_hits") or {},
        })

    adj_status = {k: v - dropped_status.get(k, 0) for k, v in by_status.items()}
    adj_status = {k: v for k, v in adj_status.items() if v}
    adj_kind = {k: v - dropped_kind.get(k, 0)
                for k, v in (totals.get("by_kind") or {}).items()}
    adj_kind = {k: v for k, v in adj_kind.items() if v}
    adj_proven = adj_status.get("grounded", 0) + adj_status.get("text-complete", 0)

    # Corpus-wide by_wiring_class, same exclusion applied as by_status above
    # so the two distributions' denominators agree. Zero-filled across all
    # five values (not just the ones seen) so "ambiguous" renders as a
    # visible 0 rather than disappearing when a run happens to clear it.
    wiring_available = bool(wiring and wiring.get("available"))
    by_wiring_class = {v: 0 for v in WIRING_CLASS_VALUES}
    dropped_wiring: dict[str, int] = {}
    if wiring_available:
        for wc, count in (wiring.get("corpus_wide") or {}).items():
            by_wiring_class[wc] = by_wiring_class.get(wc, 0) + count
        for book_id in excluded:
            for wc, count in (wiring.get("by_book") or {}).get(book_id, {}).items():
                dropped_wiring[wc] = dropped_wiring.get(wc, 0) + count
                by_wiring_class[wc] = by_wiring_class.get(wc, 0) - count

    # Doneness: the same exclusion, subtracted the same way, so the ladder's
    # denominator agrees with by_status's and by_wiring_class's. Zero-filled
    # across every value so an empty bucket renders as a visible 0 -- an absent
    # `unmeasurable` key reads as "nothing unclassifiable" rather than "no run".
    #
    # `doneness_available` is separate from `wiring_class_available` on purpose:
    # the ladder additionally requires a schema-2 cache, so an older cache that
    # is otherwise fine yields wiring classes without doneness, and the viewer
    # has to be able to tell that apart from a missing run.
    by_doneness = {v: 0 for v in DONENESS_VALUES}
    dropped_doneness: dict[str, int] = {}
    doneness_available = bool(wiring_available and (wiring or {}).get("doneness"))
    if doneness_available:
        for verdict, count in (wiring.get("doneness") or {}).items():
            by_doneness[verdict] = by_doneness.get(verdict, 0) + count
        for book_id in excluded:
            for verdict, count in (
                    wiring.get("doneness_by_book") or {}).get(book_id, {}).items():
                dropped_doneness[verdict] = dropped_doneness.get(verdict, 0) + count
                by_doneness[verdict] = by_doneness.get(verdict, 0) - count

    return {
        "available": True,
        "generated_at": inventory.get("generated_at"),
        "generated_by": inventory.get("generated_by"),
        "corpus_root": inventory.get("corpus_root"),
        "total_units": total_units - dropped_units,
        "proven_units": adj_proven,
        "by_status": adj_status,
        "by_kind": adj_kind,
        # wiring_class is orthogonal to by_status/proven above -- see the
        # compute_wiring_class_summary() docstring. "ambiguous" is always
        # present, including as an explicit 0, per GE-09: it must render as a
        # first-class value that does not count toward coverage, never be
        # folded into an "other" bucket or omitted when empty.
        "wiring_class_available": wiring_available,
        "by_wiring_class": by_wiring_class,
        "wiring_class_note": (
            None if wiring_available
            else (wiring or {}).get("note", "wiring_class summary unavailable")
        ),
        "wiring_class_determinator_versions": (wiring or {}).get("determinator_versions", []),
        # --- doneness: the cross-tab and its rollups (2026-08-12, SD-29 §46) ---
        #
        # by_status and by_wiring_class above are the two MARGINS of one table.
        # Doneness lives in its cells and cannot be recovered from either margin,
        # which is why the dashboard could show both and still not answer "how
        # done is this". `cross_tab` ships the joint distribution so the viewer
        # can show the cells directly; `by_doneness` is the ladder rolled up.
        #
        # `held` is NOT `done` and must never be summed into one figure with it:
        # per SD-29 `decisions.md §46.4` those units flip to `done` only when
        # the byte-equality sweep and the fixture check exist. `unmeasurable`
        # counts toward NEITHER coverage nor gap -- it is a defect in the
        # instruments.
        "doneness_available": doneness_available,
        # Two snapshots of the same generator feed this block: `by_status`
        # above comes from a fresh `v06_work_inventory --summary` run (its
        # stamp is this block's `generated_at`), while every doneness/
        # cross_tab figure is derived from the COMMITTED full document at
        # `full_document` (its stamp is `doneness_source_generated_at`).
        # The stamps routinely differ without the figures differing, so the
        # honest skew test is the arithmetic one: the cross_tab's status
        # margin must equal the summary's unadjusted by_status. When it does
        # not, the two sources really are different corpus snapshots and
        # `status_sources_agree` says so instead of the mismatch surfacing
        # as buckets that mysteriously fail to reconcile (2026-08-13
        # reconciliation audit).
        "doneness_source_generated_at": (wiring or {}).get("generated_at"),
        "status_sources_agree": (
            not doneness_available
            or _cross_tab_status_margin(wiring) == dict(by_status)
        ),
        "doneness_values": list(DONENESS_VALUES),
        "doneness_meaning": dict(DONENESS_MEANING),
        "by_doneness": by_doneness,
        # The mandate headline (Decision 5, operator ruling 2026-08-15,
        # launch-readiness remediation Step 4C): done against the WHOLE
        # denominator, not the in-scope-and-measurable subset the old
        # headline used. `total_units - dropped_units` above is the same
        # figure already served under `total_units` a few lines up -- passed
        # explicitly here rather than re-derived so this field can never
        # drift from it. See `_mandate_headline()`'s own docstring for why
        # nothing is subtracted.
        "mandate_headline": _mandate_headline(by_doneness, total_units - dropped_units, books,
                                              (wiring or {}).get("doneness_unmapped", {})),
        # Per-kind ladder, corpus-wide, same exclusion as by_doneness above.
        # `doneness_by_kind` on the cache is UNADJUSTED (every book, including
        # excluded ones); subtract each excluded book's per-kind contribution
        # via `doneness_by_kind_by_book` so this sums to `by_kind`, the same
        # way `by_doneness` above sums to `total_units`. Deep-copied so the
        # excluded books' book-scoped totals below stay a true rollup of the
        # raw cache, independent of this subtraction.
        "by_doneness_kind": _exclude_books_from_kind_doneness(
            (wiring or {}).get("doneness_by_kind", {}),
            (wiring or {}).get("doneness_by_kind_by_book", {}),
            excluded),
        # Same exclusion as by_doneness_kind immediately above -- see finding
        # #7, round 2, 2026-08-12: `cross_tab` on the cache is UNADJUSTED
        # (every book, including beginner_box), so subtract it the same way
        # via `cross_tab_by_book` rather than serving the raw cache value.
        "cross_tab": _exclude_books_from_flat_counts(
            (wiring or {}).get("cross_tab", {}),
            (wiring or {}).get("cross_tab_by_book", {}),
            excluded),
        # Exact per-kind count feeding the cross-tab caption's `spell`-specific
        # claim (round 19, dash-frontend finding #2, 2026-08-12): previously a
        # hand-typed "178" in the viewer with no guard against the corpus
        # growing. `cross_tab` has no per-kind breakdown (it mixes every kind
        # per cell, by design -- see renderCrossTab()'s own comment in the
        # viewer), so this is computed from `cross_tab_by_kind`, the same
        # exclusion-adjustment pattern as `by_doneness_kind` above, then
        # summed over the three specific (wiring_class, status) cells the
        # caption describes: computed+ingested-magnitude, computed+
        # text-complete, and display+ingested-magnitude -- the three cells
        # where a probeless kind's kind-agnostic colour (in-progress) disagrees
        # with its kind-capped verdict (held).
        #
        # SD30-E0-F2 (2026-08-14) fix: gated on `"spell" in NO_GROUNDING_PROBE`
        # (currently False -- see that constant's declaration). Before this
        # fix the sum below was unconditional, so it kept reporting the raw
        # structural count of spell records sitting in these three cells even
        # after `spell` left `NO_GROUNDING_PROBE` -- correct as a record count,
        # but WRONG as this field's actual contract ("records where the
        # kind-agnostic colour disagrees with the kind-capped verdict"),
        # because with the cap removed those two verdicts no longer disagree
        # for `spell` at all. The viewer's caption built on this number would
        # have kept asserting a disagreement that had stopped being true the
        # moment the cap lifted -- exactly the stale-claim shape this cycle's
        # other fixes were about, just reached through a number instead of
        # prose.
        "spell_kind_capped_count": (
            sum(
                _exclude_books_from_kind_doneness(
                    (wiring or {}).get("cross_tab_by_kind", {}),
                    (wiring or {}).get("cross_tab_by_kind_by_book", {}),
                    excluded,
                ).get("spell", {}).get(cell, 0)
                for cell in (
                    "computed|ingested-magnitude",
                    "computed|text-complete",
                    "display|ingested-magnitude",
                )
            )
            if "spell" in NO_GROUNDING_PROBE else 0
        ),
        # Book x kind doneness cross (round 11, operator UI-first reframe,
        # 2026-08-12): "how usable is Ultimate Equipment's equipment
        # specifically" cannot be answered from `by_doneness` (whole corpus)
        # or `by_doneness_kind` (kind, but every book blended together) --
        # this is the third axis. NOT a new instrument and NOT a new
        # accumulation loop: `doneness_by_kind_by_book` (book -> kind ->
        # verdict -> count) already exists on the wiring-class-summary cache,
        # added the same round as `doneness_by_kind` itself specifically so
        # the by-book exclusion above could subtract per-kind, per-book
        # contributions -- see that dict's declaration in
        # compute_wiring_class_summary(). This just exposes what was already
        # being computed for an internal subtraction, filtered the same way
        # every other rollup on this panel is: excluded books dropped
        # entirely (no subtraction needed -- each book's own dict is already
        # scoped to that book, unlike the corpus-wide margins above).
        "by_doneness_book_kind": {
            book_id: kind_map
            for book_id, kind_map in
            ((wiring or {}).get("doneness_by_kind_by_book") or {}).items()
            if book_id not in excluded
        },
        # `computed` + `grounded` per kind, same exclusion as everything above
        # (SD-29 QA finding #17, round 3, 2026-08-12) -- the mechanically-
        # confirmed sub-count of `done`, for the viewer to show next to the
        # headline "done" figure rather than leaving it buried in the raw
        # cross-tab.
        "mechanically_confirmed_by_kind": _exclude_books_from_flat_counts(
            (wiring or {}).get("mechanically_confirmed_by_kind", {}),
            (wiring or {}).get("mechanically_confirmed_by_kind_by_book", {}),
            excluded),
        # SD-32 Epic 2 T8 (D13) -- computed pre-excluded in
        # `compute_wiring_class_summary()` itself (the per-unit loop checks
        # `book not in EXCLUDED_BOOKS` inline), unlike the other fields on
        # this panel, because it is a unit-id list rather than a count
        # dict -- there is no per-book breakdown to subtract from here.
        # Falls back to an explicit zero-count shape (never absent) so an
        # older, pre-this-field cache still reports "checked, none found"
        # rather than a viewer-side `undefined`.
        "classifier_reclassified_units": (wiring or {}).get(
            "classifier_reclassified_units",
            {"predicate": "kind=='class_feature' and wiring_class=='display' and "
                          "status=='grounded' and evidence=="
                          "'explanation_id_observed_in_a_real_computation', "
                          "EXCLUDED_BOOKS dropped",
             "reclassified_to": "computed", "count": 0, "units": []}),
        "doneness_unmapped": (wiring or {}).get("doneness_unmapped", {}),
        # Single-sourced with the producer's own `NO_GROUNDING_PROBE` (round 8,
        # SD-29 QA finding, 2026-08-12) -- the viewer reads this instead of
        # hand-maintaining its own copy of the same fact. Falls back to the
        # viewer's own hardcoded default only when an older cache (pre-this-
        # field) is being served.
        "no_grounding_probe_kinds": (wiring or {}).get(
            "no_grounding_probe_kinds", list(NO_GROUNDING_PROBE)),
        "doneness_note": (
            None if doneness_available
            else "doneness rollup unavailable (needs a schema-"
                 f"{WIRING_SUMMARY_SCHEMA} wiring-class summary) -- not the same as zero."
        ),
        "status_vocabulary": inventory.get("status_vocabulary") or {},
        "books": books,
        # Stated, not silent: a denominator that shrank without saying so is
        # indistinguishable from work disappearing.
        "excluded_books": {
            "ids": sorted(excluded),
            "units": dropped_units,
            "proven": dropped_status.get("grounded", 0) + dropped_status.get("text-complete", 0),
            "by_wiring_class": dropped_wiring,
            "by_doneness": dropped_doneness,
            "reason": (
                "Operator directive 2026-08-02: a simplified introductory subset, "
                "redundant to other tomes, never coming into scope. "
                "(`core_essentials` was excluded under this same directive until "
                "2026-08-10, when the operator reversed it -- it is not "
                "redundant, Core Rulebook's races are .MOD patches over "
                "core_essentials's own base definitions, not a duplicate of them. "
                "As of SD31-ATTRIB-001 (2026-08-16) core_essentials-sourced units "
                "attribute to their TRUE book wherever provable one record deep; "
                "operator ruling §16 (2026-08-19, wave 16) then ordered every "
                "remaining residual -- content no single in-scope book could be "
                "shown to own -- DELETED as a hallucination until it appears in "
                "print, rather than merely flagged. The `core_essentials` row is "
                "now 0 units, down from 1,610 (SD31-D9-DISSOLVE-001, 2026-08-16, "
                "fixed the 516-unit re-attributable population OPEN-ISSUES.md row "
                "94 had left open) then 128 (SD31-ATTRIB-001) then 0 (ruling §16, "
                "wave 16 -- 12 re-attributed to their true book, 116 deleted), "
                "which discharges decisions.md §9's condition: the label no "
                "longer appears in `docs/work-inventory.json.books` at all. See "
                "work_inventory_panel()'s own doc comment for the full derivation "
                "and decisions.md §9/§10/§16's race-attribution work.)"
            ),
        },
        "full_document": "docs/work-inventory.json",
    }


# ---------------------------------------------------------------------------
# Retrospective event log (2026-07-30)
# ---------------------------------------------------------------------------
#
# The three dumps above all answer "what is the state of the work right now".
# None of them answers "how is the run going" -- and over a multi-day run with
# nineteen books left, the second question is the one that changes how the run
# is conducted.
#
# The evidence: in one session four different agents corrected four different
# figures the lead had stated confidently -- a feat count, a spell total, a
# book count, and a clippy baseline that was counting summary lines and had
# already propagated into every brief written that day. Every correction was
# right. None of it exists in git, which records only what landed. The pattern
# across the four is worth more than any individual fix, and it was visible
# only because each correction happened to be mentioned in prose that scrolls
# away.
#
# `scripts/retro.py` in the repo is an append-only JSONL log of exactly those
# things -- corrections, incidents, near-misses, deferrals, rework -- plus a
# `verification` event emitted automatically by every `scripts/verify.sh` run,
# which supplies an honest denominator that no one has to remember to write.
#
# Unlike the three dumps this panel does NOT need caching: retro.py is stdlib
# python reading JSONL, not a cargo build, so it costs a process start rather
# than a compile. It is still asked for `--json` summary aggregates and never
# the raw event stream, for the same reason the work inventory is: the rows
# stay on disk in the repo where a retrospective can read them, and the
# dashboard carries only the counts.
RETRO_WINDOW = os.environ.get("PF1E_RETRO_WINDOW", "14d")
RETRO_TIMEOUT_SECONDS = int(os.environ.get("PF1E_RETRO_TIMEOUT", "60"))


def load_retro_summary(
    repo_root: str = DEFAULT_REPO_ROOT,
    window: str = RETRO_WINDOW,
    timeout_seconds: int = RETRO_TIMEOUT_SECONDS,
) -> dict | None:
    """Aggregates from `scripts/retro.py summary --json`, or None."""
    import subprocess

    script = pathlib.Path(repo_root) / "scripts" / "retro.py"
    if not script.exists():
        return None
    try:
        proc = subprocess.run(
            [sys.executable, str(script), "summary", "--json", "--since", window],
            cwd=str(repo_root),
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        print(f"pf1e-producer: retro.py failed to run: {exc}", file=sys.stderr)
        return None
    if proc.returncode != 0:
        print(
            f"pf1e-producer: retro.py exit {proc.returncode}: "
            f"{proc.stderr.strip()[:400]}",
            file=sys.stderr,
        )
        return None
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        print(f"pf1e-producer: retro.py output unparseable: {exc}", file=sys.stderr)
        return None


def retro_panel(summary: dict | None) -> dict:
    """The retrospective panel, built entirely out of retro.py's own aggregates.

    Nothing is recomputed here and nothing is defaulted. An empty log and an
    unavailable log are different facts and render differently: zero
    corrections in a healthy window means nobody was wrong, whereas a broken
    emitter means nobody knows. Collapsing those two into `0` is the same
    mistake as a doneness panel rendering zeros when it cannot measure.
    """
    if not summary:
        return {
            "available": False,
            "note": (
                "scripts/retro.py could not be run; the counts below are absent, "
                "not zero -- an empty log and an unreadable one are different facts"
            ),
        }

    events = summary.get("events") or {}
    corrections = summary.get("corrections") or {}
    incidents = summary.get("incidents") or {}
    near_misses = summary.get("near_misses") or {}
    deferrals = summary.get("deferrals") or {}
    rework = summary.get("rework") or {}
    verification = summary.get("verification") or {}
    log = summary.get("log") or {}

    return {
        "available": True,
        "generated_at": summary.get("generated_at"),
        "generated_by": summary.get("generated_by"),
        "window": (summary.get("window") or {}).get("spec") or RETRO_WINDOW,
        "total_events": events.get("total", 0),
        "by_type": events.get("by_type") or {},
        # `derived` events were written by a mechanism that observed them;
        # `agent` events were asserted by whoever typed them. Kept split
        # because the trustworthiness of the two is not the same and a reader
        # should not have to take the whole log at the weaker of the two.
        "by_origin": events.get("by_origin") or {},
        "corrections": {
            "total": corrections.get("total", 0),
            "by_subject": corrections.get("by_subject") or {},
            "by_corrector": corrections.get("by_corrector") or {},
            # The headline. A subject corrected more than once in a window is
            # the finding: not that a number was wrong, but that a source of
            # numbers is unreliable.
            "repeat_subjects": corrections.get("repeat_subjects") or [],
            "already_propagated": corrections.get("with_blast_radius", 0),
        },
        "incidents": {
            "total": incidents.get("total", 0),
            "silent": incidents.get("silent", 0),
            "recurring": incidents.get("recurring") or [],
        },
        "near_misses": {
            "total": near_misses.get("total", 0),
            "escaped": near_misses.get("escaped", 0),
            "by_gate": near_misses.get("by_caught_by") or {},
        },
        "deferrals": {"total": deferrals.get("total", 0)},
        "rework": {"total": rework.get("total", 0), "by_cause": rework.get("by_cause") or {}},
        "verification": {
            "runs": verification.get("runs", 0),
            "failed_runs": verification.get("failed_runs", 0),
            "fail_rate": verification.get("fail_rate"),
            "by_failing_stage": verification.get("by_failing_stage") or {},
        },
        "git_join": summary.get("git_join") or {},
        "invalid_lines": log.get("invalid_lines", 0),
        "full_log": "docs/retro/events/*.jsonl",
        "schema": "docs/retro/schema.json",
        "how_to_run_a_retrospective": (
            "scripts/retro.py summary --since 7d  (add --json for this shape)"
        ),
    }


def load_engine_class_states(
    cache_path: str = DEFAULT_CLASS_STATE_CACHE,
    repo_root: str = DEFAULT_REPO_ROOT,
    max_age_seconds: int = CLASS_STATE_MAX_AGE_SECONDS,
) -> dict | None:
    """The engine's per-class truth, refreshed from source when it ages out.

    Returns None only when there is neither a usable cache nor a runnable
    engine -- the one case where the caller must fall back to prose.
    """
    return _load_cached_dump(
        "v06_class_state_dump", cache_path, repo_root, max_age_seconds
    )


def _readable_blocker(diagnostic: dict) -> str:
    """One diagnostic rendered as a short, readable 'what is left' phrase."""
    # `class_feature.acg.warpriest.blessing_powers.unsupported`
    #   -> "warpriest blessing powers"
    # `class_chassis.unsupported`  -> "class chassis"
    # `defense.total_save.unsupported` -> "defense total save"
    diagnostic_id = diagnostic.get("id", "")
    subject = diagnostic_id
    if subject.endswith(".unsupported"):
        subject = subject[: -len(".unsupported")]
    # The book segment is already the row's own identity, so it is noise here.
    parts = [p for p in subject.split(".") if p not in ("apg", "acg", "crb")]
    subject = " ".join(parts[-2:]).replace("_", " ").strip() or diagnostic_id

    # The engine's own first sentence names the real gap. Cut on a sentence
    # boundary, never blind mid-token -- the whole point of this rewrite.
    message = " ".join((diagnostic.get("message") or "").split())
    sentence = re.split(r"(?<=[.;])\s+(?=[A-Z(`])", message)[0] if message else ""
    if len(sentence) > 200:
        sentence = sentence[:200].rsplit(" ", 1)[0].rstrip(",;:") + "…"
    return f"{subject} — {sentence}" if sentence else subject


def _level_ranges(levels: list) -> str:
    """[1,2,3,7,8] -> '1-3, 7-8'."""
    if not levels:
        return ""
    spans = []
    start = prev = levels[0]
    for level in levels[1:]:
        if level == prev + 1:
            prev = level
            continue
        spans.append((start, prev))
        start = prev = level
    spans.append((start, prev))
    return ", ".join(str(a) if a == b else f"{a}-{b}" for a, b in spans)


def engine_open_question(entry: dict) -> str:
    """A readable remaining-features summary, derived from real diagnostics."""
    if entry.get("computed"):
        return ""
    blocked = _level_ranges(entry.get("levels_blocked") or [])
    blockers = [_readable_blocker(d) for d in (entry.get("blocking_diagnostics") or [])]
    if not blockers:
        return f"Blocked at level(s) {blocked}." if blocked else "Blocked."

    head = f"Blocked at level(s) {blocked}: " if blocked else "Blocked: "
    text = head
    for i, blocker in enumerate(blockers):
        candidate = text + ("; " if i else "") + blocker
        # Whole-item boundary, so the column never ends mid-sentence.
        if len(candidate) > 700:
            text += f"; (+{len(blockers) - i} more)"
            break
        text = candidate
    return text


def engine_class_rows(states: dict) -> list:
    """The engine dump reshaped into observer.py's own per-class row shape.

    Keeps `class_id`/`book`/`state`/`open_question` so every existing
    consumer (the matrix comprehension, aggregate_book_classes_state, the
    v0.6 channel bundle) keeps working untouched.
    """
    rows = []
    for entry in states.get("classes", []):
        rows.append({
            "class_id": entry["id"],
            "book": entry["book"],
            # The HTML only knows "full" and "in-progress" for a started
            # class, and every class on this roster has real work behind it.
            "state": "full" if entry.get("computed") else "in-progress",
            "source": "engine-state-dump",
            "open_question": engine_open_question(entry),
        })
    return rows


# The v0.6 channel's own roster: the 27 CRB/APG/ACG/Ultimate classes the
# `v0.6_class_breadth` manifest scopes. Pathfinder Unchained's four variants are
# in the engine dump but not on this roster, so they are excluded from the
# channel's reach ratio rather than pushing it over its own denominator.
# Derived from the dump minus a named exclusion, not hand-listed -- a hand-kept
# roster here is exactly the defect class that produced three stale surfaces on
# this dashboard already.
V0_6_ROSTER_EXCLUDED = (
    "unchained_barbarian", "unchained_monk", "unchained_rogue", "unchained_summoner",
)


def engine_reach(states: dict | None) -> dict | None:
    """The v0.6 reach ratio, counted off the engine dump instead of prose.

    Returns None when there is no usable dump, which is the one case the
    caller should fall back to prose-scraping for.
    """
    if not states or not states.get("classes"):
        return None
    roster = [c for c in states["classes"] if c["id"] not in V0_6_ROSTER_EXCLUDED]
    if not roster:
        return None
    reached = [c for c in roster if c.get("computed")]
    # `list`/`rest` keep the shape the viewer already renders. Names are
    # title-cased off the engine's own ids rather than transcribed, so a class
    # cannot appear here under a name the engine does not use.
    names = [c["id"].replace("_", " ").title() for c in reached]
    return {
        "reached": len(reached),
        "total": len(roster),
        "rest": ", ".join(names) + ("." if names else ""),
        "list": ", ".join(names),
        "source": "engine-state-dump",
        "generated_at": states.get("generated_at"),
        # Stated rather than assumed away: a class counts as computed under the
        # dump's seeded input posture, not under a player hand-picking every
        # choice. Carried into the payload so the figure cannot be read as a
        # stronger claim than the instrument makes.
        "input_posture": states.get("input_posture"),
        "excluded": list(V0_6_ROSTER_EXCLUDED),
    }


# ---------------------------------------------------------------------------
# Engine-derived per-book doneness matrix
# ---------------------------------------------------------------------------

# What the three markers actually claim, so the grid is checkable rather than
# decorative. Every one of them is now decided by a number the engine
# produced, and every cell carries that number in its `open_questions` text so
# a reader can check the claim without leaving the page.
#
#   "O"  This book ingests ZERO records of this kind. Not applicable, or not
#        started. Decided by a real count from the book's own compiled tables
#        (`v06_content_state_dump.books[*].kinds`), never by a book's absence
#        from some list.
#   "X"  Records ARE ingested, and the engine's own state says the kind is
#        only partly usable -- some classes do not reach `Computed`, some
#        races do not, some ingested feats compute no mechanical effect.
#        The fraction is engine-derived, never asserted.
#   "✓"  Records are ingested and there is no engine-derived partial signal.
#
# For `spells` and `equipment` the "no partial signal" case is not an evasion:
# an ingested spell/equipment record IS the resolved record (the table entry
# carries the parsed fields; there is no second wiring step that can be half
# done), unlike a feat, whose catalog record and whose computed effect are two
# separate things. The cell still reports the real record count, so an ingest
# regression shows up as a smaller number rather than as a silently-still-green
# tick.
#
# Deliberately NOT ratio-against-the-PCGen-LST: the LST name heuristic in
# `_parse_lst_first_field` over-counts (it yields 674 "spells" for a CRB
# corpus whose real record count is 652), and this repo has already been
# burned by exactly that arithmetic -- `crb/spell_list.rs`'s own doc comment
# records "675 real / 652 ingested = 96.6%" as a *measurement error*, not a
# gap. A denominator that is itself a guess cannot make a marker checkable.

MATRIX_KINDS = ("races", "classes", "spells", "equipment", "feats")

# Singular forms, for reason text that reads like English ("No race records
# are ingested", not "No races records").
_KIND_SINGULAR = {
    "races": "race",
    "classes": "class",
    "spells": "spell",
    "equipment": "equipment",
    "feats": "feat",
}


def _content_kind_counts(content: dict, book_id: str) -> dict:
    """This book's real per-kind ingested record counts, or {} if absent."""
    for book in (content or {}).get("books", []):
        if book.get("id") == book_id:
            return book.get("kinds", {}) or {}
    return {}


def derive_book_matrix(book_id: str, content: dict, classes: list) -> tuple:
    """One book's 5 markers plus the per-column reason text behind each.

    Returns `(matrix, open_questions)`. Every value is derived from `content`
    (the `v06_content_state_dump` payload) and `classes` (the
    `v06_class_state_dump` payload), so no marker in this grid is a literal
    anybody has to remember to update.
    """
    counts = _content_kind_counts(content, book_id)
    if not counts:
        # No engine dump AND no cached dump -- a machine that has never once
        # run the binary. Emitting an all-"O" row here would be the exact bug
        # this function exists to fix: "we could not measure" would render as
        # "nothing has been done". Degrade to the last known-verified shape
        # instead, and say in every cell that it is unmeasured.
        unmeasured = (
            "Engine content dump unavailable on this machine, so this marker "
            "is the last verified shape rather than a fresh measurement. It "
            "is NOT a claim that nothing is ingested."
        )
        return (
            {"races": "X", "classes": "X", "spells": "X", "equipment": "X", "feats": "X"},
            {k: unmeasured for k in MATRIX_KINDS},
        )

    matrix = {}
    reasons = {}

    for kind in MATRIX_KINDS:
        ingested = int(counts.get(kind, 0) or 0)
        if ingested == 0:
            matrix[kind] = "O"
            reasons[kind] = (
                f"No {_KIND_SINGULAR[kind]} records are ingested for this book."
            )
            continue

        if kind == "classes":
            book_classes = [
                c for c in classes
                if _observer.CLASS_BOOK.get(c.get("class_id"))
                == BOOK_ID_TO_CLASS_BOOK.get(book_id, "")
            ]
            full = [c for c in book_classes if c.get("state") == "full"]
            state = aggregate_book_classes_state(
                classes, BOOK_ID_TO_CLASS_BOOK.get(book_id, "")
            )
            matrix[kind] = status_to_marker(state)
            reasons[kind] = (
                f"{len(full)} of {len(book_classes)} ingested classes reach "
                f"HeadlessReceiptStatus::Computed at every level 1-20 "
                f"(engine sweep, v06_class_state_dump). Each class's own "
                f"remaining blockers are on its row in the class matrix."
            )
            continue

        if kind == "races":
            race_states = (content or {}).get("races", [])
            computed = [r for r in race_states if r.get("computed")]
            matrix[kind] = "✓" if race_states and len(computed) == len(race_states) else "X"
            reasons[kind] = (
                f"{len(computed)} of {len(race_states)} ingested races reach "
                f"Computed through the real compute pipeline "
                f"(v06_content_state_dump race probe); "
                f"{ingested} race records ingested."
            )
            continue

        if kind == "feats":
            per_book = (
                (content or {}).get("feat_effects", {}).get("per_book", {}).get(book_id, {})
            )
            records = int(per_book.get("records", ingested) or 0)
            wired = int(per_book.get("effect_wired", 0) or 0)
            matrix[kind] = "✓" if records and wired >= records else "X"
            postures = ", ".join(
                (content or {}).get("feat_effects", {}).get("probe_postures", [])
            )
            reasons[kind] = (
                f"{records} feat records ingested and prerequisite-evaluable, "
                f"but only {wired} of them compute a real mechanical effect: "
                f"adding the other {records - wired} to a character changes no "
                f"number this engine produces. Measured by running the real "
                f"compute pipeline with and without each catalog feat over "
                f"{postures or 'the probe postures'} — a lower bound, since a "
                f"feat needing an unmodelled context (an opponent, an ally, a "
                f"combat action) cannot show up as a delta here."
            )
            continue

        # spells / equipment -- see this section's own comment for why an
        # ingested record is a resolved record for these two kinds.
        matrix[kind] = "✓"
        reasons[kind] = (
            f"{ingested} {kind} records ingested; for this kind the ingested "
            f"table entry is the resolved record, so there is no separate "
            f"wiring step left half-done."
        )

    # A "✓" column has nothing left to say; keeping its reason would put a
    # paragraph of justification under a green tick on every book row.
    #
    # Every surviving reason is stamped with the dump's own `generated_at`.
    # When the engine cannot be rebuilt (a checkout without the binary, cron's
    # restricted PATH, a build failure) the loader keeps serving the last real
    # dump rather than a blank -- which is right, but silently serving an
    # eight-hour-old number as though it were live is how the previous
    # generation of these claims rotted. The stamp makes the age visible
    # wherever the number is.
    stamp = (content or {}).get("generated_at")
    suffix = f" [engine content dump {stamp}]" if stamp else ""
    open_questions = {
        k: v + suffix for k, v in reasons.items() if matrix[k] != "✓"
    }
    return matrix, open_questions


def build_pf1e_dashboard(
    status_text: str,
    risks_text: str,
    risks_path: str,
    usage: dict,
    refreshed: dt.datetime,
    report_text: str = "",
    pi_screen: "_PiScreen | None" = None,
) -> dict:
    # FIX-DASHBOARD-PI (2026-08-17): built ONCE for the whole run (or reused
    # from the caller, which also threads the SAME instance into
    # `build_unit_shards` -- see `main()`) and passed down through every
    # `_book_item_roster`/`_prestige_classes` call below, rather than each
    # one independently re-walking the pinned oracle checkout.
    pi_screen = pi_screen if pi_screen is not None else _PiScreen()
    # 1) Workchannels (auto-discovered)
    workchannels = discover_workchannels(status_text)

    # 2) Full matrix of every class and every race.
    #    Classes come from the engine itself (see DEFAULT_CLASS_STATE_CACHE's
    #    comment above); the SWARM_REPORT.md prose scrape is only the
    #    fallback for when the engine dump cannot be produced at all.
    engine_states = load_engine_class_states()
    if engine_states:
        classes = engine_class_rows(engine_states)
    else:
        print(
            "pf1e-producer: no engine class state available, falling back to "
            "SWARM_REPORT.md prose scrape",
            file=sys.stderr,
        )
        classes = _observer.parse_class_chassis_table(status_text, report_text)
    races = _observer.parse_race_chassis_table(risks_text)

    # 3) Book roster: 4 in-scope + 21 future-state. Build the 5-doneness grid
    #    for each.
    #
    # Every marker in this grid is now derived (see `derive_book_matrix` and
    # the marker-semantics comment above it). Two prior bugs are fixed here,
    # both of the same "a number nobody re-derives goes stale" family:
    #
    #  - `spells` and `equipment` were hand-set to a full tick for every
    #    in-scope book. That was right for CRB/APG/ACG and WRONG for
    #    Bestiary 1, which ingests zero spells and -- per its own module's
    #    register A13 finding -- has no spell-list concept at all. The
    #    dashboard was showing a green "done" tick for content that does not
    #    exist.
    #  - `races` applied the CRB race aggregate to all four books, so APG,
    #    ACG and Bestiary 1 each claimed a full race tick while ingesting no
    #    race records whatsoever.
    #
    # The old `feats` column text (a hand-typed "wired for only 16 of 185")
    # is gone entirely; the number and the roster behind it are now measured
    # by the engine probe on every refresh.
    #
    # `content` is the engine content dump; when it cannot be produced at all
    # the loader returns the last cached dump rather than None, and only a
    # machine that has never once run it falls through to the degraded
    # branch below.
    # Per-book "has any landed engine unit" signal for the FUTURE_STATE_BOOKS
    # status derivation below -- sourced from the same `work-inventory`
    # cache `work_inventory_panel()` already treats as the dashboard's one
    # authority on doneness, not from `_book_item_roster()`.
    #
    # `_book_item_roster()` reads raw PCGen LST corpus files
    # (`_BOOK_LST_FILES`), which exist for every book in `FUTURE_STATE_BOOKS`
    # regardless of whether the engine has ingested anything from them --
    # it answers "does PCGen ship content for this book," not "has this
    # program landed any of it." Using it for `status` produced a false
    # positive: four SD-29 books read `in-progress` with zero ingested
    # units, because their corpus files exist even though nothing has been
    # extracted from them yet. A book has landed real work only if
    # `work-inventory`'s own per-book `by_status` carries at least one unit
    # in a genuinely-attempted status.
    #
    # The status vocabulary (`status_vocabulary` on the same document) has
    # SIX values, and "not landed yet" is two of them, not one: `not-ingested`
    # ("the book IS ingested but the engine holds no record matching this
    # unit's identity -- a real gap inside a started book") AND `not-started`
    # ("the book has no compiled rule set at all -- nothing about this unit
    # has been attempted"). A first pass here excluded only `not-ingested`
    # and silently counted every `not-started` unit as landed, which is the
    # exact inversion of what `not-started` means -- caught by checking
    # against a book known to be genuinely untouched (`bestiary_2`, all
    # units `not-started`) rather than trusting the field name. `unknown`
    # is also excluded: "could not be classified" is not evidence of
    # landed work either way. Landed = `grounded`, `ingested-magnitude`,
    # `text-complete`, or `deferred-with-reason` -- the same broader set
    # the status vocabulary's own "real partial engine progress committed"
    # language for `in-progress` describes, a strictly wider bar than
    # `work_inventory_panel()`'s own narrower `proven` figure
    # (`grounded`/`text-complete` only), which is answering a different
    # question (fully proven) than this one (any real attempt at all).
    _NOT_LANDED_STATUSES = {"not-ingested", "not-started", "unknown"}
    _wi_for_status = load_work_inventory()
    _landed_units_by_book: dict[str, int] = {}
    if _wi_for_status:
        for _b in _wi_for_status.get("books") or []:
            _book_id = _b.get("id")
            if not _book_id:
                continue
            _landed = 0
            for _k in (_b.get("kinds") or {}).values():
                for _status, _count in (_k.get("by_status") or {}).items():
                    if _status not in _NOT_LANDED_STATUSES:
                        _landed += _count or 0
            _landed_units_by_book[_book_id] = _landed_units_by_book.get(_book_id, 0) + _landed

    def _book_has_landed_units(book_id: str) -> bool:
        return _landed_units_by_book.get(book_id, 0) > 0

    content = load_engine_content_state()
    if not content:
        print(
            "pf1e-producer: no engine content state available; per-book matrix "
            "and the Bestiary roster will degrade to record-presence only",
            file=sys.stderr,
        )
    books = []
    for b in IN_SCOPE_BOOKS:
        matrix, open_questions = derive_book_matrix(b["id"], content or {}, classes)
        items = _book_item_roster(b["id"], top_n=5, content=content, pi_screen=pi_screen)
        books.append({
            "id": b["id"],
            "title": b["title"],
            "channel": b["channel"],
            # Every IN_SCOPE_BOOKS entry is a real, actively-worked channel
            # (v0.6 alpha) -- the old `in_scope = channel == "v0.6"` check
            # was always true here since this list only ever contains v0.6
            # books, so it never actually distinguished anything. Kept
            # honest rather than removed silently: "in-progress" reflects
            # what every entry in this specific list already is.
            "status": "in-progress",
            "matrix": matrix,
            # Per-column blocker text (operator directive 2026-07-26: every
            # non-full column must show its real reason, not an empty field).
            # `derive_book_matrix` emits one for every non-"✓" column, each
            # carrying the real numbers the marker was decided from.
            "open_questions": open_questions,
            # Per-book item rosters. SD-22 ingested these in v0 shape; the
            # SD-27 license-stripping cycles (2.0.6-2.0.9) bring them to v1.
            # Until those cycles run, the dashboards' equipment/feats/spells
            # panels show top-N placeholders per kind.
            "items": items,
        })
    for b in FUTURE_STATE_BOOKS:
        # SD28 item 6 fix (2026-08-10, corrected same day): `status` used to
        # be hardcoded "unassigned" for every entry here, unconditionally --
        # true when this list was authored (2026-07-25, before any of these
        # channels existed) and silently wrong the moment a channel
        # launched. Every book below with a real SD-N channel has its own
        # STC under docs/release/ with its own kanban.md/decisions.md/
        # epic-breakdown.md; this does not invent a new "launched" signal,
        # it names the one that already exists as this book's own
        # `channel` string plus whether real ENGINE work has landed for it.
        #
        # First attempt used `_book_item_roster()`'s non-emptiness as the
        # "has real content" signal and was wrong: that function reads raw
        # PCGen LST corpus files, which exist for every book in this list
        # regardless of whether this program has ingested anything from
        # them -- it caught four SD-29 books reading `in-progress` with
        # zero ingested units, because PCGen ships their corpus even though
        # nothing has been extracted from it yet. Corrected to
        # `_book_has_landed_units()`, the same `work-inventory` per-book
        # `by_status` authority `work_inventory_panel()`'s own `proven`
        # figure is built from -- a book only reads `in-progress` once at
        # least one of its units has moved off `not-ingested`.
        #
        # Three real, distinguishable facts, not collapsed into one label:
        #   "unassigned"  -- no SD-N channel at all (does not occur in this
        #                    list today; kept as the honest fallback)
        #   "planned"     -- a real SD-N owns it, but work-inventory shows
        #                    zero landed units for it -- scoped, not started
        #   "in-progress" -- a real SD-N owns it AND work-inventory shows
        #                    at least one landed unit for it
        #
        # NOT fixed here, and worth stating rather than silently leaving:
        # `matrix` below stays a hardcoded all-"O" placeholder for every
        # FUTURE_STATE_BOOKS entry regardless of real progress -- that is
        # a separate, larger gap (this loop never calls
        # `derive_book_matrix`, unlike the IN_SCOPE_BOOKS loop above) and
        # is out of scope for this fix. `items` below is still the raw
        # corpus roster (a legitimate "what PCGen ships" display), kept for
        # that purpose only -- it no longer feeds `status`.
        channel_launched = b["channel"] in ("SD-27", "SD-28", "SD-29", "SD-30")
        items = _book_item_roster(b["id"], top_n=5, pi_screen=pi_screen) if channel_launched else {"equipment": [], "feats": [], "spells": []}
        if not channel_launched:
            status = "unassigned"
        elif _book_has_landed_units(b["id"]):
            status = "in-progress"
        else:
            status = "planned"
        books.append({
            "id": b["id"],
            "title": b["title"],
            "channel": b["channel"],
            "status": status,
            "matrix": {
                "races": "O", "classes": "O", "spells": "O",
                "equipment": "O", "feats": "O",
            },
            # Per-book item rosters for SD-27's first cycle. Future-state
            # books don't have ingested records yet; the panels show empty
            # placeholders. SD-27 cycle 2.1 (ARG) and 2.2 (AG) populate
            # the items as the pre-build runs.
            "items": items,
        })

    # 4) Per-channel HUD data
    channels_data = {
        "v0.6": build_v0_6_channel_data(status_text, risks_text, risks_path, report_text),
    }

    _expanded_races, _arg_race_note = _expand_races(races)

    # Delivered content, counted straight off the engine's own compiled tables.
    # This is the doneness question the dashboard kept failing to answer:
    # `work_inventory` measures how much of the SOURCE CORPUS has been
    # annotated, which is a different question and reads as ~17% while six
    # books are in fact ingested, reaching a player, and green on the reach
    # gate. Ship both, clearly separated.
    _content_books = (content or {}).get("books") or []

    return {
        "generated_at": refreshed.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "generated_by": "pf1e_dashboard_producer",
        "schema_version": 2,
        "usage": usage,
        "workchannels": workchannels,
        "content_state": {
            "available": bool(_content_books),
            "generated_at": (content or {}).get("generated_at"),
            "generated_by": (content or {}).get("generated_by"),
            "books": _content_books,
        },
        "matrix": {
            "classes": [
                {
                    "id": c["class_id"],
                    "book": c["book"],
                    "state": c["state"],
                    "marker": status_to_marker(c["state"]),
                    # Every class in the matrix is v0.6's work (the chassis
                    # breadth is the v0.6 work-effort's payload). The class's
                    # `book` field is the source-book id (CRB/APG/ACG/B1);
                    # the `channel` field is the workchannel id (v0.6).
                    "channel": "v0.6",
                    # Operator directive 2026-07-26: every non-full class must
                    # show its real blocker on the dashboard, not an empty
                    # field. Empty string for "full" classes (no open question).
                    "open_question": c.get("open_question", ""),
                }
                for c in classes
            ],
            "races": _expanded_races,
            # ARG's zero is a ruling, not an omission; carry it so the viewer
            # can say so instead of silently showing seven races and no context.
            "races_note": _arg_race_note,
            "prestige_classes": _prestige_classes(pi_screen=pi_screen),
        },
        "books": books,
        "channels": channels_data,
        "manifests": _seed_manifests(),
        # 5) The corpus-wide work inventory. The three panels above measure the
        #    4 ingested books; this one measures all 25, so a multi-day run has
        #    a denominator that nobody has to maintain by hand.
        "work_inventory": work_inventory_panel(load_work_inventory(), compute_wiring_class_summary()),
        # 6) How the RUN is going, as opposed to where the work stands. The
        #    five panels above are all point-in-time state; this one is the
        #    only thing here with a memory of what went wrong on the way.
        "retrospective": retro_panel(load_retro_summary()),
    }


def _load_existing_owner_state(out_path: str) -> dict:
    """Read the existing JSON (if any) and extract owner-managed fields
    that the producer should preserve across runs. Owner-managed fields
    are populated by the lead (v0.6_class_breadth manifest items) and
    the orchestrator (sd27/sd28/sd29/sd30 manifest items, channels for
    SD-27+).

    The producer regenerates derived fields (matrix, books[*].items,
    workchannels) on every run; those should not be preserved. The
    merge keeps owner-managed fields intact.

    Returns a dict with `manifests` (preserved items + stats) and
    `channels` (preserved SD-27+ channel data). v0.6 channel data is
    NOT preserved because the producer regenerates it from SWARM_STATUS.md.
    """
    # Read the canonical file, but fall back to the last known-good snapshot
    # if it is missing or unparseable. Returning empty owner state here is
    # destructive -- the merge would then overwrite the lead's manifest items
    # and the orchestrator's channel data with nothing. That is exactly what
    # happened while the publish path was non-atomic (see _atomic_write_json).
    prior = None
    for candidate in (out_path, out_path + ".last-good"):
        if not os.path.exists(candidate):
            continue
        try:
            with open(candidate, "r", encoding="utf-8") as f:
                prior = json.load(f)
            break
        except (OSError, json.JSONDecodeError) as exc:
            print(
                f"pf1e-producer: owner-state read failed for {candidate}: {exc}",
                file=sys.stderr,
            )
    if prior is None:
        return {"manifests": {}, "channels": {}}

    # Preserve each manifest's items + stats. Manifest ENTRY STRUCTURES
    # (id, workchannel, scope, managed_by) come from _seed_manifests();
    # we only need to preserve the items and stats.
    manifests = {}
    for mid, m in (prior.get("manifests") or {}).items():
        manifests[mid] = {
            "items": list(m.get("items", []) or []),
            "stats": dict(m.get("stats", {}) or {}),
        }

    # Preserve channel data for SD-27+ channels (orchestrator-managed).
    # v0.6 channel data is regenerated from SWARM_STATUS.md by the producer.
    channels = {}
    for ch_id, ch_data in (prior.get("channels") or {}).items():
        if ch_id != "v0.6":
            channels[ch_id] = ch_data

    return {"manifests": manifests, "channels": channels}


def _merge_owner_state(data: dict, prior_state: dict) -> dict:
    """Merge owner-managed fields from prior_state into data.

    For each manifest in data["manifests"]:
      - if prior_state has items for this manifest, use them
      - else use the freshly-seeded empty items

    For each channel in prior_state["channels"]:
      - if data["channels"] does not have this channel, add it
    """
    # Merge manifest items + stats. Keep the seed's entry structure
    # (id, workchannel, scope, managed_by) but replace items + stats
    # with prior values if present.
    for mid, m in data.get("manifests", {}).items():
        if mid in prior_state["manifests"]:
            prior = prior_state["manifests"][mid]
            if prior.get("items") is not None:
                m["items"] = prior["items"]
            if prior.get("stats") is not None:
                m["stats"] = prior["stats"]
    # Add prior channels not in data["channels"] (SD-27+).
    for ch_id, ch_data in prior_state["channels"].items():
        if ch_id not in data.get("channels", {}):
            data.setdefault("channels", {})[ch_id] = ch_data
    return data


def _expand_races(races: list) -> tuple:
    """Expand the matrix.races list from CRB-only (7 races) to the full
    roster: 7 CRB Core races + 30 ARG races (28 Featured + 2 Uncommon extras).

    The parser gives us 7 CRB races with their support state. ARG's races
    are not yet in the parser's source — they're pre-emptively added here
    as `untouched` placeholders so the dashboard's race matrix shows the
    full ARG roster ahead of the SD-27 ingest.

    Per-race channel assignment:
    - CRB Core 7: channel = "v0.6" (existing in-scope breadth)
    - ARG Featured 28 + ARG Uncommon 2: channel = "SD-27" (operator-pinned)
    """
    crb_races = []
    for r in races:
        crb_races.append({
            "id": r["race_id"],
            "name": r["race_id"].replace("-", " ").title(),
            "book": "CRB",
            "state": "full" if r["status"] == "fully-supported" else
                      "in-progress" if r["status"] == "human-only" else
                      "untouched",
            "marker": status_to_marker(
                "full" if r["status"] == "fully-supported" else
                "in-progress" if r["status"] == "human-only" else
                "untouched"
            ),
            "channel": "v0.6",
        })

    arg_featured = [
        ("aasimar", "Aasimar"),
        ("catfolk", "Catfolk"),
        ("changeling", "Changeling"),
        ("dhampir", "Dhampir"),
        ("duergar", "Duergar"),
        ("fetchling", "Fetchling"),
        ("gillman", "Gillman"),
        ("goblin", "Goblin"),
        ("goliath", "Goliath"),
        ("hobgoblin", "Hobgoblin"),
        ("ifrit", "Ifrit"),
        ("kitsune", "Kitsune"),
        ("kobold", "Kobold"),
        ("merfolk", "Merfolk"),
        ("nagaji", "Nagaji"),
        ("orc", "Orc"),
        ("oread", "Oread"),
        ("ratfolk", "Ratfolk"),
        ("samsaran", "Samsaran"),
        ("strix", "Strix"),
        ("suli", "Suli"),
        ("sylph", "Sylph"),
        ("tengu", "Tengu"),
        ("tiefling", "Tiefling"),
        ("undine", "Undine"),
        ("vanara", "Vanara"),
        ("vishkanya", "Vishkanya"),
        ("wayang", "Wayang"),
    ]
    arg_uncommon_extras = [
        ("grippli", "Grippli"),
        ("svirfneblin", "Svirfneblin"),
    ]

    # The 30 names above are transcribed from ARG's table of contents. They are
    # NOT race rows in this corpus, and rendering them as 30 `untouched` units
    # was fiction that made SD-27 look like it had 30 open items.
    #
    # `decisions.md §25` rules that ARG declares zero races and zero racial
    # defaults; `race_catalog.rs` asserts it in a test
    # (`...filter(|e| e.book == BOOK_ARG).count() == 0`) and calls it "a measured
    # zero". ARG's actual race contribution is 153 alternate racial traits,
    # which are deliberately not catalog rows. Several of these names (aasimar,
    # duergar, goblin, hobgoblin, kobold, merfolk, orc, svirfneblin, tengu,
    # tiefling) ARE real engine races -- they belong to Bestiary 1, not ARG.
    #
    # So: emit no ARG race rows, and carry the ruling forward as a note so the
    # zero reads as decided rather than forgotten.
    arg_race_note = {
        "book": "advanced_race_guide",
        "channel": "SD-27",
        "declared_races": 0,
        "ruling": "decisions.md §25",
        "note": (
            f"ARG declares zero races and zero racial defaults (decisions.md §25) — "
            f"a measured zero, not a gap. Its race work is 153 alternate racial "
            f"traits, which are deliberately not catalog rows. The {len(arg_featured) + len(arg_uncommon_extras)} "
            f"race names in ARG's table of contents are sourced from other books "
            f"(Bestiary 1 carries eleven of them)."
        ),
    }

    return crb_races, arg_race_note


# Per-book LST file mapping for extracting items.
# Each kind has 1+ LST files. Equipment LSTs are split across arms_armor, general, magic_items.
# For Bestiary 1, monsters come from the SD-22 cache at data/corpus/beastiary/monster/.
_PCGEN_ROOT = os.path.expanduser("~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game")
_BOOK_LST_DIRS = {
    "core_rulebook": "core_rulebook",
    "advanced_players_guide": "advanced_players_guide",
    "advanced_class_guide": "advanced_class_guide",
    "bestiary_1": "bestiary",
    "advanced_race_guide": "advanced_race_guide",
    "adventurers_guide": "adventurers_guide",
    "pathfinder_unchained": "pathfinder_unchained",
    "bestiary_2": "bestiary_2",
    "bestiary_3": "bestiary_3",
    "bestiary_4": "bestiary_4",
    "bestiary_5": "bestiary_5",
    "bestiary_6": "bestiary_6",
    "bonus_bestiary": "bonus_bestiary",
    "horror_adventures": "horror_adventures",
    "monster_codex": "monster_codex",
    "mythic_adventures": "mythic_adventures",
    "occult_adventures": "occult_adventures",
    "ultimate_campaign": "ultimate_campaign",
    "ultimate_combat": "ultimate_combat",
    "ultimate_equipment": "ultimate_equipment",
    "ultimate_intrigue": "ultimate_intrigue",
    "ultimate_magic": "ultimate_magic",
    "ultimate_wilderness": "ultimate_wilderness",
}
_BOOK_LST_FILES = {
    "core_rulebook": {
        "equipment": ["cr_equip_arms_armor.lst", "cr_equip_general.lst", "cr_equip_magic_items.lst"],
        "feats": ["cr_feats.lst"],
        "spells": ["cr_spells.lst"],
    },
    "advanced_players_guide": {
        "equipment": ["apg_equip_arms_armor.lst", "apg_equip_general.lst", "apg_equip_magic_items.lst"],
        "feats": ["apg_feats.lst"],
        "spells": ["apg_spells.lst"],
    },
    "advanced_class_guide": {
        # The ACG does not split equipment across arms_armor/general/
        # magic_items the way the CRB and APG do -- it ships one
        # `acg_equip.lst`. This used to be an empty list, so the equipment
        # panel showed nothing for a book whose engine tables hold 269 real
        # ingested equipment records (v06_content_state_dump). An empty
        # roster next to a green matrix tick is the same "absence rendered as
        # a claim" failure as the Bestiary monsters.
        "equipment": ["acg_equip.lst"],
        "feats": ["acg_feats.lst"],
        "spells": ["acg_spells.lst"],
    },
    "bestiary_1": {
        "equipment": ["b1_equip_arms_armor.lst", "b1_equip_general.lst", "b1_equip_magic_items.lst"],
        # Bestiary 1 has no feats LST file (monsters are defined in the .pcc
        # config, not LSTs). Monsters come from the SD-22 cache.
        "feats": [],
        "monsters": None,  # special: read from SD-22 cache
    },
    "advanced_race_guide": {
        "feats": ["arg_feats.lst"],
        "spells": ["arg_spells.lst"],
    },
    "adventurers_guide": {
        "feats": ["ag_feats.lst"],
        "spells": ["ag_spells.lst"],
        "prestige_classes": ["ag_classes.lst"],
    },
    "pathfinder_unchained": {
        "feats": ["pu_feats.lst"],
        "spells": ["pu_spells.lst"],
    },
    # Bestiary books have no .lst files for spells/feats/equipment; monsters
    # come from the SD-22 cache (future SD-29 cycles).
    "bestiary_2": {"monsters": None},
    "bestiary_3": {"monsters": None},
    "bestiary_4": {
        "feats": ["b4_feats.lst"],
        "equipment": ["b4_equip_arms_armor.lst", "b4_equip_magic_items.lst"],
    },
    "bestiary_5": {
        "feats": ["b5_feats.lst"],
        "spells": ["b5_spells_modified.lst"],
    },
    "bestiary_6": {
        "feats": ["b6_feats.lst"],
        "spells": ["b6_spells.lst"],
    },
    "bonus_bestiary": {"monsters": None},
    "horror_adventures": {
        "equipment": ["ha_equip_arms_armor.lst", "ha_equip_general.lst", "ha_equip_magic_items.lst"],
        "feats": ["ha_feats.lst"],
        "spells": ["ha_spells.lst"],
    },
    "monster_codex": {
        "equipment": ["mc_equip_arms_armor.lst", "mc_equip_general.lst", "mc_equip_magic_items.lst"],
        "feats": ["mc_feats.lst"],
    },
    "mythic_adventures": {
        "equipment": ["ma_equip.lst"],
        "feats": ["ma_feats.lst"],
        "spells": ["ma_spells.lst"],
    },
    "occult_adventures": {
        "equipment": ["oa_equip.lst"],
        "feats": ["oa_feats.lst"],
        "spells": ["oa_spells.lst"],
    },
    "ultimate_campaign": {
        "feats": ["uca_feats.lst"],
    },
    "ultimate_combat": {
        "equipment": ["uc_equip_arms_armor.lst", "uc_equip_general.lst", "uc_equip_magic_items.lst"],
        "feats": ["uc_feats.lst"],
        "spells": ["uc_spells.lst"],
    },
    "ultimate_equipment": {
        "equipment": ["ue_equip_arms_armor.lst", "ue_equip_general.lst", "ue_equip_magic_items.lst"],
        "feats": ["ue_feats.lst"],
        "spells": ["ue_spells.lst"],
    },
    "ultimate_intrigue": {
        "feats": ["ui_feats.lst"],
        "spells": ["ui_spells.lst"],
    },
    "ultimate_magic": {
        "equipment": ["um_equip_arms_armor.lst", "um_equip_general.lst", "um_equip_magic_items.lst"],
        "feats": ["um_feats.lst"],
        "spells": ["um_spells.lst"],
    },
    "ultimate_wilderness": {
        "equipment": ["uw_equip_arms_armor.lst", "uw_equip_general.lst", "uw_equip_magic_items.lst"],
        "feats": ["uw_feats.lst"],
        "spells": ["uw_spells.lst"],
    },
}

class _PiScreen:
    """Bundles the shared, built-once declared-PI inputs every name-emitting
    call site in this module needs: `pi_redaction`'s full-oracle name index
    (flat and per-book) plus the shared, reviewed allow-list
    (`scripts/site/pi_substring_allowlist.py` -- the SAME file
    `build_public_status.py`'s public-status projection already uses, not a
    second one). Built ONCE per producer run (`build_pf1e_dashboard`/
    `main`) and threaded down through `_book_item_roster`/
    `_parse_lst_first_field`/`_prestige_classes`/`build_unit_shards`, rather
    than each call site independently re-walking the pinned oracle
    checkout (`build_unit_shards`'s own existing comment already notes that
    walk's real cost).

    FIX-DASHBOARD-PI (2026-08-17): `build_unit_shards`'s pre-existing
    EXACT-match passes only catch a name that IS, verbatim, a declared-PI
    name. `screen()` adds the WORD-BOUNDARY embed check
    `build_public_status.py::redact_for_display` already uses for the
    public status projection (`pi_redaction.find_declared_pi_word_matches`,
    book-scoped union global, gated by the shared allow-list) -- the
    dashboard feed never had this layer at all, which is exactly how
    `Bow of Erastil`, `Witherfang`, `Legendsbane`, `Helm of the Serpent
    King` and the `Rivethun` spells shipped raw: none of those ships its
    OWN `NAMEISPI:YES` token on the row that names it (the declaration
    lives on a DIFFERENT row for the same object, or no row declares the
    exact string a `.COPY=` directive created), so a row-local or
    exact-match check alone is structurally blind to them."""

    def __init__(self, declared_pi_names=None, declared_pi_name_books=None, allowlist_index=None):
        self.names = (
            declared_pi_names if declared_pi_names is not None
            else pi_redaction.build_declared_pi_name_index()
        )
        self.by_length = sorted(self.names, key=len, reverse=True)
        name_to_books = (
            declared_pi_name_books if declared_pi_name_books is not None
            else pi_redaction.build_declared_pi_name_book_index()
        )
        self.book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
        self.allowlist_index = (
            allowlist_index if allowlist_index is not None
            else pi_substring_allowlist.build_allowlist_index()
        )

    def screen(self, name, book=None):
        """Return `name` unchanged, or `pi_redaction.REDACTED_PI_MARKER` if
        it carries a declared-PI name -- exact match first (cheap, catches
        the common case), then a word-boundary embed check (book-scoped
        union global) gated by the shared allow-list.

        `book=None` means there is no single book to scope or allow-list
        against (a `unit_index` category label aggregated across a whole
        kind, not one book-scoped object) -- the GLOBAL word-boundary check
        still runs, gated by `is_allowlisted_for_any_book` instead of the
        per-item `is_allowlisted`."""
        if not isinstance(name, str) or not name or name == pi_redaction.REDACTED_PI_MARKER:
            return name
        if name.strip() in self.names:
            return pi_redaction.REDACTED_PI_MARKER
        book_names = self.book_declared.get(book, ()) if book else ()
        # case_insensitive=True (FIX-DASHBOARD-PI): a title-shaped declared
        # name that itself begins with an ordinary word (`"The Serpent
        # King"`) naturally loses its capital when embedded mid-sentence
        # (`"Helm of the Serpent King"`) -- see
        # `find_declared_pi_word_matches`'s own docstring for the full
        # rationale. Scoped to THIS screen only; `build_public_status.py`'s
        # own case-sensitive calls are untouched.
        matches = set(pi_redaction.find_declared_pi_word_matches(name, book_names, case_insensitive=True))
        matches.update(pi_redaction.find_declared_pi_word_matches(name, self.by_length, case_insensitive=True))
        if not matches:
            return name
        if book:
            allowed = pi_substring_allowlist.is_allowlisted(name, book, self.allowlist_index)
        else:
            allowed = pi_substring_allowlist.is_allowlisted_for_any_book(name, self.allowlist_index)
        return name if allowed else pi_redaction.REDACTED_PI_MARKER


# In-memory cache of parsed LST contents, keyed (path, book_id) -- book_id
# is part of the key because the SAME physical file can legitimately screen
# differently for a different book context (FIX-DASHBOARD-PI, 2026-08-17).
_LST_CACHE = {}


def _parse_lst_first_field(path: str, book_id: str | None = None, pi_screen: "_PiScreen | None" = None) -> list:
    """Parse an LST file and return the unique first-field display names.
    Filters out modifier lines (.MOD), category markers, and inline stat
    definitions (which start with KEY:, TYPE:, STATS:, DEFINE:, BONUS:, etc.).

    `book_id`/`pi_screen` (FIX-DASHBOARD-PI, 2026-08-17): every name this
    function returns is a public roster's ONLY source for that row
    (`_book_item_roster`, `_prestige_classes`'s `ag_variants` both feed
    straight from here into `site/dashboard/**`) -- see `_PiScreen`'s own
    docstring for why the row-local NAMEISPI check alone (kept below,
    unchanged, as the cheap first layer) is not enough on its own.
    `pi_screen=None` builds one locally -- real producer call sites pass one
    built ONCE per run; a test with no real oracle checkout to worry about
    is still free to call this with no arguments at all.
    """
    cache_key = (path, book_id)
    if cache_key in _LST_CACHE:
        return _LST_CACHE[cache_key]
    seen = set()
    names = []
    if not os.path.exists(path):
        _LST_CACHE[cache_key] = names
        return names
    screen = pi_screen if pi_screen is not None else _PiScreen()
    SKIP_PREFIXES = (
        "KEY:", "TYPE:", "STATS:", "DEFINE:", "BONUS:", "PRE", "OUTPUTNAME:",
        "CHOOSE:", "STACK:", "MULT:", "DESC:", "SOURCEPAGE:", "BENEFIT:",
        "SPECIFY:", "TEMPLATE:", "ABILITY:", "MONSTERCLASS:", "CRITTER:",
        "SIZE:", "MOVE:", "REACH:", "AC", "HD", "HP", "SAVE", "INIT",
        "SPELL", "FREQ:", "SKILL", "WEAPON", "FUMBLE", "AUTO:",
        "SUBCLASSLEVEL:", "SUBCLASS:HD", "SUBCLASS:HP", "SUBCLASS:SKILL",
        "ABILITYPOOL", "ABILITYSCORE", "GAME", "VISION", "RACESUBNAME",
        "FAVOREDCLASS", "INITIATIVE", "STARTPACK", "DAMAGEMELEE",
    )
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n").rstrip("\r")
            if not line or line.startswith("#") or line.startswith("SOURCELONG") or line.startswith("SOURCEWEB") or line.startswith("SOURCESHORT") or line.startswith("SOURCEDATE"):
                continue
            parts = re.split(r"\t+", line, maxsplit=1)
            if not parts:
                continue
            name = parts[0].strip()
            if name.startswith("CLASS:"):
                name = name[6:]
            if name.startswith("SUBCLASS:"):
                name = name[9:]
            # Reject purely-numeric or empty names, MOD-suffixed modifier lines,
            # section markers like "###Block: ...", and stat-block lines.
            if not name:
                continue
            if name.isdigit():
                continue
            if name.startswith("###Block") or name.startswith("Block:"):
                continue
            if name.startswith("★") or name.startswith("◆"):
                continue
            if (len(name) <= 200 and not name.endswith(".MOD")
                    and not any(name.startswith(p) for p in SKIP_PREFIXES)):
                # FIX-DASHBOARD-PI (2026-08-17): the raw first field can
                # carry PCGen row-OPERATOR syntax (`Composite Longbow
                # (Base).COPY=Bow of Erastil` -- a `.COPY=` row CREATES a
                # new object named after the right-hand side, not the
                # left-hand source key it copies from). `clean_first_field`
                # (shared with `pi_redaction.py`'s own index-builder, so
                # both agree on what "the name" is) extracts the real
                # display name; a public roster must never leak PCGen's own
                # patch-directive syntax regardless of PI status.
                display = pi_redaction.clean_first_field(parts[0])
                if not display:
                    continue
                if display not in seen:
                    seen.add(display)
                    # Decision 12 (2026-08-17): this function reads a real
                    # PCGen row and is a public roster's ONLY source for
                    # that row's name (`_book_item_roster`,
                    # `_prestige_classes`'s `ag_variants` both feed straight
                    # from here into `site/dashboard/**`). Dedup above keys
                    # on the REAL display name, so two different declared-PI
                    # rows still each hold their own slot; only the
                    # DISPLAYED string is withheld, per "withhold the name,
                    # keep the row."
                    #
                    # Layer 1 (cheap, unchanged): THIS row's own
                    # NAMEISPI:YES token -- catches a fresh, non-operator
                    # row that declares PI on itself.
                    # Layer 2 (`screen.screen`, FIX-DASHBOARD-PI): the
                    # full-oracle exact-match AND word-boundary checks --
                    # catches the declaration living on a DIFFERENT row (a
                    # `.MOD` for the same object, e.g. `Bow of Erastil`,
                    # `Witherfang`, `Legendsbane`) or a genuine embed with
                    # no row of its own at all (`Rivethun Calm Spirit`,
                    # created by a `.COPY=` directive that never declares
                    # PI on its own line).
                    name_is_pi, _ = pi_redaction.declared_product_identity(
                        pi_redaction.parse_row_tokens(line)
                    )
                    if name_is_pi:
                        names.append(pi_redaction.REDACTED_PI_MARKER)
                    else:
                        names.append(screen.screen(display, book_id))
    _LST_CACHE[cache_key] = names
    return names


def _load_beastiary_monsters(content: dict | None = None) -> list:
    """Bestiary 1's monster roster, each entry carrying its REAL engine state.

    **The bug this fixes** (operator report, 2026-07-29: "Bestiary looks to be
    not touched -- all 41 beasts show not started"). This function used to
    return 41 bare name strings. The viewer derives an item's state by looking
    the item id up in the work-effort manifests; monsters have no manifest
    entries, so every lookup missed and every monster rendered the
    no-entry default, which reads "Not started". The engine meanwhile holds 41
    real, resolvable `MonsterStatBlock` records. The panel was not reporting a
    measurement at all -- it was reporting the absence of one.

    **The true monster count is 41**, and the two independent surfaces agree:
    `MonsterId::ALL` has exactly 41 variants (pinned by
    `beastiary1/mod.rs`'s own
    `all_has_exactly_the_41_real_monsters_with_no_duplicates` test) and
    `data/corpus/beastiary/monster/` holds exactly 41 JSON records. Do NOT
    count `name:` fields in the `monster_subset_*.rs` modules to check this --
    a monster's `natural_attacks` carry `name:` fields of their own (Claw,
    Bite), so that count is much larger than the roster and means something
    else entirely.

    Each returned item is a dict, not a string, so the state travels with the
    item instead of being guessed at by the viewer:
      - `done`        the monster resolves through the real `monster_resolve`
                      entry point AND has its JSON corpus cache record
      - `in-progress` present in one of those two surfaces but not both
    A monster in neither surface cannot appear here at all, so this roster can
    never overstate what exists.

    Falls back to the corpus JSON directory (names only, state `in-progress`)
    when no engine dump is available, because a book with 41 real ingested
    monsters must never render as untouched -- that is the entire bug.
    """
    monsters = (content or {}).get("monsters")
    if monsters:
        out = []
        for m in monsters:
            has_engine = bool(m.get("engine_stat_block"))
            has_corpus = bool(m.get("corpus_record"))
            state = "done" if (has_engine and has_corpus) else "in-progress"
            attacks = m.get("natural_attacks") or []
            detail = (
                f"CR {m.get('challenge_rating')} · size {m.get('size')} · "
                f"{m.get('race_type')}"
                + (f" ({m['race_subtype']})" if m.get("race_subtype") else "")
                + f" · speed {m.get('speed_ft')} ft · {m.get('source_page')}"
                + (f" · natural attacks: {', '.join(attacks)}" if attacks else
                   " · no natural-attack tokens on the corpus row")
            )
            out.append({
                "name": m.get("name"),
                "id": m.get("key"),
                "state": state,
                "reason": "" if state == "done" else (
                    "resolves in the engine but has no JSON corpus cache record"
                    if has_engine else
                    "has a JSON corpus cache record but does not resolve in the engine"
                ),
                "detail": detail,
            })
        return out

    path = os.path.expanduser("~/workspace/repos/codex/data/corpus/beastiary/monster")
    fallback = []
    if not os.path.isdir(path):
        return fallback
    for f in sorted(os.listdir(path)):
        if f.endswith(".json"):
            try:
                with open(os.path.join(path, f)) as fh:
                    d = json.load(fh)
                    name = d.get("data", {}).get("name")
                    if name:
                        fallback.append({
                            "name": name,
                            "id": d.get("data", {}).get("id") or name,
                            "state": "in-progress",
                            "reason": (
                                "engine content dump unavailable; state shown "
                                "from the JSON corpus cache record alone"
                            ),
                            "detail": "",
                        })
            except Exception:
                pass
    return fallback


def _book_item_roster(book_id: str, top_n: int = 5, content: dict | None = None,
                      pi_screen: "_PiScreen | None" = None) -> dict:
    """Return per-kind item rosters for a book, extracted from PCGen LST corpora.
    The dashboard panel renders the first N items per kind (default 5).
    Storage is the full list; top_n is the panel display limit.

    Items are plain name strings for every kind except Bestiary 1's monsters,
    which carry their own engine-derived state (see `_load_beastiary_monsters`).

    `pi_screen` (FIX-DASHBOARD-PI, 2026-08-17): built once here if not
    supplied, then reused across every file this ONE book reads -- not
    rebuilt per file, and not left to each `_parse_lst_first_field` call to
    build its own (see `_PiScreen`'s own docstring for why that walk is
    expensive). `build_pf1e_dashboard` builds one ONCE for the whole run
    and passes it to every `_book_item_roster` call.
    """
    out = {"equipment": [], "feats": [], "spells": [], "monsters": [], "races": []}
    if book_id not in _BOOK_LST_FILES:
        return out
    screen = pi_screen if pi_screen is not None else _PiScreen()
    book_dir = os.path.join(_PCGEN_ROOT, _BOOK_LST_DIRS[book_id])
    kind_files = _BOOK_LST_FILES[book_id]
    for kind, files in kind_files.items():
        if files is None:
            # special: bestiary monsters, engine state + SD-22 corpus cache
            if kind == "monsters" and book_id == "bestiary_1":
                out["monsters"] = _load_beastiary_monsters(content)
            continue
        names = []
        for f in files:
            names.extend(_parse_lst_first_field(os.path.join(book_dir, f), book_id=book_id, pi_screen=screen))
        # Dedupe preserving order
        seen = set()
        unique = []
        for n in names:
            if n not in seen:
                seen.add(n)
                unique.append(n)
        out[kind] = unique
    return out


def _prestige_classes(pi_screen: "_PiScreen | None" = None) -> list:
    """Return the prestige-class roster for the dashboard's Classes (Prestige)
    sub-panel. With AG dropped (2026-07-26), the AG prestige roster is gone.
    Pathfinder Unchained (now in SD-27) has 3 unchained class variants
    (Unchained Rogue, Unchained Monk, Unchained Summoner) which are class
    variants, not new chassis, but are surfaced here as prestige-class-style
    items for the dashboard. SD-28+ adds Ultimate-line prestige classes.

    Each prestige class carries a `[PRE-WIRE]` flag in its metadata — the
    dashboard renders this as a distinct pre-wire row in the todo table.
    The pre-wire is the operator-pinned early-epic prerequisite: prestige
    classes have nowhere to land in `src/rules_core/rules_tables/` until
    a rules-table slot is wired.

    Per-class shape: id (snake_case), name (Title Case), book, channel,
    pre_wire_required (always True for prestige), todo_template (5-task
    list with the pre-wire as task 1).
    """
    # Pathfinder Unchained's 3 unchained class variants. These are class
    # variants, not new base chassis, but the dashboard surfaces them as
    # prestige-class-style items so the operator sees them in the panel.
    pu_variants = [
        ("unchained_rogue", "Unchained Rogue", "Pathfinder Unchained", "SD-27"),
        ("unchained_monk", "Unchained Monk", "Pathfinder Unchained", "SD-27"),
        ("unchained_summoner", "Unchained Summoner", "Pathfinder Unchained", "SD-27"),
    ]

    # Adventurer's Guide (SD-30) prestige classes. Extracted from
    # ag_classes.lst at producer run time; re-introduced 2026-07-26 when
    # AG moved from SD-27 (replaced by Pathfinder Unchained) to SD-30.
    ag_path = os.path.join(_PCGEN_ROOT, "adventurers_guide", "ag_classes.lst")
    ag_names = _parse_lst_first_field(ag_path, book_id="adventurers_guide", pi_screen=pi_screen)
    ag_names = [n for n in ag_names if n and n != "Wizard"]
    ag_variants = [(n, n, "Adventurer's Guide", "SD-30") for n in ag_names]

    # Ultimate-line prestige classes (SD-28).
    # Format: (id, name, book, channel)
    ultimate_prestige = [
        ("arcane_archer", "Arcane Archer", "Ultimate Combat", "SD-28"),
        ("arcane_trickster", "Arcane Trickster", "Ultimate Magic", "SD-28"),
        ("assassin", "Assassin", "Ultimate Intrigue", "SD-28"),
        ("dragon_fury", "Dragon Fury", "Ultimate Combat", "SD-28"),
        ("duelist", "Duelist", "Ultimate Combat", "SD-28"),
        ("eldritch_knight", "Eldritch Knight", "Ultimate Magic", "SD-28"),
        ("hierophant", "Hierophant", "Ultimate Magic", "SD-28"),
        ("horizon_walker", "Horizon Walker", "Ultimate Wilderness", "SD-28"),
        ("loremaster", "Loremaster", "Ultimate Magic", "SD-28"),
        ("mystic_theurge", "Mystic Theurge", "Ultimate Magic", "SD-28"),
        ("nature_friend", "Nature Friend", "Ultimate Wilderness", "SD-28"),
        ("rage_prophet", "Rage Prophet", "Ultimate Wilderness", "SD-28"),
        ("stalwart_defender", "Stalwart Defender", "Ultimate Combat", "SD-28"),
        ("trickster", "Trickster", "Ultimate Intrigue", "SD-28"),
        ("warpriest", "Warpriest (UC)", "Ultimate Combat", "SD-28"),
        ("winter_witch", "Winter Witch", "Ultimate Wilderness", "SD-28"),
    ]

    out = []
    # Helper to build a prestige-class entry with the standard 5-task todo template.
    def _prestige_entry(cls_id, name, book, channel):
        return {
            "id": cls_id,
            "name": name,
            "book": book,
            "channel": channel,
            "pre_wire_required": True,
            "kind": "prestige_class",
            "todo_template": [
                {"task": "[PRE-WIRE] Create rules-table slot", "kind": "pre_wire", "status": "queued"},
                {"task": "License review (OGL/PI check)", "kind": "license", "status": "queued"},
                {"task": "Extract from LST", "kind": "extract", "status": "queued"},
                {"task": "Build dispatch (chassis + features)", "kind": "build", "status": "queued"},
                {"task": "Reach Computed", "kind": "reach", "status": "queued"},
            ],
        }
    for cls_id, name, book, channel in pu_variants:
        out.append(_prestige_entry(cls_id, name, book, channel))
    for cls_id, name, book, channel in ag_variants:
        # For AG classes, slugify the name into the id.
        slug = (cls_id or name).lower().replace(" ", "_").replace("(", "").replace(")", "").replace("/", "_")
        out.append(_prestige_entry(slug, name, book, channel))
    for cls_id, name, book, channel in ultimate_prestige:
        out.append(_prestige_entry(cls_id, name, book, channel))
    return out


def _seed_manifests() -> dict:
    """Seed the top-level manifests dict with the canonical manifests the
    producer maintains. Each manifest starts empty (no items) and the
    lead or orchestrator populates items + stats as work proceeds.

    The producer seeds manifest ENTRY STRUCTURES only — it does not write
    items. The lead owns the v0.6_class_breadth manifest; the orchestrator
    owns sd27_book_pre_build.
    """
    return {
        "v0.6_class_breadth": {
            "manifest_id": "v0.6_class_breadth",
            "workchannel": "v0.6",
            "scope": "27 CRB/APG/ACG/Ultimate classes + chassis/breadth spires",
            "schema_version": 2,
            "managed_by": "v0.6_lead",
            "stats": {"pending": 0, "in_progress": 0, "complete": 0, "blocked": 0, "failed": 0},
            "items": [],
        },
        "sd27_book_pre_build": {
            "manifest_id": "sd27_book_pre_build",
            "workchannel": "SD-27",
            "scope": "Advanced Race Guide + Pathfinder Unchained (2 books; 4 stages per book: license prep, pre-build, verify, parity)",
            "schema_version": 2,
            "managed_by": "orchestrator",
            "stats": {"pending": 0, "in_progress": 0, "complete": 0, "blocked": 0, "failed": 0},
            "items": [],
        },
        "sd28_book_pre_build": {
            "manifest_id": "sd28_book_pre_build",
            "workchannel": "SD-28",
            "scope": "All 6 Ultimate books (Ultimate Campaign, Ultimate Combat, Ultimate Equipment, Ultimate Intrigue, Ultimate Magic, Ultimate Wilderness); operator-gated on SD-27 closing cleanly",
            "schema_version": 2,
            "managed_by": "orchestrator",
            "stats": {"pending": 0, "in_progress": 0, "complete": 0, "blocked": 0, "failed": 0},
            "items": [],
        },
        "sd29_book_pre_build": {
            "manifest_id": "sd29_book_pre_build",
            "workchannel": "SD-29",
            "scope": "Remaining 5 Bestiary books (Bestiary 2-6) + Bonus Bestiary + Monster Codex (7 books total); operator-gated on SD-28 closing cleanly",
            "schema_version": 2,
            "managed_by": "orchestrator",
            "stats": {"pending": 0, "in_progress": 0, "complete": 0, "blocked": 0, "failed": 0},
            "items": [],
        },
        "sd30_book_pre_build": {
            "manifest_id": "sd30_book_pre_build",
            "workchannel": "SD-30",
            "scope": "Adventurer's Guide + Mythic Adventures + Occult Adventures + Horror Adventures (4 books total); operator-gated on SD-29 closing cleanly",
            "schema_version": 2,
            "managed_by": "orchestrator",
            "stats": {"pending": 0, "in_progress": 0, "complete": 0, "blocked": 0, "failed": 0},
            "items": [],
        },
        "project_matrix": {
            "manifest_id": "project_matrix",
            "workchannel": "all",
            "scope": "Cross-workchannel matrix: 27 classes + 7 races + 25 books (4 in-scope + 21 future-state)",
            "schema_version": 2,
            "managed_by": "operator",
            "stats": {"pending": 0, "in_progress": 0, "complete": 0, "blocked": 0, "failed": 0},
            "items": [],
        },
    }


# ---------------------------------------------------------------------------
# Atomic publish (2026-08-01)
# ---------------------------------------------------------------------------
#
# The canonical JSON path is what PF1e-dashboard.html fetches on load, so it
# must never be observed empty or half-written. This used to be a plain
# Path.write_text(), which truncates in place; a reader arriving between the
# truncate and the flush got a 0-byte body, and the viewer's single try/catch
# turns that into a stuck "Failed to load dashboard data" box.
#
# The window was not theoretical. The */5 cron schedule is shorter than
# CLASS_STATE_BUILD_TIMEOUT_SECONDS (600s), so a tick that triggers a cargo
# rebuild is still running when the next tick starts -- two producers were
# observed writing this path concurrently on 2026-08-01, and the canonical
# file was caught sitting at 0 bytes with a five-minute-old mtime. Worse, a
# truncated file makes _load_existing_owner_state() fail its json.load() and
# silently return empty owner state, so the *next* run erases the lead's
# manifest items and the orchestrator's channel data.
#
# Fix: serialize, validate, write a temp file in the same directory, fsync,
# then os.replace(). A same-filesystem rename is atomic, so a concurrent
# reader sees either the whole previous payload or the whole new one.

# Keys the viewer dereferences in main(); publishing without them renders a
# broken page just as surely as publishing nothing.
REQUIRED_TOP_LEVEL_KEYS = ("usage", "workchannels", "matrix", "books")
MIN_PLAUSIBLE_PAYLOAD_BYTES = 1024
# nginx (www-data) must be able to read the published payload.
PUBLISH_MODE = int(os.environ.get("PF1E_PUBLISH_MODE", "0644"), 8)


def _validate_payload(text: str) -> str | None:
    """Return an error string if `text` is not safe to publish, else None."""
    size = len(text.encode("utf-8"))
    if size < MIN_PLAUSIBLE_PAYLOAD_BYTES:
        return f"payload is only {size} bytes; refusing to publish a stub"
    try:
        parsed = json.loads(text)
    except json.JSONDecodeError as exc:
        return f"payload does not round-trip as JSON: {exc}"
    missing = [k for k in REQUIRED_TOP_LEVEL_KEYS if k not in parsed]
    if missing:
        return f"payload is missing required keys: {', '.join(missing)}"
    matrix = parsed.get("matrix")
    if not isinstance(matrix, dict) or "classes" not in matrix:
        return "payload has no matrix.classes; the viewer would fail to render"
    return None


def _validate_shard(text: str) -> str | None:
    """Shards carry unit rows, not the dashboard payload, so they get their own
    (much looser) contract: parseable, and shaped like {fields, rows}."""
    try:
        parsed = json.loads(text)
    except json.JSONDecodeError as exc:
        return f"shard does not round-trip as JSON: {exc}"
    if not isinstance(parsed.get("fields"), list) or not isinstance(parsed.get("rows"), list):
        return "shard is missing fields/rows"
    return None


def _atomic_write_json(out_path: str, data: dict, validate=_validate_payload,
                       indent: int | None = 2, keep_last_good: bool = True) -> str | None:
    """Publish `data` to `out_path` atomically. Returns an error string on failure."""
    text = json.dumps(data, indent=indent)
    err = validate(text)
    if err:
        return err

    target = pathlib.Path(out_path)
    tmp_path = None
    try:
        target.parent.mkdir(parents=True, exist_ok=True)
        # Temp file must live in the SAME directory: os.replace() is only
        # atomic within a filesystem.
        with tempfile.NamedTemporaryFile(
            mode="w",
            encoding="utf-8",
            dir=str(target.parent),
            prefix=target.name + ".",
            suffix=".tmp",
            delete=False,
        ) as tmp:
            tmp_path = tmp.name
            tmp.write(text)
            tmp.flush()
            os.fsync(tmp.fileno())
        # NamedTemporaryFile creates 0600 and os.replace() keeps the temp
        # file's mode, so publishing without this silently strips world-read
        # from the canonical path and nginx starts 403ing the payload.
        #
        # Assert the mode rather than inheriting the current file's: this file
        # exists to be served, and inheriting turns a single bad publish into
        # a permanent outage (observed 2026-08-01 -- one tick landed 0600 and
        # every subsequent tick faithfully preserved it).
        os.chmod(tmp_path, PUBLISH_MODE)
        os.replace(tmp_path, str(target))
    except OSError as exc:
        if tmp_path:
            try:
                os.unlink(tmp_path)
            except OSError:
                pass
        return f"write failed: {exc}"

    # Snapshot a known-good copy so a future unreadable canonical file cannot
    # silently wipe owner-managed manifest/channel state. Shards carry no
    # owner state and are regenerable from the source document, so they skip
    # this rather than doubling several MB of disk every tick.
    if keep_last_good:
        try:
            shutil.copyfile(str(target), str(target) + ".last-good")
            os.chmod(str(target) + ".last-good", PUBLISH_MODE)
        except OSError:
            pass
    return None


# ---------------------------------------------------------------------------
# Searchable unit shards (2026-08-01)
# ---------------------------------------------------------------------------
#
# work_inventory_panel() publishes aggregates only: 44,191 units collapsed to
# per-kind and per-book counts. That is the right shape for a summary, but it
# is exactly why the dashboard could report "equipment: 6227" and leave the
# operator no way to ask *which* equipment, or what is blocking it. The
# operator's complaint -- "either so stuffed it doesn't show everything, or it
# shows everything and you can't find anything" -- is a direct consequence:
# there is no layer between a single number and the whole corpus.
#
# The full document (docs/work-inventory.json) is ~22MB, far too large to
# inline into a payload the page fetches on load. So write one shard per kind
# and let the viewer fetch a shard only when the operator opens that kind.
#
# Rows are positional arrays rather than objects because at this row count the
# repeated JSON keys cost more than the values they label.

UNIT_SHARD_DIR = os.environ.get(
    "PF1E_UNIT_SHARD_DIR", os.path.expanduser("~/swarm-observer/units")
)
# `wiring_class` added round 4 (SD-29 QA findings F25/F28, 2026-08-12): the
# per-record table used to show only raw `status`, which is orthogonal to
# doneness and cannot answer "is this record usable" on its own -- the same
# status word means something different under a different wiring_class (see
# doneness_verdict() below). Shipping wiring_class alongside status lets the
# viewer compute the real per-record doneness verdict (via its `donenessOf()`
# mirror of doneness_verdict()) instead of presenting a raw status word under
# a doneness-shaped label, which is the class of bug this rebuild keeps
# reintroducing one surface at a time.
#
# `type_facet` added round 20 (three-level drilldown, 2026-08-13): it already
# exists on every unit in the source document (a dotted engineering string,
# e.g. `SpecialQuality.Extraordinary.Bloodrage.Rage`) but was never projected
# into the shards. Only the FIRST segment is the usable sub-category -- the
# rest is implementation detail specific to how the determinator classified
# the unit, same "for engineers" tier as `source_file`. See
# `category_of_type_facet()` / `CATEGORY_LABELS` below for the raw-string ->
# plain-English translation every viewer surface must go through before
# displaying a category name.
UNIT_SHARD_FIELDS = ("name", "book", "status", "wiring_class", "source_file", "type_facet")
# `spell` carries one additional field: `school` (Abjuration/Conjuration/...),
# extracted this round from the `SCHOOL:` token on the spell's own source
# `.lst` line -- NOT the same axis as `type_facet` (which gives casting
# TRADITION -- Arcane/Divine/Psychic -- for spells, not school). See
# `_spell_schools()` below for the join and its honesty/coverage notes.
SPELL_SHARD_FIELDS = UNIT_SHARD_FIELDS + ("school",)
# Bump whenever UNIT_SHARD_FIELDS/SPELL_SHARD_FIELDS or the per-kind
# `categories` rollup shape changes. The shard index (`units/index.json`) is
# cached purely on the source document's mtime (see build_unit_shards()
# below) -- unlike WIRING_SUMMARY_SCHEMA's cache, it had no schema guard at
# all before this round, so adding `type_facet` here would otherwise keep
# serving pre-round-20 shards forever on any host where the 22MB source
# document's mtime had not moved since the last publish.
#
# Round 21 QA bump (2 -> 3): `category_of()` now strips a leading `WORD:`
# field-token prefix before it's used as a grouping key (finding 1, merges
# e.g. `TYPE:Goods` into `Goods`), and `school_categories` now credits a
# multi-school spell toward every school it names instead of publishing the
# raw pipe-delimited token as its own group (finding 5) -- both change which
# category keys exist and what the school-bucket totals sum to, so a cached
# pre-round-21 shard must not keep being served.
#
# 3 -> 4: `school_join` grew a second finding-5 field,
# `multi_school_extra_credits` (the exact number to add to the spell count
# to reconcile `school_categories`' bucket-total sum -- `multi_school_
# spell_count` alone undercounts a 3-school spell's contribution). Bumped
# again rather than relying on the source document's mtime to have moved,
# same reasoning as every other bump on this constant.
#
# 4 -> 5 (round 22 QA finding 2): `categories`' dict keys changed shape --
# no longer the raw `category_of()` segment, now `category_group_key()`'s
# normalized form (so a CamelCase and a space-separated spelling of the
# same category merge into one bucket). Any client-side code or cached
# shard/index that still assumes a `categories` key is a raw type_facet
# segment (e.g. matches it directly against `categoryOfRaw()`'s per-row
# output) must be updated in lockstep -- see PF1e-dashboard.html's
# `categoryGroupKey()`, added the same round.
#
# 5 -> 6 (round 22, same finding, same session): `category_group_key()`
# grew `_CATEGORY_GROUP_KEY_ALIASES` (the SpecialQuality/SpecialQuaility
# typo-pair merge) shortly after the 4->5 bump above landed. Bumped again
# on principle rather than trusting that no schema-5-stamped shard/index
# was ever cached from the brief window between those two edits (the
# schema-gated cache in build_unit_shards() below returns bit-for-bit
# what an earlier run with the same schema wrote, with no re-check of the
# CODE that produced it).
#
# 6 -> 7 (round 23 QA finding 4): a merged category group's representative
# raw spelling (the one `category_label()` translates into the shipped
# `categories[gkey].label`) is now chosen deterministically by
# `pick_category_representative()` (override-presence, then record count,
# then alphabetical) instead of "whichever spelling `build_unit_shards()`
# iterated over first". Same category, same total counts, but the LABEL TEXT
# for an affected group (e.g. `SpellLike`/`Spelllike` -> now always
# "Spell-Like Ability", never the malformed "Spelllike") could change between
# runs of the OLD code depending on row order, so a shard cached under the
# old logic must not keep being served as if nothing changed.
#
# 7 -> 8 (round 23 QA findings 3 and 5, same publish): `CATEGORY_LABEL_
# OVERRIDES` grew entries for ACF/OOTP/ABP-prefixed/FCB_HA-suffixed
# categories and one explicit "Special Attack" identity entry, and
# `_CATEGORY_GROUP_KEY_ALIASES` grew four more verbatim-typo merges
# (General/Genaral, Metapsionic/Metasionic, Special Attack/Attck/Atack).
# Both change which raw spelling is picked as representative and/or what
# label it translates to for the affected groups -- a cached pre-round-23
# shard must not keep shipping the old untranslated/typo'd labels.
#
# 8 -> 9 (round 23, same session, QA finding 4 verification): the schema-8
# regeneration above surfaced that `SpellLike`'s deterministic-pick fix only
# fires when a kind actually HAS a `SpellLike` sibling to prefer over
# `Spelllike` -- `class_feature`'s 2 `Spelllike` records have no such
# sibling in that kind, so they were still the sole (untranslated) variant,
# and `race_trait`'s `ForlarrenSpelllike` is a different group entirely with
# the same embedded typo and no case boundary to humanize. Two more
# `CATEGORY_LABEL_OVERRIDES` entries added for these; bumped again because
# the schema-8 regen run above is proof this really was served stale
# (identical schema number, edited overrides, old labels still shipped)
# rather than a hypothetical risk.
#
# 9 -> 10 (round 23, same session, harness re-run): the four `ABP <X>`
# override labels above originally used a colon separator ("Automatic Bonus
# Progression: Legendary Gifts"), which reads fine to a human but trips the
# harness's own leaked-field-token check (BARE_COLON_RE in drilldown.js
# check 6) -- a fair catch, that check exists specifically to flag a bare
# colon as looking like an untranslated "TYPE:Goods"-shaped raw token.
# Switched to " -- " instead; bumped again so the harness's own verification
# run isn't re-checking a cached pre-fix shard.
#
# 10 -> 11 (round 25, QA finding 2): round 24 shipped a real label-content
# change ("Special Attack" -> "Special Attacks", plural) without bumping this
# schema, which QA correctly flagged as contradicting the doctrine documented
# above -- the schema-gated cache in build_unit_shards() serves cached content
# verbatim whenever schema == SHARD_SCHEMA, with no re-check of the code that
# produced it, so leaving this unbumped risks a future rollback/restore
# silently serving the stale pre-round-24 label. No active stale artifact was
# found on disk, but the guard against a *future* one needs to be armed.
#
# 11 -> 12 (round 26): the 37 singular/plural category merges (see the
# `_CATEGORY_GROUP_KEY_ALIASES` entries and the RESOLVED comment above
# `CATEGORY_LABEL_OVERRIDES`) change the category grouping SHAPE for
# `class_feature`, `race_trait`, and `companion` -- fewer, larger category
# rows, 1,156 records now counted under a different group's label than
# before. Same doctrine as every prior bump on this constant: a
# schema-gated cache keyed only on this number would otherwise keep
# serving the pre-merge (38-row) shape forever if it happened to still
# match.
#
# 12 -> 13 (P0.2 hardening, 2026-08-14): each `kinds[*]` entry now carries a
# `doneness_unmapped` dict and the top-level index carries
# `doneness_unmapped_seen` (the degrade-path flag for the previously-raising
# doneness_verdict() call in this function's per-unit loop). Same doctrine as
# every prior bump: a schema-gated cache written before this field existed
# would otherwise keep serving shards with no unmapped-status visibility
# forever, silently hiding the exact hazard this round fixes.
#
# 13 -> 14 (Decision 12, 2026-08-17): `name` is now screened against the
# pinned oracle's own declared-PI state (`pi_redacted_names`,
# `pi_oracle_available`, both new on the top-level index). Same doctrine
# again, and load-bearing this time: a cache written before this fix would
# keep serving 261 unredacted PI names indefinitely -- exactly the
# regression Decision 12 requirement #3's gate exists to catch, so this
# bump is what makes the FIRST post-fix run actually recompute instead of
# serving the pre-fix shard back unchanged.
#
# 14 -> 15 (FIX-DASHBOARD-PI, 2026-08-17): `name` (row) and `categories[*]`/
# `school_categories[*]`'s own `label` are now ALSO screened with a
# WORD-BOUNDARY embed check (`_PiScreen.screen`, book-scoped union global,
# gated by the shared `pi_substring_allowlist`) on top of the round-14
# exact-match check -- `category_labels_redacted` is new on the top-level
# index. Same doctrine again: a cache written before this fix would keep
# serving `"Helm of the Serpent King"`, `"Varisian Pilgrim Domain"` and
# every other word-boundary leak this fix closes indefinitely.
SHARD_SCHEMA = 15
# Wave-21 fix (OPEN-ISSUES.md row 336): this default used to hardcode the
# shared checkout and never consulted CODEX_REPO_ROOT (read above, at line
# 110, for an unrelated purpose) -- so a lane running in its own isolated
# worktree with CODEX_REPO_ROOT set would still publish a DIFFERENT,
# concurrently-running lane's board under its own commit. PF1E_WORK_INVENTORY_
# DOC remains the explicit, highest-priority override for any caller that
# still wants to point at a specific file regardless of worktree.
WORK_INVENTORY_FULL_DOC = os.environ.get(
    "PF1E_WORK_INVENTORY_DOC",
    os.path.join(DEFAULT_REPO_ROOT, "docs", "work-inventory.json"),
)


def publishable_document_path(doc_path: str) -> str:
    """The value a PUBLISHED feed may record for a source document.

    An absolute filesystem path must never reach `site/` (SD31-W15-INTEGRATE-001,
    two independent adversarial reviewers, wave 15). It had two live effects:

      * `verify.sh`'s `site-dashboard-check` regenerates the feed and compares it
        to the committed one after a scrub that strips only timestamps. The
        absolute path was then the ONLY differing leaf in a 1.3 MB payload, so
        the stage reported STALE from every checkout except the one that
        published -- including every linked worktree, CI, and the shared tree
        after a worktree was cleaned up. A gate that fails for a reason
        unrelated to what it guards is a gate on its way to being baselined
        away, which is the mirror image of the Decision 1(a) hazard.
      * The path (a home directory plus an ephemeral worktree id) was committed
        into `site/`, which `deploy-site.yml` publishes to Cloudflare Pages.

    So: record the path RELATIVE TO THE ENCLOSING GIT CHECKOUT. The checkout is
    found by walking up for a `.git` entry -- a directory in a normal clone and
    a FILE in a linked worktree, and the worktree case is precisely the one that
    broke, so both are handled. `DEFAULT_REPO_ROOT` is deliberately NOT used as
    the base: it is an env default pointing at the shared checkout, so it would
    still produce a checkout-specific answer from a worktree.

    A document outside any checkout keeps its resolved absolute path. That
    degrades VISIBLY rather than silently: collapsing it to a bare basename
    would make two genuinely different documents compare equal, which is the
    cache-identity confusion `compute_wiring_class_summary()` records above.
    """
    resolved = os.path.realpath(doc_path)
    parent = os.path.dirname(resolved)
    while True:
        if os.path.exists(os.path.join(parent, ".git")):
            return os.path.relpath(resolved, parent).replace(os.sep, "/")
        nxt = os.path.dirname(parent)
        if nxt == parent:
            return resolved
        parent = nxt
PROVEN_STATUSES = ("grounded", "text-complete")

# ---------------------------------------------------------------------------
# Sub-category translation (round 20, three-level drilldown)
# ---------------------------------------------------------------------------
#
# `type_facet`'s first segment is raw engineering vocabulary straight off the
# determinator (GE-01) -- exactly the class of string the "ZERO engineering
# vocabulary in user-facing labels" rule (settled rounds 1-19) forbids on any
# surface a reader reaches by normal navigation. Every category name shown
# outside a "for engineers" disclosure must go through `category_label()`.
#
# Hand-curated entries below cover the common/ambiguous cases (acronym-style
# or non-obvious splits: "PC" is not "Pc", "Goods" reads better as
# "Adventuring Gear" than as itself). Everything else falls through to
# `_humanize()`, a generic CamelCase/PascalCase/underscore splitter -- most of
# the long tail (`MonkBonusFeat`, `CompanionAdvancement`,
# `TempEvolutionChoice`, ...) is already close to readable and only needs
# word-boundary spacing, not a hand-written translation. Strings that already
# contain spaces (`"Warpriest Bonus Feat"`, `"Class Feature"`) pass through
# unchanged (title-cased only).
#
# RESOLVED (round 26; deferred round 23, recorded round 24 QA finding 2,
# scope-corrected round 24, merged round 26). Singular/plural category
# duplication -- "Class Feature" vs "Class Features" (or "Racial Trait" /
# "Racial Traits") differ by a trailing "s", not by case or spacing, so
# `category_group_key()`'s character normalization alone never merged them
# -- is now merged for all 37 documented pairs, via explicit entries in
# `_CATEGORY_GROUP_KEY_ALIASES` below, same mechanism as the verbatim-typo
# merges in that table (`SpecialQuality`/`SpecialQuaility` etc.), just at
# larger scale.
#
# Why this was safe to do now, having been refused as a blanket
# string-similarity merge in rounds 23-25: each of the 37 pairs was
# individually checked against the live `docs/work-inventory.json` records
# (source_file provenance + actual unit names on both sides) before being
# added to the alias table -- see the comment block directly above the 37
# entries in `_CATEGORY_GROUP_KEY_ALIASES` for the full per-pair evidence.
# Every pair confirmed as the SAME real category (a class's set of named
# class features, a race's set of named racial traits, or a companion's set
# of granted abilities) authored under two spellings -- never a broad
# label on one side colliding with a narrower named subtype on the other.
# Zero of the 37 were left unmerged; none showed evidence of denoting a
# genuinely different real thing. This is the exact investigation this
# comment used to say a future round would need to do before merging (see
# the git history of this comment for the original "A future round that
# decides to merge these needs..." framing) -- it has now been done.
#
# Corrected scope (round 24 QA recount; round 23's own report understated
# this as "~36 pairs", `<Class> Class Feature(s)` only): 37 pairs, 6,567
# records involved (see the round-27 QA note above `_CATEGORY_GROUP_KEY_
# ALIASES`'s "racialtrait" entry for the +1 correction: a 38th, no-op
# `race_trait` rename record the 6,566 figure originally missed). `class_feature` ("Class Features" (907) vs "Class
# Feature" (415), plus one pair per class, e.g. "Ranger Class Features"
# (265) vs "Ranger Class Feature" (7)), `race_trait` ("Racial Traits"
# (1,260) vs "Racial Trait" (29)), and `companion` ("Class Features" (1) vs
# "Class Feature" (12)).
#
# Actual verified moved-record count (computed from the real merge, not
# the round-24 QA pre-merge estimate of "~1,145"): **1,156 records** change
# display bucket -- see the per-pair breakdown comment in
# `_CATEGORY_GROUP_KEY_ALIASES` for why this isn't simply "always the
# smaller/singular side" (ten of the 37 groups resolve to their singular
# spelling winning, by `pick_category_representative()`'s own count-based
# tiebreak, not a hardcoded "plural always wins" rule).
CATEGORY_LABEL_OVERRIDES = {
    "none": "Uncategorised",
    "SpecialQuality": "Special Qualities",
    "SpecialQuaility": "Special Qualities",  # verbatim typo present in source data
    "ClassFeatures": "Class Features",
    "RacialTraits": "Racial Traits",
    "RacialTrait": "Racial Trait",
    "RaceAbility": "Race Ability",
    "Goods": "Adventuring Gear",
    "PC": "Player-Character Classes",
    "NPC": "NPC Classes",
    "SpecialAttack": "Special Attacks",
    "ArchetypeAbility": "Archetype Ability",
    "BaseMaterial": "Base Material",
    "PFSNotLegal": "Not PFS Legal",
    "ImpCompTrick": "Improved Companion Trick",
    "AnimalCompanionFeat": "Animal Companion Feat",
    "CompanionAdvancement": "Companion Advancement",
    "TempEvolutionChoice": "Temporary Evolution Choice",
    "SpellLike": "Spell-Like Ability",
    # Round 23 QA finding 4 verification: `Spelllike` (one L missing, a
    # verbatim source typo of `SpellLike` -- same convention as `Wonderous`/
    # `SpecialQuaility` above) only has a `SpellLike` sibling to lose the
    # deterministic pick to in SOME kinds (companion: 88 SpellLike vs 0
    # Spelllike; race_trait: 2 SpellLike vs 1 Spelllike -- `SpellLike` wins
    # both, via `pick_category_representative()`'s override-first rule). In
    # `class_feature` specifically there is NO `SpellLike` sibling at all --
    # only 2 records, both spelled `Spelllike` -- so it was the sole variant
    # and had nothing to lose to, still shipping the malformed label until
    # this entry existed for the misspelling itself.
    "Spelllike": "Spell-Like Ability",
    # `ForlarrenSpelllike` (race_trait, 7 records) is its OWN category -- the
    # Forlarren race's spell-like abilities specifically, a real distinct
    # group from the generic `SpellLike`/`Spelllike` bucket above, not a
    # duplicate of it -- but it carries the same `Spelllike` (missing-L)
    # typo embedded with no case boundary for `_humanize()` to split on, so
    # it needs its own explicit translation too.
    "ForlarrenSpelllike": "Forlarren Spell-Like Ability",
    "Wonderous": "Wondrous",  # verbatim typo present in source data
    # Round 21 QA finding 1: internal-vocabulary/mangled-compound one-offs
    # that don't fall out cleanly from the generic CamelCase splitter below,
    # hand-curated same as every other entry in this table.
    "SLOT_Ring": "Ring Slot",  # raw internal magic-item-slot vocabulary
    "ForbiddenRiteSDomain": "Forbidden Rite's Domain",  # possessive lost to CamelCase
    # Round 23 QA finding 5: raw acronyms that are NOT reader-understandable
    # even correctly capitalized (unlike "PC"/"NPC"/"AC"/"CMD"/"HP"/"PFS",
    # which are standard, widely-known Pathfinder/D&D-family game vocabulary
    # left alone on purpose -- this table's job is stripping ENGINEERING
    # vocabulary, not game vocabulary). ABP = Automatic Bonus Progression,
    # ACF = Alternate Class Feature, FCB = Favored Class Bonus -- all real
    # Pathfinder rules terms, but not ones a first-time reader can decode
    # from the bare acronym the way they can "AC" or "HP".
    "ACF": "Alternate Class Feature",
    # `OOTPBonusFeat` (Advanced Race Guide, arg_abilities_class.lst) -- OOTP
    # is the "Order of the Paw" ranger/druid order this bonus-feat list
    # belongs to (confirmed against the source record's own id/name,
    # `order_of_the_paw_bonus_feat_*`); not a generic engineering acronym at
    # all, just an unexpanded in-source abbreviation.
    "OOTPBonusFeat": "Order of the Paw Bonus Feat",
    # `WILDSTALKERWildTalent` -- the class name "Wildstalker" was written in
    # ALL CAPS in the source `.lst` line, which trips `_humanize()`'s
    # acronym-preserving branch (2+ consecutive uppercase letters, correctly
    # meant for real acronyms like "ABP"/"FCB") into keeping the whole word
    # shouted instead of treating it as a normal capitalized name.
    "WILDSTALKERWildTalent": "Wildstalker Wild Talent",
    # `ABP <X>` segments already contain spaces in the raw source data, so
    # `_humanize()`'s "already spaced, pass through" rule leaves "ABP"
    # untranslated -- explicit per-variant overrides rather than a
    # substring-replace so the corpus's OWN plain-language framing of each
    # bonus (Legendary Gifts/Legendary Ability/Mental Prowess/Physical
    # Prowess) is preserved unchanged, with only the "ABP" acronym itself
    # expanded.
    # Round 23 QA harness re-run: a colon separator here ("Automatic Bonus
    # Progression: Legendary Gifts") reads fine to a human but trips the
    # harness's own BARE_COLON_RE leaked-field-token check (drilldown.js
    # check 6), which exists specifically to catch a raw, untranslated
    # "TYPE:Goods"-shaped string -- a legitimate label should not need a
    # bare colon either, so it's a fair catch, not a harness false positive
    # to special-case around. Switched to " -- " to read the same without
    # tripping it.
    "ABP Legendary Gifts": "Automatic Bonus Progression -- Legendary Gifts",
    "ABP Legendary Ability": "Automatic Bonus Progression -- Legendary Ability",
    "ABP Mental Prowess": "Automatic Bonus Progression -- Mental Prowess",
    "ABP Physical Prowess": "Automatic Bonus Progression -- Physical Prowess",
    # `DwarfPaladinFCB_HA` / `ElfSorcererFCB_HA` (Horror Adventures
    # race_trait rows) -- the trailing `_HA` is the determinator's own
    # source-book tag (Horror Adventures), not a game term at all; the
    # book is already shown elsewhere on every surface that displays this
    # category, so it's dropped rather than guessed-and-mistranslated.
    "DwarfPaladinFCB_HA": "Dwarf Paladin Favored Class Bonus",
    "ElfSorcererFCB_HA": "Elf Sorcerer Favored Class Bonus",
    # Round 23 QA finding 3: an explicit override entry is required here,
    # not just the `_CATEGORY_GROUP_KEY_ALIASES` merge below -- the
    # correctly-spelled "Special Attack" and its two typo'd variants
    # ("Special Attck", "Special Atack") each have exactly 1 record, a
    # 3-way count TIE with none previously overridden, so
    # `pick_category_representative()`'s count-then-alphabetical tiebreak
    # would otherwise pick "Special Atack" (alphabetically first) as the
    # shipped label -- a typo, not the correct spelling. This override
    # guarantees the correct spelling always outranks its misspelled
    # siblings regardless of which happens to have the highest count.
    #
    # Round 24 QA finding 4: the original entry here was the IDENTITY
    # mapping "Special Attack" -> "Special Attack", which won the tiebreak
    # correctly but shipped race_trait's copy of this category as singular
    # "Special Attack" while the SAME normalized category (`specialattack`)
    # ships PLURAL "Special Attacks" everywhere else it appears
    # (class_feature, companion, feat, monster_ability -- see the
    # "SpecialAttack": "Special Attacks" override above). Mapped to the
    # plural instead so the tie still resolves deterministically but no
    # longer disagrees with itself across lanes.
    "Special Attack": "Special Attacks",
}

# A leading `WORD:` field-token prefix (e.g. `TYPE:Goods`, `TYPE:SpecialQuality`)
# is raw determinator/source-file vocabulary, not part of the category itself
# -- stripped in `category_of()` (before the value is ever used as a grouping
# key) so `TYPE:Goods` and `Goods` land in the SAME bucket instead of two.
_FIELD_TOKEN_PREFIX_RE = re.compile(r"^[A-Za-z][A-Za-z0-9]*:")

# Generic CamelCase/PascalCase/underscore tokenizer for `_humanize()`.
# Order matters: an uppercase run not immediately followed by a lowercase
# letter (`[A-Z]+(?![a-z])`) is tried first so acronym/roman-numeral runs of
# 2+ letters ("ABP", "FCB", "CMD", "OOTP", "III") are captured whole before
# the single-capital-letter alternative would otherwise claim just their
# first character.
_WORD_TOKEN_RE = re.compile(r"[A-Z]+(?![a-z])|[A-Z][a-z]*|[a-z]+|\d+|['\-]|,|\s+")
_WORD_KINDS = {"acronym", "capword", "lower", "digit"}


def _humanize(raw: str) -> str:
    """Generic engineering-string -> plain-English fallback.

    Splits CamelCase/PascalCase word boundaries and underscores into words,
    then capitalizes -- but NOT via `str.title()`, which mangles both
    existing acronym/roman-numeral runs ("ABP" -> "Abp", "III" -> "Iii") and
    apostrophes ("assassin's" -> "Assassin'S"). Not a hand-curated
    translation -- a mechanical one, used only when
    `CATEGORY_LABEL_OVERRIDES` has no entry, so a corpus-specific string
    this round's author never saw still renders as words instead of as raw
    source vocabulary.

    Rules, in order:
      - underscores become spaces (hyphens are left alone -- "Half-Elf" is a
        real compound word, not a word-boundary marker);
      - any existing run of 2+ consecutive uppercase letters (acronym or
        roman numeral, standalone or embedded in a CamelCase run, e.g. the
        "III" in "...TrainingIIISelection") is preserved as-is;
      - a letter->digit boundary gets a space ("Tattoo10" -> "Tattoo 10");
      - the character immediately after an apostrophe is never capitalized.
    """
    s = raw.replace("_", " ")
    tokens = _WORD_TOKEN_RE.findall(s)
    out: list[str] = []
    prev_kind = None
    for tok in tokens:
        if tok.isspace():
            if out and out[-1] != " ":
                out.append(" ")
            prev_kind = "space"
            continue
        if tok == ",":
            out.append(", ")
            prev_kind = "space"
            continue
        if tok == "'":
            out.append(tok)
            prev_kind = "apos"
            continue
        if tok == "-":
            out.append(tok)
            prev_kind = "hyphen"
            continue
        if tok.isdigit():
            kind = "digit"
        elif tok.isupper() and len(tok) >= 2:
            kind = "acronym"
        elif tok[0].isupper():
            kind = "capword"
        else:
            kind = "lower"
        if kind in _WORD_KINDS and prev_kind in _WORD_KINDS:
            out.append(" ")
        if kind == "lower" and prev_kind in (None, "space"):
            tok = tok[0].upper() + tok[1:]
        out.append(tok)
        prev_kind = kind
    result = "".join(out)
    result = re.sub(r"\s+", " ", result).strip()
    return result if result else raw


def category_label(raw_first_segment: str) -> str:
    """Plain-English label for a `type_facet` first segment (or `"none"`)."""
    if raw_first_segment in CATEGORY_LABEL_OVERRIDES:
        return CATEGORY_LABEL_OVERRIDES[raw_first_segment]
    return _humanize(raw_first_segment)


def category_of(type_facet) -> str:
    """First dotted segment of a `type_facet`, or the literal `"none"`.

    `"none"` (not `None`/blank) so it can be used directly as a dict key and
    round-tripped through JSON without a null-handling special case on the
    viewer side -- same convention `wiring_class` already uses for its own
    missing-value default (`"ambiguous"`), just a different literal because
    `"none"` is not a real `wiring_class` value and cannot collide.
    """
    if not type_facet:
        return "none"
    seg = str(type_facet).split(".", 1)[0]
    # Round 21 QA finding 1: a leading `WORD:` field-token prefix (e.g.
    # `TYPE:Goods`) is raw source-file/determinator vocabulary, not part of
    # the category -- stripped here, before the value is ever used as a
    # dict/grouping key, so `TYPE:Goods` and `Goods` merge into one bucket
    # instead of the prefixed form silently creating a duplicate.
    seg = _FIELD_TOKEN_PREFIX_RE.sub("", seg, count=1)
    return seg or "none"


# Round 22 QA finding 2: the raw `type_facet` data itself contains BOTH a
# CamelCase spelling and an already-space-separated spelling of the same
# category for many classes (e.g. `RangerClassFeatures` and
# `Ranger Class Features` both occur as literal type_facet first segments).
# `_humanize()` correctly turns the CamelCase form into "Ranger Class
# Features", which then COLLIDES with the already-spaced raw form on display
# -- but `category_of()` above still returns them as two different strings,
# so `categories.setdefault(cat, ...)` (below, in build_unit_shards()) was
# creating two separate buckets with an IDENTICAL rendered label and two
# different counts, neither of which was the true total. Confirmed live:
# 22 duplicate-label pairs, ~6,450 affected units, concentrated in
# `class_feature` (one CamelCase/spaced pair per class) plus a few
# `Archetype` pairs.
#
# Fix: normalize the GROUPING KEY itself (not just the display label) so a
# CamelCase spelling and its space-separated equivalent collapse into ONE
# bucket before counting -- strip everything but letters/digits and
# lowercase, so `RangerClassFeatures` and `Ranger Class Features` both
# normalize to `rangerclassfeatures`. The bucket's displayed label comes from
# `category_label()` on ONE representative raw spelling out of the group,
# chosen by `pick_category_representative()` below -- tracked per-group in
# `category_raw_variants` in build_unit_shards().
#
# Round 23 QA finding 4: the "either works -- by construction the two
# spellings already render identically" claim that used to sit here was
# FALSE. `category_group_key()` merges on a NORMALIZED KEY only
# (case/punctuation-insensitive); it says nothing about the DISPLAY LABEL,
# which still goes through `category_label()` on whichever single raw
# spelling gets picked as representative. `SpellLike` and `Spelllike` (a
# genuine one-letter source typo, not just a case/spacing variant) both
# normalize to the same key `spelllike`, but `category_label("SpellLike")`
# hits the `CATEGORY_LABEL_OVERRIDES` entry ("Spell-Like Ability") while
# `category_label("Spelllike")` has no override and falls through to
# `_humanize()`, which -- having no word-boundary to split on -- returns the
# malformed "Spelllike" verbatim. Whichever spelling used to win was simply
# whichever `build_unit_shards()` iterated over FIRST (`category_raw_repr`
# was a `setdefault`, first-seen-wins), so the shipped label depended on
# corpus row order and could silently flip on a future re-run with reordered
# input. See `pick_category_representative()` below for the deterministic
# replacement.
def category_group_key(raw_seg: str) -> str:
    """Case/whitespace/punctuation-insensitive grouping key for a
    `category_of()` result, so a CamelCase and a space-separated spelling of
    the same category merge into one bucket instead of two identically-
    labelled ones."""
    key = re.sub(r"[^a-z0-9]", "", str(raw_seg).lower()) or "none"
    return _CATEGORY_GROUP_KEY_ALIASES.get(key, key)


# Round 23 QA finding 4: picks the raw spelling used to derive a merged
# category group's display label -- deterministic and independent of corpus
# row order (unlike the `setdefault`-first-seen approach it replaces).
# `variant_counts` maps every raw spelling seen for one `category_group_key()`
# bucket to how many records used it (e.g. {"SpellLike": 40, "Spelllike": 3}).
#
# Priority, in order:
#   1. A spelling with an explicit `CATEGORY_LABEL_OVERRIDES` entry always
#      wins over one without -- a hand-curated translation is never worse
#      than falling through to the generic `_humanize()` splitter, regardless
#      of which spelling happens to be more common in the source data.
#   2. Among spellings tied on (1), the one with the higher record count --
#      "most representative of the group" is a reasonable tiebreak and, for
#      the common CamelCase-vs-already-spaced case (neither has an override),
#      this is the only signal available.
#   3. Alphabetical order, purely to guarantee full determinism if (1) and
#      (2) both tie (e.g. two spellings with identical counts and neither
#      overridden) -- never reached by row order, so a re-run with the same
#      input always picks the same representative.
def pick_category_representative(variant_counts: dict) -> str:
    def sort_key(item):
        raw, count = item
        has_override = raw in CATEGORY_LABEL_OVERRIDES
        return (0 if has_override else 1, -count, raw)
    return sorted(variant_counts.items(), key=sort_key)[0][0]


# Character-normalization alone (above) does not catch every duplicate-label
# source -- a full post-fix sweep of the live payload (round 22 QA finding 2
# verification) found ONE more: `SpecialQuality` and `SpecialQuaility` (the
# latter a verbatim typo already hand-documented in
# CATEGORY_LABEL_OVERRIDES above, both resolving to the SAME label "Special
# Qualities") differ by an actual LETTER, not just case/spacing, so they
# normalize to two different group keys ("specialquality" vs
# "specialquaility") and stayed split (144 vs 15 records under an identical
# "Special Qualities" label in the `companion` lane -- true total 159).
# Rather than generalize the normalizer into a fuzzy-match/edit-distance
# merge (which risks merging genuinely DIFFERENT categories that happen to
# be spelled similarly), this is a small explicit alias table, same
# hand-curated posture as CATEGORY_LABEL_OVERRIDES itself -- add an entry
# here whenever a future sweep finds another known-typo pair.
_CATEGORY_GROUP_KEY_ALIASES = {
    "specialquaility": "specialquality",
    # Round 23 QA finding 3: a full edit-distance-<=1 sweep of every lane's
    # shipped category labels (not just QA's 4 named pairs) turned up these
    # additional live verbatim-typo pairs. Each entry merges a misspelled
    # normalized key into the correctly-spelled group's key -- deliberately
    # NOT a fuzzy/generalized edit-distance merge (same reasoning as the
    # `specialquaility` entry above: that risks merging genuinely different
    # categories that happen to be spelled similarly), just explicit
    # hand-verified pairs.
    "genaral": "general",  # feat: "Genaral" (1) is a typo of "General" (839)
    "metasionic": "metapsionic",  # feat: "Metasionic" (1) is a typo of "Metapsionic" (34)
    "specialattck": "specialattack",  # race_trait: "Special Attck" (1) is a typo of "Special Attack" (1)
    "specialatack": "specialattack",  # race_trait: "Special Atack" (1) is a typo of "Special Attack" (1)
    # Checked and explicitly NOT merged: race_trait's "Shikigami Racial
    # Ability" and "Shinigami Racial Ability" also collide at edit-distance
    # 1, but "Shikigami" (paper-spirit servants) and "Shinigami" (death
    # gods/reapers) are two genuinely different real-world-mythology-derived
    # race/creature names in the corpus, not a typo of one another -- merging
    # them would silently combine two different races' data. Left as two
    # groups on purpose.

    # Round 26 (the singular/plural merge this table's docstring-length
    # comment above CATEGORY_LABEL_OVERRIDES deferred since round 23):
    # every one of the 37 documented singular/plural pairs was individually
    # investigated against the live `docs/work-inventory.json` records
    # before being added here -- NOT a blanket string-similarity merge. For
    # every pair the check was: (1) does the singular side's `source_file`
    # set plausibly belong to the SAME class/race/companion-kind concept as
    # the plural side (even when the two sides come from disjoint files --
    # e.g. `isc_abilities_class.lst` supplying only the singular spelling
    # for one class and a different book supplying only the plural, which
    # turned out to be the common case, not the exception), and (2) do the
    # actual unit `name`s under each side read as the same kind of thing
    # (individual named class features / racial traits / companion
    # abilities on BOTH sides, never a broad label on one side and a
    # narrower named subtype on the other). Every pair checked out: 34
    # `<Class> Class Feature(s)` pairs (spot-checked via full name listings
    # for antipaladin, bard, brawler, kineticist, occultist, summoner, plus
    # every pair that had file overlap -- alchemist, arcanist, barbarian,
    # cavalier, magus, medium, paladin, ranger, rogue, shaman, shifter,
    # skald, vigilante, witch, wizard -- all consistent, no exceptions
    # found), the generic `class_feature` "Class Feature"/"ClassFeatures"
    # catch-all pair (confirmed same class -- e.g. Bloodrager -- appearing
    # under both spellings in the SAME source file, `acg_abilities_class.
    # lst`), `companion`'s "Class Feature" (12, `ce_abilities_familiar_cr.
    # lst`, familiar-granted abilities) vs "ClassFeatures" (1, `um_
    # abilities_companion.lst`, `ultimate_magic:companion:black_blade_
    # alertness` -- the bladebound magus's Black Blade, an intelligent
    # bonded weapon, NOT an animal companion; confirmed via all 15
    # `black_blade`-prefixed units, headed by `ultimate_magic:companion:
    # black_blade | Black Blade | Monster.Companion`, siblings incl.
    # `black_blade_ego`/`black_blade_telepathy`/`black_blade_life_drinker`)
    # -- both sides are "a companion creature's granted class feature,"
    # same concept applied to a different companion subtype (familiar vs.
    # bonded weapon), not two different concepts; both even independently
    # include an "Alertness" record),
    # and `race_trait`'s "RacialTrait" (29, all from Bestiary 3's `b3_
    # abilities_race.lst`, monster/animal-lord racial traits) vs
    # "RacialTraits" (1,260, every PC race file -- Dwarf/Elf/etc.) -- both
    # sides are "a race's set of named racial traits," just authored with
    # the monster-race book on one spelling convention and the PC-race
    # books on the other. No pair among the 37 showed evidence of denoting
    # a genuinely different real category, so none were left out.
    #
    # Verified actual moved-record count (computed from this exact merge,
    # not the round-24 QA estimate of "~1,145"): 1,156 records change
    # display bucket. 6,567 total records span the 37 merged groups (see
    # the round-27 QA note above `_CATEGORY_GROUP_KEY_ALIASES`'s
    # "racialtrait" entry: this corrects the round-24 QA recount of 6,566
    # by +1, for a 38th, no-op `race_trait` rename record it missed); the
    # two figures (1,156 moved vs 6,567 spanning) differ
    # because "moved" only counts the LOSING spelling's total once
    # `pick_category_representative()`'s override-then-count-then-
    # alphabetical tiebreak picks a winner per group -- e.g. `shaman
    # classfeature` (57 combined singular records across 3 files) actually
    # OUTNUMBERS `shamanclassfeatures` (4 records, one file) and so the
    # singular spelling wins that group's label, moving the 4 plural
    # records rather than the 57 singular ones. Most groups have plural as
    # the larger/winning side (matching the doc comment's original
    # expectation), but ten of the 37 do not: brawler, hunter, investigator
    # (plural wins by count but the label still reads "Investigator Class
    # Features" -- not an exception, just noted since it's a near-tie),
    # kineticist, occultist, shaman, skald (27/27 exact tie, alphabetical
    # tiebreak picks the shorter singular spelling), slayer, spiritualist,
    # swashbuckler, and warpriest all resolve to their SINGULAR spelling
    # winning -- the mechanism decided this per-group as designed, nothing
    # hardcoded here to force a direction.
    "classfeature": "classfeatures",
    "alchemistclassfeature": "alchemistclassfeatures",
    "antipaladinclassfeature": "antipaladinclassfeatures",
    "arcanistclassfeature": "arcanistclassfeatures",
    "barbarianclassfeature": "barbarianclassfeatures",
    "bardclassfeature": "bardclassfeatures",
    "brawlerclassfeature": "brawlerclassfeatures",
    "cavalierclassfeature": "cavalierclassfeatures",
    "clericclassfeature": "clericclassfeatures",
    "druidclassfeature": "druidclassfeatures",
    "fighterclassfeature": "fighterclassfeatures",
    "gunslingerclassfeature": "gunslingerclassfeatures",
    "hunterclassfeature": "hunterclassfeatures",
    "inquisitorclassfeature": "inquisitorclassfeatures",
    "investigatorclassfeature": "investigatorclassfeatures",
    "kineticistclassfeature": "kineticistclassfeatures",
    "magusclassfeature": "magusclassfeatures",
    "mediumclassfeature": "mediumclassfeatures",
    "mesmeristclassfeature": "mesmeristclassfeatures",
    "monkclassfeature": "monkclassfeatures",
    "occultistclassfeature": "occultistclassfeatures",
    "paladinclassfeature": "paladinclassfeatures",
    "rangerclassfeature": "rangerclassfeatures",
    "rogueclassfeature": "rogueclassfeatures",
    "shamanclassfeature": "shamanclassfeatures",
    "shifterclassfeature": "shifterclassfeatures",
    "skaldclassfeature": "skaldclassfeatures",
    "slayerclassfeature": "slayerclassfeatures",
    "spiritualistclassfeature": "spiritualistclassfeatures",
    "summonerclassfeature": "summonerclassfeatures",
    "swashbucklerclassfeature": "swashbucklerclassfeatures",
    "vigilanteclassfeature": "vigilanteclassfeatures",
    "warpriestclassfeature": "warpriestclassfeatures",
    "witchclassfeature": "witchclassfeatures",
    "wizardclassfeature": "wizardclassfeatures",
    # This alias table is applied per-kind (category_group_key() is called
    # inside build_unit_shards()'s per-kind loop, into a `categories` dict
    # that is re-initialized fresh for every kind), so the single
    # "classfeature" -> "classfeatures" entry above is shared safely by
    # BOTH the `class_feature` kind's generic catch-all pair AND the
    # `companion` kind's pair ("Class Feature" (12, familiar-granted
    # abilities) vs "ClassFeatures" (1, the bladebound magus's Black
    # Blade bonded-weapon-granted abilities) -- the plural is the LOSING
    # side by count here, but
    # `pick_category_representative()`'s override-first rule still ships
    # it as "Class Features" regardless, because "ClassFeatures" already
    # has an explicit CATEGORY_LABEL_OVERRIDES entry and "Class Feature"
    # does not) -- no separate entry needed, and adding one would just be
    # a duplicate dict key mapping to the same value.
    #
    # QA note (round 27): this same per-kind sharing also means the
    # "warpriestclassfeature" -> "warpriestclassfeatures" entry above --
    # added for the `class_feature` kind's Warpriest pair -- ALSO fires in
    # the `race_trait` lane, where it matches exactly one record:
    # `advanced_class_guide:race_trait:warpriest_favored_class_blessings`
    # ("Blessings (Favored Class)"). That record has no plural-spelled
    # sibling in `race_trait`, so nothing merges -- the group key is just
    # silently renamed to a plural spelling no record in the group
    # actually uses. Harmless (the shipped label stays "Warpriest Class
    # Feature," the sole representative, count unchanged at 1) but
    # previously undocumented. This makes the true count of (kind, group)
    # pairs touched by the alias table 38, not 37 (35 in `class_feature` +
    # 2 in `race_trait` + 1 in `companion`): 37 are genuine two-sided
    # merges and this one is a no-op rename. It also means the "6,566
    # total records span the 37 merged groups" figure quoted above and in
    # the docstring-length comment above `CATEGORY_LABEL_OVERRIDES` should
    # read 6,567, to include this record.
    # `race_trait` lane.
    "racialtrait": "racialtraits",
}

# CLOSED 2026-08-24 (`decisions.md §27b`, operator ruling 2026-08-23:
# "EVERYTHING" -- no carve-outs survive). This constant used to read
# `{"beginner_box"}` on a 2026-08-02 operator directive ("genuinely
# simplified intro subset ... ruled out of scope"). That directive did not
# survive §27b: the only admissible reasons a unit may sit outside every
# closure figure are a hard impossibility -- the source data does not
# exist, or licensing forbids shipping it -- and neither holds for
# `beginner_box`. Its 19 equipment units are real, declared records sourced
# from the pinned PCGen oracle's own `bbox_equip_magic_items.lst` /
# `bbox_equip_arms_armor.lst` (`data/pathfinder/paizo/roleplaying_game/
# beginner_box/`, verified present at `PCGEN_ORACLE_SHA`
# 7f818006e371188e5717fd18d74d18a420747fc6) -- "genuinely simplified" is a
# cost/awkwardness judgment, exactly what §27b names as inadmissible. They
# were already flowing into `docs/work-inventory.json` as `not-started`
# units (evidence `no_compiled_rule_set_for_book`); the carve-out lived
# ONLY in this dashboard-reporting layer, hiding them from every
# denominator rather than reporting them honestly as not-done. Kept empty
# now rather than deleted so the mechanism survives for a FUTURE
# genuinely-admissible exclusion -- but any future entry here must carry a
# paired, admissible reason in `EXCLUDED_BOOKS_REASONS` below, checked at
# import time, so the next carve-out cannot hide silently in code the way
# this one did (`decisions.md §27b`'s own diagnosis: "it survived every
# prose sweep because it lives in Python rather than in a document").
#
# Historical note (why this constant exists at all): every corpus-wide
# figure this producer emits reads from this one constant -- by_status,
# by_kind, by_wiring_class, by_doneness, by_doneness_kind, cross_tab, and
# the unit-search shard index -- so a book cannot silently stay excluded
# from some rollups and not others the way `cross_tab` and
# `build_unit_shards()` did in round 1 (SD-29 QA findings #7/#8, round 2,
# 2026-08-12: both drifted from this set and their totals stopped matching
# the by-lane figures by exactly beginner_box's count).
EXCLUDED_BOOKS: frozenset[str] = frozenset()

# The only reasons `decisions.md §27b` admits for a book to sit in
# EXCLUDED_BOOKS. Anything else is a cost/awkwardness/novelty judgment and
# must be escalated for an operator ruling instead, per the same decision.
ADMISSIBLE_EXCLUSION_REASONS = frozenset({
    "source_data_absent",
    "licensing_forbids_shipping",
})

# book -> admissible reason. Every key of EXCLUDED_BOOKS must appear here
# with a value drawn from ADMISSIBLE_EXCLUSION_REASONS; the assertion right
# below enforces it at import time so a future carve-out cannot be added to
# EXCLUDED_BOOKS alone without also declaring, in writing, why it qualifies.
EXCLUDED_BOOKS_REASONS: dict[str, str] = {}

assert set(EXCLUDED_BOOKS) <= set(EXCLUDED_BOOKS_REASONS), (
    "EXCLUDED_BOOKS entries missing a declared reason in "
    f"EXCLUDED_BOOKS_REASONS: {sorted(set(EXCLUDED_BOOKS) - set(EXCLUDED_BOOKS_REASONS))}"
)
assert all(reason in ADMISSIBLE_EXCLUSION_REASONS for reason in EXCLUDED_BOOKS_REASONS.values()), (
    "EXCLUDED_BOOKS_REASONS carries a reason outside "
    f"ADMISSIBLE_EXCLUSION_REASONS: {EXCLUDED_BOOKS_REASONS}"
)

# ---------------------------------------------------------------------------
# wiring_class aggregation (added 2026-08-07, per GE-09
# coverage-dashboard-requirements.md "Unit wiring-class reporting")
# ---------------------------------------------------------------------------
#
# `wiring_class` is orthogonal to `status`: GE-01's determinator writes it
# per unit onto every `data/corpus/**/*.json` record and it lands in
# docs/work-inventory.json's top-level `units` array (the same array
# build_unit_shards() reads for the unit-search shards above). It does NOT
# reach `v06_work_inventory --summary`'s output -- the cached summary this
# producer already loads via load_work_inventory() has no by_wiring_class
# anywhere in it -- so this has to read the full document directly, the same
# way build_unit_shards() does, and for the same reason it caches by source
# mtime: the doc is ~22MB and reparsing it every 5-minute cron tick when the
# source has not moved is pure waste.
#
# `ambiguous` is a first-class bucket here, not folded into anything. GE-09
# is explicit that an ambiguous unit is an undone work item, and no unit's
# `wiring_class` feeds `proven` in this producer -- proven stays exactly the
# by_status calculation in work_inventory_panel(), unchanged.
WIRING_CLASS_VALUES = ("display", "static", "derived", "computed", "ambiguous")
WIRING_CLASS_CACHE = os.environ.get(
    "PF1E_WIRING_CLASS_CACHE", os.path.expanduser("~/swarm-observer/wiring-class-summary.json")
)

# Bump when the cached summary's SHAPE changes. The cache is keyed on the
# source document's mtime alone, so without this a cache written by an older
# producer -- newer than an unchanged 22MB doc, and therefore "fresh" -- is
# reused forever and the new keys never appear. Read as a hard requirement,
# not a hint: a schema mismatch recomputes.
#
# Bumped 5 -> 6 round 4 (SD-29 QA finding F26, 2026-08-12): the DICT SHAPE
# didn't change, but doneness_verdict()'s `ambiguous` branch did (the
# `grounded` sub-case now resolves to `done` instead of `held`), and a cache
# keyed only on shape would silently keep serving round 3's wrong numbers
# forever if the source doc's mtime hadn't moved -- a logic change is exactly
# as invalidating as a shape change for a cache whose whole job is "don't
# recompute a value that would come out the same."
#
# Bumped 6 -> 7 round 5 (SD-29 QA finding #1, 2026-08-12): same hazard, same
# fix. doneness_verdict()'s `ambiguous` branch reverted `grounded` back to
# `held` (round 4's `done` shortcut was refuted by this file's own
# `static`/`derived` branch). Caught live: the cache under schema 6 kept
# serving 7 `race` records and other `ambiguous`+`grounded` units as `done`
# in `by_doneness_kind` for one publish cycle after this file's logic
# changed, because the source `work-inventory.json` mtime hadn't moved and
# schema 6 still matched. Confirms this field must be bumped on every
# doneness_verdict() semantics change, not just shape changes -- no
# exceptions, or the cache re-teaches the exact bug the code fix just
# removed.
#
# Bumped 7 -> 8 round 6 (SD-29 QA, 2026-08-12): same hazard, same fix again,
# this time in the `display` branch. `display` + `grounded` reverted from
# `done` to `in-progress` -- the round-4-shaped bug (an unresolved
# instrument disagreement resolved favorably instead of skeptically) had
# relocated from the `ambiguous` branch to the `display` branch. See the
# `display` branch of `doneness_verdict()` for the full reasoning.
#
# Bumped 11 -> 12 (SD30-E0-F2, 2026-08-14): same hazard again, this time in
# `NO_GROUNDING_PROBE` rather than `doneness_verdict()` itself --
# `NO_GROUNDING_PROBE` emptied from `("companion", "spell")` to `()` this
# cycle (both kinds now confirmed reaching a nonzero `grounded` count under
# `computed`; see that constant's own declaration). Caught live: a first run
# of this producer after the code change (`docs/work-inventory.json`'s mtime
# unchanged) served a stale schema-11 cache whose OWN baked-in
# `no_grounding_probe_kinds` still read `["companion", "spell"]` and whose
# `by_doneness`/`by_doneness_kind`/`cross_tab_by_kind` were still computed
# under the old cap (`held: 7048, in-progress: 716`, not the expected
# `held: 6916, in-progress: 848`) -- exactly
# `state-goals-and-lessons.md` hazard 5 ("silently serves a stale
# wiring-class cache"), reached through a producer-constant change this time
# rather than a corpus-doc change. `NO_GROUNDING_PROBE` feeds
# `doneness_verdict()`'s capping step directly, so it is exactly as
# invalidating as the branch-logic changes above.
# Bumped 12 -> 13 (SD-32 Epic 2 T8 follow-up, `decisions.md §11`): the T8
# fix added `classifier_reclassified_units` to this function's return dict
# but did not bump this constant, so every pre-T8 warm cache (also schema
# 12) passed the equality check below unchanged and the fix never fired
# against the real `WIRING_CLASS_CACHE` -- reproduced live on the tip of
# this bundle (cached schema 12, field absent, `corpus_wide` at the pre-fix
# values). This is the SAME hazard shape schema 11->12's own history above
# already documents once. See `StaleSchemaCacheIsRejectedTest` and
# `WiringSummaryTopLevelKeysCanaryTest` in
# `scripts/tests/test_pf1e_dashboard_producer.py` for the regression
# coverage this incident earned.
WIRING_SUMMARY_SCHEMA = 13

# ---------------------------------------------------------------------------
# Doneness (added 2026-08-12, operator directive; SD-29 `decisions.md §46`)
# ---------------------------------------------------------------------------
#
# `status` and `wiring_class` are two axes of ONE table and the dashboard was
# publishing only its margins. Doneness lives in the cells: it is the answer
# to "does the evidence this unit HAS meet the bar its class REQUIRES", and
# neither margin can answer it. `static` + `ingested-magnitude` (a longsword's
# COST:15, stored and reconciling at delta 0) and `computed` +
# `ingested-magnitude` (numbers held, no consumer delta ever observed) sit in
# the same `by_status` bucket and are nowhere near the same distance from done.
#
# The bars are GE-01's, not invented here -- see
# `docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/
# wiring-class-determination.md`, "The taxonomy":
#
#   display   DONE = the record is present and its description renders
#   static    DONE = stored value byte-equal to the corpus literal, AND
#                    rendered or consumed          <- NO INSTRUMENT EXISTS
#   derived   DONE = evaluator correct at sampled inputs vs a fixture
#                                                  <- NO INSTRUMENT EXISTS
#   computed  DONE = a real consumer observes a delta   (= `grounded`)
#
# Two of the four bars have no instrument, so `static` and `derived` CANNOT
# reach `done` today at any status. They resolve to `held`: the engine holds
# the record with its real numbers and reconciles against the corpus, and the
# confirming check does not exist yet. `held` is deliberately not `done` --
# SD-29 `decisions.md §46.4` rules that a percentage which rises on the
# strength of that decision alone is exactly the over-claim this axis exists
# to prevent. When the byte-equality sweep (`successor-forward-scope-
# register.md §C3.3`) and the fixture check land, those cells flip to `done`
# HERE, in one table, and every rollup follows.
#
# Note what `grounded` does and does not buy a non-`computed` unit. For
# `display`, `text-complete` is the actual bar (the description is present
# and renders) -> `done`; `grounded` and `ingested-magnitude` on a `display`
# unit are BOTH disagreement signals -- a consumer computed something from a
# unit the determinator says has no magnitude at all, which is evidence the
# `display` classification itself may be wrong -- so both are reported as
# `in-progress`, not credited as `done` (round 6, SD-29 QA, 2026-08-12; see
# the `display` branch of `doneness_verdict()`). For `static` and `derived`,
# `grounded` proves the rendered-or-consumed HALF and says nothing about
# fidelity of the value or correctness of the evaluator -> still `held`.
# Ranking `grounded` as universally done would smuggle the missing
# instruments back in.
#
# `ambiguous` means the wiring_class DETERMINATOR failed on the consumer
# side -- it does not mean the evidence is unreliable. Real evidence is still
# real evidence -- a `grounded` consumer delta, a complete `text-complete`
# record, or a recorded `ingested-magnitude` value do not become false
# because the wiring classifier separately failed. Discarding all of it into
# `unmeasurable` is exactly the over-claim-by-omission SD-29 QA finding #4
# (round 1, 2026-08-12) caught: 280 demonstrably-proven records vanished from
# coverage entirely. Round 4 (SD-29 QA finding F26, 2026-08-12) tried
# splitting the outcome by evidence tier, sending `ambiguous` + `grounded`
# straight to `done` on the theory that `grounded` "already clears the
# hardest bar in the lattice" regardless of wiring_class. THAT THEORY IS
# REFUTED (round 5, SD-29 QA finding #1, 2026-08-12) by this function's own
# `static`/`derived` branch: `static`/`derived` + `grounded` -> `held`,
# because `static`/`derived` carry a magnitude-fidelity bar `grounded`
# evidence alone does not clear. So `grounded` does NOT exceed every bar --
# crediting `ambiguous` + `grounded` with more confidence than a record
# whose wiring_class is actually KNOWN to be `static`/`derived` is a reward
# for not knowing, not a hedge. THE LIVE RULE (round 5, confirmed sound by
# round 6 QA): when wiring_class is unresolved, the verdict must never be
# MORE FAVORABLE than what the `static`/`derived` fidelity bar guarantees
# for the same status -- so the entire `ambiguous` bucket collapses to one
# rule, no per-status special-casing: any evidence-bearing status
# (`grounded`, `text-complete`, or `ingested-magnitude`) under an unresolved
# wiring class is `held`, never `done` (SD-29 decisions.md §46.4 -- `held`
# is deliberately not `done`).
# `unmeasurable` (the doneness verdict) is reserved for `status in ("unknown",
# "unmeasurable")` (the STATUS_VOCABULARY word, renamed from `unknown` by
# `AT-33-E4-002`, 2026-08-25 -- see `_doneness_verdict_uncapped`'s own
# checked-first branch), where the record itself gives no evidence at all
# regardless of wiring_class -- the honest bucket for "this instrument cannot
# classify this unit", a work item against the INSTRUMENTS, never folded into
# coverage in either direction.
DONENESS_DONE = "done"
DONENESS_HELD = "held"
DONENESS_IN_PROGRESS = "in-progress"
DONENESS_NOT_STARTED = "not-started"
DONENESS_UNMEASURABLE = "unmeasurable"
DONENESS_DEFERRED = "deferred"

# Ladder order: best-evidence first, then the two buckets that are not on the
# ladder at all (`unmeasurable`, `deferred`) last. The viewer renders in this
# order; do not re-sort by count.
DONENESS_VALUES = (
    DONENESS_DONE,
    DONENESS_HELD,
    DONENESS_IN_PROGRESS,
    DONENESS_NOT_STARTED,
    DONENESS_UNMEASURABLE,
    DONENESS_DEFERRED,
)

# Kinds the inventory has NO consumer-delta probe for at all -- `grounded`
# would be unreachable for them by construction, not merely unobserved yet.
#
# EMPTIED (SD30-E0-F2, 2026-08-14). This tuple's own justifying comment
# ("`companion` and `spell` alone read `grounded: 0`") is now FALSE, checked
# against the live `docs/work-inventory.json` this cycle, not transcribed:
# `computed`-wiring-class `companion` reads 416 `grounded` of 793, and
# `computed`-wiring-class `spell` reads 46 `grounded` of 210
# (`v06_work_inventory.rs`'s `Kind::Companion`/`Kind::Spell` verdict arms,
# `facts.holds_key`/`facts.spell_effect_wired` respectively). Both kinds'
# consumer-delta check already exists and already lands nonzero `grounded`,
# so per this card's own acceptance bar ("the cap is removed for a kind once
# its probe lands AND is confirmed reaching a nonzero `grounded` count under
# the `computed` class for that kind") neither belongs in this tuple any
# longer. `companion`'s cap was already inert regardless (its `computed`
# population is a strict `{grounded, not-ingested}` two-way split with no
# `in-progress`-shaped status to cap -- `build_companion_catalog()` in
# `apps/desktop/src-tauri/src/companion_catalog.rs` is a proven bijection
# over `companion_chassis::COMPANION_BOOKS`, own test
# `the_catalog_serves_every_registered_companion_creature`, so no unit can
# be "in the table but not grounded"). `spell`'s cap DOES move real units:
# emptying it reclassifies 132 `computed`+`ingested-magnitude` spell units
# from `held` to `in-progress` (verified by replaying `doneness_verdict()`
# over `docs/work-inventory.json` with and without the cap, this cycle) --
# a more honest board position, not a `done` gain (spell's `done` count is
# unchanged: `computed`+`grounded` was never subject to this cap, only
# `computed`+non-`grounded` was).
#
# Left as an empty tuple, not deleted, so a FUTURE kind whose probe
# genuinely cannot exist yet (there is none on record as of this cycle) has
# somewhere to be listed without re-deriving this comment's reasoning from
# scratch. Still single-sourced on the payload
# (`work_inventory.no_grounding_probe_kinds`) exactly as before; see
# `doneness_verdict()` below for how it is used, and PF1e-dashboard.html's
# `NO_GROUNDING_PROBE` for the client-side read -- that file's own fallback
# guard (`if (ngp.length) NO_GROUNDING_PROBE = ngp`) was fixed in the SAME
# change (SD30-E0-F2) to honor an explicitly-EMPTY shipped list rather than
# silently keeping its stale `["companion", "spell"]` default, which an
# emptied list would otherwise never be able to override.
NO_GROUNDING_PROBE = ()

DONENESS_MEANING = {
    DONENESS_DONE: (
        "The evidence this unit has meets the bar its wiring_class requires. "
        "Nothing further is owed. For `static`/`derived` specifically, this "
        "requires the literal/evaluator check the class has always named -- "
        "`static` cleared by `corpus_literal_sweep`'s byte-equality compare, "
        "`derived` cleared by `derived_evaluator_fixture_check`'s "
        "evaluator-vs-fixture compare -- carried in the record as `status` "
        "`literal-verified`/`fixture-verified`. Ordinary `ingested-magnitude`/"
        "`grounded`/`text-complete` evidence on a `static`/`derived` unit does "
        "NOT clear this bar and stays `held` (SD-32 decisions.md §2, operator "
        "directive 2026-08-13)."
    ),
    DONENESS_HELD: (
        "The engine holds the record with its real data. For `static`/`derived` "
        "the check that would confirm the bar now EXISTS -- `corpus_literal_sweep` "
        "for `static`, `derived_evaluator_fixture_check` for `derived` -- but this "
        "particular unit has not yet been swept/checked and found to pass (it "
        "carries ordinary `ingested-magnitude`/`grounded`/`text-complete` "
        "evidence rather than `literal-verified`/`fixture-verified`). A unit "
        "that HAS cleared its check reads `done`, not `held` (SD-32 "
        "decisions.md §2, operator directive 2026-08-13). For `ambiguous` and "
        "`display` (specifically `display`+`grounded`, where a real consumer "
        "delta contradicts the no-magnitude classification) the bar is not even "
        "known yet -- both need a working wiring-class classifier before there "
        "is a bar to check against at all. Also where a unit of any `kind` "
        "listed in `NO_GROUNDING_PROBE` would otherwise land `in-progress`: "
        "that kind has no consumer-delta probe at all, so its bar can never "
        "be confirmed reachable by an instrument that exists. As done as the "
        "current instruments can prove, and deliberately not counted as done "
        "(SD-29 decisions.md §46.4). `NO_GROUNDING_PROBE` is empty as of "
        "SD30-E0-F2 (2026-08-14) -- `spell` and `companion`, the two kinds it "
        "used to name, both have a landed probe reaching a nonzero `grounded` "
        "count under `computed` (46 and 416 respectively, re-derived that "
        "cycle), so this clause is currently vacuous by construction, not "
        "dead code -- a future kind with a genuinely unreachable bar has "
        "somewhere to be listed."
    ),
    DONENESS_IN_PROGRESS: (
        "Really part way: ingested, short of its own class's bar, and the bar is "
        "reachable with an instrument that exists. Dominated by `computed` units "
        "holding their numbers with no consumer delta yet observed. Would never "
        "be assigned to a `kind` listed in `NO_GROUNDING_PROBE` -- currently "
        "empty (SD30-E0-F2, 2026-08-14; see that constant's declaration) -- "
        "since for such a kind no instrument that could reach the bar would "
        "exist at all."
    ),
    DONENESS_NOT_STARTED: (
        "No record in the engine -- `not-ingested` (the book is in play, this "
        "unit is not) or `not-started` (the book has not been worked)."
    ),
    DONENESS_UNMEASURABLE: (
        "`status` came back `unknown`: the record itself gives no evidence at "
        "all, regardless of wiring_class, so there is nothing to place on the "
        "ladder. A work item against the instruments, not a statement about "
        "the content, and never counted toward or against coverage."
    ),
    DONENESS_DEFERRED: (
        "Ruled out of scope with a recorded reason. Not pending work."
    ),
}


def _exclude_books_from_kind_doneness(doneness_by_kind: dict,
                                       doneness_by_kind_by_book: dict,
                                       excluded_book_ids) -> dict:
    """Subtract excluded books' per-kind doneness from the corpus-wide rollup.

    `doneness_by_kind` on the wiring-class cache is built from every book,
    including ones this dashboard has ruled out of scope (currently
    `beginner_box`). Every other corpus-wide doneness figure subtracts
    excluded books before it reaches the payload; this one did not, so its
    per-kind ladder summed to more than `by_kind` (e.g. equipment read 19
    high, exactly beginner_box's excluded equipment units) even though the
    corpus-wide `by_doneness` total was correct. Returns a fresh dict; the
    input is not mutated.
    """
    out = {kind: dict(verdicts) for kind, verdicts in (doneness_by_kind or {}).items()}
    for book_id in excluded_book_ids or ():
        for kind, verdicts in (doneness_by_kind_by_book or {}).get(book_id, {}).items():
            if kind not in out:
                continue
            for verdict, count in verdicts.items():
                out[kind][verdict] = out[kind].get(verdict, 0) - count
                if out[kind][verdict] <= 0:
                    out[kind].pop(verdict, None)
            if not out[kind]:
                out.pop(kind, None)
    return out


def _exclude_books_from_flat_counts(counts: dict, counts_by_book: dict,
                                     excluded_book_ids) -> dict:
    """Subtract excluded books' contribution from a flat `key -> count` dict.

    Same shape of bug as `_exclude_books_from_kind_doneness` above, for a
    rollup that is one level flatter (`cross_tab`'s `"<wiring_class>|<status>"`
    cells rather than `kind -> verdict -> count`). Returns a fresh dict with
    any cell that nets to zero or below dropped; the input is not mutated.
    """
    out = dict(counts or {})
    for book_id in excluded_book_ids or ():
        for cell, count in (counts_by_book or {}).get(book_id, {}).items():
            if cell not in out:
                continue
            out[cell] = out[cell] - count
            if out[cell] <= 0:
                out.pop(cell, None)
    return out


def _mandate_headline(by_doneness: dict, denominator: int, books: list,
                       unmapped: dict) -> dict:
    """Operator ruling 2026-08-14/15: the headline is done / EVERY unit in every book except
    beginner_box -- future_state books, unmeasurable and deferred all stay in the denominator.
    Nothing is subtracted; the only way this number moves is a unit reaching `done`.

    `denominator` is the caller's already-adjusted `total_units` (excluded books already
    subtracted) -- passed in rather than re-derived here so this field can never disagree
    with the top-level `total_units` the same payload serves. `books` is the SAME per-book
    list `work_inventory_panel()` already built (excluded books already dropped from it),
    used here only to recompute the pre-2026-08-15 in-scope-and-measurable secondary figure
    for continuity, not to redefine the primary one.
    """
    in_scope = [b for b in books if b.get("scope") == "in_scope"]
    is_by: dict[str, int] = {}
    for b in in_scope:
        for v, c in (b.get("by_doneness") or {}).items():
            is_by[v] = is_by.get(v, 0) + c
    is_units = sum(b.get("units") or 0 for b in in_scope)
    unmapped_units = sum(unmapped.values())
    result = {
        "done": by_doneness.get(DONENESS_DONE, 0),
        "denominator": denominator,                       # == total_units (38,521 today)
        "denominator_rule": (
            "all units, all books except beginner_box, incl. unmeasurable, deferred and "
            "future_state; nothing subtracted"
        ),
        "unmapped_units": unmapped_units,                  # units absent from every rung; still in the denominator
        "secondary_in_scope_measurable": {
            "done": is_by.get(DONENESS_DONE, 0),
            "denominator": max(0, is_units - is_by.get(DONENESS_UNMEASURABLE, 0)
                                - is_by.get(DONENESS_DEFERRED, 0)),
            "rule": "in_scope books only, minus unmeasurable and deferred (the pre-2026-08-15 headline)",
        },
    }
    # stderr-only sanity check (Step 4C spec): the cron must keep publishing
    # even if this ever disagrees -- a raised exception here would blank the
    # whole dashboard over one arithmetic mismatch, a worse failure than a
    # logged warning.
    ladder_sum = sum(by_doneness.values())
    if ladder_sum + unmapped_units != denominator:
        print(
            f"WARNING: mandate_headline sanity check failed: ladder sum ({ladder_sum}) + "
            f"unmapped ({unmapped_units}) = {ladder_sum + unmapped_units} != "
            f"denominator ({denominator})",
            file=sys.stderr,
        )
    return result


def doneness_verdict(wiring_class: str, status: str, kind: str | None = None) -> str:
    """Cross `wiring_class` with `status` into one doneness verdict.

    Raises on any pair it has no rule for. That is the point: a new
    `wiring_class` value or a new status word must force this table to be
    updated rather than silently landing in whichever bucket a default picked,
    which is how `static`'s finished work became invisible in the first place.

    `kind` is optional and, when given, caps the result: `in-progress`'s own
    definition requires "the bar is reachable with an instrument that
    exists", which is false for any `kind` in `NO_GROUNDING_PROBE` -- those
    kinds can never reach `grounded` no matter how complete their data is, so
    "in progress toward a `grounded` verdict" is not an honest description
    for them. Applied structurally at the end of this function (round 8,
    SD-29 QA finding, 2026-08-12) rather than by hand-tuning the individual
    `display`/`computed` cells that produce `in-progress`, so a future new
    wiring_class/status combination that lands in `in-progress` is capped
    automatically rather than needing its own carve-out.
    """
    verdict = _doneness_verdict_uncapped(wiring_class, status)
    if verdict == DONENESS_IN_PROGRESS and kind in NO_GROUNDING_PROBE:
        return DONENESS_HELD
    return verdict


def _doneness_verdict_uncapped(wiring_class: str, status: str) -> str:
    """The (wiring_class, status) table `doneness_verdict()` caps by kind."""
    if status == "deferred-with-reason":
        return DONENESS_DEFERRED
    if status in ("not-ingested", "not-started"):
        return DONENESS_NOT_STARTED
    # An `unknown`/`unmeasurable` status cannot be measured against any bar,
    # classifiable or not -- checked first, ahead of both the ambiguous check
    # and the per-class rules below. `AT-33-E4-002` (2026-08-25) renamed the
    # STATUS_VOCABULARY word itself from `unknown` to `unmeasurable` (the 318
    # genuinely-irreducible units keep this exact disposition, only the
    # string changed, "so the status string itself stops reading as 'nobody
    # looked'" -- that commit's own message) but this one call site, and only
    # this one, was never updated to match: `unknown` no longer appears
    # anywhere in `STATUS_VOCABULARY`
    # (`src/bin/v06_work_inventory.rs`) or in any real
    # `docs/work-inventory.json` unit, so this branch alone had gone
    # unreachable for every live unit -- including the 11
    # `('ambiguous', 'unmeasurable')` units the `ambiguous` branch below
    # raises `ValueError` on, and the 310 `('display', 'unmeasurable')` units
    # that silently fall through the `display` branch's catch-all into
    # `in-progress` instead of the honest `unmeasurable` -- both populations
    # re-derived live:
    # `python3 -c "import json,collections;u=json.load(open('docs/work-inventory.json'))['units'];print(collections.Counter((x.get('wiring_class'),x.get('status')) for x in u if x.get('status')=='unmeasurable'))"`
    # -> `Counter({('display', 'unmeasurable'): 310, ('ambiguous', 'unmeasurable'): 11})`.
    # `unknown` is kept alongside `unmeasurable` rather than replaced outright
    # -- a frozen/older `work-inventory.json` snapshot generated before this
    # rename (e.g. an archived receipt's embedded fixture) still legitimately
    # carries the old word, and this function's whole design is "never
    # silently reinterpret a status word", not "assume every caller regenerated
    # today".
    if status in ("unknown", "unmeasurable"):
        return DONENESS_UNMEASURABLE
    # `ambiguous` wiring_class is a classifier failure on the CONSUMER side --
    # the determinator could not tell how the unit is wired in, so there is no
    # class-specific bar to check its evidence against. Trace the ACTUAL
    # control flow to see what can still be sitting here: `deferred-with-
    # reason`, `not-ingested`/`not-started` and `unknown` have all already
    # returned above, so the only statuses that can reach this line are
    # `grounded`, `text-complete` and `ingested-magnitude` -- i.e. every
    # remaining case IS real evidence of some tier, never a status this
    # instrument failed to read at all.
    #
    # Round 2 fix (SD-29 QA finding #4, 2026-08-12) correctly stopped
    # discarding that evidence into `unmeasurable` (the old order threw away
    # 280 demonstrably-proven records -- 175 race_trait `grounded`, 71 feat
    # `text-complete`, 23 equipment `text-complete`, 7 race `grounded`, plus 4
    # others -- purely because the wiring classifier separately failed) but
    # then routed ALL of it to `done`, which is the over-claim SD-29
    # decisions.md §46.4 exists to prevent. Round 3 (SD-29 QA finding #15/#16)
    # overcorrected the other way and routed ALL of it to `held` uniformly --
    # close, but it undersold `display`-shaped evidence elsewhere in this
    # function, so round 4 (SD-29 QA finding F26, 2026-08-12) tried a
    # three-way split by STATUS tier and routed `grounded` specifically to
    # `done`, on the theory that `grounded` (an OBSERVED consumer delta) is
    # strong enough on its own to clear every bar in the lattice regardless
    # of which wiring_class turns out to apply.
    #
    # That theory is FALSE, and it was refuted by this function's own
    # `static`/`derived` branch below: `static`/`derived` + `grounded` ->
    # `held`, because `static`/`derived` carry a magnitude-fidelity bar that
    # `grounded` evidence alone does not clear. So `grounded` does NOT exceed
    # every bar -- it exceeds `display`'s and `computed`'s bars but NOT
    # `static`/`derived`'s. An `ambiguous` record could BE a `static` or
    # `derived` record; routing its `grounded` evidence straight to `done`
    # crediting it with more confidence than a record whose wiring_class is
    # actually KNOWN to be `static`/`derived` is not a hedge, it's a reward
    # for not knowing.
    #
    # Round 5 (SD-29 QA finding #1, 2026-08-12) fixes this with the
    # lower-bound rule that governs the rest of this function everywhere
    # else (e.g. `unknown` -> `unmeasurable`): when wiring_class is
    # unresolved, the verdict must never be MORE FAVORABLE than what the
    # `static`/`derived` fidelity bar -- the specific bar this finding is
    # about -- guarantees for the SAME status. For `grounded` that bar is
    # `held` (the `static`/`derived` branch below returns `held` for it),
    # which is exactly the ceiling `ambiguous`+`grounded` must respect.
    # `text-complete`/`ingested-magnitude` were already `held` here since
    # round 3 (SD-29 QA finding #15/#16) and stay unchanged -- that mapping
    # is not this finding's scope, and re-deriving it from scratch this round
    # (e.g. against `computed`'s stricter `in-progress` for those two
    # statuses, a genuinely different, pre-existing axis untouched across 4
    # rounds of QA) would invent a NEW doctrine change nobody asked for, the
    # exact "re-deciding the core doctrine every round" pattern this round
    # exists to stop. So the entire `ambiguous` bucket collapses to one rule,
    # no per-status special-casing: any evidence-bearing status under an
    # unresolved wiring class is `held`.
    if wiring_class == "ambiguous":
        # `literal-verified`/`fixture-verified` are evidence-bearing statuses too: the stamp
        # proves the LITERAL matched (static's bar) or the EVALUATOR matched (derived's bar),
        # neither of which is known to be THIS unit's bar while its class is unresolved -- the
        # unit could be `computed`, whose bar (an observed consumer delta) neither stamp meets.
        # Same lower-bound rule as `grounded` above: never more favourable than the least
        # favourable class the unit could turn out to be. The generator cannot emit this cell
        # (its stamping loops are gated on Static/Derived and re-derived every run); it can only
        # arise from an in-place `wiring_class` rewrite, and on the next regen the stamp goes
        # away and the unit reads `held` anyway -- so `held` is the one verdict that does not
        # depend on which tool ran last. (Launch-readiness remediation Step 4D, blocker B6.)
        if status in ("grounded", "text-complete", "ingested-magnitude",
                      "literal-verified", "fixture-verified"):
            return DONENESS_HELD
        raise ValueError(f"doneness: unmapped {wiring_class!r} + {status!r}")
    if wiring_class == "display":
        # `text-complete` is display's actual bar -- the description is
        # present and renders, full stop.
        #
        # `grounded` and `ingested-magnitude` are BOTH disagreement signals,
        # not favorable ones -- but round 7 (SD-29 QA, 2026-08-12) found
        # round 6 collapsed them to the SAME bucket (`in-progress`), which is
        # wrong for `grounded` specifically, for two independent reasons.
        # `display` means the determinator found no magnitude anywhere on
        # the unit. `ingested-magnitude` says "the generator found one
        # anyway, but the consumer-delta probe that would confirm it has not
        # been RUN yet" -- an instrument that exists and is reachable, just
        # not yet exercised. That is a genuine `in-progress`: short of the
        # bar, with a path to close the gap.
        #
        # `grounded` is a different kind of disagreement: the consumer-delta
        # probe WAS run, and it produced evidence that contradicts the
        # `display` classification outright -- a real consumer computed
        # something from this unit, which a magnitude-absent record cannot
        # produce (see e.g.
        # `advanced_class_guide:class_feature:bloodrager_indomitable_will`,
        # classed `display` by a single-row magnitude check even though its
        # `type_facet` shows it inherits magnitude from the Bloodrage/Rage
        # rows -- `computed`-shaped content misclassified as `display`).
        # Routing that contradiction to `done` (round 5) rewarded the
        # classifier's blind spot; routing it to `in-progress` (round 6) is
        # ALSO wrong, because `in-progress`'s own definition below requires
        # "the bar is reachable with an instrument that exists" -- and the
        # instrument that would actually resolve this is a wiring-class
        # classifier that checks the full token closure GE-01 defines, which
        # does not exist yet. That is exactly the same "needs a working
        # classifier before the bar is even known" situation `ambiguous`
        # evidence is in (see the `ambiguous` branch above), so it gets the
        # same verdict: `held` -- as done as the current instruments can
        # prove, and deliberately not counted as done or as a reachable
        # in-progress item.
        return (DONENESS_DONE if status == "text-complete"
                else DONENESS_HELD if status == "grounded"
                else DONENESS_IN_PROGRESS)
    if wiring_class in ("static", "derived"):
        # `literal-verified` / `fixture-verified` are the done rung SD-29
        # decisions.md §46.4 and SD-32 decisions.md §2 both named as missing:
        # the generator emits these ONLY for a unit whose corpus literal was
        # actually byte-compared clean by `corpus_literal_sweep` (`static`) or
        # whose evaluator actually matched its pinned fixture
        # (`derived_evaluator_fixture_check`) -- never for a unit that merely
        # carries `ingested-magnitude`/`grounded`/`text-complete`, which stay
        # `held` exactly as before. The word is new and strictly stronger so
        # it can never be produced by an old inventory or confused with
        # `grounded`, which means something else (a consumer-delta
        # observation, not a literal/evaluator check). Operator directive
        # 2026-08-13 ("add the done rung for static and derived"), answering
        # SD-32 decisions.md §2's open question.
        if status in ("literal-verified", "fixture-verified"):
            return DONENESS_DONE
        if status in ("ingested-magnitude", "grounded", "text-complete"):
            return DONENESS_HELD
        raise ValueError(f"doneness: unmapped {wiring_class!r} + {status!r}")
    if wiring_class == "computed":
        return DONENESS_DONE if status == "grounded" else DONENESS_IN_PROGRESS
    raise ValueError(f"doneness: unknown wiring_class {wiring_class!r}")


def compute_wiring_class_summary(doc_path: str = WORK_INVENTORY_FULL_DOC,
                                  cache_path: str = WIRING_CLASS_CACHE) -> dict:
    """Corpus-wide and per-book `wiring_class` distributions.

    Returns `available: False` with a stated reason rather than a zeroed
    distribution when the source document is missing or unreadable, for the
    same reason build_unit_shards() does: a zero here reads as "no ambiguous
    units" rather than "we could not look".
    """
    if not os.path.exists(doc_path):
        return {"available": False,
                "note": f"{doc_path} not present; wiring_class summary unavailable"}

    try:
        src_mtime = os.path.getmtime(doc_path)
    except OSError as exc:
        return {"available": False, "note": f"could not stat {doc_path}: {exc}"}

    try:
        if os.path.getmtime(cache_path) >= src_mtime:
            with open(cache_path, encoding="utf-8") as f:
                cached = json.load(f)
            # Schema equality, not >=: a cache from a NEWER producer is as
            # unusable to this one as an older one, and silently serving either
            # is how a shape change fails to reach the payload.
            #
            # P0.2 hardening (state-goals-and-lessons.md §1.3 hazard 5): the
            # mtime check alone is not a staleness proof -- it only says the
            # cache is newer than THIS doc_path's mtime, not that the cache
            # was actually COMPUTED from this doc_path. A cache left over
            # from a run against a different doc (e.g. a manual verification
            # pointed at a scratch copy, or `cache_path` reused across two
            # different `doc_path` callers) can be newer than an unrelated
            # doc and still get served for it -- this is exactly how the
            # false zero happened during measurement. Require the cache's own
            # recorded `source_document` to match the doc_path being asked
            # for now, not just a schema and a timestamp.
            if (cached.get("available")
                    and cached.get("schema") == WIRING_SUMMARY_SCHEMA
                    and cached.get("source_document")
                        == publishable_document_path(doc_path)):
                return cached
    except (OSError, json.JSONDecodeError):
        pass

    try:
        with open(doc_path, encoding="utf-8") as f:
            doc = json.load(f)
    except (OSError, json.JSONDecodeError, MemoryError) as exc:
        return {"available": False, "note": f"could not read {doc_path}: {exc}"}

    corpus_wide: dict[str, int] = {}
    by_book: dict[str, dict[str, int]] = {}
    determinator_versions: set[str] = set()
    # The cross-tab is the joint distribution the two margins above are
    # projections of, and the only place doneness can be computed. Keyed
    # "<wiring_class>|<status>" because JSON has no tuple keys; the viewer
    # splits on the pipe. Neither axis carries a pipe -- both are closed
    # vocabularies of bare words.
    cross_tab: dict[str, int] = {}
    # book -> cell -> count, same reason doneness_by_kind_by_book exists: the
    # margin above cannot be adjusted for excluded books without something to
    # subtract from it (SD-29 QA finding #7, round 2, 2026-08-12 -- `cross_tab`
    # was the one rollup round 1's beginner_box exclusion missed, so it summed
    # to 38,540 units against every other rollup's 38,521).
    cross_tab_by_book: dict[str, dict[str, int]] = {}
    # kind -> cell -> count, same shape as doneness_by_kind above, so the
    # viewer's cross-tab caption (round 19, dash-frontend finding #2,
    # 2026-08-12) can report an exact per-kind count for specific cells
    # instead of a hand-typed literal ("178") that silently goes stale as
    # the corpus grows. Unadjusted for excluded books, same as cross_tab
    # itself; cross_tab_by_kind_by_book below carries what's needed to
    # subtract them in work_inventory_panel(), the same pattern as
    # doneness_by_kind_by_book / _exclude_books_from_kind_doneness.
    cross_tab_by_kind: dict[str, dict[str, int]] = {}
    cross_tab_by_kind_by_book: dict[str, dict[str, dict[str, int]]] = {}
    doneness: dict[str, int] = {}
    doneness_by_book: dict[str, dict[str, int]] = {}
    doneness_by_kind: dict[str, dict[str, int]] = {}
    # book -> kind -> verdict -> count. doneness_by_kind alone cannot be
    # adjusted for excluded books (2026-08-12 "readability" fix): the caller
    # subtracts excluded books' units from `by_doneness` and `by_status`
    # per-verdict, but had nothing to subtract from `doneness_by_kind` with,
    # so an excluded book's units (e.g. beginner_box) silently stayed baked
    # into the per-kind ladder while every other rollup dropped them -- the
    # per-kind sum stopped matching by_kind by exactly the excluded count.
    # This nested structure gives the caller what it needs to do that.
    doneness_by_kind_by_book: dict[str, dict[str, dict[str, int]]] = {}
    # A pair doneness_verdict() has no rule for is recorded and reported, not
    # crashed on and not defaulted into a bucket. A 5-minute cron that dies on
    # one novel unit publishes nothing at all, which is strictly worse than
    # publishing a stated gap -- but a silent default is worse than both.
    unmapped: dict[str, int] = {}
    # Kind -> count of the ONE tier that is mechanically confirmed rather than
    # merely filed correctly: `computed` wiring_class (the bar is "a real
    # consumer observes a delta") crossed with `grounded` status (that delta
    # WAS observed). The only other thing that reaches `done` -- `display`/
    # `text-complete` -- proves the record renders, not that it changes
    # anything in play (`display`/`grounded` no longer reaches `done`; since
    # round 7 it is `held`, per doneness_verdict()'s `display` branch).
    # Added round 3 (SD-29 QA finding #17, 2026-08-12) so the viewer can put
    # this sub-count on screen next to the headline "done" figure instead of
    # it only being derivable by hand from
    # the raw cross-tab.
    mechanically_confirmed_by_kind: dict[str, int] = {}
    mechanically_confirmed_by_kind_by_book: dict[str, dict[str, int]] = {}
    # Every kind actually seen in the corpus, so `mechanically_confirmed_by_kind`
    # can be zero-filled below rather than omitting a kind entirely when its
    # count is 0 (SD-29 QA finding F32, round 4, 2026-08-12): a kind absent
    # from the dict and a kind explicitly at 0 look identical to a naive `or 0`
    # read in the viewer, but they mean different things -- "we never checked"
    # vs. "we checked and it's zero, including the perfect case where 100% of
    # `done` is mechanically confirmed" (see mechConfirmedSuffix() in the
    # viewer for the >= done case this also has to distinguish, the same
    # "every rung renders including zero" rule GE-09 applies elsewhere).
    all_kinds_seen: set[str] = set()
    # SD-32 Epic 2 T8 (`decisions.md §11`; D13,
    # `docs/release/SD-31-corpus-closure-grind/todo/defects.md`): the
    # `wiring_class`-vs-`status` classifier blind spot. The determinator's
    # single-row `no_magnitude_token` heuristic stamps a unit `display`
    # without ever considering that `status == "grounded"` is itself real,
    # independent evidence a live consumer already computed something from
    # it -- exactly the `bloodrager_indomitable_will` case
    # `_doneness_verdict_uncapped`'s `display` branch's own doc comment
    # names, and "the instrument that would actually resolve this is a
    # wiring-class classifier that checks the full token closure GE-01
    # defines... does not exist yet."
    #
    # This block IS that missing check, narrowly and provably scoped: a
    # `display`+`grounded` unit is reclassified to `computed` (so
    # `doneness_verdict('computed', 'grounded', kind)` -> DONE fires for it,
    # the existing, unmodified rule -- `doneness_verdict()` itself is not
    # touched) only when its own `evidence` field independently corroborates
    # the claim: `explanation_id_observed_in_a_real_computation` means the
    # compute pipeline's own explanation-id trace, not this classifier,
    # already recorded a real computation touching this exact record. A
    # generic PREDICATE (kind, wiring_class, status, evidence), not a
    # hardcoded id list, so a future unit landing in the identical
    # evidence-corroborated cell is caught automatically -- Decision 11
    # condition 1: "proved by class... not by instance". Today this
    # predicate resolves to exactly D13's named 12 (`class_feature`, all
    # `core_rulebook`) -- re-derive with the command in this cycle's receipt.
    # `monster_ability` carries shape-alike siblings D13 itself flags as
    # "not yet swept" (they do not share this evidence string), so this
    # predicate does not silently widen past D13's own scope. `EXCLUDED_BOOKS`
    # applied inline, matching every other corpus-wide figure this
    # function/`work_inventory_panel()` produces.
    T8_RECLASSIFY_EVIDENCE = "explanation_id_observed_in_a_real_computation"
    classifier_reclassified_units: list[str] = []
    for unit in doc.get("units") or []:
        # No wiring_class on a unit is itself a gap, not a zero -- report it
        # under "ambiguous" rather than dropping the unit from the count.
        wc = unit.get("wiring_class") or "ambiguous"
        st = unit.get("status") or "unknown"
        kind = unit.get("kind") or "unknown"
        book = unit.get("book") or "unknown"
        # T8 (D13) reclassification -- see the block comment above the loop.
        # Applied BEFORE every rollup below reads `wc`, so corpus_wide,
        # by_book, cross_tab and doneness all reflect the corrected class,
        # not a shadow copy.
        if (kind == "class_feature" and wc == "display" and st == "grounded"
                and unit.get("evidence") == T8_RECLASSIFY_EVIDENCE
                and book not in EXCLUDED_BOOKS):
            wc = "computed"
            unit_id = unit.get("id") or f"{book}:{kind}:{unit.get('name')}"
            classifier_reclassified_units.append(unit_id)
        corpus_wide[wc] = corpus_wide.get(wc, 0) + 1
        by_book.setdefault(book, {})
        by_book[book][wc] = by_book[book].get(wc, 0) + 1
        all_kinds_seen.add(kind)
        cell = f"{wc}|{st}"
        cross_tab[cell] = cross_tab.get(cell, 0) + 1
        cross_tab_by_book.setdefault(book, {})
        cross_tab_by_book[book][cell] = cross_tab_by_book[book].get(cell, 0) + 1
        cross_tab_by_kind.setdefault(kind, {})
        cross_tab_by_kind[kind][cell] = cross_tab_by_kind[kind].get(cell, 0) + 1
        cross_tab_by_kind_by_book.setdefault(book, {}).setdefault(kind, {})
        cross_tab_by_kind_by_book[book][kind][cell] = (
            cross_tab_by_kind_by_book[book][kind].get(cell, 0) + 1)
        if wc == "computed" and st == "grounded":
            mechanically_confirmed_by_kind[kind] = (
                mechanically_confirmed_by_kind.get(kind, 0) + 1)
            mechanically_confirmed_by_kind_by_book.setdefault(book, {})
            mechanically_confirmed_by_kind_by_book[book][kind] = (
                mechanically_confirmed_by_kind_by_book[book].get(kind, 0) + 1)
        try:
            verdict = doneness_verdict(wc, st, kind)
        except ValueError:
            unmapped[cell] = unmapped.get(cell, 0) + 1
        else:
            doneness[verdict] = doneness.get(verdict, 0) + 1
            doneness_by_book.setdefault(book, {})
            doneness_by_book[book][verdict] = (
                doneness_by_book[book].get(verdict, 0) + 1)
            doneness_by_kind.setdefault(kind, {})
            doneness_by_kind[kind][verdict] = (
                doneness_by_kind[kind].get(verdict, 0) + 1)
            doneness_by_kind_by_book.setdefault(book, {}).setdefault(kind, {})
            doneness_by_kind_by_book[book][kind][verdict] = (
                doneness_by_kind_by_book[book][kind].get(verdict, 0) + 1)
        v = unit.get("wiring_class_determinator_version")
        if v:
            determinator_versions.add(v)

    # Zero-fill (SD-29 QA finding F32, round 4, 2026-08-12): every kind the
    # corpus actually has gets an explicit entry, even 0 -- see the comment at
    # this dict's declaration above for why "absent" and "0" cannot be
    # allowed to mean the same thing here.
    for kind in all_kinds_seen:
        mechanically_confirmed_by_kind.setdefault(kind, 0)

    result = {
        "available": True,
        "schema": WIRING_SUMMARY_SCHEMA,
        "generated_at": doc.get("generated_at"),
        "source_document": publishable_document_path(doc_path),
        "wiring_class_values": list(WIRING_CLASS_VALUES),
        "corpus_wide": corpus_wide,
        "by_book": by_book,
        # --- the joint table and its rollups (2026-08-12) ---
        "cross_tab": cross_tab,
        "cross_tab_by_book": cross_tab_by_book,
        "cross_tab_by_kind": cross_tab_by_kind,
        "cross_tab_by_kind_by_book": cross_tab_by_kind_by_book,
        "doneness_values": list(DONENESS_VALUES),
        "doneness_meaning": dict(DONENESS_MEANING),
        "doneness": doneness,
        "doneness_by_book": doneness_by_book,
        "doneness_by_kind": doneness_by_kind,
        "doneness_by_kind_by_book": doneness_by_kind_by_book,
        # `computed` + `grounded` per kind -- the mechanically-confirmed
        # sub-count of `done` (SD-29 QA finding #17, round 3, 2026-08-12).
        # See the comment at this dict's declaration above for why this is
        # the one tier that proves more than "filed correctly".
        "mechanically_confirmed_by_kind": mechanically_confirmed_by_kind,
        "mechanically_confirmed_by_kind_by_book": mechanically_confirmed_by_kind_by_book,
        # SD-32 Epic 2 T8 (D13) -- see the block comment above the main loop
        # for the full rationale. These units were reclassified `display` ->
        # `computed` in the loop above (before every rollup on this cache
        # read `wc`), so they already count as `computed` in `corpus_wide`/
        # `by_book`/`cross_tab`/`doneness` -- this field is the audit trail
        # naming WHICH units and WHY, not a second, separate bucket. Always
        # present, count 0 is a real "checked, none found" (Decision 1a: the
        # empty case must fail closed, never read as "the field doesn't
        # exist" / "this run didn't check").
        "classifier_reclassified_units": {
            "predicate": "kind=='class_feature' and wiring_class=='display' and "
                         "status=='grounded' and evidence=="
                         f"'{T8_RECLASSIFY_EVIDENCE}', EXCLUDED_BOOKS dropped",
            "reclassified_to": "computed",
            "count": len(classifier_reclassified_units),
            "units": sorted(classifier_reclassified_units),
        },
        # Empty is the expected state. Non-empty means a wiring_class or status
        # word appeared that doneness_verdict() has no bar for, and those units
        # are absent from every doneness rollup above -- so the ladder will not
        # sum to the unit total. Stated so that gap is visible instead of
        # showing up as arithmetic that mysteriously fails to add up.
        "doneness_unmapped": unmapped,
        # Single-sourced probeless-kind list (round 8, SD-29 QA finding,
        # 2026-08-12) -- see `NO_GROUNDING_PROBE`'s declaration above. Shipped
        # so the viewer reads this instead of hand-maintaining its own copy.
        "no_grounding_probe_kinds": list(NO_GROUNDING_PROBE),
        # Stated per GE-09's `wiring_class_determinator_version` requirement:
        # empty here means the field is not yet present on any unit in the
        # source document, which is a real gap in the upstream generator, not
        # something this producer can synthesize. Reported, not hidden.
        "determinator_versions": sorted(determinator_versions),
    }
    if not determinator_versions:
        result["determinator_version_note"] = (
            "no unit in the source document carries "
            "wiring_class_determinator_version; GE-01's determinator does not "
            "yet emit it. Reported as a gap, not defaulted."
        )
    err = _atomic_write_json(cache_path, result, validate=lambda _t: None,
                             keep_last_good=False)
    if err:
        print(f"pf1e-producer: wiring_class summary cache not written: {err}",
              file=sys.stderr)
    return result


# Round 21 QA finding 3: anchored to a tab-or-line-start boundary so this
# can only match a genuine `SCHOOL:` field token, never the tail of a longer
# token like `SUBSCHOOL:`/`PRESPELLSCHOOL:` (both real tokens elsewhere in
# the corpus, e.g. `SUBSCHOOL:` on non-spell power/maneuver .lst rows and
# `PRESPELLSCHOOL:` on class/feat prerequisite rows -- confirmed via
# `cat -A` that `.lst` fields are tab-delimited, so a genuine `SCHOOL:`
# token is always preceded by `\t` or starts the line).
_SCHOOL_TOKEN_RE = re.compile(r"(?:^|\t)SCHOOL:(\S+)")


def _book_source_dirs(doc: dict) -> dict:
    """book id -> absolute corpus directory, for the `.lst` school join.

    Every book directory basename IS the book id used everywhere else in this
    document (`unit["book"]`, `by_doneness_book_kind` keys, shard `book`
    column) -- confirmed against `doc["books"]` round 20. `corpus_root` holds
    the Paizo roleplaying-game books directly as children; each entry in
    `additional_book_dirs` (3rd-party and campaign-setting books) IS itself
    one book's directory, not a parent to scan.
    """
    dirs: dict[str, str] = {}
    root = doc.get("corpus_root")
    if root and os.path.isdir(root):
        for name in os.listdir(root):
            p = os.path.join(root, name)
            if os.path.isdir(p):
                dirs[name] = p
    for d in doc.get("additional_book_dirs") or []:
        if os.path.isdir(d):
            dirs[os.path.basename(d)] = d
    return dirs


def _spell_schools(spell_units: list, doc: dict) -> tuple[dict, dict]:
    """Join each spell unit to its `SCHOOL:` token via `source_file`+`source_line`.

    Returns (id -> school-or-None, stats dict). Positional join: every spell
    unit's `source_file`/`source_line` already point at the exact `.lst` line
    the determinator read (confirmed round 20 by spot-checking the joined
    line's own leading name field against `unit["name"]` -- they match modulo
    the `.MOD`/`.COPY=` suffix PCGen's own patch/variant records carry, which
    is expected and not a join failure). No fuzzy name-based matching is used
    anywhere in this join -- a wrong SCHOOL for one spell would be silent and
    worse than reporting `None` for it.

    A `None` result (not folded into "Divination" or any other default) means
    either: the unit carries no source_file/source_line, the book's directory
    could not be resolved, the line is out of range, or the line has no
    `SCHOOL:` token at all (true for `.MOD` patch lines and a handful of
    other non-base-spell rows) -- reported to the viewer as literal `n/a`.
    """
    book_dirs = _book_source_dirs(doc)
    file_cache: dict[str, list | None] = {}
    result: dict[str, str | None] = {}
    stats = {"ok": 0, "no_location": 0, "file_not_found": 0, "line_out_of_range": 0, "no_school_token": 0}
    for u in spell_units:
        uid = u.get("id") or id(u)
        book = u.get("book")
        sf = u.get("source_file")
        sl = u.get("source_line")
        bookdir = book_dirs.get(book)
        if not bookdir or not sf or not sl:
            result[uid] = None
            stats["no_location"] += 1
            continue
        path = os.path.join(bookdir, sf)
        if path not in file_cache:
            try:
                with open(path, encoding="utf-8", errors="replace") as fh:
                    file_cache[path] = fh.readlines()
            except OSError:
                file_cache[path] = None
        lines = file_cache[path]
        if lines is None:
            result[uid] = None
            stats["file_not_found"] += 1
            continue
        idx = sl - 1
        if idx < 0 or idx >= len(lines):
            result[uid] = None
            stats["line_out_of_range"] += 1
            continue
        m = _SCHOOL_TOKEN_RE.search(lines[idx])
        if not m:
            result[uid] = None
            stats["no_school_token"] += 1
            continue
        result[uid] = m.group(1)
        stats["ok"] += 1
    return result, stats


def _category_bucket() -> dict:
    return {v: 0 for v in DONENESS_VALUES}


def build_unit_shards(doc_path: str = WORK_INVENTORY_FULL_DOC,
                      shard_dir: str = UNIT_SHARD_DIR,
                      declared_pi_names: set | None = None,
                      pi_screen: "_PiScreen | None" = None) -> dict:
    """Emit per-kind unit shards and return the index the viewer navigates.

    `declared_pi_names` is the full-oracle name index (see the module-level
    comment on its build site below) -- pass a pre-built one from a caller
    that also needs it elsewhere in the same run (`main()` reuses one
    instance for both this function and the top-level document's own
    blanket sweep, rather than paying the ~2.5s oracle walk twice). `None`
    (the default, and every existing/test call site) builds it locally so
    this function stays independently callable.

    Returns a dict with `available: False` and a stated reason when the source
    document is absent or unreadable -- never a zeroed index, because a zero
    unit count reads as "nothing to do" rather than "we could not look".
    """
    if not os.path.exists(doc_path):
        return {"available": False,
                "note": f"{doc_path} not present; unit search unavailable"}

    manifest_path = os.path.join(shard_dir, "index.json")
    try:
        src_mtime = os.path.getmtime(doc_path)
    except OSError as exc:
        return {"available": False, "note": f"could not stat {doc_path}: {exc}"}

    # Reparsing 22MB every 5 minutes is pure waste when the source has not
    # moved; reuse the existing index whenever it is at least as new. Also
    # requires a matching SHARD_SCHEMA (round 20 addition -- see that
    # constant's comment): without this a cache written before `type_facet`
    # was added would be newer than an unchanged source doc and would keep
    # serving pre-round-20 shards forever.
    try:
        if os.path.getmtime(manifest_path) >= src_mtime:
            with open(manifest_path, encoding="utf-8") as f:
                cached = json.load(f)
            if cached.get("available") and cached.get("schema") == SHARD_SCHEMA:
                return cached
    except (OSError, json.JSONDecodeError):
        pass

    try:
        with open(doc_path, encoding="utf-8") as f:
            doc = json.load(f)
    except (OSError, json.JSONDecodeError, MemoryError) as exc:
        return {"available": False, "note": f"could not read {doc_path}: {exc}"}

    # Same EXCLUDED_BOOKS exclusion every other corpus-wide figure applies.
    # Missed here in round 1 (SD-29 QA finding #8, round 2, 2026-08-12): the
    # unit-search index read 6,227 equipment units against the by-lane card's
    # 6,208 for the same kind -- exactly beginner_box's 19 equipment records,
    # reachable by a normal user clicking Equipment on #/corpus.
    grouped: dict[str, list] = {}
    for unit in doc.get("units") or []:
        if (unit.get("book") or "unknown") in EXCLUDED_BOOKS:
            continue
        grouped.setdefault(unit.get("kind") or "unknown", []).append(unit)

    try:
        os.makedirs(shard_dir, exist_ok=True)
        os.chmod(shard_dir, 0o755)
    except OSError as exc:
        return {"available": False, "note": f"could not create {shard_dir}: {exc}"}

    # Decision 12 (2026-08-17): "withhold the name, keep the row." Every
    # unit's own (book, source_file, source_line) is cross-checked against
    # the pinned oracle's own NAMEISPI:YES declaration -- built ONCE here
    # and reused for all ~38k units, not per-unit, since the same
    # (book, source_file) pair repeats across an entire LST file's worth of
    # sibling records. `oracle_checker.available` is false only when the
    # pinned checkout itself could not be found (a machine that never ran
    # `scripts/fetch-pcgen-oracle.sh`); the shard still ships in that case
    # -- degrading availability of a PI SCREEN is not degrading to "ship
    # unscreened," it is reported alongside the shard index below so a
    # reader can tell the two apart.
    oracle_checker = pi_redaction.OracleNameChecker()
    pi_redacted_total = 0
    # Defense-in-depth (found this cycle, real gap): a `.MOD` row that
    # merely tags an EXISTING declared-PI record (e.g. inner_sea_world_
    # guide's PFS-legality `.MOD` rows for races originally declared PI in
    # a different book) carries no `NAMEISPI:YES` token on its OWN line,
    # so `oracle_checker.declared()` -- which reads only the unit's exact
    # cited coordinate -- correctly reports no declaration THERE while the
    # object is still genuinely PI. A blanket exact-name sweep over the
    # finished `rows`, using the SAME full-oracle name index
    # `scripts/site_dashboard_pi_gate.py` scans the committed feed with,
    # closes that gap without needing every `.MOD` reference resolved
    # back to its defining row.
    declared_pi_name_index = (
        declared_pi_names if declared_pi_names is not None
        else pi_redaction.build_declared_pi_name_index()
    )
    # Per-book counterpart (SD31-W13-INTEGRATE-001 finding 2): the flat
    # index above is deliberately conservative -- it drops any name that is
    # PI in one book and a genuinely different, non-PI object in another
    # (Teleport, Shield), which also means it drops a name that is the SAME
    # object, legitimately PI in one book and legitimately not another
    # (`Weapon and Armor Proficiency`: PI at inner_sea_magic:175, ordinary
    # everywhere else). Rows already carry `book`, so this closes that gap
    # with context the flat set structurally cannot have.
    declared_pi_name_books = pi_redaction.build_declared_pi_name_book_index()
    # FIX-DASHBOARD-PI (2026-08-17): the word-boundary layer (see
    # `_PiScreen`'s own docstring) -- built from the SAME
    # `declared_pi_name_index`/`declared_pi_name_books` just constructed
    # above rather than re-walking the oracle a third time, unless a caller
    # already built one for the whole run and passed it in.
    screen = (
        pi_screen if pi_screen is not None
        else _PiScreen(declared_pi_names=declared_pi_name_index,
                       declared_pi_name_books=declared_pi_name_books)
    )
    category_labels_redacted = 0

    def _screen_category_label(label, screen_obj):
        nonlocal category_labels_redacted
        screened = screen_obj.screen(label, book=None)
        if screened != label:
            category_labels_redacted += 1
        return screened

    kinds: dict[str, dict] = {}
    for kind, units in sorted(grouped.items()):
        is_spell = kind == "spell"
        fields = SPELL_SHARD_FIELDS if is_spell else UNIT_SHARD_FIELDS
        schools_by_id: dict = {}
        school_stats = None
        if is_spell:
            schools_by_id, school_stats = _spell_schools(units, doc)

        # No wiring_class on a unit is a gap, not a zero -- same "ambiguous"
        # default compute_wiring_class_summary() uses (line ~2690), so a
        # record missing the field still resolves to a real doneness verdict
        # client-side rather than reading as `undefined`.
        def _field(u, f):
            if f == "wiring_class":
                return u.get("wiring_class") or "ambiguous"
            if f == "school":
                return schools_by_id.get(u.get("id") or id(u))
            if f == "name":
                name_is_pi, _ = oracle_checker.declared(
                    u.get("book"), u.get("source_file"), u.get("source_line")
                )
                if name_is_pi:
                    nonlocal pi_redacted_total
                    pi_redacted_total += 1
                    return pi_redaction.REDACTED_PI_MARKER
                return u.get(f)
            return u.get(f)

        rows = [[_field(u, f) for f in fields] for u in units]
        # Blanket defense-in-depth sweep (see comment on
        # `declared_pi_name_index` above): catches a declared-PI name that
        # the exact-coordinate check above missed (a `.MOD` row citing an
        # object whose declaration lives on a different line entirely).
        # Exact-match only (`in declared_pi_name_index`), never a substring
        # scan -- `redact_declared_pi_names`'s own docstring covers why.
        name_idx = fields.index("name") if "name" in fields else None
        book_idx = fields.index("book") if "book" in fields else None
        if name_idx is not None:
            for row in rows:
                val = row[name_idx]
                if isinstance(val, str) and val != pi_redaction.REDACTED_PI_MARKER and val in declared_pi_name_index:
                    row[name_idx] = pi_redaction.REDACTED_PI_MARKER
                    pi_redacted_total += 1
        # Per-book pass (see `declared_pi_name_books` above): runs AFTER the
        # flat sweep so it only ever has to check rows the flat, book-blind
        # index left alone -- a name already redacted is skipped by the
        # `!= REDACTED_PI_MARKER` guard, same as the flat pass.
        if name_idx is not None and book_idx is not None:
            for row in rows:
                val = row[name_idx]
                if not isinstance(val, str) or val == pi_redaction.REDACTED_PI_MARKER:
                    continue
                if row[book_idx] in declared_pi_name_books.get(val, ()):
                    row[name_idx] = pi_redaction.REDACTED_PI_MARKER
                    pi_redacted_total += 1
        # Word-boundary pass (FIX-DASHBOARD-PI, 2026-08-17): the two passes
        # above are EXACT-match only (a `name` field that IS, verbatim, a
        # declared-PI name) -- structurally blind to a genuine EMBED like
        # `"Helm of the Serpent King"` (the declared-PI record's own name
        # is just `"The Serpent King"`) or `"Death (Pharasma)"`, the same
        # gap `build_public_status.py::redact_for_display` already closed
        # for the public status projection. Runs AFTER both exact passes
        # (the `!= REDACTED_PI_MARKER` guard skips anything already
        # redacted) and is gated by the SAME shared, reviewed allow-list.
        if name_idx is not None and book_idx is not None:
            for row in rows:
                val = row[name_idx]
                if not isinstance(val, str) or val == pi_redaction.REDACTED_PI_MARKER:
                    continue
                screened = screen.screen(val, row[book_idx])
                if screened != val:
                    row[name_idx] = screened
                    pi_redacted_total += 1
        # `type_facet` pass (FIX-DASHBOARD-PI, 2026-08-17): this shard field
        # ships the RAW PCGen TYPE token verbatim (`"ClassFeatures.Hellknight
        # Signifer Class Feature.SpecialQuality.Extraordinary"`) and had NO
        # screen of any kind before this fix -- a real leak, independent of
        # `name`. `type_facet` is a compound machine identifier, not
        # natural-language prose, so this uses PLAIN SUBSTRING matching
        # (`value_carries_declared_pi_substring`), GLOBALLY, with no
        # allow-list -- the SAME convention (and the SAME PROVEN 30-hit
        # false-positive-free result on this exact class of token)
        # `build_public_status.py`'s own `type_facet` screen already
        # established for the public status projection; see that module's
        # own comment for the substring-vs-word-boundary rationale specific
        # to this field's shape.
        type_facet_idx = fields.index("type_facet") if "type_facet" in fields else None
        if type_facet_idx is not None:
            for row in rows:
                tf = row[type_facet_idx]
                if (isinstance(tf, str) and tf != pi_redaction.REDACTED_PI_MARKER
                        and pi_redaction.value_carries_declared_pi_substring(tf, screen.by_length)):
                    row[type_facet_idx] = pi_redaction.REDACTED_PI_MARKER
                    pi_redacted_total += 1
        by_status: dict[str, int] = {}
        # Sub-category rollup (round 20): per-category doneness-verdict
        # counts, computed with the SAME doneness_verdict() every other
        # rollup on this payload uses -- this is a grouping/display axis on
        # top of an unchanged measurement, not a new instrument. `category`
        # (from `type_facet`'s first segment, "none" when absent) is always
        # present; `school_category` (spell only, from the SCHOOL: join) is
        # `None`/absent whenever the join found nothing, reported as its own
        # explicit "n/a" bucket rather than silently dropped or merged into
        # "none".
        categories: dict[str, dict] = {}
        # Round 22 QA finding 2: `categories` is keyed by `category_group_key()`
        # (normalized), not the raw `category_of()` segment. Round 23 QA
        # finding 4: this now tracks EVERY raw spelling seen per group key,
        # with a per-spelling record count, rather than just the first one
        # seen -- `pick_category_representative()` (above) picks the
        # display-label source deterministically from this, instead of the
        # old row-order-dependent `setdefault` "first seen wins".
        category_raw_variants: dict[str, dict[str, int]] = {}
        school_categories: dict[str, dict] = {} if is_spell else None
        multi_school_count = 0
        multi_school_extra_credits = 0
        # P0.2 hardening (state-goals-and-lessons.md §1.3 hazard 4): this call
        # was the one doneness_verdict() site with no try/except around it --
        # every OTHER caller (compute_wiring_class_summary()'s cross_tab loop,
        # ~line 3654) already degrades a (wiring_class, status) pair the table
        # has no bar for into an `unmapped` bucket instead of raising. This
        # loop used to let that same ValueError escape uncaught, which crashed
        # build_unit_shards() -> build_pf1e_dashboard() -> main() on the FIRST
        # unrecognised status word the generator ever emits, publishing
        # nothing for that whole cron tick instead of degrading. Match the
        # sibling caller's posture: bucket as `unmeasurable` (the verdict this
        # table already uses for "an instrument could not read this"), log it
        # loudly, and keep counting -- one bad status word must never take
        # down the other 44k units' worth of data.
        shard_doneness_unmapped: dict[str, int] = {}
        for u in units:
            st = u.get("status") or "unknown"
            by_status[st] = by_status.get(st, 0) + 1
            wc = u.get("wiring_class") or "ambiguous"
            try:
                verdict = doneness_verdict(wc, st, kind)
            except ValueError:
                cell = f"{wc}|{st}"
                shard_doneness_unmapped[cell] = shard_doneness_unmapped.get(cell, 0) + 1
                print(f"pf1e-producer: WARNING doneness_verdict has no bar for "
                      f"wiring_class={wc!r} status={st!r} kind={kind!r} "
                      f"(unit id={u.get('id')!r}) -- degrading to 'unmeasurable' "
                      f"instead of crashing", file=sys.stderr)
                verdict = DONENESS_UNMEASURABLE
            cat = category_of(u.get("type_facet"))
            gkey = category_group_key(cat)
            bucket = categories.setdefault(gkey, _category_bucket())
            bucket[verdict] += 1
            variants = category_raw_variants.setdefault(gkey, {})
            variants[cat] = variants.get(cat, 0) + 1
            if is_spell:
                sch = schools_by_id.get(u.get("id") or id(u)) or "n/a"
                # Round 21 QA finding 5: a small number of spells (Words of
                # Power spells in um_spells_wordsofpower.lst) carry a
                # pipe-delimited SCHOOL: token with more than one school
                # (e.g. `Abjuration|Necromancy|Transmutation`). Rather than
                # publish that raw pipe string as its own untranslated
                # single-record group (which also undercounted every real
                # school it named), credit the spell toward EACH listed
                # school -- same convention as a multi-classed unit
                # counting toward every class it holds elsewhere in this
                # producer. This means `school_categories` bucket totals no
                # longer sum to exactly the spell count; `multi_school_count`
                # below documents the deliberate overcount so the
                # reconciliation guard doesn't need to be silently wrong.
                schools_here = sch.split("|") if sch != "n/a" else [sch]
                for one_sch in schools_here:
                    sbucket = school_categories.setdefault(one_sch, _category_bucket())
                    sbucket[verdict] += 1
                if len(schools_here) > 1:
                    multi_school_count += 1
                    multi_school_extra_credits += len(schools_here) - 1
        filename = f"PF1e-units-{kind}.json"
        # indent=None: these are machine-read only, and pretty-printing 18k
        # rows triples the bytes on the wire for no reader benefit.
        err = _atomic_write_json(
            os.path.join(shard_dir, filename),
            {"kind": kind, "fields": list(fields), "rows": rows},
            validate=_validate_shard,
            indent=None,
            keep_last_good=False,
        )
        if err:
            print(f"pf1e-producer: shard {kind} not written: {err}", file=sys.stderr)
            continue
        # Reconciliation guard, not just a convenience: every category
        # bucket's total (across all six doneness verdicts) must sum to
        # `len(rows)` -- each unit lands in exactly one category, so this can
        # never legitimately be false. Raised loudly rather than published
        # silently wrong, same posture as `_validate_shard` above.
        cat_total = sum(sum(b.values()) for b in categories.values())
        if cat_total != len(rows):
            print(f"pf1e-producer: WARNING category rollup for {kind} sums to "
                  f"{cat_total}, expected {len(rows)}", file=sys.stderr)
        kind_entry = {
            "units": len(rows),
            "proven": sum(by_status.get(s, 0) for s in PROVEN_STATUSES),
            "by_status": by_status,
            "shard": filename,
            "bytes": os.path.getsize(os.path.join(shard_dir, filename)),
            # Empty is the expected state (same convention as
            # compute_wiring_class_summary()'s `doneness_unmapped`, ~line
            # 3709): non-empty means a (wiring_class, status) pair with no
            # doneness bar appeared in THIS kind's units and was degraded to
            # `unmeasurable` rather than crashing the producer.
            "doneness_unmapped": shard_doneness_unmapped,
            # Round 22 QA finding 2: keyed by the normalized
            # `category_group_key()`, not the raw `category_of()` segment.
            # Round 23 QA finding 4: `pick_category_representative()` chooses
            # ONE representative raw spelling per group deterministically
            # (override-presence, then record count, then alphabetical) --
            # not whichever spelling happened to be iterated over first.
            "categories": {
                gkey: {
                    # FIX-DASHBOARD-PI (2026-08-17): a category label is a
                    # BUILT-UP string (the TYPE token's own first segment,
                    # translated for display) -- exactly the shape
                    # `site_dashboard_pi_gate.py`'s own "KNOWN RESIDUAL GAP"
                    # note warned exact-leaf matching is blind to
                    # (`"Varisian Pilgrim Domain"` embeds the declared-PI
                    # archetype name `"Varisian Pilgrim"` verbatim). No
                    # `book` to scope against here -- a category is
                    # aggregated across every book in the whole KIND -- so
                    # `screen()` runs the GLOBAL word-boundary check, gated
                    # by `is_allowlisted_for_any_book` instead of a
                    # per-book entry.
                    "label": _screen_category_label(
                        category_label(pick_category_representative(category_raw_variants[gkey])),
                        screen,
                    ),
                    **buckets,
                }
                for gkey, buckets in sorted(categories.items(),
                                             key=lambda kv: -sum(kv[1].values()))
            },
            "category_axis_label": "Category",
        }
        if is_spell:
            # Tradition (Arcane/Divine/Psychic) IS `type_facet`'s first
            # segment for spells -- same `categories` field as every other
            # kind, just relabelled for the axis name so the viewer does not
            # need a spell-specific special case to know what it is showing.
            kind_entry["category_axis_label"] = "Tradition"
            kind_entry["school_categories"] = {
                sch: {"label": _screen_category_label(sch, screen), **buckets}
                for sch, buckets in sorted(school_categories.items(),
                                            key=lambda kv: -sum(kv[1].values()))
            }
            kind_entry["school_join"] = {
                "method": "positional: book directory (resolved from book id) "
                          "+ source_file + source_line, SCHOOL: token on that "
                          "exact .lst line -- no name-based matching",
                "stats": school_stats,
                "coverage_pct": round(100 * school_stats["ok"] / len(units), 1) if units else None,
                # Round 21 QA finding 5: a spell with a pipe-delimited
                # SCHOOL: token (e.g. "Abjuration|Necromancy|Transmutation")
                # is credited toward every school it names, so the sum of
                # `school_categories` bucket totals is `len(rows)` PLUS one
                # extra credit per additional school on a multi-school
                # spell, not exactly `len(rows)`. This count makes that
                # deliberate overcount explicit rather than silent.
                "multi_school_spell_count": multi_school_count,
                # The number to actually add to `len(rows)` to get the
                # expected `school_categories` bucket-total sum: one extra
                # credit per ADDITIONAL school beyond the first on each
                # multi-school spell (a 3-school spell contributes 2 extra,
                # not 1) -- deliberately a different number than
                # `multi_school_spell_count` above, which just counts spells.
                "multi_school_extra_credits": multi_school_extra_credits,
            }
            cat2 = sum(sum(b.values()) for b in school_categories.values())
            expected_cat2 = len(rows) + multi_school_extra_credits
            if cat2 != expected_cat2:
                print(f"pf1e-producer: WARNING school rollup for {kind} sums to "
                      f"{cat2}, expected {expected_cat2} (= {len(rows)} spells + "
                      f"{multi_school_count} multi-school credit(s))", file=sys.stderr)
        kinds[kind] = kind_entry

    # Dedicated degrade-path flag (P0.2 hardening): True whenever ANY kind
    # hit a (wiring_class, status) pair doneness_verdict() has no bar for.
    # Distinct from `status_sources_agree` (a different, pre-existing
    # SWARM_STATUS.md-vs-report_text signal) so this doesn't overload that
    # flag's meaning -- a viewer or on-call human can tell "the generator
    # emitted a status word we've never seen" apart from "the two status
    # sources disagree" at a glance.
    doneness_unmapped_seen = any(k["doneness_unmapped"] for k in kinds.values())
    index = {
        "available": bool(kinds),
        "schema": SHARD_SCHEMA,
        "generated_at": doc.get("generated_at"),
        "source_document": publishable_document_path(doc_path),
        "shard_base": "units/",
        "fields": list(UNIT_SHARD_FIELDS),
        "total_units": sum(k["units"] for k in kinds.values()),
        "proven_units": sum(k["proven"] for k in kinds.values()),
        "doneness_unmapped_seen": doneness_unmapped_seen,
        # Decision 12: how many unit names were withheld this run and
        # whether the pinned oracle was even reachable to screen against.
        # The row itself is never dropped for a name-PI hit (`units` above
        # already counts it); this reports only the display substitution.
        "pi_redacted_names": pi_redacted_total,
        # FIX-DASHBOARD-PI (2026-08-17): how many `categories[*].label`/
        # `school_categories[*].label` strings (built-up TYPE-token
        # translations, not raw unit names) were withheld this run --
        # tracked separately from `pi_redacted_names` because it is a
        # different kind of substitution (a whole category's display
        # label, not one unit's row).
        "category_labels_redacted": category_labels_redacted,
        "pi_oracle_available": oracle_checker.available,
        "kinds": kinds,
    }
    if not kinds:
        index["note"] = "no shards could be written; unit search unavailable"
    _atomic_write_json(manifest_path, index, validate=lambda _t: None,
                       keep_last_good=False)
    return index


def main() -> int:
    import argparse
    p = argparse.ArgumentParser(description="PF1e dashboard JSON producer.")
    p.add_argument("--status", default=os.environ.get("SWARM_STATUS", DEFAULT_STATUS))
    p.add_argument("--risks-doc", default=os.environ.get("SWARM_RISKS_DOC", DEFAULT_RISKS_DOC))
    p.add_argument("--report", default=os.environ.get("SWARM_REPORT", DEFAULT_REPORT))
    p.add_argument("--usage-cache", default=os.environ.get("SWARM_USAGE_CACHE", DEFAULT_USAGE_CACHE))
    p.add_argument("--out", default=os.environ.get("PF1E_JSON_OUT", DEFAULT_OUT))
    args = p.parse_args()

    refreshed = dt.datetime.now(dt.timezone.utc)
    status_text = _observer.read_status_raw(args.status)
    risks_text = pathlib.Path(args.risks_doc).read_text(encoding="utf-8", errors="replace") if pathlib.Path(args.risks_doc).exists() else ""
    report_text = pathlib.Path(args.report).read_text(encoding="utf-8", errors="replace") if pathlib.Path(args.report).exists() else ""
    usage = _observer.read_usage(args.usage_cache)

    # Decision 12 (2026-08-17) / FIX-DASHBOARD-PI (2026-08-17): one shared
    # `_PiScreen` (full-oracle declared-PI name index, flat and per-book,
    # plus the shared allow-list), built ONCE and threaded through
    # `build_pf1e_dashboard` (which threads it further into
    # `_book_item_roster`/`_prestige_classes`) and `build_unit_shards`
    # below -- a ~2.5s oracle walk is not paid three times in the same run.
    pi_screen = _PiScreen()
    data = build_pf1e_dashboard(status_text, risks_text, args.risks_doc, usage, refreshed, report_text,
                                pi_screen=pi_screen)
    # Load prior owner-managed state (manifests items, SD-27+ channels) and
    # merge into the regenerated data. This preserves the lead's manifest
    # writes and the orchestrator's SD-27+ channel data across producer runs.
    prior_state = _load_existing_owner_state(args.out)
    data = _merge_owner_state(data, prior_state)
    # Written alongside the payload rather than inside it: the shards are the
    # drill-down layer, and inlining 44k units would defeat the point.
    data["unit_index"] = build_unit_shards(
        shard_dir=os.path.join(os.path.dirname(os.path.abspath(args.out)), "units"),
        declared_pi_names=pi_screen.names,
        pi_screen=pi_screen,
    )
    # Blanket defense-in-depth sweep over the WHOLE top-level document --
    # catches a declared-PI name surfacing through any field this cycle did
    # not individually chase (a future roster shape), on top of the
    # precise, targeted fixes already wired into `_parse_lst_first_field`,
    # `build_unit_shards`'s own `name` field, and its category labels.
    # Exact-match only; see `pi_redaction.redact_declared_pi_names`'s
    # docstring for why a substring scan is the wrong tool for a blanket,
    # schema-agnostic pass over arbitrary internal-engineering prose.
    data = pi_redaction.redact_declared_pi_names(data, pi_screen.names)
    err = _atomic_write_json(args.out, data)
    if err:
        # Leave the previous payload in place rather than publishing a broken
        # one, and make the failure loud: this line used to print
        # unconditionally, so the log claimed success over a corrupt file.
        print(f"pf1e-producer: NOT published: {err}", file=sys.stderr)
        return 1
    print(f"pf1e-producer: rendered {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
