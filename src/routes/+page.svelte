<script lang="ts">
   
   import { echarts } from '$lib/echarts';
    import { ProgressRing } from '@skeletonlabs/skeleton-svelte';
    import type { EChartsOption } from 'echarts';

    var option: EChartsOption = {
        backgroundColor: '#000',
  title: {
    text: 'Class popularity',
    textStyle: {
        color: "#BBB",
          fontFamily: "Lato",
          fontWeight: 300
    },
    top: 40,
    left: 'center'
  },
  tooltip: {
    trigger: 'item'
  },
  series: [
    {
      name: 'Class poulatrity',
      type: 'pie',
      radius: '50%',
      roseType: 'radius',
      data: [
        { value: 50154, name: 'Souleater' },
        { value: 47719, name: 'Paladin' },
        { value: 44996, name: 'Artist' },
        { value: 40533, name: 'Slayer' },
        { value: 39936, name: 'Sorceress' },
      ].sort(function (a, b) {
        return a.value - b.value;
      }),
      itemStyle: {
        color: '#999999',
        shadowBlur: 200,
        shadowColor: 'rgba(0, 0, 0, 0.5)'
      },
      animationType: 'scale',
      animationEasing: 'elasticOut',
      animationDelay: function (idx) {
        return Math.random() * 200;
      }
    }
  ]
};

var option1: EChartsOption = {
    backgroundColor: '#000',
  title: {
    text: 'Item Level Breakdown',
    top: 40,
     left: 'center',
     textStyle: {
        color: "#BBB",
        fontFamily: "Lato",
        fontWeight: 300
    },
  },
  tooltip: {
    trigger: 'axis'
  },
  xAxis: {
    type: 'category',
    data: ['1700+', '1690+', '1680+', '1670+', '1660+']
  },
  yAxis: {
    type: 'value',
  },
  grid: {
    top: "25%"
  },
  series: [
    {
      name: 'Samples',
      type: 'bar',
      data: [15544, 21466, 51790, 28110, 80854],
      itemStyle: {
        color: '#999999'
      },
      showBackground: true,
      backgroundStyle: {
        color: '#111111'
      }
    }
  ]
};

const tableData = [
  { metric: 'Total characters', value: '656,186' },
  { metric: 'Total Rosters', value: '146,527' },
  { metric: 'Total Guilds', value: '33,795' },
  { metric: 'Average Roster Size', value: '4.48' }
];

</script>

<div class="p-2 flex flex-col items-center">
    <h6 class="text-3xl text-[#BBB] lato-bold">Overview of Arkesia</h6>
    <div class="flex">
        <div class="w-[500px] h-[400px]" use:echarts={option}></div>
        <div class="w-[500px] h-[400px]" use:echarts={option1}></div>
    </div>
    <div class="mt-2 grid grid-cols-[auto_1fr_auto] gap-4">
        <div class="flex flex-col items-center card p-4 preset-tonal-surface-500">
            <span class="text-[#BBB] lato-bold">Support percentage</span>
            <ProgressRing classes="mt-4" value={19.9} max={100} showLabel />
        </div>
        <div class="card p-4 preset-tonal-surface-500 w-[500px]">
            <div class="table-wrap">
                <table class="table caption-bottom">
                  <tbody class="[&>tr]:hover:preset-tonal-primary">
                    {#each tableData as row}
                        <tr>
                        <td>{row.metric}</td>
                        <td class="text-right">{row.value}</td>
                        </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
        </div>
    </div>

</div>