import { useState } from 'react';
import { addFriend, deleteFriend, getFriends, updateFriend, type Friend } from './friends';

const NEW_FRIEND_ID = 'new';

const rowInputStyle = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  fontSize: '0.85rem',
  padding: '0.4rem 0.6rem',
  width: '100%',
} as const;

function isValid(name: string, gmailAddress: string): boolean {
  return name.trim().length > 0 && gmailAddress.trim().includes('@');
}

/** A friend's row, in its own form while being added or edited. */
function FriendForm(props: {
  initialName?: string;
  initialGmail?: string;
  onCancel: () => void;
  onSave: (name: string, gmailAddress: string) => void;
}) {
  const [name, setName] = useState(props.initialName ?? '');
  const [gmailAddress, setGmailAddress] = useState(props.initialGmail ?? '');
  const valid = isValid(name, gmailAddress);

  return (
    <div
      style={{
        backgroundColor: 'var(--color-surface)',
        border: '1px solid var(--color-accent)',
        borderRadius: 8,
        display: 'flex',
        gap: '0.5rem',
        padding: '0.6rem 0.8rem',
      }}
    >
      <div style={{ display: 'flex', flex: 1, gap: '0.5rem' }}>
        <input type="text" placeholder="Name" value={name} onChange={(event) => setName(event.target.value)} style={rowInputStyle} />
        <input
          type="email"
          placeholder="friend@gmail.com"
          value={gmailAddress}
          onChange={(event) => setGmailAddress(event.target.value)}
          style={rowInputStyle}
        />
      </div>
      <button
        type="button"
        onClick={() => valid && props.onSave(name.trim(), gmailAddress.trim())}
        disabled={!valid}
        style={{
          backgroundColor: 'var(--color-accent)',
          border: 'none',
          borderRadius: 6,
          color: 'var(--color-on-accent)',
          cursor: valid ? 'pointer' : 'not-allowed',
          flexShrink: 0,
          fontSize: '0.8rem',
          fontWeight: 600,
          opacity: valid ? 1 : 0.5,
          padding: '0.3rem 0.7rem',
        }}
      >
        Save
      </button>
      <button
        type="button"
        onClick={props.onCancel}
        style={{
          background: 'none',
          border: '1px solid var(--color-border)',
          borderRadius: 6,
          color: 'var(--color-text-muted)',
          cursor: 'pointer',
          flexShrink: 0,
          fontSize: '0.8rem',
          padding: '0.3rem 0.7rem',
        }}
      >
        Cancel
      </button>
    </div>
  );
}

/** Local address book of gaming contacts, for filling in campaign invites more easily. */
export function FriendsSection() {
  const [friends, setFriends] = useState<Friend[]>(getFriends());
  const [editingId, setEditingId] = useState<string | null>(null);

  function refresh() {
    setFriends(getFriends());
  }

  return (
    <div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.06em', margin: '0 0 0.3rem', textTransform: 'uppercase' }}>
        Friends
      </p>
      <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', lineHeight: 1.5, margin: '0 0 0.75rem' }}>
        Gaming contacts, kept locally, so campaign invites are quicker to fill in.
      </p>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
        {friends.map((friend) =>
          editingId === friend.id ? (
            <FriendForm
              key={friend.id}
              initialName={friend.name}
              initialGmail={friend.gmailAddress}
              onCancel={() => setEditingId(null)}
              onSave={(name, gmailAddress) => {
                updateFriend(friend.id, { name, gmailAddress });
                setEditingId(null);
                refresh();
              }}
            />
          ) : (
            <div
              key={friend.id}
              style={{
                alignItems: 'center',
                border: '1px solid var(--color-border)',
                borderRadius: 8,
                display: 'flex',
                gap: '0.75rem',
                justifyContent: 'space-between',
                padding: '0.6rem 0.8rem',
              }}
            >
              <div style={{ minWidth: 0 }}>
                <p style={{ fontWeight: 700, margin: 0 }}>{friend.name}</p>
                <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.78rem', margin: '0.15rem 0 0' }}>{friend.gmailAddress}</p>
              </div>
              <div style={{ display: 'flex', flexShrink: 0, gap: '0.4rem' }}>
                <button
                  type="button"
                  onClick={() => setEditingId(friend.id)}
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: '1px solid var(--color-border)',
                    borderRadius: 6,
                    color: 'var(--color-text)',
                    cursor: 'pointer',
                    fontSize: '0.8rem',
                    fontWeight: 600,
                    padding: '0.3rem 0.7rem',
                  }}
                >
                  Edit
                </button>
                <button
                  type="button"
                  onClick={() => {
                    deleteFriend(friend.id);
                    refresh();
                  }}
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: '1px solid var(--color-error-border)',
                    borderRadius: 6,
                    color: 'var(--color-error)',
                    cursor: 'pointer',
                    fontSize: '0.8rem',
                    fontWeight: 600,
                    padding: '0.3rem 0.7rem',
                  }}
                >
                  Delete
                </button>
              </div>
            </div>
          )
        )}

        {editingId === NEW_FRIEND_ID ? (
          <FriendForm
            onCancel={() => setEditingId(null)}
            onSave={(name, gmailAddress) => {
              addFriend(name, gmailAddress);
              setEditingId(null);
              refresh();
            }}
          />
        ) : null}
      </div>

      {editingId !== NEW_FRIEND_ID ? (
        <button
          type="button"
          onClick={() => setEditingId(NEW_FRIEND_ID)}
          style={{
            background: 'none',
            border: '1px dashed var(--color-border)',
            borderRadius: 8,
            color: 'var(--color-text-secondary)',
            cursor: 'pointer',
            fontSize: '0.85rem',
            marginTop: '0.5rem',
            padding: '0.4rem 0.75rem',
          }}
        >
          + Add friend
        </button>
      ) : null}
    </div>
  );
}
