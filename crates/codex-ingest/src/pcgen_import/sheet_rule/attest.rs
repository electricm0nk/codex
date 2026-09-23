//! SD-36 F1c-3, defect D4 (`epic-f-class-completion.md` §3.4 "Known-empty vs unknown"): the
//! closure-complete attestation the converter writes on every class principal rule.
//!
//! A reader that walks a class's converted closure and finds no weapon-proficiency grant cannot
//! tell "the class grants none" from "the grant was lost at conversion" -- unless the converter,
//! which saw every loss, says which. This module is where it says so.
//!
//! # The rule (one mechanism, no per-class case)
//!
//! A class's grant closure is every record reachable from the class principal through the edges
//! the held-set fixpoint walks: `Granter::Class { id: <class> }` (any level) and
//! `Granter::Rule` (transitively), plus -- for a class-selection class
//! (`Effect::TakenOnClass`) -- its base class's own closure. Gates are ignored: the attestation
//! is about what the conversion kept, not about what one character holds.
//!
//! `closure_complete` is true iff no record in that closure carries a closure defect:
//! a row in one of [`CLOSURE_DEFECT_KINDS`] attributed to it (`"<record id>: <detail>"`), or a
//! grant edge it hands out onto a target the converter did not write
//! (`grants-to-unconverted-targets`). Prose defects (literal-in-prose, editorial markers, ...)
//! are not closure defects: they degrade words, never drop a grant.

use std::collections::{BTreeMap, BTreeSet};

use codex::rules_core::sheet_rule::{Effect, Grant, Granter, RuleId, SheetRule};

/// The `_defects/<kind>.json` families that mean a reference in a record was not converted into
/// the edge or fact the oracle states.
pub const CLOSURE_DEFECT_KINDS: [&str; 5] =
    ["unresolved-references", "ambiguous-parent-category-target", "grant-by-type", "undefined-variables", "unrecognized-proficiency-tag"];

/// The record a rule id belongs to (`book:kind:slug#suffix` -> `book:kind:slug`).
fn record_of(id: &str) -> &str {
    id.split('#').next().unwrap_or(id)
}

/// Every record carrying a closure defect: the attributed rows of [`CLOSURE_DEFECT_KINDS`] plus
/// the granter of every grant edge left unattached (`unattached`: target -> its grants).
pub fn defective_records(defects: &BTreeMap<String, Vec<String>>, unattached: &BTreeMap<RuleId, Vec<Grant>>) -> BTreeSet<RuleId> {
    let mut out = BTreeSet::new();
    for kind in CLOSURE_DEFECT_KINDS {
        for row in defects.get(kind).into_iter().flatten() {
            if let Some((id, _)) = row.split_once(": ") {
                out.insert(record_of(id).to_string());
            }
        }
    }
    for grants in unattached.values() {
        for g in grants {
            match &g.by {
                Granter::Rule(r) => {
                    out.insert(record_of(r).to_string());
                }
                // A class line that grants an unconverted target: the class itself is defective.
                Granter::Class { id, .. } => {
                    out.insert(format!("class-line:{id}"));
                }
                _ => {}
            }
        }
    }
    out
}

