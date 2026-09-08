//! The FORMULA rows: source arithmetic -> our `Expr`.
//!
//! The converter carries its own parser for the source formula grammar (integer and decimal
//! literals, `+ - * /`, parentheses, unary minus, comparisons as 0/1 values, `&&`/`||`, the
//! function calls the corpus uses) and lowers the AST to `Expr` by the FORMULA rows of the
//! mapping table (`table.rs`): division is exact, `floor()`/`ceil()` are explicit,
//! `if()` and comparisons use the exact `Min`/`Max` encoding (`SYNTHESIS.md` C17), an ability
//! abbreviation is the modifier, `TL` is `Level`, `CL` is the owning class's level, a corpus
//! variable resolves through `RecordCtx::resolve_variable` (rows 27/28/30, C1/C2), and every
//! shape outside the table refuses by name.
//!
//! Position matters for one rule only: an identifier DEFINEd nowhere is `Const(0)` in a bare
//! or additive position and a refusal inside a function call or under `*` / `/` (row 30).

use super::ctx::RecordCtx;
use crate::rules_core::sheet_rule::{Ability, ClassRef, Expr, HeldFilter, SpellKind};

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(String),
    Ident(String),
    Str(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    Ge,
    Le,
    Eq,
    Ne,
    Gt,
    Lt,
    AndAnd,
    OrOr,
}

fn tokenize(s: &str) -> Result<Vec<Tok>, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' => i += 1,
            '+' => {
                out.push(Tok::Plus);
                i += 1;
            }
            '-' => {
                out.push(Tok::Minus);
                i += 1;
            }
            '*' => {
                out.push(Tok::Star);
                i += 1;
            }
            '/' => {
                out.push(Tok::Slash);
                i += 1;
            }
            '(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            ',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            '>' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Ge);
                    i += 2;
                } else {
                    out.push(Tok::Gt);
                    i += 1;
                }
            }
            '<' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Le);
                    i += 2;
                } else {
                    out.push(Tok::Lt);
                    i += 1;
                }
            }
            '=' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Eq);
                    i += 2;
                } else {
                    return Err("FORMULA:malformed (parser refusals)".into());
                }
            }
            '!' => {
                if chars.get(i + 1) == Some(&'=') {
                    out.push(Tok::Ne);
                    i += 2;
                } else {
                    return Err("FORMULA:malformed (parser refusals)".into());
                }
            }
            '&' => {
                if chars.get(i + 1) == Some(&'&') {
                    out.push(Tok::AndAnd);
                    i += 2;
                } else {
                    return Err("FORMULA:malformed (parser refusals)".into());
                }
            }
            '|' => {
                if chars.get(i + 1) == Some(&'|') {
                    out.push(Tok::OrOr);
                    i += 2;
                } else {
                    return Err("FORMULA:malformed (parser refusals)".into());
                }
            }
            '"' => {
                let mut j = i + 1;
                let mut s = String::new();
                while j < chars.len() && chars[j] != '"' {
                    s.push(chars[j]);
                    j += 1;
                }
                if j >= chars.len() {
                    return Err("FORMULA:malformed (parser refusals)".into());
                }
                out.push(Tok::Str(s));
                i = j + 1;
            }
            c if c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit())) => {
                let mut j = i;
                let mut s = String::new();
                while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                    s.push(chars[j]);
                    j += 1;
                }
                out.push(Tok::Num(s));
                i = j;
            }
            c if c.is_ascii_alphabetic() || c == '_' || c == '%' => {
                let mut j = i;
                let mut s = String::new();
                while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_' || chars[j] == '%' || chars[j] == '.') {
                    s.push(chars[j]);
                    j += 1;
                }
                out.push(Tok::Ident(s));
                i = j;
            }
            _ => return Err("FORMULA:malformed (parser refusals)".into()),
        }
    }
    Ok(out)
}

