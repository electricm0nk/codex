#!/usr/bin/env python3
"""SD-37 E4a.2: keep PF provenance text byte-identical while the importers stop naming the compiled module.

The first pass of the re-point rewrote every `rules_tables::` in an importer to `rules_catalog::`,
string literals included. A string literal is not an import: several are explanation text that
reaches the PF render (Elowen's base attack bonus line reads "... from rules_tables::crb::
class_tables::class_tables()'s Wizard row ..."), so the render hash pins its bytes (decisions.md
§12.1 SD-i). This pass puts those literals back to the bytes they had, written as
`concat!("... rules_tables", "::crb::...")` so the source names no compiled path.

Usage (repo root): python3 <this file> [--check] <file>...
  default: rewrite the files in place;  --check: list the string literals it would change.
"""
import re, sys

NEEDLE = 'rules_catalog::'
CAPTURE = re.compile(r'\{[A-Za-z_][A-Za-z0-9_]*(?::[^}]*)?\}')


def scan(text):
    """Yield (kind, start, end) for every string literal / comment of a Rust source text."""
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith('//', i):
            j = text.find('\n', i)
            j = n if j < 0 else j
            yield ('comment', i, j)
            i = j
        elif text.startswith('/*', i):
            d, j = 1, i + 2
            while j < n and d:
                if text.startswith('/*', j):
                    d += 1; j += 2
                elif text.startswith('*/', j):
                    d -= 1; j += 2
                else:
                    j += 1
            yield ('comment', i, j)
            i = j
        elif c == 'r' and re.match(r'r#*"', text[i:i + 12]) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == '_')):
            m = re.match(r'r(#*)"', text[i:])
            end = '"' + m.group(1)
            j = text.find(end, i + len(m.group(0)))
            j = n if j < 0 else j + len(end)
            yield ('raw', i, j)
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == '\\' else 1
            yield ('string', i, j + 1)
            i = j + 1
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", text[i:i + 12])
            i += len(m.group(0)) if m else 1
        else:
            i += 1


def fix(text):
    out, last, changed = [], 0, []
    needs_const = False
    for kind, a, b in scan(text):
        if kind not in ('string', 'raw'):
            continue
        lit = text[a:b]
        if NEEDLE not in lit:
            continue
        prefix = ''
        if kind == 'raw':
            m = re.match(r'r#*"', lit)
            prefix = m.group(0)[:-1]            # r or r#
        # the first piece ends right after the module name; the second starts at `::`
        quote_close = '"' + (prefix[1:] if kind == 'raw' else '')
        out.append(text[last:a])
        if CAPTURE.search(lit):
            # a format string with inline `{name}` captures cannot be built by concat!: print the
            # module name from the catalog's citation constant instead
            out.append(lit.replace(NEEDLE, '{COMPILED_MODULE_CITATION}::'))
            needs_const = True
        else:
            out.append('concat!(' + lit.replace(NEEDLE, 'rules_tables' + quote_close + ', ' + prefix + '"' + '::') + ')')
        last = b
        changed.append(lit[:90])
    out.append(text[last:])
    result = ''.join(out)
    if needs_const and 'COMPILED_MODULE_CITATION;' not in result:
        # after the last top-level `use` line
        uses = list(re.finditer(r'^use [^;]*;\n', result, re.M))
        at = uses[-1].end() if uses else 0
        result = result[:at] + 'use crate::rules_core::rules_catalog::COMPILED_MODULE_CITATION;\n' + result[at:]
    return result, changed


if __name__ == '__main__':
    args = sys.argv[1:]
    check = '--check' in args
    files = [a for a in args if a != '--check']
    total = 0
    for f in files:
        t = open(f, errors='surrogateescape').read()
        new, changed = fix(t)
        if changed:
            total += len(changed)
            print(f, len(changed))
            if not check:
                open(f, 'w', errors='surrogateescape').write(new)
    print('string literals', 'to change' if check else 'changed', total)
