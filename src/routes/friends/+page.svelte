<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { friends, showToastMessage } from "../../lib/stores.js";
    import type { Friend } from "../../lib/types.js";

    // Local state
    let friendCode = $state("");
    let showAddFriend = $state(false);

    // Friend management functions
    async function loadFriends() {
        try {
            const friendKeys: string[] = await invoke("get_friends");
            const friendsList: Friend[] = friendKeys.map((key) => ({
                publicKey: key,
                addedAt: Date.now(), // We don't store timestamps in backend yet
            }));
            friends.set(friendsList);
        } catch (error) {
            console.error("Failed to load friends:", error);
        }
    }

    async function addFriend() {
        if (friendCode.trim()) {
            try {
                const publicKey: string = await invoke("add_friend", {
                    friendCode: friendCode,
                });

                console.log("added friend: ", publicKey);
                showAddFriend = false;
                friendCode = "";
                await loadFriends(); // Reload friends after adding
                showToastMessage("Friend added successfully!");
            } catch (error) {
                console.error("Failed to add friend:", error);
                showToastMessage("Failed to add friend", true);
            }
        }
    }

    async function removeFriend(publicKey: string) {
        try {
            await invoke("remove_friend", {
                publicKey: publicKey,
            });
            await loadFriends(); // Reload friends after removing
            showToastMessage("Friend removed successfully!");
        } catch (error) {
            console.error("Failed to remove friend:", error);
            showToastMessage("Failed to remove friend", true);
        }
    }

    onMount(async () => {
        await loadFriends();
    });
</script>

<div class="friends-view">
    <header class="friends-header">
        <h1>Friends</h1>
        <div class="friend-actions">
            <button
                class="btn btn-primary"
                on:click={() => (showAddFriend = true)}
            >
                Add Friend
            </button>
        </div>
    </header>

    <div class="friends-list">
        {#if $friends.length === 0}
            <div class="empty-state">
                <p>No friends yet. Add a friend using their FriendCode!</p>
            </div>
        {:else}
            {#each $friends as friend (friend.publicKey)}
                <div class="friend-card">
                    <div class="friend-info">
                        <h3 class="friend-key">{friend.publicKey}</h3>
                        <p class="friend-added">
                            Added {new Date(
                                friend.addedAt,
                            ).toLocaleDateString()}
                        </p>
                    </div>
                    <button
                        class="btn btn-small btn-outline"
                        on:click={() => removeFriend(friend.publicKey)}
                    >
                        Remove
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>

<!-- Add Friend Modal -->
{#if showAddFriend}
    <div
        class="modal-overlay"
        role="button"
        tabindex="0"
        on:click={() => (showAddFriend = false)}
        on:keydown={(e) => e.key === "Escape" && (showAddFriend = false)}
    >
        <div
            class="modal"
            role="dialog"
            tabindex="0"
            on:click|stopPropagation
            on:keydown|stopPropagation
        >
            <h3>Add Friend</h3>
            <p class="modal-description">
                Enter the FriendCode of the person you want to add as a friend.
            </p>
            <textarea
                bind:value={friendCode}
                placeholder="Paste the friend code here (long hex string)..."
                class="modal-textarea"
                rows="4"
            ></textarea>
            <div class="modal-actions">
                <button
                    class="btn btn-secondary"
                    on:click={() => (showAddFriend = false)}
                >
                    Cancel
                </button>
                <button class="btn btn-primary" on:click={addFriend}>
                    Add Friend
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .friends-view {
        flex: 1;
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    .friends-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem 2rem;
        border-bottom: 1px solid var(--border-color);
        background: var(--bg-color);
    }

    .friends-header h1 {
        margin: 0;
        font-size: 1.5rem;
    }

    .friend-actions {
        display: flex;
        gap: 0.5rem;
    }

    .friends-list {
        flex: 1;
        padding: 1rem 2rem;
        overflow-y: auto;
    }

    .friend-card {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        margin-bottom: 0.5rem;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
    }

    .friend-info {
        flex: 1;
    }

    .friend-key {
        margin: 0 0 0.25rem 0;
        font-size: 0.9rem;
        font-weight: 600;
        font-family: monospace;
        word-break: break-all;
    }

    .friend-added {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.8rem;
    }

    .empty-state {
        text-align: center;
        padding: 3rem 1rem;
        color: var(--text-muted);
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
</style>
