<script lang="ts">
    import { useEncounterMonitor } from "$lib/store/encounter";
    import NotRunning from "./NotRunning.svelte";
    import { useProcessWatcher } from "$lib/store/managers";
    import Meter from "./Meter.svelte";
    const monitor = useEncounterMonitor();
    const {
        result
    } = useProcessWatcher();

</script>

<div class="h-full">
    {#if !result }
        <div class="h-full flex justify-center items-center">
            <NotRunning/>
        </div>
    {:else if $monitor.encounter }
        <Meter encounter={$monitor.encounter}/>
    {:else if $result?.state.type == "ProcessNotRunning"}
        <div class="h-full flex justify-center items-center">
            <NotRunning/>
        </div>
    {/if}
</div>