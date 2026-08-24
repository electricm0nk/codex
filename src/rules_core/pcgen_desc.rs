//! Renders a PCGen `DESC:` token into the prose a player may actually be
//! shown, and guards against the raw token reaching a screen.
//!
//! ## Why this exists as a shared module rather than a fourth private copy
//!
//! Three ingest binaries already carry their own private copy of this
//! treatment (`src/bin/ingest_races.rs`, `src/bin/ingest_race_traits.rs`,
//! `src/bin/ingest_pu_classes.rs`). Each of those reads a `.lst` row and can
//! resolve `%N` against that row's own `DEFINE:`/`BONUS:VAR` literals, so the
//! copies are genuinely row-shaped and are left alone.
//!
//! Spell descriptions are different in two ways that made a fourth private
//! copy the wrong answer:
//!
//! 1. **The leak reaches the player from the compiled tables, not from
//!    `data/corpus/`.** `apps/desktop/src-tauri/src/spell_catalog.rs` serves
//!    the Spell Catalog screen (and the Character Sheet's Add Spell picker)
//!    straight out of `rules_tables::{crb,apg,acg,advanced_race_guide}::
//!    spell_list`. Fixing only the ingest binary that writes
//!    `data/corpus/<book>/spell/*.json` would leave every leaking string on
//!    screen exactly as before.
//! 2. **A spell row defines no variables.** A spell's `DESC:` arguments are
//!    caster-level expressions (`CASTERLEVEL`, `min(3,1+(CASTERLEVEL-3)/4)`),
//!    never same-row literals, so there is no per-row variable table to pass
//!    in and the row-shaped signature the binaries use buys nothing here.
//!
//! ## What the render does
//!
//! PCGen's description syntax is not prose. A `DESC:` token is
//! `<prose>|<arg1>|<arg2>...`, `%N` references argument N, and `%%` is a
//! literal-percent escape. This module:
//!
//! * collapses `%%` to one `%` (lossless — nothing is looked up, nothing can
//!   be lost);
//! * substitutes `%N` when argument N is a plain integer literal;
//! * **drops** `%N` when argument N is anything this module's own narrow
//!   `<Name><+|-><integer>` offset shape (see [`resolve_desc_argument`]) does
//!   not cover, taking the `+`/`-` sign that introduced it with it, closing
//!   the whitespace, and **reporting** the dropped argument rather than
//!   guessing a value. (`SD-27 decisions.md §24.1`'s formula-interpreter ban
//!   this doc used to cite here was overturned by `SD-31 decisions.md`
//!   Decision 20 on 2026-08-21, before this module's own `resolve_desc_
//!   argument` was last touched — `Halfling ~ Adaptable Luck`, this doc's own
//!   worked example, now resolves at ingest time too, via `race_trait_
//!   formula_binding::resolve_same_row_formula`; this module's narrower
//!   offset-only shape is unaffected and still correct for the population it
//!   serves, but is no longer the *only* option the way this comment implied);
//! * removes the `|`-delimited argument tail in all cases;
//! * removes a trailing `|`-delimited **PCGen qualifier** (`PREABILITY:…`,
//!   `!PRERULE:1,DisplayFullSpell`, …). A `DESC:` token may end in one or
//!   more `PRE`-family clauses that gate *whether the description applies*,
//!   and they are not arguments — the prose references no `%N` for them, so
//!   the argument-tail rule above never reached them. ACG's `Twinned Feint`
//!   shipped to the Add Feat picker reading *"…as a move action
//!   instead.|!PREABILITY:1,CATEGORY=FEAT,Improved Feint"* for exactly this
//!   reason. Stripping them is the same treatment `acg::spell_list`'s own
//!   ingest already applied to `|(!)PRERULE:1,DisplayFullSpell` ("a
//!   display-rule directive, not spell text"), applied here so it holds for
//!   every content kind rather than one book's spells;
//! * decodes the PCGen entity escape `&nl;` to a real newline. The other
//!   four entities PCGen writes (`&lbracket;`, `&rbracket;`, `&pipe;`,
//!   `&comma;`) are decoded at ingest and appear in **zero** served table
//!   rows today (`grep -rhoE '&[a-zA-Z]+;' src/rules_core/rules_tables/`
//!   returns them only from two doc comments); `&nl;` is the one that
//!   survived, in 3 ACG feat descriptions. [`leaked_pcgen_syntax`] flags any
//!   of the five, so an undecoded entity fails loudly instead of reaching a
//!   player as a literal `&nl;`.
//!
//! ## The `|` rule differs from the race-trait binary's, deliberately
//!
//! `ingest_race_traits::leaked_pcgen_syntax` treats *any* `|` as a raw
//! argument tail. That is safe for racial-trait prose and **wrong** for spell
//! prose: CRB and APG spell text renders rulebook tables inline with ` | `
//! column separators. Derived over all four spell tables rather than assumed:
//! 151 `|` characters have whitespace on at least one side (every one of them
//! a prose table separator — "Hardness and Rarity | Examples", "HD | Strength
//! | Lingering Aura Duration"), and 13 have whitespace on neither side (every
//! one of them a real PCGen argument tail — "…the cloud's effects|CASTERLEVEL").
//! So the guard here flags a `|` only when it is *tight* — no whitespace on
//! either side — which is exactly the shape PCGen writes and exactly the shape
//! rulebook prose does not.

/// The named PCGen variables this engine has already resolved for **one
/// character**, keyed by the variable name a `DESC:` argument names.
///
/// # Why this is not the formula interpreter `decisions.md §24` forbids
///
/// §24 bans evaluating PCGen's formula language. Nothing here evaluates one.
/// The caller computes each value with a hand-modelled, corpus-verified pure
/// function it already owns — `monk_features::ki_points`,
/// `barbarian_features::rage_rounds_per_day`, `rogue_features::master_strike_dc`
/// and their siblings — and *states* the result under the name PCGen uses for
/// it. This type is a lookup table; the arithmetic happened upstream, in the
/// same hand-modelled functions §24 prescribes.
///
/// The operator's ruling that motivates it (2026-08-01):
///
/// > You do not need a full blown engine for things like uses per day. You just
/// > need the ability to calculate the value that is displayed in the
/// > description or elsewhere in the UI. […] These are all just display values.
///
/// A description resolved through this table therefore renders a *number*, and
/// re-renders a different number when the character changes, without any
/// uses-tracking, resource-pool or expenditure state existing anywhere.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PcgenDisplayValues {
    values: std::collections::BTreeMap<String, i64>,
}

impl PcgenDisplayValues {
    /// An empty table. Rendering against it is byte-identical to the
    /// value-free [`render_pcgen_desc`], which is what every caller with no
    /// character in hand (the spell catalog) keeps getting.
    pub fn new() -> Self {
        Self::default()
    }

    /// States one resolved value under the PCGen variable name that stands for
    /// it. Overwrites rather than accumulates: the caller's hand-modelled
    /// function already produced the *total*, and adding to it here would
    /// double-count.
    pub fn set(&mut self, name: &str, value: i64) {
        self.values.insert(name.to_string(), value);
    }

    /// The resolved value for one PCGen variable name, or `None` when this
    /// engine has not computed it. `None` is what keeps a placeholder dropped
    /// rather than guessed.
    pub fn get(&self, name: &str) -> Option<i64> {
        self.values.get(name).copied()
    }

    /// True when nothing has been resolved.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Every variable name this table carries, sorted. Exists so a test can
    /// assert *which* values a character resolved rather than only that the
    /// text came out right.
    pub fn names(&self) -> Vec<&str> {
        self.values.keys().map(String::as_str).collect()
    }
}

