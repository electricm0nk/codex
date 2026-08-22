#!/usr/bin/env python3
"""scripts/observer/pi_redaction.py -- Decision 12 (2026-08-17, operator
ruling), `SD-31-corpus-closure-grind/decisions.md`: "withhold the name, keep
the row."

THE RULE. A public artifact (`site/dashboard/**`) may publish that a record
exists, and every derived figure about it -- counts, percentages, kind, book,
status -- but never a name its own PCGen source row declares Product Identity
(`NAMEISPI:YES`). The row survives; the name is replaced with
`REDACTED_PI_MARKER`.

THE AUTHORITY IS THE ORACLE'S OWN DECLARATION, NOT A BLACKLIST. Decision 12's
binding requirement #2 is explicit: an exact-substring blacklist is not
sufficient evidence of safety -- this program has already shipped deity-name
typo variants straight through one (wave 10) and 28 more `raw_tokens` hits
past another (wave 12). This module never maintains its own term list; every
answer it gives is read directly off the pinned PCGen oracle's own
`NAMEISPI:`/`DESCISPI:` tokens (`src/rules_core/pi_screening.rs`'s
`declared_product_identity`, ported here because there is no Python binding
for the Rust crate). Where the module cannot find the source row at all it
reports "no declaration found" (False), exactly like every existing reader --
it is never treated as proof the name is safe; callers that need that
distinction check `.available` separately.

Two independent lookup shapes, because the two callers hold different
coordinates:

* `OracleNameChecker.declared(book, source_file, source_line)` -- exact
  (book, source_file, source_line) coordinates, as carried by
  `docs/work-inventory.json` units. Used by `build_unit_shards`.
* `build_declared_pi_name_index(corpus_root)` -- a full pinned-oracle sweep
  returning the set of every UNAMBIGUOUS name the oracle itself declares
  `NAMEISPI:YES`, for scanning free-form roster text
  (`_parse_lst_first_field`) and for `scripts/site_dashboard_pi_gate.py`'s
  exact-match scan (see `find_declared_pi_leaks`) over whatever the
  producer actually shipped -- the same technique `OPEN-ISSUES.md` row 149
  already applied by hand.

Both are read-only over the pinned PCGen checkout; neither ever writes to it.

The book-directory resolution below (`build_book_index`,
`find_file_recursive`) is deliberately reimplemented rather than imported
from `src/rules_core/wiring_class.rs` or shared with
`scripts/ground_truth_evidence_guard.py` -- same rationale that guard's own
module doc gives: there is no Python binding for the Rust resolver, and a
module that shared a resolver's bug would be blind to exactly the failure
class this file exists to catch. The shape (walk the whole tree once, index
every directory by basename) is intentionally the same pattern
`ground_truth_evidence_guard.py` already proved correct for this repo.
"""
from __future__ import annotations

import os

# Matches `src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER` byte-for-byte --
# the same marker every existing redaction path in this repo already writes
# into `data/corpus/**/*.json`'s `description` field, so a reader that has
# learned to recognise one recognises both.
REDACTED_PI_MARKER = "[redacted PI]"

# Same skip-prefix / cleanup rules `pf1e_dashboard_producer.py`'s own
# `_parse_lst_first_field` already applies to a row's first field, kept in
# sync deliberately (see that function's own call into `clean_first_field`
# below) so a name looked up here and a name emitted there are the same
# string.
_NAME_PREFIX_STRIPS = ("CLASS:", "SUBCLASS:")


def clean_first_field(raw: str) -> str:
    """The row's first tab-delimited field, with the same CLASS:/SUBCLASS:
    prefix stripping `_parse_lst_first_field` already applies, PLUS PCGen
    row-operator syntax normalised (SD31-W13-INTEGRATE-001 finding 1): a
    `.MOD`/`.FORGET` row tags an EXISTING object, and a `.COPY=<new name>`
    row CREATES one -- neither is part of the object's own published name,
    and a `.MOD` declaration must resolve onto the same bare name any other
    reference to that object uses, or a real declared-PI leak (a `.MOD` row
    tagging an object a `.COPY=` row elsewhere creates) is silently missed.
    Kept here so both the roster builder and the PI index agree on what
    "the name" is."""
    name = raw.strip()
    for prefix in _NAME_PREFIX_STRIPS:
        if name.startswith(prefix):
            name = name[len(prefix):]
    name = name.strip()
    # `.COPY=<new name>` creates a NEW object named after the right-hand
    # side; take that, not the left-hand source key.
    upper = name.upper()
    if ".COPY=" in upper:
        idx = upper.index(".COPY=")
        name = name[idx + len(".COPY="):]
        name = name.strip()
    else:
        # `.MOD`/`.FORGET` tag an EXISTING object by the same key; strip the
        # suffix so the tagged object's bare name is what gets indexed.
        upper = name.upper()
        for suffix in (".MOD", ".FORGET"):
            if upper.endswith(suffix):
                name = name[: -len(suffix)]
                break
        name = name.strip()
    # A leading `CATEGORY=...|` qualifier is not part of the name.
    if name.upper().startswith("CATEGORY=") and "|" in name:
        name = name.split("|", 1)[1]
    return name.strip()


