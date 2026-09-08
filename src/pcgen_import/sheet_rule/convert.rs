//! One record's closure -> its `SheetRule`(s), or a refusal per token type.
//!
//! The walk is one pass over the closure rows in application order (`.COPY=` base, own rows,
//! level lines, `.MOD` rows). Every token head is routed through the transcribed table
//! (`table::row_for_head`): a `Refuse` row refuses the record under that row's `token_type`; a
//! head with no row refuses it as `unmapped:<HEAD>`; every other row is converted by the arm
//! that cites it below. A formula, gate, or prose refusal bubbles up as its own token type.

use std::collections::{BTreeMap, BTreeSet};

use super::closure::{Closure, ClosureRowKind, PinnedTree};
use super::ctx::{parse_bonus_type, slug, split_gates, split_top_level, CorpusIndex, RecordCtx, RecordRef};
use super::formula::{convert_formula, integer_literal};
use super::prereq::{convert_pre_token, resolve_holdable_rule};
use super::prose::{convert_desc_like, convert_labelled, convert_positional, decode_entities, expand_output_name, pi_hit};
use super::table::{row_for_head, MapsTo};
use crate::rules_core::sheet_rule::*;

pub const CONVERTER_VERSION: &str = "sheet_rule_convert/0.15.0";

/// The result of converting one record.
#[derive(Debug, Default)]
pub struct Converted {
    pub rules: Vec<SheetRule>,
    /// Grant edges this record hands to OTHER rules: `(target rule id, grant)`.
    pub grants_out: Vec<(RuleId, Grant)>,
    /// Contributions to cross-record variable tables: `(VarId, name upper, contribution)`.
    pub var_contribs: Vec<(VarId, String, VarContribution)>,
    /// Names this record's own rows declare.
    pub var_declares: Vec<(VarId, String)>,
    pub refusals: BTreeSet<String>,
    /// The token census (SD-35 AT-35-E2-004): every mapping-table row key this record's closure
    /// exercised (`unmapped:<HEAD>` / `BONUS:<SUB>` when the table has none).
    pub tokens: BTreeSet<String>,
    /// Refusal shape -> the token type(s) it arose under.
    pub refusal_under: BTreeMap<String, BTreeSet<String>>,
    pub defects: BTreeMap<String, Vec<String>>,
    pub var_names: BTreeMap<VarId, String>,
}

/// The census key for one token: its mapping-table row's `token_type`, else the same
/// `BONUS:<SUB>` / `unmapped:<HEAD>` string the refusal would carry. A PI-redacted head is
/// `[redacted PI] token`.
pub fn token_key(key: &str, value: &str) -> String {
    if key.starts_with("[redacted") {
        return "[redacted PI] token".to_string();
    }
    match row_for_head(key, value) {
        Some(r) => r.token_type.to_string(),
        None if key == "BONUS" => {
            let sub = value.split('|').next().unwrap_or("").split('=').next().unwrap_or("").trim();
            format!("BONUS:{sub}")
        }
        None => format!("unmapped:{key}"),
    }
}

/// A value line the record yields (principal first).
struct Line {
    suffix: Option<String>,
    label: String,
    value: SheetValue,
    also: Vec<(ValueRole, SheetValue)>,
    target: Option<BonusTarget>,
    bonus_type: Option<BonusType>,
    applies: Applies,
    prose: Vec<ProseSegment>,
}

struct Acc {
    category: String,
    tags: Vec<String>,
    print: bool,
    repeatable: bool,
    output_name: Option<String>,
    desc: Vec<ProseSegment>,
    benefit: Vec<ProseSegment>,
    special: Vec<ProseSegment>,
    aspects: BTreeMap<String, Vec<ProseSegment>>,
    when_active: Vec<ProseSegment>,
    stat_block: Vec<ProseSegment>,
    gates: Vec<Applies>,
    lines: Vec<Line>,
    also: Vec<(ValueRole, SheetValue)>,
    check_type: Option<String>,
    check_count: Option<Expr>,
    offers: Option<Choice>,
    select_count: Option<Expr>,
    grants: Vec<Effect>,
    granted_by: Vec<Grant>,
    subject: Subject,
    desc_redacted: bool,
    tempdesc_seen: bool,
}

/// A die literal with an optional flat modifier: `"1d8"` -> `("1d8", None)`, `"1d8+2"` ->
/// `("1d8", Some(Const(2)))`; `None` for anything that is not `NdM[+-K]`.
pub fn dice_literal(s: &str) -> Option<(String, Option<Expr>)> {
    let t = s.trim();
    let (dice, modifier) = match t.find(['+', '-']) {
        Some(i) if i > 0 => (&t[..i], Some(&t[i..])),
        _ => (t, None),
    };
    let (n, m) = dice.split_once('d')?;
    if n.is_empty() || m.is_empty() || !n.chars().all(|c| c.is_ascii_digit()) || !m.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let modifier = match modifier {
        Some(mv) => Some(Expr::Const(mv.replace('+', "").parse::<i32>().ok()?)),
        None => None,
    };
    Some((dice.to_string(), modifier))
}

fn save_of(name: &str) -> Option<Save> {
    match name.trim().to_ascii_uppercase().as_str() {
        "FORTITUDE" | "FORT" => Some(Save::Fortitude),
        "REFLEX" | "REF" => Some(Save::Reflex),
        "WILL" => Some(Save::Will),
        _ => None,
    }
}

fn scope_of(ctx: &RecordCtx, target: &str) -> Scope {
    let t = target.trim();
    if t.contains("%LIST") || t.contains("%CHOICE") {
        return Scope::Chosen(ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone()));
    }
    if let Some(x) = t.strip_prefix("SCHOOL.") {
        Scope::School(x.to_string())
    } else if let Some(x) = t.strip_prefix("SUBSCHOOL.") {
        Scope::Subschool(x.to_string())
    } else if let Some(x) = t.strip_prefix("DESCRIPTOR.") {
        Scope::Descriptor(x.to_string())
    } else if let Some(x) = t.strip_prefix("CLASS.") {
        Scope::Class(ctx.class_id(x))
    } else if let Some(x) = t.strip_prefix("SPELL.") {
        if x == "%LIST" || x == "%CHOICE" {
            Scope::Chosen(ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone()))
        } else {
            Scope::Spell(x.to_string())
        }
    } else if t.eq_ignore_ascii_case("ALLSPELLS") || t.eq_ignore_ascii_case("ALL") {
        Scope::All
    } else if t.contains("%LIST") || t.contains("%CHOICE") {
        Scope::Chosen(ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone()))
    } else {
        Scope::Class(ctx.class_id(t))
    }
}

fn weapon_ref(ctx: &RecordCtx, name: &str) -> WeaponRef {
    let n = name.trim();
    if n.contains("%LIST") || n.contains("%CHOICE") {
        WeaponRef::Chosen(ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone()))
    } else if let Some(g) = n.strip_prefix("TYPE.").or_else(|| n.strip_prefix("TYPE=")) {
        WeaponRef::Group(g.to_string())
    } else {
        WeaponRef::Named(n.to_string())
    }
}