/// One rendered `DESC:` token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedPcgenDesc {
    /// The prose a player may be shown. Byte-identical to the input when the
    /// input carried no PCGen syntax at all.
    pub text: String,
    /// The `DESC:` arguments that could not be resolved to a literal and were
    /// therefore dropped from `text`. Reported, never guessed; a caller that
    /// wants to know what a description is missing reads this.
    pub dropped_args: Vec<String>,
}

/// The highest `%N` argument index the prose references, or 0 when it
/// references none. `%%` is an escape and is skipped, so `%%1` never counts
/// as `%1`.
fn max_arg_reference(raw: &str) -> usize {
    let chars: Vec<char> = raw.chars().collect();
    let mut max = 0usize;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            // `%%<digit>` is counted as a CANDIDATE reference, not as an
            // escape, and this is the half of the `%%N` reading that decides
            // the other half. `render_pcgen_desc_with_values` treats `%%N` as
            // an argument only when argument N exists — but "exists" is decided
            // by this function, so skipping the digit here would make that
            // branch permanently unreachable and leave the four corpus rows
            // that write `DC %%1 … |<DC var>` rendering `DC %1` on a player's
            // screen (`decisions.md §61.3`).
            //
            // Counting it is safe in the other direction: a token with no
            // `|` tail yields zero arguments no matter what this returns, so
            // `20%%1 chance` still renders its literal per cent sign.
            if let Some(digit) = chars.get(i + 2).and_then(|c| c.to_digit(10))
                && digit >= 1
            {
                max = max.max(digit as usize);
                i += 3;
                continue;
            }
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            max = max.max(digit as usize);
            i += 2;
            continue;
        }
        i += 1;
    }
    max
}

/// Collapses every whitespace run to a single space and trims the ends.
/// Applied only when a placeholder was dropped, so prose that needed no edit
/// stays byte-identical to the source.
fn collapse_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

/// The PCGen entity escapes that can appear inside a `DESC:` token, paired
/// with the character each stands for.
///
/// Only `&nl;` survives ingest into a served table today (3 ACG feat rows);
/// the other four are decoded upstream. All five are listed so
/// [`leaked_pcgen_syntax`] can name any that turns up, and so a future book
/// whose ingest forgets one is caught here rather than on a screen.
const PCGEN_ENTITIES: [(&str, &str); 5] = [
    ("&nl;", "\n"),
    ("&lbracket;", "["),
    ("&rbracket;", "]"),
    ("&pipe;", "|"),
    ("&comma;", ","),
];

/// Whether one `|`-delimited segment is a PCGen `PRE`-family qualifier
/// (`PREABILITY:1,CATEGORY=FEAT,Improved Feint`,
/// `!PRERULE:1,DisplayFullSpell`) rather than prose or a `%N` argument.
///
/// Deliberately narrow: an optional leading `!`, then `PRE`, then at least
/// one more uppercase letter, then a `:`. Rulebook prose does not produce
/// that shape, so this cannot eat real text.
fn is_pcgen_qualifier(segment: &str) -> bool {
    let body = segment.strip_prefix('!').unwrap_or(segment);
    let Some(rest) = body.strip_prefix("PRE") else {
        return false;
    };
    let Some(token) = rest.split(':').next() else {
        return false;
    };
    !token.is_empty()
        && token.len() < rest.len()
        && token.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

/// Drops any run of trailing PCGen `PRE`-family qualifier segments, returning
/// what remains of the token **and the qualifiers that were removed**, in
/// source order.
///
/// Applied before the argument split so a qualifier can never be mistaken for
/// argument N, and unconditionally so it is removed even from a token whose
/// prose references no `%N` at all — the case that put
/// `|!PREABILITY:1,CATEGORY=FEAT,Improved Feint` in front of a player.
///
/// The qualifiers were previously discarded. They are returned now because a
/// `DESC:` gate is not decoration: `Halfling ~ Adaptable Luck` writes its
/// uses-per-day as two mutually exclusive segments,
/// `Three|PREVARLTEQ:…,3` and `%1|…|PREVARGTEQ:…,4`, and a renderer that keeps
/// both emits *"Three 5 times per day"*. See [`render_pcgen_desc_tokens`].
fn strip_trailing_qualifiers(raw: &str) -> (String, Vec<String>) {
    let mut segments: Vec<&str> = raw.split('|').collect();
    let mut qualifiers: Vec<String> = Vec::new();
    while segments.len() > 1 && is_pcgen_qualifier(segments[segments.len() - 1]) {
        qualifiers.push(segments.pop().expect("checked non-empty").to_string());
    }
    qualifiers.reverse();
    (segments.join("|"), qualifiers)
}

/// Resolves one `DESC:` argument to an integer, or `None` when this engine has
/// not computed it.
///
/// Three shapes, and deliberately no more:
///
/// 1. **An integer literal** (`2`) — what [`render_pcgen_desc`] already read.
/// 2. **A bare variable name** (`KiPoints`, `RageDuration`) — looked up in
///    `values`, which the caller filled from its own hand-modelled functions.
/// 3. **A variable with an integer offset** (`Halfling_AdaptableLuck_Bonus-1`).
///
/// Shape 3 is not the thin end of an interpreter — it is the *entire*
/// non-bare argument population of the shipped corpus, derived by command
/// rather than assumed:
///
/// ```text
/// # over every data/corpus/**/*.json DESC token's argument tail,
/// # excluding PRE-family gates and bare integers:
/// non-bare, non-literal DESC args across the whole corpus:
///     {'Halfling_AdaptableLuck_Bonus-1': 1}
/// ```
///
/// One argument, in one record. Supporting it is transcription of a single
/// subtraction whose left operand this engine already holds; supporting
/// anything beyond what the corpus contains would be the speculative
/// evaluator `decisions.md §24` bans. A shape not on this list resolves to
/// `None` and its placeholder is dropped and reported, exactly as before.
fn resolve_desc_argument(arg: &str, values: &PcgenDisplayValues) -> Option<i64> {
    let arg = arg.trim();
    if let Ok(literal) = arg.parse::<i64>() {
        return Some(literal);
    }
    if let Some(value) = values.get(arg) {
        return Some(value);
    }
    // Shape 3: `<Name><+|-><integer>`, split at the last sign so a name
    // containing an underscore or digits is not torn apart.
    let split_at = arg.rfind(['+', '-']).filter(|index| *index > 0)?;
    let (name, offset) = arg.split_at(split_at);
    let base = values.get(name.trim())?;
    let offset = offset.parse::<i64>().ok()?;
    Some(base + offset)
}

/// Whether one `DESC:` gate holds for this character.
///
/// `Undecided` is a first-class outcome, not a failure: a gate naming a
/// variable this engine has not resolved must leave its prose alone. Deleting
/// real rulebook text on the strength of an unknown is a worse error than
/// showing an extra clause, and it is the error a two-valued result would
/// force.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateOutcome {
    Applies,
    DoesNotApply,
    Undecided,
}

