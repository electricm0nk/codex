//! Starfinder 1e mapping table for the overloaded PCGen fields (`decisions.md §8`, SD-37 E3.3).
//!
//! PCGen's Starfinder data reuses Pathfinder field names with other meanings. Read through the
//! Pathfinder mapping they print numbers that look valid and are wrong: `BONUS:HP|ALTHP` is
//! Stamina, not a second hit-point figure; `BONUS:COMBAT|AC` feeds two armour classes (EAC, KAC)
//! that the Pathfinder `AC.Total` sums into one; `FACT:KeyAbilityScore` is sometimes a choice
//! (`Str or Dex`, `INT or WIS`). The table (`SF_MAPPING_TABLE`, JSON) names, per sheet field, the
//! oracle tokens that feed it (its *terms*), with the SRD rule and a PCGen run observation per row.
//!
//! [`evaluate`] reads those terms off the pinned oracle's `.lst` records for one build and sums
//! them per row. It is the instrument that makes the table falsifiable: the seed fixtures and the
//! planted mutations M1–M4 (`sf_mapping_mutations.py`) run against it. A term shape it does not
//! know, a token it cannot find on the named carrier, or a choice-form key ability without a
//! legal choice is refused by name, never guessed. Every `BONUS:HP|` token on a carrier the build
//! reaches that no term claims is reported in [`SfSheet::unclaimed`].
//!
//! Only values that feed a sheet total are computed (the paper-sheet rule); this module renders
//! nothing and writes no rule file.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

/// Repo-relative path of the table (the E3 row's `artifacts/epic_3/**`; E3.3 fixed the path).
pub const SF_MAPPING_TABLE: &str =
    "docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf-mapping-table.v1.json";

#[derive(Debug, Clone, Deserialize)]
pub struct SfMappingTable {
    pub schema: String,
    pub oracle_sha: String,
    pub rows: Vec<SfMappingRow>,
    pub reused_pf_fields: Vec<ReusedPfField>,
}

