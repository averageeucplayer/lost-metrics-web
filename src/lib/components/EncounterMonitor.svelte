<script lang="ts">
    import { onEncounterClear, onEncounterUpdate } from '$lib/api';
    import { setEncounterMonitor } from '$lib/store/encounter';
    import type { EncounterMonitor } from '$lib/types';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';
    
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    let handle: (() => void) | null = null;
    let intervalHandle: number | null = null;

    const monitor = writable<EncounterMonitor>({
        duration: {
            raw: 0,
            mmss: ""
        }
    });
    
    onMount(() => {
        registerEncounterEvents();

        return () => {
            intervalHandle && clearInterval(intervalHandle);
            handle && handle();
        };
    })

    function toMMSS(seconds: number) {
        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;
        const formattedMinutes = String(minutes).padStart(2, '0');
        const formattedSeconds = String(remainingSeconds).padStart(2, '0');
        return `${formattedMinutes}:${formattedSeconds}`;
    }

    async function registerEncounterEvents() {

        await onEncounterClear(() => {
            intervalHandle && clearInterval(intervalHandle);
        });

        handle = await onEncounterUpdate((encounter) => {
            monitor.update(pr => {
                pr.encounter = encounter;
                return pr;
            });

            if(!intervalHandle) {
                intervalHandle = setInterval(() => {
                    monitor.update(pr => {
                        const currentTime = Date.now(); 
                        const startedOn = +new Date(pr.encounter!.startedOn);
                        const elapsed = Math.floor((currentTime - startedOn) / 1000); 

                        pr.duration = {
                            mmss: toMMSS(elapsed),
                            raw: 0
                        };
                        return pr;
                    });
                }, 1000);
            }
        });
    }

    setEncounterMonitor(monitor);

</script>

{@render children?.()}