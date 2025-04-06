<script lang="ts">
    import type { Snippet } from "svelte";
    import "../app.css";
    import AppStateProvider from "$lib/components/AppStateProvider.svelte";
    import Providers from "$lib/components/Providers.svelte";
    import SideBar from "$lib/components/SideBar.svelte";
    
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

</script>

<AppStateProvider>
    <Providers>
        <SideBar>
            {@render children?.()}
        </SideBar>
    </Providers>
</AppStateProvider>