/// One sheet field the overloaded tokens feed.
#[derive(Debug, Clone, Deserialize)]
pub struct SfMappingRow {
    /// `hit_points`, `stamina`, `eac`, `kac`, `key_ability`, `resolve`.
    pub id: String,
    pub sheet_field: String,
    /// What the Pathfinder mapping would print from the same tokens.
    pub pf_reading: String,
    pub terms: Vec<SfTerm>,
    pub srd: Vec<SrdCitation>,
    pub oracle: Vec<OracleObservation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SfTerm {
    pub id: String,
    /// The oracle token as it appears in the `.lst` (a prefix for coefficient shapes).
    pub token: String,
    /// `.lst` path under the corpus `starfinder/` directory.
    pub file: String,
    /// First tab field of the carrying record; `{class}`, `{race}` and `{feat}` are bound per
    /// build. `KEY:{armor}` matches an equipment record by its `KEY:` field.
    pub record: String,
    pub shape: TermShape,
    /// `DexCappedByMaxDex` only: the `.lst` holding the worn armour's `MAXDEX:`.
    #[serde(default)]
    pub armor_file: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TermShape {
    /// `<token>|<k>*<Class>LVL` on the class record: k × class level.
    ClassLevelCoefficient,
    /// `HD:<n>` on the class record: n per level (PCGen's d1 hit die; n other than 1 is refused).
    HitDiePerLevel,
    /// `BONUS:HP|CURRENTMAX|RaceHP` on `Default`, the value from `BONUS:VAR|RaceHP|<n>` on
    /// `<race> Race Selection ~ Default`.
    RaceHpVar,
    /// `BONUS:HP|ALTHP|CON*TL` on the Constitution stat: Con modifier × total level.
    ConModTimesLevel,
    /// `<token>` = `BONUS:HP|ALTHP|TL` on a feat the build holds: total level, else 0.
    FeatTotalLevel,
    /// `<token>` = `BONUS:VAR|<var>|<n>` on a feat the build holds: n, else 0.
    FeatFlatBonus,
    /// `BONUS:COMBAT|AC|<n>|TYPE=Base`: n.
    AcBase,
    /// `BONUS:COMBAT|AC|<n>|TYPE=<EAC_Armor|KAC_Armor>` on the worn armour: n (0 unarmoured).
    ArmorTypedBonus,
    /// `BONUS:COMBAT|AC|min(ACAbilityStat, min(MXDXEN,MODEQUIPMAXDEX))|TYPE=Ability`: Dex
    /// modifier capped by the worn armour's `MAXDEX:`.
    DexCappedByMaxDex,
    /// `FACT:KeyAbilityScore|<form>` on the class record; a choice form needs the build's choice.
    KeyAbilityFact,
    /// `BONUS:VAR|Resolve|max(1,Resolve_PCLvl+KeyAbilityBonus)` with
    /// `BONUS:VAR|Resolve_PCLvl|max(1,EffectiveLVL/2)` on `Default`.
    ResolveLevelAndKey,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SrdCitation {
    pub url: String,
    pub section: String,
}

/// One value a committed PCGen run printed (`oracle-builds/<build>.oracle.txt`, key `key`).
#[derive(Debug, Clone, Deserialize)]
pub struct OracleObservation {
    pub build: String,
    pub key: String,
    pub value: String,
}

/// A Pathfinder mapping-table row the Starfinder data uses with its Pathfinder meaning unchanged.
#[derive(Debug, Clone, Deserialize)]
pub struct ReusedPfField {
    pub field: String,
    pub why_unchanged: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stat {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

impl Stat {
    fn index(self) -> usize {
        self as usize
    }
    fn parse(s: &str) -> Option<Stat> {
        match s.to_ascii_uppercase().as_str() {
            "STR" => Some(Stat::Str),
            "DEX" => Some(Stat::Dex),
            "CON" => Some(Stat::Con),
            "INT" => Some(Stat::Int),
            "WIS" => Some(Stat::Wis),
            "CHA" => Some(Stat::Cha),
            _ => None,
        }
    }
}

/// One character, as the seed fixtures specify it.
#[derive(Debug, Clone)]
pub struct SfBuild {
    pub race: String,
    pub class: String,
    pub level: i64,
    /// Final scores, STR DEX CON INT WIS CHA.
    pub scores: [i64; 6],
    pub key_ability_choice: Option<Stat>,
    pub feats: Vec<String>,
    /// `KEY:` of the worn armour record, if any.
    pub armor_key: Option<String>,
}

impl SfBuild {
    /// SF Table 2-1: floor(score / 2) − 5 (the oracle's `d20Mod`).
    pub fn modifier(&self, stat: Stat) -> i64 {
        self.scores[stat.index()].div_euclid(2) - 5
    }
}

#[derive(Debug, Clone)]
pub struct SfSheet {
    pub hit_points: i64,
    pub stamina: i64,
    pub eac: i64,
    pub kac: i64,
    pub key_ability: Stat,
    pub resolve: i64,
    /// (row id, term id, value) for every term evaluated.
    pub terms: Vec<(String, String, i64)>,
    /// `BONUS:HP|` tokens on a carrier this build reaches that no term claims.
    pub unclaimed: Vec<String>,
}

pub fn load_table(path: &Path) -> Result<SfMappingTable, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let table: SfMappingTable = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    if table.schema != "sf-mapping-table.v1" {
        return Err(format!("{}: schema {} is not sf-mapping-table.v1", path.display(), table.schema));
    }
    Ok(table)
}

/// The table the sheet-rule converter routes Starfinder `BONUS:HP` tokens by (SD-37 E3.4):
/// [`SF_MAPPING_TABLE`] under the repo root, read once per process. The converter reads the
/// table itself rather than a transcription, so an edit to a row (a planted mutation M1–M4)
/// moves the converted package too.
pub fn converter_table() -> Result<&'static SfMappingTable, String> {
    static TABLE: std::sync::OnceLock<Result<SfMappingTable, String>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| load_table(&crate::repo_root().join(SF_MAPPING_TABLE))).as_ref().map_err(|e| e.clone())
}

/// The table row a Starfinder `BONUS:HP|<pool>|<value>` token on `carrier` feeds, or `None` when
/// no term claims it (`decisions.md §8`: a field with no oracle row is a named refusal, never a
/// guess).
///
/// `carrier` is the record the token sits on as the table's terms name it: `CLASS:<name>` for a
/// class, else the record's KEY (`Toughness`, `Default`, `Constitution`), compared without case
/// (the converter passes the row's own upper-cased declaration). A term claims the token
/// when its token's pool is `<pool>`, its record is the carrier (`CLASS:{class}` = any class),
/// and the value has the term's shape: the literal value the term's token states, or for a
/// [`TermShape::ClassLevelCoefficient`] `<k>*<Class>LVL` naming the carrier's own class. So the
/// drone's `BONUS:HP|CURRENTMAX|-1` and `+1 Hit Point`'s `BONUS:HP|CURRENTMAX|1` are unclaimed
/// (the table's named refusals), while Soldier's `BONUS:HP|ALTHP|7*SoldierLVL` is `stamina`.
pub fn hp_pool_row<'t>(table: &'t SfMappingTable, carrier: &str, pool: &str, value: &str) -> Option<&'t str> {
    let value = value.trim();
    for row in &table.rows {
        for term in &row.terms {
            let mut parts = term.token.split('|');
            if parts.next() != Some("BONUS:HP") || parts.next() != Some(pool) {
                continue;
            }
            let stated = parts.next();
            let claims = match (term.record.as_str(), stated) {
                ("CLASS:{class}", None) if term.shape == TermShape::ClassLevelCoefficient => carrier
                    .strip_prefix("CLASS:")
                    .is_some_and(|class| class_level_coefficient(value, class).is_some()),
                (record, Some(v)) => record.eq_ignore_ascii_case(carrier) && v == value,
                _ => false,
            };
            if claims {
                return Some(row.id.as_str());
            }
        }
    }
    None
}

