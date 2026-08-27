#!/usr/bin/env python3
"""retro.py -- the emitter and reader for a project's retrospective event log.

WHAT THIS IS
------------
A small append-only event log for things version control does not capture:
a figure that was stated confidently and turned out to be wrong, a problem
verification caught before it shipped, work that was consciously deferred,
work that had to be redone, and routine verification runs themselves.

THE ONE LINE MOST ADOPTERS CHANGE
----------------------------------
`DEFAULT_EVENTS_DIR` below. It defaults to `<repo-root>/docs/retro/events`
(sharded, one file per actor). Point it wherever this project keeps its log,
or override per-invocation with `--events-dir` / `$RETRO_EVENTS_DIR`.

Set `RETRO_ACTOR` in the environment of each agent or role that emits events,
so events are grouped by who did the work rather than by whatever directory
name happened to be current.

WHY THIS EXISTS
----------------
Version control records what landed. It says nothing about what nearly
landed wrong, who caught it, or that the same person was wrong several times
in one session. Imagine a session where four different reviewers correct
four different figures a lead stated confidently -- a part count, a
unit-price total, a supplier count, and a defect-rate baseline that had
already propagated into every status update written that day. Every
correction is right. None of it survives in version control, and the
pattern across the four is worth more to a retrospective than any
individual fix.

The event vocabulary and the field contract live in `retro-schema.json`,
next to this script, not in this file. This script reads that schema and
builds its own command line out of it, so adding a field is a data edit
rather than a code change, and the schema can never drift from what the
tool accepts.

WHY IT IS SHAPED LIKE THIS
--------------------------
Hand-maintained status artifacts drift and then actively mislead. So:

  * Emission is one shell line with only the fields that carry the finding.
    Timestamp, id, actor, branch, head and worktree are captured for you. If
    recording an event costs more than a sentence of prose, it will lose to
    the prose, and the prose scrolls away.
  * A verification script can emit its own events. Nobody decides to record
    a verification run, so the denominator is honest -- including the runs
    where a stage failed and the fix was quick enough that no one would have
    thought it worth writing down.
  * Nothing version control already holds is copied in. Commits and authors
    are joined at query time (`vcs_join` in the summary), so this log can
    never become a second, staler copy of history.
  * The log is sharded one file per actor. Many agents can work concurrently
    in sibling checkouts; a single append-only file conflicts on nearly
    every merge, and a shared scratch artifact is a single point of
    contention between writers. Distinct filenames remove the shared write
    target instead of coordinating access to it.

USAGE
-----
  retro.py types                       # the vocabulary, one line each
  retro.py help correction             # fields for one type

  retro.py correction --subject lead \\
      --claimed "1,200 units in stock" --actual "1,150 units in stock" \\
      --verified-by "recount against warehouse scan"

  retro.py near-miss --would-have "ship the storefront with broken checkout" \\
      --caught-by "smoke-test:checkout"

  retro.py summary --since 7d          # the retrospective
  retro.py summary --since 7d --json   # same, for a dashboard
  retro.py query --type correction --subject lead
  retro.py validate                    # every line parses and conforms
  retro.py derive-git --since 30d      # revert-shaped commits -> rework

Stdlib only, no build step: an emission costs a process start, not a build.
"""

from __future__ import annotations

import argparse
import collections
import datetime as dt
import json
import os
import pathlib
import re
import secrets
import subprocess
import sys

SCRIPT_DIR = pathlib.Path(__file__).resolve().parent
SCHEMA_PATH = SCRIPT_DIR / "retro-schema.json"

# The one constant most adopters change: where the drop this script writes
# to and reads from lives. Point it at wherever your project keeps its log,
# or override per-invocation with --events-dir / $RETRO_EVENTS_DIR.
DEFAULT_EVENTS_DIR = SCRIPT_DIR.parent.parent / "retro" / "events"

# Fields whose value is not a plain string. Kept beside the schema rather than
# inside it because these are a property of the command line, not of the event
# contract: the JSON on disk is typed either way.
LIST_FIELDS = {"tags", "stages_passed", "stages_failed", "actors_affected"}
BOOL_FIELDS = {"derived", "escaped", "silent"}
NUMBER_FIELDS = {
    "claimed_value",
    "actual_value",
    "time_lost_minutes",
    "duration_seconds",
    "used_percent",
}

SLUG_RE = re.compile(r"[^a-z0-9._-]+")

# Envelope fields the emitter owns outright. `--set` may not reach them.
#
# The whole claim of this log is that provenance is machine-written: `actor`,
# `ts`, `origin` and `derived` say who observed a thing and when, and an event
# that can restate any of those is an event that can be dressed up as
# something it was not. `--set` exists for fields the schema has not named
# yet, and the log is append-only, so a bad line cannot be edited out later --
# which makes refusing it at write time the only place the check can live.
RESERVED_FIELDS = {
    "id", "ts", "type", "actor", "actor_source", "origin", "source", "derived", "repo",
}


# ---------------------------------------------------------------------------
# Schema
# ---------------------------------------------------------------------------


def load_schema(path: pathlib.Path = SCHEMA_PATH) -> dict:
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def type_names(schema: dict) -> list[str]:
    return list(schema["event_types"].keys())


