#!/usr/bin/env python3
"""E1.6 combat lane: re-derive the lane's figures twice and check the data against the source.

Run from the repo root:  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.6_checks.py
Needs the page text (P) and the PDF, which live outside the git tree:
  P=/home/ubuntu/workspace/inbox/cyberpunk-red/pages
  PDF=/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf
Exit 0 when every check passes, 1 otherwise.

Does not cover: that a value the page prints is the value the page image shows for every cell outside the
pages the receipt lists as image-checked (that is E1.V's 10% sample); the vehicle stat table, melee weapon
table, armour table and street-drug table (E1.4 / E1.5 own those kinds).
"""
import json, os, re, subprocess, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 5))
CORPUS = os.path.join(ROOT, "data", "cyberpunk-red", "corpus")
P = os.environ.get("P", "/home/ubuntu/workspace/inbox/cyberpunk-red/pages")
PDF = os.environ.get("PDF", "/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf")
FAILS = []


def load(kind):
    return json.load(open(os.path.join(CORPUS, kind + ".json")))


def ptxt(n):
    return open(f"{P}/p{n:03d}.txt").read()


def pdf(n, layout):
    cmd = ["pdftotext", "-f", str(n), "-l", str(n)] + (["-layout"] if layout else []) + [PDF, "-"]
    return subprocess.run(cmd, capture_output=True, text=True).stdout


def check(label, ok, detail):
    print(("PASS " if ok else "FAIL ") + label + ": " + detail)
    if not ok:
        FAILS.append(label)


def table_region(text, start_re, stop_re):
    out, f = [], False
    for ln in text.splitlines():
        if re.search(start_re, ln):
            f = True
            continue
        if f and re.search(stop_re, ln):
            f = False
        if f and ln.strip():
            out.append(ln)
    return out


def norm(s):
    s = s.replace("’", "'").replace("‘", "'").replace("®", "")
    return re.sub(r"\s+", " ", s).strip()


# ---- F-70 actions (estimate 18) ----
def f70():
    a = 0
    names_a = []
    for n in (169, 170):
        for ln in table_region(ptxt(n), r"^\s+Action\s+Data", r"A\s+ctions in Detail|^\s*\d+\s*$"):
            if len(ln) - len(ln.lstrip()) < 28:
                a += 1
                names_a.append(ln.strip()[:40].split("  ")[0])
    b = set()
    for n in (169, 170):
        for ln in table_region(pdf(n, True), r"^\s+Action\s+Data", r"A\s+ctions in Detail|^\s*\d+\s*$"):
            if len(ln) - len(ln.lstrip()) < 28:
                b.add(ln.strip().split("  ")[0])
    j = [r["name"] for r in load("action")]
    check("F-70 A (page-text label column)", a == 18, f"{a}")
    check("F-70 B (PDF layout label set == record names)", b == set(j) and len(j) == 18, f"{len(b)} labels, {len(j)} records")


