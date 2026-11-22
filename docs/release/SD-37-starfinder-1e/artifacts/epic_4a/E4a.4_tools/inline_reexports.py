#!/usr/bin/env python3
"""SD-37 E4a.4: move every item the catalog still re-exports from the compiled `rules_tables`
module into the catalog file that re-exports it, so the compiled module can be deleted.

For each catalog mirror file (src/rules_core/rules_catalog/<chain>.rs) and its compiled source
(src/rules_core/rules_tables/<chain>.rs):
  * each `pub use rt::<chain>::<Name>;` line is replaced by the compiled definition(s) of <Name>
    (its doc comment and attributes kept), every `impl` block whose self type or trait is a moved
    name, and -- to a fixed point -- every private compiled item a moved item names that the
    catalog file does not already define;
  * every `#[cfg(test)]` module of the compiled file is moved too (the compiled module's own
    tests now run against the package-backed tables), with E4a.2's path/value rewrite;
  * the `use crate::rules_core::rules_tables as rt;` line is dropped.
Paths are rewritten only in code: string literals and comments keep their bytes (provenance text
reaches the PF render, SD-i).
Run from the repo root. Prints a census of what moved.
"""
import collections, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, '..', 'E4a.2_gen_catalog')
sys.path.insert(0, GEN)
import rtparse  # noqa: E402

RT = 'src/rules_core/rules_tables'
CAT = 'src/rules_core/rules_catalog'
SKIP_FILES = {'equivalence_tests.rs', 'override_tests.rs', 'control_tests.rs', 'golden_tests.rs'}
TOK = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
RE_EXPORT = re.compile(r'^(?P<vis>pub(?:\(crate\))?) use rt::(?P<path>[A-Za-z0-9_:]+);\s*$')


def items_with_lead(path):
    """rtparse.top_items plus each item's full source span: the comment lines directly above it
    (doc comments), its attributes and its text."""
    raw = open(path).read()
    items = rtparse.top_items(path)
    pos = 0
    for it in items:
        start = raw.find(it['text'], pos)
        assert start >= 0, (path, it['name'])
        head = start
        attrs = it['attrs'].strip()
        if attrs:
            first = attrs.split(']')[0]
            a = raw.rfind(first, pos, start)
            if a >= 0:
                head = a
        line_start = raw.rfind('\n', 0, head) + 1
        lead_start = line_start
        while lead_start > pos:
            prev_end = lead_start - 1
            prev_start = raw.rfind('\n', 0, prev_end) + 1
            line = raw[prev_start:prev_end].strip()
            if line.startswith('//') and not line.startswith('//!'):
                lead_start = prev_start
            else:
                break
        end = start + len(it['text'])
        it['full'] = raw[max(lead_start, pos):end]
        pos = end
    return raw, items


def code_only_rewrite(text, wrap_statics=None):
    """crate::rules_core::rules_tables:: -> rules_catalog::, in code only (not strings/comments)."""
    code = rtparse.strip_noncode(text)
    out = []
    last = 0
    for m in re.finditer(r'(crate::rules_core::)?\brules_tables::', code):
        out.append(text[last:m.start()])
        out.append((m.group(1) or '') + 'rules_catalog::')
        last = m.end()
    out.append(text[last:])
    return ''.join(out)


def impl_names(code):
    """(self type base name, trait base name or None) of an `impl` item."""
    t = code.strip()
    assert t.startswith('impl')
    i = 4
    t2 = t[i:].lstrip()
    if t2.startswith('<'):
        d = 0
        for k, ch in enumerate(t2):
            if ch == '<':
                d += 1
            elif ch == '>':
                d -= 1
                if d == 0:
                    t2 = t2[k + 1:]
                    break
    head = t2[:t2.index('{')]
    head = re.split(r'\bwhere\b', head)[0]
    # split on top-level ' for '
    d = 0
    cut = None
    for m in re.finditer(r'[<>]|\bfor\b', head):
        s = m.group(0)
        if s == '<':
            d += 1
        elif s == '>':
            d -= 1
        elif d == 0:
            cut = m
            break
    def base(s):
        s = s.strip().lstrip('&').strip()
        s = re.sub(r"^'\w+\s+", '', s)
        s = re.sub(r'^(mut|dyn)\s+', '', s)
        s = s.split('<')[0]
        return s.split('::')[-1].strip()
    if cut:
        return base(head[cut.end():]), base(head[:cut.start()])
    return base(head), None


def chain_of(cat_path):
    rel = os.path.relpath(cat_path, CAT)[:-3]
    parts = rel.split('/')
    if parts[-1] == 'mod':
        parts = parts[:-1]
    return tuple(parts)


def compiled_path(chain):
    a = RT + '/' + '/'.join(chain) if chain else RT
    for c in ((a + '.rs', a + '/mod.rs') if chain else (RT + '/mod.rs',)):
        if os.path.exists(c):
            return c
    return None


def is_test(it):
    return 'cfg(test)' in re.sub(r'\s', '', it['attrs'])


# E4a.2's value rewrite (registered statics used by value become `&*NAME`) for moved tests.
sys.argv = [sys.argv[0]]
import analyze as A  # noqa: E402
_st, _bu, _vi = A.registry()
STATIC_NAMES = set()
for i in _st + _bu:
    parts = i.split('/')
    chain, name = tuple(parts[:-1]), parts[-1]
    if any(it['name'] == name and it['kw'] in ('const', 'static') for it in A.items(chain)):
        STATIC_NAMES.add(name)
