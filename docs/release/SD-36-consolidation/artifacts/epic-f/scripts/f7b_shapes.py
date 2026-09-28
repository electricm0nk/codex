#!/usr/bin/env python3
"""SD-36 Epic F7b (converter step): the shape checks for the F7b prose deltas.

F7b changes one field on the records it touches, `prose` (plus, on the records whose prose
formula reads a name the oracle provably reads as 0, the `provenance.undeclared_in_pinned_tree`
note that reading always writes). Every prose delta is a combination of these classes, each
checked here structurally against the baseline record, never by a record list alone:

  - `formula_render`: a formula the source wrote into a prose TEXT (`(min(10,CASTERLEVEL))d6`,
    `(CASTERLEVEL) rounds`) is a typed piece now (`Slot`, `DiceCount`, or a `Dice` modifier);
    the words around it are unchanged (`sheet_rule/prose.rs::lower_prose_formulas`);
  - `formula_unconverted`: such a formula that does not convert is gone from the words, and the
    words around it are unchanged (an `inline-formula-unconverted` row names the record);
  - `bracket_escape`: in a spell's or psionic power's text, `[`/`]` print as the book's `(`/`)`;
  - `out_of_inventory_line`: a prose line whose condition names a record outside the converted
    inventory, decided as the evaluator decides it (never held), is decided never to print and
    is not emitted (`prose.rs::decide_out_of_inventory`);
  - `out_of_inventory_condition`: such a condition decided to a smaller condition, or to none;
  - `former_words`: a record the pre-F7b words rewrite (`inline-formula-in-prose`, baseline
    `_defects/`) had already turned into words; the formula is a typed piece now, or (a `DC10+HD`
    aside the old rewrite mistook for a formula) the book's words again. Checked by the family,
    condition and flag sequence being unchanged and no formula shape surviving.

The detector below is the converter's own rule, restated (`formula.rs::is_prose_formula`,
`prose.rs::prose_formula_spans`); a disagreement between the two shows as a failed shape check.
"""
from __future__ import annotations

import copy
import json
import re

FUNCS = {"min", "max", "floor", "ceil"}
LEAVES = {
    "CASTERLEVEL", "SPELLLEVEL", "CL", "TL", "HD", "BAB", "CR", "SIZE", "SIZEMOD",
    "STR", "DEX", "CON", "INT", "WIS", "CHA",
}
ABILITIES = {"STR", "DEX", "CON", "INT", "WIS", "CHA"}
_TOK = re.compile(r"\s*(?:(\d+(?:\.\d+)?)|([A-Za-z_%][A-Za-z0-9_%.]*)|(>=|<=|==|!=|&&|\|\||[-+*/(),<>\"]))")


def _tokens(s: str):
    out, i = [], 0
    while i < len(s):
        if s[i] in " \t":
            i += 1
            continue
        m = _TOK.match(s, i)
        if not m or m.end() == i:
            return None
        out.append(m.groups())
        i = m.end()
    return out


def _parses(toks) -> bool:
    pos = [0]

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else None

    def primary():
        t = peek()
        if t is None:
            raise ValueError
        pos[0] += 1
        if t[0]:
            return
        if t[2] == "(":
            expr()
            if peek() is None or peek()[2] != ")":
                raise ValueError
            pos[0] += 1
            return
        if t[1]:
            if peek() is not None and peek()[2] == "(":
                pos[0] += 1
                if peek() is not None and peek()[2] == ")":
                    pos[0] += 1
                    return
                while True:
                    expr()
                    if peek() is not None and peek()[2] == ",":
                        pos[0] += 1
                        continue
                    break
                if peek() is None or peek()[2] != ")":
                    raise ValueError
                pos[0] += 1
            return
        raise ValueError

    def unary():
        t = peek()
        if t is not None and t[2] in ("-", "+"):
            pos[0] += 1
            return unary()
        primary()

    def term():
        unary()
        while peek() is not None and peek()[2] in ("*", "/"):
            pos[0] += 1
            unary()

    def expr():
        term()
        while peek() is not None and peek()[2] in ("+", "-"):
            pos[0] += 1
            term()

    try:
        expr()
        return pos[0] == len(toks)
    except ValueError:
        return False


def _is_leaf(name: str) -> bool:
    return name in LEAVES or (len(name) > 5 and name.endswith("SCORE") and name[:-5] in ABILITIES)


def _is_source_variable(name: str) -> bool:
    upper = sum(c.isupper() for c in name) >= 4 and all(c.isupper() or c.isdigit() or c == "_" for c in name)
    camel = name[:1].isalpha() and name.isalnum() and bool(re.search(r"[a-z][A-Z]", name))
    return upper or camel


