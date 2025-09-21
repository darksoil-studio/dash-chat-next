import { writable } from 'svelte/store';
import type { Group, Friend, ChatMessage, Participant } from './types.js';

// Global state stores
export const groups = writable<Group[]>([]);
export const friends = writable<Friend[]>([]);
export const messages = writable<ChatMessage[]>([]);
export const participants = writable<Map<string, Participant>>(new Map());
export const myPublicKey = writable<string>("");
export const selectedGroup = writable<Group | null>(null);

// UI state
export const toastMessage = writable<string>("");
export const showToast = writable<boolean>(false);
export const isErrorToast = writable<boolean>(false);

// Toast helper function
export function showToastMessage(message: string, isError = false) {
    toastMessage.set(message);
    isErrorToast.set(isError);
    showToast.set(true);

    // Auto-hide after 3 seconds
    setTimeout(() => {
        showToast.set(false);
    }, 3000);
}
