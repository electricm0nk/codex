#!/usr/bin/env node
// Builds a residue-free runtime mirror of data/corpus/ for the desktop
// bundle -- run by `npm run build` (apps/desktop/package.json, before
// `vite build`) and by `scripts/verify.sh`'s `corpus-bundle` stage.
//
// Deliberately lives under repo-root `scripts/`, NOT `apps/desktop/`: this
// file's own source text necessarily names the PCGen token vocabulary it
// strips (`raw_tokens`, `BONUS:`, `PRE[A-Z]+:`, ...), and `apps/desktop/**`
// is a `LIVE_ROOT` for `scripts/pcgen_residue_gate.py` with zero carve-outs
// (`decisions.md` §11, operator ruling AT-35-E6-002 -- "no live path is
// exempt"). `scripts/` is tooling, not a live root, so this file can hold
// that vocabulary in service of REMOVING it from what ships without itself
// tripping the gate it feeds. Verified: placing an earlier draft of this
// generator at `apps/desktop/scripts/gen-corpus-bundle.mjs` made
// `pcgen_residue_gate.py --check --closure` fail (`apps/desktop files=1
// hits=4`) purely from its own pattern-literal array; moving it here brought
// the gate back to `live_files=0 live_hits=0 verdict=PASS`.
//
// SD-36 Epic A found commit 217f712bab (tranche/16) bundling the raw,
// git-tracked `data/corpus/` tree wholesale into the Tauri installer so
// packaged builds have a corpus root (fixing an empty race roster in
// off-checkout builds). But `data/corpus/**/*.json` carries ingest-time
// PCGen residue no live consumer reads: `data.raw_tokens` / `raw_bonus_chains`
// arrays, a trailing `|`-delimited PCGen token clause some records'
// `description` field never had stripped, and free-text provenance fields
// that can themselves quote a token name or token-syntax substring. Shipping
// any of that verbatim puts PCGen token text on a user's disk -- exactly what
// `decisions.md` §11 / the residue gate's ruling B17 forbids, and
// `epic-breakdown.md` A2 requires the gate at 0/0.
//
// This does NOT touch the git-tracked `data/corpus/` (the ingest pipeline's
// own source of truth). It mirrors ONLY the `data/corpus/<book>/<kind>/`
// directories `codex`'s live corpus loaders actually read at runtime
// (grep-verified against `src/rules_core/corpus_loader.rs`, `race_resolver.rs`,
// `trait_pool.rs` -- no other kind directory is read by any live/packaged code
// path today), trims each record to the fields that loader actually reads,
// and -- as a defense-in-depth net over every string value that survives the
// trim -- strips every occurrence of the residue gate's own pattern
// vocabulary (`scripts/pcgen_residue_gate.py`'s `DATA_PATTERNS`, duplicated
// below because this script must run on every OS the release workflow
// builds on -- linux, macOS, windows -- and only Node, not Python, is
// guaranteed on all three; see `.github/workflows/publish-tester-release.yml`).
// A shipped file this script produces cannot carry a pattern hit: either the
// field was dropped, or its string content had the pattern text removed.
//
//     _settled/            kept, sanitized (already residue-free content;
//                          `settled_corpus::read_*_bundle` deserializes it
//                          fully via serde, so structure is untouched)
//     equipment/            `{}` -- `load_equipment_corpus` never opens these
//                          files; it derives the record key from the PATH
//                          and reads content from the book's
//                          `_settled/equipment.json` bundle. Only the file's
//                          on-disk PRESENCE (for key enumeration) matters.
//     race/, race_trait/   trimmed CorpusRecordV1 envelope -- `data` -> {}
//                          (ignored), `source` kept (`path`/`line` feed
//                          `lst_citation`), `license`/`pi_field`/`pi_marker`
//                          kept (race_trait's `description_redacted` flag
//                          reads them) -- sanitized.
//     spell/                `{"data": {"key", "school"}}` only -- exactly the
//                          two fields `load_spell_corpus`/`spell_record_from_json`
//                          read; `source` is never read for spell.
//     trait_generic/         `{"data": {"key","name","description"},
//                          "source": {"path","line"}}` -- exactly what
//                          `read_trait_record`/`converted_race_trait_pool`
//                          read. `description`'s trailing `|`-clause, if any,
//                          is cut at the first `|`.
//
// No other kind directory (class, feat, domain, ability, template, power,
// skill, class_feature, companion, ...) is bundled: no CORE game-mechanics
// loader (corpus_loader.rs, race_resolver.rs, trait_pool.rs) reads them, so
// shipping them by default would be pure, avoidable residue. One exception,
// scoped deliberately rather than overlooked: `apps/desktop/src-tauri/src/
// reference_library_catalog.rs` is a registered (but not yet frontend-
// invoked) Tauri command that reads twelve of these kinds, including
// `trait_generic` (bundled here) and eleven that are not
// (`REFERENCE_LIBRARY_KIND_DIRS`). That module resolves its own corpus root
// through `codex_repo_root()` and refuses (loud error, not a silent empty
// catalog) any of the eleven un-bundled kinds when running against a
// packaged build's resource directory -- see its `PACKAGED_BUNDLE_KIND_DIRS`
// and `reference_library_entries_for_root`, kept in sync with this file's
// own `KIND_DIRS` by its own test. If that command is ever wired into the
// UI for one of the eleven, either widen `KIND_DIRS` here (with a residue
// audit for the newly-included record shapes) or keep it a source-checkout-
// only feature; do not just delete the guard.
//
// Deterministic and idempotent: book/kind/file iteration is sorted, JSON is
// serialized with stable (no whitespace) formatting, and stale output is
// deleted before regenerating so a removed upstream record never survives as
// a stale bundle file.
//
// Run from the repo root whenever data/corpus/ changes and the bundle needs
// regenerating:
//
//     node scripts/gen-corpus-bundle.mjs
//
// Verify with `python3 scripts/pcgen_residue_gate.py --check --closure`
// afterward (expects `live_files=0 live_hits=0 verdict=PASS`), or run
// `scripts/verify.sh --only corpus-bundle`.

