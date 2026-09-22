//! `sheet_rule_convert` -- the ingest-time converter (SD-35 AT-35-E2-001).
//!
//! ```text
//! cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --write # regenerate data/sheet_rules/ whole
//! cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --check # verify the package is fresh and clean
//! cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --one <unit id>   # print one unit's conversion
//! cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --dump <dir>      # write a fresh conversion to
//!                                                                                   # a SCRATCH dir, never data/sheet_rules
//! ```
//!
//! Prints `records=<n> converted=<n> refused=<n>` (summing to the population) on every run, and
//! the refusal counts per token type. `--check` exits non-zero when the on-disk package differs
//! from a fresh conversion, carries a source-format literal, or references a variable with no
//! table (`blockers.md` B10).
//!
//! `--dump <dir>` (SD-36 Epic F1 adversarial finding 4, `epic-f-class-completion.md` §3.5): writes
//! the SAME rendered package `--check` would compare against, but to an arbitrary directory the
//! caller names -- never `data/sheet_rules` -- so a structural diff
//! (`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py`) can compare a
//! full fresh run against the on-disk baseline without a corpus-wide write to the tracked package.
//!
//! `--write` (SD-36 Epic F1 re-check round 1, finding 3): the corpus-wide in-place rewrite of the
//! tracked `data/sheet_rules` package is gated behind this EXPLICIT flag, never a fall-through
//! default. Before this flag existed, any unrecognized argument (a typo, a mistyped `--dump`
//! operand) silently fell through to the same branch and rewrote the tracked package whole --
//! exactly the corpus-wide write `--dump` exists to let callers avoid. A missing `--dump`
//! operand, or any argument this parser does not recognize, is now a hard error (exit 2), never a
//! silent fall-through.


use codex_ingest::pcgen_import::sheet_rule;

/// Explicit argument parse -- no unrecognized argument and no operand-less flag may fall through
/// to any other branch, least of all the corpus-wide `--write` rewrite of `data/sheet_rules`
/// (SD-36 Epic F1 re-check round 1, finding 3).
struct Args {
    check: bool,
    write: bool,
    one: Option<String>,
    dump_dir: Option<std::path::PathBuf>,
}

fn parse_args(raw: &[String]) -> Result<Args, String> {
    let mut check = false;
    let mut write = false;
    let mut one = None;
    let mut dump_dir = None;
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--check" => check = true,
            "--write" => write = true,
            "--one" => {
                let id = raw.get(i + 1).ok_or_else(|| "--one requires an operand (a unit id)".to_string())?;
                one = Some(id.clone());
                i += 1;
            }
            "--dump" => {
                let dir = raw.get(i + 1).ok_or_else(|| "--dump requires an operand (a scratch directory)".to_string())?;
                dump_dir = Some(std::path::PathBuf::from(dir));
                i += 1;
            }
            other => return Err(format!("unrecognized argument: {other}")),
        }
        i += 1;
    }
    Ok(Args { check, write, one, dump_dir })
}

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&raw_args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("sheet_rule_convert: {e}");
            eprintln!("usage: sheet_rule_convert [--write | --check | --one <unit id> | --dump <dir>]");
            std::process::exit(2);
        }
    };
    let repo = codex_ingest::repo_root();
    let out_dir = repo.join("data/sheet_rules");
    if let Some(id) = &args.one {
        match sheet_rule::convert_one(&repo, id) {
            Ok((c, rows)) => {
                for r in rows {
                    println!("row {r}");
                }
                println!("refusals={:?}", c.refusals);
                println!("defects={:?}", c.defects);
                println!("{}", serde_json::to_string_pretty(&c.rules).unwrap());
                for (id, name, contrib) in &c.var_contribs {
                    println!("contrib {id} ({name}) {}", serde_json::to_string(contrib).unwrap());
                }
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(2);
            }
        }
        return;
    }
    let check = args.check;
    let dump_dir = args.dump_dir.clone();
    if let Some(d) = &dump_dir {
        // Never the tracked package: `--dump` exists so a structural diff can run a full fresh
        // conversion without a corpus-wide write to `data/sheet_rules` (F1 adversarial finding 4).
        if d == &out_dir {
            eprintln!("sheet_rule_convert: --dump must not target data/sheet_rules; pass a scratch directory");
            std::process::exit(2);
        }
    }
    if !check && dump_dir.is_none() && !args.write {
        eprintln!("sheet_rule_convert: the in-place rewrite of data/sheet_rules requires --write; pass --check, --dump <dir>, --one <id>, or --write");
        std::process::exit(2);
    }
    let started = std::time::Instant::now();
    let (run, _index) = match sheet_rule::convert_repo(&repo) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("sheet_rule_convert: {e}");
            std::process::exit(2);
        }
    };
    let r = &run.report;
    if let Some(d) = &dump_dir {
        let rendered = sheet_rule::render(&run);
        if let Err(e) = sheet_rule::write_output(d, &rendered) {
            eprintln!("sheet_rule_convert: writing {}: {e}", d.display());
            std::process::exit(2);
        }
        // A real (non-dump) run also rewrites the tracked `scripts/oracle_harness/var_names.json`
        // -- emit the same file into the scratch dir so a structural diff can cover it too (F1
        // re-check round 1, finding 3).
        if let Err(e) = sheet_rule::write_var_names_to(d, &run) {
            eprintln!("sheet_rule_convert: writing {}/var_names.json: {e}", d.display());
            std::process::exit(2);
        }
        println!("records={} converted={} refused={} rules={} var_tables={} dumped={} -> {} ({:.1}s)", r.records, r.converted, r.refused, r.rules_written, r.var_tables, rendered.len(), d.display(), started.elapsed().as_secs_f64());
        return;
    }
    if check {
        match sheet_rule::check(&out_dir, &run) {
            Ok(()) => {
                println!("records={} converted={} refused={} rules={} var_tables={} verdict=PASS ({:.1}s)", r.records, r.converted, r.refused, r.rules_written, r.var_tables, started.elapsed().as_secs_f64());
            }
            Err(problems) => {
                for p in &problems {
                    eprintln!("  {p}");
                }
                println!("records={} converted={} refused={} rules={} var_tables={} verdict=FAIL problems={}", r.records, r.converted, r.refused, r.rules_written, r.var_tables, problems.len());
                std::process::exit(1);
            }
        }
    } else {
        let rendered = sheet_rule::render(&run);
        if let Err(e) = sheet_rule::write_output(&out_dir, &rendered) {
            eprintln!("sheet_rule_convert: writing {}: {e}", out_dir.display());
            std::process::exit(2);
        }
        if let Err(e) = sheet_rule::write_var_names(&repo, &run) {
            eprintln!("sheet_rule_convert: writing var_names.json: {e}");
            std::process::exit(2);
        }
        println!("records={} converted={} refused={} rules={} var_tables={} files={} ({:.1}s)", r.records, r.converted, r.refused, r.rules_written, r.var_tables, rendered.len(), started.elapsed().as_secs_f64());
    }
    let mut by_type: Vec<(&String, &usize)> = r.refused_by_token_type.iter().collect();
    by_type.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (t, n) in by_type {
        println!("refused {n:>6}  {t}");
    }
    for (k, v) in &run.report.by_kind {
        println!("kind {k}: records={} converted={} refused={}", v.records, v.converted, v.refused);
    }
}