/// Evaluates one `DESC:` gate segment against the resolved display values.
///
/// Only PCGen's `PREVAR<CMP>:<lhs>,<rhs>[,<lhs>,<rhs>…]` family is decided,
/// with an optional leading `!` negating the result and every pair required to
/// hold. The comparator set is closed and derived by command over the shipped
/// corpus, not guessed:
///
/// ```text
/// DESC gate token kinds:
///   {'PREVARLTEQ': 2, 'PREVAREQ': 2, 'PREVARGTEQ': 2, 'PREVARGT': 1,
///    '!PREABILITY': 144, 'PREABILITY': 7}
/// ```
///
/// Seven `PREVAR*` gates across four comparators — `LT` and `NEQ` are
/// implemented alongside them because they are the same one-line comparison
/// and their absence would be an arbitrary hole, but nothing outside the
/// `PREVAR` family is decided here. The 151 `PREABILITY` gates ask which
/// abilities the character holds, which is not a fact this table carries, so
/// they return `Undecided` and their prose survives — the behaviour every
/// caller has today.
///
/// Mirrors `ingest_races::eval_prevar_gate`, which does the same reading over
/// a row's own same-row variables, and differs from it in exactly one way: an
/// undecidable gate is reported rather than being an error, because this
/// renderer runs against a live character where a missing value is ordinary.
fn eval_desc_gate(gate: &str, values: &PcgenDisplayValues) -> GateOutcome {
    let (negated, body) = match gate.strip_prefix('!') {
        Some(rest) => (true, rest),
        None => (false, gate),
    };
    let Some((token, operands)) = body.split_once(':') else {
        return GateOutcome::Undecided;
    };
    let Some(comparator) = token.strip_prefix("PREVAR") else {
        return GateOutcome::Undecided;
    };

    let parts: Vec<&str> = operands.split(',').collect();
    if parts.is_empty() || !parts.len().is_multiple_of(2) {
        return GateOutcome::Undecided;
    }

    let mut all_hold = true;
    for pair in parts.chunks(2) {
        let (Some(left), Some(right)) =
            (resolve_desc_argument(pair[0], values), resolve_desc_argument(pair[1], values))
        else {
            return GateOutcome::Undecided;
        };
        let holds = match comparator {
            "EQ" => left == right,
            "NEQ" => left != right,
            "GT" => left > right,
            "GTEQ" => left >= right,
            "LT" => left < right,
            "LTEQ" => left <= right,
            _ => return GateOutcome::Undecided,
        };
        all_hold &= holds;
    }

    match all_hold ^ negated {
        true => GateOutcome::Applies,
        false => GateOutcome::DoesNotApply,
    }
}

/// Replaces every PCGen entity escape with the character it stands for.
///
/// Runs last, after whitespace collapsing, so a decoded `&nl;` newline is not
/// immediately squashed back into a space.
fn decode_pcgen_entities(text: &str) -> String {
    let mut out = text.to_string();
    for (entity, replacement) in PCGEN_ENTITIES {
        if out.contains(entity) {
            out = out.replace(entity, replacement);
        }
    }
    out
}

/// Splits a raw `DESC:` token into its prose and its argument tail.
///
/// The tail is taken from the **right**, exactly as many segments as the prose
/// actually references, so a `|` inside the prose itself (a rulebook table
/// separator) is rejoined rather than mistaken for an argument boundary. A
/// token whose prose references no argument is returned whole and untouched.
/// A trailing PCGen bonus/variable tag with no `%N` argument reference
/// anywhere in the prose to justify the split (e.g. UPsi's
/// `up_equipment.lst` `"...up to +5|DisruptorShieldBonus"` -- a real,
/// corpus-genuine tag, not a fabricated case). `leaked_pcgen_syntax` already
/// distinguishes a *tight* `|` (no whitespace on either side, real PCGen
/// separator) from a loose one that could appear in ordinary prose; this
/// reuses that same boundary on the LAST `|` so a tag like this is still
/// stripped even when there is no `%N` reference consuming it.
fn strip_trailing_tight_pipe_tag(raw: &str) -> Option<String> {
    let chars: Vec<char> = raw.chars().collect();
    let idx = chars.iter().rposition(|&c| c == '|')?;
    let left_tight = idx > 0 && !chars[idx - 1].is_whitespace();
    let right_tight = idx + 1 < chars.len() && !chars[idx + 1].is_whitespace();
    if left_tight && right_tight {
        Some(chars[..idx].iter().collect())
    } else {
        None
    }
}

fn split_prose_and_args(raw: &str) -> (String, Vec<String>) {
    let (raw, _gates) = strip_trailing_qualifiers(raw);
    let raw = raw.as_str();
    let max = max_arg_reference(raw);
    if max == 0 {
        return match strip_trailing_tight_pipe_tag(raw) {
            Some(prose) => (prose, Vec::new()),
            None => (raw.to_string(), Vec::new()),
        };
    }
    let segments: Vec<&str> = raw.split('|').collect();
    if segments.len() <= max {
        // Fewer tail segments than the prose references: keep segment 0 as
        // prose and treat the rest as arguments. The unmatched `%N` then has
        // no argument at all and is dropped by `render_pcgen_desc`, which is
        // the honest outcome — the alternative is inventing a value.
        let prose = segments.first().copied().unwrap_or_default().to_string();
        let args = segments[1..].iter().map(|s| (*s).to_string()).collect();
        return (prose, args);
    }
    let split_at = segments.len() - max;
    let prose = segments[..split_at].join("|");
    let args = segments[split_at..].iter().map(|s| (*s).to_string()).collect();
    (prose, args)
}

/// The `|`-delimited argument tail of a raw `DESC:` token, exactly as
/// [`render_pcgen_desc_with_values`] itself reads it -- exposed read-only so a
/// caller that wants to resolve an argument through a mechanism OTHER than
/// [`PcgenDisplayValues`]'s named-lookup (e.g. evaluating the argument text
/// directly as a PCGen formula, when it is itself a raw expression like
/// `"max(1,WarpriestLVL/2)"` rather than a bare variable name) can see the
/// exact argument strings this module's own renderer will later try to
/// resolve, without duplicating `split_prose_and_args`'s own tail-taken-
/// from-the-right parsing (SD-32 T12 Epic 8 row 18 cycle 15,
/// `class_feature_grant_consumer::resolved_description_for_formula_only_desc_argument`).
pub(crate) fn desc_token_arguments(raw: &str) -> Vec<String> {
    split_prose_and_args(raw).1
}

/// Whether `chars[i]` (a `'%'`) is part of standard "d%"/"D%" percentile-
/// dice notation (= d100), a real, resolved shape that must be preserved
/// literally, never treated as a leak or dropped. `chars` is the FULL text
/// under scan (or the prose being rendered) — this is a lookbehind only,
/// never a lookahead, so it composes safely with whatever comes after `%`.
///
/// Recognized as: the character immediately before `%` is `d`/`D`, AND the
/// character before THAT is either a word boundary (start of string or
/// non-alphabetic) or a DIGIT — so both `"roll d% for..."` and the real
/// Teleport/Plane Shift/Planar Wanderer corpus shape `"5d% miles"` (a dice
/// COUNT immediately before the die) match, while a word merely ending in
/// `d` (`"gold%"`) does not. A digit is deliberately excluded from failing
/// the word-boundary test — `is_alphabetic`, not `is_alphanumeric` — the
/// one correction wave-8 integration made to this rule after the first
/// version flagged `"5d%"` as a leak (`equipment_catalog`'s own
/// `no_catalog_serves_a_description_carrying_raw_pcgen_syntax` test caught
/// it against real `spell:Teleport`/`spell:Plane Shift`/`feat:Planar
/// Wanderer` corpus text).
fn is_percentile_dice_notation(chars: &[char], i: usize) -> bool {
    i >= 1
        && matches!(chars[i - 1], 'd' | 'D')
        && (i < 2 || !chars[i - 2].is_alphabetic())
}

/// Renders one raw PCGen `DESC:` token into player-facing prose.
///
/// See the module doc for the full contract. The short version: `%%` becomes
/// `%`, a `%N` backed by an integer literal becomes that integer, every other
/// `%N` is dropped and reported, and the argument tail never survives.
pub fn render_pcgen_desc(raw: &str) -> RenderedPcgenDesc {
    render_pcgen_desc_with_values(raw, &PcgenDisplayValues::new())
}