/// Set `closure_complete` on every class principal in `files` (rule file -> rules; the first
/// rule of a `<book>/class/<slug>.json` file is the principal). Returns
/// `(attested, class principals)`.
pub fn attest_class_closures(files: &mut BTreeMap<String, Vec<SheetRule>>, defective: &BTreeSet<RuleId>) -> (usize, usize) {
    // Edges, over records.
    let mut rule_edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut class_edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut principals: BTreeMap<String, (String, Option<String>)> = BTreeMap::new();
    for (rel, rules) in files.iter() {
        for rule in rules {
            let target = record_of(&rule.id).to_string();
            for g in &rule.granted_by {
                match &g.by {
                    Granter::Rule(r) => {
                        rule_edges.entry(record_of(r).to_string()).or_default().insert(target.clone());
                    }
                    Granter::Class { id, .. } => {
                        class_edges.entry(id.clone()).or_default().insert(target.clone());
                    }
                    _ => {}
                }
            }
        }
        if let Some(first) = rules.first()
            && rel.split('/').nth(1) == Some("class")
        {
            let slug = first.id.rsplit(':').next().unwrap_or_default().to_string();
            let base = first.grants.iter().find_map(|e| match e {
                Effect::TakenOnClass(b) => Some(b.clone()),
                _ => None,
            });
            principals.insert(rel.clone(), (slug, base));
        }
    }
    let principal_of_slug: BTreeMap<String, String> = files
        .iter()
        .filter(|(rel, _)| principals.contains_key(*rel))
        .filter_map(|(_, rules)| rules.first().map(|r| (r.id.rsplit(':').next().unwrap_or_default().to_string(), r.id.clone())))
        .collect();
    let complete = |slug: &str, base: Option<&str>| -> bool {
        let mut stack: Vec<String> = Vec::new();
        for class in std::iter::once(slug).chain(base) {
            if defective.contains(&format!("class-line:{class}")) {
                return false;
            }
            stack.extend(principal_of_slug.get(class).cloned());
            stack.extend(class_edges.get(class).into_iter().flatten().cloned());
        }
        let mut seen: BTreeSet<String> = BTreeSet::new();
        while let Some(record) = stack.pop() {
            if !seen.insert(record.clone()) {
                continue;
            }
            if defective.contains(&record) {
                return false;
            }
            stack.extend(rule_edges.get(&record).into_iter().flatten().cloned());
        }
        true
    };
    let mut attested = 0;
    for (rel, (slug, base)) in &principals {
        let ok = complete(slug, base.as_deref());
        if let Some(first) = files.get_mut(rel).and_then(|rules| rules.first_mut()) {
            first.closure_complete = ok;
        }
        attested += usize::from(ok);
    }
    (attested, principals.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::sheet_rule::{Applies, Fact, ProfRef, SheetValue, Subject};

    fn rule(id: &str, granted_by: Vec<Grant>) -> SheetRule {
        SheetRule {
            id: id.into(),
            label: id.into(),
            value: SheetValue::Text,
            also: Vec::new(),
            prose: Vec::new(),
            applies: Applies::Always,
            target: None,
            bonus_type: None,
            print: true,
            pool: String::new(),
            tags: Vec::new(),
            subject: Subject::Character,
            repeatable: false,
            granted_by,
            offers: None,
            grants: Vec::new(),
            closure_complete: false,
            provenance: Default::default(),
        }
    }

    fn by_class(c: &str) -> Vec<Grant> {
        vec![Grant { by: Granter::Class { id: c.into(), at_level: 1 }, when: Applies::Always }]
    }

    fn by_rule(r: &str) -> Vec<Grant> {
        vec![Grant { by: Granter::Rule(r.into()), when: Applies::Always }]
    }

    /// Two classes, each with a two-deep closure (class line -> feature -> sub-feature); `fx`'s
    /// sub-feature and `gx`'s are both clean unless a test adds a defect.
    fn files() -> BTreeMap<String, Vec<SheetRule>> {
        let mut f = BTreeMap::new();
        f.insert("b/class/fx.json".into(), vec![rule("b:class:fx", Vec::new())]);
        f.insert("b/class/gx.json".into(), vec![rule("b:class:gx", Vec::new())]);
        let mut feature = rule("b:class_feature:fx_feature", by_class("fx"));
        feature.grants = vec![Effect::FactGrant(Fact::Proficiency(ProfRef::Weapon("Club".into())))];
        f.insert("b/class_feature/fx_feature.json".into(), vec![feature, rule("b:class_feature:fx_feature#bonus0", Vec::new())]);
        f.insert("b/class_feature/fx_sub.json".into(), vec![rule("b:class_feature:fx_sub", by_rule("b:class_feature:fx_feature#bonus0"))]);
        f.insert("b/class_feature/gx_feature.json".into(), vec![rule("b:class_feature:gx_feature", by_class("gx"))]);
        f
    }

    fn attested(defects: &[(&str, &str)], unattached: &[(&str, Grant)]) -> BTreeMap<String, bool> {
        let mut d: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (k, row) in defects {
            d.entry(k.to_string()).or_default().push(row.to_string());
        }
        let mut u: BTreeMap<RuleId, Vec<Grant>> = BTreeMap::new();
        for (t, g) in unattached {
            u.entry(t.to_string()).or_default().push(g.clone());
        }
        let mut f = files();
        attest_class_closures(&mut f, &defective_records(&d, &u));
        f.iter().filter(|(rel, _)| rel.contains("/class/")).map(|(_, r)| (r[0].id.clone(), r[0].closure_complete)).collect()
    }

    /// D4 RED: a closure defect on ANY rule of the closure -- the class line's direct grant, a
    /// rule two edges down (reached through a sibling's edge), each defect family, and a grant
    /// onto an unconverted target -- makes that class's attestation false and no other's.
    #[test]
    fn attestation_is_false_when_any_closure_rule_has_a_defect() {
        let clean = attested(&[], &[]);
        assert_eq!(clean.get("b:class:fx"), Some(&true), "{clean:?}");
        assert_eq!(clean.get("b:class:gx"), Some(&true), "{clean:?}");
        for kind in CLOSURE_DEFECT_KINDS {
            let got = attested(&[(kind, "b:class_feature:fx_sub: Some Category|Some Target")], &[]);
            assert_eq!(got.get("b:class:fx"), Some(&false), "{kind} two edges down: {got:?}");
            assert_eq!(got.get("b:class:gx"), Some(&true), "{kind} must not touch gx: {got:?}");
        }
        let got = attested(&[("undefined-variables", "b:class_feature:fx_feature: SomeVar")], &[]);
        assert_eq!(got.get("b:class:fx"), Some(&false), "{got:?}");
        let got = attested(&[("grant-by-type", "b:class:gx: Internal|TYPE-selector")], &[]);
        assert_eq!(got.get("b:class:gx"), Some(&false), "the principal's own defect: {got:?}");
        assert_eq!(got.get("b:class:fx"), Some(&true), "{got:?}");
        let lost = Grant { by: Granter::Rule("b:class_feature:gx_feature".into()), when: Applies::Always };
        let got = attested(&[], &[("b:class_feature:never_converted", lost)]);
        assert_eq!(got.get("b:class:gx"), Some(&false), "a grant onto an unconverted target: {got:?}");
        // A prose defect is not a closure defect.
        let got = attested(&[("literal-in-prose", "b:class_feature:fx_sub: description (x)")], &[]);
        assert_eq!(got.get("b:class:fx"), Some(&true), "{got:?}");
    }

    /// A class-selection class's closure includes its base class's.
    #[test]
    fn a_class_selection_class_inherits_its_base_class_defects() {
        let mut d: BTreeMap<String, Vec<String>> = BTreeMap::new();
        d.insert("unresolved-references".into(), vec!["b:class_feature:fx_sub: X|Y".into()]);
        let mut f = files();
        let mut sel = rule("b:class:hx", Vec::new());
        sel.grants = vec![Effect::TakenOnClass("fx".into())];
        f.insert("b/class/hx.json".into(), vec![sel]);
        attest_class_closures(&mut f, &defective_records(&d, &BTreeMap::new()));
        assert!(!f["b/class/hx.json"][0].closure_complete);
        assert!(f["b/class/gx.json"][0].closure_complete);
    }
}
