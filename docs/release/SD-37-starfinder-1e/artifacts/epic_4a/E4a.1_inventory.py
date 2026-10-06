#!/usr/bin/env python3
"""SD-37 E4a.1 inventory of src/rules_core/rules_tables (run from the repo root).

Prints the populations the receipt quotes, each with its predicate:
  A  file-level (brace depth 0) const/static items whose type is an array/slice  -> package tables
  S  file-level const/static items that are NOT arrays (scalars, registries)     -> not tables
  F  file-level zero-argument fns returning `&'static [..]`, `Vec<..>` or `impl Iterator`
  M  fns (outside test modules) with >= 5 match arms whose right-hand side is a literal
The Rust tests in rules_data_package.rs re-derive A and F independently (own scanner).
"""
import os, re, collections

ROOT = 'src/rules_core/rules_tables'

def depths(text):
    """Brace depth at the start of each line, ignoring comments, strings, char literals."""
    d, k, n, out = 0, 0, len(text), [0]
    while k < n:
        c = text[k]
        if text.startswith('//', k):
            e = text.find('\n', k); k = n if e < 0 else e; continue
        if text.startswith('/*', k):
            e = text.find('*/', k); out.extend([d] * text.count('\n', k, e)); k = e + 2; continue
        if c == '"':
            k += 1
            while k < n and text[k] != '"':
                if text[k] == '\\': k += 1
                elif text[k] == '\n': out.append(d)
                k += 1
            k += 1; continue
        if c == "'" and k + 2 < n and (text[k + 2] == "'" or (text[k + 1] == '\\' and text[k + 3] == "'")):
            k += 3 if text[k + 2] == "'" else 4; continue
        if c == '{': d += 1
        elif c == '}': d -= 1
        elif c == '\n': out.append(d)
        k += 1
    return out

item = re.compile(r'^\s*(pub(?:\([a-z]+\))?\s+)?(const|static)\s+([A-Z_0-9]+)\s*:\s*(.+?)\s*=')
fnre = re.compile(r'^\s*(pub(?:\([a-z]+\))?\s+)?(const\s+)?fn\s+(\w+)\s*\(\s*\)\s*->\s*(.+?)\s*\{?\s*$')
A, S, F, M = [], collections.Counter(), [], []
for d, _, fs in os.walk(ROOT):
    for f in sorted(fs):
        if not f.endswith('.rs'):
            continue
        p = os.path.join(d, f); text = open(p).read(); lines = text.split('\n'); dep = depths(text)
        for i, line in enumerate(lines):
            if i >= len(dep) or dep[i] != 0:
                continue
            m = item.match(line)
            if m:
                ty = m.group(4)
                if ty.startswith('[') or (ty.startswith('&') and '[' in ty):
                    A.append((p, m.group(3)))
                else:
                    S[ty] += 1
            m = fnre.match(line)
            if m and ('[' in m.group(4) or m.group(4).startswith('Vec<') or m.group(4).startswith('impl Iterator')):
                F.append((p, m.group(3)))
        cut = next((i for i, l in enumerate(lines) if l.startswith('#[cfg(test)]')), len(lines))
        body = lines[:cut]; i = 0
        while i < len(body):
            m = re.match(r'^(\s*)(pub(\([a-z]+\))?\s+)?(const\s+)?fn\s+(\w+)', body[i])
            if m:
                ind = m.group(1); j = i + 1; lit = 0
                while j < len(body) and not (body[j].startswith(ind + '}') and body[j].strip() == '}'):
                    if re.search(r'=>\s*(Some\()?(-?\d+|"[^"]*"|true|false)\)?\s*,?\s*(//.*)?$', body[j]):
                        lit += 1
                    j += 1
                if lit >= 5:
                    M.append((p[len(ROOT) + 1:], m.group(5), lit))
                i = j
            i += 1
print(f"A file-level array tables: {len(A)}")
print(f"S file-level non-array consts/statics: {sum(S.values())} {dict(S)}")
print(f"F zero-arg collection fns: {len(F)}")
print(f"M match-literal fns (>=5 literal arms): {len(M)}, arms {sum(x[2] for x in M)}")
for row in sorted(M):
    print("   ", row)
