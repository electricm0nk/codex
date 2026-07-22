import { loadGe08AuthoringWorkbench } from '../boundary/loadGe08AuthoringWorkbench';
import { loadPilotShellSnapshot } from '../boundary/loadPilotShellSnapshot';
import { loadReleaseTruth } from '../boundary/loadReleaseTruth';
import { loadSupportStateMatrix } from '../boundary/loadSupportStateMatrix';
import { loadBackendHealth } from '../boundary/loadBackendHealth';
import {
  loadTesterWorkbenchSurface,
  type TesterWorkbenchSurface,
  type WorkbenchRuntimeContext,
} from './loadTesterWorkbenchSurface';

export async function loadTesterWorkbenchSurfaceRuntime(
  context: WorkbenchRuntimeContext
): Promise<TesterWorkbenchSurface> {
  return loadTesterWorkbenchSurface(context, {
    loadGe08AuthoringWorkbench,
    loadPilotShellSnapshot,
    loadReleaseTruth,
    loadSupportStateMatrix,
    loadBackendHealth,
  });
}
