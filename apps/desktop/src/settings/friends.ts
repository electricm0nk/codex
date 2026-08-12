/**
 * Friends list — a lightweight local address book of gaming contacts, meant
 * to make campaign invites easier to fill in later (campaign members today
 * are bare email strings, see campaign/campaignModel.ts's `CampaignMember`).
 * Purely local (localStorage), same persistence pattern as campaignModel.ts
 * and settings/googleDrive.ts — no backend/Tauri layer needed for this data.
 */

export interface Friend {
  id: string;
  name: string;
  gmailAddress: string;
}

const FRIENDS_KEY = 'codex.friends';

export function getFriends(): Friend[] {
  try {
    const raw = localStorage.getItem(FRIENDS_KEY);
    return raw ? (JSON.parse(raw) as Friend[]) : [];
  } catch {
    return [];
  }
}

function saveFriends(friends: Friend[]): void {
  try {
    localStorage.setItem(FRIENDS_KEY, JSON.stringify(friends));
  } catch {
    /* non-persistent environments still get the change applied this session */
  }
}

export function addFriend(name: string, gmailAddress: string): Friend {
  const friend: Friend = { id: crypto.randomUUID(), name, gmailAddress };
  saveFriends([...getFriends(), friend]);
  return friend;
}

export function updateFriend(id: string, changes: Partial<Omit<Friend, 'id'>>): Friend | null {
  const friends = getFriends();
  const index = friends.findIndex((friend) => friend.id === id);
  if (index === -1) {
    return null;
  }
  const updated: Friend = { ...friends[index], ...changes };
  friends[index] = updated;
  saveFriends(friends);
  return updated;
}

export function deleteFriend(id: string): void {
  saveFriends(getFriends().filter((friend) => friend.id !== id));
}
