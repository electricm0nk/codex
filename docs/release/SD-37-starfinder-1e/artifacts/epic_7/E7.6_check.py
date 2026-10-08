#!/usr/bin/env python3
"""SD-37 E7.6 acceptance: every figure in release-notes.md has a command, and the value written
equals the value re-derived now by two independent implementations.

Exit 0 only if all hold. `--plant` also proves the check can fail: it corrupts one value, drops one
command and adds one unregistered figure id in memory, and requires each corruption to be rejected.
"""
import importlib.util, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("e76_figures", os.path.join(HERE, "E7.6_figures.py"))
F = importlib.util.module_from_spec(spec)
spec.loader.exec_module(F)
NOTES = os.path.join(F.ROOT, F.PKG, "release-notes.md")


def problems(text, derived):
    bad = []
    lines = {}
    for l in text.splitlines():
        m = re.match(r"^- \*\*(R-\d+)\*\*", l)
        if m:
            lines[m.group(1)] = l
    for fid, d in derived.items():
        if not d["agree"]:
            bad.append(f"{fid}: implementations disagree A={d['A']!r} B={d['B']!r}")
        l = lines.get(fid)
        if l is None:
            bad.append(f"{fid}: no figure line in release-notes.md")
            continue
        if f"**{d['A']}**" not in l:
            bad.append(f"{fid}: the line does not carry the derived value **{d['A']}**")
        if d["A_cmd"] not in l:
            bad.append(f"{fid}: the line does not quote implementation A's command")
        if d["B_desc"] not in l:
            bad.append(f"{fid}: the line does not name implementation B")
    for fid in lines:
        if fid not in derived:
            bad.append(f"{fid}: a figure line with no registered command")
    for need in ("## Known Issues", "## Does not cover", "## Figures"):
        if need not in text:
            bad.append(f"missing section {need}")
    return bad


def main():
    if not os.path.isfile(NOTES):
        print("NO_RELEASE_NOTES", NOTES)
        return 1
    text = open(NOTES, encoding="utf-8").read()
    derived = F.derive()
    bad = problems(text, derived)
    for b in bad:
        print("FAIL", b)
    if "--plant" in sys.argv and not bad:
        first = next(iter(derived))
        v = derived[first]["A"]
        plants = {
            "wrong value": text.replace(f"**{v}**", "**0**", 1),
            "dropped command": text.replace(derived[first]["A_cmd"], "", 1),
            "unregistered figure": text + "\n- **R-999** **1** invented\n",
        }
        for name, t in plants.items():
            n = len(problems(t, derived))
            print(f"PLANT {name}: {'REJECTED' if n else 'ACCEPTED (check is blind)'} ({n} problems)")
            if not n:
                bad.append("plant " + name)
    print("RESULT", "FAIL" if bad else "PASS", len(derived), "figures")
    return 1 if bad else 0


sys.exit(main())
