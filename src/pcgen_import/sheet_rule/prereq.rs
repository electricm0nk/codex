//! The PREREQ rows: `PRE<kind>:...` and `!PRE<kind>:...` -> our two-valued `Applies` (plus
//! `Situational`, `SYNTHESIS.md` C9). Each arm cites the mapping-table row it transcribes.

use super::ctx::{split_top_level, RecordCtx};
use super::formula::{ability, cmp_expr, convert_formula};
use crate::rules_core::sheet_rule::{Applies, Cmp, DeityRef, Expr, Holdable, ProfRef, SpellKind};

/// `F/D/T/S/M/L/H/G/C` -> 0..8.
pub fn size_rank(code: &str) -> Option<i32> {
    match code.trim().to_ascii_uppercase().as_str() {
        "F" | "FINE" => Some(0),
        "D" | "DIMINUTIVE" => Some(1),
        "T" | "TINY" => Some(2),
        "S" | "SMALL" => Some(3),
        "M" | "MEDIUM" => Some(4),
        "L" | "LARGE" => Some(5),
        "H" | "HUGE" => Some(6),
        "G" | "GARGANTUAN" => Some(7),
        "C" | "COLOSSAL" => Some(8),
        _ => None,
    }
}

fn count_prefix(body: &str) -> (u8, Vec<String>) {
    let parts = split_top_level(body, ',');
    let mut it = parts.into_iter();
    let first = it.next().unwrap_or_default();
    match first.trim().parse::<u8>() {
        Ok(n) => (n, it.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()),
        Err(_) => (1, std::iter::once(first).chain(it).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()),
    }
}

fn at_least(n: u8, of: Vec<Applies>) -> Applies {
    match of.len() {
        0 => Applies::Never,
        1 if n <= 1 => of.into_iter().next().unwrap(),
        _ => Applies::AtLeast { n, of },
    }
}

fn holds(what: Holdable) -> Applies {
    Applies::Holds { what, count: 1 }
}

fn situational(text: &str) -> Applies {
    Applies::Situational { text: text.to_string() }
}

/// A gate as a 0/1 `Expr` for folding into an addend (row 19 / C17); `None` when the gate
/// reads a held-set fact the arithmetic cannot express.
pub fn applies_as_01(a: &Applies) -> Option<Expr> {
    Some(match a {
        Applies::Always => Expr::Const(1),
        Applies::Never => Expr::Const(0),
        Applies::Compare { lhs, op, rhs } => cmp_expr(*op, lhs.clone(), rhs.clone()),
        Applies::All(v) => {
            let mut acc = Expr::Const(1);
            for x in v {
                acc = Expr::min(acc, applies_as_01(x)?);
            }
            acc
        }
        Applies::AtLeast { n, of } => {
            let mut terms = Vec::new();
            for x in of {
                terms.push(applies_as_01(x)?);
            }
            cmp_expr(Cmp::Gte, Expr::sum(terms), Expr::Const(*n as i32))
        }
        Applies::Not(x) => Expr::sum(vec![Expr::Const(1), Expr::neg(applies_as_01(x)?)]),
        _ => return None,
    })
}

/// Convert one `PRE...` / `!PRE...` token (`KIND:body`) to `Applies`, or refuse.
pub fn convert_pre_token(ctx: &mut RecordCtx, token: &str) -> Result<Applies, String> {
    let t = token.trim();
    if let Some(rest) = t.strip_prefix('!') {
        // Row `!PRE* (all negated forms)`: Not(<the positive conversion>).
        let inner = convert_pre_token(ctx, rest)?;
        return Ok(match inner {
            Applies::Always => Applies::Never,
            Applies::Never => Applies::Always,
            Applies::Situational { .. } => inner,
            other => Applies::Not(Box::new(other)),
        });
    }
    let (kind, body) = match t.split_once(':') {
        Some((k, b)) => (k.trim(), b.trim()),
        None => return Err(format!("unmapped:{t}")),
    };
    if body.contains("[redacted PI]") && !matches!(kind, "PREMULT") {
        // C20: a redacted gate prints "requirement withheld".
        ctx.pi_declared.push(kind.to_string());
        return Ok(situational("requirement withheld"));
    }
    convert_pre(ctx, kind, body)
}

