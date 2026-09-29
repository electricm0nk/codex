import { rosterErrorMessage, type RaceRosterSurface } from './raceRoster';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-36: the packaged-build "No race could be read from the corpus." defect
 * had no path in it a player or operator could act on. `rosterErrorMessage`
 * is the seam that puts the backend's own diagnostic path onto the screen;
 * these pin that it actually does, and that a usable roster renders no error
 * at all.
 */
function verifiesTheDiagnosticPathReachesTheScreen() {
  const surface: RaceRosterSurface = {
    options: [],
    diagnostics: ['corpus root not found: /x/data/corpus'],
  };
  const message = rosterErrorMessage(surface);
  assert(message !== null, 'an empty roster must produce an error message');
  assert(
    message !== null && message.includes('corpus root not found: /x/data/corpus'),
    `message must carry the backend diagnostic's own path, got: ${String(message)}`
  );
}

function verifiesAnEmptyRosterWithNoDiagnosticsStillReportsAnHonestMessage() {
  const surface: RaceRosterSurface = { options: [], diagnostics: [] };
  assertEqual(
    rosterErrorMessage(surface),
    'No race could be read from the corpus.',
    'no diagnostics at all still needs an honest message, not a blank screen'
  );
}

function verifiesANonEmptyRosterProducesNoErrorMessage() {
  const surface: RaceRosterSurface = {
    options: [
      {
        id: 'race:human',
        label: 'Human',
        book: 'CRB',
        abilityAdjustments: {},
        floatingBonusPoints: 2,
        size: 'Medium',
        vision: 'Normal',
        baseSpeedFt: 30,
        body: null,
      },
    ],
    diagnostics: [],
  };
  assertEqual(rosterErrorMessage(surface), null, 'a usable roster renders the picker, not an error');
}

async function main() {
  verifiesTheDiagnosticPathReachesTheScreen();
  verifiesAnEmptyRosterWithNoDiagnosticsStillReportsAnHonestMessage();
  verifiesANonEmptyRosterProducesNoErrorMessage();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
