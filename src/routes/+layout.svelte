<script lang="ts">
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import {
        myPublicKey,
        showToast,
        toastMessage,
        isErrorToast,
    } from "$lib/stores.js";
    import "../styles.css";

    // Load public key on mount
    onMount(async () => {
        console.log("Loading public key");
        try {
            const pubKey = await invoke("me");
            myPublicKey.set(pubKey);
            console.log("Public key loaded:", pubKey);
        } catch (error) {
            console.error("Failed to load public key:", error);
        }
    });

    function navigateTo(path: string) {
        goto(path);
    }

    function isActive(path: string): boolean {
        return $page.url.pathname === path;
    }
</script>

<main class="app">
    <!-- Navigation Bar -->
    <nav class="nav-bar">
        <div class="nav-content">
            <h1 class="nav-title">p2pandash chat</h1>
            <div class="nav-links">
                <button
                    class="nav-link {isActive('/groups') ? 'active' : ''}"
                    on:click={() => navigateTo("/groups")}
                >
                    Groups
                </button>
                <button
                    class="nav-link {isActive('/friends') ? 'active' : ''}"
                    on:click={() => navigateTo("/friends")}
                >
                    Friends
                </button>
            </div>
        </div>
    </nav>

    <!-- Main Content -->
    <div class="main-content">
        <slot />
    </div>
</main>

<!-- Toast Notification -->
{#if $showToast}
    <div
        class="toast {$isErrorToast ? 'error' : ''} {$showToast
            ? 'toast-show'
            : ''}"
    >
        <div class="toast-content">
            <div class="toast-icon">{$isErrorToast ? "✕" : "✓"}</div>
            <div class="toast-message">{$toastMessage}</div>
        </div>
    </div>
{/if}

<style>
    .app {
        height: 100vh;
        display: flex;
        flex-direction: column;
        background: var(--bg-color);
        color: var(--text-color);
        transition:
            background-color 0.3s ease,
            color 0.3s ease;
    }

    .nav-bar {
        background: var(--bg-color);
        border-bottom: 1px solid var(--border-color);
        padding: 0 2rem;
        transition:
            background-color 0.3s ease,
            border-color 0.3s ease;
    }

    .nav-content {
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 60px;
    }

    .nav-title {
        margin: 0;
        font-size: 1.5rem;
        font-weight: 600;
        color: var(--text-color);
    }

    .nav-links {
        display: flex;
        gap: 1rem;
    }

    .nav-link {
        padding: 0.5rem 1rem;
        background: none;
        border: 1px solid var(--border-color);
        border-radius: 6px;
        color: var(--text-color);
        cursor: pointer;
        font-size: 0.9rem;
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .nav-link:hover {
        border-color: var(--accent-color);
        background: rgba(100, 108, 255, 0.1);
    }

    .nav-link.active {
        background: var(--accent-color);
        color: white;
        border-color: var(--accent-color);
    }

    .main-content {
        flex: 1;
        overflow: hidden;
        background: var(--bg-color);
        color: var(--text-color);
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
</style>
