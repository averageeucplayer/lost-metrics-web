<script lang="ts">
    import type { Snippet } from "svelte";
    import "../app.css";
    import AppStateProvider from "$lib/components/AppStateProvider.svelte";
    import UpdaterManager from "$lib/components/UpdaterManager.svelte";
    import EncounterMonitor from "$lib/components/EncounterMonitor.svelte";
    import ProcessWatcher from "$lib/components/ProcessWatcher.svelte";
    import SettingsManager from "$lib/components/SettingsManager.svelte";
    import SideBar from "$lib/components/SideBar.svelte";
    
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

</script>

<AppStateProvider>
    <SettingsManager>
        <ProcessWatcher>
            <UpdaterManager>
                <EncounterMonitor>
                    <SideBar>
                        {@render children?.()}
                    </SideBar>
                </EncounterMonitor>
            </UpdaterManager>
        </ProcessWatcher>
    </SettingsManager>
</AppStateProvider>