<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount, createEventDispatcher } from "svelte";
    import { page } from "$app/stores";
    import {
        messages,
        participants,
        myPublicKey,
        showToastMessage,
        friends,
    } from "../../../lib/stores.js";
    import type { ChatMessage, Participant } from "../../../lib/types.js";

    // Get chatId from route parameters
    let chatId = $derived($page.params.chatId);

    const dispatch = createEventDispatcher();

    // Local state
    let newMessage = $state("");
    let showAddMember = $state(false);
    let selectedFriends = $state<Set<string>>(new Set());

    let membersInterval: any;

    // Chat functions
    async function loadMessages() {
        try {
            console.log("trying to get_messages for", chatId);
            let msgs: ChatMessage[] = await invoke("get_messages", {
                chatId: chatId,
            });

            msgs.sort((a, b) => a.timestamp - b.timestamp);
            messages.set(msgs);
        } catch (error) {
            console.error("Failed to load messages:", error);
        }
    }

    async function loadParticipants() {
        try {
            const members: string[] = await invoke("get_members", {
                chatId: chatId,
            });

            const participantsMap = new Map<string, Participant>();
            members.forEach((participant: string) => {
                participantsMap.set(participant, {
                    publicKey: participant,
                    name: participant,
                    avatar: "",
                });
            });
            participants.set(participantsMap);
        } catch (error) {
            console.error("Failed to load participants:", error);
        }
    }

    async function sendMessage() {
        console.log("sendMessage", newMessage, chatId);
        if (newMessage.trim()) {
            try {
                const message: ChatMessage = {
                    content: newMessage.trim(),
                    author: $myPublicKey, // Current user's key
                    timestamp: Date.now(),
                };

                await invoke("send_message", {
                    chatId: chatId,
                    message,
                });

                messages.update((current) => [...current, message]);
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

    function getParticipant(publicKey: string): Participant | null {
        return (
            $participants.get(publicKey) || {
                publicKey,
                name: publicKey,
                avatar: "",
            }
        );
    }

    function isMyMessage(publicKey: string): boolean {
        return publicKey === $myPublicKey;
    }

    function formatTimestamp(timestamp: number): string {
        return new Date(timestamp).toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
        });
    }

    // Add member functions (updated to use friends)
    function toggleFriendSelection(publicKey: string) {
        const newSelection = new Set(selectedFriends);
        if (newSelection.has(publicKey)) {
            newSelection.delete(publicKey);
        } else {
            newSelection.add(publicKey);
        }
        selectedFriends = newSelection;
    }

    async function addSelectedFriends() {
        if (selectedFriends.size === 0) return;

        try {
            for (const friendPublicKey of selectedFriends) {
                // Get the friend's member info
                const friend = $friends.find(
                    (f) => f.publicKey === friendPublicKey,
                );
                if (friend) {
                    await invoke("add_member", {
                        chatId: chatId,
                        pubkey: friendPublicKey,
                    });
                }
            }

            showAddMember = false;
            selectedFriends = new Set();
            showToastMessage("Members added successfully!");
            await loadParticipants(); // Reload participants after adding
        } catch (error) {
            console.error("Failed to add members:", error);
            showToastMessage("Failed to add members", true);
        }
    }

    onMount(async () => {
        await loadMessages();
        await loadParticipants();

        // Set up interval for polling members
        membersInterval = setInterval(async () => {
            await loadParticipants();
        }, 3000);
    });

    // Clean up interval on component destroy
    onDestroy(() => {
        clearInterval(membersInterval);
    });
</script>

<div class="chat-view">
    <header class="chat-header">
        <div class="chat-header-left">
            <button class="back-btn" on:click={() => dispatch("backToGroups")}
                >←</button
            >
            <div class="chat-info">
                <h2>Group Chat</h2>
                <p>
                    {Object.keys($participants).length} member{Object.keys(
                        $participants,
                    ).length !== 1
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
        {#if $messages.length === 0}
            <div class="empty-chat">
                <p>No messages yet. Start the conversation!</p>
            </div>
        {:else}
            {#each $messages as message (message.timestamp)}
                {@const participant = getParticipant(message.author)}
                {@const isMine = isMyMessage(message.author)}
                <div
                    class="message {isMine ? 'message-mine' : 'message-other'}"
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

<!-- Add Member Modal (Updated to show friends) -->
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
            <h3>Add Members to Group</h3>
            <p class="modal-description">
                Select friends to add to this group.
            </p>

            {#if $friends.length === 0}
                <div class="no-friends">
                    <p>
                        No friends available. <a href="/friends"
                            >Add some friends first</a
                        >.
                    </p>
                </div>
            {:else}
                <div class="friends-selection">
                    {#each $friends as friend (friend.publicKey)}
                        <label class="friend-option">
                            <input
                                type="checkbox"
                                checked={selectedFriends.has(friend.publicKey)}
                                on:change={() =>
                                    toggleFriendSelection(friend.publicKey)}
                            />
                            <span class="friend-key">{friend.publicKey}</span>
                        </label>
                    {/each}
                </div>
            {/if}

            <div class="modal-actions">
                <button
                    class="btn btn-secondary"
                    on:click={() => (showAddMember = false)}
                >
                    Cancel
                </button>
                <button
                    class="btn btn-primary"
                    on:click={addSelectedFriends}
                    disabled={selectedFriends.size === 0}
                >
                    Add {selectedFriends.size} Member{selectedFriends.size !== 1
                        ? "s"
                        : ""}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
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
        max-width: 500px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    }

    .modal h3 {
        margin: 0 0 1rem 0;
        font-size: 1.2rem;
    }

    .modal-description {
        color: var(--text-muted);
        font-size: 0.9rem;
        margin-bottom: 1rem;
        line-height: 1.4;
    }

    .no-friends {
        text-align: center;
        padding: 2rem;
        color: var(--text-muted);
    }

    .no-friends a {
        color: #646cff;
        text-decoration: none;
    }

    .no-friends a:hover {
        text-decoration: underline;
    }

    .friends-selection {
        max-height: 300px;
        overflow-y: auto;
        margin-bottom: 1rem;
    }

    .friend-option {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem;
        border-radius: 4px;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .friend-option:hover {
        background: var(--border-color);
    }

    .friend-option input[type="checkbox"] {
        margin: 0;
    }

    .friend-key {
        font-family: monospace;
        font-size: 0.8rem;
        word-break: break-all;
    }

    .modal-actions {
        display: flex;
        gap: 0.5rem;
        justify-content: flex-end;
    }
</style>
