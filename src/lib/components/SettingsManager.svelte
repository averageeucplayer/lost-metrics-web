<script lang="ts">
    import { getSettings, saveSettings } from '$lib/api';
    import { setMeterSettings, setSnifferSettings, setUpdaterSettings } from '$lib/store/settings';
    import type { MeterSettings, SnifferSettings, UpdaterSettings } from '$lib/types';
    import { onMount, type Snippet } from 'svelte';
    import { writable } from 'svelte/store';
    
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    const meterSettings = writable<MeterSettings>();
    const updaterSettings = writable<UpdaterSettings>();
    const snifferSettings = writable<SnifferSettings>();

    onMount(() => {
		onLoad()
	});

    async function onLoad() {
        const result = await getSettings();

        snifferSettings.set(result.sniffer);
    }

    async function saveCombinedSettings() {
        // saveSettings({
        //     sniffer,
        //     meter,
        //     updater,
        //     version
        // });
    }

    setMeterSettings(meterSettings);
    setUpdaterSettings(updaterSettings);
    setSnifferSettings(snifferSettings);

</script>

{@render children?.()}