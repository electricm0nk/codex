import { resolveSpellRouting } from './spellRoutingModel';
import type { HeldClass } from './characterProgression';
import type { SpellSelectionDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';

const WIZARD_CLASS_ID = 'class:wizard';

function heldClass(classId: string): HeldClass {
  return { classId, classLabel: classId, level: 1 };
}

/**
 * risks-and-open-questions.md item 25: `handleAddSpell`'s two real routing
 * decisions (which held class a spell pick attributes to, and whether that
 * pick needs the atomic Wizard-bootstrap command) had no test coverage
 * anywhere before this extraction.
 */
function verifiesFighterOnlyBuildRoutesToTheHeldFighterClassAndThePlainPath() {
  const routing = resolveSpellRouting([heldClass('class:fighter')], [], WIZARD_CLASS_ID);
  if (!routing) {
    throw new Error('a Fighter-only build must resolve a routing decision');
  }
  assertEqual(routing.primaryClassId, 'class:fighter', 'a Fighter-only build attributes the pick to Fighter');
  assert(!routing.useAtomicBootstrap, 'a non-Wizard pick never needs the atomic bootstrap path');
}

function verifiesFighterWizardMulticlassResolvesToWizardNotHeldClassesZeroAndRoutesToTheAtomicPath() {
  const routing = resolveSpellRouting([heldClass('class:fighter'), heldClass(WIZARD_CLASS_ID)], [], WIZARD_CLASS_ID);
  if (!routing) {
    throw new Error('a Fighter/Wizard multiclass must resolve a routing decision');
  }
  assertEqual(
    routing.primaryClassId,
    WIZARD_CLASS_ID,
    'a Fighter/Wizard multiclass attributes the pick to Wizard, not heldClasses[0] (Fighter)'
  );
  assert(routing.useAtomicBootstrap, 'a Wizard pick with no existing Wizard spell needs the atomic bootstrap path');
}

function verifiesWizardOnlyBuildRoutesToTheAtomicPath() {
  const routing = resolveSpellRouting([heldClass(WIZARD_CLASS_ID)], [], WIZARD_CLASS_ID);
  if (!routing) {
    throw new Error('a Wizard-only build must resolve a routing decision');
  }
  assertEqual(routing.primaryClassId, WIZARD_CLASS_ID, 'a Wizard-only build attributes the pick to Wizard');
  assert(routing.useAtomicBootstrap, 'a Wizard-only build with no existing Wizard spell needs the atomic bootstrap path');
}

function verifiesAWizardBuildWithAnExistingSpellRoutesToThePlainPath() {
  const existingSpells: SpellSelectionDto[] = [
    { spellId: 'Light', sourceClassId: WIZARD_CLASS_ID, acquisitionMode: 'Known' },
  ];
  const routing = resolveSpellRouting([heldClass(WIZARD_CLASS_ID)], existingSpells, WIZARD_CLASS_ID);
  assert(
    routing !== null && !routing.useAtomicBootstrap,
    'once a Wizard spell already exists, a later pick uses the plain path, not the bootstrap'
  );
}

function verifiesNoHeldClassResolvesToNull() {
  const routing = resolveSpellRouting([], [], WIZARD_CLASS_ID);
  assertEqual(routing, null, 'a character with no held class at all resolves to null, not a fabricated class id');
}

async function main() {
  verifiesFighterOnlyBuildRoutesToTheHeldFighterClassAndThePlainPath();
  verifiesFighterWizardMulticlassResolvesToWizardNotHeldClassesZeroAndRoutesToTheAtomicPath();
  verifiesWizardOnlyBuildRoutesToTheAtomicPath();
  verifiesAWizardBuildWithAnExistingSpellRoutesToThePlainPath();
  verifiesNoHeldClassResolvesToNull();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
