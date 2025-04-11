<script lang="ts">
    import { useAppState } from '$lib/store/app';
    import { Navigation, ProgressRing } from '@skeletonlabs/skeleton-svelte';
    import { IconAdjustmentsAlt, IconBug, IconDeviceDesktopAnalytics, IconDeviceGamepad, IconGauge, IconHistory, IconHome, IconInfoCircle } from '@tabler/icons-svelte';
    import type { Snippet } from 'svelte';
	import { page } from "$app/state";

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    const appState = useAppState();

	console.log(page.url.pathname);

</script>


{#if $appState.isLoading}
	<div class="size-full">
		<div class="size-full flex justify-center items-center">
			<div>
				<ProgressRing value={null} size="size-14" meterStroke="stroke-tertiary-600-400" trackStroke="stroke-tertiary-50-950" />
			</div>
		</div>
	</div>
{:else}
	<div class="card border-surface-100-900 grid h-screen w-full grid-cols-[auto_1fr] border-[1px]">
		<Navigation.Rail value={page.url.pathname}>
			{#snippet tiles()}
			<Navigation.Tile href="/info" id="/info" label="Info"><IconInfoCircle /></Navigation.Tile>
			<Navigation.Tile href="/" id="/" label="Stats"><IconDeviceDesktopAnalytics /></Navigation.Tile>
			<Navigation.Tile href="/meter" id="/meter" label="Meter"><IconGauge /></Navigation.Tile>
			<Navigation.Tile href="/history" id="/history" label="History"><IconHistory /></Navigation.Tile>
			<Navigation.Tile href="/simulator" id="/simulator" label="Simulator"><IconDeviceGamepad /></Navigation.Tile>
			<Navigation.Tile href="/settings" id="/settings" label="Settings"><IconAdjustmentsAlt /></Navigation.Tile>
			<Navigation.Tile href="/debug" id="/debug" label="Debug"><IconBug /></Navigation.Tile>
			{/snippet}
		</Navigation.Rail>
	
		<div class="">
			{@render children?.()}
		</div>
	</div>
{/if}