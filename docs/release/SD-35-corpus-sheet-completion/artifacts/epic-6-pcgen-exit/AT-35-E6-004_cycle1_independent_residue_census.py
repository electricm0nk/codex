"""AT-35-E6-004 cycle 1 -- an INDEPENDENT reading of the live-side PCGen residue.

Run from the repository root:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_independent_residue_census.py

Why a second instrument. `AT-35-E6-004`'s whole job is to certify that the gate
reads zero *and* that a zero means what it claims -- a green gate that does not
measure the thing it names is a false green (`decisions.md` §19 / ruling B16,
`AGENTS.md` rule 7, `validate-proxies-against-known-truth`). Taking the gate's
own word for its own correctness would be the same mistake one layer up. So
this script re-reads the same population with a DIFFERENT algorithm: a
character-level Rust scanner that classifies every byte as code, comment, or
string/char-literal body (raw strings and block comments included) and
brace-matches `#[cfg(test)]` item regions over code characters only. It borrows
from the gate only its live-root list and its regex set -- never its line
splitting, its comment rule, or its region finder.

It answers one question: does the gate SKIP any shipping line that carries a
PCGen hit? Output at the cycle's HEAD:

    gate_skipped_lines=90433 independent_skipped_lines=90433 files_with_boundary_mismatch=0
    INDEPENDENT live_files=0 live_hits=0
    over_skip_hits_hidden_by_gate=0
    lines_gate_counts_that_independent_calls_test=0

Before this cycle's fix to `cfg_test_ranges` the first line read
`gate_skipped_lines=91053 independent_skipped_lines=90433
files_with_boundary_mismatch=3`: `b"not-json-{garbage"` inside a `#[cfg(test)]`
module of `apps/desktop/src-tauri/src/update/transaction.rs` left the gate's
line-level brace counter one `{` deep, and its skip blanked 618 lines of
shipping code. `over_skip_hits_hidden_by_gate` was 0 then too -- the verdict was
right by luck, which is exactly why the check exists.
"""
import os, re, sys
sys.path.insert(0, 'scripts')
import pcgen_residue_gate as G

PAT = {n: re.compile(rx) for n, rx in G.PATTERNS.items()}
CFG = re.compile(r"#!?\[\s*cfg\(\s*test\s*\)\s*\]")

def classify(text):
    """Per-character class map: 0=code, 1=comment, 2=string/char literal."""
    n = len(text); out = bytearray(n); i = 0
    while i < n:
        c = text[i]
        if c == '/' and i+1 < n and text[i+1] == '/':
            j = text.find('\n', i)
            j = n if j < 0 else j
            for k in range(i, j): out[k] = 1
            i = j
        elif c == '/' and i+1 < n and text[i+1] == '*':
            j = text.find('*/', i+2)
            j = n if j < 0 else j+2
            for k in range(i, j): out[k] = 1
            i = j
        elif c == 'r' and i+1 < n and text[i+1] in '#"':
            k = i+1; hashes = 0
            while k < n and text[k] == '#': hashes += 1; k += 1
            if k < n and text[k] == '"':
                term = '"' + '#'*hashes
                j = text.find(term, k+1)
                j = n if j < 0 else j+len(term)
                for m in range(i, j): out[m] = 2
                i = j
            else:
                i += 1
        elif c == '"':
            j = i+1
            while j < n:
                if text[j] == '\\': j += 2; continue
                if text[j] == '"': j += 1; break
                j += 1
            for m in range(i, min(j, n)): out[m] = 2
            i = j
        elif c == "'":
            # char literal or lifetime; only treat as literal when it closes within 4 chars
            j = i+1; k = j
            closed = -1
            while k < n and k < j+4:
                if text[k] == '\\': k += 2; continue
                if text[k] == "'": closed = k; break
                k += 1
            if closed > 0:
                for m in range(i, closed+1): out[m] = 2
                i = closed+1
            else:
                i += 1
        else:
            i += 1
    return out

def ind_cfg_ranges(text):
    """0-based inclusive line ranges of #[cfg(test)] items, brace-matched over
    code characters only."""
    cls = classify(text)
    starts = [0]
    for i, ch in enumerate(text):
        if ch == '\n': starts.append(i+1)
    def lineno(pos):
        lo, hi = 0, len(starts)-1
        while lo < hi:
            mid = (lo+hi+1)//2
            if starts[mid] <= pos: lo = mid
            else: hi = mid-1
        return lo
    ranges = []
    for m in CFG.finditer(text):
        if cls[m.start()]: continue          # inside a comment or string
        i = m.end(); depth = 0; started = False; end = None
        while i < len(text):
            if cls[i]:
                i += 1; continue
            c = text[i]
            if c == '{': depth += 1; started = True
            elif c == '}':
                depth -= 1
                if started and depth <= 0: end = i; break
            elif c == ';' and not started: end = i; break
            i += 1
        if end is None: end = len(text)-1
        ranges.append((lineno(m.start()), lineno(end)))
    # merge overlaps
    ranges.sort(); merged = []
    for a, b in ranges:
        if merged and a <= merged[-1][1]: merged[-1] = (merged[-1][0], max(merged[-1][1], b))
        else: merged.append((a, b))
    return merged

over_skip = []          # gate skipped, independent says ships, AND carries a hit
under = []
tot_gate_skip = tot_ind_skip = 0
files_boundary_mismatch = 0
ind_live_files = ind_live_hits = 0
for lr, rel, ap in G._iter_live_source_files('.'):
    text = open(ap, encoding='utf-8', errors='replace').read()
    lines = text.splitlines()
    g = set()
    for a, b in G.cfg_test_ranges(lines): g.update(range(a, b+1))
    ind = set()
    for a, b in ind_cfg_ranges(text): ind.update(range(a, b+1))
    tot_gate_skip += len(g); tot_ind_skip += len(ind)
    if g != ind: files_boundary_mismatch += 1
    # independent live reading: drop comments per B14, drop ind cfg(test) ranges per B15
    fh = 0
    for i, line in enumerate(lines):
        if i in ind: continue
        if line.lstrip().startswith('//'): continue
        h = sum(len(rx.findall(line)) for rx in PAT.values())
        fh += h
        if h and i in g:
            over_skip.append((rel, i+1, line.strip()[:110]))
    if fh:
        ind_live_files += 1; ind_live_hits += fh
    for i in (ind - g):
        line = lines[i]
        if line.lstrip().startswith('//'): continue
        if sum(len(rx.findall(line)) for rx in PAT.values()):
            under.append((rel, i+1))

print(f"gate_skipped_lines={tot_gate_skip} independent_skipped_lines={tot_ind_skip} files_with_boundary_mismatch={files_boundary_mismatch}")
print(f"INDEPENDENT live_files={ind_live_files} live_hits={ind_live_hits}")
print(f"over_skip_hits_hidden_by_gate={len(over_skip)}")
for r in over_skip[:40]: print("  OVERSKIP", r)
print(f"lines_gate_counts_that_independent_calls_test={len(under)}")
for r in under[:20]: print("  EXTRA", r)
