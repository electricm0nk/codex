import type { AdmittedKindDto, AdmittedRecordDto, OverrideFlagDto } from '../boundary/loadSavedCharacterDetail';
import { OVERRIDE_TOGGLES } from './customModel';

export function overrideTagText(flag: OverrideFlagDto): string {
  return `Override: ${OVERRIDE_TOGGLES.find((toggle) => toggle.key === flag)?.label ?? flag}`;
}

/** The flags that let in the record of this kind and id; none for what the rules allowed. */
export function flagsOf(admitted: readonly AdmittedRecordDto[] | undefined, kind: AdmittedKindDto, id: string): OverrideFlagDto[] {
  return (admitted ?? []).filter((row) => row.kind === kind && row.id === id).map((row) => row.override);
}

/** One line a class for the levels an override let in. No tab lists a level, so the Custom tab does. */
export function classLevelsAdmitted(
  admitted: readonly AdmittedRecordDto[] | undefined,
  classes: ReadonlyArray<{ classId: string; classLabel: string }>,
): Array<{ label: string; levels: number; flag: OverrideFlagDto }> {
  const lines: Array<{ id: string; label: string; levels: number; flag: OverrideFlagDto }> = [];
  for (const row of admitted ?? []) {
    if (row.kind !== 'class_level') {
      continue;
    }
    const line = lines.find((other) => other.id === row.id && other.flag === row.override);
    if (line) {
      line.levels += 1;
    } else {
      const label = classes.find((held) => held.classId === row.id)?.classLabel ?? row.id;
      lines.push({ id: row.id, label, levels: 1, flag: row.override });
    }
  }
  return lines.map(({ label, levels, flag }) => ({ label, levels, flag }));
}

/** What says a record is here because an override let it in, not because the rules allow it. */
export function OverrideTag(props: { flag: OverrideFlagDto }) {
  return (
    <span
      style={{
        border: '1px solid var(--color-accent)',
        borderRadius: 4,
        color: 'var(--color-accent)',
        fontSize: '0.68rem',
        fontWeight: 700,
        marginLeft: '0.5rem',
        padding: '0.05rem 0.3rem',
        whiteSpace: 'nowrap',
      }}
    >
      {overrideTagText(props.flag)}
    </span>
  );
}