fn convert_pre(ctx: &mut RecordCtx, kind: &str, body: &str) -> Result<Applies, String> {
    Ok(match kind {
        // Row PREMULT: AtLeast{N, [sub-clauses]}.
        "PREMULT" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let inner = item.trim().trim_start_matches('[').trim_end_matches(']');
                of.push(convert_pre_token(ctx, inner)?);
            }
            at_least(n, of)
        }
        // Rows PREVAR*: Compare{resolve(name), op, value}; several pairs are all required.
        "PREVARGTEQ" | "PREVARGT" | "PREVARLT" | "PREVARLTEQ" | "PREVAREQ" | "PREVARNEQ" => {
            let op = match kind {
                "PREVARGTEQ" => Cmp::Gte,
                "PREVARGT" => Cmp::Gt,
                "PREVARLT" => Cmp::Lt,
                "PREVARLTEQ" => Cmp::Lte,
                "PREVAREQ" => Cmp::Eq,
                _ => Cmp::Ne,
            };
            let parts = split_top_level(body, ',');
            if !parts.len().is_multiple_of(2) {
                return Err(format!("{kind} (odd operand count)"));
            }
            let mut terms = Vec::new();
            for pair in parts.chunks(2) {
                let lhs = convert_formula(ctx, pair[0].trim())?;
                let rhs = convert_formula(ctx, pair[1].trim())?;
                if let (Expr::Const(a), Expr::Const(b)) = (&lhs, &rhs) {
                    // Both sides settled at convert time (a DEFINE-only flag against a literal):
                    // the gate is decided now, never printed.
                    let pass = match op {
                        Cmp::Gte => a >= b,
                        Cmp::Gt => a > b,
                        Cmp::Lt => a < b,
                        Cmp::Lte => a <= b,
                        Cmp::Eq => a == b,
                        Cmp::Ne => a != b,
                    };
                    terms.push(if pass { Applies::Always } else { Applies::Never });
                    continue;
                }
                terms.push(Applies::Compare { lhs, op, rhs });
            }
            Applies::all(terms)
        }
        // Row PREABILITY: AtLeast{N, Holds{Rule|RuleTag}}.
        "PREABILITY" => {
            let (n, items) = count_prefix(body);
            let mut category = String::new();
            let mut of = Vec::new();
            for item in items {
                if item == "CHECKMULT" {
                    continue;
                }
                if let Some(c) = item.strip_prefix("CATEGORY=") {
                    category = c.trim().to_string();
                    continue;
                }
                if let Some(t) = item.strip_prefix("TYPE.").or_else(|| item.strip_prefix("TYPE=")) {
                    of.push(holds(Holdable::RuleTag { pool: super::ctx::slug(&category), tag: t.to_string() }));
                    continue;
                }
                of.push(holds(resolve_holdable_rule(ctx, &category, &item)));
            }
            at_least(n, of)
        }
        // Row PRECLASS: AtLeast{N, Compare{ClassLevel >= lvl}}; SPELLCASTER forms -> HighestSpellLevel.
        "PRECLASS" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let (name, lvl) = item.split_once('=').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((item.trim(), "1"));
                let lvl: i32 = lvl.parse().unwrap_or(1);
                if name.eq_ignore_ascii_case("ANY") {
                    of.push(Applies::Compare { lhs: Expr::Level, op: Cmp::Gte, rhs: Expr::Const(lvl) });
                } else if let Some(rest) = name.strip_prefix("SPELLCASTER") {
                    let kind = match rest.trim_start_matches('.').to_ascii_uppercase().as_str() {
                        "ARCANE" => SpellKind::Arcane,
                        "DIVINE" => SpellKind::Divine,
                        "PSYCHIC" => SpellKind::Psychic,
                        _ => SpellKind::Any,
                    };
                    of.push(Applies::Compare { lhs: Expr::HighestSpellLevel(kind), op: Cmp::Gte, rhs: Expr::Const(1) });
                } else if let Some(t) = name.strip_prefix("TYPE.") {
                    of.push(holds(Holdable::ClassTag(t.to_string())));
                } else {
                    of.push(Applies::Compare { lhs: Expr::ClassLevel(ctx.class_id(name)), op: Cmp::Gte, rhs: Expr::Const(lvl) });
                }
            }
            at_least(n, of)
        }
        "PRECLASSLEVELMAX" => {
            let (_, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let (name, lvl) = item.split_once('=').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((item.trim(), "1"));
                let lvl: i32 = lvl.parse().unwrap_or(1);
                of.push(Applies::Compare { lhs: Expr::ClassLevel(ctx.class_id(name)), op: Cmp::Lte, rhs: Expr::Const(lvl) });
            }
            Applies::all(of)
        }
        // Row PRESUBCLASS: Words until the subclass reader exists.
        "PRESUBCLASS" => situational("requires the matching arcane school"),
        // Row PRESTAT: AtLeast{N, Compare{AbilityScore >= n}}.
        "PRESTAT" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let (ab, score) = item.split_once('=').ok_or_else(|| format!("{kind} (no =)"))?;
                let a = ability(ab.trim()).ok_or_else(|| format!("{kind} (unknown ability)"))?;
                of.push(Applies::Compare { lhs: Expr::AbilityScore(a), op: Cmp::Gte, rhs: Expr::Const(score.trim().parse().unwrap_or(0)) });
            }
            at_least(n, of)
        }
        // Row PRESKILL: AtLeast{N, Compare{SkillRanks(skill) >= ranks}}.
        "PRESKILL" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let (skill, ranks) = item.split_once('=').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((item.trim(), "1"));
                let ranks: i32 = ranks.parse().unwrap_or(1);
                if let Some(t) = skill.strip_prefix("TYPE.").or_else(|| skill.strip_prefix("TYPE=")) {
                    of.push(situational(&format!("requires {ranks} ranks in a {} skill", t.to_ascii_lowercase())));
                } else {
                    of.push(Applies::Compare { lhs: Expr::SkillRanks(ctx.skill_id(skill)), op: Cmp::Gte, rhs: Expr::Const(ranks) });
                }
            }
            at_least(n, of)
        }
        "PRECSKILL" => {
            let (n, items) = count_prefix(body);
            let of = items.into_iter().map(|s| holds(Holdable::ClassSkill(ctx.skill_id(&s)))).collect();
            at_least(n, of)
        }
        "PRETOTALAB" => Applies::Compare { lhs: Expr::BaseAttack, op: Cmp::Gte, rhs: Expr::Const(body.trim().parse().unwrap_or(0)) },
        "PRECHECKBASE" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let (save, v) = item.split_once('=').ok_or_else(|| format!("{kind} (no =)"))?;
                let s = match save.trim().to_ascii_uppercase().as_str() {
                    "FORTITUDE" => crate::rules_core::sheet_rule::Save::Fortitude,
                    "REFLEX" => crate::rules_core::sheet_rule::Save::Reflex,
                    "WILL" => crate::rules_core::sheet_rule::Save::Will,
                    _ => return Err(format!("{kind} (unknown save)")),
                };
                of.push(Applies::Compare { lhs: Expr::BaseSave(s), op: Cmp::Gte, rhs: Expr::Const(v.trim().parse().unwrap_or(0)) });
            }
            at_least(n, of)
        }
        "PRELEVEL" | "PREPCLEVEL" => {
            let mut terms = Vec::new();
            for part in body.split(',') {
                let p = part.trim();
                if let Some(v) = p.strip_prefix("MIN=") {
                    terms.push(Applies::Compare { lhs: Expr::Level, op: Cmp::Gte, rhs: Expr::Const(v.parse().unwrap_or(1)) });
                } else if let Some(v) = p.strip_prefix("MAX=") {
                    terms.push(Applies::Compare { lhs: Expr::Level, op: Cmp::Lte, rhs: Expr::Const(v.parse().unwrap_or(20)) });
                } else if let Ok(v) = p.parse::<i32>() {
                    terms.push(Applies::Compare { lhs: Expr::Level, op: Cmp::Gte, rhs: Expr::Const(v) });
                }
            }
            Applies::all(terms)
        }
        "PRELEVELMAX" => Applies::Compare { lhs: Expr::Level, op: Cmp::Lte, rhs: Expr::Const(body.trim().parse().unwrap_or(20)) },
        // Row PREHD (C8): racial hit dice.
        "PREHD" => {
            let mut terms = Vec::new();
            for part in body.split(',') {
                let p = part.trim();
                if let Some(v) = p.strip_prefix("MIN=") {
                    terms.push(Applies::Compare { lhs: Expr::HitDice, op: Cmp::Gte, rhs: Expr::Const(v.parse().unwrap_or(0)) });
                } else if let Some(v) = p.strip_prefix("MAX=") {
                    terms.push(Applies::Compare { lhs: Expr::HitDice, op: Cmp::Lte, rhs: Expr::Const(v.parse().unwrap_or(0)) });
                } else if let Some((a, b)) = p.split_once('+')
                    && b.is_empty()
                {
                    terms.push(Applies::Compare { lhs: Expr::HitDice, op: Cmp::Gte, rhs: Expr::Const(a.parse().unwrap_or(0)) });
                } else if let Ok(v) = p.parse::<i32>() {
                    terms.push(Applies::Compare { lhs: Expr::HitDice, op: Cmp::Gte, rhs: Expr::Const(v) });
                }
            }
            Applies::all(terms)
        }
        // Row PRERACE: Holds{Race|RaceType|RaceSubtype}; a trailing % is a prefix wildcard.
        "PRERACE" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                if let Some(t) = item.strip_prefix("RACETYPE=") {
                    of.push(holds(Holdable::RaceType(t.to_string())));
                } else if let Some(t) = item.strip_prefix("RACESUBTYPE=") {
                    of.push(holds(Holdable::RaceSubtype(t.to_string())));
                } else if let Some(prefix) = item.strip_suffix('%') {
                    let p = prefix.to_ascii_uppercase();
                    let matches: Vec<Applies> = ctx
                        .index
                        .by_kind_name
                        .iter()
                        .filter(|((k, n), _)| (k == "race" || k == "monster") && n.starts_with(&p))
                        .map(|(_, id)| holds(Holdable::Race(id.clone())))
                        .collect();
                    if matches.is_empty() {
                        of.push(holds(Holdable::Race(super::ctx::slug(prefix))));
                    } else {
                        of.push(at_least(1, matches));
                    }
                } else {
                    let id = ctx.resolve_kind("race", &item).or_else(|| ctx.resolve_kind("monster", &item)).unwrap_or_else(|| super::ctx::slug(&item));
                    of.push(holds(Holdable::Race(id)));
                }
            }
            at_least(n, of)
        }
        // Row PREFACT: Holds{Rule(declarer)}; no declarer -> the fact itself (Exclude unless declared).
        "PREFACT" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            let mut it = items.into_iter();
            let _set = it.next().unwrap_or_default();
            for item in it {
                let (name, value) = item.split_once('=').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((item.trim(), "true"));
                let key = (name.to_ascii_uppercase(), value.to_ascii_uppercase());
                match ctx.index.fact_declarers.get(&key) {
                    Some(ids) if !ids.is_empty() => {
                        let alts: Vec<Applies> = ids.iter().map(|id| holds(Holdable::Rule(id.clone()))).collect();
                        of.push(at_least(1, alts));
                    }
                    _ => of.push(holds(Holdable::Fact { name: name.to_string(), value: value.to_string() })),
                }
            }
            at_least(n, of)
        }
        // Head alias of the PRERACE row's `RACETYPE=<t>` clause (the table has no PRERACETYPE row;
        // recorded as a table defect in the cycle receipt).
        "PRERACETYPE" => {
            let (n, items) = count_prefix(body);
            let of = items.into_iter().map(|t| holds(Holdable::RaceType(t))).collect();
            at_least(n, of)
        }
        "PRETEMPLATE" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|t| {
                    let id = ctx.resolve_kind("template", &t).unwrap_or_else(|| super::ctx::slug(&t));
                    holds(Holdable::Template(id))
                })
                .collect();
            at_least(n, of)
        }
        // Rows PRESIZE* / PREBASESIZE*: Compare{Size|BaseSize op rank}.
        "PRESIZEEQ" | "PRESIZEGTEQ" | "PRESIZEGT" | "PRESIZELTEQ" | "PRESIZELT" | "PREBASESIZEEQ" | "PREBASESIZEGTEQ" | "PREBASESIZEGT" | "PREBASESIZELTEQ" | "PREBASESIZELT" => {
            let base = kind.contains("BASE");
            let op = if kind.ends_with("GTEQ") {
                Cmp::Gte
            } else if kind.ends_with("LTEQ") {
                Cmp::Lte
            } else if kind.ends_with("EQ") {
                Cmp::Eq
            } else if kind.ends_with("GT") {
                Cmp::Gt
            } else {
                Cmp::Lt
            };
            let rank = size_rank(body).ok_or_else(|| format!("{kind} (unknown size code)"))?;
            Applies::Compare { lhs: if base { Expr::BaseSize } else { Expr::Size }, op, rhs: Expr::Const(rank) }
        }
        // Row PREALIGN: Holds{Alignment(set)}; `Deity` -> AlignmentMatchesDeity.
        "PREALIGN" => {
            let set: Vec<String> = body.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            if set.iter().any(|s| s.eq_ignore_ascii_case("Deity")) {
                let mut of = vec![holds(Holdable::AlignmentMatchesDeity)];
                let rest: Vec<String> = set.into_iter().filter(|s| !s.eq_ignore_ascii_case("Deity")).collect();
                if !rest.is_empty() {
                    of.push(holds(Holdable::Alignment(rest)));
                }
                at_least(1, of)
            } else {
                holds(Holdable::Alignment(set))
            }
        }
        // Row PREDEITY: Y/N, names, PANTHEON.<name>.
        "PREDEITY" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                match item.as_str() {
                    "Y" | "YES" => of.push(holds(Holdable::Deity(DeityRef::Any))),
                    "N" | "NO" => of.push(Applies::Not(Box::new(holds(Holdable::Deity(DeityRef::Any))))),
                    other => {
                        if let Some(p) = other.strip_prefix("PANTHEON.") {
                            of.push(holds(Holdable::DeityInPantheon(p.to_string())));
                        } else {
                            let id = ctx.resolve_kind("deity", other).unwrap_or_else(|| super::ctx::slug(other));
                            of.push(holds(Holdable::Deity(DeityRef::Named(id))));
                        }
                    }
                }
            }
            at_least(n, of)
        }
        "PREDEITYDOMAIN" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|d| {
                    let id = ctx.resolve_kind("domain", &d).unwrap_or_else(|| super::ctx::slug(&d));
                    holds(Holdable::DeityGrantsDomain(id))
                })
                .collect();
            at_least(n, of)
        }
        "PREDEITYALIGN" => holds(Holdable::DeityAlignment(body.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())),
        "PREDOMAIN" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|d| {
                    if d.eq_ignore_ascii_case("ANY") {
                        return situational("requires a domain");
                    }
                    let id = ctx.resolve_kind("domain", &d).unwrap_or_else(|| super::ctx::slug(&d));
                    holds(Holdable::Rule(id))
                })
                .collect();
            at_least(n, of)
        }
        // Row PREWEAPONPROF: Weapon / TYPE.<x> / DEITYWEAPON.
        "PREWEAPONPROF" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|w| {
                    if w.eq_ignore_ascii_case("DEITYWEAPON") {
                        holds(Holdable::Proficiency(ProfRef::DeityFavoredWeapon))
                    } else if let Some(t) = w.strip_prefix("TYPE.").or_else(|| w.strip_prefix("TYPE=")) {
                        holds(Holdable::Proficiency(ProfRef::WeaponTag(t.to_string())))
                    } else {
                        holds(Holdable::Proficiency(ProfRef::Weapon(w)))
                    }
                })
                .collect();
            at_least(n, of)
        }
        "PREPROFWITHSHIELD" | "PREPROFWITHARMOR" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|w| {
                    let tag = w.trim_start_matches("TYPE.").trim_start_matches("TYPE=").to_string();
                    if kind == "PREPROFWITHSHIELD" {
                        holds(Holdable::Proficiency(ProfRef::ShieldGroup(tag)))
                    } else {
                        holds(Holdable::Proficiency(ProfRef::ArmorGroup(tag)))
                    }
                })
                .collect();
            at_least(n, of)
        }
        // Situational rows (C9): the words come from a per-type template, never the token body.
        "PREARMORTYPE" => {
            let (_, items) = count_prefix(body);
            let kinds: Vec<String> = items.iter().map(|i| i.trim_start_matches("TYPE.").trim_start_matches("TYPE=").to_ascii_lowercase()).collect();
            situational(&format!("while wearing {} armor", kinds.join(" or ")))
        }
        "PREEQUIP" | "PREEQUIPPRIMARY" | "PREEQUIPSECONDARY" | "PREEQUIPBOTH" | "PREEQUIPTWOWEAPON" => {
            let (_, items) = count_prefix(body);
            let names: Vec<String> = items.iter().map(|i| i.trim_start_matches("TYPE.").trim_start_matches("TYPE=").to_ascii_lowercase()).collect();
            situational(&format!("while {} is equipped", names.join(" or ")))
        }
        "PREDR" => situational("requires damage reduction"),
        "PREHANDSGTEQ" | "PREHANDSGT" | "PREHANDSEQ" | "PREHANDSLT" | "PREHANDSLTEQ" => situational(&format!("requires {} hands", body.trim())),
        "PREREACHGTEQ" | "PREREACHGT" | "PREREACHEQ" => situational(&format!("requires reach {} ft. or more", body.trim())),
        "PRETEXT" => situational(body.trim()),
        "PREITEM" => {
            let (_, items) = count_prefix(body);
            situational(&format!("requires {}", items.iter().map(|i| i.to_ascii_lowercase()).collect::<Vec<_>>().join(" or ")))
        }
        // Row PRESPELL / PRESPELLTYPE / PRESPELLCAST / PRESPELLBOOK / PRESPELLDESCRIPTOR.
        "PRESPELL" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|s| {
                    let id = ctx.resolve_kind("spell", &s).unwrap_or_else(|| super::ctx::slug(&s));
                    holds(Holdable::Spell(id))
                })
                .collect();
            at_least(n, of)
        }
        "PRESPELLTYPE" => {
            let (n, items) = count_prefix(body);
            let mut of = Vec::new();
            for item in items {
                let (k, lvl) = item.split_once('=').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((item.trim(), "1"));
                let kind = match k.to_ascii_uppercase().as_str() {
                    "ARCANE" => SpellKind::Arcane,
                    "DIVINE" => SpellKind::Divine,
                    "PSYCHIC" => SpellKind::Psychic,
                    _ => SpellKind::Any,
                };
                of.push(Applies::Compare { lhs: Expr::HighestSpellLevel(kind), op: Cmp::Gte, rhs: Expr::Const(lvl.parse().unwrap_or(1)) });
            }
            at_least(n, of)
        }
        "PRESPELLCAST" => {
            let mut terms = Vec::new();
            for part in body.split(',') {
                let p = part.trim();
                if let Some(t) = p.strip_prefix("TYPE=") {
                    let kind = match t.to_ascii_uppercase().as_str() {
                        "ARCANE" => SpellKind::Arcane,
                        "DIVINE" => SpellKind::Divine,
                        "PSYCHIC" => SpellKind::Psychic,
                        _ => SpellKind::Any,
                    };
                    terms.push(Applies::Compare { lhs: Expr::HighestSpellLevel(kind), op: Cmp::Gte, rhs: Expr::Const(1) });
                } else if p.starts_with("MEMORIZE=") {
                    terms.push(situational(if p.ends_with('Y') { "requires a prepared caster" } else { "requires a spontaneous caster" }));
                }
            }
            Applies::all(terms)
        }
        "PRESPELLBOOK" => situational("requires a spellbook"),
        "PRESPELLDESCRIPTOR" => situational("requires a spell with the named descriptor"),
        "PRESPELLSCHOOL" => situational("requires a spell of the named school"),
        // Row PREMOVE / PREVISION / PRELANG / PREGENDER / PREAGESET.
        "PREMOVE" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|m| {
                    let (mode, min) = m.split_once('=').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((m.trim(), "1"));
                    holds(Holdable::Movement { mode: mode.to_string(), min: min.parse().unwrap_or(1) })
                })
                .collect();
            at_least(n, of)
        }
        "PREVISION" => {
            let (n, items) = count_prefix(body);
            let of = items
                .into_iter()
                .map(|v| {
                    let mode = v.split('=').next().unwrap_or("").trim();
                    holds(Holdable::Vision(mode.to_string()))
                })
                .collect();
            at_least(n, of)
        }
        "PRELANG" => {
            let (n, items) = count_prefix(body);
            let of = items.into_iter().map(|l| holds(Holdable::Language(l))).collect();
            at_least(n, of)
        }
        "PREGENDER" => holds(Holdable::Gender(body.trim().to_string())),
        "PREAGESET" => {
            let (n, items) = count_prefix(body);
            let of = items.into_iter().map(|a| holds(Holdable::AgeCategory(a))).collect();
            at_least(n, of)
        }
        // Row PRETYPE: the ITEM's tags.
        "PRETYPE" => {
            let (n, items) = count_prefix(body);
            let tags = items.into_iter().map(|t| t.replace("EQMODTYPE=", "eqmod ").replace("TYPE=", "").replace('=', " ")).collect();
            Applies::ItemHas { tags, n }
        }
        // Metadata rows: never gate the sheet.
        "PRERULE" | "PRECAMPAIGN" | "PRECHARACTERTYPE" | "PRE" | "PREKIT" | "PREAPPLY" | "PREDEFAULTMONSTER" | "PREPOINTBUYMETHOD" | "PREBIRTHPLACE" | "PRECITY" | "PREREGION" => Applies::Always,
        "PRESA" | "PRESRGTEQ" | "PREUATT" | "PREHASDEITY" | "PREWIELD" | "PREARMORPROF" | "PRESHIELDPROF" | "PREVAR" => situational(&format!("requires {}", body.trim().to_ascii_lowercase())),
        other => return Err(format!("unmapped:{other}")),
    })
}