/// `<k>*<Class>LVL` (spaces allowed around `*`, class name compared without spaces and
/// case-insensitively) -> k.
fn class_level_coefficient(value: &str, class: &str) -> Option<i64> {
    let (k, rest) = value.split_once('*')?;
    let want = format!("{}LVL", class.replace(' ', "")).to_ascii_uppercase();
    (rest.trim().replace(' ', "").to_ascii_uppercase() == want).then_some(())?;
    k.trim().parse().ok()
}

/// `FACT:KeyAbilityScore` value → the abilities it allows: `CHA` → [Cha]; `Str or Dex` →
/// [Str, Dex]. Anything else is refused by name.
pub fn parse_key_ability_fact(value: &str) -> Result<Vec<Stat>, String> {
    let parts: Vec<&str> = value.split(" or ").map(str::trim).collect();
    let stats: Option<Vec<Stat>> = parts.iter().map(|p| Stat::parse(p)).collect();
    match stats {
        Some(v) if !v.is_empty() => Ok(v),
        _ => Err(format!("FACT:KeyAbilityScore form {value:?} is neither a stat abbreviation nor `<stat> or <stat>`")),
    }
}

/// The tab fields of every non-comment line of `file` whose first field is `record` (or, for
/// `KEY:<k>`, which carries the field `KEY:<k>`).
fn record_fields(sf_root: &Path, file: &str, record: &str, cache: &mut BTreeMap<String, String>) -> Result<Vec<String>, String> {
    if !cache.contains_key(file) {
        let path = sf_root.join(file);
        let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        cache.insert(file.to_string(), String::from_utf8_lossy(&bytes).into_owned());
    }
    let text = &cache[file];
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let fields: Vec<&str> = line.split('\t').filter(|f| !f.is_empty()).collect();
        let hit = match record.strip_prefix("KEY:") {
            Some(_) => fields.contains(&record),
            None => fields.first().is_some_and(|f| *f == record),
        };
        if hit {
            out.extend(fields.iter().map(|f| f.to_string()));
        }
    }
    if out.is_empty() {
        return Err(format!("no record {record:?} in {file}"));
    }
    Ok(out)
}

fn bind(record: &str, build: &SfBuild) -> String {
    let mut r = record.replace("{class}", &build.class).replace("{race}", &build.race);
    if let Some(a) = &build.armor_key {
        r = r.replace("{armor}", a);
    }
    r
}

fn find_field<'a>(fields: &'a [String], token: &str) -> Option<&'a str> {
    fields.iter().map(String::as_str).find(|f| *f == token || f.starts_with(&format!("{token}|")))
}

fn parse_int(s: &str, what: &str) -> Result<i64, String> {
    s.trim().parse::<i64>().map_err(|_| format!("{what}: {s:?} is not an integer"))
}

struct Ctx<'a> {
    sf_root: &'a Path,
    build: &'a SfBuild,
    cache: BTreeMap<String, String>,
    key_ability: Option<Stat>,
    /// every carrier (file, record) the evaluation read, for the unclaimed scan
    carriers: Vec<(String, String)>,
}