# ---- F-71/F-72 range DV + D-1 ----
def f71():
    tok = r"(?:\d+|N/A)"
    rows8 = rows5 = cells = nas = 0
    lines8, lines5 = [], []
    for ln in ptxt(174).splitlines():
        t = ln.split()
        if len(t) >= 8 and all(re.fullmatch(tok, x) for x in t[-8:]) and not re.fullmatch(tok, t[-9] if len(t) > 8 else "x"):
            rows8 += 1
            lines8.append(t[-8:])
        elif len(t) >= 5 and all(re.fullmatch(tok, x) for x in t[-5:]) and len(t) <= 7 and re.search(r"[A-Za-z]", ln.split()[0]):
            rows5 += 1
            lines5.append(t[-5:])
    cells = sum(len(x) for x in lines8)
    nas = sum(x.count("N/A") for x in lines8)
    check("F-71 A (value-row lines, 8 bands; N/A cells counted from the page, CUI F-71 says 11)", rows8 == 8 and cells == 64 and nas == 8, f"rows={rows8} cells={cells} N/A={nas}")
    check("F-72 A (value-row lines, 5 bands)", rows5 == 2, f"rows={rows5}")
    rd = load("range_dv")
    ss = [r for r in rd if r["mode"] == "single_shot"]
    af = [r for r in rd if r["mode"] == "autofire"]
    got8 = [[("N/A" if b["dv"] is None else str(b["dv"])) for b in r["bands"]] for r in ss]
    got5 = [[str(b["dv"]) for b in r["bands"]] for r in af]
    check("F-71 B (record cells == page cells, row order)", got8 == lines8, f"{len(got8)} rows")
    check("F-72 B (record cells == page cells, row order)", got5 == lines5, f"{len(got5)} rows")
    # D-1: every one of the 11 weapon types in exactly one single-shot row
    p172 = norm(ptxt(172) + pdf(172, False))
    types = ["Medium Pistol", "Heavy Pistol", "Very Heavy Pistol", "SMG", "Heavy SMG", "Shotgun", "Assault Rifle",
             "Sniper Rifle", "Bows & Crossbows", "Grenade Launcher", "Rocket Launcher"]
    on_page = [t for t in types if all(w in p172 for w in t.replace("&", "").split())]
    nalt = len(re.findall(r"Alt\. Fire Modes", ptxt(172)))
    check("D-1 weapon types on p172 (every word of each name present; 'Alt. Fire Modes' rows == 11)", len(on_page) == 11 and nalt == 11, f"{len(on_page)} of 11 names, {nalt} Alt. Fire Modes rows")
    cnt = {t: sum(t in r["weapon_types"] for r in ss) for t in types}
    check("D-1 each of the 11 types in exactly one single-shot row", all(v == 1 for v in cnt.values()) and len(cnt) == 11, str(cnt))
    allw = sorted(w for r in ss for w in r["weapon_types"])
    check("D-1 no extra type in any row", allw == sorted(types), f"{len(allw)} entries")
    wj = os.path.join(CORPUS, "weapon.json")
    if os.path.exists(wj):
        wnames = {w.get("name") for w in json.load(open(wj))}
        check("D-1 weapon.json present: all 11 names are weapon records or absent-by-name (info)", True,
              f"weapon.json has {len(wnames)} records; names matching the 11 types: {sorted(set(types) & wnames)}")
    else:
        print("INFO weapon.json not in this tree (E1.4 lane); the 11 types are checked against p172 above; E1.V re-runs against weapon.json")


# ---- F-73 damage bands ----
def f73():
    def dice(n):
        for ln in ptxt(n).splitlines():
            if ln.strip().startswith("Damage") and re.search(r"\dd6", ln):
                return re.findall(r"\dd6", ln)
    a1, a2 = dice(177), dice(179)
    db = load("damage_band")
    b1 = [r["damage_dice"] for r in db if r["system"] == "brawling"]
    b2 = [r["damage_dice"] for r in db if r["system"] == "martial_arts"]
    check("F-73 A (page Damage rows)", len(a1) == 4 and len(a2) == 4, f"p177 {a1} p179 {a2}")
    check("F-73 B (records == page rows)", a1 == b1 and a2 == b2, f"{b1} {b2}")


# ---- F-74 martial arts moves (estimate 9) ----
def f74():
    raw = pdf(180, False) + pdf(181, False)
    a = len(re.findall(r"Requirement:", raw))
    b = len([l for l in raw.splitlines() if l.startswith("▶") and "◀" not in l])
    j = load("martial_arts")
    check("F-74 A (count of 'Requirement:' on p180-p181)", a == 9, f"{a}")
    check("F-74 B (move headings, minus form headings)", b == 9, f"{b}")
    names_ok = all(f"▶ {r['name']}" in raw for r in j)
    forms = {}
    for r in j:
        forms[r["form"]] = forms.get(r["form"], 0) + 1
    check("F-74 records (9 = 1 shared + 4 x 2, every heading on the page)", len(j) == 9 and names_ok and forms == {"Shared": 1, "Aikido": 2, "Karate": 2, "Judo": 2, "Taekwondo": 2}, str(forms))


# ---- F-75 cover ----
def f75():
    a = [m.group(1) for m in re.finditer(r"(?:^|\s{2,})(Steel|Stone|Bulletproof Glass|Concrete|Wood|Plaster/Foam/Plastic)\s{2,}\d+ HP\s{2,}(?:\d+ HP|0 HP \(Not Cover\))\s*$", ptxt(183), re.M)]
    j = load("cover")
    check("F-75 A (p183 material rows)", len(a) == 6, f"{a}")
    check("F-75 B (record order/materials/HP == page)", [r["material"] for r in j] == a, "")
    ex = 0
    for ln in table_region(pdf(184, True), r"Cover Material and Thickness Examples", r"Using Shields"):
        if re.search(r"\s{2,}(Thick|Thin|0 HP \(Not Cover\)).*\d+ (or \d+ )?HP", ln):
            ex += 1
    nj = sum(len(r["examples"]) + len(r.get("not_cover_examples", [])) for r in j)
    check("F-75 examples (p184 rows == records)", ex == 24 == nj, f"page {ex}, records {nj}")