/// Renders one raw PCGen `DESC:` token against the values this engine has
/// already computed for a specific character.
///
/// Identical to [`render_pcgen_desc`] except that a `%N` whose argument names a
/// variable present in `values` renders **that character's number** instead of
/// being dropped. Passing an empty table is byte-identical to
/// [`render_pcgen_desc`], which is exactly how that function is implemented.
///
/// The no-fabrication rule is unchanged and load-bearing: an argument this
/// engine has not computed is still dropped, still takes its introducing sign
/// with it, and is still reported in
/// [`dropped_args`](RenderedPcgenDesc::dropped_args). Resolution widened; the
/// honesty bar did not move.
pub fn render_pcgen_desc_with_values(raw: &str, values: &PcgenDisplayValues) -> RenderedPcgenDesc {
    let (prose, args) = split_prose_and_args(raw);
    let chars: Vec<char> = prose.chars().collect();
    let mut out = String::new();
    let mut dropped_args: Vec<String> = Vec::new();
    let mut dropped_any = false;
    let mut i = 0;

    while i < chars.len() {
        // The escape is checked first: `%%` is a literal per cent sign, so
        // `%% spell failure chance` must not be read as an argument.
        //
        // With ONE exception, and it is an upstream escaping defect rather than
        // a syntax this renderer chose to support. Four `DESC:` tokens in the
        // whole Paizo tree write `%%<digit>` where the row's own argument list
        // supplies that argument and nothing else could consume it:
        //
        // ```text
        // grep -rl '%%[0-9]' --include='*.lst' ~/workspace/repos/pcgen/data/pathfinder/paizo/
        //   bestiary_3/b3_abilities_race.lst
        //   ultimate_wilderness/uw_abilities_companion.lst   (2 rows)
        //   player_companion/familiar_folio/ff_abilities_race.lst
        // ```
        //
        // all of the shape `... must make a DC %%1 Fortitude save ...|<DC var>`.
        // Read as an escape the row renders "DC %1", which is PCGen syntax on a
        // player's screen -- `leaked_pcgen_syntax` rejects it, and until Ultimate
        // Wilderness's companions landed no ingested book carried one, so the
        // renderer and the guard had been in silent contradiction for the whole
        // program (`decisions.md §61.3`).
        //
        // The narrow reading is what ships: `%%N` is an argument reference ONLY
        // when argument N exists. Everything else keeps the literal per cent,
        // so no text that renders correctly today can change.
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            let escaped_arg = chars
                .get(i + 2)
                .and_then(|c| c.to_digit(10))
                .filter(|digit| *digit >= 1)
                .and_then(|digit| args.get(digit as usize - 1).map(|arg| (digit, arg)));
            if let Some((_, arg)) = escaped_arg {
                match resolve_desc_argument(arg, values) {
                    Some(value) => out.push_str(&value.to_string()),
                    None => {
                        dropped_args.push(arg.trim().to_string());
                        while out.ends_with('+') || out.ends_with('-') {
                            out.pop();
                        }
                        dropped_any = true;
                    }
                }
                i += 3;
                continue;
            }
            // SD31-W8-INTEGRATE-001: a `%%` escape ordinarily collapses to
            // a literal `%` sign UNCONDITIONALLY here -- correct for the
            // common case ("20%%." -> "20%.", digit already in `out`) and
            // for percentile-dice notation ("d%%" -> "d%"). But when the
            // NUMBER this percent sign belongs to was itself a `%N`
            // argument reference that had to be dropped moments earlier
            // (real corpus shape: `ultimate_intrigue:spell:absolution`'s
            // `"has a %1%% chance"`, where `%1` names a formula this
            // engine cannot resolve), `out` ends with neither a digit nor
            // a dice-notation `d`/`D` -- pushing the bare `%` here would
            // leave exactly the orphaned "a % chance" hole this cycle's
            // `leaked_pcgen_syntax` widening now (correctly) refuses to
            // serve. Drop it the same no-fabrication way instead of
            // rendering a percent sign with no number in front of it.
            let out_chars: Vec<char> = out.chars().collect();
            let n = out_chars.len();
            let preceded_by_digit = n > 0 && out_chars[n - 1].is_ascii_digit();
            let preceded_by_dice_notation = n > 0
                && matches!(out_chars[n - 1], 'd' | 'D')
                && (n < 2 || !out_chars[n - 2].is_alphabetic());
            if preceded_by_digit || preceded_by_dice_notation {
                out.push('%');
            } else {
                dropped_args.push("%%".to_string());
                while out.ends_with('+') || out.ends_with('-') {
                    out.pop();
                }
                dropped_any = true;
            }
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            let arg = args.get(digit as usize - 1);
            match arg.and_then(|a| resolve_desc_argument(a, values)) {
                Some(value) => out.push_str(&value.to_string()),
                None => {
                    if let Some(name) = arg {
                        dropped_args.push(name.trim().to_string());
                    }
                    while out.ends_with('+') || out.ends_with('-') {
                        out.pop();
                    }
                    dropped_any = true;
                }
            }
            i += 2;
            continue;
        }
        // CONFIRMED finding (`SD31-W6-INTEGRATE-001`): a `%<KEYWORD>`
        // substitution (`%CHOICE`, the only shape any shipped `description:`
        // text carries -- `%LIST` appears only in `qualifiers` fields today,
        // covered here on the same general mechanism rather than a
        // `%CHOICE`-specific special case). PCGen keyword substitutions name
        // a chargen-time PLAYER SELECTION (e.g. a bloodline/mystery choice)
        // this engine has no `PcgenDisplayValues` slot for at all -- there is
        // no resolution path to attempt, unlike `%N`, so this is
        // unconditionally dropped, never guessed at. Same no-fabrication
        // treatment as an unresolved `%N`: takes its introducing `+`/`-` with
        // it, collapses the surrounding whitespace.
        if chars[i] == '%' && chars.get(i + 1).is_some_and(char::is_ascii_uppercase) {
            let start = i + 1;
            let mut end = start;
            while chars.get(end).is_some_and(char::is_ascii_uppercase) {
                end += 1;
            }
            dropped_args.push(chars[start..end].iter().collect());
            while out.ends_with('+') || out.ends_with('-') {
                out.pop();
            }
            dropped_any = true;
            i = end;
            continue;
        }
        // CONFIRMED finding (`SD31-W8-INTEGRATE-001`): a bare '%' --
        // neither a digit argument reference nor an uppercase keyword
        // substitution -- previously fell through to the plain
        // `out.push` below and reached a player's screen unchanged
        // (`leaked_pcgen_syntax`'s widened check caught this in
        // DIAGNOSIS; `equipment_catalog`'s own `the_raw_percent_escape_
        // stops_at_the_catalog_boundary` test went red against 42
        // served descriptions still carrying it). Same no-fabrication
        // drop as `%<KEYWORD>` above, with the ONE named exception that
        // check also carves out: "d%"/"D%" percentile-dice notation
        // (= d100) at a word boundary is a real, resolved shape and must
        // be preserved literally.
        if chars[i] == '%' {
            if is_percentile_dice_notation(&chars, i) {
                out.push('%');
                i += 1;
                continue;
            }
            dropped_args.push("%".to_string());
            while out.ends_with('+') || out.ends_with('-') {
                out.pop();
            }
            dropped_any = true;
            i += 1;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }

    let text = if dropped_any { collapse_whitespace(&out) } else { out };
    RenderedPcgenDesc { text: decode_pcgen_entities(&text), dropped_args }
}

