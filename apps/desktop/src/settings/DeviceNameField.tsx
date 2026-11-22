import { useEffect, useState } from 'react';
import { getDeviceIdentity, setDeviceName } from '../boundary/campaignFolder/characters';

/** "This device's name": what the other devices read in a campaign backup's master record. */
export function DeviceNameField() {
  const [saved, setSaved] = useState<string | null>(null);
  const [name, setName] = useState('');
  const [message, setMessage] = useState('');

  useEffect(() => {
    getDeviceIdentity()
      .then((device) => {
        setSaved(device.name);
        setName(device.name);
      })
      .catch((err: unknown) => setMessage(String(err)));
  }, []);

  const save = () => {
    setDeviceName(name)
      .then((device) => {
        setSaved(device.name);
        setName(device.name);
        setMessage('Saved.');
      })
      .catch((err: unknown) => setMessage(String(err)));
  };

  return (
    <section aria-label="This device's name" style={{ display: 'grid', gap: '0.4rem', marginTop: '1.25rem' }}>
      <label htmlFor="device-name" style={{ fontWeight: 700 }}>This device&apos;s name</label>
      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <input
          id="device-name"
          type="text"
          value={name}
          onChange={(event) => setName(event.target.value)}
          style={{ flex: 1, backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text)', padding: '0.4rem 0.6rem' }}
        />
        <button type="button" onClick={save} disabled={saved === null || name.trim().length === 0 || name.trim() === saved}>
          Save
        </button>
      </div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: 0 }}>
        Other devices see this name on this device&apos;s character backups. {message}
      </p>
    </section>
  );
}
