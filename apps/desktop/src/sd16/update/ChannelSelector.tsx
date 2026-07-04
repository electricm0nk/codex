import type { CSSProperties } from 'react';
import {
  SD16_UPDATE_CHANNEL_OPTIONS,
  type Sd16UpdateChannelLabel,
} from './updateModel';

export interface Sd16ChannelSelectorProps {
  selected: Sd16UpdateChannelLabel;
  onChange: (channel: Sd16UpdateChannelLabel) => void;
  disabled?: boolean;
}

const SELECT_STYLE: CSSProperties = {
  padding: '4px 8px',
  borderRadius: '4px',
  border: '1px solid #d0d7de',
  fontSize: '14px',
  backgroundColor: '#ffffff',
};

/**
 * The pinned channel dropdown. AV-UI-1 forces the rendered option list to
 * be exactly `["alpha","beta","stable"]` in that order — the release
 * promotion order, not the stability order.
 */
export function Sd16ChannelSelector({
  selected,
  onChange,
  disabled = false,
}: Sd16ChannelSelectorProps) {
  return (
    <label data-testid="sd16-channel-selector-label">
      <span>Channel: </span>
      <select
        id="channel-selector"
        data-testid="sd16-channel-selector"
        value={selected}
        disabled={disabled}
        onChange={(event) =>
          onChange(event.currentTarget.value as Sd16UpdateChannelLabel)
        }
        style={SELECT_STYLE}
      >
        {SD16_UPDATE_CHANNEL_OPTIONS.map((channel) => (
          <option key={channel} value={channel}>
            {channel}
          </option>
        ))}
      </select>
    </label>
  );
}
