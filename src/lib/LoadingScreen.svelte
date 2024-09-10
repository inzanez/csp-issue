<script lang="ts">
    import { fade } from 'svelte/transition';
</script>

<div in:fade class="loading-wrapper">
    Loading
</div>

<style>
    .loading-wrapper {
        background-color: rgb(250, 250, 250);
        justify-content: center;
        width: 100vw;
        height: 100vh;
    }
</style>