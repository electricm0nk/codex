//! Converter side: settle one stored source `description` into a
//! [`DescTemplate`](codex::rules_core::desc_template::DescTemplate) once, at ingest.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 16 (`decisions.md` §11). This is the authoring-time half of
//! the split described in `rules_core::desc_template`'s module doc: the scan that decides what
//! each piece of the stored description *means* runs here, against the source text, and the live
//! side is left with an ordered op list and a `name -> i64` environment.
//!
//! **The scan below is [`super::pcgen_desc::render_pcgen_desc_with_values`]'s own scan, branch
//! for branch, with the value-dependent half lifted out.** Every branch that decides something
//! from the source text alone (an escape, a keyword substitution, a bare marker, percentile dice
//! notation, which argument a slot names, whether the row supplied that argument at all) is
//! evaluated here and frozen into an op. Every branch that decides something from the accumulated
//! output (whether a slot resolved, whether a per-cent sign has a subject, the sign swallowing,
//! the whitespace collapse) stays live, because it must.
//!
//! That correspondence is **proved over the real corpus, not asserted**: [`tests`] below renders
//! every described `class_feature` record in `data/corpus/` both ways under six value
//! environments, and every `description` of every other record kind under two, comparing `text`
//! and `dropped_args` field for field. A branch this module reads differently from the renderer
//! it replaces fails there, on real records, rather than on a fixture.

use codex::rules_core::desc_template::{DescArgument, DescOp, DescTemplate};

use super::pcgen_desc::{PCGEN_ENTITIES, is_percentile_dice_notation, split_prose_and_args};

/// Settles one raw stored `description` into the template the live side renders.
///
/// An empty or argument-free description settles to a template whose ops are a single piece of
/// literal text (or to an empty template, when the description is empty) — which renders back to
/// the same string, by construction.
pub fn template_for(raw: &str) -> DescTemplate {
    let (prose, args) = split_prose_and_args(raw);
    let chars: Vec<char> = prose.chars().collect();

    let mut ops: Vec<DescOp> = Vec::new();
    let mut literal = String::new();
    let mut i = 0usize;

    while i < chars.len() {
        // The escape is read first, exactly as the renderer does, so a literal per-cent sign is
        // never mistaken for a slot. The one exception is the renderer's own: an escaped digit
        // IS a slot when the row supplied that argument, and that is decidable here.
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            let escaped_arg = chars
                .get(i + 2)
                .and_then(|c| c.to_digit(10))
                .filter(|digit| *digit >= 1)
                .and_then(|digit| args.get(digit as usize - 1));
            if let Some(arg) = escaped_arg {
                flush(&mut literal, &mut ops);
                ops.push(slot(arg));
                i += 3;
                continue;
            }
            flush(&mut literal, &mut ops);
            ops.push(DescOp::PercentOrDrop);
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            flush(&mut literal, &mut ops);
            ops.push(match args.get(digit as usize - 1) {
                Some(arg) => slot(arg),
                // The prose references an argument the row never supplied. The renderer drops it
                // and — deliberately — reports nothing, because there is no argument text to
                // name. Reproduced, not tidied.
                None => DescOp::MissingArg,
            });
            i += 2;
            continue;
        }
        // A keyword substitution names a chargen-time player selection this program has no slot
        // for at all. There is no resolution to attempt, so it is settled as an unconditional
        // drop rather than as a slot that will always miss.
        if chars[i] == '%' && chars.get(i + 1).is_some_and(char::is_ascii_uppercase) {
            let start = i + 1;
            let mut end = start;
            while chars.get(end).is_some_and(char::is_ascii_uppercase) {
                end += 1;
            }
            flush(&mut literal, &mut ops);
            ops.push(DescOp::Drop { report: chars[start..end].iter().collect() });
            i = end;
            continue;
        }
        if chars[i] == '%' {
            // Both exemptions are lookbehinds over the SOURCE prose, never over the rendered
            // output, so both are decidable here: percentile dice notation ("d%", "5d%") and a
            // per-cent sign already attached to a literal number ("75% chance").
            let preceded_by_digit = i > 0 && chars[i - 1].is_ascii_digit();
            if is_percentile_dice_notation(&chars, i) || preceded_by_digit {
                literal.push('%');
                i += 1;
                continue;
            }
            flush(&mut literal, &mut ops);
            ops.push(DescOp::Drop { report: "%".to_string() });
            i += 1;
            continue;
        }
        literal.push(chars[i]);
        i += 1;
    }
    flush(&mut literal, &mut ops);

    // The escape decoding runs over the FINISHED text, so a substitution can in principle be
    // formed across a dropped slot. Every escape begins with `&` and a filled slot only ever
    // contributes digits, so "the prose contains no `&`" is a complete proof that none can be
    // formed; otherwise all five pairs are carried, and decoding one that is absent is a no-op.
    let escapes = if prose.contains('&') {
        PCGEN_ENTITIES.iter().map(|(from, to)| ((*from).to_string(), (*to).to_string())).collect()
    } else {
        Vec::new()
    };

    DescTemplate {
        ops,
        args: args.iter().map(|a| a.trim().to_string()).collect(),
        escapes,
    }
}