_ROW_OPERATOR_SUFFIXES = (".MOD", ".FORGET")


def _is_row_operator_reference(first_field: str) -> bool:
    """True when `first_field` is a PCGen row-OPERATOR reference to an
    existing object (`.MOD`, `.FORGET`, `.COPY=`) rather than a fresh
    object definition. SD31-W13-INTEGRATE-001 finding 1's root cause: such
    a row very often carries no `NAMEISPI:`/`DESCISPI:` token of its own
    (the declaration lives on a DIFFERENT row for the same object), so
    treating its silence as "this name is non-PI" is wrong -- it silently
    cancelled a real `.MOD` declaration via the pi_names - non_pi_names
    subtraction below. An operator row is therefore never treated as
    non-PI evidence; it can still contribute a PI hit if it happens to
    carry the token itself."""
    upper = first_field.upper()
    if ".COPY=" in upper:
        return True
    return any(upper.endswith(suffix) for suffix in _ROW_OPERATOR_SUFFIXES)


def pcgen_corpus_root() -> str:
    """Resolve the pinned PCGen oracle's data root, per AGENTS.md's standing
    rule: `$PCGEN_CORPUS_ROOT`, never a literal local path. Default matches
    `scripts/fetch-pcgen-oracle.sh`'s own default checkout location."""
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    home = os.environ.get("HOME") or os.path.expanduser("~")
    return os.path.join(home, "workspace/repos/pcgen/data")


def parse_row_tokens(line: str) -> list[tuple[str, str]]:
    """A PCGen row's tab-delimited `KEY:VALUE` fields as `(key, value)`
    pairs. Mirrors `declared_pi_shipping_audit.rs::declared_at`'s own
    `row.split('\\t').filter_map(|f| f.split_once(':'))` exactly: a field
    with no `:` (a bare flag) is silently skipped, not an error."""
    tokens: list[tuple[str, str]] = []
    for field in line.split("\t"):
        if ":" not in field:
            continue
        key, value = field.split(":", 1)
        tokens.append((key, value))
    return tokens


def declared_product_identity(tokens: list[tuple[str, str]]) -> tuple[bool, bool]:
    """`(name_is_pi, description_is_pi)`, read off a row's tokens. Ported
    from `src/rules_core/pi_screening.rs::declared_product_identity`
    field-for-field: keys matched case-insensitively, values trimmed and
    matched case-insensitively against `YES` (PCGen also writes
    `NAMEISPI:NO` explicitly -- anything other than `YES` is absence, not a
    hit)."""
    name = False
    description = False
    for key, value in tokens:
        if value.strip().upper() != "YES":
            continue
        key_upper = key.strip().upper()
        if key_upper == "NAMEISPI":
            name = True
        elif key_upper == "DESCISPI":
            description = True
    return name, description


