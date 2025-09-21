// Shared types for the p2p chat app

export interface Group {
    id: ChatId;
    name: string;
    memberCount: number;
}

export interface ChatMessage {
    content: string;
    author: PubKey; // 32-byte public key
    timestamp: number;
}

export interface Participant {
    publicKey: PubKey;
    name: string;
    avatar: string; // base64 dataUrl
}

export interface Friend {
    publicKey: PubKey;
    addedAt: number; // timestamp when friend was added
}

export type PubKey = string;
export type ChatId = string;
export type FriendCode = string; 