# ---- F-76 critical injuries ----
def f76():
    inj = re.compile(r"^\s+(2|3|4|5|6|7|8|9|10|11|12)\s+([A-Z][A-Za-z ]+?)(?:\s{2,}|\s*$)")
    def rows(pages):
        out = []
        for n in pages:
            for ln in ptxt(n).splitlines():
                m = inj.match(ln)
                if m:
                    out.append((int(m.group(1)), m.group(2).strip()))
        return out
    c1 = rows([222, 223])
    c2 = rows([188, 189])
    j = load("critical_injury")
    jr = [(r["roll_2d6"], r["name"]) for r in j]
    check("F-76 A (p222-p223 rows)", len(c1) == 22, f"{len(c1)}")
    check("F-76 B (p188-p189 rows)", len(c2) == 22, f"{len(c2)}")
    check("F-76 records == both copies, same order", jr == c1 == c2, f"{len(jr)} records")
    both = 0
    for r in j:
        e = norm(r["injury_effect"])
        pgs = [r["source_page"], r["also_printed_on"]]
        ok = all(e in norm(pdf(int(p[1:]), False)) or e in norm(ptxt(int(p[1:]))) or all(s in norm(pdf(int(p[1:]), False)) for s in re.split(r"(?<=[.]) ", e)) for p in pgs)
        both += ok
    check("F-76 each effect string is on both the canonical page and the combat-chapter copy", both == 22, f"{both} of 22")
    bd = sum(r["base_death_save_penalty_increase"] for r in j)
    raw = "".join(norm(ptxt(n)) for n in (222, 223))
    check("F-76 Base Death Save Penalty +1 count (records vs page phrase)", bd == len(re.findall(r"Base Death Save Penalty is increased by 1", raw)) == 11, f"records {bd}")


# ---- F-77 wound states ----
def f77():
    j = load("wound_state")
    t1, t2 = norm(ptxt(187)), norm(ptxt(221))
    ok = all(r["name"] in t1 and r["name"].split()[0] in t2 for r in j) and len(j) == 4
    dv = re.findall(r"(Lightly|Seriously|Mortally) Wounded: DV(\d+)", norm(ptxt(223)))
    check("F-77 A (4 names on p187 and p221)", ok, str([r["name"] for r in j]))
    check("F-77 B (stabilization DVs 10/13/15 on p223 prose == records)", [int(d) for _, d in dv] == [r["stabilization_dv"] for r in j[:3]] == [10, 13, 15], str(dv))


# ---- F-78 hazards (print-only) ----
def f78():
    j = load("hazard")
    t = norm(ptxt(181) + ptxt(182) + pdf(181, False) + pdf(182, False))
    miss = []
    for r in j:
        for s in re.split(r"(?<=[.]) ", norm(r["text"])):
            if s not in t:
                miss.append((r["name"], s[:50]))
    check("F-78 records 9, every sentence verbatim on p181-p182", len(j) == 9 and not miss, f"{len(j)} records, missing {miss}")


# ---- F-79 vehicle maneuvers (estimate 7) ----
def f79():
    a = []
    for ln in table_region(ptxt(193), r"Situations that require a Maneuver", r"Losing Control"):
        m = re.match(r"^.{60,}?\s{2,}([A-Z][A-Za-z ()]+?)\s{2,}(\d+)\s*$", ln) or re.match(r"^\s*([A-Z][A-Za-z ()]+?)\s{2,}(\d+)\s*$", ln)
        if m and "Maneuver" not in m.group(1) or (m and "Air Vehicle" in m.group(1)):
            a.append((m.group(1).strip(), int(m.group(2))))
    raw = [l.strip() for l in pdf(193, False).splitlines()]
    i = raw.index("DV")
    seq = [l for l in raw[i + 1:i + 40] if l]
    b = [(seq[k], int(seq[k + 1])) for k in range(0, 14, 2)]
    j = [(r["name"], r["dv"]) for r in load("vehicle_maneuver")]
    check("F-79 A (p193 layout rows ending in a DV)", len(a) == 7, f"{a}")
    check("F-79 B (PDF raw name/DV pairs)", len(b) == 7 and all(not x[0].isdigit() for x in b), f"{b}")
    check("F-79 records == both, same order", j == a == b, f"{j}")