def paizo_root(corpus_root: str) -> str:
    """Narrows any oracle walk to Paizo's own published content
    (`<corpus_root>/pathfinder/paizo`) — real, found false positive this
    cycle: the full `$PCGEN_CORPUS_ROOT` checkout on this box is NOT
    Pathfinder-only (it also carries Spycraft, Starfinder, Deadlands and
    several third-party Pathfinder publishers under `pathfinder/*`, none
    of which are this program's corpus). An unscoped walk found
    `spycraft/crafty_games/spycraft/shadowforce_archer/
    shadowforce_archer_gear_mystic.lst`'s own `NAMEISPI:YES` row named
    "Teleport" — a Spycraft ritual that happens to share a bare word with
    the Core Rulebook's completely unrelated, non-PI "Teleport" spell —
    and flagged the CRB spell as a leak. Every book id this program's
    38-book mandate roster uses resolves under `paizo/{roleplaying_game,
    campaign_setting,player_companion,adventure_path,gamemastery_cards}`,
    so rooting here is not a narrowing of coverage, only of noise. Falls
    back to `<corpus_root>/pathfinder` then `corpus_root` itself so a
    scratch test fixture that only builds a `pathfinder/paizo/...` shape
    (or an even smaller one) still resolves."""
    paizo = os.path.join(corpus_root, "pathfinder", "paizo")
    if os.path.isdir(paizo):
        return paizo
    pathfinder = os.path.join(corpus_root, "pathfinder")
    return pathfinder if os.path.isdir(pathfinder) else corpus_root


def build_book_index(corpus_root: str) -> dict:
    """Book id (directory basename) -> absolute directory path, built by
    walking the Paizo-scoped oracle tree once (see `paizo_root`). Same
    overall pattern as `scripts/ground_truth_evidence_guard.py
    ::build_book_index` (see that module's own comment for why duplicating
    rather than sharing is deliberate here) -- scoped to `paizo_root`
    rather than that guard's wider `pathfinder/` walk, for the same
    cross-game-system collision reason `paizo_root`'s own docstring
    explains."""
    index: dict[str, str] = {}
    if not corpus_root or not os.path.isdir(corpus_root):
        return index
    walk_root = paizo_root(corpus_root)
    for dirpath, dirnames, _files in os.walk(walk_root):
        base = os.path.basename(dirpath)
        dirnames.sort()
        index.setdefault(base, dirpath)
    return index


def find_file_recursive(book_dir: str, filename: str) -> list[str]:
    """Every path under `book_dir` whose basename is exactly `filename`."""
    matches: list[str] = []
    if not book_dir or not os.path.isdir(book_dir):
        return matches
    for dirpath, _dirs, files in os.walk(book_dir):
        if filename in files:
            matches.append(os.path.join(dirpath, filename))
    return matches


def iter_lst_files(root: str):
    """Every `.lst` file under `root`, case-insensitive extension match."""
    if not root or not os.path.isdir(root):
        return
    for dirpath, _dirs, files in os.walk(root):
        for fn in files:
            if fn.lower().endswith(".lst"):
                yield os.path.join(dirpath, fn)


class OracleNameChecker:
    """Cross-references `(book, source_file, source_line)` coordinates --
    the shape `docs/work-inventory.json` units carry -- against the pinned
    oracle's own `NAMEISPI:`/`DESCISPI:` declarations.

    Built once per producer run (`self.available` reports whether the
    pinned checkout could even be found, so a caller can tell "we checked
    and found no declaration" apart from "we could not check at all" --
    both currently degrade to `(False, False)` from `.declared()`, but a
    caller that must not ship on an unchecked unit reads `.available`
    first, same posture as every other honest-gap reader in this file)."""

    def __init__(self, corpus_root: str | None = None):
        self.corpus_root = corpus_root or pcgen_corpus_root()
        self.available = os.path.isdir(self.corpus_root)
        self._book_index = build_book_index(self.corpus_root) if self.available else {}
        # (book, source_file) -> list[str] lines, or None if unresolvable.
        self._file_cache: dict[tuple[str, str], list[str] | None] = {}

    def _lines_for(self, book: str, source_file: str) -> list[str] | None:
        key = (book, source_file)
        if key in self._file_cache:
            return self._file_cache[key]
        lines: list[str] | None = None
        book_dir = self._book_index.get(book)
        if book_dir:
            matches = find_file_recursive(book_dir, source_file)
            if matches:
                try:
                    with open(matches[0], encoding="utf-8", errors="replace") as f:
                        lines = f.readlines()
                except OSError:
                    lines = None
        self._file_cache[key] = lines
        return lines

    def declared(self, book: str | None, source_file: str | None, source_line: int | None) -> tuple[bool, bool]:
        """`(name_is_pi, description_is_pi)` for the row at
        `(book, source_file, source_line)` (1-indexed, matching
        `declared_pi_shipping_audit.rs::declared_at`). Any missing
        coordinate, an unresolvable book/file, or an out-of-range line all
        degrade to `(False, False)` -- "no declaration found," the same
        honest-gap shape every other reader in this repo uses; it is never
        proof the row is safe on its own (see module docstring)."""
        if not book or not source_file or not source_line:
            return False, False
        lines = self._lines_for(book, source_file)
        if not lines or source_line < 1 or source_line > len(lines):
            return False, False
        row = lines[source_line - 1].rstrip("\n").rstrip("\r")
        return declared_product_identity(parse_row_tokens(row))


