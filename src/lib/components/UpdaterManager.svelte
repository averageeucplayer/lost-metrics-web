<script lang="ts">
    import { checkForUpdate, installUpdate, onUpdateCheck } from '$lib/api';
    import { setUpdateManager } from '$lib/store/managers';
    import type { UpdateCheckResult } from '$lib/types';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    let handle: (() => void) | null = null;

    const checkResult = writable<UpdateCheckResult | undefined>();

    onMount(() => {
        registerUpdateCheck();

        return () => {
            handle && handle();
        };
    })

    async function registerUpdateCheck() {
        handle = await onUpdateCheck((result) => {
            checkResult.set(result);
        });
    }

    setUpdateManager({
        result: checkResult,
        checkForUpdate: checkForUpdate,
        install: installUpdate
    });

</script>

{@render children?.()}