# ---- F-80 Trauma Team ----
def f80():
    t = load("trauma_team")
    a = len(re.findall(r"\s10\s+\d+\s+\d+\s+\d+\s*$", ptxt(225), re.M))
    h = len(re.findall(r"^\s+(DV17 or higher|DV15|DV13|DV10)\s*$", ptxt(226), re.M))
    sh = t[0]["shared"]
    members = sum(m["count"] for m in sh["member_rows"])
    check("F-80 A (p225 stat-row lines, p226 DV rows)", a == 4 and h == 4, f"member rows {a}, hospital rows {h}")
    check("F-80 B (records: 2 plans, 4 member rows (5 people), 4 hospital rows)", len(t) == 2 and len(sh["member_rows"]) == 4 and members == 5 and len(sh["hospital_rows"]) == 4, f"{len(t)} plans, {len(sh['member_rows'])} rows, {members} people, {len(sh['hospital_rows'])} bill rows")
    nums = re.findall(r"\s(10)\s+(\d+)\s+(\d+)\s+(\d+)\s*$", ptxt(225), re.M)
    rec = [(str(m["combat_number"]), str(m["sp"]), str(m["hp"]), str(m["move_and_body"])) for m in sh["member_rows"]]
    check("F-80 member numbers == page", [tuple(x) for x in nums] == rec, str(rec))


# ---- all text fields: sentence-level verbatim against the source page(s) ----
def verbatim():
    cache = {}
    def page(p):
        n = int(p[1:])
        if n not in cache:
            cache[n] = norm(ptxt(n)) + " " + norm(pdf(n, False))
        return cache[n]
    miss, tot, joined = [], 0, [0]
    skip = {"source_page", "also_printed_on", "examples_source_page", "transcription_note", "members_source_page", "hospital_source_page", "member_rows_note", "check_text"}
    def walk(o, path):
        if isinstance(o, str):
            yield path, o
        elif isinstance(o, dict):
            for k, v in o.items():
                if k not in skip:
                    yield from walk(v, path + "/" + k)
        elif isinstance(o, list):
            for i, v in enumerate(o):
                yield from walk(v, path + f"[{i}]")
    for f in sorted(os.listdir(CORPUS)):
        k = f[:-5]
        if k not in ("action", "range_dv", "damage_band", "martial_arts", "cover", "critical_injury", "wound_state", "hazard", "vehicle_maneuver", "trauma_team"):
            continue
        for r in load(k):
            pages = [r["source_page"]] + [r[x] for x in ("also_printed_on", "examples_source_page") if x in r]
            if k == "trauma_team":
                pages += ["p224", "p225", "p226"]
            body = " ".join(page(p) for p in pages)
            for path, s in walk(r, ""):
                if len(s) < 45 or "|" in s:
                    continue
                for sent in re.split(r"(?<=[.]) ", norm(s)):
                    if len(sent) < 25:
                        continue
                    tot += 1
                    if sent in body:
                        continue
                    if sent in body.replace("offer- ing", "offering").replace("/ month", "/month"):
                        joined[0] += 1
                        continue
                    w = sent.split(" ")
                    if any(" ".join(w[:i]) in body and " ".join(w[i:]) in body for i in range(2, len(w) - 1)):
                        joined[0] += 1  # a printed column break splits the sentence in reading order
                        continue
                    miss.append((k, r["name"], path, sent[:70]))
    check("verbatim (sentence-level, text fields of 10 kinds)", not miss, f"{tot} sentences checked, {joined[0]} matched only across a column break or hyphen, {len(miss)} not found: {miss[:6]}")


for fn in (f70, f71, f73, f74, f75, f76, f77, f78, f79, f80, verbatim):
    fn()
print("RESULT", "FAIL " + str(FAILS) if FAILS else "ALL PASS")
sys.exit(1 if FAILS else 0)