def build_declared_pi_name_index(corpus_root: str | None = None) -> set[str]:
    """Every UNAMBIGUOUS NAME the pinned oracle declares `NAMEISPI:YES`,
    scoped to Paizo's own content (`paizo_root`). This is the full-sweep
    counterpart to `OracleNameChecker`: where that class answers "is THIS
    exact row PI?", this answers "what is the complete set of declared-PI
    names anywhere in the oracle?" -- needed for scanning already-generated
    free text (a roster string, a committed JSON blob) that carries no
    `(book, source_file, source_line)` coordinate of its own. Same
    full-tree-scan technique `OPEN-ISSUES.md` row 149 already applied by
    hand to find the 56-name leak this module exists to prevent from
    recurring.

    "UNAMBIGUOUS" is load-bearing. A bare word can be BOTH a declared-PI
    record's name in one book AND a completely unrelated, non-PI record's
    name elsewhere (`§13`'s own guard: "a shared name never implies a
    shared thing"). The real example this cycle found: the Core
    Rulebook's ordinary, non-PI "Teleport" spell shares its bare name with
    an unrelated declared-PI "Teleport" ritual elsewhere in the oracle.
    A name that appears ANYWHERE without a `NAMEISPI:YES` declaration is
    therefore dropped from this index -- this only ever makes the returned
    set SMALLER (a strictly more conservative gate, never a name the oracle
    never declared PI at all), so it cannot turn a real leak into a false
    negative for any name that is genuinely PI everywhere it appears."""
    root = paizo_root(corpus_root or pcgen_corpus_root())
    pi_names: set[str] = set()
    non_pi_names: set[str] = set()
    for path in iter_lst_files(root):
        try:
            with open(path, encoding="utf-8", errors="replace") as f:
                text = f.read()
        except OSError:
            continue
        for line in text.split("\n"):
            if not line or line.startswith("#"):
                continue
            name_is_pi, _ = declared_product_identity(parse_row_tokens(line))
            first_field = line.split("\t", 1)[0]
            cleaned = clean_first_field(first_field)
            if not cleaned:
                continue
            if name_is_pi:
                pi_names.add(cleaned)
            elif not _is_row_operator_reference(first_field):
                # See `_is_row_operator_reference`'s docstring: a `.MOD`/
                # `.COPY=` row's silence is not a non-PI assertion.
                non_pi_names.add(cleaned)
    return pi_names - non_pi_names


def build_declared_pi_name_book_index(corpus_root: str | None = None) -> dict[str, set[str]]:
    """`name -> {book ids the oracle declares it NAMEISPI:YES in}`.
    SD31-W13-INTEGRATE-001 finding 2: `build_declared_pi_name_index`'s
    global `pi_names - non_pi_names` subtraction is the right conservative
    default for a caller with NO book context (it must not flag "Teleport"
    or "Shield" -- two unrelated objects sharing a bare name, one PI in an
    unrelated book, one not) -- but it also silently drops a name that is
    genuinely PI in one book and a genuinely different, genuinely non-PI
    object in another, for any caller that DOES have a book to check
    against. Shard rows carry `book`; this index lets that caller ask
    "is this name declared PI in THIS book" instead of "is this name
    unambiguous everywhere," which is what "the record's own declared-PI
    state is the authority" (Decision 12) actually requires. Built by
    walking each book directory (`build_book_index`) individually rather
    than the whole paizo tree at once, so every declaration can be
    attributed to the book it came from."""
    book_index = build_book_index(corpus_root or pcgen_corpus_root())
    index: dict[str, set[str]] = {}
    for book, book_dir in book_index.items():
        for path in iter_lst_files(book_dir):
            try:
                with open(path, encoding="utf-8", errors="replace") as f:
                    text = f.read()
            except OSError:
                continue
            for line in text.split("\n"):
                if not line or line.startswith("#"):
                    continue
                name_is_pi, _ = declared_product_identity(parse_row_tokens(line))
                if not name_is_pi:
                    continue
                first_field = line.split("\t", 1)[0]
                cleaned = clean_first_field(first_field)
                if not cleaned:
                    continue
                index.setdefault(cleaned, set()).add(book)
    return index