def first_sentence(text: str) -> str:
    """The one-line blurb for `--help`.

    Cut at a sentence boundary rather than a character count: a hard
    truncation of the `why` prose can produce entries reading like "the
    branch shipped un-compilabl", which is how a help screen teaches people
    not to read it.
    """
    for end in (". ", ".\n"):
        index = text.find(end)
        if index != -1:
            return text[: index + 1]
    return text.strip()


def cli_name(event_type: str) -> str:
    """`near_miss` is `near-miss` on the command line; hyphens are what a
    person types and underscores are what JSON consumers want to key on."""
    return event_type.replace("_", "-")


# ---------------------------------------------------------------------------
# Environment capture
#
# Everything here is best-effort by design. An emission must never fail because
# git was busy or the worktree was odd -- a dropped event is a silently missing
# finding, which is strictly worse than an event with a missing `repo` block.
# ---------------------------------------------------------------------------


def git_root() -> pathlib.Path:
    """Which repository git questions are asked of.

    Anchored to the current working directory by default -- unlike a
    project-embedded copy of this script, a standalone tool has no fixed repo
    it lives inside. `RETRO_GIT_ROOT` overrides it, which is how the
    derivation path can be tested against a real repository with real revert
    commits rather than a mock.
    """
    override = os.environ.get("RETRO_GIT_ROOT")
    return pathlib.Path(override) if override else pathlib.Path.cwd()


def _git(args: list[str], cwd: pathlib.Path | None = None) -> str | None:
    try:
        proc = subprocess.run(
            ["git", *args],
            cwd=str(cwd or git_root()),
            capture_output=True,
            text=True,
            timeout=15,
        )
    except (OSError, subprocess.SubprocessError):
        return None
    if proc.returncode != 0:
        return None
    return proc.stdout.strip()


def slugify(value: str) -> str:
    slug = SLUG_RE.sub("-", (value or "").strip().lower()).strip("-.")
    return slug[:60] or "unknown"


OPAQUE_WORKTREE_RE = re.compile(r"^agent-[0-9a-f]{8,}$")


def resolve_actor(explicit: str | None) -> tuple[str, str]:
    """Returns (actor, how_it_was_resolved).

    `--actor`, then `$RETRO_ACTOR`, then the checkout directory name, then
    git `user.name`. The directory fallback exists so an emission never
    needs setup to happen at all -- but it is the weakest of the four, and
    how the name was arrived at is recorded on the event rather than left
    for a reader to guess. `by_actor` counts in a retrospective mean
    something quite different when the names are self-declared than when
    they are scraped off a directory.

    Set `RETRO_ACTOR` per agent or role rather than relying on a harness
    variable, because most harnesses do not expose one that names the
    calling agent or role. A fallback keyed to a variable nothing sets would
    be one more artifact asserting something untrue, which is the failure
    this whole log is a reaction to.
    """
    if explicit:
        return explicit.strip(), "flag"
    from_env = os.environ.get("RETRO_ACTOR")
    if from_env:
        return from_env.strip(), "env"
    top = _git(["rev-parse", "--show-toplevel"])
    name = pathlib.Path(top).name if top else pathlib.Path.cwd().name
    if name and name not in {"/", ""}:
        return name, "worktree"
    return (_git(["config", "user.name"]) or "unknown"), "git-config"


def warn_if_opaque_actor(actor: str, source: str) -> None:
    """A checkout named `agent-<hex>` identifies a directory, not a role.

    It is unique and traceable, so it is still better than refusing to emit --
    but a retrospective that groups by actor gets a column of hashes out of it,
    and by then the checkouts are gone and the mapping is unrecoverable. Said
    once on stderr, at the only moment anyone can still fix it.
    """
    if source == "worktree" and OPAQUE_WORKTREE_RE.match(actor):
        print(
            f"retro: actor `{actor}` came from the checkout directory name and "
            f"identifies a directory rather than a role; set RETRO_ACTOR to the "
            f"agent's name so a retrospective can group by who, not by where",
            file=sys.stderr,
        )


def repo_context() -> dict:
    # Two calls on purpose. `git rev-parse --abbrev-ref HEAD --short HEAD`
    # looks like it returns branch-then-sha and does not: `--abbrev-ref` stays
    # in effect for every revision after it, so both lines come back as the
    # branch name. That silently stamped the branch into `repo.head` on every
    # event until an emitted event was actually read back and looked at.
    context: dict[str, str] = {}
    combined = _git(["rev-parse", "--show-toplevel", "--abbrev-ref", "HEAD"])
    if combined:
        lines = combined.splitlines()
        if len(lines) >= 1:
            context["worktree"] = pathlib.Path(lines[0]).name
        if len(lines) >= 2:
            context["branch"] = lines[1]
    head = _git(["rev-parse", "--short", "HEAD"])
    if head:
        context["head"] = head
    return context


def now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace(
        "+00:00", "Z"
    )


# ---------------------------------------------------------------------------
# Storage
# ---------------------------------------------------------------------------


def events_dir(override: str | None = None) -> pathlib.Path:
    if override:
        return pathlib.Path(override)
    env = os.environ.get("RETRO_EVENTS_DIR")
    if env:
        return pathlib.Path(env)
    return DEFAULT_EVENTS_DIR


def shard_path(directory: pathlib.Path, actor: str) -> pathlib.Path:
    return directory / f"{slugify(actor)}.jsonl"


