<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import "./styles.css";

    // Types
    interface Group {
        id: string;
        name: string;
        memberCount: number;
        joinCode: string;
    }

    interface ChatMessage {
        id: string;
        content: string;
        author: Uint8Array; // 32-byte public key
        timestamp: number;
    }

    interface Participant {
        publicKey: Uint8Array;
        name: string;
        avatar: string; // base64 dataUrl
    }

    // State
    let currentView: "groups" | "chat" = "groups";
    let selectedGroup: Group | null = null;
    let groups: Group[] = [];
    let messages: ChatMessage[] = [];
    let participants: Map<string, Participant> = new Map();
    let newMessage = "";
    let newGroupName = "";
    let joinCode = "";
    let showCreateGroup = false;
    let showJoinGroup = false;
    let showAddMember = false;
    let newMemberPublicKey = "";
    let toastMessage = "";
    let showToast = false;
    let isErrorToast = false;

    // Group management functions
    async function loadGroups() {
        try {
            // This would call your backend to get groups
            // For now, using mock data
            groups = [
                {
                    id: "1",
                    name: "General Chat",
                    memberCount: 5,
                    joinCode:
                        "abc123def456ghi789jkl012mno345pqr678stu901vwx234yz",
                },
                {
                    id: "2",
                    name: "Project Discussion",
                    memberCount: 3,
                    joinCode:
                        "xyz789abc123def456ghi789jkl012mno345pqr678stu901vwx",
                },
            ];
        } catch (error) {
            console.error("Failed to load groups:", error);
        } finally {
            console.log("groups loaded");
        }
    }

    async function createGroup() {
        const ok = true;
        // const ok = newGroupName.trim();
        if (ok) {
            try {
                // This would call your backend to create a group
                // For now, just close the dialog
                showCreateGroup = false;
                newGroupName = "";
                if (
                    await invoke("create_chat", {
                        name: newGroupName,
                    })
                ) {
                    showToastMessage("Group created successfully!");
                } else {
                    showToastMessage("Failed to create group", true);
                }
                await loadGroups(); // Reload groups after creation
            } catch (error) {
                console.error("Failed to create group:", error);
                showToastMessage("Failed to create group", true);
            }
        }
    }

    async function joinGroup() {
        if (joinCode.trim()) {
            try {
                // This would call your backend to join a group
                // For now, just close the dialog
                showJoinGroup = false;
                joinCode = "";
                await loadGroups(); // Reload groups after joining
                showToastMessage("Successfully joined group!");
            } catch (error) {
                console.error("Failed to join group:", error);
                showToastMessage("Failed to join group", true);
            }
        }
    }

    async function copyJoinCode(joinCode: string) {
        try {
            await navigator.clipboard.writeText(joinCode);
            showToastMessage("Join code copied to clipboard!");
        } catch (error) {
            console.error("Failed to copy join code:", error);
            showToastMessage("Failed to copy join code", true);
        }
    }

    function showToastMessage(message: string, isError = false) {
        toastMessage = message;
        isErrorToast = isError;
        showToast = true;

        // Auto-hide after 3 seconds
        setTimeout(() => {
            showToast = false;
        }, 3000);
    }

    // Member management functions
    async function addMember() {
        if (newMemberPublicKey.trim()) {
            try {
                // This would call your backend to add a member to the group
                // For now, just close the dialog and show success
                showAddMember = false;
                newMemberPublicKey = "";
                showToastMessage("Member added successfully!");
                await loadParticipants(); // Reload participants after adding
            } catch (error) {
                console.error("Failed to add member:", error);
                showToastMessage("Failed to add member", true);
            }
        }
    }

    // Chat functions
    function openChat(group: Group) {
        selectedGroup = group;
        currentView = "chat";
        loadMessages();
        loadParticipants();
    }

    function goBackToGroups() {
        currentView = "groups";
        selectedGroup = null;
        messages = [];
    }

    async function loadMessages() {
        if (!selectedGroup) return;

        try {
            // This would call your backend to get messages for the group
            // For now, using mock data
            messages = [
                {
                    id: "1",
                    content: "Hello everyone!",
                    author: new Uint8Array(32).fill(1),
                    timestamp: Date.now() - 10000,
                },
                {
                    id: "2",
                    content: "Hey there! How's everyone doing?",
                    author: new Uint8Array(32).fill(2),
                    timestamp: Date.now() - 5000,
                },
            ];
        } catch (error) {
            console.error("Failed to load messages:", error);
        }
    }

    async function loadParticipants() {
        if (!selectedGroup) return;

        try {
            // This would call your backend to get participants
            // For now, using mock data
            const mockParticipants: Participant[] = [
                {
                    publicKey: new Uint8Array(32).fill(1),
                    name: "Alice",
                    avatar: "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHZpZXdCb3g9IjAgMCA0MCA0MCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMjAiIGZpbGw9IiM2MzY2RjEiLz4KPHN2ZyB4PSI4IiB5PSI4IiB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDEyQzE0LjIwOTEgMTIgMTYgMTAuMjA5MSAxNiA4QzE2IDUuNzkwODYgMTQuMjA5MSA0IDEyIDRDOS43OTA4NiA0IDggNS43OTA4NiA4IDhDOCAxMC4yMDkxIDkuNzkwODYgMTIgMTIgMTJaIiBmaWxsPSJ3aGl0ZSIvPgo8cGF0aCBkPSJNMTIgMTRDOC42ODYyOSAxNCA2IDE2LjY4NjMgNiAyMEgxOEMxOCAxNi42ODYzIDE1LjMxMzcgMTQgMTIgMTRaIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4KPC9zdmc+",
                },
                {
                    publicKey: new Uint8Array(32).fill(2),
                    name: "Bob",
                    avatar: "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHZpZXdCb3g9IjAgMCA0MCA0MCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMjAiIGZpbGw9IiNFRjQ0NDQiLz4KPHN2ZyB4PSI4IiB5PSI4IiB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDEyQzE0LjIwOTEgMTIgMTYgMTAuMjA5MSAxNiA4QzE2IDUuNzkwODYgMTQuMjA5MSA0IDEyIDRDOS43OTA4NiA0IDggNS43OTA4NiA4IDhDOCAxMC4yMDkxIDkuNzkwODYgMTIgMTIgMTJaIiBmaWxsPSJ3aGl0ZSIvPgo8cGF0aCBkPSJNMTIgMTRDOC42ODYyOSAxNCA2IDE2LjY4NjMgNiAyMEgxOEMxOCAxNi42ODYzIDE1LjMxMzcgMTQgMTIgMTRaIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4KPC9zdmc+",
                },
            ];

            participants.clear();
            mockParticipants.forEach((participant) => {
                const key = Array.from(participant.publicKey).join(",");
                participants.set(key, participant);
            });
        } catch (error) {
            console.error("Failed to load participants:", error);
        }
    }

    async function sendMessage() {
        if (newMessage.trim() && selectedGroup) {
            try {
                // This would call your backend to send a message
                // For now, just add to local messages
                const message: ChatMessage = {
                    id: Date.now().toString(),
                    content: newMessage.trim(),
                    author: new Uint8Array(32).fill(0), // Current user's key
                    timestamp: Date.now(),
                };
                messages = [...messages, message];
                newMessage = "";
            } catch (error) {
                console.error("Failed to send message:", error);
            }
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter" && !event.shiftKey) {
            event.preventDefault();
            sendMessage();
        }
    }

    function getParticipant(publicKey: Uint8Array): Participant | null {
        const key = Array.from(publicKey).join(",");
        return participants.get(key) || null;
    }

    function isMyMessage(publicKey: Uint8Array): boolean {
        // This would check against the current user's public key
        // For now, assuming the first message is from current user
        return publicKey.every((byte) => byte === 0);
    }

    function formatTimestamp(timestamp: number): string {
        return new Date(timestamp).toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
        });
    }

    onMount(async () => {
        console.log("onMount");
        await loadGroups();
    });