#[derive(Debug, Clone, PartialEq)]
enum Ast {
    Num(String),
    Ident(String),
    Str(String),
    Neg(Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Call(String, Vec<Ast>),
    Cmp(CmpOp, Box<Ast>, Box<Ast>),
    And(Box<Ast>, Box<Ast>),
    Or(Box<Ast>, Box<Ast>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CmpOp {
    Ge,
    Le,
    Eq,
    Ne,
    Gt,
    Lt,
}

struct Parser<'a> {
    toks: &'a [Tok],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }
    fn bump(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.pos).cloned();
        self.pos += 1;
        t
    }
    fn expect(&mut self, t: &Tok) -> Result<(), String> {
        if self.peek() == Some(t) {
            self.pos += 1;
            Ok(())
        } else {
            Err("FORMULA:malformed (parser refusals)".into())
        }
    }

    fn parse_or(&mut self) -> Result<Ast, String> {
        let mut lhs = self.parse_and()?;
        while self.peek() == Some(&Tok::OrOr) {
            self.bump();
            let rhs = self.parse_and()?;
            lhs = Ast::Or(Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_and(&mut self) -> Result<Ast, String> {
        let mut lhs = self.parse_cmp()?;
        while self.peek() == Some(&Tok::AndAnd) {
            self.bump();
            let rhs = self.parse_cmp()?;
            lhs = Ast::And(Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_cmp(&mut self) -> Result<Ast, String> {
        let lhs = self.parse_expr()?;
        let op = match self.peek() {
            Some(Tok::Ge) => CmpOp::Ge,
            Some(Tok::Le) => CmpOp::Le,
            Some(Tok::Eq) => CmpOp::Eq,
            Some(Tok::Ne) => CmpOp::Ne,
            Some(Tok::Gt) => CmpOp::Gt,
            Some(Tok::Lt) => CmpOp::Lt,
            _ => return Ok(lhs),
        };
        self.bump();
        let rhs = self.parse_expr()?;
        Ok(Ast::Cmp(op, Box::new(lhs), Box::new(rhs)))
    }

    fn parse_expr(&mut self) -> Result<Ast, String> {
        let mut lhs = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.bump();
                    let rhs = self.parse_term()?;
                    lhs = Ast::Add(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Minus) => {
                    self.bump();
                    let rhs = self.parse_term()?;
                    lhs = Ast::Sub(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn parse_term(&mut self) -> Result<Ast, String> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.bump();
                    let rhs = self.parse_unary()?;
                    lhs = Ast::Mul(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Slash) => {
                    self.bump();
                    let rhs = self.parse_unary()?;
                    lhs = Ast::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn parse_unary(&mut self) -> Result<Ast, String> {
        match self.peek() {
            Some(Tok::Minus) => {
                self.bump();
                Ok(Ast::Neg(Box::new(self.parse_unary()?)))
            }
            Some(Tok::Plus) => {
                self.bump();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Ast, String> {
        match self.bump() {
            Some(Tok::Num(n)) => Ok(Ast::Num(n)),
            Some(Tok::Str(s)) => Ok(Ast::Str(s)),
            Some(Tok::LParen) => {
                let v = self.parse_or()?;
                self.expect(&Tok::RParen)?;
                Ok(v)
            }
            Some(Tok::Ident(name)) => {
                if self.peek() == Some(&Tok::LParen) {
                    self.bump();
                    let mut args = Vec::new();
                    if self.peek() != Some(&Tok::RParen) {
                        loop {
                            args.push(self.parse_or()?);
                            if self.peek() == Some(&Tok::Comma) {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(&Tok::RParen)?;
                    Ok(Ast::Call(name, args))
                } else {
                    Ok(Ast::Ident(name))
                }
            }
            _ => Err("FORMULA:malformed (parser refusals)".into()),
        }
    }
}

fn parse(formula: &str) -> Result<Ast, String> {
    let toks = tokenize(formula)?;
    if toks.is_empty() {
        return Err("FORMULA:malformed (parser refusals)".into());
    }
    let mut p = Parser { toks: &toks, pos: 0 };
    let ast = p.parse_or()?;
    if p.pos != toks.len() {
        return Err("FORMULA:malformed (parser refusals)".into());
    }
    Ok(ast)
}

/// A decimal literal as an exact rational `Expr` (`0.5` -> `Div(1, 2)`, `3` -> `Const(3)`).
fn literal(text: &str) -> Result<Expr, String> {
    if let Ok(i) = text.parse::<i64>() {
        return Ok(Expr::Const(i.clamp(i32::MIN as i64, i32::MAX as i64) as i32));
    }
    let (int_part, frac_part) = text.split_once('.').ok_or("FORMULA:malformed (parser refusals)")?;
    let int_part = if int_part.is_empty() { "0" } else { int_part };
    let frac = frac_part.trim_end_matches('0');
    if frac.is_empty() {
        return literal(int_part);
    }
    let den: i64 = 10i64.pow(frac.len() as u32);
    let num: i64 = int_part.parse::<i64>().map_err(|_| "FORMULA:malformed (parser refusals)")? * den
        + frac.parse::<i64>().map_err(|_| "FORMULA:malformed (parser refusals)")?;
    let g = gcd(num, den);
    Ok(Expr::div(Expr::Const((num / g) as i32), Expr::Const((den / g) as i32)))
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

pub fn ability(abbr: &str) -> Option<Ability> {
    match abbr.trim().to_ascii_uppercase().as_str() {
        "STR" | "STRENGTH" => Some(Ability::Str),
        "DEX" | "DEXTERITY" => Some(Ability::Dex),
        "CON" | "CONSTITUTION" => Some(Ability::Con),
        "INT" | "INTELLIGENCE" => Some(Ability::Int),
        "WIS" | "WISDOM" => Some(Ability::Wis),
        "CHA" | "CHARISMA" => Some(Ability::Cha),
        _ => None,
    }
}

/// The exact 0/1 encoding of `lhs >= rhs` for integer-valued operands (C17).
pub fn cmp_ge(lhs: Expr, rhs: Expr) -> Expr {
    Expr::min(Expr::Const(1), Expr::max(Expr::Const(0), Expr::sum(vec![lhs, Expr::neg(rhs), Expr::Const(1)])))
}

fn one_minus(e: Expr) -> Expr {
    Expr::sum(vec![Expr::Const(1), Expr::neg(e)])
}

pub fn cmp_expr(op: crate::rules_core::sheet_rule::Cmp, lhs: Expr, rhs: Expr) -> Expr {
    use crate::rules_core::sheet_rule::Cmp;
    match op {
        Cmp::Gte => cmp_ge(lhs, rhs),
        Cmp::Gt => cmp_ge(lhs, Expr::sum(vec![rhs, Expr::Const(1)])),
        Cmp::Lt => one_minus(cmp_ge(lhs, rhs)),
        Cmp::Lte => one_minus(cmp_ge(lhs, Expr::sum(vec![rhs, Expr::Const(1)]))),
        Cmp::Eq => Expr::min(cmp_ge(lhs.clone(), rhs.clone()), one_minus(cmp_ge(lhs, Expr::sum(vec![rhs, Expr::Const(1)])))),
        Cmp::Ne => one_minus(cmp_expr(Cmp::Eq, lhs, rhs)),
    }
}

fn lower(ctx: &mut RecordCtx, ast: &Ast, strict: bool) -> Result<Expr, String> {
    use crate::rules_core::sheet_rule::Cmp;
    Ok(match ast {
        Ast::Num(n) => literal(n)?,
        Ast::Str(_) => return Err("FORMULA:malformed (parser refusals)".into()),
        Ast::Ident(name) => lower_ident(ctx, name, strict)?,
        Ast::Neg(a) => Expr::neg(lower(ctx, a, strict)?),
        Ast::Add(a, b) => Expr::sum(vec![lower(ctx, a, strict)?, lower(ctx, b, strict)?]),
        Ast::Sub(a, b) => Expr::sum(vec![lower(ctx, a, strict)?, Expr::neg(lower(ctx, b, strict)?)]),
        Ast::Mul(a, b) => Expr::mul(lower(ctx, a, true)?, lower(ctx, b, true)?),
        Ast::Div(a, b) => Expr::div(lower(ctx, a, true)?, lower(ctx, b, true)?),
        Ast::Cmp(op, a, b) => {
            let op = match op {
                CmpOp::Ge => Cmp::Gte,
                CmpOp::Le => Cmp::Lte,
                CmpOp::Eq => Cmp::Eq,
                CmpOp::Ne => Cmp::Ne,
                CmpOp::Gt => Cmp::Gt,
                CmpOp::Lt => Cmp::Lt,
            };
            cmp_expr(op, lower(ctx, a, true)?, lower(ctx, b, true)?)
        }
        Ast::And(a, b) => Expr::min(lower(ctx, a, true)?, lower(ctx, b, true)?),
        Ast::Or(a, b) => Expr::max(lower(ctx, a, true)?, lower(ctx, b, true)?),
        Ast::Call(name, args) => lower_call(ctx, name, args)?,
    })
}

fn lower_ident(ctx: &mut RecordCtx, name: &str, strict: bool) -> Result<Expr, String> {
    let upper = name.to_ascii_uppercase();
    if let Some(a) = ability(&upper) {
        return Ok(Expr::AbilityMod(a));
    }
    if let Some(stripped) = upper.strip_suffix("SCORE")
        && let Some(a) = ability(stripped)
    {
        return Ok(Expr::AbilityScore(a));
    }
    match upper.as_str() {
        "TL" => return Ok(Expr::Level),
        "CL" => return owning_class_level(ctx),
        "HD" => return Ok(Expr::HitDice),
        "BAB" => return Ok(Expr::BaseAttack),
        "SIZE" => return Ok(Expr::Size),
        "SIZEMOD" => return Ok(Expr::SizeMod),
        "CR" => return Ok(Expr::ChallengeRating),
        "CASTERLEVEL" => return Ok(Expr::CasterLevel(ClassRef::Holder)),
        "SPELLLEVEL" => return Ok(Expr::SpellLevel),
        "%CHOICE" | "%LIST" => {
            return match &ctx.choice_id {
                Some(c) => Ok(Expr::Choice(c.clone())),
                None => {
                    ctx.defect("choice-marker-without-choose", ctx.record.id.clone());
                    Ok(Expr::Choice(ctx.record.id.clone()))
                }
            };
        }
        // Equipment-modifier pricing markers: metadata, never a sheet value.
        "%SPELLLEVEL" | "%CASTERLEVEL" | "%CHARGES" | "%SPELLCOST" | "%SPELLXPCOST" | "BASECOST" => {
            return Err("FORMULA:%SPELLLEVEL %CASTERLEVEL %CHARGES %SPELLCOST %SPELLXPCOST BASECOST (equipment-modifier pricing)".into());
        }
        "ENCUMBERANCE" | "ENCUMBRANCE" | "ACCHECK" | "MOVEBASE" | "MAXDEX" | "SPELLFAILURE" => {
            return Err(format!("FORMULA:var(<export token>) ({upper}: equipment/encumbrance state)"));
        }
        _ => {}
    }
    if upper.starts_with('%') {
        return Err(format!("FORMULA:malformed (parser refusals) (marker {upper})"));
    }
    ctx.resolve_variable(name, strict)
}

fn owning_class_level(ctx: &mut RecordCtx) -> Result<Expr, String> {
    match &ctx.owning_class {
        Some(c) => Ok(Expr::ClassLevel(c.clone())),
        None => Err("FORMULA:CL-no-owner".into()),
    }
}

fn str_arg(args: &[Ast], i: usize) -> Option<String> {
    match args.get(i) {
        Some(Ast::Str(s)) => Some(s.clone()),
        Some(Ast::Ident(s)) => Some(s.clone()),
        _ => None,
    }
}

fn lower_call(ctx: &mut RecordCtx, name: &str, args: &[Ast]) -> Result<Expr, String> {
    let lname = name.to_ascii_lowercase();
    let is_lower_or_upper = name.chars().all(|c| !c.is_ascii_alphabetic() || c.is_ascii_lowercase())
        || name.chars().all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());
    if !is_lower_or_upper {
        return Err(format!("FORMULA:malformed (parser refusals) (mixed-case function {name})"));
    }
    match lname.as_str() {
        "max" | "min" => {
            if args.is_empty() {
                return Err("FORMULA:malformed (parser refusals)".into());
            }
            let mut it = args.iter();
            let mut acc = lower(ctx, it.next().unwrap(), true)?;
            for a in it {
                let e = lower(ctx, a, true)?;
                acc = if lname == "max" { Expr::max(acc, e) } else { Expr::min(acc, e) };
            }
            Ok(acc)
        }
        "floor" => {
            if args.len() != 1 {
                return Err("FORMULA:malformed (parser refusals)".into());
            }
            Ok(Expr::Floor(Box::new(lower(ctx, &args[0], true)?)))
        }
        "ceil" => {
            if args.len() != 1 {
                return Err("FORMULA:malformed (parser refusals)".into());
            }
            Ok(Expr::Ceil(Box::new(lower(ctx, &args[0], true)?)))
        }
        "if" => {
            if args.len() != 3 {
                return Err("FORMULA:malformed (parser refusals)".into());
            }
            let c = lower(ctx, &args[0], true)?;
            let a = lower(ctx, &args[1], true)?;
            let b = lower(ctx, &args[2], true)?;
            // if(c,a,b) = b + c*(a-b); c is 0/1 by the comparison encoding, or a bare
            // numeric condition clamped to 0/1 (nonzero = true).
            let c01 = match &args[0] {
                Ast::Cmp(..) | Ast::And(..) | Ast::Or(..) => c,
                _ => Expr::min(Expr::Const(1), Expr::max(Expr::Const(0), c)),
            };
            Ok(Expr::sum(vec![b.clone(), Expr::mul(c01, Expr::sum(vec![a, Expr::neg(b)]))]))
        }
        "classlevel" | "cl" => {
            match str_arg(args, 0) {
                None => owning_class_level(ctx),
                Some(s) if s.eq_ignore_ascii_case("APPLIEDAS=NONEPIC") => owning_class_level(ctx),
                Some(s) => {
                    let class = s.split(';').next().unwrap_or("").trim().to_string();
                    Ok(Expr::ClassLevel(ctx.class_id(&class)))
                }
            }
        }
        "charbonusto" => {
            let kind = str_arg(args, 0).unwrap_or_default();
            let class = str_arg(args, 1).unwrap_or_default();
            if kind.eq_ignore_ascii_case("PCLEVEL") && !class.is_empty() {
                let id = ctx.class_id(&class);
                Ok(Expr::sum(vec![Expr::CasterLevel(ClassRef::Class(id.clone())), Expr::neg(Expr::ClassLevel(id))]))
            } else {
                Err(format!("FORMULA:charbonusto({kind})"))
            }
        }
        "var" => {
            let s = str_arg(args, 0).unwrap_or_default();
            if let Some(class) = s.strip_prefix("CL=") {
                return Ok(Expr::ClassLevel(ctx.class_id(class)));
            }
            if let Some(class) = s.strip_prefix("BL=") {
                let id = ctx.class_id(class);
                return Ok(Expr::sum(vec![Expr::CasterLevel(ClassRef::Class(id.clone())), Expr::neg(Expr::ClassLevel(id))]));
            }
            if let Some(rest) = s.strip_prefix("STAT.")
                && let Some((idx, "SCORE")) = rest.split_once('.')
                && let Ok(i) = idx.parse::<usize>()
            {
                let a = [Ability::Str, Ability::Dex, Ability::Con, Ability::Int, Ability::Wis, Ability::Cha];
                if let Some(ab) = a.get(i) {
                    return Ok(Expr::AbilityScore(*ab));
                }
            }
            if let Some(rest) = s.strip_prefix("SKILL.")
                && let Some((skill, prop)) = rest.rsplit_once('.')
            {
                return match prop {
                    "RANK" | "TOTALRANK" => Ok(Expr::SkillRanks(ctx.skill_id(skill))),
                    "TOTAL" => Ok(Expr::SkillTotal(ctx.skill_id(skill))),
                    _ => Err(format!("FORMULA:var(SKILL.<name>.{prop})")),
                };
            }
            if !s.contains(['.', '[', '=']) && ctx.tree.is_declared(&s) {
                return ctx.resolve_variable(&s, true);
            }
            let prefix: String = s.split(['.', '[', '=']).next().unwrap_or("").to_string();
            Err(format!("FORMULA:var({prefix})"))
        }
        "skillinfo" => {
            let what = str_arg(args, 0).unwrap_or_default().to_ascii_uppercase();
            let skill = str_arg(args, 1).unwrap_or_default();
            match what.as_str() {
                "RANK" | "TOTALRANK" => Ok(Expr::SkillRanks(ctx.skill_id(&skill))),
                "TOTAL" => Ok(Expr::SkillTotal(ctx.skill_id(&skill))),
                other => Err(format!("FORMULA:skillinfo({other})")),
            }
        }
        "count" => {
            let what = str_arg(args, 0).unwrap_or_default().to_ascii_uppercase();
            if what != "ABILITIES" {
                return Err(format!("FORMULA:count({what})"));
            }
            let mut pool = String::from("any");
            let mut filter = HeldFilter::Any;
            for a in args.iter().skip(1) {
                let Some(s) = (match a {
                    Ast::Str(s) => Some(s.clone()),
                    _ => None,
                }) else {
                    return Err("FORMULA:count(ABILITIES, <non-literal filter>)".into());
                };
                for part in s.split('|') {
                    if let Some(c) = part.strip_prefix("CATEGORY=") {
                        pool = super::ctx::slug(c);
                    } else if let Some(t) = part.strip_prefix("TYPE=") {
                        filter = HeldFilter::Tag(t.to_string());
                    } else if let Some(k) = part.strip_prefix("KEY=") {
                        let cat = args.iter().skip(1).find_map(|x| match x {
                            Ast::Str(s) => s.strip_prefix("CATEGORY=").map(|c| c.to_string()),
                            _ => None,
                        });
                        let id = cat.and_then(|c| ctx.resolve_rule(&c, k)).unwrap_or_else(|| super::ctx::slug(k));
                        filter = HeldFilter::Rule(id);
                    } else if let Some(n) = part.strip_prefix("NAME=") {
                        filter = HeldFilter::Rule(super::ctx::slug(n));
                    } else if part.starts_with("VISIBILITY=") || part.starts_with("NATURE=") || part.starts_with("ASPECT=") {
                        // Bookkeeping filters: counting still works on the pool.
                    } else {
                        return Err(format!("FORMULA:count(ABILITIES, {})", part.split('=').next().unwrap_or("?")));
                    }
                }
            }
            Ok(Expr::HeldCount { pool, filter })
        }
        "mastervar" => {
            let s = str_arg(args, 0).unwrap_or_default();
            if s.is_empty() {
                return Err("FORMULA:malformed (parser refusals)".into());
            }
            let id = super::ctx::var_id(&s);
            ctx.var_names.insert(id.clone(), s.to_ascii_uppercase());
            Ok(Expr::MasterVar(id))
        }
        "abs" => Err("FORMULA:abs()".into()),
        "highestspelllevel" | "spelltype" => Ok(Expr::HighestSpellLevel(SpellKind::Any)),
        other => Err(format!("FORMULA:{other}()")),
    }
}

/// Convert one source formula string to `Expr`, or refuse with the token type that has no
/// mapping (the caller records it per token shape).
pub fn convert_formula(ctx: &mut RecordCtx, formula: &str) -> Result<Expr, String> {
    let f = formula.trim();
    if f.is_empty() {
        return Err("FORMULA:malformed (parser refusals) (empty)".into());
    }
    let ast = parse(f)?;
    lower(ctx, &ast, false)
}

/// Whether a formula is a plain integer literal.
pub fn integer_literal(formula: &str) -> Option<i32> {
    formula.trim().parse::<i32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_accepts_the_corpus_grammar() {
        for f in [
            "10+X_SpellLVL+X_DCMod",
            "max(floor((L+3)/6*2),4)",
            "if((A==0&&B>=3),1,0)",
            "1+(K>=3)+(K>=5)",
            "classlevel(\"Investigator\")-1",
            "charbonusto(\"PCLEVEL\",\"Fighter\")",
            "skillinfo(\"TOTALRANK\",\"Perception\")",
            "count(\"ABILITIES\",\"CATEGORY=Special Ability\",\"TYPE=RagePower\")",
            "0.5",
            "-2",
            "%CHOICE*2",
            "var(\"CL=Wizard\")",
        ] {
            assert!(parse(f).is_ok(), "{f}");
        }
        assert!(parse("").is_err());
        assert!(parse("1 +").is_err());
        assert!(parse("Max(1,2)").is_ok(), "mixed case is refused at lowering, not parsing");
    }

    #[test]
    fn literals_are_exact_rationals() {
        assert_eq!(literal("3").unwrap(), Expr::Const(3));
        assert_eq!(literal("0.5").unwrap(), Expr::div(Expr::Const(1), Expr::Const(2)));
        assert_eq!(literal("1.5").unwrap(), Expr::div(Expr::Const(3), Expr::Const(2)));
        assert_eq!(literal("2.0").unwrap(), Expr::Const(2));
    }

    #[test]
    fn comparison_encoding_is_the_exact_min_max_form() {
        use crate::rules_core::sheet_rule::Cmp;
        let e = cmp_expr(Cmp::Gte, Expr::Level, Expr::Const(3));
        assert_eq!(
            e,
            Expr::Min(
                Box::new(Expr::Const(1)),
                Box::new(Expr::Max(Box::new(Expr::Const(0)), Box::new(Expr::Sum(vec![Expr::Level, Expr::Const(-3), Expr::Const(1)]))))
            )
        );
    }
}