/// The sheet total(s) a `BONUS:<sub>|<target>` feeds, with the label words for the line.
fn bonus_targets(ctx: &mut RecordCtx, sub: &str, target: &str) -> Result<Vec<(BonusTarget, String)>, String> {
    let t = target.trim();
    let chosen = |ctx: &RecordCtx| BonusTarget::Chosen(ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone()));
    Ok(match sub {
        "STAT" => {
            let mut out = Vec::new();
            for a in t.split(',') {
                let a = a.trim();
                if a.contains("%LIST") || a.contains("%CHOICE") {
                    out.push((chosen(ctx), "chosen ability score".to_string()));
                } else if let Some(ab) = super::formula::ability(a).or_else(|| super::formula::ability(a.get(..3).unwrap_or(""))) {
                    out.push((BonusTarget::Ability(ab), format!("{ab:?}")));
                } else {
                    return Err(format!("BONUS:STAT (target {})", a.split('=').next().unwrap_or("?")));
                }
            }
            out
        }
        "SKILL" => {
            let mut out = Vec::new();
            for s in split_top_level(t, ',') {
                let s = s.trim();
                if s.eq_ignore_ascii_case("LIST") || s.contains("%LIST") || s.contains("%CHOICE") {
                    out.push((chosen(ctx), "chosen skill".to_string()));
                } else if let Some(g) = s.strip_prefix("TYPE.").or_else(|| s.strip_prefix("TYPE=")) {
                    out.push((BonusTarget::SkillGroup(g.to_string()), format!("{} skills", g.to_ascii_lowercase())));
                } else if s.eq_ignore_ascii_case("ALL") {
                    out.push((BonusTarget::SkillGroup("All".into()), "all skills".into()));
                } else {
                    out.push((BonusTarget::Skill(ctx.skill_id(s)), s.to_string()));
                }
            }
            out
        }
        "COMBAT" => {
            let mut out = Vec::new();
            for c in t.split(',') {
                let c = c.trim();
                let cu = c.to_ascii_uppercase();
                let item = if cu == "AC" {
                    (BonusTarget::Ac, "AC".to_string())
                } else if cu == "BASEAB" {
                    (BonusTarget::BaseAttack, "base attack".to_string())
                } else if cu == "INITIATIVE" {
                    (BonusTarget::Initiative, "initiative".to_string())
                } else if cu.starts_with("TOHIT") {
                    (BonusTarget::Attack, "attack".to_string())
                } else if cu.starts_with("DAMAGE") {
                    let w = if cu.contains("MELEE") {
                        WeaponRef::Melee
                    } else if cu.contains("RANGED") {
                        WeaponRef::Ranged
                    } else {
                        WeaponRef::Any
                    };
                    (BonusTarget::Damage(w), "damage".to_string())
                } else if cu == "CMB" {
                    (BonusTarget::Cmb, "CMB".into())
                } else if cu == "CMD" {
                    (BonusTarget::Cmd, "CMD".into())
                } else {
                    (BonusTarget::Other(c.to_ascii_lowercase().replace('.', " ")), c.to_ascii_lowercase())
                };
                out.push(item);
            }
            out
        }
        "SAVE" => {
            let mut out = Vec::new();
            for s in t.split(',') {
                let s = s.trim();
                if let Some(b) = s.strip_prefix("BASE.") {
                    let sv = save_of(b).ok_or_else(|| format!("BONUS:SAVE (target {b})"))?;
                    out.push((BonusTarget::BaseSave(sv), format!("base {sv:?} save")));
                } else if s.eq_ignore_ascii_case("ALL") {
                    for sv in [Save::Fortitude, Save::Reflex, Save::Will] {
                        out.push((BonusTarget::Save(sv), format!("{sv:?} save")));
                    }
                } else if s.contains("%LIST") || s.contains("%CHOICE") {
                    out.push((chosen(ctx), "chosen save".to_string()));
                } else {
                    let sv = save_of(s).ok_or_else(|| format!("BONUS:SAVE (target {s})"))?;
                    out.push((BonusTarget::Save(sv), format!("{sv:?} save")));
                }
            }
            out
        }
        "CASTERLEVEL" => {
            if t.ends_with(".RESET") {
                return Ok(Vec::new());
            }
            let sc = scope_of(ctx, t);
            vec![(BonusTarget::CasterLevel(sc), "caster level".into())]
        }
        "DC" => {
            let mut out = Vec::new();
            for s in t.split(',') {
                out.push((BonusTarget::SpellDc(scope_of(ctx, s)), "spell DC".into()));
            }
            out
        }
        "HP" => vec![(BonusTarget::Hp, "hit points".into())],
        "MOVEADD" | "POSTMOVEADD" => {
            let mode = t.trim_start_matches("TYPE.").trim_start_matches("TYPE=").to_string();
            vec![(BonusTarget::Speed(mode.clone()), format!("{} speed", mode.to_ascii_lowercase()))]
        }
        "MOVEMULT" => vec![(BonusTarget::Other("speed multiplier".into()), "speed".into())],
        "VISION" => vec![(BonusTarget::Vision(t.to_string()), t.to_ascii_lowercase())],
        "DR" => vec![(BonusTarget::Dr, format!("DR/{}", t.to_ascii_lowercase()))],
        "SPELLCAST" | "SPELLKNOWN" => {
            let mut class = String::new();
            let mut level: u8 = 0;
            for p in t.split(';') {
                if let Some(c) = p.strip_prefix("CLASS=") {
                    class = c.trim().to_string();
                } else if let Some(l) = p.strip_prefix("LEVEL=") {
                    level = l.trim().parse().unwrap_or(0);
                } else if let Some(ty) = p.strip_prefix("TYPE=") {
                    class = ty.trim().to_string();
                }
            }
            if class.is_empty() {
                return Err(format!("BONUS:{sub} (target shape)"));
            }
            let id = ctx.class_id(&class);
            if sub == "SPELLCAST" {
                vec![(BonusTarget::SpellCell { class: id, level }, format!("level {level} spells per day"))]
            } else {
                vec![(BonusTarget::SpellsKnown { class: id, level }, format!("level {level} spells known"))]
            }
        }
        "ABILITYPOOL" => vec![(BonusTarget::Pool(slug(t)), format!("{} picks", t.to_ascii_lowercase()))],
        "SITUATION" => {
            let mut out = Vec::new();
            for pair in t.split(',') {
                let (skill, situation) = pair.split_once('=').ok_or_else(|| "BONUS:SITUATION (target shape)".to_string())?;
                if situation.contains('%') || situation.contains('=') {
                    return Err("BONUS:SITUATION (target shape)".into());
                }
                let sit = situation.trim().to_ascii_lowercase();
                out.push((BonusTarget::SkillSituation { skill: ctx.skill_id(skill), situation: sit.clone() }, format!("{} ({sit})", skill.trim())));
            }
            out
        }
        "SIZEMOD" => vec![(BonusTarget::Other("size".into()), "size".into())],
        "PCLEVEL" => vec![(BonusTarget::SpellcastingLevels(ctx.class_id(t)), format!("spellcasting as a {}", t.to_ascii_lowercase()))],
        "MISC" => vec![(BonusTarget::Other(t.to_ascii_lowercase()), t.to_ascii_lowercase())],
        "DOMAIN" => vec![(BonusTarget::Other("domains".into()), "domains".into())],
        "SKILLRANK" => {
            if t.contains("%LIST") || t.contains("%CHOICE") {
                vec![(chosen(ctx), "ranks in the chosen skill".into())]
            } else {
                vec![(BonusTarget::Other(format!("ranks in {}", t.to_ascii_lowercase())), format!("ranks in {t}"))]
            }
        }
        "SKILLPOINTS" => vec![(BonusTarget::Other("skill points".into()), "skill points".into())],
        "CONCENTRATION" => vec![(BonusTarget::Other("concentration".into()), "concentration".into())],
        "FEAT" => vec![(BonusTarget::Pool("feat".into()), "feats".into())],
        "LANGUAGES" => vec![(BonusTarget::Other("languages".into()), "languages".into())],
        "UDAM" => vec![(BonusTarget::Other("unarmed damage".into()), "unarmed damage".into())],
        "EQMARMOR" => vec![(BonusTarget::Other(format!("armor {}", t.to_ascii_lowercase())), "armor".into())],
        "RANGEADD" | "RANGEMULT" => vec![(BonusTarget::Other("range".into()), "range".into())],
        "WEAPON" => vec![(BonusTarget::Other(format!("weapon {}", t.to_ascii_lowercase())), "weapon".into())],
        "WIELDCATEGORY" => vec![(BonusTarget::Other("wield category".into()), "wield category".into())],
        "SPECIALTYSPELLKNOWN" => vec![(BonusTarget::Other("specialty spells known".into()), "specialty spells known".into())],
        "SKILLPOOL" => vec![(BonusTarget::Other("skill pool".into()), "skill pool".into())],
        other => return Err(format!("BONUS:{other}")),
    })
}

/// A source qualifier as a game-rule word: `TYPE=Martial` -> `Martial`, `ANY[TYPE=Exotic]` ->
/// `Exotic`, `!PC,TYPE=Spoken` -> `Spoken (not already known)`, `SHIELDTYPE=Tower` -> `Tower`,
/// `EQMODTYPE=Mundane` -> `eqmod Mundane`. No `=` survives.
fn tag_word(s: &str) -> String {
    let mut t = s.trim().to_string();
    if t.contains("%LIST") || t.contains("%CHOICE") {
        return "the chosen option".to_string();
    }
    let mut notes: Vec<&str> = Vec::new();
    if let Some(rest) = t.strip_prefix("!PC,") {
        notes.push("not already known");
        t = rest.to_string();
    } else if let Some(rest) = t.strip_prefix("PC,") {
        notes.push("already known");
        t = rest.to_string();
    }
    if let Some(inner) = t.strip_prefix("ANY[").and_then(|x| x.strip_suffix(']')) {
        t = inner.to_string();
    }
    for prefix in ["EQMODTYPE=", "ARMORTYPE=", "SHIELDTYPE=", "WEAPONTYPE=", "TYPE=", "TYPE."] {
        if let Some(rest) = t.strip_prefix(prefix) {
            t = if prefix == "EQMODTYPE=" { format!("eqmod {rest}") } else { rest.to_string() };
            break;
        }
    }
    t = t.replace('=', " ");
    if notes.is_empty() { t } else { format!("{t} ({})", notes.join(", ")) }
}

