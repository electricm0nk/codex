import { loadGe08AuthoringWorkbench } from '../boundary/loadGe08AuthoringWorkbench';
import { loadPilotShellSnapshot } from '../boundary/loadPilotShellSnapshot';
import { loadSd12ReleaseTruth } from '../boundary/loadSd12ReleaseTruth';
import {
  loadSd11TesterWorkbenchSurface,
  type Sd11TesterWorkbenchSurface,
  type Sd11WorkbenchRuntimeContext,
} from './loadSd11TesterWorkbenchSurface';

export async function loadSd11TesterWorkbenchSurfaceRuntime(
  context: Sd11WorkbenchRuntimeContext
): Promise<Sd11TesterWorkbenchSurface> {
  return loadSd11TesterWorkbenchSurface(context, {
    loadGe08AuthoringWorkbench,
    loadPilotShellSnapshot,
    loadSd12ReleaseTruth,
  });
}
