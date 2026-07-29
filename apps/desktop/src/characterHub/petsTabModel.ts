import type {
  AnimalCompanionDto,
  CompanionStatDto,
  PilotSnapshotDto,
} from '../boundary/loadCreateCharacter';

/**
 * Pure logic backing `PetsTab`: turning the engine's real animal-companion
 * / mount stat block into something a player can read.
 *
 * Why this exists (traced against source, not assumed): the Pets tab fell
 * through to the sheet's generic `"{tab} — coming soon."` placeholder while
 * the companion was fully computed in `pilot_compute.rs` the whole time —
 * `ground_wolf_companion_stat_block` / `ground_horse_companion_stat_block`
 * emit Hit Dice, base attack bonus, all three base saves, hit points,
 * armor class, the natural-armor and Strength advances and the natural
 * attack, across all twenty master levels, for Druid, Hunter and the
 * Cavalier's Mount. The values simply stopped at the Tauri DTO boundary:
 * `PilotSnapshotDto` carried no companion field, exactly the way
 * `EquipmentEffects.per_item` was fully populated and simply not carried,
 * leaving an "AC breakdown by source" panel sitting as a placeholder over
 * data that already existed. Same defect shape as the feats one
 * (`featsTabModel.ts`) and the spells one (`spellsTabModel.ts`).
 *
 * This module fabricates nothing. Every number and every line of prose it
 * renders arrives from the engine via `PilotSnapshotDto.companion`; the
 * only judgements made here are presentational — how a value is written,
 * what order rows read in (already canonical from the engine side), and
 * which of the three genuinely different empty states applies.
 */

/** One stat-block row, ready to render. */
export interface RenderedCompanionStat {
  /** The engine's own label for what `value` is, verbatim. */
  label: string;
  value: number;
  /** `value` written the way this kind of statistic is written. */
  rendered: string;
  /** The engine's own derivation prose, verbatim. */
  detail: string;
}

/**
 * What the Pets tab shows. Three genuinely distinct states, deliberately
 * not collapsed into two:
 *
 * * `Companion` — this build grounds one, here it is.
 * * `None` — this build computed, and it grounds no companion. A real,
 *   known answer about this character.
 * * `Unavailable` — the build is blocked, so there is no snapshot and the
 *   tab genuinely does not know. Rendering this as `None` would state a
 *   fact the app has not established.
 */
export type PetsTabView =
  | {
      kind: 'Companion';
      /** The species, e.g. `"Wolf"`. */
      heading: string;
      /** Who it belongs to and what they call it, e.g. `"Druid Animal Companion"`. */
      subheading: string;
      /** The engine's recognition-record prose. */
      summaryDetail: string;
      stats: RenderedCompanionStat[];
      /** Provably-vacuous named abilities (Link, Share Spells). */
      notes: string[];
      /**
       * The engine's own honest list of the companion columns deliberately
       * left ungrounded, or `null` when it emitted none. Never paraphrased
       * and never substituted for with an invented list.
       */
      advancementNote: string | null;
    }
  | { kind: 'None'; message: string }
  | { kind: 'Unavailable'; message: string };

/**
 * Whether a statistic is a modifier (written with an explicit sign) or a
 * total (written plain).
 *
 * Keyed off the engine's own label rather than a hardcoded list of record
 * ids, so a companion column added engine-side inherits the right
 * treatment automatically: PF1 writes every bonus/save/modifier signed and
 * every pool/score plain, and the engine's labels already say which each
 * one is. An unrecognized label falls to the plain form — understating the
 * claim rather than asserting a sign that may be wrong.
 */
function isModifierStat(label: string): boolean {
  return /\b(Bonus|Save|Modifier|Penalty)$/.test(label);
}

/**
 * How one statistic's value reads, e.g. `"17"` for Hit Points and `"+2"`
 * for an Attack Bonus.
 *
 * A real computed `0` on a modifier renders `"+0"`, not blank: a Wolf at 2
 * Hit Dice genuinely has a Will save of floor(2/3) = 0, which is a value,
 * not an absence. (Contrast `damage_reduction`, which the engine itself
 * filters at zero because real PF1 has no "DR 0" — that filtering is the
 * engine's call and has already happened before anything reaches here.)
 */
export function formatCompanionStatValue(stat: CompanionStatDto): string {
  if (!isModifierStat(stat.label)) {
    return String(stat.value);
  }
  return stat.value < 0 ? String(stat.value) : `+${stat.value}`;
}

/**
 * Projects the loaded snapshot onto the Pets tab's view.
 *
 * Takes the whole snapshot rather than just `snapshot.companion` so it can
 * tell a blocked build (`null`/`undefined` snapshot — nothing is known)
 * apart from a computed one that simply has no companion.
 */
export function buildPetsTabView(snapshot: PilotSnapshotDto | null | undefined): PetsTabView {
  if (snapshot === null || snapshot === undefined) {
    return {
      kind: 'Unavailable',
      message:
        'This build has not computed, so its companion — if it has one — cannot be shown yet. Resolve the blocking diagnostics on the sheet first.',
    };
  }

  const companion = snapshot.companion;
  if (companion === undefined || companion === null) {
    return {
      kind: 'None',
      message: 'This character has no animal companion or mount.',
    };
  }

  return {
    kind: 'Companion',
    heading: companion.species,
    subheading: `${companion.ownerClassLabel} ${companion.roleLabel}`,
    summaryDetail: companion.summaryDetail,
    stats: companion.stats.map((stat) => ({
      label: stat.label,
      value: stat.value,
      rendered: formatCompanionStatValue(stat),
      detail: stat.detail,
    })),
    notes: companion.notes,
    advancementNote: companion.advancementNote ?? null,
  };
}

/** Re-exported for the component, which needs the DTO shape it renders. */
export type { AnimalCompanionDto };