def compile_name_patterns(names):
    """Kept for backward compatibility with any external caller expecting a
    patterns argument; `find_declared_pi_leaks`/`redact_declared_pi_names`
    now take the plain name SET directly (see their own docstrings for why
    exact-string matching replaced substring/word-boundary matching)."""
    return {n for n in names if n}


def find_declared_pi_leaks(value, declared_names, path: str = "$") -> list[tuple[str, str]]:
    """Recursively walk a decoded JSON value (dict/list/str/scalar),
    returning `(json_path, declared_name)` for every string LEAF that
    EXACTLY equals (after trimming) a declared-PI name. Structure-agnostic
    on purpose -- the exact shape of `PF1e-dashboard.json` or a shard is an
    implementation detail that already drifted once (row 149's
    `manifests`/`content_state`/`matrix` sections were three different
    structural shapes carrying the same class of leak); walking every
    string leaf is the only scan that does not need to be re-taught the
    schema on the next drift.

    EXACT MATCH, NOT SUBSTRING. An earlier version of this function did a
    word-boundary substring scan and flagged `"Shackles of Compliance"`
    (an ordinary, non-PI magic item) purely because the declared-PI
    background name `"Shackles"` occurs as one word inside it -- the exact
    `§13` guard this program keeps re-learning: "a shared name never
    implies a shared thing" applies to substrings of a name just as much as
    to a whole shared name. Decision 12's rule is about publishing THE
    NAME -- a field whose value equals the declared name -- not about a
    declared name appearing as a token inside unrelated prose."""
    hits: list[tuple[str, str]] = []
    if isinstance(value, dict):
        for k, v in value.items():
            hits.extend(find_declared_pi_leaks(v, declared_names, f"{path}.{k}"))
    elif isinstance(value, list):
        for i, v in enumerate(value):
            hits.extend(find_declared_pi_leaks(v, declared_names, f"{path}[{i}]"))
    elif isinstance(value, str) and value.strip() in declared_names:
        hits.append((path, value.strip()))
    return hits


def find_declared_pi_leaks_in_shard_rows(doc, name_to_books: dict) -> list[tuple[str, str]]:
    """Per-book leak scan for a shard's own `{"fields": [...], "rows": [...]}`
    shape (`pf1e_dashboard_producer.py`'s `UNIT_SHARD_FIELDS`/
    `SPELL_SHARD_FIELDS`). Closes SD31-W13-INTEGRATE-001 finding 2 for the
    one shape that DOES carry a `book` alongside each `name`: a shard row.
    A no-op (returns `[]`) on any document that is not shaped this way --
    `find_declared_pi_leaks`'s global, book-blind exact-match scan remains
    the net for everything else (the top-level feed's `categories[*].label`
    and similar book-free text)."""
    hits: list[tuple[str, str]] = []
    fields = doc.get("fields") if isinstance(doc, dict) else None
    rows = doc.get("rows") if isinstance(doc, dict) else None
    if not isinstance(fields, list) or not isinstance(rows, list):
        return hits
    if "name" not in fields or "book" not in fields:
        return hits
    name_idx = fields.index("name")
    book_idx = fields.index("book")
    for i, row in enumerate(rows):
        if not isinstance(row, list) or len(row) <= max(name_idx, book_idx):
            continue
        name = row[name_idx]
        book = row[book_idx]
        if not isinstance(name, str) or name == REDACTED_PI_MARKER:
            continue
        if book in name_to_books.get(name.strip(), set()):
            hits.append((f"$.rows[{i}][{name_idx}]", f"{name!r} declared PI in book {book!r}"))
    return hits


def build_book_declared_name_lists(name_to_books: dict[str, set[str]]) -> dict[str, list[str]]:
    """Invert `name_to_books` (`name -> {books it is declared PI in}`,
    `build_declared_pi_name_book_index`'s own return shape) into
    `book -> [declared names], longest first` for efficient same-book
    substring screening. SD31-W13-INTEGRATE-001-VERIFY finding 1/2: a
    caller that DOES have a book to check a string against (a published
    item row, a status-data shard) needs "which names are declared PI in
    THIS book" as a fast per-book lookup, not a re-scan of the whole
    corpus per item. Longest-first ordering matches the convention
    `build_public_status.py`'s substring screens already use elsewhere
    (does not change the boolean result, only scan order)."""
    by_book: dict[str, list[str]] = {}
    for name, books in name_to_books.items():
        for book in books:
            by_book.setdefault(book, []).append(name)
    for book in by_book:
        by_book[book].sort(key=len, reverse=True)
    return by_book