def is_prose_formula(span: str, arithmetic_context: bool = False) -> bool:
    toks = _tokens(span)
    if not toks or not _parses(toks):
        return False
    tight = not any(c.isspace() for c in span)
    has_op = arithmetic_context or (tight and any(t[2] in ("+", "-", "*", "/") for t in toks))
    marked = False
    for i, t in enumerate(toks):
        if t[1]:
            if i + 1 < len(toks) and toks[i + 1][2] == "(":
                one_case = t[1] == t[1].lower() or t[1] == t[1].upper()
                if not one_case or t[1].lower() not in FUNCS:
                    return False
                marked = True
            elif _is_leaf(t[1]):
                marked |= len(t[1]) >= 4 or has_op
            elif _is_source_variable(t[1]):
                marked = True
            else:
                return False
        elif t[2] and t[2] not in ("+", "-", "*", "/", "(", ")", ","):
            return False
    return marked


def _close(t: str, open_: int):
    depth = 0
    for j in range(open_, len(t)):
        if t[j] == "(":
            depth += 1
        elif t[j] == ")":
            depth -= 1
            if depth == 0:
                return j
    return None


def spans(t: str) -> list[tuple[int, int]]:
    out, i = [], 0
    while i < len(t):
        boundary = i == 0 or not (t[i - 1].isascii() and (t[i - 1].isalnum() or t[i - 1] == "_"))
        open_ = None
        if t[i] == "(":
            open_ = i
        elif boundary and t[i].isascii() and t[i].isalpha():
            j = i
            while j < len(t) and t[j].isascii() and (t[j].isalnum() or t[j] == "_"):
                j += 1
            if j < len(t) and t[j] == "(":
                open_ = j
        if open_ is not None:
            c = _close(t, open_)
            if c is not None and is_prose_formula(t[i : c + 1], t[:i].rstrip().endswith(("+", "-"))):
                out.append((i, c + 1))
                i = c + 1
                continue
        i += 1
    return out


def has_formula(text: str) -> bool:
    return bool(spans(text))


# ---- the out-of-inventory decision (prose.rs::decide_out_of_inventory, restated) -------------

def _is_missing(a) -> bool:
    return isinstance(a, dict) and "Holds" in a and isinstance(a["Holds"].get("what"), dict) and "MissingRule" in a["Holds"]["what"]


def names_missing(a) -> bool:
    if _is_missing(a):
        return True
    if isinstance(a, dict):
        if "All" in a:
            return any(names_missing(t) for t in a["All"])
        if "AtLeast" in a:
            return any(names_missing(t) for t in a["AtLeast"]["of"])
        if "Not" in a:
            return names_missing(a["Not"])
    return False


def _all(terms):
    out = []
    for t in terms:
        if t == "Always":
            continue
        if t == "Never":
            return "Never"
        for i in (t["All"] if isinstance(t, dict) and "All" in t else [t]):
            if i not in out:
                out.append(i)
    if not out:
        return "Always"
    if len(out) == 1:
        return out[0]
    return {"All": out}


def decide(a):
    if _is_missing(a):
        return "Never"
    if isinstance(a, dict):
        if "Not" in a:
            d = decide(a["Not"])
            return "Always" if d == "Never" else "Never" if d == "Always" else {"Not": d}
        if "All" in a:
            return _all([decide(t) for t in a["All"]])
        if "AtLeast" in a:
            n = a["AtLeast"]["n"]
            dec = [decide(t) for t in a["AtLeast"]["of"]]
            met = sum(1 for t in dec if t == "Always")
            open_ = [t for t in dec if t not in ("Always", "Never")]
            need = max(0, n - met)
            if need == 0:
                return "Always"
            if len(open_) < need:
                return "Never"
            if len(open_) == 1:
                return open_[0]
            if need == len(open_):
                return _all(open_)
            return {"AtLeast": {"n": need, "of": open_}}
    return a


# ---- flattening -------------------------------------------------------------------------------

_DICE_PLUS = re.compile(r"(\d+d\d+)\s*\+\s*\x00")


def _old_text(t: str, drop: bool) -> str:
    out, cur = [], 0
    for a, b in spans(t):
        out.append(t[cur:a])
        rest = t[b:]
        m = re.match(r"d(\d+)(?![A-Za-z0-9])", rest)
        if drop:
            if (not out[-1] or out[-1].endswith(" ")) and rest.startswith(" "):
                b += 1
        else:
            out.append("\x00")
            if m:
                b += m.end()
        cur = b
    out.append(t[cur:])
    return "".join(out)