import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const SRC = join(REPO_ROOT, 'data/corpus');
const DST = join(REPO_ROOT, 'apps/desktop/src-tauri/resources/corpus_bundle');
const KIND_DIRS = ['_settled', 'equipment', 'race', 'race_trait', 'spell', 'trait_generic'];

// `scripts/pcgen_residue_gate.py`'s `DATA_PATTERNS` (IDENTIFIER_PATTERNS +
// TOKEN_SYNTAX_PATTERNS + JSON_KEY_PATTERNS), re-expressed as JS RegExp. Kept
// in sync BY HAND with the Python source of truth -- there is no cross-language
// import here, so `scripts/verify.sh`'s `corpus-bundle` stage re-derives
// `DATA_PATTERNS`' vocabulary from the Python module directly and fails if a
// term is missing from this file's source text.
const RESIDUE_PATTERNS = [
  /\braw_tokens\b/g,
  /\braw_bonus_chains\b/g,
  /\bPcgenFormulaEvaluator\b/g,
  /\brender_pcgen_desc\b/g,
  /\bbonus_stack_reader\b/g,
  /\bpre_tokens\b/g,
  /\blst_file\b/g,
  /\bBONUS:/g,
  /\bDEFINE:/g,
  /\bPRE[A-Z]+:/g,
  /\bSAB:/g,
  /\bDESC:/g,
  /%CHOICE/g,
  /%LIST/g,
  /\bTYPE=/g,
  /"raw_tokens"\s*:/g,
  /"raw_bonus_chains"\s*:/g,
];

function cleanDescription(desc) {
  if (typeof desc !== 'string') return desc;
  const idx = desc.indexOf('|');
  return idx === -1 ? desc : desc.slice(0, idx);
}

/** Defense-in-depth: remove every residue-gate pattern match from every
 * surviving string leaf, recursively. Never drops a key or changes types, so
 * required struct fields keep deserializing; only string CONTENT that would
 * trip the gate is cut out of that string. */
