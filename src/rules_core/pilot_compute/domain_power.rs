//! PCGen domain-power formula interpreter (SD-31 wave 25, OPERATOR-RULINGS
//! 2026-08-21 section 20 overturning SD-27 decisions.md section 24.1's "no
//! formula interpreter" ban, for this package only). SD-31 wave 26 widened
//! [`DOMAIN_POWER_CATALOG`] from Good/War/Strength to five entries
//! (+Destruction's Destructive Smite, +Glory's Touch of Glory) and wired
//! Cleric's own domain-power branch (previously Good/Healing-only, unlike
//! Inquisitor's already-generic one) onto the same shared catalog -- see
//! `explain_cleric_level1_spell_baseline` in `pilot_compute/mod.rs`. Before
//! widening the catalog, this lane confirmed the interpreted path still
//! reproduces Good and Healing's pre-existing pinned values exactly (no
//! `fixture_check_tests` change was needed for either -- they were, and
//! remain, green).
//!
//! Before this seam, `ground_or_block_cleric_domain_power`'s equivalent (the
//! inline block inside `explain_cleric_level1_spell_baseline`) and
//! `ground_or_block_inquisitor_domain_power` each hand-wrote the Good and
//! Healing domains' granted-power arithmetic as bespoke closed-form Rust
//! (`(level / 2).max(1)`, `(3 + wisdom_modifier).max(0)`), gated behind a
//! two-item allowlist (`GOOD_DOMAIN_SELECTION`, `HEALING_DOMAIN_SELECTION`).
//! Every other domain in every book rode a claim-blocking diagnostic because
//! extending that pattern meant writing, and independently primary-source
//! verifying, one MORE bespoke function per domain.
//!
//! This module replaces the ARITHMETIC (not the allowlist's two selection
//! constants, which many existing tests reference by name and which still
//! name real, already-verified domains) with a small, general PCGen formula
//! evaluator applied to formula strings TRANSCRIBED VERBATIM from
//! `data/corpus/core_rulebook/class_feature/domain_power/*.json` and
//! `data/corpus/core_rulebook/class_feature/<domain>/<domain>.json`'s own
//! `DESC`/`BONUS:VAR` tokens (see each [`DomainPowerSpec`]'s doc comment for
//! its own upstream `.lst` line and sha256). Adding a new domain to
//! [`DOMAIN_POWER_CATALOG`] means transcribing ONE more formula string, not
//! deriving and independently verifying a new closed-form Rust expression --
//! the throughput problem `OPERATOR-RULINGS-2026-08-21.md` section 20 names.
//! SD-34 wave 38 lane A widened [`DomainPowerSpec`] with an OPTIONAL
//! per-spec `uses_per_day_formula` override (see its own doc comment) for
//! entries whose corpus formula slot IS the uses-per-day count rather than
//! a flat bonus -- Construct Subdomain's Animate Servant -- without
//! touching the shared `3+WIS` default every pre-existing entry still uses.
//!
//! this module's own `fixture_check_tests` inline test module (below) is its fixture gate,
//! written to the SAME rigor `derived_evaluator_fixture_check` uses without
//! touching that proof harness (out of this lane's granted write scope): it
//! embeds the real corpus JSON via `include_str!`, asserts every
//! [`DomainPowerSpec`] formula string here is byte-for-byte what the corpus
//! states (the transcription half), and asserts this module's evaluator
//! reproduces expected numbers computed independently of the evaluator (the
//! interpretation half) -- both halves `derived_evaluator_fixture_check`'s
//! own module doc names as the two ways a formula-reading seam can be wrong.
//!
//! ## What this interpreter covers
//! Integer literals, a bound variable environment (any identifier ending in
//! `LVL` resolves to the granting class's own level -- verified true for
//! every CRB domain's own `BONUS:VAR|Domain<X>LVL|DomainLVL|TYPE=Domain`
//! chain by this file's own `domain_header_lvl_chain_matches_the_shared_shape`
//! test; the six PF1 ability abbreviations resolve to the character's own
//! ability modifier), `+ - * /` with standard precedence (PCGen integer
//! division, truncating toward zero -- Rust's own `/` on signed integers,
//! matching the pre-existing hand-derived arithmetic this seam replaces),
//! parenthesised sub-expressions, and the two-argument `max(a,b)`/`min(a,b)`
//! functions.
//!
//! ## What it does NOT cover -- refuses (`None`) rather than guesses
//! - Dice notation (`1d4`) -- Healing's Rebuke Death heal amount stays
//!   ungrounded for exactly this reason, unchanged from before this seam.
//! - Multi-`DESC`-token, level-gated formula variants (a `PREVARLT:<var>,<n>`
//!   token picking between two description texts, e.g. Rebuke Death's own
//!   heal-amount pair, and Acid Dart/Artificer's Touch/Blast Rune/Fire Bolt/
//!   Icicle/Lightning Arc/Storm Burst in the wider CRB domain-power corpus).
//!   `parse_pcgen_expr` never sees a `PREVARLT`/`PREVARGTEQ` fragment because
//!   [`DOMAIN_POWER_CATALOG`] never carries one as a `magnitude_formula`.
//! - Domain records whose corpus ingest lacks the header's `Domain<X>LVL`/
//!   `Domain<X>Times` `BONUS:VAR` chain entirely -- confirmed absent for
//!   Inner Sea World Guide's Void and Scalykind domains and the Dark
//!   Tapestry subdomain (their granted-power records carry a bare `ASPECT`
//!   reference to a `Domain<X>Times`-shaped name with no `BONUS:VAR` token
//!   anywhere in the book establishing what it resolves to, and no domain
//!   header record is ingested at all) -- `resolve_domain` returns `None`
//!   for every selection this catalog does not carry, and the caller's
//!   pre-existing claim-blocking diagnostic is preserved unchanged for them.
//! - Any granted power whose real mechanic targets an ENEMY rather than
//!   applying a buff (Evil's Touch of Evil, Darkness's Touch of Darkness,
//!   Madness's Vision of Madness all parse cleanly under this grammar --
//!   `max(Domain<X>LVL/2,1)`, the exact shape Good/War/Strength use -- but
//!   `active_touch_of_good_bonus`'s "self-application only" approximation,
//!   honest for a power that helps its target, would misrepresent an
//!   enemy-facing sicken/conceal/mind-affecting-swap effect as a self-buff.
//!   [`DOMAIN_POWER_CATALOG`] deliberately omits them; they stay named in
//!   the surrounding claim-blocking diagnostic, unchanged.

use super::*;

/// A PCGen arithmetic expression, parsed from a formula string transcribed
/// verbatim from the corpus. Deliberately narrow: see this module's own doc
/// comment for exactly which corpus formula shapes this covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Expr {
    Int(i32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Max(Box<Expr>, Box<Expr>),
    Min(Box<Expr>, Box<Expr>),
}

/// Parses a PCGen arithmetic formula string (e.g. `"max(DomainGoodLVL/2,1)"`,
/// `"3+WIS"`) into an [`Expr`]. Returns `None` on ANY syntax this small
/// recursive-descent grammar does not recognize -- dice notation, a
/// `PREVARLT`/`PREVARGTEQ` fragment, a function other than `max`/`min`, or
/// trailing unconsumed input -- refusing rather than guessing, the same
/// discipline `derived_evaluator_fixture_check::parse_class_feature_level_scaling`
/// uses for its own narrower grammar.
pub(super) fn parse_pcgen_expr(input: &str) -> Option<Expr> {
    let compact: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut pos = 0usize;
    let expr = parse_expr(&compact, &mut pos)?;
    if pos != compact.len() {
        return None; // trailing unconsumed input -- refuse rather than guess
    }
    Some(expr)
}

fn parse_expr(chars: &[char], pos: &mut usize) -> Option<Expr> {
    let mut node = parse_term(chars, pos)?;
    loop {
        match chars.get(*pos) {
            Some('+') => {
                *pos += 1;
                let rhs = parse_term(chars, pos)?;
                node = Expr::Add(Box::new(node), Box::new(rhs));
            }
            Some('-') => {
                *pos += 1;
                let rhs = parse_term(chars, pos)?;
                node = Expr::Sub(Box::new(node), Box::new(rhs));
            }
            _ => break,
        }
    }
    Some(node)
}

fn parse_term(chars: &[char], pos: &mut usize) -> Option<Expr> {
    let mut node = parse_unary(chars, pos)?;
    loop {
        match chars.get(*pos) {
            Some('*') => {
                *pos += 1;
                let rhs = parse_unary(chars, pos)?;
                node = Expr::Mul(Box::new(node), Box::new(rhs));
            }
            Some('/') => {
                *pos += 1;
                let rhs = parse_unary(chars, pos)?;
                node = Expr::Div(Box::new(node), Box::new(rhs));
            }
            _ => break,
        }
    }
    Some(node)
}

fn parse_unary(chars: &[char], pos: &mut usize) -> Option<Expr> {
    if chars.get(*pos) == Some(&'-') {
        *pos += 1;
        let inner = parse_unary(chars, pos)?;
        return Some(Expr::Neg(Box::new(inner)));
    }
    parse_primary(chars, pos)
}

