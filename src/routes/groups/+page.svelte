<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { groups, myPublicKey, showToastMessage } from "../../lib/stores.js";
    import type { Group } from "../../lib/types.js";

    // Local state
    let newGroupName = $state("");
    let joinCode = $state("");
    let showCreateGroup = $state(false);
    let showJoinGroup = $state(false);

    let groupsInterval: any;

    // Group management functions
    async function loadGroups() {
        try {
            for (const group of $groups) {
                try {
                    const members: string[] = await invoke("get_members", {
                        chatId: group.id,
                    });
                    group.memberCount = members.length;
                } catch (error) {
                    console.error("Failed to load members:", error);
                    group.memberCount = -1;
                }
            }
        } catch (error) {
            console.error("Failed to load groups:", error);
        }
    }

    async function createGroup() {
        const ok = true;
        if (ok) {
            try {
                const chatId: string = await invoke("create_group", {
                    name: newGroupName,
                });

                console.log("[ts] created group, chatId: {}", chatId);

                const newGroup: Group = {
                    id: chatId,
                    name: newGroupName,
                    memberCount: 1,
                };

                groups.update((current) => [...current, newGroup]);
                showCreateGroup = false;
                newGroupName = "";
                await loadGroups(); // Reload groups after creation
                showToastMessage("Group created successfully!");
            } catch (error) {
                console.error("Failed to create group:", error);
                showToastMessage("Failed to create group", true);
            }
        }
    }

    async function joinGroup() {
        if (joinCode.trim()) {
            try {
                await invoke("join_group", {
                    chatId: joinCode,
                });

                showJoinGroup = false;
                joinCode = "";
                await loadGroups(); // Reload groups after joining
                showToastMessage("Group joined successfully!");
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

    async function copyMyPublicKey() {
        try {
            if (!$myPublicKey) {
                throw new Error("Pubkey not set");
            }
            await navigator.clipboard.writeText($myPublicKey);
            showToastMessage("Public key copied to clipboard!");
        } catch (error) {
            console.error("Failed to copy public key:", error);
            showToastMessage("Failed to copy public key", true);
        }
    }

    function openChat(group: Group) {
        goto(`/chat/${group.id}`);
    }

    onMount(async () => {
        await loadGroups();

        // Set up interval for polling groups
        groupsInterval = setInterval(async () => {
            await loadGroups();
        }, 3000);
    });

    // Clean up interval on component destroy
    onDestroy(() => {
        clearInterval(groupsInterval);
    });
</script>

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
            <button class="btn btn-outline" on:click={copyMyPublicKey}>
                Copy Pubkey
            </button>
        </div>
    </header>

    <div class="groups-list">
        {#if $groups.length === 0}
            <div class="empty-state">
                <p>No groups yet. Create one or join an existing group!</p>
            </div>
        {:else}
            {#each $groups as group (group.id)}
                <div
                    class="group-card"
                    role="button"
                    tabindex="0"
                    on:click={() => openChat(group)}
                    on:keydown={(e) => e.key === "Enter" && openChat(group)}
                >
                    <div class="group-info">
                        <h3 class="group-name">{group.name}</h3>
                        <p class="group-members">
                            {group.memberCount} member{group.memberCount !== 1
                                ? "s"
                                : ""}
                        </p>
                    </div>
                    <button
                        class="btn btn-small btn-outline"
                        on:click|stopPropagation={() => copyJoinCode(group.id)}
                    >
                        Copy Join Code
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>

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

<style>
    .groups-view {
        flex: 1;
        display: flex;
        flex-direction: column;
        height: 100%;
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

    .modal-actions {
        display: flex;
        gap: 0.5rem;
        justify-content: flex-end;
    }
</style>
