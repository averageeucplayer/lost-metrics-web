<script lang="ts">
    import { Navigation } from '@skeletonlabs/skeleton-svelte';
    import { IconAdjustmentsAlt, IconDeviceDesktopAnalytics, IconDeviceGamepad, IconGauge, IconHistory, IconHome } from '@tabler/icons-svelte';
    import type { Snippet } from 'svelte';
     
    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

</script>

<div class="h-full card border-surface-100-900 grid w-full grid-cols-[auto_1fr] border-[1px]">

    <Navigation.Rail>
      {#snippet tiles()}
        <Navigation.Tile href="/" label="Stats"><IconDeviceDesktopAnalytics /></Navigation.Tile>
        <Navigation.Tile href="/meter" label="Meter"><IconGauge /></Navigation.Tile>
        <Navigation.Tile href="/history" label="History"><IconHistory /></Navigation.Tile>
        <Navigation.Tile href="/simulator" label="Simulator"><IconDeviceGamepad /></Navigation.Tile>
        <Navigation.Tile href="/settings" label="Settings"><IconAdjustmentsAlt /></Navigation.Tile>
      {/snippet}
    </Navigation.Rail>

    <div class="">
        {@render children?.()}
    </div>
</div>