def append_event(path: pathlib.Path, event: dict) -> None:
    """One O_APPEND write() of one complete line.

    Two processes appending to the same shard therefore interleave whole lines
    and never partial ones. `os.write` on an O_APPEND descriptor is the only
    form that guarantees this; a buffered file object can split a long line
    across writes and produce an unparseable record under concurrency.
    """
    path.parent.mkdir(parents=True, exist_ok=True)
    line = json.dumps(event, ensure_ascii=False, sort_keys=True) + "\n"
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
    try:
        os.write(fd, line.encode("utf-8"))
    finally:
        os.close(fd)


def read_shards(directory: pathlib.Path) -> tuple[list[dict], list[tuple[str, int, str]]]:
    """Returns (events, problems). A problem is (shard, line number, reason).

    Unparseable lines are reported, never skipped silently: a log that quietly
    drops what it cannot read is how a count ends up wrong in the direction
    nobody checks.
    """
    events: list[dict] = []
    problems: list[tuple[str, int, str]] = []
    if not directory.is_dir():
        return events, problems
    for shard in sorted(directory.glob("*.jsonl")):
        with open(shard, encoding="utf-8") as handle:
            for lineno, raw in enumerate(handle, start=1):
                raw = raw.strip()
                if not raw:
                    continue
                try:
                    obj = json.loads(raw)
                except json.JSONDecodeError as exc:
                    problems.append((shard.name, lineno, f"unparseable: {exc}"))
                    continue
                if not isinstance(obj, dict):
                    problems.append((shard.name, lineno, "not a JSON object"))
                    continue
                obj["_shard"] = shard.name
                obj["_line"] = lineno
                events.append(obj)
    events.sort(key=lambda e: (e.get("ts", ""), e.get("id", "")))
    return events, problems


def dedupe_keys_in_shard(path: pathlib.Path) -> set[str]:
    keys: set[str] = set()
    if not path.exists():
        return keys
    with open(path, encoding="utf-8") as handle:
        for raw in handle:
            raw = raw.strip()
            if not raw:
                continue
            try:
                obj = json.loads(raw)
            except json.JSONDecodeError:
                continue
            key = obj.get("dedupe_key") if isinstance(obj, dict) else None
            if key:
                keys.add(key)
    return keys


# ---------------------------------------------------------------------------
# Validation
# ---------------------------------------------------------------------------


def validate_event(event: dict, schema: dict) -> list[str]:
    errors: list[str] = []
    required_envelope = schema["envelope"]["required"]
    for field in required_envelope:
        if event.get(field) in (None, "", []):
            errors.append(f"missing required envelope field `{field}`")

    event_type = event.get("type")
    spec = schema["event_types"].get(event_type)
    if spec is None:
        errors.append(f"unknown event type `{event_type}`")
        return errors

    for field in spec.get("required", []):
        if event.get(field) in (None, "", []):
            errors.append(f"`{event_type}` requires `{field}`")

    origin = event.get("origin")
    if origin not in {"agent", "human", "derived"}:
        errors.append(f"origin must be agent|human|derived, got `{origin}`")
    if event.get("derived") and origin != "derived":
        errors.append("derived events must carry origin=derived")
    if not isinstance(event.get("derived"), bool):
        errors.append("`derived` must be a boolean")
    return errors


# ---------------------------------------------------------------------------
# Emission
# ---------------------------------------------------------------------------


def auto_summary(event_type: str, values: dict) -> str:
    """A one-line summary synthesised from the fields that carry the finding.

    This exists purely to make emission cheaper: --summary stays optional, so
    the required flags are the whole call. A synthesised line is never as good
    as a written one, but it is far better than the event not being recorded.
    """
    get = lambda k: str(values.get(k, "")).strip()  # noqa: E731
    if event_type == "correction":
        return f"{get('subject')} claimed {get('claimed')}; actual {get('actual')}"
    if event_type == "incident":
        return f"{get('impact')} (detected by {get('detected_by')})"
    if event_type == "near_miss":
        return f"caught before {get('would_have')} by {get('caught_by')}"
    if event_type == "deferral":
        return f"deferred {get('what')}: {get('reason')}"
    if event_type == "resolution":
        return f"resolved {get('resolves')}: {get('how')}"
    if event_type == "rework":
        return f"redid {get('what')}: {get('cause')}"
    if event_type == "verification":
        failed = values.get("stages_failed") or []
        tail = f" ({', '.join(failed)} failed)" if failed else ""
        return f"verify {get('mode')}: {get('result')}{tail}"
    # `note` has no fields to synthesise from, so there is nothing to return
    # but the empty string -- which validation then rejects for a missing
    # `summary`. A `note` whose entire content is the word "note" would pass
    # every check and carry no finding, and a log full of those is worse than
    # a log with a gap, because the count looks healthy.
    return ""


