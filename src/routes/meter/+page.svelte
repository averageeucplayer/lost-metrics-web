<script lang="ts">
    import { useEncounterMonitor } from "$lib/store/encounter";
    import NotRunning from "./NotRunning.svelte";
    import { useProcessWatcher } from "$lib/store/managers";
    const monitor = useEncounterMonitor();
    const processWatcher = useProcessWatcher();


</script>

<div class="h-full flex justify-center items-center">
    {#if false }
    {:else}
        <NotRunning/>
    {/if}
    
    <!-- {#each $monitor as row}
        
    {/each} -->
</div>