/// Renders a record's **whole** `DESC:` token list into one description,
/// honouring the `PREVAR*` gates that decide which segments apply.
///
/// A PCGen record may carry several `DESC:` tokens whose surviving segments
/// concatenate in source order — the shape `ingest_races::render_description`
/// already reads at ingest time. This is that reading, moved to render time so
/// the gates can be decided against a **live character** instead of against one
/// row's own constants.
///
/// Why it must exist separately from [`render_pcgen_desc_with_values`]: the
/// gate and the value are the same fact seen twice. `Halfling ~ Adaptable Luck`
/// writes
///
/// ```text
/// DESC:Three|PREVARLTEQ:Halfling_AdaptableLuck_Times,3
/// DESC:%1|Halfling_AdaptableLuck_Times|PREVARGTEQ:Halfling_AdaptableLuck_Times,4
/// DESC:times per day, a halfling can gain a +%1 luck bonus …
/// ```
///
/// so a halfling with the Fortunate One feat must render *"4 times per day"* —
/// which requires both substituting `%1` **and** suppressing the word "Three".
/// Rendering the tokens one at a time cannot do the second.
///
/// Segments are dropped only on a **decided** false gate.
/// [`GateOutcome::Undecided`] keeps its prose, so a caller with no values
/// resolved gets the same text it does today.
pub fn render_pcgen_desc_tokens(tokens: &[&str], values: &PcgenDisplayValues) -> RenderedPcgenDesc {
    let mut segments: Vec<String> = Vec::new();
    let mut dropped_args: Vec<String> = Vec::new();

    for token in tokens {
        let (_, gates) = strip_trailing_qualifiers(token);
        if gates.iter().any(|gate| eval_desc_gate(gate, values) == GateOutcome::DoesNotApply) {
            continue;
        }
        let rendered = render_pcgen_desc_with_values(token, values);
        dropped_args.extend(rendered.dropped_args);
        if !rendered.text.is_empty() {
            segments.push(rendered.text);
        }
    }

    RenderedPcgenDesc { text: segments.join(" "), dropped_args }
}