impl Ctx<'_> {
    fn fields(&mut self, file: &str, record: &str) -> Result<Vec<String>, String> {
        let f = record_fields(self.sf_root, file, record, &mut self.cache)?;
        if !self.carriers.iter().any(|(a, b)| a == file && b == record) {
            self.carriers.push((file.to_string(), record.to_string()));
        }
        Ok(f)
    }

    fn armor_fields(&mut self, term: &SfTerm) -> Result<Option<Vec<String>>, String> {
        match &self.build.armor_key {
            None => Ok(None),
            Some(_) => {
                let rec = bind(&term.record, self.build);
                if !rec.starts_with("KEY:") {
                    return Err(format!("term {}: an armour term's record must be `KEY:{{armor}}`, not {rec:?}", term.id));
                }
                self.fields(&term.file, &rec).map(Some)
            }
        }
    }

    fn term_value(&mut self, term: &SfTerm) -> Result<i64, String> {
        let b = self.build;
        let missing = |rec: &str| format!("term {}: token {:?} not found on {rec} ({})", term.id, term.token, term.file);
        match term.shape {
            TermShape::ClassLevelCoefficient => {
                let rec = bind(&term.record, b);
                let fields = self.fields(&term.file, &rec)?;
                let f = find_field(&fields, &term.token).ok_or_else(|| missing(&rec))?;
                let formula = &f[term.token.len() + 1..];
                let want = format!("*{}LVL", b.class);
                let k = formula.strip_suffix(&want).ok_or_else(|| {
                    format!("term {}: {f:?} is not the `<k>*{}LVL` shape", term.id, b.class)
                })?;
                Ok(parse_int(k, &term.id)? * b.level)
            }
            TermShape::HitDiePerLevel => {
                let rec = bind(&term.record, b);
                let fields = self.fields(&term.file, &rec)?;
                let f = fields.iter().find(|f| f.starts_with(&term.token)).ok_or_else(|| missing(&rec))?;
                let n = parse_int(&f[term.token.len()..], &term.id)?;
                if n != 1 {
                    return Err(format!("term {}: HD:{n} is a rolled die; only the d1 Starfinder die is read", term.id));
                }
                Ok(n * b.level)
            }
            TermShape::RaceHpVar => {
                let rec = bind(&term.record, b);
                let fields = self.fields(&term.file, &rec)?;
                if !fields.contains(&term.token) {
                    return Err(missing(&rec));
                }
                let sel = format!("{} Race Selection ~ Default", b.race);
                let sel_fields = self.fields(&term.file, &sel)?;
                let v = sel_fields
                    .iter()
                    .find_map(|f| f.strip_prefix("BONUS:VAR|RaceHP|"))
                    .ok_or_else(|| format!("term {}: no BONUS:VAR|RaceHP on {sel:?}", term.id))?;
                parse_int(v, &term.id)
            }
            TermShape::ConModTimesLevel => {
                let rec = bind(&term.record, b);
                let fields = self.fields(&term.file, &rec)?;
                if !fields.contains(&term.token) {
                    return Err(missing(&rec));
                }
                Ok(b.modifier(Stat::Con) * b.level)
            }
            TermShape::FeatTotalLevel | TermShape::FeatFlatBonus => {
                // the carrier must exist in the oracle whether or not this build holds the feat
                let feat = term.record.clone();
                let fields = self.fields(&term.file, &feat)?;
                let f = fields.iter().find(|f| **f == term.token).ok_or_else(|| missing(&feat))?;
                if !b.feats.contains(&feat) {
                    return Ok(0);
                }
                match term.shape {
                    TermShape::FeatTotalLevel => Ok(b.level),
                    _ => {
                        let n = f.rsplit('|').next().unwrap_or_default();
                        parse_int(n, &term.id)
                    }
                }
            }
            TermShape::AcBase => {
                let fields = self.fields(&term.file, &term.record)?;
                let f = fields.iter().find(|f| **f == term.token).ok_or_else(|| missing(&term.record))?;
                let n = f.split('|').nth(2).unwrap_or_default();
                parse_int(n, &term.id)
            }
            TermShape::ArmorTypedBonus => {
                let Some(fields) = self.armor_fields(term)? else { return Ok(0) };
                let suffix = term.token.rsplit('|').next().unwrap_or_default().to_string(); // TYPE=EAC_Armor
                let prefix = "BONUS:COMBAT|AC|";
                let f = fields
                    .iter()
                    .find(|f| f.starts_with(prefix) && f.ends_with(&format!("|{suffix}")))
                    .ok_or_else(|| format!("term {}: worn armour carries no {}…|{suffix}", term.id, prefix))?;
                parse_int(f[prefix.len()..f.len() - suffix.len() - 1].trim(), &term.id)
            }
            TermShape::DexCappedByMaxDex => {
                let fields = self.fields(&term.file, &term.record)?;
                if !fields.contains(&term.token) {
                    return Err(missing(&term.record));
                }
                let dex = b.modifier(Stat::Dex);
                let armor = match &b.armor_key {
                    None => return Ok(dex),
                    Some(k) => k.clone(),
                };
                if term.armor_file.is_empty() {
                    return Err(format!("term {}: no armor_file for the MAXDEX: lookup", term.id));
                }
                let afields = self.fields(&term.armor_file.clone(), &format!("KEY:{armor}"))?;
                match afields.iter().find_map(|f| f.strip_prefix("MAXDEX:")) {
                    Some(m) => Ok(dex.min(parse_int(m, &term.id)?)),
                    None => Ok(dex),
                }
            }
            TermShape::KeyAbilityFact => {
                let rec = bind(&term.record, b);
                let fields = self.fields(&term.file, &rec)?;
                let form = fields
                    .iter()
                    .find_map(|f| f.strip_prefix(&format!("{}|", term.token)))
                    .ok_or_else(|| missing(&rec))?
                    .to_string();
                let allowed = parse_key_ability_fact(&form)?;
                let stat = if allowed.len() == 1 {
                    allowed[0]
                } else {
                    match b.key_ability_choice {
                        Some(c) if allowed.contains(&c) => c,
                        Some(c) => return Err(format!("{}: key ability is `{form}`; the build chose {c:?}", b.class)),
                        None => return Err(format!("{}: key ability is the choice `{form}` and the build makes none", b.class)),
                    }
                };
                self.key_ability = Some(stat);
                Ok(0)
            }
            TermShape::ResolveLevelAndKey => {
                let fields = self.fields(&term.file, &term.record)?;
                if !fields.contains(&term.token) {
                    return Err(missing(&term.record));
                }
                let lvl_token = "BONUS:VAR|Resolve_PCLvl|max(1,EffectiveLVL/2)";
                if !fields.iter().any(|f| f == lvl_token) {
                    return Err(format!("term {}: {lvl_token} not found on {}", term.id, term.record));
                }
                let key = self.key_ability.ok_or_else(|| format!("term {}: no key ability (the key_ability row runs first)", term.id))?;
                let pc_lvl = (b.level / 2).max(1);
                Ok((pc_lvl + b.modifier(key)).max(1))
            }
        }
    }
}

