<script lang="ts">
    import { getStats } from '$lib/api';
	import { echarts, getClassPopularity, getItemLevelbreakdown, getServerPopulation } from '$lib/echarts/index';
    import type { Metric, ServerPopulation } from '$lib/types';
	import { ProgressRing } from '@skeletonlabs/skeleton-svelte';
    import { onMount } from 'svelte';

	interface State {
		metrics: Metric[];
		classPopularity: Metric[];
    	itemLevelBreakdown: Metric[];
        serverPopulation?: ServerPopulation;
	}

	let test = $state(getClassPopularity([]));
    let pageState = $state<State>({
		metrics: [],
		classPopularity: [],
		itemLevelBreakdown: [],
        serverPopulation: undefined
	});

	onMount(() => {
		onLoad()
	});

    async function onLoad() {

        try {
            const stats = await getStats();
            pageState = {
                ...stats
            };
        }
        catch(error) {
            console.log(error);
        }
    }

</script>

<div class="p-2 flex flex-col items-center">
    <h6 class="text-3xl text-[#BBB] lato-bold">Overview of Arkesia</h6>
    <div class="flex">
        <div class="w-[350px] h-[350px]" use:echarts={getClassPopularity(pageState.classPopularity)}></div>
        <div class="w-[350px] h-[350px]" use:echarts={getItemLevelbreakdown(pageState.itemLevelBreakdown)}></div>
    </div>
    <div class="flex items-center">
        <div class="w-[300px] h-[300px]" use:echarts={getServerPopulation(pageState.serverPopulation)}></div>
        <div class="card p-4 preset-tonal-surface-500 w-[400px]">
            <div class="table-wrap">
                <table class="table caption-bottom">
                  <tbody class="[&>tr]:hover:preset-tonal-primary">
                    {#each pageState.metrics as row}
                        <tr>
                        <td>{row.name}</td>
                        <td class="text-right">{row.value}</td>
                        </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
        </div>
    </div>
</div>