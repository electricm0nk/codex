import type {
  RaceSelectionResponse,
  RenderedTraitDescriptionDto,
} from '../boundary/loadAlternateRacialTraits';

/**
 * Projects `load_saved_character`'s `resolvedRacialTraits` payload into the
 * character sheet's Racial Traits section.
 *
 * **This module authors no rules and composes no sentence.** Which traits
 * apply is `RaceCorpus::resolve`'s answer; what each one *says* is
 * `race_trait_picker::render_trait_description`'s, rendered against the
 * character's own persisted feats before it ever reached the wire
 * (`decisions.md §29.1`: one renderer, several consumers). Everything below is
 * grouping and labelling — the same discipline `classFeaturesModel.ts` applies
 * to the engine's `class_feature.*` records.
 *
 * The defect it closes is narrow and was visible on every sheet: a chosen
 * alternate racial trait rendered as a name-only card
 * (`traitKey.split(' ~ ')` → `"Adaptable Luck"`), so a player could see *that*
 * they had taken a trait and never what it did or what it cost them. The
 * standard traits their race keeps were not on the sheet at all.
 *
 * Three rules govern everything here:
 *
 * 1. **`text` is rendered verbatim.** It is corpus prose with the engine's own
 *    numbers resolved into it.
 * 2. **A move is the engine's claim, never this module's.** `movedByFeats` and
 *    `displayValueFeats` are both derived in Rust by rendering the record twice
 *    and comparing; nothing here infers a move from the feat list.
 * 3. **Absence is rendered as absence.** A dropped `DESC:` argument means the
 *    engine could not resolve a magnitude, and
 *    [`RacialTraitsSurface.incompleteRows`] carries that to the screen rather
 *    than letting a partially-rendered sentence read as a whole one.
 */

/** One racial trait as the sheet shows it. */
export interface RacialTraitRow {
  /** The corpus key, verbatim — the row's stable identity and audit handle. */
  key: string;
  name: string;
  /** `'CRB'` / `'B1'` / `'ARG'`, the backend's own book code. */
  book: string;
  /**
   * `'default'`, `'alternate'` or `'flagGranted'` — the resolver's own
   * classification, passed through unchanged. An unrecognised value is kept
   * as-is rather than coerced into one of the three.
   */
  role: string;
  /** A readable rendering of [`role`](RacialTraitRow.role). */
  roleLabel: string;
  /** The engine-rendered prose. Render verbatim. */
  text: string;
  /** `DESC:` arguments the engine could not resolve, so a gap is visible. */
  droppedArgs: string[];
  /** True when this character's own feats changed the sentence. */
  movedByFeats: boolean;
  /**
   * The standard trait(s) this row replaced, by display name — read off the
   * resolver's `suppressions`, matched on the trait key it recorded as the
   * setter. Never inferred from names or flags here.
   */
  replaces: string[];
}

/** A standard trait a chosen alternate removed. */
export interface ReplacedTrait {
  key: string;
  name: string;
  /** The alternate that replaced it. */
  byName: string;
}

/** A trait whose description the engine could only partly resolve. */
export interface IncompleteTrait {
  key: string;
  name: string;
  droppedArgs: string[];
}

export interface RacialTraitsSurface {
  /** Applied traits in the resolver's own emission order. */
  rows: RacialTraitRow[];
  /** What the chosen alternates took away, so a swap is legible as a swap. */
  replaced: ReplacedTrait[];
  /**
   * The subset of the character's feats that actually moved a number in the
   * prose above, as the engine derived it one feat at a time. The screen's
   * evidence for *why* a magnitude differs from the book's printed one.
   */
  displayValueFeats: string[];
  /** Traits whose sentence is knowingly incomplete. */
  incompleteRows: IncompleteTrait[];
  /**
   * Non-null when no traits can be shown, carrying the engine's own reason.
   *
   * Rendering an empty list silently would read as "this race has no racial
   * traits", which is a different claim and a false one.
   */
  unavailableReason: string | null;
}

const EMPTY: RacialTraitsSurface = {
  rows: [],
  replaced: [],
  displayValueFeats: [],
  incompleteRows: [],
  unavailableReason: null,
};

const ROLE_LABELS: Record<string, string> = {
  default: 'Racial trait',
  alternate: 'Alternate racial trait',
  flagGranted: 'Granted by an alternate',
};

/**
 * The reason shown when the payload is absent entirely — an older saved
 * character loaded through a build that predates the field, or a load that
 * failed before the resolver ran. Stated rather than swallowed.
 */
export const NO_RESOLUTION_MESSAGE =
  'The rules engine returned no racial-trait resolution for this character, so its traits cannot be shown.';

export function buildRacialTraitsSurface(
  resolved: RaceSelectionResponse | null | undefined,
): RacialTraitsSurface {
  if (!resolved) {
    return { ...EMPTY, unavailableReason: NO_RESOLUTION_MESSAGE };
  }
  if (resolved.errors.length > 0) {
    return { ...EMPTY, unavailableReason: resolved.errors.join('; ') };
  }

  const rendered = new Map<string, RenderedTraitDescriptionDto>(
    resolved.renderedTraitDescriptions.map((row) => [row.key, row]),
  );

  // Suppressions, keyed by the alternate the resolver recorded as the setter.
  const replacedBySetter = new Map<string, string[]>();
  for (const suppression of resolved.suppressions) {
    const existing = replacedBySetter.get(suppression.setByTraitKey);
    if (existing) {
      existing.push(suppression.suppressedTraitName);
    } else {
      replacedBySetter.set(suppression.setByTraitKey, [suppression.suppressedTraitName]);
    }
  }

  const rows: RacialTraitRow[] = resolved.appliedTraits.map((applied) => {
    const row = rendered.get(applied.key);
    return {
      key: applied.key,
      name: applied.name,
      book: applied.book,
      role: applied.role,
      roleLabel: ROLE_LABELS[applied.role] ?? applied.role,
      // The backend pins these equal for every applied trait; preferring the
      // rendered row keeps a single origin for the prose, and the applied
      // description is the honest fallback rather than a blank cell.
      text: row?.text ?? applied.description,
      droppedArgs: row?.droppedArgs ?? [],
      movedByFeats: row?.movedByFeats ?? false,
      replaces: replacedBySetter.get(applied.key) ?? [],
    };
  });

  return {
    rows,
    replaced: resolved.suppressions.map((suppression) => ({
      key: suppression.suppressedTraitKey,
      name: suppression.suppressedTraitName,
      byName: suppression.setByTraitName,
    })),
    displayValueFeats: [...resolved.displayValueFeats],
    incompleteRows: rows
      .filter((row) => row.droppedArgs.length > 0)
      .map((row) => ({ key: row.key, name: row.name, droppedArgs: row.droppedArgs })),
    unavailableReason: null,
  };
}
