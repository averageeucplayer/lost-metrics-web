<script lang="ts">
    import { onProcessUpdate } from '$lib/api';
    import { setProcessWatcher } from '$lib/store/managers';
    import type { ProcessWatcherResult } from '$lib/types';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    let handle: (() => void) | null = null;

    const checkResult = writable<ProcessWatcherResult | undefined>();

    onMount(() => {
        registerProcessUpdate();

        return () => {
            handle && handle();
        };
    })

    async function registerProcessUpdate() {
        handle = await onProcessUpdate((result) => {
            checkResult.set(result);
        });
    }

    setProcessWatcher({
        result: checkResult,
    });

</script>

{@render children?.()}