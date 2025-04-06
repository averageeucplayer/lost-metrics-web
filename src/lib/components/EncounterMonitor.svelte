<script lang="ts">
    import { onEncounterUpdate } from '$lib/api';
    import { setEncounterMonitor } from '$lib/store/encounter';
    import type { EncounterMonitor } from '$lib/types';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';
    
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    let handle: (() => void) | null = null;

    const monitor = writable<EncounterMonitor>({});
    
    onMount(() => {
        registerEncounterUpdate();

        return () => {
            handle && handle();
        };
    })

    async function registerEncounterUpdate() {
        handle = await onEncounterUpdate((encounter) => {
            monitor.update(pr => {
                pr.encounter = encounter;
                return pr;
            })
        });
    }

    setEncounterMonitor(monitor);

</script>

{@render children?.()}