fn flush(literal: &mut String, ops: &mut Vec<DescOp>) {
    if !literal.is_empty() {
        ops.push(DescOp::Text { text: std::mem::take(literal) });
    }
}

/// Settles one argument text into how the live side must get its number — the same three shapes
/// `pcgen_desc::resolve_desc_argument` tries, in the same order, with the first two collapsed
/// into the settled form each one becomes.
fn slot(arg: &str) -> DescOp {
    let trimmed = arg.trim();
    let settled = if let Ok(value) = trimmed.parse::<i64>() {
        DescArgument::Literal { value }
    } else {
        // Shape 3 is the FALLBACK, never the first reading: an argument text that is itself a
        // bound name wins, which is why the offset decomposition is carried beside the name
        // rather than instead of it.
        let offset = trimmed
            .rfind(['+', '-'])
            .filter(|index| *index > 0)
            .and_then(|index| {
                let (name, offset) = trimmed.split_at(index);
                offset.parse::<i64>().ok().map(|delta| (name.trim().to_string(), delta))
            });
        DescArgument::Named { name: trimmed.to_string(), offset }
    };
    DescOp::Arg { arg: settled, report: trimmed.to_string() }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::{Path, PathBuf};

    use serde_json::Value;

    use super::super::pcgen_desc::{
        PcgenDisplayValues, desc_token_arguments, render_pcgen_desc_with_values,
    };
    use super::*;

    fn repo_root() -> PathBuf {
        crate::repo_root()
    }

    /// Every name a caller could bind for one description: each argument text, and — for the
    /// offset shape — the base name it decomposes to.
    fn bindable_names(raw: &str) -> Vec<String> {
        let mut names: BTreeSet<String> = BTreeSet::new();
        for arg in desc_token_arguments(raw) {
            let trimmed = arg.trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(index) = trimmed.rfind(['+', '-']).filter(|i| *i > 0) {
                let (base, offset) = trimmed.split_at(index);
                if offset.parse::<i64>().is_ok() {
                    names.insert(base.trim().to_string());
                }
            }
            names.insert(trimmed);
        }
        names.into_iter().collect()
    }

    /// The value environments both renderers are compared under. `which` selects a policy rather
    /// than a fixed table, so every record is probed with bindings derived from its OWN
    /// arguments: nothing bound, everything bound, only every other one bound (which is what
    /// exercises the sign swallowing and the whitespace collapse against surviving text), and
    /// only the offset base names bound.
    fn probe(raw: &str, which: usize) -> BTreeMap<String, i64> {
        let names = bindable_names(raw);
        let mut env = BTreeMap::new();
        for (index, name) in names.iter().enumerate() {
            let bind = match which {
                0 => false,
                1 => true,
                2 => index % 2 == 0,
                3 => index % 2 == 1,
                4 => true,
                _ => !name.contains(['+', '-']),
            };
            if !bind {
                continue;
            }
            let value = match which {
                4 => -7,
                5 => 0,
                _ => i64::try_from(index).unwrap_or(0) + 3,
            };
            env.insert(name.clone(), value);
        }
        env
    }

    fn display_values(env: &BTreeMap<String, i64>) -> PcgenDisplayValues {
        let mut values = PcgenDisplayValues::new();
        for (name, value) in env {
            values.set(name, *value);
        }
        values
    }

    /// Renders `raw` both ways under `probes` environments and returns every disagreement as a
    /// human-readable line.
    fn disagreements(raw: &str, probes: usize) -> Vec<String> {
        let template = template_for(raw);
        let mut out = Vec::new();
        for which in 0..probes {
            let env = probe(raw, which);
            let ours = template.render(&env);
            let theirs = render_pcgen_desc_with_values(raw, &display_values(&env));
            if ours.text != theirs.text || ours.dropped_args != theirs.dropped_args {
                out.push(format!(
                    "probe={which} raw={raw:?}\n  settled : text={:?} dropped={:?}\n  renderer: \
                     text={:?} dropped={:?}",
                    ours.text, ours.dropped_args, theirs.text, theirs.dropped_args
                ));
            }
        }
        out
    }

    fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                walk_json_files(&path, out);
            } else if path.extension().is_some_and(|e| e == "json") {
                out.push(path);
            }
        }
    }

    fn book_dirs() -> Vec<PathBuf> {
        let Ok(books) = std::fs::read_dir(repo_root().join("data/corpus")) else {
            return Vec::new();
        };
        let mut dirs: Vec<_> = books.flatten().collect();
        dirs.sort_by_key(|e| e.file_name());
        dirs.into_iter().map(|e| e.path()).collect()
    }

    #[test]
    fn every_described_class_feature_record_settles_to_the_same_prose_the_renderer_produced() {
        let mut compared = 0usize;
        let mut with_a_slot = 0usize;
        let mut failures: Vec<String> = Vec::new();
        for book in book_dirs() {
            let dir = book.join("class_feature");
            if !dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let Some(raw) = doc["data"]["description"].as_str() else { continue };
                if raw.trim().is_empty() {
                    continue;
                }
                compared += 1;
                if !desc_token_arguments(raw).is_empty() {
                    with_a_slot += 1;
                }
                failures.extend(disagreements(raw, 6));
            }
        }
        assert!(
            compared >= 3_000,
            "the described class_feature population collapsed: compared={compared} -- this test \
             is only a proof while it reads the real corpus"
        );
        assert!(
            with_a_slot >= 200,
            "no value slots exercised: with_a_slot={with_a_slot} of {compared}"
        );
        assert!(
            failures.is_empty(),
            "{} of {compared} described class_feature records render differently once \
             settled:\n{}",
            failures.len(),
            failures.iter().take(20).cloned().collect::<Vec<_>>().join("\n")
        );
    }

    #[test]
    fn every_other_record_kinds_description_settles_the_same_way_too() {
        let mut compared = 0usize;
        let mut failures: Vec<String> = Vec::new();
        let mut kinds: BTreeSet<String> = BTreeSet::new();
        for book in book_dirs() {
            let Ok(entries) = std::fs::read_dir(&book) else { continue };
            let mut kind_dirs: Vec<_> = entries.flatten().map(|e| e.path()).collect();
            kind_dirs.sort();
            for kind_dir in kind_dirs {
                if !kind_dir.is_dir() {
                    continue;
                }
                let name = kind_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                if name.starts_with('_') || name == "class_feature" {
                    continue;
                }
                let mut files = Vec::new();
                walk_json_files(&kind_dir, &mut files);
                for file in files {
                    let Ok(text) = std::fs::read_to_string(&file) else { continue };
                    let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                    let Some(raw) = doc["data"]["description"].as_str() else { continue };
                    if raw.trim().is_empty() {
                        continue;
                    }
                    kinds.insert(name.clone());
                    compared += 1;
                    failures.extend(disagreements(raw, 2));
                }
            }
        }
        assert!(
            compared >= 5_000 && kinds.len() >= 5,
            "the wider description population collapsed: compared={compared} kinds={kinds:?}"
        );
        assert!(
            failures.is_empty(),
            "{} of {compared} descriptions across {} other record kinds render differently once \
             settled:\n{}",
            failures.len(),
            kinds.len(),
            failures.iter().take(20).cloned().collect::<Vec<_>>().join("\n")
        );
    }

    #[test]
    fn the_shapes_the_renderer_documents_settle_the_way_it_describes_them() {
        // Each of these is a shape the renderer's own module doc names, kept here as a readable
        // statement of the correspondence the two corpus sweeps above prove exhaustively.
        for raw in [
            "a +%1 luck bonus|MyVar",
            "Three",
            "DC %%1 Fortitude save|MyDC",
            "a %1%% chance|MyChance",
            "20%% of the time",
            "roll 5d% miles",
            "gains %CHOICE as a bonus feat",
            "75% chance of failure",
            "one&nl;two|MyVar %1",
            "up to +5|DisruptorShieldBonus",
            "%1 rounds|MyVar-1",
            "%1 and %2|4|MyOther",
            "%3 with only one argument|MyVar",
        ] {
            let failures = disagreements(raw, 6);
            assert!(failures.is_empty(), "{}", failures.join("\n"));
        }
    }
}