/// Resolve an ability-shaped reference `(category, name)` to a `Holdable`, splitting a
/// parameterised feat (`Weapon Focus (Longsword)`) into base id + option (C18).
pub fn resolve_holdable_rule(ctx: &mut RecordCtx, category: &str, name: &str) -> Holdable {
    if let Some(id) = ctx.resolve_rule(category, name) {
        return Holdable::Rule(id);
    }
    if let Some((base, option)) = name.rsplit_once(" (")
        && option.ends_with(')')
        && let Some(id) = ctx.resolve_rule(category, base)
    {
        // Parameterised: base rule held with the option chosen. Expressed as the rule; the
        // option narrows at pick time through the record's own choice.
        let _ = option;
        return Holdable::Rule(id);
    }
    ctx.defect("unresolved-references", format!("{}: {category}|{name}", ctx.record.id));
    Holdable::MissingRule { pool: super::ctx::slug(category), name: name.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_ranks_are_the_pathfinder_ordinals() {
        assert_eq!(size_rank("M"), Some(4));
        assert_eq!(size_rank("Large"), Some(5));
        assert_eq!(size_rank("X"), None);
    }

    #[test]
    fn count_prefix_defaults_to_one() {
        assert_eq!(count_prefix("1,A,B"), (1, vec!["A".into(), "B".into()]));
        assert_eq!(count_prefix("2,A"), (2, vec!["A".into()]));
        assert_eq!(count_prefix("A"), (1, vec!["A".into()]));
    }

    #[test]
    fn applies_01_encodes_compare_gates_only() {
        let g = Applies::Compare { lhs: Expr::Level, op: Cmp::Gte, rhs: Expr::Const(8) };
        assert!(applies_as_01(&g).is_some());
        let h = Applies::Holds { what: Holdable::Rule("a:b:c".into()), count: 1 };
        assert!(applies_as_01(&h).is_none());
    }
}