function sanitize(value) {
  if (Array.isArray(value)) return value.map(sanitize);
  if (value !== null && typeof value === 'object') {
    const out = {};
    for (const [k, v] of Object.entries(value)) out[k] = sanitize(v);
    return out;
  }
  if (typeof value === 'string') {
    let s = value;
    for (const rx of RESIDUE_PATTERNS) s = s.replace(rx, '');
    return s;
  }
  return value;
}

function pick(obj, keys) {
  const out = {};
  if (obj && typeof obj === 'object') {
    for (const k of keys) if (k in obj) out[k] = obj[k];
  }
  return out;
}

function transform(kind, doc) {
  if (kind === '_settled') return sanitize(doc);
  if (kind === 'equipment') return {};
  if (kind === 'spell') {
    const data = doc && typeof doc.data === 'object' && doc.data !== null ? doc.data : {};
    return sanitize({ data: pick(data, ['key', 'school']) });
  }
  if (kind === 'trait_generic') {
    const data = doc && typeof doc.data === 'object' && doc.data !== null ? doc.data : {};
    const newData = pick(data, ['key', 'name', 'description']);
    if ('description' in newData) newData.description = cleanDescription(newData.description);
    const source = doc && typeof doc.source === 'object' && doc.source !== null ? doc.source : {};
    const newSource = pick(source, ['path', 'line']);
    return sanitize({ data: newData, source: newSource });
  }
  if (kind === 'race' || kind === 'race_trait') {
    const trimmed = {
      population: doc ? doc.population : undefined,
      completeness: doc ? doc.completeness : undefined,
      ingested_at: doc ? doc.ingested_at : undefined,
      data: {},
      source: doc ? doc.source : undefined,
      license: doc ? doc.license : undefined,
      pi_field: doc ? doc.pi_field : undefined,
      pi_marker: doc ? doc.pi_marker : undefined,
    };
    return sanitize(trimmed);
  }
  throw new Error(`unhandled kind ${kind}`);
}

function walk(dir) {
  const out = [];
  const stack = [dir];
  while (stack.length) {
    const current = stack.pop();
    const entries = readdirSync(current, { withFileTypes: true }).sort((a, b) =>
      a.name < b.name ? -1 : a.name > b.name ? 1 : 0,
    );
    for (const entry of entries) {
      const full = join(current, entry.name);
      if (entry.isDirectory()) stack.push(full);
      else out.push(full);
    }
  }
  return out;
}

function main() {
  if (existsSync(DST)) rmSync(DST, { recursive: true, force: true });
  let copied = 0;
  const books = readdirSync(SRC, { withFileTypes: true })
    .filter((e) => e.isDirectory())
    .map((e) => e.name)
    .sort();
  for (const book of books) {
    const bookDir = join(SRC, book);
    for (const kind of [...KIND_DIRS].sort()) {
      const kindSrc = join(bookDir, kind);
      if (!existsSync(kindSrc) || !statSync(kindSrc).isDirectory()) continue;
      for (const srcPath of walk(kindSrc)) {
        const name = srcPath.split(/[\\/]/).pop();
        if (name === 'LICENSE.json' || !name.endsWith('.json')) continue;
        const relDir = relative(SRC, dirname(srcPath));
        const outDir = join(DST, relDir);
        mkdirSync(outDir, { recursive: true });
        let doc;
        try {
          doc = JSON.parse(readFileSync(srcPath, 'utf8'));
        } catch {
          continue;
        }
        const cleaned = transform(kind, doc);
        writeFileSync(join(outDir, name), JSON.stringify(cleaned));
        copied += 1;
      }
    }
  }
  // `.gitkeep` is tracked so the resource path always exists on a clean
  // checkout (the rest of this directory is gitignored, generated output).
  mkdirSync(DST, { recursive: true });
  if (!existsSync(join(DST, '.gitkeep'))) writeFileSync(join(DST, '.gitkeep'), '');
  process.stderr.write(`files_copied=${copied} dst=${DST}\n`);
}

main();