def build_event(event_type: str, values: dict, args: argparse.Namespace) -> dict:
    actor, actor_source = resolve_actor(getattr(args, "actor", None))
    warn_if_opaque_actor(actor, actor_source)
    derived = bool(getattr(args, "derived", False))
    origin = getattr(args, "origin", None) or ("derived" if derived else "agent")
    if derived:
        origin = "derived"

    event: dict = {
        "id": f"{int(dt.datetime.now(dt.timezone.utc).timestamp() * 1000)}"
        f"-{slugify(actor)}-{secrets.token_hex(3)}",
        "ts": now_iso(),
        "type": event_type,
        "actor": actor,
        "actor_source": actor_source,
        "origin": origin,
        "source": getattr(args, "source", None) or "cli",
        "derived": derived,
        "summary": (getattr(args, "summary", None) or auto_summary(event_type, values)).strip(),
    }
    context = repo_context()
    if context:
        event["repo"] = context
    for optional in ("task", "corrects", "dedupe_key"):
        value = getattr(args, optional, None)
        if value:
            event[optional] = value
    tags = getattr(args, "tag", None)
    if tags:
        event["tags"] = tags
    # Type-specific fields are layered on top of the envelope, never through
    # it: see RESERVED_FIELDS. `--set` is an escape hatch for fields the schema
    # does not name yet, not a way to rewrite who emitted an event or when.
    event.update({k: v for k, v in values.items() if v not in (None, "", [])})
    return event


def coerce(field: str, raw):
    if raw is None:
        return None
    if field in LIST_FIELDS:
        if isinstance(raw, list):
            items = raw
        else:
            items = [raw]
        out: list[str] = []
        for item in items:
            out.extend(part.strip() for part in str(item).split(",") if part.strip())
        return out
    if field in BOOL_FIELDS:
        if isinstance(raw, bool):
            return raw
        return str(raw).strip().lower() in {"1", "true", "yes", "y"}
    if field in NUMBER_FIELDS:
        try:
            text = str(raw).strip()
            return int(text) if re.fullmatch(r"-?\d+", text) else float(text)
        except ValueError:
            return raw
    return raw