/// Evaluate every row of `table` for `build` over the oracle at `sf_root` (the corpus
/// `starfinder/` directory). Rows are evaluated in table order except `key_ability`, which runs
/// first because Resolve reads it.
pub fn evaluate(table: &SfMappingTable, sf_root: &Path, build: &SfBuild) -> Result<SfSheet, String> {
    let mut ctx = Ctx { sf_root, build, cache: BTreeMap::new(), key_ability: None, carriers: Vec::new() };
    let mut totals: BTreeMap<String, i64> = BTreeMap::new();
    let mut terms = Vec::new();
    let mut order: Vec<&SfMappingRow> = table.rows.iter().filter(|r| r.id == "key_ability").collect();
    order.extend(table.rows.iter().filter(|r| r.id != "key_ability"));
    for row in order {
        let mut sum = 0;
        for term in &row.terms {
            let v = ctx.term_value(term)?;
            terms.push((row.id.clone(), term.id.clone(), v));
            sum += v;
        }
        totals.insert(row.id.clone(), sum);
    }
    let get = |id: &str| totals.get(id).copied().ok_or_else(|| format!("the table has no {id} row"));
    // the unclaimed scan: every BONUS:HP| token on a carrier this build reached, plus the race
    // and feat records it holds, must be some term's token
    // claimed = equal to a term's token, or a coefficient term's `<token>|<k>*<Class>LVL`
    let claims = |f: &str| {
        table.rows.iter().flat_map(|r| r.terms.iter()).any(|t| {
            f == t.token
                || (t.shape == TermShape::ClassLevelCoefficient
                    && f.strip_prefix(&format!("{}|", t.token)).is_some_and(|rest| rest.ends_with("LVL") && rest.contains('*')))
        })
    };
    let mut scan = ctx.carriers.clone();
    for feat in &build.feats {
        scan.push(("paizo/core/scr_feats.lst".to_string(), feat.clone()));
    }
    scan.push(("paizo/core/scr_races.lst".to_string(), build.race.clone()));
    let mut unclaimed = Vec::new();
    for (file, record) in scan {
        let Ok(fields) = record_fields(sf_root, &file, &record, &mut ctx.cache) else { continue };
        for f in fields.iter().filter(|f| f.starts_with("BONUS:HP|")) {
            if !claims(f) {
                unclaimed.push(format!("{record} ({file}): {f}"));
            }
        }
    }
    unclaimed.sort();
    unclaimed.dedup();
    Ok(SfSheet {
        hit_points: get("hit_points")?,
        stamina: get("stamina")?,
        eac: get("eac")?,
        kac: get("kac")?,
        key_ability: ctx.key_ability.ok_or("the table's key_ability row set no key ability")?,
        resolve: get("resolve")?,
        terms,
        unclaimed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifier_is_floor_score_over_two_minus_five() {
        let mut b = SfBuild {
            race: String::new(),
            class: String::new(),
            level: 1,
            scores: [8, 9, 10, 11, 18, 19],
            key_ability_choice: None,
            feats: vec![],
            armor_key: None,
        };
        assert_eq!(
            [Stat::Str, Stat::Dex, Stat::Con, Stat::Int, Stat::Wis, Stat::Cha].map(|s| b.modifier(s)),
            [-1, -1, 0, 0, 4, 4]
        );
        b.scores[0] = 1;
        assert_eq!(b.modifier(Stat::Str), -5);
    }

    /// SD-37 E3.4: the converter's pool routing reads the committed table. Every claim below is
    /// a token the pinned Core Rulebook carries (`decisions.md §8`'s two tables).
    #[test]
    fn hp_pool_row_routes_claimed_tokens_and_refuses_the_rest() {
        let table = converter_table().expect("the committed SF mapping table loads");
        assert_eq!(hp_pool_row(table, "CLASS:Soldier", "CURRENTMAX", "6*SoldierLVL"), Some("hit_points"));
        assert_eq!(hp_pool_row(table, "CLASS:Soldier", "ALTHP", "7*SoldierLVL"), Some("stamina"));
        assert_eq!(hp_pool_row(table, "Default", "CURRENTMAX", "RaceHP"), Some("hit_points"));
        assert_eq!(hp_pool_row(table, "Constitution", "ALTHP", "CON*TL"), Some("stamina"));
        assert_eq!(hp_pool_row(table, "Toughness", "ALTHP", "TL"), Some("stamina"));
        // The table's named refusals.
        assert_eq!(hp_pool_row(table, "CLASS:Drone", "CURRENTMAX", "-1"), None);
        assert_eq!(hp_pool_row(table, "CLASS:Drone", "CURRENTMAX", "(10*DroneLVL)+((DroneLVL+1)/2)"), None);
        assert_eq!(hp_pool_row(table, "+1 Hit Point", "CURRENTMAX", "1"), None);
        assert_eq!(hp_pool_row(table, "Energy Shield", "ALTHP", "DroneMasterLVL"), None);
        // A coefficient naming another class, or a carrier that is not the term's record.
        assert_eq!(hp_pool_row(table, "CLASS:Soldier", "CURRENTMAX", "6*EnvoyLVL"), None);
        assert_eq!(hp_pool_row(table, "Weapon Focus", "ALTHP", "TL"), None);
    }

    #[test]
    fn find_field_matches_whole_token_or_token_prefix() {
        let f = vec!["BONUS:HP|CURRENTMAX|6*SoldierLVL".to_string(), "BONUS:HP|ALTHP|7*SoldierLVL".to_string()];
        assert_eq!(find_field(&f, "BONUS:HP|ALTHP"), Some("BONUS:HP|ALTHP|7*SoldierLVL"));
        assert_eq!(find_field(&f, "BONUS:HP|ALT"), None);
    }
}
