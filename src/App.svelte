<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import "./styles.css";

    let message = "";
    let messages: string[] = [];

    async function sendMessage() {
        if (message.trim()) {
            await invoke("send_message", {
                message: message.trim(),
            });
            message = "";
        }
    }

    async function getMessages() {
        try {
            const result = await invoke("get_messages", {});
            messages = result as string[];
        } catch (error) {
            console.error("Failed to get messages:", error);
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            sendMessage();
        }
    }

    onMount(() => {
        // Get messages immediately and then every second
        getMessages();
        const interval = setInterval(getMessages, 1000);

        return () => clearInterval(interval);
    });
</script>

<main class="container">
    <h1>P2Panda Tauri Chat</h1>

    <div class="row">
        <a href="https://vite.dev" target="_blank">
            <img src="/src/assets/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
            <img
                src="/src/assets/tauri.svg"
                class="logo tauri"
                alt="Tauri logo"
            />
        </a>
        <a href="https://svelte.dev" target="_blank">
            <img
                src="/src/assets/svelte.svg"
                class="logo svelte"
                alt="Svelte logo"
            />
        </a>
    </div>

    <p>Click on the logos to learn more about the frameworks</p>

    <form class="row" on:submit|preventDefault={sendMessage}>
        <input
            bind:value={message}
            placeholder="Enter a message..."
            on:keydown={handleKeydown}
        />
        <button type="submit">Send</button>
    </form>

    <div class="messages">
        <h3>Messages:</h3>
        {#if messages.length === 0}
            <p class="no-messages">No messages yet. Send one above!</p>
        {:else}
            <ul>
                {#each messages as msg, index (index)}
                    <li>{msg}</li>
                {/each}
            </ul>
        {/if}
    </div>
</main>

<style>
    .messages {
        margin-top: 2rem;
        text-align: left;
        max-width: 600px;
        margin-left: auto;
        margin-right: auto;
    }

    .messages ul {
        list-style: none;
        padding: 0;
        background: var(--bg-color);
        border-radius: 8px;
        padding: 1rem;
        max-height: 300px;
        overflow-y: auto;
    }

    .messages li {
        padding: 0.5rem 0;
        border-bottom: 1px solid var(--border-color);
    }

    .messages li:last-child {
        border-bottom: none;
    }

    .no-messages {
        color: var(--text-muted);
        font-style: italic;
    }

    .logo.svelte:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
    }
</style>