fn parse_primary(chars: &[char], pos: &mut usize) -> Option<Expr> {
    match chars.get(*pos) {
        Some('(') => {
            *pos += 1;
            let inner = parse_expr(chars, pos)?;
            if chars.get(*pos) != Some(&')') {
                return None;
            }
            *pos += 1;
            Some(inner)
        }
        Some(c) if c.is_ascii_digit() => {
            let start = *pos;
            while chars.get(*pos).is_some_and(|c| c.is_ascii_digit()) {
                *pos += 1;
            }
            let text: String = chars[start..*pos].iter().collect();
            text.parse::<i32>().ok().map(Expr::Int)
        }
        Some(c) if c.is_ascii_alphabetic() => {
            let start = *pos;
            while chars.get(*pos).is_some_and(|c| c.is_ascii_alphanumeric()) {
                *pos += 1;
            }
            let name: String = chars[start..*pos].iter().collect();
            if chars.get(*pos) == Some(&'(') && (name == "max" || name == "min") {
                *pos += 1;
                let a = parse_expr(chars, pos)?;
                if chars.get(*pos) != Some(&',') {
                    return None;
                }
                *pos += 1;
                let b = parse_expr(chars, pos)?;
                if chars.get(*pos) != Some(&')') {
                    return None;
                }
                *pos += 1;
                return Some(if name == "max" {
                    Expr::Max(Box::new(a), Box::new(b))
                } else {
                    Expr::Min(Box::new(a), Box::new(b))
                });
            }
            Some(Expr::Var(name))
        }
        _ => None,
    }
}

/// Evaluates a parsed [`Expr`] against `env`, a variable-name resolver.
/// `env` returning `None` for a name this expression actually references
/// propagates as `None` (refuse rather than silently treat an unresolved
/// variable as zero).
pub(super) fn eval_expr(expr: &Expr, env: &impl Fn(&str) -> Option<i32>) -> Option<i32> {
    match expr {
        Expr::Int(n) => Some(*n),
        Expr::Var(name) => env(name),
        Expr::Add(a, b) => Some(eval_expr(a, env)? + eval_expr(b, env)?),
        Expr::Sub(a, b) => Some(eval_expr(a, env)? - eval_expr(b, env)?),
        Expr::Mul(a, b) => Some(eval_expr(a, env)? * eval_expr(b, env)?),
        Expr::Div(a, b) => {
            let denom = eval_expr(b, env)?;
            if denom == 0 {
                return None;
            }
            Some(eval_expr(a, env)? / denom) // truncates toward zero, PCGen's own semantics
        }
        Expr::Neg(a) => Some(-eval_expr(a, env)?),
        Expr::Max(a, b) => Some(eval_expr(a, env)?.max(eval_expr(b, env)?)),
        Expr::Min(a, b) => Some(eval_expr(a, env)?.min(eval_expr(b, env)?)),
    }
}

/// Resolves a PCGen ability abbreviation (`STR`/`DEX`/`CON`/`INT`/`WIS`/`CHA`)
/// against an already-computed [`AbilityModifiers`]. `None` for anything else
/// -- the caller's own `LVL`-suffixed variable names never reach this
/// function (see [`domain_power_env`]).
fn ability_abbreviation_modifier(modifiers: &AbilityModifiers, name: &str) -> Option<i32> {
    match name {
        "STR" => Some(i32::from(modifiers.strength)),
        "DEX" => Some(i32::from(modifiers.dexterity)),
        "CON" => Some(i32::from(modifiers.constitution)),
        "INT" => Some(i32::from(modifiers.intelligence)),
        "WIS" => Some(i32::from(modifiers.wisdom)),
        "CHA" => Some(i32::from(modifiers.charisma)),
        _ => None,
    }
}

/// The variable environment every domain-power formula in
/// [`DOMAIN_POWER_CATALOG`] resolves against: any name ending in `LVL`
/// (`Domain<X>LVL`, PCGen's own spelling) is the granting class's level --
/// correct because every CRB domain header's own
/// `BONUS:VAR|Domain<X>LVL|DomainLVL|TYPE=Domain` chain resolves `DomainLVL`
/// to the granting class's level with no per-domain offset (the same
/// structural fact `active_touch_of_good_bonus`'s own doc comment already
/// states for Good, reused unmodified here); the six ability abbreviations
/// resolve to `modifiers`' own field. Any other identifier -- this grammar
/// never emits one, since [`DOMAIN_POWER_CATALOG`]'s formula strings are
/// hand-transcribed and fixture-checked -- resolves to `None`.
fn domain_power_env(
    class_level: u8,
    modifiers: &AbilityModifiers,
) -> impl Fn(&str) -> Option<i32> + use<> {
    let class_level = i32::from(class_level);
    let modifiers = *modifiers;
    move |name: &str| {
        if let Some(m) = ability_abbreviation_modifier(&modifiers, name) {
            return Some(m);
        }
        if name.ends_with("LVL") {
            return Some(class_level);
        }
        None
    }
}

/// PF1 Core Rulebook Domains' shared granted-power uses-per-day formula,
/// `3+WIS` -- transcribed verbatim from
/// `data/corpus/core_rulebook/class_feature/domains/domains.json`'s own
/// `BONUS:VAR|DomainPowerTimes|3+WIS` token. Every one of the 20 CRB domain
/// headers' own `BONUS:VAR|Domain<X>Times|DomainPowerTimes|TYPE=Domain`
/// chain resolves to this SAME formula (verified corpus-wide against a
/// sample of six domain headers besides Good/Healing this cycle: Luck,
/// Travel, Magic, Madness, Rune, and Healing itself all carry the identical
/// chain) -- so this is genuinely one shared, corpus-stated formula, not
/// per-domain content, and every [`DOMAIN_POWER_CATALOG`] entry reuses this
/// single interpreted call rather than repeating `3+WIS` as a per-domain
/// constant.
pub(super) const DOMAIN_POWER_TIMES_FORMULA: &str = "3+WIS";

/// Interprets [`DOMAIN_POWER_TIMES_FORMULA`] for `modifiers`, floored at 0
/// (PF1's own "times per day" floor -- a formula that would resolve
/// negative for a very low Wisdom score never grants negative uses). Used
/// for every [`DomainPowerSpec`] in [`DOMAIN_POWER_CATALOG`], replacing the
/// THREE separate hand-written `(3 + wisdom_modifier).max(0)` call sites
/// this seam previously carried (Cleric's Good uses/day, Cleric's Healing
/// Rebuke Death uses/day, Inquisitor's Good uses/day) with one interpreted
/// evaluation of the corpus's own formula string.
pub(super) fn domain_power_uses_per_day(modifiers: &AbilityModifiers) -> i16 {
    let expr = parse_pcgen_expr(DOMAIN_POWER_TIMES_FORMULA)
        .expect("DOMAIN_POWER_TIMES_FORMULA is a fixed, fixture-checked literal");
    let env = domain_power_env(0, modifiers); // LVL never appears in "3+WIS"
    let value = eval_expr(&expr, &env)
        .expect("3+WIS resolves under domain_power_env for any AbilityModifiers");
    i16::try_from(value.max(0)).unwrap_or(i16::MAX)
}

/// SD-34 wave 38 lane A: interprets `spec`'s own uses-per-day formula --
/// `spec.uses_per_day_formula` when present (a per-spec override needed
/// when the corpus's own granted-power formula slot IS the uses-per-day
/// count rather than a flat bonus, see [`DomainPowerSpec::uses_per_day_formula`]),
/// else [`DOMAIN_POWER_TIMES_FORMULA`] (`3+WIS`), the shared formula every
/// catalog entry through SD-34 wave 37 used unconditionally via
/// [`domain_power_uses_per_day`]. Does NOT replace that function: Good's own
/// specially-integrated branch and Healing's Rebuke Death (which has no
/// catalog spec at all) keep calling `domain_power_uses_per_day` directly
/// and are unaffected by this addition -- only the two GENERIC per-spec
/// dispatch loops (Cleric's `other_catalog_domains`, Inquisitor's single
/// resolved spec) call this function, since only those iterate over a live
/// `spec` that might carry an override. Floored at 0, the same PF1 "times
/// per day" floor `domain_power_uses_per_day` already applies. Unlike that
/// fixed-arity function (`3+WIS` never references a class level), this one
/// takes `class_level` because an override formula CAN (Animate Servant's
/// own does).
pub(super) fn domain_power_uses_per_day_for(
    spec: &DomainPowerSpec,
    class_level: u8,
    modifiers: &AbilityModifiers,
) -> i16 {
    let formula = spec.uses_per_day_formula.unwrap_or(DOMAIN_POWER_TIMES_FORMULA);
    let expr = parse_pcgen_expr(formula)
        .unwrap_or_else(|| panic!("catalog uses_per_day formula must parse: {formula}"));
    let env = domain_power_env(class_level, modifiers);
    let value = eval_expr(&expr, &env).unwrap_or_else(|| {
        panic!("catalog uses_per_day formula must resolve under domain_power_env: {formula}")
    });
    i16::try_from(value.max(0)).unwrap_or(i16::MAX)
}