STATIC_PAT = re.compile(r'(?<![A-Za-z0-9_])(?:(?:[A-Za-z_][A-Za-z0-9_]*::)+)?(?:' + '|'.join(sorted(STATIC_NAMES, key=len, reverse=True)) + r')(?![A-Za-z0-9_])')


def wrap_statics(text, own_statics):
    """In moved TEST code: a registered table static used as a value becomes `&*NAME`
    (a `Table` derefs to `[T]`; the compiled array was a value). Code positions only."""
    code = rtparse.strip_noncode(text)
    protect = [(m.start(), m.end()) for m in re.finditer(r'\buse\s+[^;]*;', code)]
    out = []
    last = 0
    for m in STATIC_PAT.finditer(code):
        s = m.start()
        if any(a <= s < b for a, b in protect):
            continue
        name = m.group(0).split('::')[-1]
        if '::' not in m.group(0) and name not in own_statics:
            continue
        nxt = code[m.end():].lstrip()[:1]
        prev = code[max(0, s - 2):s]
        if nxt in ('.', '[') or prev.endswith('&*') or prev.endswith('&'):
            continue
        out.append(text[last:s])
        out.append('&*' + text[s:m.end()])
        last = m.end()
    out.append(text[last:])
    return ''.join(out)


census = collections.Counter()
report = collections.defaultdict(list)


def process(cat_file):
    chain = chain_of(cat_file)
    cpath = compiled_path(chain)
    text = open(cat_file).read()
    if cpath is None:
        report['no-compiled-source'].append(cat_file)
        return
    raw, citems = items_with_lead(cpath)
    _, fitems = items_with_lead(cat_file)
    defined = {it['name'] for it in fitems if it['name'] and it['kw'] not in ('use', 'impl')}
    own_statics = {it['name'] for it in fitems if it['kw'] in ('static', 'const')}
    by_name = collections.defaultdict(list)
    impls = []
    tests = []
    for it in citems:
        if is_test(it):
            tests.append(it)
            continue
        if it['kw'] == 'impl':
            impls.append(it)
        elif it['name'] and it['kw'] != 'use':
            by_name[it['name']].append(it)

    lines = text.split('\n')
    new_lines = []
    moved = set()
    blocks = []  # (insert index, list of item texts)

    def take(name):
        out = []
        for it in by_name.get(name, []):
            out.append(it)
        return out

    for line in lines:
        m = RE_EXPORT.match(line)
        if m and m.group('path').split('::')[:-1] == list(chain):
            name = m.group('path').split('::')[-1]
            its = take(name)
            if not its:
                report['re-export-without-definition'].append(f'{cat_file}: {line.strip()}')
                new_lines.append(line)
                continue
            moved.add(name)
            for it in its:
                census[it['kw']] += 1
            new_lines.append('\x00MOVE:' + name)
            continue
        if line.strip() == 'use crate::rules_core::rules_tables as rt;':
            continue
        new_lines.append(line)

    # private helpers a moved item (or a moved test) names, to a fixed point
    need_text = '\n'.join(it['code'] for n in moved for it in by_name[n])
    need_text += '\n'.join(it['code'] for it in impls)
    need_text += '\n'.join(it['code'] for it in tests)
    extra = []
    changed = True
    tokens = set(TOK.findall(need_text))
    while changed:
        changed = False
        for name, its in by_name.items():
            if name in moved or name in defined or name not in tokens:
                continue
            moved.add(name)
            for it in its:
                extra.append(it)
                census['private:' + it['kw']] += 1
                tokens |= set(TOK.findall(it['code']))
            changed = True
    moved_impls = []
    for it in impls:
        selfty, trait = impl_names(it['code'])
        if selfty in moved or (trait and trait in moved):
            moved_impls.append(it)
            census['impl'] += 1
        elif selfty in defined:
            # an impl on a type the catalog already defines -- must not exist twice
            report['impl-on-catalog-type'].append(f'{cat_file}: impl {selfty}')
        else:
            report['impl-left-behind'].append(f'{cat_file}: impl {trait} for {selfty}')

    def render(it):
        return code_only_rewrite(it['full'].rstrip())

    out = []
    for line in new_lines:
        if line.startswith('\x00MOVE:'):
            name = line[len('\x00MOVE:'):]
            for it in by_name[name]:
                out.append(render(it))
            for it in list(moved_impls):
                selfty, trait = impl_names(it['code'])
                if selfty == name:
                    out.append(render(it))
                    moved_impls.remove(it)
        else:
            out.append(line)
    for it in extra:
        out.append(render(it))
        for imp in list(moved_impls):
            selfty, trait = impl_names(imp['code'])
            if selfty == it['name']:
                out.append(render(imp))
                moved_impls.remove(imp)
    for imp in moved_impls:
        out.append(render(imp))
    for it in tests:
        t = code_only_rewrite(it['full'].rstrip())
        t = wrap_statics(t, own_statics | {n for n in STATIC_NAMES})
        out.append(t)
        census['test-module'] += 1
        census['#[test]'] += len(re.findall(r'#\[test\]', it['code']))
    new = '\n'.join(out)
    if not new.endswith('\n'):
        new += '\n'
    open(cat_file, 'w').write(new)


def main():
    files = []
    for d, _, fs in os.walk(CAT):
        for f in fs:
            if f.endswith('.rs') and f not in SKIP_FILES:
                files.append(os.path.join(d, f))
    for f in sorted(files):
        process(f)
    print('files', len(files))
    print('census', dict(sorted(census.items())))
    for k, v in report.items():
        print(k, len(v))
        for x in v[:60]:
            print('   ', x)


if __name__ == '__main__':
    main()
