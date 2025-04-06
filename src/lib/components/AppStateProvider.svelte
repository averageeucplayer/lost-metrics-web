<script lang="ts">
    import { load } from '$lib/api';
    import { setAppState } from '$lib/store/app';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';
    import type { AppState } from '$lib/types';

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();
    const appState = writable<AppState>({ isLoading: true, version: "", appName: "" });

    onMount(() => {
		onLoad()
	});

    async function onLoad() {
        const result = await load();

        appState.update(pr => {
            pr.isLoading = false;
            pr.version = result.version;
            return pr;
        })
    }

    setAppState(appState);
</script>

{@render children?.()}