/// Map the `TYPE:` facet's dot-segments to tags.
fn tags_of(value: &str) -> Vec<String> {
    value.split('.').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

/// Which prose families the corpus record declares product identity for.
fn declared_pi(record: &RecordRef) -> (bool, bool) {
    let name = record.pi_fields.iter().any(|f| f == "name");
    let desc = record.pi_fields.iter().any(|f| f == "description");
    (name, desc)
}

pub fn convert_record(tree: &PinnedTree, index: &CorpusIndex, record: &RecordRef, closure: &Closure) -> Converted {
    let mut ctx = RecordCtx::new(tree, index, record, closure);
    let mut out = Converted::default();
    let (_pi_name, pi_desc) = declared_pi(record);
    if pi_desc {
        ctx.pi_declared.push("description".into());
    }
    let mut acc = Acc {
        category: record.category.clone(),
        tags: tags_of(&record.type_facet),
        print: true,
        repeatable: false,
        output_name: None,
        desc: Vec::new(),
        benefit: Vec::new(),
        special: Vec::new(),
        aspects: BTreeMap::new(),
        when_active: Vec::new(),
        stat_block: Vec::new(),
        gates: Vec::new(),
        lines: Vec::new(),
        also: Vec::new(),
        check_type: None,
        check_count: None,
        offers: None,
        select_count: None,
        grants: Vec::new(),
        granted_by: Vec::new(),
        subject: Subject::Character,
        desc_redacted: pi_desc,
        tempdesc_seen: false,
    };
    // The choice id is the record's own id when it carries a CHOOSE (pre-scan so %CHOICE
    // markers before the CHOOSE token still bind).
    let has_choose = closure.rows.iter().any(|r| r.tokens.iter().any(|(k, v)| k == "CHOOSE" && !v.trim().eq_ignore_ascii_case("NOCHOICE")));
    if has_choose {
        ctx.choice_id = Some(record.id.clone());
    }
    // Token-less feat records: the `prerequisites` list is the record's PRE closure.
    for pre in &record.prerequisites {
        let (head, tail) = pre.split_once(':').unwrap_or((pre.as_str(), ""));
        let under = token_key(head.trim(), tail);
        ctx.carry(under.clone());
        match convert_pre_token(&mut ctx, pre) {
            Ok(a) => acc.gates.push(a),
            Err(tt) => ctx.refuse_under(&under, tt),
        }
    }

    for row in &closure.rows {
        let level_gate = row.level_gate;
        for (key, value) in &row.tokens {
            let key = key.trim();
            let value = value.as_str();
            if row.kind == ClosureRowKind::LevelLine && !matches!(key, "ABILITY" | "BONUS" | "ADD" | "DOMAIN" | "TEMPLATE" | "SPELLS" | "UDAM" | "UMULT" | "DEFINE" | "AUTO" | "CSKILL") {
                // Class chassis rows (CAST, KNOWN, SPECIALS, ...) the engine already holds.
                continue;
            }
            // The census key: the row this token resolves to, recorded before any branch
            // below decides what to do with it (AT-35-E2-004).
            let under = token_key(key, value);
            ctx.carry(under.clone());
            if value.contains("[redacted PI]") {
                match key {
                    "BONUS" | "DEFINE" | "SPELLS" => {
                        if let Some(r) = row_for_head(key, value) {
                            ctx.refuse_under(&under, r.token_type);
                        }
                    }
                    k if k.starts_with("PRE") || k.starts_with("!PRE") => {
                        ctx.pi_declared.push(k.to_string());
                        acc.gates.push(Applies::Situational { text: "requirement withheld".into() });
                    }
                    other => ctx.pi_declared.push(other.to_string()),
                }
                continue;
            }
            if key.starts_with("[redacted") {
                ctx.pi_declared.push("token".into());
                continue;
            }
            let Some(trow) = row_for_head(key, value) else {
                // No row: the census key IS the refusal shape (`BONUS:<SUB>` / `unmapped:<HEAD>`).
                ctx.refuse_under(&under, under.clone());
                continue;
            };
            if trow.maps_to == MapsTo::Refuse {
                ctx.refuse_under(&under, trow.token_type);
                continue;
            }
            if let Err(tt) = convert_token(&mut ctx, &mut acc, &mut out, key, value, level_gate, row.kind) {
                ctx.refuse_under(&under, tt);
            }
        }
    }

    // ---- assemble -------------------------------------------------------------------------
    let label = {
        let base = record.name.clone();
        match &acc.output_name {
            Some(on) if !record.pi_fields.iter().any(|f| f == "name") => {
                let expanded = expand_output_name(on, &base);
                if pi_hit(&expanded).is_some() {
                    ctx.pi_term_hits.push("output_name".into());
                    base
                } else {
                    expanded
                }
            }
            _ => base,
        }
    };
    if let Some(cnt) = acc.check_count.take() {
        let period = acc.check_type.clone().unwrap_or_else(|| "day".into());
        acc.also.push((ValueRole::Uses { period }, SheetValue::Number(cnt)));
    }
    if let (Some(count), Some(offers)) = (acc.select_count.take(), acc.offers.as_mut()) {
        offers.count = count;
    }
    let mut prose: Vec<ProseSegment> = Vec::new();
    if !acc.desc_redacted {
        prose.append(&mut acc.desc);
    }
    prose.append(&mut acc.benefit);
    prose.append(&mut acc.special);
    for (_, segs) in acc.aspects.iter_mut() {
        prose.append(segs);
    }
    prose.append(&mut acc.stat_block);
    prose.append(&mut acc.when_active);
    let record_applies = Applies::all(acc.gates.clone());
    let provenance = Provenance {
        book: record.book.clone(),
        kind: record.kind.clone(),
        closure_rows: closure.rows.iter().map(|r| r.cite.clone()).collect(),
        oracle_pin: super::oracle_pin(),
        converter_version: CONVERTER_VERSION.into(),
        pi: PiStamp { declared: dedup(ctx.pi_declared.clone()), term_hits: dedup(ctx.pi_term_hits.clone()) },
        overlay: closure.overlay.clone(),
    };
    let pool = slug(&acc.category);
    let mut lines: Vec<Line> = std::mem::take(&mut acc.lines).into_iter().filter(|l| l.applies != Applies::Never).collect();
    if lines.is_empty() {
        lines.push(Line { suffix: None, label: label.clone(), value: SheetValue::Text, also: Vec::new(), target: None, bonus_type: None, applies: Applies::Always, prose: Vec::new() });
    }
    let mut principal_also = std::mem::take(&mut acc.also);
    for (i, line) in lines.into_iter().enumerate() {
        let id = match (&line.suffix, i) {
            (_, 0) => record.id.clone(),
            (Some(s), _) => format!("{}#{s}", record.id),
            (None, n) => format!("{}#{n}", record.id),
        };
        let mut also = line.also;
        if i == 0 {
            also.append(&mut principal_also);
        }
        let mut line_prose = line.prose;
        if i == 0 {
            line_prose.extend(prose.iter().cloned());
        }
        out.rules.push(SheetRule {
            id,
            label: if i == 0 { label.clone() } else { line.label },
            value: line.value,
            also,
            prose: line_prose,
            applies: Applies::all(vec![record_applies.clone(), line.applies]),
            target: line.target,
            bonus_type: line.bonus_type,
            print: acc.print,
            pool: pool.clone(),
            tags: acc.tags.clone(),
            subject: acc.subject,
            repeatable: acc.repeatable,
            granted_by: if i == 0 { acc.granted_by.clone() } else { Vec::new() },
            offers: if i == 0 { acc.offers.clone() } else { None },
            grants: if i == 0 { acc.grants.clone() } else { Vec::new() },
            provenance: provenance.clone(),
        });
    }
    out.refusals = ctx.refusals;
    out.tokens = ctx.tokens;
    out.refusal_under = ctx.refusal_under;
    out.defects = ctx.defects;
    out.var_names = ctx.var_names;
    out
}

fn dedup(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

fn gates_of(ctx: &mut RecordCtx, gates: &[String], level_gate: Option<u8>) -> Result<Applies, String> {
    let mut terms = Vec::new();
    for g in gates {
        if g == "PRE:.CLEAR" {
            terms.clear();
            continue;
        }
        terms.push(convert_pre_token(ctx, g)?);
    }
    if let Some(l) = level_gate
        && let Some(cls) = ctx.owning_class.clone()
    {
        terms.push(Applies::Compare { lhs: Expr::ClassLevel(cls), op: Cmp::Gte, rhs: Expr::Const(l as i32) });
    }
    Ok(Applies::all(terms))
}

fn push_stat(acc: &mut Acc, label: &str, pieces: Vec<ProsePiece>) {
    if pieces.is_empty() {
        return;
    }
    acc.stat_block.push(ProseSegment { family: ProseFamily::StatBlock(label.to_string()), pieces, applies: None, pick_last: false, suppress_when_all_zero: false });
}

fn text_stat(ctx: &mut RecordCtx, acc: &mut Acc, label: &str, value: &str, field: &str) {
    let text = decode_entities(value.trim());
    if text.is_empty() {
        return;
    }
    if pi_hit(&text).is_some() {
        ctx.pi_term_hits.push(field.to_string());
        return;
    }
    push_stat(acc, label, vec![ProsePiece::Text(text)]);
}

/// A number-or-formula field as a stat-block piece: a literal stays text, anything else is a slot.
fn number_piece(ctx: &mut RecordCtx, field: &str) -> Result<ProsePiece, String> {
    let f = field.trim();
    if integer_literal(f).is_some() {
        return Ok(ProsePiece::Text(f.to_string()));
    }
    Ok(ProsePiece::Slot(convert_formula(ctx, f)?))
}

#[allow(clippy::too_many_arguments)]
fn convert_token(ctx: &mut RecordCtx, acc: &mut Acc, out: &mut Converted, key: &str, value: &str, level_gate: Option<u8>, row_kind: ClosureRowKind) -> Result<(), String> {
    let v = value.trim();
    match key {
        // ---- identity / metadata rows ------------------------------------------------------
        "KEY" | "SORTKEY" | "SOURCEPAGE" | "SOURCELONG" | "SOURCESHORT" | "SOURCEWEB" | "SOURCEDATE" | "SOURCELINK" | "KIT" | "STARTFEATS" | "LEVELSPERFEAT" | "MAXLEVEL" | "RACETYPE"
        | "RACESUBTYPE" | "SUBRACE" | "DEITYWEAP" | "ALIGN" | "USEUNTRAINED" | "ROLE" | "NAMEOPT" | "ITYPE" | "REPLACES" | "FORMATCAT" | "ASSIGNTOALL" | "REGION" | "REMOVABLE" | "VARIANTS"
        | "INFO" | "EXCLUSIVE" | "ALLOWBASECLASS" | "EXCLASS" | "WEAPONBONUS" | "ACHECK" | "CHANGEPROF" | "ADDSPELLLEVEL" | "WT" | "COST" | "PLUS" | "ADDLEVEL" | "KEYSTAT" | "ITEM" | "EQMOD"
        | "ALTEQMOD" | "PROFICIENCY" | "WIELD" | "CONTAINS" | "CHARGES" | "BASEITEM" | "BASEQTY" | "MODS" | "FUMBLERANGE" | "SIZE" | "LEGS" | "HANDS" | "SPELLLEVEL" | "SPELLKNOWN" | "CRMOD"
        | "DEFINESTAT" | "STAT" | "AC" | "LEVELADJUSTMENT" | "SITUATION" | "MONSTERCLASS" | "FACTSET" => {}
        "CATEGORY" => {
            acc.category = v.to_string();
        }
        "TYPE" => {
            if v == ".CLEAR" {
                acc.tags.clear();
            } else if let Some(rest) = v.strip_prefix(".CLEAR.") {
                acc.tags = tags_of(rest);
            } else if row_kind == ClosureRowKind::Mod {
                for t in tags_of(v) {
                    if !acc.tags.contains(&t) {
                        acc.tags.push(t);
                    }
                }
            } else {
                acc.tags = tags_of(v);
            }
        }
        "VISIBLE" => {
            // Row VISIBLE (R1): print iff YES | EXPORT | QUALIFY; NO and DISPLAY held silently.
            let head = v.split('|').next().unwrap_or("").trim().to_ascii_uppercase();
            acc.print = matches!(head.as_str(), "YES" | "EXPORT" | "QUALIFY" | "");
        }
        "MULT" => acc.repeatable = v.eq_ignore_ascii_case("YES"),
        "STACK" => {}
        "NAMEISPI" => {
            if v.eq_ignore_ascii_case("YES") {
                ctx.pi_declared.push("name".into());
            }
        }
        "DESCISPI" => {
            if v.eq_ignore_ascii_case("YES") {
                ctx.pi_declared.push("description".into());
                acc.desc_redacted = true;
            }
        }
        "OUTPUTNAME" => acc.output_name = Some(decode_entities(v)),
        "FACT" => {
            if let Some((name, val)) = v.split_once('|') {
                acc.grants.push(Effect::FactDeclare { name: name.trim().to_string(), value: val.trim().to_string() });
            }
        }
        "DEFINE" => {
            if let Some((name, _)) = v.split_once('|') {
                let id = super::ctx::var_id(name);
                out.var_declares.push((id, name.trim().to_ascii_uppercase()));
            }
        }
        // ---- record-level gates ------------------------------------------------------------
        k if k.starts_with("PRE") || k.starts_with("!PRE") => {
            if k == "PRE" && v == ".CLEAR" {
                acc.gates.clear();
                return Ok(());
            }
            if k == "PRETYPE" {
                // Row PRETYPE: the ITEM's tags -- the rule's subject is the item.
                acc.subject = Subject::Item;
            }
            let a = convert_pre_token(ctx, &format!("{k}:{v}"))?;
            acc.gates.push(a);
        }
        // ---- prose rows --------------------------------------------------------------------
        "DESC" => {
            if v == ".CLEAR" {
                acc.desc.clear();
            } else if let Some(seg) = convert_desc_like(ctx, ProseFamily::Desc, v, "description")? {
                acc.desc.push(seg);
            }
        }
        "BENEFIT" => {
            if v == ".CLEAR" {
                acc.benefit.clear();
            } else if let Some(seg) = convert_desc_like(ctx, ProseFamily::Benefit, v, "benefit")? {
                acc.benefit.push(seg);
            }
        }
        "SPROP" | "SAB" => {
            if v == ".CLEAR" {
                acc.special.clear();
            } else if let Some(seg) = convert_positional(ctx, ProseFamily::Special, v, key.to_ascii_lowercase().as_str())? {
                acc.special.push(seg);
            }
        }
        "TEMPDESC" => {
            acc.tempdesc_seen = true;
            if let Some(seg) = convert_labelled(ctx, ProseFamily::WhenActive, v, "tempdesc", false)? {
                acc.when_active.push(seg);
            }
        }
        "ASPECT" => {
            let (fields, gates) = split_gates(v);
            let sub = fields.first().map(|s| s.trim().to_string()).unwrap_or_default();
            if sub.contains("[redacted") {
                ctx.pi_declared.push("aspect".into());
                return Ok(());
            }
            if sub.contains('%') {
                // C21: a malformed sub-key carrying a marker is refused by shape.
                return Err("ASPECT:<malformed sub-key>".into());
            }
            if super::table::is_structural_aspect(&sub) {
                return Ok(());
            }
            if sub.ends_with("CheckType") {
                acc.check_type = fields.get(1).map(|s| s.trim().to_ascii_lowercase());
                return Ok(());
            }
            if sub.ends_with("CheckCount") {
                // Row ASPECT:CheckCount: the number of boxes = the named variable's Expr.
                let arg = fields.get(2).or(fields.get(1)).map(|s| s.trim().to_string()).unwrap_or_default();
                if arg.is_empty() {
                    return Err("ASPECT:CheckCount / ASPECT:CheckType (no variable)".into());
                }
                let e = if let Some(n) = integer_literal(&arg) { Expr::Const(n) } else { convert_formula(ctx, &arg)? };
                acc.check_count = Some(e);
                return Ok(());
            }
            let rest: Vec<String> = fields.iter().skip(1).cloned().collect();
            let mut body = rest.join("|");
            if !gates.is_empty() {
                body.push('|');
                body.push_str(&gates.join("|"));
            }
            let family = if sub == "NAME" { ProseFamily::Desc } else { ProseFamily::Aspect(sub.clone()) };
            if let Some(seg) = convert_labelled(ctx, family, &body, "aspect", sub != "NAME")? {
                acc.aspects.entry(sub).or_default().push(seg);
            }
        }
        "SCHOOL" | "SUBSCHOOL" | "DESCRIPTOR" | "DURATION" | "CASTTIME" | "TARGETAREA" | "COMPS" | "SAVEINFO" | "SPELLRES" | "QUALITY" | "UNENCUMBEREDMOVE" | "CR" => {
            let label = match key {
                "SCHOOL" => "School",
                "SUBSCHOOL" => "Subschool",
                "DESCRIPTOR" => "Descriptor",
                "DURATION" => "Duration",
                "CASTTIME" => "Casting time",
                "TARGETAREA" => "Target or area",
                "COMPS" => "Components",
                "SAVEINFO" => "Saving throw",
                "SPELLRES" => "Spell resistance",
                "QUALITY" => "Quality",
                "UNENCUMBEREDMOVE" => "Unencumbered movement",
                _ => "CR",
            };
            let shown = if key == "QUALITY" { v.replace('|', ": ") } else { v.replace('|', ", ") };
            text_stat(ctx, acc, label, &shown, key);
        }
        "CRITRANGE" | "ALTCRITRANGE" => {
            let n: i32 = v.parse().unwrap_or(20);
            let text = if n >= 20 { "20".to_string() } else { format!("{n}-20") };
            push_stat(acc, if key == "CRITRANGE" { "Critical threat" } else { "Alternate critical threat" }, vec![ProsePiece::Text(text)]);
        }
        "CRITMULT" | "ALTCRITMULT" | "UMULT" => {
            let text = if v.starts_with('x') { v.to_string() } else { format!("x{v}") };
            push_stat(acc, "Critical multiplier", vec![ProsePiece::Text(text)]);
        }
        // ---- dice rows ---------------------------------------------------------------------
        "DAMAGE" | "ALTDAMAGE" => {
            // Row DAMAGE / ALTDAMAGE: Dice; `0` / `Special` -> Text.
            match dice_literal(v) {
                Some((dice, modifier)) => acc.lines.push(Line {
                    suffix: if key == "ALTDAMAGE" { Some("alt".into()) } else { None },
                    label: if key == "ALTDAMAGE" { format!("{} (second head)", ctx.record.name) } else { ctx.record.name.clone() },
                    value: SheetValue::Dice { dice, modifier, size_steps: None },
                    also: Vec::new(),
                    target: None,
                    bonus_type: None,
                    applies: Applies::Always,
                    prose: Vec::new(),
                }),
                None => {
                    let words = if v == "0" { "no damage".to_string() } else { v.to_ascii_lowercase() };
                    push_stat(acc, "Damage", vec![ProsePiece::Text(words)]);
                }
            }
        }
        "UDAM" => {
            let parts: Vec<&str> = v.split(',').map(|s| s.trim()).collect();
            if parts.len() == 9 {
                let mut arr: [String; 9] = Default::default();
                for (i, p) in parts.iter().enumerate() {
                    arr[i] = p.to_string();
                }
                acc.lines.push(Line { suffix: None, label: ctx.record.name.clone(), value: SheetValue::DiceBySize(arr), also: Vec::new(), target: None, bonus_type: None, applies: Applies::Always, prose: Vec::new() });
            } else if let Some((dice, modifier)) = dice_literal(parts.first().unwrap_or(&"")) {
                acc.lines.push(Line { suffix: None, label: ctx.record.name.clone(), value: SheetValue::Dice { dice, modifier, size_steps: None }, also: Vec::new(), target: None, bonus_type: None, applies: gates_of(ctx, &[], level_gate)?, prose: Vec::new() });
            } else {
                return Err("UDAM (shape)".into());
            }
        }
        "NATURALATTACKS" => {
            // Row NATURALATTACKS: one Dice line per entry (name, count, die).
            for (i, entry) in v.split('|').enumerate() {
                let parts: Vec<&str> = entry.split(',').map(|s| s.trim()).collect();
                if parts.len() < 4 {
                    return Err("NATURALATTACKS (shape)".into());
                }
                let name = parts[0].to_string();
                if pi_hit(&name).is_some() {
                    ctx.pi_term_hits.push("natural attack".into());
                    continue;
                }
                let count: u32 = parts[2].trim_start_matches('*').parse().unwrap_or(1);
                let (dice, modifier) = match dice_literal(parts[3]) {
                    Some(d) => d,
                    None if parts[3].trim().parse::<i32>().is_ok_and(|n| n > 0) => {
                        // A fixed damage amount (Fine creatures deal 1 point): a final number.
                        let n: i32 = parts[3].trim().parse().unwrap_or(0);
                        let label = if count > 1 { format!("{count} {name}") } else { name.clone() };
                        acc.lines.push(Line { suffix: Some(format!("natural{i}")), label, value: SheetValue::Number(Expr::Const(n)), also: Vec::new(), target: None, bonus_type: None, applies: Applies::Always, prose: Vec::new() });
                        continue;
                    }
                    None if parts[3].trim() == "0" => {
                        // A touch attack with no damage die: words, like `DAMAGE:0`.
                        let label = if count > 1 { format!("{count} {name}") } else { name.clone() };
                        acc.lines.push(Line { suffix: Some(format!("natural{i}")), label, value: SheetValue::Text, also: Vec::new(), target: None, bonus_type: None, applies: Applies::Always, prose: vec![ProseSegment { family: ProseFamily::Special, pieces: vec![ProsePiece::Text("touch attack, no damage".into())], applies: None, pick_last: false, suppress_when_all_zero: false }] });
                        continue;
                    }
                    None => return Err("NATURALATTACKS (die shape)".into()),
                };
                let mut prose = Vec::new();
                for extra in parts.iter().skip(4) {
                    if let Some(sp) = extra.strip_prefix("SPROP=") {
                        if pi_hit(sp).is_some() {
                            ctx.pi_term_hits.push("natural attack".into());
                        } else {
                            prose.push(ProseSegment { family: ProseFamily::Special, pieces: vec![ProsePiece::Text(sp.to_string())], applies: None, pick_last: false, suppress_when_all_zero: false });
                        }
                    }
                }
                let label = if count > 1 { format!("{count} {name}") } else { name.clone() };
                acc.lines.push(Line { suffix: Some(format!("natural{i}")), label, value: SheetValue::Dice { dice, modifier, size_steps: None }, also: Vec::new(), target: None, bonus_type: None, applies: Applies::Always, prose });
            }
        }
        // ---- stat-block numbers --------------------------------------------------------------
        "DR" => {
            let (fields, gates) = split_gates(v);
            let body = fields.first().cloned().unwrap_or_default();
            let (amount, bypass) = body.split_once('/').unwrap_or((body.as_str(), "-"));
            let piece = number_piece(ctx, amount)?;
            let mut seg = ProseSegment { family: ProseFamily::StatBlock("DR".into()), pieces: vec![piece, ProsePiece::Text(format!("/{}", bypass.trim()))], applies: None, pick_last: false, suppress_when_all_zero: false };
            let g = gates_of(ctx, &gates, level_gate)?;
            if g != Applies::Always {
                seg.applies = Some(g);
            }
            acc.stat_block.push(seg);
        }
        "SR" => {
            let (fields, gates) = split_gates(v);
            let body = fields.first().cloned().unwrap_or_default();
            let piece = number_piece(ctx, &body)?;
            let mut seg = ProseSegment { family: ProseFamily::StatBlock("SR".into()), pieces: vec![piece], applies: None, pick_last: false, suppress_when_all_zero: false };
            let g = gates_of(ctx, &gates, level_gate)?;
            if g != Applies::Always {
                seg.applies = Some(g);
            }
            acc.stat_block.push(seg);
        }
        "RANGE" => {
            // Row RANGE: Close/Medium/Long keywords are caster-level formulas; anything else is words.
            let low = v.to_ascii_lowercase();
            let cl = Expr::CasterLevel(ClassRef::Holder);
            let expr = if low.starts_with("close") {
                Some(Expr::sum(vec![Expr::Const(25), Expr::mul(Expr::Const(5), Expr::Floor(Box::new(Expr::div(cl, Expr::Const(2)))))]))
            } else if low.starts_with("medium") {
                Some(Expr::sum(vec![Expr::Const(100), Expr::mul(Expr::Const(10), cl)]))
            } else if low.starts_with("long") {
                Some(Expr::sum(vec![Expr::Const(400), Expr::mul(Expr::Const(40), cl)]))
            } else {
                None
            };
            match expr {
                Some(e) => push_stat(acc, "Range", vec![ProsePiece::Slot(e), ProsePiece::Text(" ft.".into())]),
                None => text_stat(ctx, acc, "Range", v, "range"),
            }
        }
        "MOVE" => {
            let parts: Vec<&str> = v.split(',').map(|s| s.trim()).collect();
            let mut pieces = Vec::new();
            for pair in parts.chunks(2) {
                if pair.len() < 2 {
                    break;
                }
                if !pieces.is_empty() {
                    pieces.push(ProsePiece::Text(", ".into()));
                }
                pieces.push(ProsePiece::Text(format!("{} ", pair[0])));
                pieces.push(number_piece(ctx, pair[1])?);
                pieces.push(ProsePiece::Text(" ft.".into()));
            }
            push_stat(acc, "Speed", pieces);
        }
        "MOVECLONE" => {
            // Row MOVECLONE (C23): `/k` divides, `*k` multiplies, `+k` or bare k adds.
            let parts: Vec<&str> = v.split(',').map(|s| s.trim()).collect();
            if parts.len() < 3 {
                return Err("MOVECLONE (shape)".into());
            }
            let base = Expr::Speed(parts[0].to_string());
            let arg = parts[2];
            let expr = if let Some(k) = arg.strip_prefix('/') {
                Expr::div(base, convert_formula(ctx, k)?)
            } else if let Some(k) = arg.strip_prefix('*') {
                Expr::mul(base, convert_formula(ctx, k)?)
            } else {
                Expr::sum(vec![base, convert_formula(ctx, arg.trim_start_matches('+'))?])
            };
            push_stat(acc, "Speed", vec![ProsePiece::Text(format!("{} ", parts[1])), ProsePiece::Slot(expr), ProsePiece::Text(" ft.".into())]);
        }
        "REACH" => {
            let piece = number_piece(ctx, v)?;
            push_stat(acc, "Reach", vec![piece, ProsePiece::Text(" ft.".into())]);
        }
        "REACHMULT" => text_stat(ctx, acc, "Reach multiplier", v, key),
        "VISION" => {
            let (fields, gates) = split_gates(v);
            let g = gates_of(ctx, &gates, level_gate)?;
            for entry in fields {
                let e = entry.trim();
                if e == ".CLEAR" || e.is_empty() {
                    continue;
                }
                let (name, range) = match e.find('(') {
                    Some(i) => (e[..i].trim().to_string(), Some(e[i + 1..].trim_end_matches(')').trim().to_string())),
                    None => (e.to_string(), None),
                };
                if pi_hit(&name).is_some() {
                    ctx.pi_term_hits.push("vision".into());
                    continue;
                }
                let mut pieces = vec![ProsePiece::Text(name)];
                if let Some(r) = range {
                    let r = r.trim_end_matches('\'').trim();
                    if !r.is_empty() {
                        pieces.push(ProsePiece::Text(" ".into()));
                        pieces.push(number_piece(ctx, r)?);
                        pieces.push(ProsePiece::Text(" ft.".into()));
                    }
                }
                let mut seg = ProseSegment { family: ProseFamily::StatBlock("Senses".into()), pieces, applies: None, pick_last: false, suppress_when_all_zero: false };
                if g != Applies::Always {
                    seg.applies = Some(g.clone());
                }
                acc.stat_block.push(seg);
            }
        }
        "HITDIE" => {
            let n = v.split('|').next().unwrap_or(v).trim();
            push_stat(acc, "Hit die", vec![ProsePiece::Text(format!("d{n}"))]);
        }
        "HD" => {
            if v.contains("TEMPLATE") {
                // Row HD (template band): the band gates a template grant.
                let (band, rest) = v.split_once(':').unwrap_or((v, ""));
                let (lo, hi) = band.split_once('-').unwrap_or((band, band));
                let name = rest.trim_start_matches("TEMPLATE:").trim();
                let id = ctx.resolve_kind("template", name).unwrap_or_else(|| slug(name));
                let when = Applies::all(vec![
                    Applies::Compare { lhs: Expr::HitDice, op: Cmp::Gte, rhs: Expr::Const(lo.trim().parse().unwrap_or(0)) },
                    Applies::Compare { lhs: Expr::HitDice, op: Cmp::Lte, rhs: Expr::Const(hi.trim().parse().unwrap_or(99)) },
                ]);
                out.grants_out.push((id, Grant { by: Granter::Rule(ctx.record.id.clone()), when }));
            } else {
                push_stat(acc, "Hit die", vec![ProsePiece::Text(format!("d{}", v.trim()))]);
            }
        }
        "ACCHECK" | "SPELLFAILURE" | "MAXDEX" => {
            let label = match key {
                "ACCHECK" => "Armor check penalty",
                "SPELLFAILURE" => "Arcane spell failure",
                _ => "Max Dex bonus",
            };
            let piece = number_piece(ctx, v)?;
            let mut pieces = vec![piece];
            if key == "SPELLFAILURE" {
                pieces.push(ProsePiece::Text("%".into()));
            }
            push_stat(acc, label, pieces);
        }
        "FOLLOWERS" => {
            let (role, formula) = v.split_once('|').unwrap_or((v, "1"));
            let count = convert_formula(ctx, formula)?;
            acc.grants.push(Effect::FactGrant(Fact::CompanionSlots { role: role.trim().to_string(), count }));
        }
        "TEMPVALUE" => {
            let mut min = Expr::Const(0);
            let mut max = Expr::Const(0);
            for p in v.split('|') {
                if let Some(m) = p.strip_prefix("MIN=") {
                    min = convert_formula(ctx, m)?;
                } else if let Some(m) = p.strip_prefix("MAX=") {
                    max = convert_formula(ctx, m)?;
                }
            }
            ctx.choice_id = Some(ctx.record.id.clone());
            acc.offers = Some(Choice { id: ctx.record.id.clone(), count: Expr::Const(1), from: OptionSet::Number { min, max } });
        }
        // ---- the bonus rows ----------------------------------------------------------------
        "BONUS" | "TEMPBONUS" => {
            let (mut fields, gates) = split_gates(v);
            let situational = if key == "TEMPBONUS" {
                // Row TEMPBONUS (C16): drop the target-scope field; the line is situational.
                if fields.is_empty() {
                    return Err("TEMPBONUS (shape)".into());
                }
                let scope = fields.remove(0);
                if scope.starts_with("EQ") && !fields.is_empty() {
                    // `EQ|<item>|<sub>|...`: the item name field precedes the sub.
                    fields.remove(0);
                }
                true
            } else {
                false
            };
            if fields.len() < 3 {
                return Err(format!("{key} (shape)"));
            }
            let sub_field = fields[0].clone();
            let (sub, sub_arg) = sub_field.split_once('=').map(|(a, b)| (a.trim().to_string(), Some(b.trim().to_string()))).unwrap_or((sub_field.trim().to_string(), None));
            let target = fields[1].clone();
            let formula = fields[2].clone();
            let bonus_type = fields.iter().skip(3).find_map(|f| parse_bonus_type(f));
            let when = gates_of(ctx, &gates, level_gate)?;
            let when = if situational { Applies::all(vec![when, Applies::Situational { text: "when active".into() }]) } else { when };
            match sub.as_str() {
                "VAR" => {
                    // Row BONUS:VAR: a typed contribution to a corpus variable, never a sheet line.
                    let expr = convert_formula(ctx, &formula)?;
                    if target.contains("[redacted") {
                        return Err("BONUS:VAR ([redacted PI] value)".into());
                    }
                    for name in target.split(',') {
                        let id = super::ctx::var_id(name);
                        out.var_contribs.push((id, name.trim().to_ascii_uppercase(), VarContribution { rule_id: ctx.record.id.clone(), expr: expr.clone(), bonus_type: bonus_type.clone(), when: when.clone() }));
                    }
                }
                "SLOTS" | "FOLLOWERS" | "MONSKILLPTS" => {}
                "WEAPONPROF" => {
                    // Row BONUS:WEAPONPROF=<name>: the property named by the second field.
                    let weapon = weapon_ref(ctx, sub_arg.as_deref().unwrap_or(""));
                    let expr = convert_formula(ctx, &formula)?;
                    let prop = target.to_ascii_uppercase();
                    let (bt, words) = match prop.as_str() {
                        "DAMAGE" => (BonusTarget::Damage(weapon.clone()), "damage".to_string()),
                        "TOHIT" => (BonusTarget::WeaponAttack(weapon.clone()), "attack".to_string()),
                        "DAMAGESIZE" => (BonusTarget::DamageSize(weapon.clone()), "damage die steps".to_string()),
                        other => (BonusTarget::Other(format!("weapon {}", other.to_ascii_lowercase())), other.to_ascii_lowercase()),
                    };
                    let wname = match &weapon {
                        WeaponRef::Named(n) => n.clone(),
                        WeaponRef::Group(g) => format!("{g} weapons"),
                        _ => "chosen weapon".into(),
                    };
                    acc.lines.push(Line { suffix: Some(format!("weapon{}", acc.lines.len())), label: format!("{wname} {words}"), value: SheetValue::Number(expr), also: Vec::new(), target: Some(bt), bonus_type: bonus_type.clone(), applies: when, prose: Vec::new() });
                }
                other => {
                    let expr = convert_formula(ctx, &formula)?;
                    let targets = bonus_targets(ctx, other, &target)?;
                    for (bt, words) in targets {
                        acc.lines.push(Line { suffix: Some(format!("bonus{}", acc.lines.len())), label: format!("{} ({words})", ctx.record.name), value: SheetValue::Number(expr.clone()), also: Vec::new(), target: Some(bt), bonus_type: bonus_type.clone(), applies: when.clone(), prose: Vec::new() });
                    }
                }
            }
        }
        // ---- spell-like abilities -------------------------------------------------------------
        "SPELLS" => {
            // Row SPELLS: one line per spell entry with uses / caster level / DC.
            let (fields, gates) = split_gates(v);
            let when = gates_of(ctx, &gates, level_gate)?;
            let mut times: Option<String> = None;
            let mut unit: String = "day".into();
            let mut cl: Option<String> = None;
            let mut entries: Vec<String> = Vec::new();
            for (i, f) in fields.iter().enumerate() {
                if i == 0 {
                    continue; // spellbook name
                }
                if let Some(t) = f.strip_prefix("TIMES=") {
                    times = Some(t.trim().to_string());
                } else if let Some(u) = f.strip_prefix("TIMEUNIT=") {
                    unit = u.trim().to_ascii_lowercase();
                } else if let Some(c) = f.strip_prefix("CASTERLEVEL=") {
                    cl = Some(c.trim().to_string());
                } else if let Some(c) = f.strip_prefix("DC=") {
                    // Rare alternate DC field: applies to every entry.
                    entries.push(format!(",{c}"));
                } else {
                    entries.push(f.trim().to_string());
                }
            }
            let cl_expr = match cl.as_deref() {
                Some(c) => Some(convert_formula(ctx, c)?),
                None => None,
            };
            let at_will = times.as_deref().is_some_and(|t| t.eq_ignore_ascii_case("ATWILL")) || unit == "constant";
            let uses_expr = if at_will {
                None
            } else {
                match times.as_deref() {
                    Some(t) => Some(convert_formula(ctx, t)?),
                    None => Some(Expr::Const(1)),
                }
            };
            for (i, entry) in entries.iter().enumerate() {
                if entry.starts_with(',') || entry.is_empty() {
                    continue;
                }
                let (spell, dc) = match entry.split_once(',') {
                    Some((s, d)) => (s.trim().to_string(), Some(d.trim().to_string())),
                    None => (entry.trim().to_string(), None),
                };
                if pi_hit(&spell).is_some() {
                    ctx.pi_term_hits.push("spell-like ability".into());
                    continue;
                }
                let mut also = Vec::new();
                if let Some(u) = &uses_expr {
                    also.push((ValueRole::Uses { period: unit.clone() }, SheetValue::Number(u.clone())));
                }
                if let Some(c) = &cl_expr {
                    also.push((ValueRole::CasterLevel, SheetValue::Number(c.clone())));
                }
                if let Some(d) = dc {
                    also.push((ValueRole::SaveDc, SheetValue::Number(convert_formula(ctx, &d)?)));
                }
                let mut prose = Vec::new();
                let value = match &uses_expr {
                    Some(u) => SheetValue::Number(u.clone()),
                    None => {
                        prose.push(ProseSegment { family: ProseFamily::Special, pieces: vec![ProsePiece::Text(if unit == "constant" { "Constant".into() } else { "At will".into() })], applies: None, pick_last: false, suppress_when_all_zero: false });
                        SheetValue::Text
                    }
                };
                acc.lines.push(Line { suffix: Some(format!("spell{}_{}", acc.lines.len(), slug(&spell))), label: spell.clone(), value, also, target: None, bonus_type: None, applies: when.clone(), prose });
                let _ = i;
            }
        }
        // ---- ownership / choice rows ------------------------------------------------------------
        "ABILITY" => {
            // Row ABILITY: a grant edge from this record to each target.
            let (fields, gates) = split_gates(v);
            if fields.len() < 3 {
                return Err("ABILITY (shape)".into());
            }
            let category = fields[0].trim().to_string();
            let nature = fields[1].trim().to_ascii_uppercase();
            let when = gates_of(ctx, &gates, level_gate)?;
            for target in fields.iter().skip(2) {
                let t = target.trim();
                if t.is_empty() || t == ".CLEAR" {
                    continue;
                }
                if t.contains("%LIST") || t.contains("%CHOICE") {
                    continue; // the holder's choice grants the picked option (Choice lane)
                }
                if t.starts_with("TYPE=") || t.starts_with("TYPE.") {
                    ctx.defect("grant-by-type", format!("{}: {category}|{t}", ctx.record.id));
                    continue;
                }
                let by = if nature == "NORMAL" { Granter::Choice(ctx.record.id.clone()) } else { Granter::Rule(ctx.record.id.clone()) };
                let by = match level_gate {
                    Some(l) if ctx.record.kind == "class" => Granter::Class { id: ctx.owning_class.clone().unwrap_or_else(|| slug(&ctx.record.key)), at_level: l },
                    _ => by,
                };
                if let Holdable::Rule(id) = resolve_holdable_rule(ctx, &category, t) {
                    out.grants_out.push((id, Grant { by, when: when.clone() }));
                }
            }
        }
        "AUTO" => {
            let (fields, gates) = split_gates(v);
            let when = gates_of(ctx, &gates, level_gate)?;
            let head = fields.first().map(|s| s.trim().to_ascii_uppercase()).unwrap_or_default();
            let items: Vec<String> = fields.iter().skip(1).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            let choice = ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone());
            let facts: Vec<Fact> = match head.as_str() {
                "LANG" => items.into_iter().map(|l| if l.contains("%LIST") { Fact::Chosen(choice.clone()) } else { Fact::Language(tag_word(&l)) }).collect(),
                "WEAPONPROF" => items
                    .into_iter()
                    .map(|w| {
                        if w.contains("%LIST") {
                            Fact::Proficiency(ProfRef::Chosen(choice.clone()))
                        } else if w.starts_with("TYPE=") || w.starts_with("TYPE.") {
                            Fact::Proficiency(ProfRef::WeaponGroup(tag_word(&w)))
                        } else if w.eq_ignore_ascii_case("DEITYWEAPONS") {
                            Fact::Proficiency(ProfRef::DeityFavoredWeapon)
                        } else {
                            Fact::Proficiency(ProfRef::Weapon(tag_word(&w)))
                        }
                    })
                    .collect(),
                "ARMORPROF" => items.into_iter().map(|a| if a.contains("%LIST") { Fact::Chosen(choice.clone()) } else { Fact::Proficiency(ProfRef::ArmorGroup(tag_word(&a))) }).collect(),
                "SHIELDPROF" => items.into_iter().map(|a| if a.contains("%LIST") { Fact::Chosen(choice.clone()) } else { Fact::Proficiency(ProfRef::ShieldGroup(tag_word(&a))) }).collect(),
                "EQUIP" => items.into_iter().map(|e| Fact::Equipment(tag_word(&e))).collect(),
                _ => return Err(format!("AUTO ({head})")),
            };
            for f in facts {
                acc.grants.push(Effect::FactGrant(f));
            }
            let _ = when;
        }
        "CSKILL" | "CCSKILL" | "MONCSKILL" => {
            let (fields, _gates) = split_gates(v);
            let choice = ctx.choice_id.clone().unwrap_or_else(|| ctx.record.id.clone());
            for s in fields {
                let s = s.trim();
                if s == ".CLEAR" || s.is_empty() {
                    continue;
                }
                let fact = if s.contains("%LIST") || s.eq_ignore_ascii_case("LIST") {
                    Fact::ClassSkillChosen(choice.clone())
                } else if let Some(g) = s.strip_prefix("TYPE=").or_else(|| s.strip_prefix("TYPE.")) {
                    Fact::ClassSkillGroup(g.to_string())
                } else if s.eq_ignore_ascii_case("ALL") {
                    Fact::ClassSkillGroup("All".into())
                } else if key == "CCSKILL" {
                    Fact::CrossClassSkill(ctx.skill_id(s))
                } else {
                    Fact::ClassSkill(ctx.skill_id(s))
                };
                acc.grants.push(Effect::FactGrant(fact));
            }
        }
        "CLASSES" => {
            // Row CLASSES (C22): spell-level grant on spell-shaped records; class-skill on skills.
            let (fields, _gates) = split_gates(v);
            if ctx.record.kind == "skill" {
                for c in fields.iter().flat_map(|f| f.split(',')) {
                    let c = c.trim();
                    if c.is_empty() || c.eq_ignore_ascii_case("ALL") || c.starts_with('!') {
                        continue;
                    }
                    acc.granted_by.push(Grant { by: Granter::Class { id: ctx.class_id(c), at_level: 1 }, when: Applies::Always });
                }
                return Ok(());
            }
            for group in fields {
                let g = group.trim();
                if g.is_empty() || g == ".CLEAR" {
                    continue;
                }
                let (body, bracket) = match g.find('[') {
                    Some(i) => (&g[..i], Some(g[i + 1..].trim_end_matches(']').to_string())),
                    None => (g, None),
                };
                let when = match bracket {
                    Some(b) => convert_pre_token(ctx, &b)?,
                    None => Applies::Always,
                };
                let (classes, level) = body.split_once('=').unwrap_or((body, "1"));
                let level: u8 = level.trim().parse().unwrap_or(0);
                for c in classes.split(',') {
                    let c = c.trim();
                    if c.is_empty() {
                        continue;
                    }
                    acc.granted_by.push(Grant { by: Granter::ClassSpellList { id: ctx.class_id(c), spell_level: level }, when: when.clone() });
                }
            }
        }
        "DOMAIN" => {
            // Row `class LEVEL grants` (B5): a numbered DOMAIN line -- the class offers the domain.
            if row_kind != ClosureRowKind::LevelLine {
                return Err("unmapped:DOMAIN".into());
            }
            let (fields, gates) = split_gates(v);
            let when = gates_of(ctx, &gates, level_gate)?;
            let cls = ctx.owning_class.clone().unwrap_or_else(|| slug(&ctx.record.key));
            for d in fields.iter().flat_map(|f| f.split(',')) {
                let d = d.trim();
                if d.is_empty() {
                    continue;
                }
                let id = ctx.resolve_kind("domain", d).unwrap_or_else(|| slug(d));
                out.grants_out.push((id, Grant { by: Granter::Class { id: cls.clone(), at_level: level_gate.unwrap_or(1) }, when: when.clone() }));
            }
        }
        "DOMAINS" => {
            let (fields, gates) = split_gates(v);
            let when = gates_of(ctx, &gates, level_gate)?;
            for d in fields.iter().flat_map(|f| f.split(',')) {
                let d = d.trim();
                if d.is_empty() {
                    continue;
                }
                let id = ctx.resolve_kind("domain", d).unwrap_or_else(|| slug(d));
                out.grants_out.push((id, Grant { by: Granter::Deity(ctx.record.id.clone()), when: when.clone() }));
            }
        }
        "TEMPLATE" => {
            let (fields, gates) = split_gates(v);
            let when = gates_of(ctx, &gates, level_gate)?;
            let mut names: Vec<String> = fields.iter().map(|s| s.trim().to_string()).collect();
            if names.first().is_some_and(|n| n.starts_with("CHOOSE:")) {
                let first = names.remove(0);
                names.insert(0, first.trim_start_matches("CHOOSE:").to_string());
                let ids: Vec<RuleId> = names.iter().map(|n| ctx.resolve_kind("template", n).unwrap_or_else(|| slug(n))).collect();
                ctx.choice_id = Some(ctx.record.id.clone());
                acc.offers = Some(Choice { id: ctx.record.id.clone(), count: Expr::Const(1), from: OptionSet::Templates(ids) });
                return Ok(());
            }
            for n in names {
                if n.contains("%LIST") || n.is_empty() {
                    continue;
                }
                if let Some(base) = n.strip_suffix(".REMOVE") {
                    let id = ctx.resolve_kind("template", base).unwrap_or_else(|| slug(base));
                    acc.grants.push(Effect::Revokes(id));
                    continue;
                }
                let id = ctx.resolve_kind("template", &n).unwrap_or_else(|| slug(&n));
                let by = match level_gate {
                    Some(l) if ctx.record.kind == "class" => Granter::Class { id: ctx.owning_class.clone().unwrap_or_else(|| slug(&ctx.record.key)), at_level: l },
                    _ => Granter::Rule(ctx.record.id.clone()),
                };
                out.grants_out.push((id, Grant { by, when: when.clone() }));
            }
        }
        "SERVESAS" => {
            // Row SERVESAS (C22): counts as a rule, a class, or a race.
            let (fields, _gates) = split_gates(v);
            let head = fields.first().cloned().unwrap_or_default();
            if let Some(cat) = head.strip_prefix("ABILITY=") {
                for n in fields.iter().skip(1) {
                    if let Holdable::Rule(id) = resolve_holdable_rule(ctx, cat, n) {
                        acc.grants.push(Effect::CountsAs(CountsAs::Rule(id)));
                    }
                }
            } else if head == "CLASS" {
                for n in fields.iter().skip(1) {
                    acc.grants.push(Effect::CountsAs(CountsAs::Class(ctx.class_id(n))));
                }
            } else if head == "RACE" {
                for n in fields.iter().skip(1) {
                    let id = ctx.resolve_kind("race", n).unwrap_or_else(|| slug(n));
                    acc.grants.push(Effect::CountsAs(CountsAs::Race(id)));
                }
            }
        }
        "QUALIFY" => {
            let (fields, _gates) = split_gates(v);
            let head = fields.first().cloned().unwrap_or_default();
            if let Some(cat) = head.strip_prefix("ABILITY=") {
                for n in fields.iter().skip(1) {
                    if let Holdable::Rule(id) = resolve_holdable_rule(ctx, cat, n) {
                        acc.grants.push(Effect::Waives(id));
                    }
                }
            }
        }
        "REMOVE" => {
            let (fields, _gates) = split_gates(v);
            let head = fields.first().cloned().unwrap_or_default();
            let cat = if head.eq_ignore_ascii_case("FEAT") { "FEAT".to_string() } else { head.strip_prefix("ABILITY=").unwrap_or(&head).to_string() };
            for n in fields.iter().skip(1) {
                if n.trim().parse::<u8>().is_ok() {
                    continue;
                }
                if let Holdable::Rule(id) = resolve_holdable_rule(ctx, &cat, n) {
                    acc.grants.push(Effect::Revokes(id));
                }
            }
        }
        "ADD" => {
            let (fields, _gates) = split_gates(v);
            let head = fields.first().map(|s| s.trim().to_ascii_uppercase()).unwrap_or_default();
            match head.as_str() {
                "SPELLCASTER" => {
                    let who = fields.get(1).cloned().unwrap_or_default();
                    if pi_hit(&who).is_none() {
                        acc.special.push(ProseSegment { family: ProseFamily::Special, pieces: vec![ProsePiece::Text(format!("Casts spells as a {who}."))], applies: None, pick_last: false, suppress_when_all_zero: false });
                    }
                }
                "LANGUAGE" | "FEAT" | "VFEAT" | "ABILITY" => {
                    let count = fields.get(1).and_then(|c| integer_literal(c)).map(Expr::Const).unwrap_or(Expr::Const(1));
                    let from = match head.as_str() {
                        "LANGUAGE" => OptionSet::Languages(fields.iter().skip(2).flat_map(|f| f.split(',')).filter(|s| !s.contains('%')).map(tag_word).collect()),
                        "ABILITY" => OptionSet::Rules { pool: slug(fields.get(2).map(|s| s.as_str()).unwrap_or("")), tags: fields.iter().skip(4).filter_map(|f| f.strip_prefix("TYPE=").map(|t| t.to_string())).collect(), requires: Applies::Always },
                        _ => OptionSet::Rules { pool: "feat".into(), tags: fields.iter().skip(2).filter_map(|f| f.strip_prefix("TYPE=").map(|t| t.to_string())).collect(), requires: Applies::Always },
                    };
                    ctx.choice_id = Some(ctx.record.id.clone());
                    acc.offers = Some(Choice { id: ctx.record.id.clone(), count, from });
                }
                "EQUIP" => {}
                other => return Err(format!("ADD ({other})")),
            }
        }
        "CHOOSE" => {
            // Row CHOOSE: the record offers a choice; the option set by selector.
            let (fields, gates) = split_gates(v);
            let selector = fields.first().map(|s| s.trim().to_string()).unwrap_or_default();
            let sel_head = selector.split('=').next().unwrap_or("").to_ascii_uppercase();
            let args: Vec<String> = fields.iter().skip(1).map(|s| s.trim().to_string()).collect();
            let requires = gates_of(ctx, &gates, None)?;
            let from = match sel_head.as_str() {
                "NOCHOICE" => return Ok(()),
                "NUMCHOICES" => {
                    let n = selector.split('=').nth(1).unwrap_or("1");
                    let count = if let Some(c) = integer_literal(n) { Expr::Const(c) } else { convert_formula(ctx, n)? };
                    acc.select_count = Some(count);
                    // The real selector follows in the args; fall through by re-dispatch.
                    let inner = args.join("|");
                    if inner.is_empty() {
                        return Ok(());
                    }
                    return convert_token(ctx, acc, out, "CHOOSE", &inner, level_gate, row_kind);
                }
                "ABILITYSELECTION" | "ABILITY" => {
                    let pool = slug(args.first().map(|s| s.as_str()).unwrap_or(""));
                    let tags: Vec<String> = args.iter().skip(1).filter_map(|a| a.strip_prefix("TYPE=").or_else(|| a.strip_prefix("TYPE.")).map(|t| t.to_string())).collect();
                    OptionSet::Rules { pool, tags, requires }
                }
                "SKILL" | "SKILLBONUS" => OptionSet::Skills(args.iter().filter(|a| !a.starts_with("TYPE") && !a.contains('%')).map(|a| ctx.skill_id(a)).collect()),
                "WEAPONPROFICIENCY" => OptionSet::Weapons(args.iter().filter(|a| !a.contains('%')).map(|a| tag_word(a)).collect()),
                "SHIELDPROFICIENCY" | "ARMORPROFICIENCY" => OptionSet::Weapons(args.iter().filter(|a| !a.contains('%')).map(|a| tag_word(a)).collect()),
                "LANG" => OptionSet::Languages(args.iter().filter(|a| !a.contains('%')).map(|a| tag_word(a)).collect()),
                "SCHOOLS" => OptionSet::Schools,
                "CLASS" => OptionSet::Classes(args.iter().map(|a| ctx.class_id(a)).collect()),
                "TEMPLATE" => OptionSet::Templates(args.iter().map(|n| ctx.resolve_kind("template", n).unwrap_or_else(|| slug(n))).collect()),
                "EQUIPMENT" | "EQBUILDER.SPELL" => OptionSet::Equipment,
                "PCSTAT" | "STATBONUS" => OptionSet::FreeText,
                "SPELLS" => {
                    let mut class = String::new();
                    let mut lo = 0u8;
                    let mut hi = 9u8;
                    for a in &args {
                        if let Some(rest) = a.strip_prefix("CLASSLIST=") {
                            let (c, opts) = rest.split_once('[').unwrap_or((rest, ""));
                            class = c.trim().to_string();
                            for o in opts.trim_end_matches(']').split(';') {
                                if let Some(x) = o.strip_prefix("LEVELMIN=") {
                                    lo = x.parse().unwrap_or(0);
                                } else if let Some(x) = o.strip_prefix("LEVELMAX=") {
                                    hi = x.parse().unwrap_or(9);
                                }
                            }
                        }
                    }
                    OptionSet::Spells { class: ctx.class_id(&class), levels: (lo, hi) }
                }
                "STRING" | "USERINPUT" => OptionSet::FreeText,
                "NUMBER" => {
                    let mut min = Expr::Const(0);
                    let mut max = Expr::Const(0);
                    for a in &args {
                        if let Some(m) = a.strip_prefix("MIN=") {
                            min = convert_formula(ctx, m)?;
                        } else if let Some(m) = a.strip_prefix("MAX=") {
                            max = convert_formula(ctx, m)?;
                        }
                    }
                    OptionSet::Number { min, max }
                }
                "DEITY" => OptionSet::Deities,
                "DOMAIN" => OptionSet::Domains,
                "RACE" => OptionSet::Races(args.iter().map(|n| ctx.resolve_kind("race", n).unwrap_or_else(|| slug(n))).collect()),
                other => return Err(format!("CHOOSE ({other})")),
            };
            ctx.choice_id = Some(ctx.record.id.clone());
            let count = acc.select_count.clone().unwrap_or(Expr::Const(1));
            acc.offers = Some(Choice { id: ctx.record.id.clone(), count, from });
        }
        "SELECT" => {
            let count = if let Some(c) = integer_literal(v) { Expr::Const(c) } else { convert_formula(ctx, v)? };
            acc.select_count = Some(count);
        }
        "LANGBONUS" => {
            let langs: Vec<String> = v.split(',').map(tag_word).filter(|s| !s.is_empty() && s != ".CLEAR" && !s.contains('%')).collect();
            if acc.offers.is_none() {
                acc.offers = Some(Choice { id: ctx.record.id.clone(), count: Expr::max(Expr::Const(0), Expr::AbilityMod(Ability::Int)), from: OptionSet::Languages(langs) });
            }
        }
        "COMPANIONLIST" => {
            let (fields, gates) = split_gates(v);
            let _when = gates_of(ctx, &gates, level_gate)?;
            let role = fields.first().cloned().unwrap_or_default();
            let races: Vec<RuleId> = fields.iter().skip(1).flat_map(|f| f.split(',')).filter(|s| !s.starts_with("FOLLOWERADJUSTMENT")).map(|n| ctx.resolve_kind("race", n.trim()).or_else(|| ctx.resolve_kind("monster", n.trim())).unwrap_or_else(|| slug(n.trim()))).collect();
            acc.offers.get_or_insert(Choice { id: ctx.record.id.clone(), count: Expr::Const(1), from: OptionSet::Races(races) });
            let _ = role;
        }
        other => return Err(format!("unmapped:{other}")),
    }
    Ok(())
}