/// One domain this catalog grounds for real: its granted power's magnitude
/// formula, transcribed verbatim from the corpus, plus enough provenance for
/// this module's own `fixture_check_tests` inline test module to pin it against the real
/// upstream `.lst` bytes.
pub(super) struct DomainPowerSpec {
    /// The `choice:cleric_domain`/`choice:inquisitor_domain` selection id
    /// this spec answers for, e.g. `GOOD_DOMAIN_SELECTION`.
    pub selection_id: &'static str,
    /// Lowercase, underscore-safe domain name (e.g. `"good"`, `"war"`),
    /// used to build this power's `class_feature.domain.<slug>_<ability_id>_*`
    /// explanation ids -- for Good, this reproduces
    /// `class_feature.domain.good_touch_of_good_self_application` and its
    /// siblings byte-for-byte, the ids the pre-existing tests already pin.
    pub domain_slug: &'static str,
    pub domain_display_name: &'static str,
    pub granted_power_name: &'static str,
    /// The `class_ability_activations` id this power's self-application
    /// activation is recorded under.
    pub ability_id: &'static str,
    /// Verbatim from the granted-power record's own `DESC` embedded
    /// formula segment (the text after the description's first `|`).
    pub magnitude_formula: &'static str,
    /// A short, honest label for what the magnitude number IS -- reused in
    /// this power's grounded explanation text.
    pub magnitude_label: &'static str,
    /// Verbatim-accurate framing of this power's own duration/trigger, as
    /// the corpus `DESC` text states it -- substituted into the shared
    /// "actively using {power} ... {effect_duration_phrase}" explanation
    /// sentence. Added SD-31 wave 26 when widening past Good/War/Strength
    /// (all three genuinely "for 1 round" per their own DESC text) surfaced
    /// a real accuracy gap: Destructive Smite is a single declared attack
    /// with no duration at all, and Touch of Glory's own DESC states "for
    /// one hour, or until the creature touched elects to apply the bonus
    /// to a roll" -- reusing a hardcoded "for 1 round" for either would be
    /// a plausible-looking but WRONG sentence, exactly the failure mode
    /// this seam exists to avoid.
    pub effect_duration_phrase: &'static str,
    /// SD-34 wave 37 lane A: `true` when `magnitude_formula` is a flat
    /// combat/skill/save bonus (every entry through SD-31 wave 26) -- `false`
    /// when the corpus's own formula slot is something else entirely (Undead
    /// Subdomain's Death's Kiss: an effect DURATION in rounds, not a bonus).
    /// Gates the "self_application"/"not_active" explanation block in BOTH
    /// `explain_cleric_level1_spell_baseline` and
    /// `ground_or_block_inquisitor_domain_power` -- when `false`, neither
    /// call site ever interpolates `magnitude_formula`'s value into the
    /// shared "a +{magnitude} {label} {duration}" sentence (which would
    /// misrepresent a round count as a game bonus); only this power's real,
    /// honestly-labeled uses-per-day is ever computed and reported for it.
    pub grounds_self_application: bool,
    /// SD-34 wave 38 lane A (wave 37 lane A's own next-cycle plan item 1):
    /// overrides the shared [`DOMAIN_POWER_TIMES_FORMULA`] (`3+WIS`) uses-
    /// per-day formula for a spec whose OWN corpus `DESC` formula slot IS
    /// the uses-per-day count, rather than a flat bonus -- Construct
    /// Subdomain's Animate Servant, whose `DESC`'s only formula segment is
    /// `DomainArtificeLVL/4-1`, confirmed the uses-per-day count (not a
    /// magnitude) by the sibling `"Domain Power ~ Animate Servant"` corpus
    /// record's own `ASPECT|CheckType|Uses per Day` token, read directly.
    /// `None` (every entry through wave 37, unchanged) computes uses-per-day
    /// via the shared `3+WIS` formula exactly as before this field existed;
    /// `Some(f)` interprets `f` under the SAME [`domain_power_env`] the
    /// magnitude formula already uses -- so, unlike the fixed `3+WIS`
    /// (which never references a class level), an override formula CAN
    /// depend on the granting class's own level. Does not remove or weaken
    /// the shared default for any pre-existing entry: this is an additive,
    /// opt-in override, read only by [`domain_power_uses_per_day_for`],
    /// which the pre-existing per-spec `3+WIS` call sites do not use.
    pub uses_per_day_formula: Option<&'static str>,
    // Provenance-only: read by this module's own `fixture_check_tests` (`catalog_provenance_
    // matches_the_corpus_records_own_source_citation`, below) against the corpus's own `source`
    // object, never by any PRODUCTION/runtime code path -- `cargo build --lib` (the non-test
    // profile) correctly flags them `dead_code` because that build never compiles `#[cfg(test)]`
    // code in. `#[allow]`d deliberately rather than deleted: these fields are load-bearing for the
    // provenance fixture test that already exists and passes, not unused dead weight.
    #[allow(dead_code)]
    pub upstream_lst: &'static str,
    #[allow(dead_code)]
    pub upstream_lst_sha256: &'static str,
    #[allow(dead_code)]
    pub upstream_line: u64,
}

