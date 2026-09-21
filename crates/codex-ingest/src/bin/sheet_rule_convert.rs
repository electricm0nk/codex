//! `sheet_rule_convert` -- the ingest-time converter (SD-35 AT-35-E2-001).
//!
//! ```text
//! cargo run --locked --bin sheet_rule_convert            # regenerate data/sheet_rules/ whole
//! cargo run --locked --bin sheet_rule_convert -- --check # verify the package is fresh and clean
//! cargo run --locked --bin sheet_rule_convert -- --one <unit id>   # print one unit's conversion
//! cargo run --locked --bin sheet_rule_convert -- --dump <dir>      # write a fresh conversion to
//!                                                                   # a SCRATCH dir, never data/sheet_rules
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


use codex_ingest::pcgen_import::sheet_rule;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let repo = codex_ingest::repo_root();
    let out_dir = repo.join("data/sheet_rules");
    if let Some(pos) = args.iter().position(|a| a == "--one") {
        let id = args.get(pos + 1).cloned().unwrap_or_default();
        match sheet_rule::convert_one(&repo, &id) {
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
    let check = args.iter().any(|a| a == "--check");
    let dump_dir = args.iter().position(|a| a == "--dump").and_then(|pos| args.get(pos + 1)).map(std::path::PathBuf::from);
    if let Some(d) = &dump_dir {
        // Never the tracked package: `--dump` exists so a structural diff can run a full fresh
        // conversion without a corpus-wide write to `data/sheet_rules` (F1 adversarial finding 4).
        if d == &out_dir {
            eprintln!("sheet_rule_convert: --dump must not target data/sheet_rules; pass a scratch directory");
            std::process::exit(2);
        }
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
