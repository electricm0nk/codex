import * as model from './characterHubModel';
import { assert } from '../testSupport/asserts';

/**
 * F-12 (scout audit item 7, orchestrator ruling Q3): the create form
 * previewed a PF1 aging ability modifier in its Calculated column that
 * `handleSubmit` never submitted, so the saved character disagreed with
 * what the player was shown. The engine has no aging model, and authoring
 * one in TS is forbidden (brief §3.3), so the table and helper are deleted
 * rather than wired through. This pins that they stay deleted — the form
 * cannot re-import what does not exist.
 */
const exported = model as Record<string, unknown>;
assert(!('ageEffectForAbility' in exported), 'ageEffectForAbility (TS aging rules math) must not exist');
assert(!('AGE_EFFECTS' in exported), 'AGE_EFFECTS (TS aging rules table) must not exist');
assert('AGE_OPTIONS' in exported, 'AGE_OPTIONS stays: it is a bio label list, not rules math');
console.log('noAgingPreview tests passed');