/// The domains this seam grounds for real, replacing the previous
/// Good/Healing-only allowlist's ARITHMETIC. Every entry's `magnitude_formula`
/// is a self-application-safe, non-dice, single-`DESC`-token formula -- see
/// this module's own doc comment for exactly why Evil/Darkness/Madness (same
/// formula shape, enemy-facing effect) and Void/Scalykind (no corpus header
/// chain) are deliberately absent. Healing carries no `magnitude_formula`
/// (Rebuke Death's heal amount is a dice roll, `1d4+…`) -- unchanged from
/// before this seam, its uses-per-day is still grounded via
/// [`domain_power_uses_per_day`].
pub(super) const DOMAIN_POWER_CATALOG: &[DomainPowerSpec] = &[
    DomainPowerSpec {
        selection_id: GOOD_DOMAIN_SELECTION,
        domain_slug: "good",
        domain_display_name: "Good",
        granted_power_name: "Touch of Good",
        ability_id: TOUCH_OF_GOOD_ABILITY_ID,
        magnitude_formula: "max(DomainGoodLVL/2,1)",
        magnitude_label: "sacred bonus",
        effect_duration_phrase: "for 1 round",
        grounds_self_application: true,
        uses_per_day_formula: None,
        upstream_lst: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst",
        upstream_lst_sha256: "b2ce1a9db06e3921c0d6169040a21f85bec23e8ddfff0eda608247c04359a282",
        upstream_line: 713,
    },
    DomainPowerSpec {
        selection_id: WAR_DOMAIN_SELECTION,
        domain_slug: "war",
        domain_display_name: "War",
        granted_power_name: "Battle Rage",
        ability_id: BATTLE_RAGE_ABILITY_ID,
        magnitude_formula: "max(DomainWarLVL/2,1)",
        magnitude_label: "melee damage bonus",
        effect_duration_phrase: "for 1 round",
        grounds_self_application: true,
        uses_per_day_formula: None,
        upstream_lst: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst",
        upstream_lst_sha256: "b2ce1a9db06e3921c0d6169040a21f85bec23e8ddfff0eda608247c04359a282",
        upstream_line: 747,
    },
    DomainPowerSpec {
        selection_id: STRENGTH_DOMAIN_SELECTION,
        domain_slug: "strength",
        domain_display_name: "Strength",
        granted_power_name: "Strength Surge",
        ability_id: STRENGTH_SURGE_ABILITY_ID,
        magnitude_formula: "max(DomainStrengthLVL/2,1)",
        magnitude_label: "enhancement bonus",
        effect_duration_phrase: "for 1 round",
        grounds_self_application: true,
        uses_per_day_formula: None,
        upstream_lst: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst",
        upstream_lst_sha256: "b2ce1a9db06e3921c0d6169040a21f85bec23e8ddfff0eda608247c04359a282",
        upstream_line: 739,
    },
    // SD-31 wave 26 (OPERATOR-RULINGS-2026-08-21.md section 20; "PROVE BEFORE YOU
    // EXTEND" satisfied first -- see this module's own `fixture_check_tests`, which
    // reproduce Good/War/Strength's pinned values before either entry below is
    // added). Both scanned corpus-wide for the SAME self-application-safe shape
    // Good/War/Strength already establish (a beneficial effect on a touched/self
    // target, a single non-dice `DESC`-embedded magnitude formula, a real
    // `Domain<X>LVL`/`Domain<X>Times` header chain) -- confirmed against
    // `data/corpus/core_rulebook/class_feature/destruction/destruction.json` and
    // `.../glory/glory.json` respectively, both carrying the identical
    // `BONUS:VAR|Domain<X>LVL|DomainLVL|TYPE=Domain` /
    // `BONUS:VAR|Domain<X>Times|DomainPowerTimes|TYPE=Domain` chain Good/War/
    // Strength's own fixture test already verifies for those three.
    DomainPowerSpec {
        selection_id: DESTRUCTION_DOMAIN_SELECTION,
        domain_slug: "destruction",
        domain_display_name: "Destruction",
        granted_power_name: "Destructive Smite",
        ability_id: DESTRUCTIVE_SMITE_ABILITY_ID,
        magnitude_formula: "max(DomainDestructionLVL/2,1)",
        magnitude_label: "morale bonus on damage rolls",
        // Corpus DESC: "the supernatural ability to make a single melee attack
        // with a +%1 morale bonus on damage rolls. You must declare the
        // destructive smite before making the attack." No round-based duration
        // at all -- unlike Good/War/Strength, this is a single declared attack,
        // not a buff that persists for a round.
        effect_duration_phrase: "on a single melee attack, which must be declared before the attack roll is made",
        grounds_self_application: true,
        uses_per_day_formula: None,
        upstream_lst: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst",
        upstream_lst_sha256: "b2ce1a9db06e3921c0d6169040a21f85bec23e8ddfff0eda608247c04359a282",
        upstream_line: 703,
    },
    DomainPowerSpec {
        selection_id: GLORY_DOMAIN_SELECTION,
        domain_slug: "glory",
        domain_display_name: "Glory",
        granted_power_name: "Touch of Glory",
        ability_id: TOUCH_OF_GLORY_ABILITY_ID,
        // Bare `DomainGloryLVL` -- no `max(.../2,1)` wrap, unlike Good/War/
        // Strength/Destruction. Verified byte-identical to the corpus DESC's
        // own first formula segment by this module's own fixture test.
        magnitude_formula: "DomainGloryLVL",
        magnitude_label: "bonus to a single Charisma-based skill check or Charisma ability check",
        // Corpus DESC: "This ability lasts for one hour or until the creature
        // touched elects to apply the bonus to a roll." A real, different
        // duration shape from Good/War/Strength's "for 1 round".
        effect_duration_phrase:
            "for one hour, or until the creature touched elects to apply the bonus to a roll",
        grounds_self_application: true,
        uses_per_day_formula: None,
        upstream_lst: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst",
        upstream_lst_sha256: "b2ce1a9db06e3921c0d6169040a21f85bec23e8ddfff0eda608247c04359a282",
        upstream_line: 711,
    },
    // SD-34 wave 37 lane A (bucket D's "domain-vs-class_feature dual-
    // representation" mechanism gap, item 5 of wave 36 lane C's next-cycle
    // plan): the first APG SUBDOMAIN this catalog grounds, and the first
    // entry whose own corpus formula slot is NOT a flat combat/skill/save
    // bonus -- see `UNDEAD_SUBDOMAIN_SELECTION`'s own doc comment in
    // `pilot_compute/mod.rs` for the full provenance and why
    // `grounds_self_application` is `false` here. Confirmed a real, legal
    // domain for BOTH classes this catalog serves: Cleric (subdomain
    // substitution, `data/corpus/advanced_players_guide/domain/
    // undead_subdomain.json`'s own `PREMULT` gate) and Inquisitor
    // (`data/corpus/advanced_players_guide/class_feature/inquisitor/
    // inquisitor_domains.json`'s own `DEFINE:InquisitorDomainUndeadSubdomain|0`
    // token, confirmed present by direct corpus read).
    DomainPowerSpec {
        selection_id: UNDEAD_SUBDOMAIN_SELECTION,
        domain_slug: "undead_subdomain",
        domain_display_name: "Undead Subdomain",
        granted_power_name: "Death's Kiss",
        ability_id: DEATH_S_KISS_ABILITY_ID,
        // Verbatim from `data/corpus/advanced_players_guide/class_feature/
        // undead_subdomain/death_s_kiss.json`'s own DESC token's first `|`
        // segment: the power's effect DURATION in rounds, not a bonus
        // amount (unlike every entry above). `DomainLVL` is the bare,
        // universal granting-class-level variable `domain_power_env`
        // already resolves for every entry (no per-domain `Domain<X>LVL`
        // chain is needed for this term -- Undead Subdomain's own header
        // carries none at all, confirmed by direct corpus read).
        magnitude_formula: "max(1,DomainLVL/2)",
        // Unused in production (`grounds_self_application: false` skips the
        // block that would read this field) -- kept honest and test-pinned
        // for provenance, and so a future cycle that DOES honestly ground
        // this power's real effect (a status change, not a numeric bonus)
        // has the corpus-verified formula already on hand.
        magnitude_label: "rounds of undead-traits self-application \
            (a duration, not a numeric game bonus -- never surfaced as a \
            magnitude explanation, see `grounds_self_application`)",
        effect_duration_phrase: "",
        grounds_self_application: false,
        // Death's Kiss's own uses-per-day chains through `DomainDeathTimes`
        // to the SAME shared `DomainPowerTimes|3+WIS` chain every entry
        // above rides (confirmed by direct read of
        // `data/corpus/core_rulebook/class_feature/death/death.json`'s own
        // `BONUS:VAR|DomainDeathTimes|DomainPowerTimes|TYPE=Domain` token,
        // SD-34 wave 37 lane A) -- no override needed here.
        uses_per_day_formula: None,
        upstream_lst: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_abilities_class.lst",
        upstream_lst_sha256: "fab93d7178fc730992d62c21262dd2e9f8ff709304059478a38d860a912e58e3",
        upstream_line: 1807,
    },
    // SD-34 wave 38 lane A (wave 37 lane A's own next-cycle plan item 1):
    // the second APG SUBDOMAIN this catalog grounds, and the first entry
    // whose corpus `DESC` formula slot is BOTH its ONLY formula segment AND
    // genuinely the power's uses-per-day count -- not a magnitude at all
    // (unlike Death's Kiss, whose own formula slot is at least a real
    // magnitude-shaped number, an effect duration). Confirmed the formula
    // slot's real meaning by direct read of the sibling `"Domain Power ~
    // Animate Servant"` corpus record's own `ASPECT|CheckType|Uses per Day`
    // / `ASPECT|CheckCount|%1|DomainArtificeLVL/4-1` tokens
    // (`data/corpus/advanced_players_guide/class_feature/domain_power/
    // animate_servant.json`) -- not assumed from the `DESC` text's prose
    // alone ("you can use this ability %1 times per day" is itself
    // unambiguous, and the ASPECT tokens independently confirm it). A real,
    // legal domain for BOTH classes this catalog serves: Cleric (subdomain
    // substitution, `data/corpus/advanced_players_guide/domain/
    // construct_subdomain.json`'s own `PREMULT` gate:
    // `[PREDOMAIN:1,Construct Subdomain],[PREVARLT:ArtificeDomain,1]`) and
    // Inquisitor (`data/corpus/advanced_players_guide/class_feature/
    // inquisitor/inquisitor_domains.json`'s own
    // `DEFINE:InquisitorDomainConstructSubdomain|0` token, confirmed
    // present by direct corpus read). Its real effect ("you can give life
    // to inanimate objects. This ability functions as animate objects
    // using your cleric level as the caster level") is a spell-like
    // ability with no self-application buff magnitude at all to ground --
    // there is no honest number this catalog's magnitude/activation-state
    // shape could report for it, so `grounds_self_application` stays
    // `false` here exactly as it does for Death's Kiss, though for a
    // structurally DIFFERENT reason (Death's Kiss has a real formula that
    // is merely the wrong SHAPE to be a bonus; Animate Servant has no
    // bonus-shaped effect to formulate at all).
    DomainPowerSpec {
        selection_id: CONSTRUCT_SUBDOMAIN_SELECTION,
        domain_slug: "construct_subdomain",
        domain_display_name: "Construct Subdomain",
        granted_power_name: "Animate Servant",
        ability_id: ANIMATE_SERVANT_ABILITY_ID,
        // Verbatim from `data/corpus/advanced_players_guide/class_feature/
        // construct_subdomain/animate_servant.json`'s own DESC token's
        // first (and only) `|` segment. Kept here, byte/parse-checked by
        // this module's own `fixture_check_tests`, purely for provenance --
        // production code never reads this field as a magnitude
        // (`grounds_self_application: false` skips that block entirely).
        // `uses_per_day_formula` below is the field production code
        // actually reads for this power's real uses-per-day count -- the
        // SAME string, because this corpus record's only formula slot IS
        // that count.
        magnitude_formula: "DomainArtificeLVL/4-1",
        // Unused in production for the same reason `magnitude_formula`
        // itself is -- kept honest and test-pinned for provenance, and so
        // a future cycle that DOES honestly ground this power's real
        // effect (casting animate objects, a spell-like ability) has a
        // truthful label already on hand rather than a fabricated one.
        magnitude_label: "casts of animate objects \
            (a spell-like ability, not a numeric game bonus -- never \
            surfaced as a magnitude explanation, see \
            `grounds_self_application`)",
        effect_duration_phrase: "",
        grounds_self_application: false,
        // SD-34 wave 38 lane A: the field this entry exists to prove real
        // -- Animate Servant's own corpus formula slot IS its uses-per-day
        // count, genuinely different from the shared `3+WIS` every other
        // catalog entry (Death's Kiss included) rides.
        uses_per_day_formula: Some("DomainArtificeLVL/4-1"),
        upstream_lst: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_abilities_class.lst",
        upstream_lst_sha256: "fab93d7178fc730992d62c21262dd2e9f8ff709304059478a38d860a912e58e3",
        upstream_line: 1752,
    },
];