def value_carries_declared_pi_substring(value: str, declared_names_by_length) -> bool:
    """True if `value` contains any name in `declared_names_by_length` as a
    case-sensitive substring.

    SHARED on purpose between the producer (`build_public_status.py`'s
    per-field substring screens: `name`, scoped per-book via
    `build_book_declared_name_lists`; `type_facet`, scoped globally) and
    its safety-net gate (`site_public_status_pi_gate.py`) so the two can
    never drift into checking different things -- exactly the failure mode
    SD31-W13-INTEGRATE-001-VERIFY finding 1 found (the producer had a
    substring screen for `type_facet` and not for `name`; nothing forced
    the two to agree). See `build_public_status.py`'s
    `redact_for_display`/`_type_facet_carries_declared_pi` docstrings for
    the substring-vs-exact-match rationale and the false-positive
    (`"Shackles of Compliance"`) this trades off against -- callers that
    need that trade-off must pre-scope `declared_names_by_length` (e.g. to
    one book) themselves; this function does the substring test only."""
    for name in declared_names_by_length:
        if name and name in value:
            return True
    return False


def _is_word_char(ch: str) -> bool:
    """True for characters `find_declared_pi_word_matches` treats as part of
    a token (so a match immediately touching one on either side is a
    same-word fusion, not a freestanding embed)."""
    return ch.isalnum()


def find_declared_pi_word_matches(value: str, declared_names_by_length, case_insensitive: bool = False) -> list[str]:
    """Every name in `declared_names_by_length` that appears in `value` as a
    freestanding WORD -- immediately bounded by a non-alphanumeric character
    (or the start/end of the string) on BOTH sides -- rather than merely
    fused as a same-word stem/prefix inside a longer, differently-spelled
    word. Case-sensitive BY DEFAULT (`case_insensitive=False`), same
    convention as `value_carries_declared_pi_substring` and unchanged for
    every existing caller that does not pass the new argument.

    `case_insensitive=True` (FIX-DASHBOARD-PI, 2026-08-17): a small, real
    class of declared-PI names IS a title beginning with an ordinary
    English word (`"The Serpent King"`, `"The Green Mother"` -- 11 such
    names in the pinned oracle at this writing) -- a mid-sentence embed of
    one naturally lowercases the article (`"Helm of the Serpent King"`),
    which a case-sensitive compare never matches even though the embed is
    exactly as genuine as `"Death (Pharasma)"`'s. Opt-in and additive
    (returned names are still the ORIGINAL, correctly-cased declared
    strings -- only the comparison folds case) so every existing caller
    and test is byte-for-byte unaffected; `_PiScreen`
    (`pf1e_dashboard_producer.py`) is the one caller that opts in."""
    if case_insensitive:
        return _find_declared_pi_word_matches_impl(value.casefold(), declared_names_by_length,
                                                    fold=str.casefold)
    return _find_declared_pi_word_matches_impl(value, declared_names_by_length, fold=None)