/// The PCGen syntax that must never reach a player: an unsubstituted `%N`
/// argument reference, an unresolved `%%` literal-percent escape, an undecoded
/// entity escape such as `&nl;`, or a *tight* `|` argument tail (see the module
/// doc for why tightness, not the bare character, is the test).
///
/// Returns the name of the leak so a caller can fail loudly with a reason
/// rather than a bare boolean.
pub fn leaked_pcgen_syntax(text: &str) -> Option<&'static str> {
    if text.contains("%%") {
        return Some("unescaped '%%' literal-percent escape");
    }
    for (entity, _) in PCGEN_ENTITIES {
        if text.contains(entity) {
            return Some("undecoded PCGen entity escape");
        }
    }
    let chars: Vec<char> = text.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c == '%' && chars.get(i + 1).is_some_and(char::is_ascii_digit) {
            return Some("unsubstituted '%N' argument reference");
        }
        // CONFIRMED finding (`SD31-W6-INTEGRATE-001`): PCGen also uses
        // uppercase KEYWORD substitutions (`%CHOICE`, the only one this
        // corpus's shipped `description:` text carries, re-derived
        // corpus-wide) that carry no digit at all -- the digit-only check
        // above never caught them, so `%CHOICE` shipped to the player
        // verbatim on the equipment render path.
        if *c == '%' && chars.get(i + 1).is_some_and(char::is_ascii_uppercase) {
            return Some("unsubstituted '%<KEYWORD>' argument reference");
        }
        // CONFIRMED finding (`SD31-W8-INTEGRATE-001`, wave-8 adversarial
        // review): the two checks above only catch a '%' immediately
        // followed by a digit or an uppercase letter. 31 real corpus
        // records ship a hole neither shape catches -- a `%` followed by
        // a space, punctuation, or a lowercase letter ("Cast % 1/day",
        // "Darkvision % ft.", "+%d6 additional ectoplasmic damage") --
        // and read `text-complete`/`done` with the placeholder still
        // visible. A literal percent SIGN is always immediately preceded
        // by a digit ("50% chance", "20% of something"); a `%` that is
        // NOT preceded by a digit has no other legitimate PCGen meaning
        // in a `DESC:`/`SPROP:` field, so it is always a leak here --
        // EXCEPT the one named real shape this repo already renders on
        // purpose: "d%"/"D%" percentile-dice notation (= d100), which a
        // `%%` escape collapse can legitimately leave behind (see
        // `a_prose_table_beside_a_real_argument_keeps_the_table_and_loses_
        // the_tail`'s "roll d% for..." fixture) -- recognized narrowly as
        // the single letter d/D at a word boundary immediately before
        // '%', not any word merely ending in 'd'.
        if *c == '%'
            && !(i > 0 && chars[i - 1].is_ascii_digit())
            && !is_percentile_dice_notation(&chars, i)
        {
            return Some("unsubstituted bare '%' gap");
        }
        if *c == '|' {
            let left_open = i == 0 || chars[i - 1].is_whitespace();
            let right_open = chars.get(i + 1).is_none_or(|next| next.is_whitespace());
            if !left_open && !right_open {
                return Some("raw '|' argument tail");
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prose_with_no_pcgen_syntax_is_returned_byte_identical() {
        let raw = "You transform one alchemical item or firearm into another.";
        let rendered = render_pcgen_desc(raw);
        assert_eq!(rendered.text, raw);
        assert!(rendered.dropped_args.is_empty());
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn the_double_percent_escape_collapses_to_one_sign() {
        let rendered = render_pcgen_desc("any spellcasting with a verbal component has a 20%% spell failure chance.");
        assert_eq!(rendered.text, "any spellcasting with a verbal component has a 20% spell failure chance.");
        assert!(rendered.dropped_args.is_empty());
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// The upstream `%%N` escaping defect, and both halves of the narrow
    /// reading that resolves it (`decisions.md §61.3`).
    ///
    /// Ultimate Wilderness's `Seaweed Leshy ~ Water Jet` is the row that made
    /// this reachable: read as an escape it renders `DC %1 Fortitude save`,
    /// which `leaked_pcgen_syntax` rejects and `companion_catalog` panics on.
    #[test]
    fn a_double_percent_before_a_digit_is_an_argument_when_the_row_supplies_one() {
        let rendered = render_pcgen_desc("must make a DC %%1 Fortitude save|WaterJetDC");
        assert_eq!(
            rendered.text, "must make a DC Fortitude save",
            "an unresolvable formula argument is dropped, exactly as a plain %N is"
        );
        assert_eq!(rendered.dropped_args, vec!["WaterJetDC".to_string()]);
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);

        let substituted = render_pcgen_desc("must make a DC %%1 Fortitude save|17");
        assert_eq!(substituted.text, "must make a DC 17 Fortitude save");
        assert!(substituted.dropped_args.is_empty());
    }

    /// The other half: with NO argument to consume it, `%%` stays a literal per
    /// cent sign even before a digit. Nothing that renders correctly today
    /// changes.
    #[test]
    fn a_double_percent_before_a_digit_with_no_such_argument_stays_a_literal_sign() {
        let rendered = render_pcgen_desc("a 20%%1 chance");
        assert_eq!(rendered.text, "a 20%1 chance");
        assert!(rendered.dropped_args.is_empty());
    }

    #[test]
    fn a_percent_n_backed_by_an_integer_literal_is_substituted() {
        let rendered = render_pcgen_desc("a +%1 luck bonus|2");
        assert_eq!(rendered.text, "a +2 luck bonus");
        assert!(rendered.dropped_args.is_empty());
    }

    #[test]
    fn a_percent_n_backed_by_a_formula_is_dropped_and_reported_never_guessed() {
        let rendered = render_pcgen_desc("contained within you for up to %1 rounds.|CASTERLEVEL");
        assert_eq!(rendered.text, "contained within you for up to rounds.");
        assert_eq!(rendered.dropped_args, vec!["CASTERLEVEL".to_string()]);
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn a_dropped_argument_takes_its_introducing_sign_with_it() {
        let rendered = render_pcgen_desc("The target gains a +%1 insight bonus.|min(6,2+CASTERLEVEL/4)");
        assert_eq!(rendered.text, "The target gains a insight bonus.");
        assert_eq!(rendered.dropped_args, vec!["min(6,2+CASTERLEVEL/4)".to_string()]);
    }

    #[test]
    fn two_arguments_are_taken_from_the_right_in_order() {
        let rendered = render_pcgen_desc("an AC equal to %1 and %2 hit points.|CASTERLEVEL/2+10|CASTERLEVEL");
        assert_eq!(rendered.text, "an AC equal to and hit points.");
        assert_eq!(
            rendered.dropped_args,
            vec!["CASTERLEVEL/2+10".to_string(), "CASTERLEVEL".to_string()]
        );
    }

    /// The reason this module exists rather than a fourth copy of the
    /// race-trait binary's helper: rulebook prose renders tables with ` | `
    /// column separators, and the race-trait binary's "any `|` is a tail" rule
    /// would shred them.
    #[test]
    fn a_prose_table_separator_is_not_an_argument_tail() {
        let raw = "as indicated on the following table. \nHardness and Rarity | Examples Duration \nVegetable matter | 2 hr./level";
        let rendered = render_pcgen_desc(raw);
        assert_eq!(rendered.text, raw, "no `%N` reference means no tail to strip");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn a_prose_table_beside_a_real_argument_keeps_the_table_and_loses_the_tail() {
        let rendered = render_pcgen_desc("roll d%% for %1 rounds. \nRoll | Effect \n01-25 | Act normally|CASTERLEVEL");
        // Whitespace is collapsed only because an argument was dropped —
        // that is the one branch that rewrites spacing, and it is why the
        // newlines in the table are gone here but survive in
        // `a_prose_table_separator_is_not_an_argument_tail` above.
        assert_eq!(rendered.text, "roll d% for rounds. Roll | Effect 01-25 | Act normally");
        assert_eq!(rendered.dropped_args, vec!["CASTERLEVEL".to_string()]);
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    #[test]
    fn leak_guard_names_every_shape_that_must_never_reach_a_player() {
        assert_eq!(leaked_pcgen_syntax("Clean prose with 50% of something."), None);
        assert_eq!(leaked_pcgen_syntax("A +%1 bonus."), Some("unsubstituted '%N' argument reference"));
        assert_eq!(leaked_pcgen_syntax("reduced by 20%%."), Some("unescaped '%%' literal-percent escape"));
        assert_eq!(leaked_pcgen_syntax("the cloud's effects|CASTERLEVEL"), Some("raw '|' argument tail"));
        assert_eq!(leaked_pcgen_syntax("Hardness and Rarity | Examples"), None);
        assert_eq!(leaked_pcgen_syntax("trailing pipe |"), None);
    }

    /// SD31-W8-INTEGRATE-001: wave-8 adversarial review CONFIRMED that a
    /// `%` hole neither followed by a digit nor an uppercase letter slips
    /// past both existing checks -- 31 real corpus records ship one of
    /// exactly these shapes (`core_rulebook:equipmods:itempower_castone`
    /// and siblings, `ultimate_psionics:equipmods:plusn_svs` and
    /// siblings). A literal percent sign (`"50% of something"`, `"20%
    /// chance"`) must still pass clean -- the discriminator is "preceded
    /// by a digit", not "any bare %".
    #[test]
    fn a_percent_hole_not_followed_by_a_digit_or_keyword_still_leaks() {
        assert_eq!(
            leaked_pcgen_syntax("Cast % 1/day"),
            Some("unsubstituted bare '%' gap")
        );
        assert_eq!(
            leaked_pcgen_syntax("Cast % at will"),
            Some("unsubstituted bare '%' gap")
        );
        assert_eq!(
            leaked_pcgen_syntax("+% enhancement"),
            Some("unsubstituted bare '%' gap")
        );
        assert_eq!(
            leaked_pcgen_syntax("Darkvision % ft."),
            Some("unsubstituted bare '%' gap")
        );
        assert_eq!(
            leaked_pcgen_syntax("Item has 10 ranks in %"),
            Some("unsubstituted bare '%' gap")
        );
        assert_eq!(
            leaked_pcgen_syntax("+%d6 additional ectoplasmic damage"),
            Some("unsubstituted bare '%' gap")
        );
        // Literal percent signs (digit immediately before '%') must never
        // be flagged by this new arm.
        assert_eq!(leaked_pcgen_syntax("a 20% chance of failure"), None);
        assert_eq!(leaked_pcgen_syntax("50%"), None);
        // Percentile-dice notation ("d%" = d100) is a real, resolved
        // shape, not a leak -- the named exception this arm must not
        // regress (`a_prose_table_beside_a_real_argument_keeps_the_table_
        // and_loses_the_tail` exercises the actual render path that
        // produces it).
        assert_eq!(leaked_pcgen_syntax("roll d% for damage"), None);
        assert_eq!(leaked_pcgen_syntax("roll a D% to determine"), None);
        // SD31-W8-INTEGRATE-001, second correction: a DICE COUNT
        // immediately before the die ("5d%") is the SAME real notation,
        // not a leak -- found against real corpus text
        // (`spell:Teleport`/`spell:Plane Shift`'s "5d%" and
        // `feat:Planar Wanderer`'s "5d20 miles ... instead of 5d% miles")
        // when the first version of this exception (word-boundary only)
        // flagged all three.
        assert_eq!(leaked_pcgen_syntax("5d% miles away"), None);
        assert_eq!(leaked_pcgen_syntax("12d% rounds"), None);
        // But a lowercase 'd' is not a blanket exception -- only the
        // named "d%" dice-notation shape at a word boundary OR preceded
        // by a dice count.
        assert_eq!(
            leaked_pcgen_syntax("gold%"),
            Some("unsubstituted bare '%' gap")
        );
    }

    /// CONFIRMED finding (integration-cycle adversarial review, `SD31-W6-
    /// INTEGRATE-001`): the equipment render path ships the raw PCGen
    /// substitution token `%CHOICE` verbatim to the player
    /// (`ultimate_equipment:equipment_modifier:special_ability_defiant_armor`'s
    /// real shipped description, "+2 enhancement bonus and DR 2/- against
    /// %CHOICE") because this guard only ever flagged `%` followed by an
    /// ASCII DIGIT, never `%` followed by an uppercase PCGen keyword.
    #[test]
    fn leak_guard_catches_percent_choice_and_other_uppercase_pcgen_substitution_keywords() {
        assert_eq!(
            leaked_pcgen_syntax("+2 enhancement bonus and DR 2/- against %CHOICE"),
            Some("unsubstituted '%<KEYWORD>' argument reference")
        );
        assert_eq!(
            leaked_pcgen_syntax("+2d6 damage against foe with %CHOICE bloodline"),
            Some("unsubstituted '%<KEYWORD>' argument reference")
        );
        // Digit case must still be caught by the SAME message it always was
        // (no regression on the pre-existing shape).
        assert_eq!(leaked_pcgen_syntax("A +%1 bonus."), Some("unsubstituted '%N' argument reference"));
        // A literal percent followed by ordinary lowercase prose is not a
        // PCGen keyword and must not false-positive.
        assert_eq!(leaked_pcgen_syntax("Clean prose with 50% of something."), None);
    }

    /// The exact ACG `Twinned Feint` token that shipped to the Add Feat
    /// picker. Its prose references no `%N`, so the argument-tail rule never
    /// fired and the `PRE` clause went straight to the player.
    #[test]
    fn a_trailing_pre_qualifier_is_stripped_even_with_no_percent_n_in_the_prose() {
        let rendered = render_pcgen_desc(
            "If you have the Improved Feint feat, you can use this feat as a move action \
             instead.|!PREABILITY:1,CATEGORY=FEAT,Improved Feint",
        );
        assert_eq!(
            rendered.text,
            "If you have the Improved Feint feat, you can use this feat as a move action instead."
        );
        assert!(rendered.dropped_args.is_empty(), "a qualifier is not a dropped argument");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// A qualifier sitting behind a real argument tail must not be counted as
    /// argument N — that would shift every `%N` onto the wrong value.
    #[test]
    fn a_qualifier_behind_an_argument_tail_is_stripped_before_the_arguments_are_numbered() {
        let rendered = render_pcgen_desc("a +%1 luck bonus|2|PRERULE:1,DisplayFullSpell");
        assert_eq!(rendered.text, "a +2 luck bonus");
        assert!(rendered.dropped_args.is_empty());
    }

    /// `PRE` is not a magic prefix on ordinary words: prose must survive.
    #[test]
    fn a_segment_that_merely_starts_with_pre_is_not_a_qualifier() {
        let raw = "Roll | Effect \nPRESENT | Nothing happens";
        assert_eq!(render_pcgen_desc(raw).text, raw);
        assert!(!is_pcgen_qualifier("PRESENT | Nothing happens"));
        assert!(is_pcgen_qualifier("PREABILITY:1,CATEGORY=FEAT,Improved Feint"));
        assert!(is_pcgen_qualifier("!PRERULE:1,DisplayFullSpell"));
    }

    /// `&nl;` is PCGen's newline escape. Three ACG feat descriptions carried
    /// it to the picker verbatim.
    #[test]
    fn the_pcgen_newline_entity_is_decoded_rather_than_shown() {
        let rendered = render_pcgen_desc("This effect lasts for 1 minute.&nl; If an ally is under it");
        assert_eq!(rendered.text, "This effect lasts for 1 minute.\n If an ally is under it");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
        assert_eq!(
            leaked_pcgen_syntax("lasts for 1 minute.&nl; If an ally"),
            Some("undecoded PCGen entity escape")
        );
    }

    /// The decode runs *after* the whitespace collapse a dropped argument
    /// triggers, so the newline is not squashed straight back into a space.
    #[test]
    fn a_decoded_newline_survives_the_dropped_argument_whitespace_collapse() {
        let rendered = render_pcgen_desc("%1 times per day.&nl; Then this.|BattleCryTimes");
        assert_eq!(rendered.text, "times per day.\n Then this.");
        assert_eq!(rendered.dropped_args, vec!["BattleCryTimes".to_string()]);
    }

    /// A `%N` with no argument at all is still never shown.
    #[test]
    fn an_unmatched_percent_n_is_dropped_rather_than_rendered() {
        let rendered = render_pcgen_desc("a bonus of %3 with no tail");
        assert_eq!(rendered.text, "a bonus of with no tail");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// CONFIRMED finding (integration-cycle full-gate run, `SD31-W6-
    /// INTEGRATE-001`): PCGen also uses uppercase KEYWORD substitutions
    /// (`%CHOICE`), not only `%N` numeric argument references. This renderer
    /// previously copied `%CHOICE` through verbatim (its main loop only
    /// recognized a digit or another `%` after `%`), which is exactly the
    /// live shape `apps/desktop/src-tauri/src/equipment_catalog.rs`'s own
    /// pre-existing `no_catalog_serves_a_description_carrying_raw_pcgen_
    /// syntax` test caught the moment `leaked_pcgen_syntax` was widened to
    /// detect it (6 real shipped equipment descriptions). There is no
    /// `PcgenDisplayValues` support for a keyword argument -- an ACG
    /// bloodline/mystery choice is a chargen-time PLAYER SELECTION this
    /// engine does not model at all, so `%CHOICE` is unconditionally
    /// dropped, the same no-fabrication treatment `%N` already gets when
    /// unresolved: it takes its introducing `+`/`-` with it and the
    /// surrounding whitespace collapses.
    #[test]
    fn a_percent_keyword_argument_is_dropped_the_same_way_an_unresolved_percent_n_is() {
        let rendered = render_pcgen_desc(
            "+2 enhancement, +2d6 damage against foe with %CHOICE bloodline",
        );
        assert_eq!(rendered.text, "+2 enhancement, +2d6 damage against foe with bloodline");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
        assert_eq!(rendered.dropped_args, vec!["CHOICE".to_string()]);
    }

    /// SD31-W8-INTEGRATE-001: the render-path counterpart to
    /// `a_percent_hole_not_followed_by_a_digit_or_keyword_still_leaks`.
    /// `leaked_pcgen_syntax` alone catching the hole was not enough --
    /// the render function must actually DROP it, or the catalog keeps
    /// serving the raw text (confirmed: `equipment_catalog`'s own
    /// `the_raw_percent_escape_stops_at_the_catalog_boundary` test found
    /// 42 served descriptions still leaking after only the detector was
    /// widened). Real corpus shapes, verbatim.
    #[test]
    fn a_bare_percent_hole_is_dropped_the_same_way_a_keyword_is() {
        let rendered = render_pcgen_desc("Cast % 1/day");
        assert_eq!(rendered.text, "Cast 1/day");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
        assert_eq!(rendered.dropped_args, vec!["%".to_string()]);

        let rendered = render_pcgen_desc("Darkvision % ft.");
        assert_eq!(rendered.text, "Darkvision ft.");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);

        let rendered = render_pcgen_desc("+%d6 additional ectoplasmic damage");
        assert_eq!(rendered.text, "d6 additional ectoplasmic damage");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);

        let rendered = render_pcgen_desc("Item has 10 ranks in %");
        assert_eq!(rendered.text, "Item has 10 ranks in");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// SD31-W8-INTEGRATE-001, third correction: a `%%` escape whose
    /// preceding NUMBER argument was itself dropped (real corpus shape,
    /// `ultimate_intrigue:spell:absolution`'s `"has a %1%% chance"` where
    /// `%1` names a formula this engine cannot resolve) must not leave an
    /// orphaned bare `%` behind -- `equipment_catalog`'s own
    /// `no_catalog_serves_a_description_carrying_raw_pcgen_syntax` test
    /// caught exactly this shape once the detector was widened to flag a
    /// bare `%`.
    #[test]
    fn an_escaped_percent_orphaned_by_its_own_dropped_argument_is_dropped_too() {
        let rendered = render_pcgen_desc("has a %1%% chance of success");
        assert_eq!(rendered.text, "has a chance of success");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
        // `%1` names argument 1, but this token carries no `|`-delimited
        // argument tail at all (the real `ultimate_intrigue:spell:
        // absolution` row's `%1` resolves against its OWN row's tail --
        // this test isolates just the escaped-`%%`-orphan behavior, so
        // `args` is empty here and `%1` itself contributes no named
        // entry to `dropped_args`, only the orphaned `%%` does).
        assert_eq!(rendered.dropped_args, vec!["%%".to_string()]);
        // The ordinary case (a digit genuinely precedes the escape) is
        // UNCHANGED -- must not regress into dropping every `%%`.
        assert_eq!(render_pcgen_desc("reduced by 20%%.").text, "reduced by 20%.");
    }

    /// `core_rulebook:spell:teleport`'s real corpus text writes percentile-
    /// dice notation with a stray space before one of its three `%%`
    /// occurrences ("Distance off target is d %% of the distance",
    /// verified against the pinned oracle at `cr_spells.lst:1371`) while
    /// the other two are tight ("roll d%%"/"rolling d%%"). The tight ones
    /// must still render "d%"; the spaced one has no digit or dice-letter
    /// immediately before it in `out` and is honestly dropped rather than
    /// guessed at -- this is NOT a fabrication regression, it is the same
    /// no-fabrication policy every other unresolvable shape in this
    /// function already follows.
    #[test]
    fn a_spaced_percentile_dice_escape_with_no_adjacent_d_is_dropped_not_guessed() {
        let rendered = render_pcgen_desc("roll d%% and consult. rolling d%%, no target. d %% of the distance.");
        assert_eq!(
            rendered.text,
            "roll d% and consult. rolling d%, no target. d of the distance."
        );
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// The percentile-dice exception must survive at render time too --
    /// this is the SAME "roll d%% for..." shape
    /// `a_prose_table_beside_a_real_argument_keeps_the_table_and_loses_
    /// the_tail` already exercises end to end; this test isolates just
    /// the bare-`%`-drop branch's own carve-out.
    #[test]
    fn a_bare_percent_hole_drop_preserves_percentile_dice_notation() {
        let rendered = render_pcgen_desc("roll d%% for damage");
        assert_eq!(rendered.text, "roll d% for damage");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// The trailing-sign-stripping shape (mirrors `an_unmatched_percent_n_
    /// is_dropped_rather_than_rendered`): a `%CHOICE` at the very end of the
    /// sentence, preceded by a bare `against`, must not leave a dangling
    /// leak or a stray trailing space.
    #[test]
    fn a_percent_keyword_at_the_end_of_the_sentence_leaves_no_trailing_leak() {
        let rendered = render_pcgen_desc("+2 enhancement bonus and DR 2/- against %CHOICE");
        assert_eq!(rendered.text, "+2 enhancement bonus and DR 2/- against");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// A `%CHOICE` at the START of the sentence (`Masterwork Tool`'s real
    /// shipped shape) must not leave a leading leak either.
    #[test]
    fn a_percent_keyword_at_the_start_of_the_sentence_leaves_no_leading_leak() {
        let rendered = render_pcgen_desc("%CHOICE circumstance Bonus");
        assert_eq!(rendered.text, "circumstance Bonus");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// A different keyword (`%LIST`, real PCGen syntax used elsewhere in
    /// this corpus's `qualifiers` fields, though not currently in any
    /// `description:`) must be caught by the SAME general mechanism, not a
    /// `%CHOICE`-specific special case.
    #[test]
    fn a_different_percent_keyword_is_also_dropped() {
        let rendered = render_pcgen_desc("a bonus to %LIST checks");
        assert_eq!(rendered.text, "a bonus to checks");
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    // -----------------------------------------------------------------
    // Character-resolved display values (operator ruling, 2026-08-01).
    // -----------------------------------------------------------------

    fn values(pairs: &[(&str, i64)]) -> PcgenDisplayValues {
        let mut out = PcgenDisplayValues::new();
        for (name, value) in pairs {
            out.set(name, *value);
        }
        out
    }

    /// The whole point: a `%N` whose argument names a variable the engine has
    /// already computed renders the number instead of vanishing.
    #[test]
    fn a_percent_n_naming_a_known_variable_renders_its_value() {
        let rendered = render_pcgen_desc_with_values(
            "[Ki Pool = %1] ...|KiPoints",
            &values(&[("KiPoints", 3)]),
        );
        assert_eq!(rendered.text, "[Ki Pool = 3] ...");
        assert!(rendered.dropped_args.is_empty());
        assert_eq!(leaked_pcgen_syntax(&rendered.text), None);
    }

    /// The same token with a different character state renders a different
    /// number. This is the property that makes it a display value rather than
    /// a constant baked at ingest.
    #[test]
    fn the_same_token_renders_a_different_number_for_a_different_character() {
        let raw = "[Ki Pool = %1] ...|KiPoints";
        assert_eq!(
            render_pcgen_desc_with_values(raw, &values(&[("KiPoints", 3)])).text,
            "[Ki Pool = 3] ..."
        );
        assert_eq!(
            render_pcgen_desc_with_values(raw, &values(&[("KiPoints", 9)])).text,
            "[Ki Pool = 9] ..."
        );
    }

    /// An unknown variable is still dropped and reported, never guessed. The
    /// no-fabrication rule is unchanged by the new capability.
    #[test]
    fn an_unknown_variable_is_still_dropped_and_reported() {
        let rendered =
            render_pcgen_desc_with_values("You add +%1 to Perception.|TrapfindingBonus", &values(&[]));
        assert_eq!(rendered.text, "You add to Perception.");
        assert_eq!(rendered.dropped_args, vec!["TrapfindingBonus".to_string()]);
    }

    /// The one non-bare argument shape the whole corpus contains.
    #[test]
    fn the_single_offset_argument_shape_the_corpus_uses_resolves() {
        let rendered = render_pcgen_desc_with_values(
            "the full +%1 bonus; afterward only +%2.|Halfling_AdaptableLuck_Bonus|Halfling_AdaptableLuck_Bonus-1",
            &values(&[("Halfling_AdaptableLuck_Bonus", 2)]),
        );
        assert_eq!(rendered.text, "the full +2 bonus; afterward only +1.");
        assert!(rendered.dropped_args.is_empty());
    }

    /// A `PREVAR*` gate that is decidably false removes its whole segment, so
    /// the "Three" / "%1" alternation PCGen writes renders one branch, not both.
    #[test]
    fn a_decidably_false_prevar_gate_drops_its_segment() {
        let tokens = [
            "Three|PREVARLTEQ:Halfling_AdaptableLuck_Times,3",
            "%1|Halfling_AdaptableLuck_Times|PREVARGTEQ:Halfling_AdaptableLuck_Times,4",
            "times per day.",
        ];
        assert_eq!(
            render_pcgen_desc_tokens(&tokens, &values(&[("Halfling_AdaptableLuck_Times", 3)])).text,
            "Three times per day."
        );
        assert_eq!(
            render_pcgen_desc_tokens(&tokens, &values(&[("Halfling_AdaptableLuck_Times", 5)])).text,
            "5 times per day."
        );
    }

    /// An undecidable gate keeps its segment. Dropping it would delete real
    /// prose on the strength of a variable the engine has not resolved.
    #[test]
    fn an_undecidable_gate_keeps_its_segment() {
        let tokens = ["Three|PREVARLTEQ:Halfling_AdaptableLuck_Times,3", "times per day."];
        assert_eq!(
            render_pcgen_desc_tokens(&tokens, &values(&[])).text,
            "Three times per day."
        );
    }

    /// `PREVAREQ` / `PREVARGT`, the other two comparators the corpus's seven
    /// `DESC` gates use, on the record that uses them.
    #[test]
    fn the_rogues_edge_gate_pair_selects_singular_or_plural_prose() {
        let tokens = [
            "You have mastered",
            "a single skill beyond that skill's normal boundaries,|PREVAREQ:RoguesEdgeLVL,1",
            "%1 skills beyond those skill's normal boundaries,|RoguesEdgeLVL|PREVARGT:RoguesEdgeLVL,1",
            "gaining results others only dream about.",
        ];
        assert_eq!(
            render_pcgen_desc_tokens(&tokens, &values(&[("RoguesEdgeLVL", 1)])).text,
            "You have mastered a single skill beyond that skill's normal boundaries, gaining results others only dream about."
        );
        assert_eq!(
            render_pcgen_desc_tokens(&tokens, &values(&[("RoguesEdgeLVL", 3)])).text,
            "You have mastered 3 skills beyond those skill's normal boundaries, gaining results others only dream about."
        );
    }

    /// Back-compatibility: the value-free entry point must behave exactly as
    /// it did before this capability existed. Every caller that has no
    /// character in hand (the spell catalog) keeps its current output.
    #[test]
    fn the_value_free_entry_point_is_unchanged_by_the_new_capability() {
        for raw in [
            "contained within you for up to %1 rounds.|CASTERLEVEL",
            "a +%1 luck bonus|2",
            "reduced by 20%%.",
            "If you have the Improved Feint feat.|!PREABILITY:1,CATEGORY=FEAT,Improved Feint",
        ] {
            assert_eq!(
                render_pcgen_desc(raw),
                render_pcgen_desc_with_values(raw, &PcgenDisplayValues::new()),
                "{raw}"
            );
        }
    }
}