/// Builds this power's `class_feature.domain.<slug>_<ability_id>_<suffix>`
/// explanation id -- for Good this reproduces
/// `class_feature.domain.good_touch_of_good_self_application` (and its
/// `_not_active`/`_uses_per_day` siblings) byte-for-byte, the exact ids
/// pre-existing tests already pin, so swapping Good's own call sites over to
/// this builder changes no observable id.
pub(super) fn domain_power_explanation_id(spec: &DomainPowerSpec, suffix: &str) -> String {
    format!("class_feature.domain.{}_{}_{suffix}", spec.domain_slug, spec.ability_id)
}

/// Looks up `selection_id` (a `choice:*_domain` selection, e.g.
/// `"domain:good"`) in [`DOMAIN_POWER_CATALOG`]. `None` for Healing (which
/// has no magnitude formula to look up here -- its own caller grounds
/// uses/day directly via [`domain_power_uses_per_day`]) and for every domain
/// this catalog does not carry.
pub(super) fn resolve_domain_power(selection_id: &str) -> Option<&'static DomainPowerSpec> {
    DOMAIN_POWER_CATALOG.iter().find(|spec| spec.selection_id == selection_id)
}

/// AT-34-E3-001 bridge for `v06_work_inventory`'s completion-atlas classifier.
///
/// Each `docs/work-inventory.json` `"Domain Power ~ <granted power name>"`
/// unit (e.g. `"Domain Power ~ Touch of Good"`) is a real corpus record this
/// module has no relationship to unless its `granted_power_name` field
/// matches. This returns, for every catalog entry, the exact `(selection_id,
/// granted_power_name, [explanation ids])` triple the classifier needs to run
/// its OWN probe (select `selection_id` on a real cleric, sweep the real
/// pipeline, and check whether any of the three ids is genuinely emitted) --
/// never a static "this power is covered" claim, since only a live
/// computation can tell a genuinely-wired domain (Good, War, Strength,
/// Destruction, Glory) from one this catalog does not carry a formula for at
/// all (every other CRB domain).
pub fn domain_power_probe_catalog() -> Vec<(&'static str, &'static str, [String; 3])> {
    DOMAIN_POWER_CATALOG
        .iter()
        .map(|spec| {
            (
                spec.selection_id,
                spec.granted_power_name,
                [
                    domain_power_explanation_id(spec, "self_application"),
                    domain_power_explanation_id(spec, "not_active"),
                    domain_power_explanation_id(spec, "uses_per_day"),
                ],
            )
        })
        .collect()
}

/// SD-34 wave 37 lane A bridge for `v06_work_inventory`'s completion-atlas
/// classifier: the SIBLING corpus shape to `"Domain Power ~ <power>"` --
/// every catalog entry's granted power is ALSO ingested a second time under
/// its own domain's key (`"<domain_display_name> ~ <power>"`, e.g. `"Undead
/// Subdomain ~ Death's Kiss"`, a real, separate `.lst` line confirmed by
/// direct corpus read, not a duplicate to collapse). Returns each catalog
/// entry's own `(domain_display_name, granted_power_name)` pair so the
/// classifier can match a unit's own `"<group> ~ <feature>"` split against a
/// SPECIFIC, named catalog spec -- never a bare feature-name match, which
/// would wrongly credit an unrelated same-named record from a different
/// mechanism (`"Rage Power ~ Strength Surge"` and `"Strength Blessing ~
/// Strength Surge"` both collide with this catalog's own `"Strength Surge"`
/// granted-power name, confirmed by direct corpus scan -- a bare
/// feature-name check would misclassify both as Cleric/Inquisitor domain
/// power).
pub fn domain_power_catalog_group_and_power_names() -> Vec<(&'static str, &'static str)> {
    DOMAIN_POWER_CATALOG.iter().map(|spec| (spec.domain_display_name, spec.granted_power_name)).collect()
}

