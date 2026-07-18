import type { CSSProperties } from 'react';
import {
  UPDATE_CHANNEL_OPTIONS,
  type UpdateChannelLabel,
} from './updateModel';

export interface ChannelSelectorProps {
  selected: UpdateChannelLabel;
  onChange: (channel: UpdateChannelLabel) => void;
  disabled?: boolean;
}

const SELECT_STYLE: CSSProperties = {
  padding: '4px 8px',
  borderRadius: '4px',
  border: '1px solid var(--color-border)',
  fontSize: '14px',
  backgroundColor: 'var(--color-surface)',
};

/**
 * The pinned channel dropdown. AV-UI-1 forces the rendered option list to
 * be exactly `["alpha","beta","stable"]` in that order — the release
 * promotion order, not the stability order.
 */
export function ChannelSelector({
  selected,
  onChange,
  disabled = false,
}: ChannelSelectorProps) {
  return (
    <label data-testid="sd16-channel-selector-label">
      <span>Channel: </span>
      <select
        id="channel-selector"
        data-testid="sd16-channel-selector"
        value={selected}
        disabled={disabled}
        onChange={(event) =>
          onChange(event.currentTarget.value as UpdateChannelLabel)
        }
        style={SELECT_STYLE}
      >
        {UPDATE_CHANNEL_OPTIONS.map((channel) => (
          <option key={channel} value={channel}>
            {channel}
          </option>
        ))}
      </select>
    </label>
  );
}
