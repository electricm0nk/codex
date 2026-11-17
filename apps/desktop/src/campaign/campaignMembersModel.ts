import type { FolderMember } from '../boundary/campaignFolder/types';

/**
 * The campaign's typed member list (SD-42 `decisions.md §13.1`): plain names, the creator always
 * member #1. No emails, no roles, no invites — sharing happens outside Codex.
 */

const YOUR_NAME_KEY = 'codex.campaign.yourName';

/** The last "Your name" typed on this install, or '' when none (or storage is unavailable). */
export function readYourName(): string {
  try {
    return localStorage.getItem(YOUR_NAME_KEY) ?? '';
  } catch {
    return '';
  }
}

export function rememberYourName(name: string): void {
  try {
    localStorage.setItem(YOUR_NAME_KEY, name.trim());
  } catch {
    /* the pre-fill is a convenience; the campaign still saves its members */
  }
}

/** The members a new campaign is saved with: the creator first, then the other typed names. */
export function membersForCreate(yourName: string, otherNames: string[], now: string): FolderMember[] {
  const creator = yourName.trim();
  if (!creator) {
    throw new Error('Your name is required: the creator is member #1.');
  }
  const others = otherNames.map((name) => name.trim()).filter(Boolean);
  return [creator, ...others].map((name) => ({ name, addedAt: now }));
}

/** A blank row appended for the user to type into. */
export function addMember(members: FolderMember[], now: string): FolderMember[] {
  return [...members, { name: '', addedAt: now }];
}

export function renameMember(members: FolderMember[], index: number, name: string): FolderMember[] {
  return members.map((member, i) => (i === index ? { ...member, name } : member));
}

/** Removes any member but the first (the creator), so the list never drops below one. */
export function removeMember(members: FolderMember[], index: number): FolderMember[] {
  if (index <= 0 || index >= members.length) {
    return members;
  }
  return members.filter((_, i) => i !== index);
}

/** The list as saved: names trimmed, blank rows after the first dropped; a blank first is refused. */
export function membersForSave(members: FolderMember[], now: string): FolderMember[] {
  const [first, ...rest] = members.map((member) => ({ ...member, name: member.name.trim() }));
  if (!first?.name) {
    throw new Error('The first member (the campaign creator) needs a name.');
  }
  return [first, ...rest.filter((member) => member.name)].map((member) => ({
    name: member.name,
    addedAt: member.addedAt || now,
  }));
}