/// Interprets `spec`'s own `magnitude_formula` at `class_level`. `expect`s
/// success: every catalog entry's formula is a fixed literal this module's
/// own tests parse-check at `cargo test` time (`domain_power_catalog_formulas_all_parse`),
/// so a parse failure here would mean the catalog itself is malformed, not a
/// live-input problem.
pub(super) fn domain_power_magnitude(
    spec: &DomainPowerSpec,
    class_level: u8,
    modifiers: &AbilityModifiers,
) -> i16 {
    let expr = parse_pcgen_expr(spec.magnitude_formula)
        .unwrap_or_else(|| panic!("catalog formula must parse: {}", spec.magnitude_formula));
    let env = domain_power_env(class_level, modifiers);
    let value = eval_expr(&expr, &env).unwrap_or_else(|| {
        panic!("catalog formula must resolve under domain_power_env: {}", spec.magnitude_formula)
    });
    i16::try_from(value).unwrap_or(i16::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_bare_ability_sum() {
        assert_eq!(
            parse_pcgen_expr("3+WIS"),
            Some(Expr::Add(Box::new(Expr::Int(3)), Box::new(Expr::Var("WIS".to_owned()))))
        );
    }

    #[test]
    fn parses_max_of_a_division_and_a_literal() {
        let parsed = parse_pcgen_expr("max(DomainGoodLVL/2,1)").expect("must parse");
        assert_eq!(
            parsed,
            Expr::Max(
                Box::new(Expr::Div(
                    Box::new(Expr::Var("DomainGoodLVL".to_owned())),
                    Box::new(Expr::Int(2))
                )),
                Box::new(Expr::Int(1))
            )
        );
    }

    #[test]
    fn refuses_dice_notation() {
        assert_eq!(parse_pcgen_expr("1d4"), None, "dice notation is not this grammar's shape");
    }

    #[test]
    fn refuses_a_prevar_fragment() {
        assert_eq!(
            parse_pcgen_expr("PREVARLT:DomainHealingLVL,2"),
            None,
            "a PREVARLT condition is not an arithmetic expression"
        );
    }

    #[test]
    fn refuses_trailing_garbage() {
        assert_eq!(parse_pcgen_expr("3+WIS,"), None, "an unconsumed trailing comma must refuse");
        assert_eq!(parse_pcgen_expr("3+WIS)"), None, "an unconsumed trailing paren must refuse");
    }

    #[test]
    fn evaluates_max_domain_good_lvl_over_two_floored_at_one() {
        let expr = parse_pcgen_expr("max(DomainGoodLVL/2,1)").expect("must parse");
        for (level, expected) in [(1u8, 1i32), (2, 1), (3, 1), (4, 2), (6, 3), (8, 4), (20, 10)] {
            let env = domain_power_env(level, &AbilityModifiers::default());
            assert_eq!(
                eval_expr(&expr, &env),
                Some(expected),
                "level {level} should evaluate to {expected}"
            );
        }
    }

    #[test]
    fn domain_power_uses_per_day_matches_the_pre_existing_hand_arithmetic() {
        for wisdom in -5i16..=10 {
            let modifiers = AbilityModifiers { wisdom, ..AbilityModifiers::default() };
            let interpreted = domain_power_uses_per_day(&modifiers);
            let hand_written = (3 + wisdom).max(0);
            assert_eq!(
                interpreted, hand_written,
                "interpreted 3+WIS must match the pre-existing hand-written formula at WIS \
                 modifier {wisdom}"
            );
        }
    }

    /// `cleric_touch_of_good_bonus` now WRAPS this module's own interpreter
    /// (SD-31 wave 25), so comparing the two directly would be a tautology.
    /// This pins the interpreted output against an expected table computed
    /// independently of BOTH `cleric_touch_of_good_bonus` and this module's
    /// evaluator -- by hand, from PF1 Core Rulebook Good Domain's own rule
    /// text ("half the cleric's level, minimum 1") -- so a bug shared by
    /// both the wrapper and the interpreter still fails this test.
    #[test]
    fn domain_power_magnitude_for_good_matches_an_independently_hand_computed_table() {
        let spec = resolve_domain_power(GOOD_DOMAIN_SELECTION).expect("Good must be catalogued");
        let expected_half_level_min_one: [(u8, i16); 8] =
            [(1, 1), (2, 1), (3, 1), (4, 2), (5, 2), (10, 5), (19, 9), (20, 10)];
        for (level, expected) in expected_half_level_min_one {
            let interpreted = domain_power_magnitude(spec, level, &AbilityModifiers::default());
            assert_eq!(
                interpreted, expected,
                "PF1 Good Domain Touch of Good at cleric level {level}: half level, minimum 1"
            );
        }
    }

    #[test]
    fn every_catalog_entry_has_a_unique_selection_and_ability_id() {
        let mut selections: Vec<&str> =
            DOMAIN_POWER_CATALOG.iter().map(|s| s.selection_id).collect();
        selections.sort_unstable();
        let mut deduped = selections.clone();
        deduped.dedup();
        assert_eq!(selections, deduped, "no two catalog entries may share a selection id");

        let mut ability_ids: Vec<&str> =
            DOMAIN_POWER_CATALOG.iter().map(|s| s.ability_id).collect();
        ability_ids.sort_unstable();
        let mut deduped_abilities = ability_ids.clone();
        deduped_abilities.dedup();
        assert_eq!(
            ability_ids, deduped_abilities,
            "no two catalog entries may share an activation ability id"
        );
    }

    #[test]
    fn good_spec_explanation_ids_match_the_pre_existing_pinned_strings() {
        let spec = resolve_domain_power(GOOD_DOMAIN_SELECTION).expect("Good must be catalogued");
        assert_eq!(
            domain_power_explanation_id(spec, "self_application"),
            "class_feature.domain.good_touch_of_good_self_application"
        );
        assert_eq!(
            domain_power_explanation_id(spec, "not_active"),
            "class_feature.domain.good_touch_of_good_not_active"
        );
        assert_eq!(
            domain_power_explanation_id(spec, "uses_per_day"),
            "class_feature.domain.good_touch_of_good_uses_per_day"
        );
    }

    #[test]
    fn domain_power_catalog_formulas_all_parse() {
        for spec in DOMAIN_POWER_CATALOG {
            assert!(
                parse_pcgen_expr(spec.magnitude_formula).is_some(),
                "catalog formula must parse: {} ({})",
                spec.magnitude_formula,
                spec.domain_display_name
            );
        }
    }

    /// SD-34 wave 38 lane A: the sibling of `domain_power_catalog_formulas_
    /// all_parse` for the new `uses_per_day_formula` override field -- every
    /// catalog entry that carries one must have a formula this grammar can
    /// actually parse (entries carrying `None` are exempt, since `None`
    /// never reaches `parse_pcgen_expr` at all -- `domain_power_uses_per_day_for`
    /// substitutes `DOMAIN_POWER_TIMES_FORMULA` instead).
    #[test]
    fn domain_power_catalog_uses_per_day_override_formulas_all_parse() {
        for spec in DOMAIN_POWER_CATALOG {
            if let Some(formula) = spec.uses_per_day_formula {
                assert!(
                    parse_pcgen_expr(formula).is_some(),
                    "catalog uses_per_day_formula override must parse: {formula} ({})",
                    spec.domain_display_name
                );
            }
        }
    }

    /// SD-34 wave 38 lane A: a regression guard mirroring `death_s_kiss_
    /// does_not_ground_a_self_application_bonus` -- Animate Servant must be
    /// catalogued with `grounds_self_application: false` (its real effect
    /// is a spell-like ability, no bonus to ground) AND a real
    /// `uses_per_day_formula` override (its corpus formula slot is the
    /// uses-per-day count, not `3+WIS`); every OTHER entry (Death's Kiss
    /// included) must keep `uses_per_day_formula: None` -- a future edit
    /// that widened the override to an entry whose uses/day genuinely IS
    /// the shared `3+WIS` chain would silently stop testing the shared
    /// path for that entry.
    #[test]
    fn animate_servant_does_not_ground_a_self_application_bonus_and_carries_a_uses_per_day_override()
     {
        let spec = resolve_domain_power(CONSTRUCT_SUBDOMAIN_SELECTION)
            .expect("Construct Subdomain must be catalogued");
        assert!(
            !spec.grounds_self_application,
            "Animate Servant's real effect is a spell-like ability, not a flat bonus -- \
             grounds_self_application must stay false"
        );
        assert_eq!(
            spec.uses_per_day_formula,
            Some("DomainArtificeLVL/4-1"),
            "Animate Servant's own corpus DESC formula slot IS its uses-per-day count"
        );
        for other in [
            GOOD_DOMAIN_SELECTION,
            WAR_DOMAIN_SELECTION,
            STRENGTH_DOMAIN_SELECTION,
            DESTRUCTION_DOMAIN_SELECTION,
            GLORY_DOMAIN_SELECTION,
            UNDEAD_SUBDOMAIN_SELECTION,
        ] {
            let other_spec = resolve_domain_power(other).expect("must be catalogued");
            assert_eq!(
                other_spec.uses_per_day_formula, None,
                "{} must keep the shared 3+WIS uses-per-day formula (uses_per_day_formula: \
                 None), unchanged by this cycle's own additive widening",
                other_spec.domain_display_name
            );
        }
    }

    /// SD-34 wave 38 lane A: the classifier bridge's own pairing must name
    /// Animate Servant under Construct Subdomain's real display name,
    /// matching the corpus's own `"Construct Subdomain ~ Animate Servant"`
    /// key -- proves `domain_power_catalog_group_and_power_names` (consumed
    /// by `v06_work_inventory`'s classifier) actually carries this entry,
    /// mirroring `group_and_power_names_bridge_carries_death_s_kiss` above.
    #[test]
    fn group_and_power_names_bridge_carries_animate_servant() {
        let pairs = domain_power_catalog_group_and_power_names();
        assert!(
            pairs.contains(&("Construct Subdomain", "Animate Servant")),
            "expected (\"Construct Subdomain\", \"Animate Servant\") among {pairs:?}"
        );
    }

    /// SD-34 wave 38 lane A: `domain_power_uses_per_day_for` must correctly
    /// dispatch to a spec's own override when one is present, rather than
    /// always falling back to the shared `3+WIS` -- proven directly against
    /// `domain_power_uses_per_day`'s own output on the SAME modifiers, which
    /// must differ from the override's output (a passing test that happened
    /// to agree by coincidence would not prove the branch was taken).
    #[test]
    fn domain_power_uses_per_day_for_uses_the_override_when_present() {
        let spec = resolve_domain_power(CONSTRUCT_SUBDOMAIN_SELECTION)
            .expect("Construct Subdomain must be catalogued");
        let modifiers = AbilityModifiers { wisdom: 1, ..AbilityModifiers::default() };
        let shared_formula_value = domain_power_uses_per_day(&modifiers);
        let overridden_value = domain_power_uses_per_day_for(spec, 12, &modifiers);
        assert_eq!(
            overridden_value, 2,
            "DomainArtificeLVL/4-1 at level 12: 12/4-1 = 2, floored at 0"
        );
        assert_ne!(
            overridden_value, shared_formula_value,
            "the override must produce a DIFFERENT value than the shared 3+WIS formula would \
             at this Wisdom modifier ({shared_formula_value}), proving the override branch was \
             genuinely taken rather than silently falling back"
        );
    }

    /// SD-34 wave 38 lane A: a spec with NO override (`uses_per_day_formula:
    /// None`) must still resolve to the shared `3+WIS` formula through
    /// `domain_power_uses_per_day_for` -- the fallback half of the branch
    /// `domain_power_uses_per_day_for_uses_the_override_when_present` proves
    /// the override half of.
    #[test]
    fn domain_power_uses_per_day_for_falls_back_to_the_shared_formula_when_absent() {
        let spec =
            resolve_domain_power(GOOD_DOMAIN_SELECTION).expect("Good must be catalogued");
        for wisdom in -5i16..=10 {
            let modifiers = AbilityModifiers { wisdom, ..AbilityModifiers::default() };
            assert_eq!(
                domain_power_uses_per_day_for(spec, 7, &modifiers),
                domain_power_uses_per_day(&modifiers),
                "Good has no uses_per_day_formula override -- must match the shared 3+WIS \
                 formula at every class level, WIS modifier {wisdom}"
            );
        }
    }
}

/// This module's own fixture gate -- the `derived_evaluator_fixture_check`
/// rigor, without touching that proof harness (out of this lane's granted
/// write scope; SD-31 wave 25 brief, "Files: the domain-power functions in
/// src/rules_core/pilot_compute/ and their fixtures"). Mirrors
/// `tests/derived_evaluator_fixture_check.rs`'s own module doc's four
/// independent guarantees:
///
/// 1. **Different source artifact.** Every assertion below reads the corpus
///    JSON via `include_str!`, a file this module's own production code
///    never opens (the production path takes a formula STRING already
///    embedded as a Rust `&'static str` constant in [`DOMAIN_POWER_CATALOG`]
///    -- it performs no file I/O at all, consistent with the rest of
///    `pilot_compute`, which never reads `data/corpus` at compute time).
/// 2. **Re-derivable from the pinned corpus text.**
///    [`good_war_strength_headers_share_the_domainlvl_and_domainpowertimes_chain`]
///    and [`granted_power_magnitude_formulas_are_byte_identical_to_the_corpus`]
///    re-read each pinned corpus field and assert this module's own
///    `&'static str` constants match it byte-for-byte -- a hand-transcribed
///    formula that drifted from the corpus fails here.
/// 3. **Anchored to the same upstream bytes.**
///    [`catalog_provenance_matches_the_corpus_records_own_source_citation`]
///    compares each [`DomainPowerSpec`]'s `upstream_lst`/`upstream_lst_sha256`/
///    `upstream_line` against the SAME corpus JSON's own `source` object.
/// 4. **Independently computed expected values, never read from this
///    module's own evaluator.**
///    [`interpreted_magnitude_matches_a_hand_computed_table_derived_from_pf1_rule_text`]
///    -- the same discipline `good_spec_explanation_ids_match_the_pre_existing_pinned_strings`'s
///    sibling test above already uses for Good alone, extended here to War
///    and Strength and tied to the corpus bytes directly.
///
/// Mutation-provable: flip `eval_expr`'s `Expr::Div` to multiply, or
/// `Expr::Max` to `min`, or change any `DOMAIN_POWER_CATALOG` formula
/// string, and guarantee 2 or guarantee 4 below goes red.
#[cfg(test)]
mod fixture_check_tests {
    use super::*;

    const DOMAINS_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/domains/domains.json");
    const GOOD_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/good/good.json");
    const WAR_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/war/war.json");
    const STRENGTH_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/strength/strength.json");
    const TOUCH_OF_GOOD_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/touch_of_good.json"
    );
    const BATTLE_RAGE_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/battle_rage.json"
    );
    const STRENGTH_SURGE_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/strength_surge.json"
    );
    // SD-31 wave 26 additions.
    const DESTRUCTION_HEADER_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/destruction/destruction.json"
    );
    const GLORY_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/glory/glory.json");
    const DESTRUCTIVE_SMITE_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/destructive_smite.json"
    );
    const TOUCH_OF_GLORY_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/touch_of_glory.json"
    );
    // SD-34 wave 37 lane A addition. Pinned against the SUBDOMAIN-keyed
    // record (`"Undead Subdomain ~ Death's Kiss"`), matching this catalog
    // entry's own `domain_display_name` -- the sibling `"Domain Power ~
    // Death's Kiss"` record (a real, separate `.lst` line, different
    // `upstream_line`) is a different corpus key this same formula also
    // byte-matches, confirmed by direct read, but is not this test's own
    // provenance anchor.
    const DEATH_S_KISS_JSON: &str = include_str!(
        "../../../data/corpus/advanced_players_guide/class_feature/undead_subdomain/death_s_kiss.json"
    );
    // SD-34 wave 38 lane A addition. Pinned against the SUBDOMAIN-keyed
    // record (`"Construct Subdomain ~ Animate Servant"`), matching this
    // catalog entry's own `domain_display_name` -- the sibling `"Domain
    // Power ~ Animate Servant"` record (a real, separate `.lst` line,
    // different `upstream_line`, carrying the confirming `ASPECT|CheckType|
    // Uses per Day` token) is a different corpus key this same formula also
    // byte-matches, confirmed by direct read, but is not this test's own
    // provenance anchor.
    const ANIMATE_SERVANT_JSON: &str = include_str!(
        "../../../data/corpus/advanced_players_guide/class_feature/construct_subdomain/animate_servant.json"
    );

    fn parse(json: &str) -> serde_json::Value {
        serde_json::from_str(json).expect("committed corpus JSON must parse")
    }

    /// Every `BONUS` token value on a corpus record, in file order.
    fn bonus_values(doc: &serde_json::Value) -> Vec<String> {
        doc["data"]["raw_tokens"]
            .as_array()
            .expect("raw_tokens array")
            .iter()
            .filter(|t| t["key"].as_str() == Some("BONUS"))
            .map(|t| t["value"].as_str().expect("BONUS value").to_owned())
            .collect()
    }

    /// Guarantee 1/2's structural half: confirms `domain_power_env`'s core
    /// assumption -- "any `LVL`-suffixed variable resolves to the granting
    /// class's own level, with no per-domain offset" -- is what the corpus
    /// ACTUALLY states for every domain this module grounds, rather than an
    /// assumption carried over from Good alone. Also confirms the
    /// uses-per-day chain (`Domain<X>Times|DomainPowerTimes|TYPE=Domain`)
    /// for all five. SD-31 wave 26 widened this from three (Good/War/
    /// Strength) to five (+Destruction/Glory) -- same assertion, more
    /// domains, per this lane's "prove before you extend" requirement.
    #[test]
    fn catalog_domain_headers_share_the_domainlvl_and_domainpowertimes_chain() {
        for (json, domain) in [
            (GOOD_HEADER_JSON, "Good"),
            (WAR_HEADER_JSON, "War"),
            (STRENGTH_HEADER_JSON, "Strength"),
            (DESTRUCTION_HEADER_JSON, "Destruction"),
            (GLORY_HEADER_JSON, "Glory"),
        ] {
            let doc = parse(json);
            let bonuses = bonus_values(&doc);
            assert!(
                bonuses.iter().any(|b| {
                    b.starts_with("VAR|Domain") && b.ends_with("LVL|DomainLVL|TYPE=Domain")
                }),
                "{domain} domain header must chain its own LVL var to the shared DomainLVL: {bonuses:?}"
            );
            assert!(
                bonuses.iter().any(|b| {
                    b.starts_with("VAR|Domain")
                        && b.ends_with("Times|DomainPowerTimes|TYPE=Domain")
                }),
                "{domain} domain header must chain its own Times var to the shared \
                 DomainPowerTimes: {bonuses:?}"
            );
        }
    }

    /// The shared uses-per-day formula itself: [`DOMAIN_POWER_TIMES_FORMULA`]
    /// must be byte-for-byte what `domains.json`'s own `BONUS:VAR|DomainPowerTimes|…`
    /// token states, not a hand-recalled `"3+WIS"`.
    #[test]
    fn domain_power_times_formula_constant_is_byte_identical_to_the_corpus() {
        let doc = parse(DOMAINS_HEADER_JSON);
        let bonuses = bonus_values(&doc);
        let corpus_formula = bonuses
            .iter()
            .find_map(|b| b.strip_prefix("VAR|DomainPowerTimes|"))
            .expect("domains.json must carry a BONUS:VAR|DomainPowerTimes| token");
        assert_eq!(
            corpus_formula, DOMAIN_POWER_TIMES_FORMULA,
            "DOMAIN_POWER_TIMES_FORMULA must match the corpus's own DomainPowerTimes formula \
             byte-for-byte"
        );
    }

    /// Each catalog entry's own `magnitude_formula` must be byte-for-byte
    /// what the granted-power record's `DESC` token embeds as its FIRST
    /// formula segment (PCGen's own `%1` substitution slot) -- not a
    /// hand-recalled or hand-simplified rewrite.
    #[test]
    fn granted_power_magnitude_formulas_are_byte_identical_to_the_corpus() {
        for (json, selection_id) in [
            (TOUCH_OF_GOOD_JSON, GOOD_DOMAIN_SELECTION),
            (BATTLE_RAGE_JSON, WAR_DOMAIN_SELECTION),
            (STRENGTH_SURGE_JSON, STRENGTH_DOMAIN_SELECTION),
            (DESTRUCTIVE_SMITE_JSON, DESTRUCTION_DOMAIN_SELECTION),
            (TOUCH_OF_GLORY_JSON, GLORY_DOMAIN_SELECTION),
            (DEATH_S_KISS_JSON, UNDEAD_SUBDOMAIN_SELECTION),
            (ANIMATE_SERVANT_JSON, CONSTRUCT_SUBDOMAIN_SELECTION),
        ] {
            let doc = parse(json);
            let desc = doc["data"]["raw_tokens"]
                .as_array()
                .expect("raw_tokens")
                .iter()
                .find(|t| t["key"].as_str() == Some("DESC"))
                .expect("a DESC token")["value"]
                .as_str()
                .expect("DESC value")
                .to_owned();
            let first_formula_segment = desc
                .split('|')
                .nth(1)
                .expect("DESC must carry at least one %N formula segment after the description text");
            let spec = resolve_domain_power(selection_id).expect("must be catalogued");
            assert_eq!(
                first_formula_segment, spec.magnitude_formula,
                "{}'s magnitude_formula must match the corpus DESC's own %1 formula segment \
                 byte-for-byte",
                spec.domain_display_name
            );
        }
    }

    /// Each catalog entry's `upstream_lst`/`upstream_lst_sha256`/`upstream_line`
    /// must match the SAME corpus JSON's own `source` object -- the anchor
    /// guarantee 4 names: if the corpus is ever regenerated against a
    /// different upstream revision, this goes red instead of silently
    /// comparing two different rows.
    #[test]
    fn catalog_provenance_matches_the_corpus_records_own_source_citation() {
        for (json, selection_id) in [
            (TOUCH_OF_GOOD_JSON, GOOD_DOMAIN_SELECTION),
            (BATTLE_RAGE_JSON, WAR_DOMAIN_SELECTION),
            (STRENGTH_SURGE_JSON, STRENGTH_DOMAIN_SELECTION),
            (DESTRUCTIVE_SMITE_JSON, DESTRUCTION_DOMAIN_SELECTION),
            (TOUCH_OF_GLORY_JSON, GLORY_DOMAIN_SELECTION),
            (DEATH_S_KISS_JSON, UNDEAD_SUBDOMAIN_SELECTION),
            (ANIMATE_SERVANT_JSON, CONSTRUCT_SUBDOMAIN_SELECTION),
        ] {
            let doc = parse(json);
            let spec = resolve_domain_power(selection_id).expect("must be catalogued");
            assert_eq!(
                doc["source"]["path"].as_str().expect("source.path"),
                spec.upstream_lst,
                "{} upstream_lst mismatch",
                spec.domain_display_name
            );
            assert_eq!(
                doc["source"]["sha256"].as_str().expect("source.sha256"),
                spec.upstream_lst_sha256,
                "{} upstream_lst_sha256 mismatch",
                spec.domain_display_name
            );
            assert_eq!(
                doc["source"]["line"].as_u64().expect("source.line"),
                spec.upstream_line,
                "{} upstream_line mismatch",
                spec.domain_display_name
            );
        }
    }

    /// Guarantee 4: expected values computed BY HAND from PF1 Core Rulebook
    /// Domains' own granted-power rule text ("half the domain's effective
    /// level, minimum 1" -- Good, War, Strength, and Destruction all share
    /// this exact magnitude shape per their own `DESC` text, independently
    /// confirmed this cycle), never read back from [`eval_expr`] or
    /// [`domain_power_magnitude`]. A mutated evaluator (e.g. `Div` swapped
    /// for `Mul`, or `Max` swapped for `Min`) fails this test even though it
    /// would still satisfy the transcription-only tests above.
    #[test]
    fn interpreted_magnitude_matches_a_hand_computed_table_derived_from_pf1_rule_text() {
        let half_level_min_one = |level: u8| -> i16 { (i16::from(level) / 2).max(1) };
        for selection_id in [
            GOOD_DOMAIN_SELECTION,
            WAR_DOMAIN_SELECTION,
            STRENGTH_DOMAIN_SELECTION,
            DESTRUCTION_DOMAIN_SELECTION,
            // Death's Kiss's own formula is `max(1,DomainLVL/2)` -- args
            // reversed from Good/War/Strength/Destruction's `max(X/2,1)`,
            // but `max` is commutative so the same hand-computed table
            // applies. (Its VALUE is a round count, never surfaced as a
            // bonus -- see `grounds_self_application` -- but the arithmetic
            // itself is still real and worth proving correct.)
            UNDEAD_SUBDOMAIN_SELECTION,
        ] {
            let spec = resolve_domain_power(selection_id).expect("must be catalogued");
            for level in [1u8, 2, 3, 4, 7, 12, 20] {
                let expected = half_level_min_one(level);
                let interpreted =
                    domain_power_magnitude(spec, level, &AbilityModifiers::default());
                assert_eq!(
                    interpreted, expected,
                    "{} magnitude at level {level}: expected half-level-minimum-one = {expected}",
                    spec.domain_display_name
                );
            }
        }
    }

    /// Guarantee 4, Glory's own shape: unlike the four half-level-minimum-one
    /// entries above, Touch of Glory's own `DESC` formula segment is the bare
    /// `DomainGloryLVL` (no `max(.../2,1)` wrap) -- read directly off the
    /// pinned corpus text (`granted_power_magnitude_formulas_are_byte_
    /// identical_to_the_corpus`, above, already pins this exact string), not
    /// re-derived from an external source this session had no access to
    /// verify against. `DomainGloryLVL` chains 1:1 to `DomainLVL` (the
    /// granting class's own level, per `catalog_domain_headers_share_the_
    /// domainlvl_and_domainpowertimes_chain` above) with no division at all,
    /// so the expected value here is simply the character level, unhalved.
    /// A mutated evaluator that silently divided this bare variable by 2
    /// (the Good/War/Strength/Destruction shape) would still parse and run,
    /// but fails THIS hand-computed table.
    #[test]
    fn interpreted_magnitude_for_glory_matches_a_hand_computed_table_derived_from_pf1_rule_text() {
        let spec = resolve_domain_power(GLORY_DOMAIN_SELECTION).expect("Glory must be catalogued");
        for level in [1u8, 2, 3, 4, 7, 12, 20] {
            let expected = i16::from(level);
            let interpreted = domain_power_magnitude(spec, level, &AbilityModifiers::default());
            assert_eq!(
                interpreted, expected,
                "Glory magnitude at level {level}: expected the bare cleric level = {expected}"
            );
        }
    }

    /// SD-34 wave 37 lane A: Death's Kiss must be catalogued with
    /// `grounds_self_application: false` -- a regression guard against a
    /// future edit accidentally flipping it back to `true`, which would
    /// re-enable the misleading "a +{magnitude} rounds" bonus sentence this
    /// entry exists specifically to avoid. Every pre-existing entry stays
    /// `true`.
    #[test]
    fn death_s_kiss_does_not_ground_a_self_application_bonus() {
        let spec = resolve_domain_power(UNDEAD_SUBDOMAIN_SELECTION)
            .expect("Undead Subdomain must be catalogued");
        assert!(
            !spec.grounds_self_application,
            "Death's Kiss's own corpus formula is an effect duration in rounds, not a flat \
             bonus -- grounds_self_application must stay false"
        );
        for other in [
            GOOD_DOMAIN_SELECTION,
            WAR_DOMAIN_SELECTION,
            STRENGTH_DOMAIN_SELECTION,
            DESTRUCTION_DOMAIN_SELECTION,
            GLORY_DOMAIN_SELECTION,
        ] {
            let other_spec = resolve_domain_power(other).expect("must be catalogued");
            assert!(
                other_spec.grounds_self_application,
                "{} is a real flat bonus and must keep grounds_self_application: true",
                other_spec.domain_display_name
            );
        }
    }

    /// SD-34 wave 37 lane A: the classifier bridge's own pairing must name
    /// Death's Kiss under its subdomain's real display name, matching the
    /// corpus's own `"Undead Subdomain ~ Death's Kiss"` key -- proves
    /// `domain_power_catalog_group_and_power_names` (consumed by
    /// `v06_work_inventory`'s classifier) actually carries this entry.
    #[test]
    fn group_and_power_names_bridge_carries_death_s_kiss() {
        let pairs = domain_power_catalog_group_and_power_names();
        assert!(
            pairs.contains(&("Undead Subdomain", "Death's Kiss")),
            "expected (\"Undead Subdomain\", \"Death's Kiss\") among {pairs:?}"
        );
    }

    /// SD-34 wave 38 lane A: Animate Servant must be catalogued with a real
    /// `uses_per_day_formula` override, and `grounds_self_application` must
    /// stay `false` -- a regression guard against a future edit accidentally
    /// clearing the override back to `None` (which would silently revert
    /// this power to the WRONG shared `3+WIS` uses-per-day count) or
    /// flipping `grounds_self_application` to `true` (which would fabricate
    /// a "+{magnitude}" bonus sentence for a spell-like ability with no
    /// bonus to ground at all).
    #[test]
    fn animate_servant_carries_a_uses_per_day_override_and_no_self_application_bonus() {
        let spec = resolve_domain_power(CONSTRUCT_SUBDOMAIN_SELECTION)
            .expect("Construct Subdomain must be catalogued");
        assert!(
            !spec.grounds_self_application,
            "Animate Servant's real effect is a spell-like ability, not a flat bonus -- \
             grounds_self_application must stay false"
        );
        assert_eq!(
            spec.uses_per_day_formula,
            Some("DomainArtificeLVL/4-1"),
            "Animate Servant's own corpus formula slot IS its uses-per-day count"
        );
    }

    /// Guarantee 4, Animate Servant's own shape: expected values computed BY
    /// HAND from the corpus DESC text ("You can use this ability %1 times
    /// per day.|DomainArtificeLVL/4-1", confirmed the uses-per-day count --
    /// not a magnitude -- by the sibling record's own `ASPECT|CheckType|Uses
    /// per Day` token, read directly, not assumed from prose alone), never
    /// read back from [`eval_expr`] or [`domain_power_uses_per_day_for`]:
    /// class level divided by 4 (PCGen integer division, truncating toward
    /// zero), minus 1, floored at 0 overall (PF1's own "times per day"
    /// floor). A mutated evaluator (e.g. `Div` swapped for `Mul`, or the
    /// `Sub` in `X-1` swapped for `Add`) fails this test even though it
    /// would still satisfy the transcription-only tests above.
    #[test]
    fn interpreted_uses_per_day_for_animate_servant_matches_a_hand_computed_table_derived_from_pf1_rule_text()
     {
        let spec = resolve_domain_power(CONSTRUCT_SUBDOMAIN_SELECTION)
            .expect("Construct Subdomain must be catalogued");
        let expected_floor_level_over_4_minus_1_floored_at_0 =
            |level: u8| -> i16 { (i16::from(level) / 4 - 1).max(0) };
        for level in [1u8, 4, 7, 8, 11, 12, 16, 20] {
            let expected = expected_floor_level_over_4_minus_1_floored_at_0(level);
            let interpreted =
                domain_power_uses_per_day_for(spec, level, &AbilityModifiers::default());
            assert_eq!(
                interpreted, expected,
                "Animate Servant uses/day at class level {level}: expected \
                 floor(level/4)-1, floored at 0 overall = {expected}"
            );
        }
    }
}
