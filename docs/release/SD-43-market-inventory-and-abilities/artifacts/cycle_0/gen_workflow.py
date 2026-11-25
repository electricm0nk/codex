#!/usr/bin/env python3
"""Generate sd43-workflow.js from the package files (workflow-instruction.md §2.4).

Card table, prefix (§2.1), procedure (§6) and each card's epic-breakdown section are copied
verbatim from the package, so the script cannot drift from it. Re-run after any package edit:
    python3 docs/release/SD-43-market-inventory-and-abilities/artifacts/cycle_0/gen_workflow.py
"""
import json, re, pathlib

PKG = pathlib.Path(__file__).resolve().parents[2]
wi = (PKG / "workflow-instruction.md").read_text()
eb = (PKG / "epic-breakdown.md").read_text()
kb = (PKG / "kanban.md").read_text()

# §2.1 prefix: the ```text block after the "### 2.1" heading
s21 = wi.split("### 2.1", 1)[1]
prefix = s21.split("```text\n", 1)[1].split("\n```", 1)[0]
# §6 procedure: from "## 6." up to "## 7."
proc = wi.split("## 6. Per-cycle procedure", 1)[1].split("\n## 7.", 1)[0]
proc = "## 6. Per-cycle procedure" + proc

def sections(text):
    out, cur, buf = {}, None, []
    for line in text.splitlines():
        m = re.match(r"^### ((?:C|E)\d+(?:\.(?:\d+|MC|G))?) ", line)
        if m or line.startswith("## "):
            if cur:
                out[cur] = "\n".join(buf).rstrip()
            cur, buf = (m.group(1), [line]) if m else (None, [])
        elif cur:
            buf.append(line)
    if cur:
        out[cur] = "\n".join(buf).rstrip()
    return out

secs = sections(eb)
cards = []
for line in kb.splitlines():
    f = line.split("|")
    if len(f) < 10 or not re.match(r"^ (C|E)\d", f[1]):
        continue
    cid = f[1].strip()
    deps = [] if f[5].strip() in ("—", "") else [d.strip() for d in f[5].split(",")]
    cards.append({
        "id": cid, "tier": f[3].strip(), "deps": deps, "lane": f[6].strip(),
        "cargo": f[7].strip() != "no", "hub": f[8].strip() == "hub",
        "row": line.strip(), "section": secs.get(cid, ""),
    })

# A discovered row that says "must be complete before X" becomes a dependency of X (run 8: E3.MC
# started while E3.18-E3.20 were todo, because only the note said so).
by_id = {c["id"]: c for c in cards}
for c in cards:
    for target in re.findall(r"must be complete before ((?:C|E)\d+\.\w+)", c["row"]):
        if c["id"] not in by_id[target]["deps"]:
            by_id[target]["deps"].append(c["id"])
            by_id[target]["row"] += f" [script: depends on {c['id']} by its note]"

eb_ids = set(re.findall(r"^\| ((?:C|E)\d+(?:\.\w+)?) \|", eb, re.M))
assert {c["id"] for c in cards} == eb_ids, ({c["id"] for c in cards} ^ eb_ids)
missing = [c["id"] for c in cards if not c["section"]]
assert not missing, f"no epic-breakdown section for {missing}"
assert all(c["tier"] in ("opus", "sonnet", "haiku", "lead", "operator") for c in cards)
ids = {c["id"] for c in cards}
assert all(d in ids for c in cards for d in c["deps"])

def phase(cid):
    return {"C": "C — package, pointer, graph", "E0": "E0 — census and engine-gap measurement",
            "E1": "E1 — Market and inventory", "E2": "E2 — sheet layout, campaign button, rest text",
            "E3": "E3 — the Abilities tab", "E4": "E4 — item names (E4.G operator wait)",
            "E5": "E5 — reports, Update menu, Developer tab",
            "E6": "E6 — campaign folder, House Rules, custom items", "E7": "E7 — closure"}[
        "C" if cid.startswith("C") else cid.split(".")[0]]

for c in cards:
    c["phase"] = phase(c["id"])

# Lane order = kanban order, except the sheet lane runs E6.2–E6.4 before E4.2 so the
# E4.G wait does not stall them (workflow-instruction.md §2.4 item 6).
order = [c["id"] for c in cards]
sheet_tail = ["E4.1", "E6.2", "E6.3", "E6.4", "E4.2"]
lanes = {}
for c in cards:
    lanes.setdefault(c["lane"], []).append(c["id"])
lanes["sheet"] = [i for i in lanes["sheet"] if i not in sheet_tail] + sheet_tail
# Stable reorder: a card moves after any same-lane dependency listed later (discovered rows sit below their MC).
for lane, seq in lanes.items():
    out = []
    pending = list(seq)
    while pending:
        cid = next((i for i in pending if all(d not in pending for d in by_id[i]["deps"])), None)
        assert cid, f"dependency cycle in lane {lane}: " + str({i: by_id[i]["deps"] for i in pending if any(d in pending for d in by_id[i]["deps"])})
        pending.remove(cid)
        out.append(cid)
    lanes[lane] = out
# every dependency inside a lane must come earlier in that lane (no self-deadlock)
for lane, seq in lanes.items():
    for i, cid in enumerate(seq):
        dep = next(c for c in cards if c["id"] == cid)["deps"]
        for d in dep:
            if d in seq:
                assert seq.index(d) < i, (lane, cid, d)

tmpl = (PKG / "artifacts/cycle_0/sd43-workflow.template.js").read_text()
js = (tmpl.replace("__PREFIX__", json.dumps(prefix))
          .replace("__PROC__", json.dumps(proc))
          .replace("__CARDS__", json.dumps(cards, indent=1, ensure_ascii=False))
          .replace("__LANES__", json.dumps(lanes)))
(PKG / "artifacts/cycle_0/sd43-workflow.js").write_text(js)
print(f"cards={len(cards)} lanes={ {k: len(v) for k, v in lanes.items()} }")
