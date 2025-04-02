<script lang="ts">
    import { load, type LoadResult } from '$lib/api';
    import { setAppState, type AppState } from '$lib/store/app';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';
    
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

</script>

{@render children?.()}