def cmd_emit(args: argparse.Namespace, schema: dict) -> int:
    event_type = args._event_type
    spec = schema["event_types"][event_type]
    fields = list(spec.get("required", [])) + list(spec.get("optional", {}).keys())

    values: dict = {}
    for field in fields:
        raw = getattr(args, field, None)
        value = coerce(field, raw)
        if value not in (None, "", []):
            values[field] = value
    for pair in getattr(args, "set", None) or []:
        if "=" not in pair:
            print(f"retro: --set needs key=value, got `{pair}`", file=sys.stderr)
            return 2
        key, _, raw = pair.partition("=")
        key = key.strip()
        if key in RESERVED_FIELDS:
            print(
                f"retro: `{key}` is set by the emitter and cannot be overridden "
                f"with --set; it is what makes the event's provenance trustworthy",
                file=sys.stderr,
            )
            return 2
        values[key] = coerce(key, raw)

    event = build_event(event_type, values, args)
    errors = validate_event(event, schema)
    if errors:
        print(f"retro: refusing to emit an invalid `{event_type}` event:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        print(f"\nrun: {sys.argv[0]} help {cli_name(event_type)}", file=sys.stderr)
        return 2

    if args.dry_run:
        print(json.dumps(event, indent=2, sort_keys=True))
        return 0

    directory = events_dir(args.events_dir)
    path = shard_path(directory, event["actor"])
    if event.get("dedupe_key") and event["dedupe_key"] in dedupe_keys_in_shard(path):
        if not args.quiet:
            print(f"retro: dedupe_key already present, no-op ({event['dedupe_key']})")
        return 0
    append_event(path, event)
    if not args.quiet:
        print(f"retro: {event_type} {event['id']} -> {path}")
    return 0


# ---------------------------------------------------------------------------
# Windows and reading
# ---------------------------------------------------------------------------


RELATIVE_RE = re.compile(r"^(\d+)\s*([smhdw])$")


def parse_when(text: str | None) -> dt.datetime | None:
    """Accepts `7d`, `36h`, `2026-07-01`, or a full ISO-8601 timestamp."""
    if not text:
        return None
    text = text.strip()
    match = RELATIVE_RE.match(text.lower())
    if match:
        amount, unit = int(match.group(1)), match.group(2)
        seconds = {"s": 1, "m": 60, "h": 3600, "d": 86400, "w": 604800}[unit]
        return dt.datetime.now(dt.timezone.utc) - dt.timedelta(seconds=amount * seconds)
    normalized = text.replace("Z", "+00:00")
    try:
        parsed = dt.datetime.fromisoformat(normalized)
    except ValueError:
        raise SystemExit(f"retro: cannot parse time `{text}` (try 7d, or 2026-07-01)")
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed


def event_time(event: dict) -> dt.datetime | None:
    ts = event.get("ts")
    if not ts:
        return None
    try:
        parsed = dt.datetime.fromisoformat(str(ts).replace("Z", "+00:00"))
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed


def select(events: list[dict], args: argparse.Namespace) -> list[dict]:
    since = parse_when(getattr(args, "since", None))
    until = parse_when(getattr(args, "until", None))
    wanted_types = {t.replace("-", "_") for t in (getattr(args, "type", None) or [])}
    actors = {a.lower() for a in (getattr(args, "actor", None) or [])}
    subjects = {s.lower() for s in (getattr(args, "subject", None) or [])}
    tags = {t.lower() for t in (getattr(args, "tag", None) or [])}
    pattern = getattr(args, "grep", None)
    regex = re.compile(pattern, re.IGNORECASE) if pattern else None

    out = []
    for event in events:
        when = event_time(event)
        if since and (when is None or when < since):
            continue
        if until and (when is None or when > until):
            continue
        if wanted_types and event.get("type") not in wanted_types:
            continue
        if actors and str(event.get("actor", "")).lower() not in actors:
            continue
        if subjects and str(event.get("subject", "")).lower() not in subjects:
            continue
        if tags and not tags & {str(t).lower() for t in (event.get("tags") or [])}:
            continue
        if regex and not regex.search(json.dumps(event, ensure_ascii=False)):
            continue
        out.append(event)
    return out


# ---------------------------------------------------------------------------
# Summary -- the actual deliverable
# ---------------------------------------------------------------------------


def git_join(since: dt.datetime | None, until: dt.datetime | None) -> dict:
    """The denominator. Commits and authors come from git rather than from the
    log, so `corrections per commit` is a ratio between an independently
    recorded numerator and an independently recorded denominator, and neither
    can be inflated by how diligently anyone used this tool."""
    args = ["log", "--no-merges", "--pretty=%H%x1f%an"]
    if since:
        args.append(f"--since={since.isoformat()}")
    if until:
        args.append(f"--until={until.isoformat()}")
    out = _git(args)
    if out is None:
        return {"available": False, "note": "git log unavailable"}
    authors: collections.Counter = collections.Counter()
    commits = 0
    for line in out.splitlines():
        if not line.strip():
            continue
        parts = line.split("\x1f")
        commits += 1
        if len(parts) > 1:
            authors[parts[1]] += 1
    return {
        "available": True,
        "commits": commits,
        "authors": len(authors),
        "top_authors": authors.most_common(5),
    }


def counter_dict(counter: collections.Counter) -> dict:
    return dict(sorted(counter.items(), key=lambda kv: (-kv[1], kv[0])))


def build_summary(events: list[dict], problems, args: argparse.Namespace, directory) -> dict:
    since = parse_when(getattr(args, "since", None))
    until = parse_when(getattr(args, "until", None))

    by_type: collections.Counter = collections.Counter()
    by_actor: collections.Counter = collections.Counter()
    by_origin: collections.Counter = collections.Counter()
    for event in events:
        by_type[event.get("type", "?")] += 1
        by_actor[event.get("actor", "?")] += 1
        by_origin[event.get("origin", "?")] += 1

    def of_type(name: str) -> list[dict]:
        return [e for e in events if e.get("type") == name]

    corrections = of_type("correction")
    subjects: collections.Counter = collections.Counter()
    correctors: collections.Counter = collections.Counter()
    methods: collections.Counter = collections.Counter()
    caught_before: collections.Counter = collections.Counter()
    for event in corrections:
        subjects[event.get("subject", "?")] += 1
        correctors[event.get("actor", "?")] += 1
        methods[event.get("verified_by", "?")] += 1
        if event.get("caught_before"):
            caught_before[event["caught_before"]] += 1

    incidents = of_type("incident")
    recurrence: collections.Counter = collections.Counter()
    for event in incidents:
        recurrence[event.get("recurrence_key") or event.get("summary", "?")] += 1
    silent_incidents = sum(1 for e in incidents if e.get("silent"))

    near_misses = of_type("near_miss")
    caught_by: collections.Counter = collections.Counter()
    for event in near_misses:
        caught_by[event.get("caught_by", "?")] += 1
    escaped = sum(1 for e in near_misses if e.get("escaped"))

    deferrals = of_type("deferral")
    # A deferral is open until some later event -- almost always a
    # `resolution` -- names its id in its own `resolves` field. This is
    # checked generically over every event's `resolves`, not only
    # `resolution`-typed ones, on purpose: the field is what defines
    # "resolved", not the type name carrying it, so the vocabulary can grow a
    # second way to close something out later without this computation
    # silently missing it.
    resolved_ids = {e.get("resolves") for e in events if e.get("resolves")}
    open_deferrals = [e for e in deferrals if e.get("id") not in resolved_ids]
    resolved_deferrals = [e for e in deferrals if e.get("id") in resolved_ids]
    rework = of_type("rework")
    rework_causes: collections.Counter = collections.Counter()
    for event in rework:
        rework_causes[event.get("cause", "?")] += 1

    verifications = of_type("verification")
    failing_stages: collections.Counter = collections.Counter()
    failed_runs = 0
    for event in verifications:
        stages = event.get("stages_failed") or []
        if stages or str(event.get("result", "")).upper().startswith("FAIL"):
            failed_runs += 1
        for stage in stages:
            failing_stages[stage] += 1

    joined = git_join(since, until)
    per_commit = None
    if joined.get("available") and joined.get("commits"):
        per_commit = round(len(events) / joined["commits"], 4)

    def brief(event: dict) -> dict:
        return {
            "ts": event.get("ts"),
            "actor": event.get("actor"),
            "summary": event.get("summary"),
            "id": event.get("id"),
        }

    limit = getattr(args, "limit", 10) or 10
    return {
        "generated_at": now_iso(),
        "generated_by": "retro.py summary",
        "schema_version": 1,
        "window": {
            "since": since.isoformat() if since else None,
            "until": until.isoformat() if until else None,
            "spec": getattr(args, "since", None),
        },
        "events": {
            "total": len(events),
            "by_type": counter_dict(by_type),
            "by_actor": counter_dict(by_actor),
            "by_origin": counter_dict(by_origin),
        },
        "corrections": {
            "total": len(corrections),
            "by_subject": counter_dict(subjects),
            "by_corrector": counter_dict(correctors),
            "repeat_subjects": [
                {"subject": s, "count": c} for s, c in subjects.most_common() if c > 1
            ],
            "by_verification_method": counter_dict(methods),
            "caught_before": counter_dict(caught_before),
            "with_blast_radius": sum(1 for e in corrections if e.get("blast_radius")),
            "recent": [brief(e) for e in corrections[-limit:]],
        },
        "incidents": {
            "total": len(incidents),
            "silent": silent_incidents,
            "by_recurrence_key": counter_dict(recurrence),
            "recurring": [
                {"recurrence_key": k, "count": c} for k, c in recurrence.most_common() if c > 1
            ],
            "time_lost_minutes": sum(
                e.get("time_lost_minutes", 0) or 0
                for e in incidents
                if isinstance(e.get("time_lost_minutes"), (int, float))
            ),
            "recent": [brief(e) for e in incidents[-limit:]],
        },
        "near_misses": {
            "total": len(near_misses),
            "escaped": escaped,
            "by_caught_by": counter_dict(caught_by),
            "recent": [brief(e) for e in near_misses[-limit:]],
        },
        "deferrals": {
            "total": len(deferrals),
            # `open` is a count of unresolved deferrals -- it must not vary
            # with --limit. A naive `deferrals[-limit:]` never measures
            # openness at all: `--limit 3` would report 3 "open", `--limit 29`
            # would report 29 of the same total. See `resolution` in the
            # schema.
            "open": len(open_deferrals),
            "resolved": len(resolved_deferrals),
            "open_items": [
                {
                    "id": e.get("id"),
                    "ts": e.get("ts"),
                    "actor": e.get("actor"),
                    "what": e.get("what"),
                    "reason": e.get("reason"),
                    "revisit": e.get("revisit"),
                }
                for e in open_deferrals
            ],
            "recent": [brief(e) for e in deferrals[-limit:]],
        },
        "rework": {
            "total": len(rework),
            "by_cause": counter_dict(rework_causes),
            "recent": [brief(e) for e in rework[-limit:]],
        },
        "verification": {
            "runs": len(verifications),
            "failed_runs": failed_runs,
            "fail_rate": round(failed_runs / len(verifications), 4) if verifications else None,
            "by_failing_stage": counter_dict(failing_stages),
        },
        "git_join": {**joined, "events_per_commit": per_commit},
        "log": {
            "path": str(directory),
            "shards": len({e.get("_shard") for e in events if e.get("_shard")}),
            "invalid_lines": len(problems),
            "problems": [
                {"shard": s, "line": n, "reason": r} for s, n, r in problems[:20]
            ],
        },
    }


def render_summary(doc: dict) -> str:
    lines: list[str] = []
    add = lines.append
    window = doc["window"]
    add("retrospective -- event log summary")
    add(f"window: {window['spec'] or 'all time'}"
        f"{'  (since ' + window['since'] + ')' if window['since'] else ''}")
    add("")

    events = doc["events"]
    add(f"EVENTS  {events['total']} total")
    for name, count in events["by_type"].items():
        add(f"  {count:>5}  {name}")
    if events["by_origin"]:
        add("  origin: " + ", ".join(f"{k}={v}" for k, v in events["by_origin"].items()))
    add("")

    corrections = doc["corrections"]
    add(f"CORRECTIONS  {corrections['total']}")
    if corrections["by_subject"]:
        add("  who was wrong:")
        for subject, count in corrections["by_subject"].items():
            flag = "   <-- repeated" if count > 1 else ""
            add(f"    {count:>4}  {subject}{flag}")
    if corrections["by_corrector"]:
        add("  who caught it:")
        for actor, count in corrections["by_corrector"].items():
            add(f"    {count:>4}  {actor}")
    if corrections["with_blast_radius"]:
        add(f"  {corrections['with_blast_radius']} had already propagated when caught")
    for item in corrections["recent"]:
        add(f"    - {item['ts']}  {item['summary']}")
    add("")

    incidents = doc["incidents"]
    add(f"INCIDENTS  {incidents['total']}  ({incidents['silent']} produced plausible wrong results)")
    for item in incidents["recurring"]:
        add(f"  RECURRING x{item['count']}  {item['recurrence_key']}")
    for item in incidents["recent"]:
        add(f"    - {item['ts']}  {item['summary']}")
    add("")

    near = doc["near_misses"]
    add(f"NEAR MISSES  {near['total']}  ({near['escaped']} escaped and were fixed after the fact)")
    for gate, count in near["by_caught_by"].items():
        add(f"    {count:>4}  caught by {gate}")
    add("")

    deferrals = doc["deferrals"]
    add(f"DEFERRALS  {deferrals['total']} total, {deferrals['open']} open, "
        f"{deferrals['resolved']} resolved")
    for item in deferrals["open_items"]:
        revisit = f"  [revisit: {item['revisit']}]" if item.get("revisit") else ""
        add(f"    - {item['what']} -- {item['reason']}{revisit}")
    add("")

    rework = doc["rework"]
    add(f"REWORK  {rework['total']}")
    for cause, count in rework["by_cause"].items():
        add(f"    {count:>4}  {cause}")
    add("")

    verification = doc["verification"]
    add(f"VERIFICATION  {verification['runs']} runs, {verification['failed_runs']} with a failing stage")
    for stage, count in verification["by_failing_stage"].items():
        add(f"    {count:>4}  {stage}")
    add("")

    joined = doc["git_join"]
    if joined.get("available"):
        add(f"GIT (same window)  {joined['commits']} commits by {joined['authors']} authors")
        if joined.get("events_per_commit") is not None:
            add(f"    {joined['events_per_commit']} events per commit")
    add("")

    log = doc["log"]
    add(f"LOG  {log['shards']} shards at {log['path']}")
    if log["invalid_lines"]:
        add(f"  {log['invalid_lines']} INVALID LINES -- run `retro.py validate`")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------


def cmd_query(args: argparse.Namespace, schema: dict) -> int:
    directory = events_dir(args.events_dir)
    events, _ = read_shards(directory)
    selected = select(events, args)
    if args.limit:
        selected = selected[-args.limit :]
    if args.json:
        for event in selected:
            event.pop("_shard", None)
            event.pop("_line", None)
        print(json.dumps(selected, indent=2, sort_keys=True))
        return 0
    for event in selected:
        # Every field is rendered through str() before padding. `query` is the
        # first thing anyone reaches for when the log looks wrong, so it has to
        # survive a malformed line rather than raise on it -- the tool that
        # shows you the corruption cannot be the one that dies of it.
        print(
            f"{event.get('ts', '?')}  {str(event.get('type', '?')):<12} "
            f"{str(event.get('actor', '?')):<22} {event.get('summary', '')}"
        )
    return 0


def cmd_summary(args: argparse.Namespace, schema: dict) -> int:
    directory = events_dir(args.events_dir)
    events, problems = read_shards(directory)
    selected = select(events, args)
    doc = build_summary(selected, problems, args, directory)
    if args.json:
        print(json.dumps(doc, indent=2, sort_keys=True))
    else:
        print(render_summary(doc))
    return 0


def cmd_validate(args: argparse.Namespace, schema: dict) -> int:
    directory = events_dir(args.events_dir)
    events, problems = read_shards(directory)
    failures = 0
    for shard, lineno, reason in problems:
        print(f"{shard}:{lineno}: {reason}")
        failures += 1
    for event in events:
        for error in validate_event(event, schema):
            print(f"{event.get('_shard')}:{event.get('_line')}: {error}")
            failures += 1
    total = len(events)
    if failures:
        print(f"\nretro: {failures} problem(s) across {total} event(s)")
        return 1
    print(f"retro: {total} event(s) in {directory}, all valid")
    return 0


def cmd_types(args: argparse.Namespace, schema: dict) -> int:
    for name, spec in schema["event_types"].items():
        required = ", ".join(spec.get("required", [])) or "(none)"
        print(f"{cli_name(name):<14} required: {required}")
    return 0


def cmd_help_type(args: argparse.Namespace, schema: dict) -> int:
    name = args.event_type.replace("-", "_")
    spec = schema["event_types"].get(name)
    if spec is None:
        print(f"retro: unknown type `{args.event_type}`; known: "
              f"{', '.join(cli_name(t) for t in type_names(schema))}", file=sys.stderr)
        return 2
    print(f"{cli_name(name)}\n")
    print(spec.get("why", ""))
    print("\nrequired:")
    notes = spec.get("field_notes", {})
    for field in spec.get("required", []):
        note = notes.get(field)
        print(f"  --{field.replace('_', '-')}" + (f"\n      {note}" if note else ""))
    optional = spec.get("optional", {})
    if optional:
        print("\noptional:")
        for field, description in optional.items():
            print(f"  --{field.replace('_', '-')}\n      {description}")
    print("\nalways available: --actor --summary --task --tag --corrects "
          "--dedupe-key --set key=value --dry-run")
    return 0


REVERT_RE = re.compile(r'^(Revert|Reapply)\s+"', re.IGNORECASE)


def cmd_derive_git(args: argparse.Namespace, schema: dict) -> int:
    """Derive `rework` events from revert-shaped commits.

    Deliberately narrow. A revert is the one form of rework git records
    unambiguously; everything else it might suggest (a `fix(` commit touching
    a file a recent commit touched, a force-push) is a guess, and a guessed
    event in this log is worth less than no event. It is common for a branch
    to show zero revert-shaped commits despite rework having demonstrably
    happened -- which is the point, not a bug: rebase, amend and squash erase
    rework entirely, so almost all of it has to be emitted at the moment it
    happens or it is gone.
    """
    since = parse_when(args.since)
    log_args = ["log", "--pretty=%H%x1f%an%x1f%aI%x1f%s"]
    if since:
        log_args.append(f"--since={since.isoformat()}")
    out = _git(log_args)
    if out is None:
        print("retro: git log unavailable", file=sys.stderr)
        return 1

    directory = events_dir(args.events_dir)
    emitted = 0
    skipped = 0
    for line in out.splitlines():
        parts = line.split("\x1f")
        if len(parts) < 4:
            continue
        sha, author, when, subject = parts[0], parts[1], parts[2], parts[3]
        if not REVERT_RE.match(subject):
            continue
        key = f"git-revert:{sha}"
        path = shard_path(directory, author)
        if key in dedupe_keys_in_shard(path):
            skipped += 1
            continue
        event = {
            "id": f"{int(dt.datetime.now(dt.timezone.utc).timestamp() * 1000)}"
            f"-{slugify(author)}-{secrets.token_hex(3)}",
            "ts": when,
            "type": "rework",
            "actor": author,
            "origin": "derived",
            "source": "git",
            "derived": True,
            "summary": subject[:160],
            "what": subject[:160],
            "cause": "revert-shaped commit in git history",
            "first_attempt": sha,
            "dedupe_key": key,
        }
        errors = validate_event(event, schema)
        if errors:
            print(f"retro: derived event for {sha} is invalid: {errors}", file=sys.stderr)
            return 1
        if args.dry_run:
            print(json.dumps(event, sort_keys=True))
        else:
            append_event(path, event)
        emitted += 1
    print(f"retro: derived {emitted} rework event(s) from git, {skipped} already present")
    return 0


# ---------------------------------------------------------------------------
# Command line, built from the schema
# ---------------------------------------------------------------------------


def add_common_emit_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--actor", help="who is emitting (default: $RETRO_ACTOR, else the worktree name)")
    parser.add_argument("--summary", help="one line; synthesised from the required fields if omitted")
    parser.add_argument("--task", help="task or brief id")
    parser.add_argument("--tag", action="append", help="repeatable")
    parser.add_argument("--corrects", help="id of an earlier event this supersedes")
    parser.add_argument("--dedupe-key", dest="dedupe_key", help="emitting the same key twice is a no-op")
    parser.add_argument("--source", help="mechanism writing this event (default: cli)")
    parser.add_argument("--origin", choices=["agent", "human", "derived"])
    parser.add_argument("--derived", action="store_true", help="computed from an existing signal")
    parser.add_argument("--set", action="append", metavar="KEY=VALUE", help="any additional field")
    parser.add_argument("--dry-run", action="store_true", help="print the event, write nothing")
    parser.add_argument("--quiet", action="store_true")
    parser.add_argument("--events-dir", dest="events_dir", help="override the log location")


def add_read_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--since", help="7d, 36h, 2026-07-01, or an ISO timestamp")
    parser.add_argument("--until", help="same forms as --since")
    parser.add_argument("--type", action="append", help="repeatable")
    parser.add_argument("--actor", action="append", help="repeatable")
    parser.add_argument("--subject", action="append", help="corrections: who was wrong")
    parser.add_argument("--tag", action="append", help="repeatable")
    parser.add_argument("--grep", help="regex over the whole event")
    parser.add_argument("--limit", type=int, default=0)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--events-dir", dest="events_dir", help="override the log location")


def build_parser(schema: dict) -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="retro.py",
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    sub = parser.add_subparsers(dest="command", required=True)

    for name, spec in schema["event_types"].items():
        emitter = sub.add_parser(
            cli_name(name),
            help=first_sentence(spec.get("why", "")),
            description=spec.get("why", ""),
            formatter_class=argparse.RawDescriptionHelpFormatter,
        )
        for field in spec.get("required", []):
            emitter.add_argument(
                f"--{field.replace('_', '-')}",
                dest=field,
                required=True,
                help=spec.get("field_notes", {}).get(field, "required"),
            )
        for field, description in (spec.get("optional") or {}).items():
            if field in BOOL_FIELDS:
                # A flag, not a flag-with-a-value: `--escaped` reads correctly
                # and `--escaped true` would not. `default=None` matters --
                # store_true's default of False would stamp an explicit
                # `"silent": false` onto every incident, which reads as an
                # assertion that it was checked rather than as absence.
                emitter.add_argument(
                    f"--{field.replace('_', '-')}",
                    dest=field,
                    action="store_true",
                    default=None,
                    help=description,
                )
                continue
            emitter.add_argument(
                f"--{field.replace('_', '-')}",
                dest=field,
                action="append" if field in LIST_FIELDS else "store",
                help=description,
            )
        add_common_emit_flags(emitter)
        emitter.set_defaults(_handler=cmd_emit, _event_type=name)

    query = sub.add_parser("query", help="list matching events")
    add_read_flags(query)
    query.set_defaults(_handler=cmd_query)

    summary = sub.add_parser("summary", help="the retrospective: counts by type, actor and window")
    add_read_flags(summary)
    summary.set_defaults(_handler=cmd_summary, limit=10)

    validate = sub.add_parser("validate", help="every line parses and conforms to the schema")
    validate.add_argument("--events-dir", dest="events_dir")
    validate.set_defaults(_handler=cmd_validate)

    types = sub.add_parser("types", help="the event vocabulary")
    types.set_defaults(_handler=cmd_types)

    help_type = sub.add_parser("help", help="fields for one event type")
    help_type.add_argument("event_type")
    help_type.set_defaults(_handler=cmd_help_type)

    derive = sub.add_parser("derive-git", help="revert-shaped commits -> rework events")
    derive.add_argument("--since", help="default: all history")
    derive.add_argument("--dry-run", action="store_true")
    derive.add_argument("--events-dir", dest="events_dir")
    derive.set_defaults(_handler=cmd_derive_git)

    return parser


def main(argv: list[str] | None = None) -> int:
    schema = load_schema()
    parser = build_parser(schema)
    args = parser.parse_args(argv)
    return args._handler(args, schema)


if __name__ == "__main__":
    sys.exit(main())