def _find_declared_pi_word_matches_impl(search_value: str, declared_names_by_length, fold) -> list[str]:
    """Shared body for `find_declared_pi_word_matches`: `search_value` is
    already folded (or not) by the caller above; `fold` (a callable or
    `None`) is applied to each candidate NAME so the search-side and the
    name-side always agree on case-folding, while the returned match is
    always the original, correctly-cased name from `declared_names_by_length`.

    WHY WORD-BOUNDARY, NOT PLAIN SUBSTRING, FOR A NATURAL-LANGUAGE NAME:
    `redact_for_display`'s own `name` field is prose (an object's title),
    not a compound machine identifier like `type_facet` -- so unlike that
    field (see `value_carries_declared_pi_substring`'s own docstring for why
    IT stays plain-substring), a name field has real word boundaries to
    check, and doing so resolves the exact tension this module has hit
    twice now:

      * A GLOBAL plain-substring scan over every published `name`
        (SITE-PUBSTATUS-002's own history, this docstring's sibling in
        `build_public_status.py`) flags `"Brightness Seeker"` purely
        because the declared-PI deity word `"Brigh"` happens to be its
        first five letters -- `"Brigh"` is FUSED into `"Brightness"`
        (immediately followed by `t`, an alphanumeric character), not a
        word of its own.
      * A BOOK-SCOPED substring scan (this module's prior fix for that)
        stops the false positive, but at the cost of a real miss: a
        declared-PI deity or region name is very often declared PI in a
        DIFFERENT book than the one publishing an item that embeds it
        verbatim (`"Death (Pharasma)"`, published in
        `advanced_players_guide`, embeds the deity name `"Pharasma"`,
        declared PI under `inner_sea_gods`/`inner_sea_world_guide`) --
        book-scoping never even looks there.

    WORD-BOUNDARY matching resolves both at once, with no book-scoping and
    no per-name hardcoding: `"Pharasma"` in `"Death (Pharasma)"` is bounded
    by `(`/`)` on both sides -- a genuine embed, caught GLOBALLY regardless
    of book. `"Brigh"` in `"Brightness Seeker"` is followed by `t` -- fused,
    never even a candidate. The same test also naturally clears every
    grammatically-INFLECTED derivative of a declared-PI region/ethnicity
    word (`"Numerian"` from `"Numeria"`, `"Druman"` from `"Druma"`,
    `"Razmiri"` from `"Razmir"`, `"Ulfen's"` would still bound-match --
    apostrophe is non-alphanumeric -- but `"Vargouille"` from `"Varg"` and
    `"Next"` from `"Nex"` do not) WITHOUT needing an allow-list entry for
    any of them: the fusion itself is the evidence they are not the
    declared word.

    A whole-word embed CAN still be an intentional, mundane, non-PI use
    (`"Shackles of Compliance"` -- "Shackles" bounded by nothing but
    ordinary English on both sides, meaning literal manacles, not the
    pirate-isle region) -- this function only ANSWERS "is there a
    freestanding embed," it does not judge intent. A caller that must
    publish some of those anyway consults a reviewed, per-name allow-list
    on top of this result (see `scripts/site/pi_substring_allowlist.py`);
    this function itself carries no such list and never should.

    Returns every declared name that matches (there can be more than one --
    an item could embed two different region words), not just the first,
    so a caller can report exactly which term(s) triggered."""
    hits: list[str] = []
    n = len(search_value)
    for name in declared_names_by_length:
        if not name:
            continue
        candidate = fold(name) if fold else name
        start = 0
        while True:
            idx = search_value.find(candidate, start)
            if idx == -1:
                break
            before_ok = idx == 0 or not _is_word_char(search_value[idx - 1])
            end = idx + len(candidate)
            after_ok = end == n or not _is_word_char(search_value[end])
            if before_ok and after_ok:
                hits.append(name)
                break
            start = idx + 1
    return hits


def value_carries_declared_pi_word(value: str, declared_names_by_length) -> bool:
    """True if `value` embeds any name in `declared_names_by_length` as a
    freestanding word (see `find_declared_pi_word_matches`'s own
    docstring for the full rationale)."""
    return bool(find_declared_pi_word_matches(value, declared_names_by_length))


def redact_declared_pi_names(value, declared_names):
    """Recursively return a COPY of `value` with every string leaf that
    EXACTLY equals a declared-PI name replaced by `REDACTED_PI_MARKER`.
    Same exact-match semantics as `find_declared_pi_leaks` (see its
    docstring) -- this is that function's producer-side counterpart: a
    blanket, defense-in-depth final pass applied to the whole assembled
    dashboard document right before it is written, so a name-shaped leak
    through a surface this cycle did not individually chase (a category
    label, a future field) is still caught rather than shipped. The two
    call-site-specific fixes (`build_unit_shards`'s `name` field,
    `_parse_lst_first_field`'s roster rows) remain the PRIMARY defense --
    they redact with full (book, source_file, source_line) precision and
    are what the shard's own `pi_redacted_names` count reports; this pass
    is the net underneath them, not a replacement."""
    if isinstance(value, dict):
        return {k: redact_declared_pi_names(v, declared_names) for k, v in value.items()}
    if isinstance(value, list):
        return [redact_declared_pi_names(v, declared_names) for v in value]
    if isinstance(value, str) and value.strip() in declared_names:
        return REDACTED_PI_MARKER
    return value