</script>

<main class="app">
    {#if currentView === "groups"}
        <div class="groups-view">
            <header class="groups-header">
                <h1>Groups</h1>
                <div class="group-actions">
                    <button
                        class="btn btn-primary"
                        on:click={() => {
                            console.log("create group opened");
                            showCreateGroup = true;
                        }}
                    >
                        Create Group
                    </button>
                    <button
                        class="btn btn-secondary"
                        on:click={() => (showJoinGroup = true)}
                    >
                        Join Group
                    </button>
                </div>
            </header>

            <div class="groups-list">
                {#if groups.length === 0}
                    <div class="empty-state">
                        <p>
                            No groups yet. Create one or join an existing group!
                        </p>
                    </div>
                {:else}
                    {#each groups as group (group.id)}
                        <div
                            class="group-card"
                            role="button"
                            tabindex="0"
                            on:click={() => openChat(group)}
                            on:keydown={(e) =>
                                e.key === "Enter" && openChat(group)}
                        >
                            <div class="group-info">
                                <h3 class="group-name">{group.name}</h3>
                                <p class="group-members">
                                    {group.memberCount} member{group.memberCount !==
                                    1
                                        ? "s"
                                        : ""}
                                </p>
                            </div>
                            <button
                                class="btn btn-small btn-outline"
                                on:click|stopPropagation={() =>
                                    copyJoinCode(group.joinCode)}
                            >
                                Copy Join Code
                            </button>
                        </div>
                    {/each}
                {/if}
            </div>
        </div>
    {:else if currentView === "chat" && selectedGroup}
        <div class="chat-view">
            <header class="chat-header">
                <div class="chat-header-left">
                    <button class="back-btn" on:click={goBackToGroups}>←</button
                    >
                    <div class="chat-info">
                        <h2>{selectedGroup.name}</h2>
                        <p>
                            {selectedGroup.memberCount} member{selectedGroup.memberCount !==
                            1
                                ? "s"
                                : ""}
                        </p>
                    </div>
                </div>
                <button
                    class="btn btn-small btn-outline"
                    on:click={() => (showAddMember = true)}
                >
                    Add Member
                </button>
            </header>

            <div class="messages-container">
                {#if messages.length === 0}
                    <div class="empty-chat">
                        <p>No messages yet. Start the conversation!</p>
                    </div>
                {:else}
                    {#each messages as message (message.id)}
                        {@const participant = getParticipant(message.author)}
                        {@const isMine = isMyMessage(message.author)}
                        <div
                            class="message {isMine
                                ? 'message-mine'
                                : 'message-other'}"
                        >
                            {#if !isMine && participant}
                                <img
                                    src={participant.avatar}
                                    alt={participant.name}
                                    class="message-avatar"
                                />
                            {/if}
                            <div class="message-content">
                                {#if !isMine && participant}
                                    <div class="message-author">
                                        {participant.name}
                                    </div>
                                {/if}
                                <div class="message-bubble">
                                    {message.content}
                                </div>
                                <div class="message-time">
                                    {formatTimestamp(message.timestamp)}
                                </div>
                            </div>
                        </div>
                    {/each}
                {/if}
            </div>

            <div class="message-input-container">
                <form on:submit|preventDefault={sendMessage}>
                    <input
                        bind:value={newMessage}
                        placeholder="Type a message..."
                        on:keydown={handleKeydown}
                        class="message-input"
                    />
                    <button
                        type="submit"
                        class="send-btn"
                        disabled={!newMessage.trim()}
                    >
                        Send
                    </button>
                </form>
            </div>
        </div>
    {/if}

    <!-- Create Group Modal -->
    {#if showCreateGroup}
        <div
            class="modal-overlay"
            role="button"
            tabindex="0"
            on:click={() => (showCreateGroup = false)}
            on:keydown={(e) => e.key === "Escape" && (showCreateGroup = false)}
        >
            <div
                class="modal"
                role="dialog"
                tabindex="0"
                on:click|stopPropagation
                on:keydown|stopPropagation
            >
                <h3>Create New Group</h3>
                <input
                    bind:value={newGroupName}
                    placeholder="Group name"
                    class="modal-input"
                />
                <div class="modal-actions">
                    <button
                        class="btn btn-secondary"
                        on:click={() => (showCreateGroup = false)}
                    >
                        Cancel
                    </button>
                    <button class="btn btn-primary" on:click={createGroup}>
                        Create
                    </button>
                </div>
            </div>
        </div>
    {/if}

    <!-- Join Group Modal -->
    {#if showJoinGroup}
        <div
            class="modal-overlay"
            role="button"
            tabindex="0"
            on:click={() => (showJoinGroup = false)}
            on:keydown={(e) => e.key === "Escape" && (showJoinGroup = false)}
        >
            <div
                class="modal"
                role="dialog"
                tabindex="0"
                on:click|stopPropagation
                on:keydown|stopPropagation
            >
                <h3>Join Group</h3>
                <input
                    bind:value={joinCode}
                    placeholder="Paste join code here"
                    class="modal-input"
                />
                <div class="modal-actions">
                    <button
                        class="btn btn-secondary"
                        on:click={() => (showJoinGroup = false)}
                    >
                        Cancel
                    </button>
                    <button class="btn btn-primary" on:click={joinGroup}>
                        Join
                    </button>
                </div>
            </div>
        </div>
    {/if}

    <!-- Add Member Modal -->
    {#if showAddMember}
        <div
            class="modal-overlay"
            role="button"
            tabindex="0"
            on:click={() => (showAddMember = false)}
            on:keydown={(e) => e.key === "Escape" && (showAddMember = false)}
        >
            <div
                class="modal"
                role="dialog"
                tabindex="0"
                on:click|stopPropagation
                on:keydown|stopPropagation
            >
                <h3>Add Member to Group</h3>
                <p class="modal-description">
                    Enter the public key of the person you want to add to this
                    group.
                </p>
                <textarea
                    bind:value={newMemberPublicKey}
                    placeholder="Paste the public key here (long string)..."
                    class="modal-textarea"
                    rows="4"
                ></textarea>
                <div class="modal-actions">
                    <button
                        class="btn btn-secondary"
                        on:click={() => (showAddMember = false)}
                    >
                        Cancel
                    </button>
                    <button class="btn btn-primary" on:click={addMember}>
                        Add Member
                    </button>
                </div>
            </div>
        </div>
    {/if}

    <!-- Toast Notification -->
    {#if showToast}
        <div
            class="toast {isErrorToast ? 'error' : ''} {showToast
                ? 'toast-show'
                : ''}"
        >
            <div class="toast-content">
                <div class="toast-icon">{isErrorToast ? "✕" : "✓"}</div>
                <div class="toast-message">{toastMessage}</div>
            </div>
        </div>
    {/if}
</main>

<style>
    .app {
        height: 100vh;
        display: flex;
        flex-direction: column;
        background: var(--bg-color);
        color: var(--text-color);
    }

    /* Groups View */
    .groups-view {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    .groups-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem 2rem;
        border-bottom: 1px solid var(--border-color);
        background: var(--bg-color);
    }

    .groups-header h1 {
        margin: 0;
        font-size: 1.5rem;
    }

    .group-actions {
        display: flex;
        gap: 0.5rem;
    }

    .groups-list {
        flex: 1;
        padding: 1rem 2rem;
        overflow-y: auto;
    }

    .group-card {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        margin-bottom: 0.5rem;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .group-card:hover {
        border-color: #646cff;
        box-shadow: 0 2px 8px rgba(100, 108, 255, 0.1);
    }

    .group-info {
        flex: 1;
    }

    .group-name {
        margin: 0 0 0.25rem 0;
        font-size: 1.1rem;
        font-weight: 600;
    }

    .group-members {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.9rem;
    }

    .empty-state {
        text-align: center;
        padding: 3rem 1rem;
        color: var(--text-muted);
    }

    /* Chat View */
    .chat-view {
        flex: 1;
        display: flex;
        flex-direction: column;
        height: 100vh;
    }

    .chat-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 1rem 2rem;
        border-bottom: 1px solid var(--border-color);
        background: var(--bg-color);
    }

    .chat-header-left {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .back-btn {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0.5rem;
        margin-right: 1rem;
        color: var(--text-color);
    }

    .back-btn:hover {
        background: var(--border-color);
        border-radius: 4px;
    }

    .chat-info h2 {
        margin: 0 0 0.25rem 0;
        font-size: 1.2rem;
    }

    .chat-info p {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.9rem;
    }

    .messages-container {
        flex: 1;
        padding: 1rem 2rem;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .empty-chat {
        text-align: center;
        padding: 3rem 1rem;
        color: var(--text-muted);
    }

    .message {
        display: flex;
        gap: 0.5rem;
        max-width: 70%;
    }

    .message-mine {
        align-self: flex-end;
        flex-direction: row-reverse;
    }

    .message-other {
        align-self: flex-start;
    }

    .message-avatar {
        width: 32px;
        height: 32px;
        border-radius: 50%;
        flex-shrink: 0;
    }

    .message-content {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .message-mine .message-content {
        align-items: flex-end;
    }

    .message-other .message-content {
        align-items: flex-start;
    }

    .message-author {
        font-size: 0.8rem;
        color: var(--text-muted);
        margin-bottom: 0.25rem;
    }

    .message-bubble {
        padding: 0.75rem 1rem;
        border-radius: 18px;
        word-wrap: break-word;
    }

    .message-mine .message-bubble {
        background: #646cff;
        color: white;
        border-bottom-right-radius: 4px;
    }

    .message-other .message-bubble {
        background: var(--border-color);
        color: var(--text-color);
        border-bottom-left-radius: 4px;
    }

    .message-time {
        font-size: 0.7rem;
        color: var(--text-muted);
        margin-top: 0.25rem;
    }

    .message-input-container {
        padding: 1rem 2rem;
        border-top: 1px solid var(--border-color);
        background: var(--bg-color);
    }

    .message-input-container form {
        display: flex;
        gap: 0.5rem;
    }

    .message-input {
        flex: 1;
        padding: 0.75rem 1rem;
        border: 1px solid var(--border-color);
        border-radius: 20px;
        outline: none;
        font-size: 1rem;
    }

    .message-input:focus {
        border-color: #646cff;
    }

    .send-btn {
        padding: 0.75rem 1.5rem;
        background: #646cff;
        color: white;
        border: none;
        border-radius: 20px;
        cursor: pointer;
        font-weight: 500;
    }

    .send-btn:disabled {
        background: var(--text-muted);
        cursor: not-allowed;
    }

    .send-btn:not(:disabled):hover {
        background: #535bf2;
    }

    /* Buttons */
    .btn {
        padding: 0.5rem 1rem;
        border: 1px solid var(--border-color);
        border-radius: 6px;
        background: var(--bg-color);
        color: var(--text-color);
        cursor: pointer;
        font-size: 0.9rem;
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .btn:hover {
        border-color: #646cff;
    }

    .btn-primary {
        background: #646cff;
        color: white;
        border-color: #646cff;
    }

    .btn-primary:hover {
        background: #535bf2;
        border-color: #535bf2;
    }

    .btn-secondary {
        background: transparent;
    }

    .btn-outline {
        background: transparent;
        border-color: #646cff;
        color: #646cff;
    }

    .btn-outline:hover {
        background: #646cff;
        color: white;
    }

    .btn-small {
        padding: 0.25rem 0.75rem;
        font-size: 0.8rem;
    }

    /* Modal */
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .modal {
        background: var(--bg-color);
        padding: 2rem;
        border-radius: 8px;
        min-width: 400px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    }

    .modal h3 {
        margin: 0 0 1rem 0;
        font-size: 1.2rem;
    }

    .modal-input {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid var(--border-color);
        border-radius: 6px;
        margin-bottom: 1rem;
        font-size: 1rem;
    }

    .modal-description {
        color: var(--text-muted);
        font-size: 0.9rem;
        margin-bottom: 1rem;
        line-height: 1.4;
    }

    .modal-textarea {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid var(--border-color);
        border-radius: 6px;
        margin-bottom: 1rem;
        font-size: 0.9rem;
        font-family: monospace;
        resize: vertical;
        min-height: 100px;
    }

    .modal-textarea:focus {
        border-color: #646cff;
        outline: none;
    }

    .modal-actions {
        display: flex;
        gap: 0.5rem;
        justify-content: flex-end;
    }

    /* Toast Notification */
    .toast {
        position: fixed;
        bottom: 20px;
        right: 20px;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        z-index: 2000;
        transform: translateX(100%);
        opacity: 0;
        transition: all 0.3s ease;
        min-width: 300px;
        max-width: 400px;
    }

    .toast-show {
        transform: translateX(0);
        opacity: 1;
    }

    .toast-content {
        display: flex;
        align-items: center;
        padding: 1rem;
        gap: 0.75rem;
    }

    .toast-icon {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        background: #10b981;
        color: white;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 14px;
        font-weight: bold;
        flex-shrink: 0;
    }

    .toast-message {
        color: var(--text-color);
        font-size: 0.9rem;
        font-weight: 500;
        line-height: 1.4;
    }

    /* Error toast */
    .toast.error .toast-icon {
        background: #ef4444;
    }

    /* Animation keyframes */
    @keyframes slideInRight {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }

    @keyframes slideOutRight {
        from {
            transform: translateX(0);
            opacity: 1;
        }
        to {
            transform: translateX(100%);
            opacity: 0;
        }
    }
</style>