def _flatten(pieces, old: bool, spell: bool, drop: bool, slot_exprs: list):
    s = []
    for p in pieces:
        if "Text" in p:
            t = _old_text(p["Text"], drop) if old else p["Text"]
            if old and spell:
                t = t.replace("[", "(").replace("]", ")")
            s.append(t)
        elif "Slot" in p:
            slot_exprs.append(json.dumps(p["Slot"], sort_keys=True))
            s.append("\x00")
        elif "DiceCount" in p:
            s.append("\x00")
        elif "Dice" in p:
            d = p["Dice"]
            if d.get("modifier") is not None:
                slot_exprs.append(json.dumps(d["modifier"], sort_keys=True))
                s.append(f"{d['dice']}+\x00")
            else:
                s.append(f"\x02{d['dice']}\x02")
        elif "ChoiceName" in p:
            s.append(f"\x03{p['ChoiceName']}\x03")
        else:
            s.append("\x04" + json.dumps(p, sort_keys=True))
    return _DICE_PLUS.sub(lambda m: m.group(1) + "+\x00", "".join(s))


def _seg_key(seg):
    return (json.dumps(seg["family"], sort_keys=True), json.dumps(seg.get("applies"), sort_keys=True), seg.get("pick_last"), seg.get("suppress_when_all_zero"))


def _is_subsequence(small: list, big: list) -> bool:
    it = iter(big)
    return all(any(x == y for y in it) for x in small)


def classify_prose(rid: str, old: list, new: list, unconverted: bool, former_words: bool) -> str | None:
    """The `+`-joined F7b classes that explain `old -> new` exactly, or None."""
    kind = rid.split(":")[1] if rid.count(":") >= 2 else ""
    spell = kind in ("spell", "power")
    classes = set()
    kept = []
    for seg in old:
        ap = seg.get("applies")
        if ap is not None and names_missing(ap):
            d = decide(ap)
            if d == "Never":
                classes.add("out_of_inventory_line")
                continue
            seg = copy.deepcopy(seg)
            if d == "Always":
                seg.pop("applies", None)
            else:
                seg["applies"] = d
            classes.add("out_of_inventory_condition")
        kept.append(seg)
    if [_seg_key(s) for s in kept] != [_seg_key(s) for s in new]:
        return None
    if former_words:
        if any(has_formula(p["Text"]) for s in new for p in s["pieces"] if "Text" in p):
            return None
        classes.add("former_words")
        return "+".join(sorted(classes))
    old_exprs: list = []
    new_exprs: list = []
    for o_seg, n_seg in zip(kept, new):
        n = _flatten(n_seg["pieces"], False, spell, False, new_exprs)
        exprs: list = []
        if _flatten(o_seg["pieces"], True, spell, False, exprs) == n:
            drop = False
        elif unconverted and _flatten(o_seg["pieces"], True, spell, True, exprs := []) == n:
            drop = True
        else:
            return None
        old_exprs.extend(exprs)
        texts = [p["Text"] for p in o_seg["pieces"] if "Text" in p]
        if any(has_formula(t) for t in texts):
            classes.add("formula_unconverted" if drop else "formula_render")
        if spell and any("[" in t or "]" in t for t in texts):
            classes.add("bracket_escape")
    if not _is_subsequence(old_exprs, new_exprs):
        return None
    if any(has_formula(p["Text"]) for s in new for p in s["pieces"] if "Text" in p):
        return None
    return "+".join(sorted(classes)) if classes else None


def classify_provenance(old: dict, new: dict) -> str | None:
    """`provenance` differs only by names added to `undeclared_in_pinned_tree` (the oracle-zero
    reading a lowered prose formula took)."""
    o, n = dict(old or {}), dict(new or {})
    ou, nu = o.pop("undeclared_in_pinned_tree", []) or [], n.pop("undeclared_in_pinned_tree", []) or []
    if o != n or not set(ou) < set(nu):
        return None
    return "formula_oracle_zero_note"


def defect_records(other_files: dict, name: str) -> set[str]:
    raw = other_files.get(f"_defects/{name}.json")
    if not raw:
        return set()
    return {row.split(": ", 1)[0] for row in json.loads(raw)}


def is_active(fresh_rules: dict, base_rules: dict, owner: str) -> bool:
    """F7b's pins apply when the fresh tree carries F7b and the baseline does not: the owner (the
    CRB Fireball) states its short-form damage as a `DiceCount` only after F7b."""
    def carries(rules):
        r = rules.get(owner) or {}
        return any("DiceCount" in p for s in r.get("prose", []) for p in s["pieces"])
    return carries(fresh_rules) and not carries(base_rules)
