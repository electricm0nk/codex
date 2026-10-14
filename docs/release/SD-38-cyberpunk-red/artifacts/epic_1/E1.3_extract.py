#!/usr/bin/env python3
"""E1.3 helper: per-skill blocks from the Skill List pages (PDF printed 131-142 = file p132-p143)
using pdftotext -bbox-layout (blocks separate the sidebars from the two text columns).
Usage: python3 -I E1.3_extract.py <pdf> -> JSON {skill: {stat,x2,page,text,rejoined}} on stdout."""
import html, json, re, subprocess, sys
PDF = sys.argv[1]
HEAD = re.compile(r"^(?P<n>[A-Za-z][A-Za-z/& ]*?)\s*(?P<x>\(x2\))?\s*\.{3,}\s*(?P<s>INT|REF|DEX|TECH|COOL|WILL|EMP)$")
CAT = re.compile(r"^(Awareness|Body|Control|Education|Fighting|Performance|Ranged Weapon|Social|Technique?) Skills?$")
out, cur = {}, None
for pg in range(132, 144):
    x = subprocess.run(["pdftotext", "-f", str(pg), "-l", str(pg), "-bbox-layout", PDF, "-"],
                       capture_output=True, text=True, check=True).stdout
    blocks = []
    for b in re.finditer(r'<block xMin="([\d.]+)" yMin="([\d.]+)" xMax="([\d.]+)" yMax="([\d.]+)">(.*?)</block>', x, re.S):
        xmin, ymin = float(b[1]), float(b[2])
        lines = [" ".join(html.unescape(w) for w in re.findall(r">([^<]+)</word>", ln))
                 for ln in re.findall(r"<line .*?</line>", b[5], re.S)]
        blocks.append((xmin, ymin, lines))
    heads = [xm for xm, ym, ls in blocks if len(ls) == 1 and HEAD.match(ls[0].strip())]
    cols = sorted({round(h) for h in heads})
    # cluster column x within 4pt
    cl = []
    for c in cols:
        if not cl or c - cl[-1] > 4:
            cl.append(c)
    def col_of(xm):
        for i, c in enumerate(cl):
            if abs(xm - c) <= 4:
                return i
        return None
    seq = sorted([(col_of(xm), ym, ls) for xm, ym, ls in blocks if col_of(xm) is not None], key=lambda t: (t[0], t[1]))
    for _, _, ls in seq:
        first = ls[0].strip()
        m = HEAD.match(first) if len(ls) == 1 else None
        if m:
            cur = m["n"].strip()
            out[cur] = {"stat": m["s"], "x2": bool(m["x"]), "page": pg, "parts": []}
            continue
        if len(ls) == 1 and CAT.match(first):
            cur = None
            continue
        if cur:
            out[cur]["parts"].append(ls)
for k, v in out.items():
    txt, rej = "", []
    for ls in v.pop("parts"):
        for l in ls:
            l = l.strip()
            if txt.endswith("-") and l[:1].islower():
                w = txt.rsplit(" ", 1)[-1] + l.split(" ", 1)[0]
                rej.append(w)
                txt = txt[:-1] + l
            else:
                txt = (txt + " " + l) if txt else l
    v["text"], v["rejoined"] = txt, rej
print(json.dumps(out, indent=1, ensure_ascii=False))
