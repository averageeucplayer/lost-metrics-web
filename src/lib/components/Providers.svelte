<script lang="ts">
    import UpdaterManager from "$lib/components/UpdaterManager.svelte";
    import EncounterMonitor from "$lib/components/EncounterMonitor.svelte";
    import ProcessWatcher from "$lib/components/ProcessWatcher.svelte";
    import SettingsManager from "$lib/components/SettingsManager.svelte";
    import { onMount, type Snippet } from "svelte";

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

</script>

<SettingsManager>
    <UpdaterManager>
        <EncounterMonitor>
            <ProcessWatcher>
                {@render children?.()}
            </ProcessWatcher>
        </EncounterMonitor>
    </UpdaterManager>
</SettingsManager>