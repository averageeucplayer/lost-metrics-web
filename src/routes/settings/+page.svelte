<script lang="ts">
    import { Tabs } from "@skeletonlabs/skeleton-svelte";
    import { IconCar, IconDatabase, IconGauge, IconNetwork, IconRefresh } from "@tabler/icons-svelte";
    import Sniffer from "./sniffer.svelte";

    interface State {
        currentTab: string;
        hasChanged: boolean;
    }

    let state = $state<State>({
        currentTab: "sniffer",
        hasChanged: false
    });

    function onTabSelect(event: any) {
        console.log(event);
        state.currentTab = event.value;
    }

</script>

<div class="p-2 flex flex-col items-center">
    <Tabs value={state.currentTab} onValueChange={onTabSelect} fluid>
        {#snippet list()}
            <Tabs.Control value="sniffer">
            {#snippet lead()}<IconNetwork size={20} />{/snippet}
            Sniffer
            </Tabs.Control>
            <Tabs.Control value="meter">
            {#snippet lead()}<IconGauge size={20} />{/snippet}
            Meter
            </Tabs.Control>
            <Tabs.Control value="updater">
                {#snippet lead()}<IconRefresh size={20} />{/snippet}
                Updater
                </Tabs.Control>
            <Tabs.Control value="database">
            {#snippet lead()}<IconDatabase size={20} />{/snippet}
            Store
            </Tabs.Control>
        {/snippet}
        {#snippet content()}
            <Tabs.Panel value="sniffer"><Sniffer/></Tabs.Panel>
            <Tabs.Panel value="meter"><Sniffer/></Tabs.Panel>
            <Tabs.Panel value="updater"><Sniffer/></Tabs.Panel>
            <Tabs.Panel value="IconDatabase"><Sniffer/></Tabs.Panel>
        {/snippet}
    </Tabs>
    <button type="button" class="btn preset-filled-primary-500" disabled={state.hasChanged}>Save</button>
</div>