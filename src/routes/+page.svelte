<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    onMount(() => {
        // Redirect to groups page by default
        goto("/groups");
    });
</script>

<div class="loading">
    <p>Loading...</p>
</div>

<style>
    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
        font-size: 1.2rem;
        color: var(--text-muted);
    }
</style>
