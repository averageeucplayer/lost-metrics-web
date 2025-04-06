import type { Metric } from "$lib/types";
import type { EChartsOption } from "echarts";

export function getItemLevelbreakdown(data: Metric[]): EChartsOption | undefined {

	if(!data.length) {
		return;
	}

	const { xAxisData, seriesData } = data.reduce(
		(acc, item) => {
			acc.xAxisData.push(item.name);
			acc.seriesData.push(item.value);
			return acc;
		},
		{ 
			xAxisData: new Array<string>(),
			seriesData: new Array<number>()
		}
	);

	const option: EChartsOption = {
		backgroundColor: "#000",
		title: {
			text: "Item Level Breakdown",
			top: 40,
			left: "center",
			textStyle: {
				color: "#BBB",
				fontFamily: "Lato",
				fontWeight: 300
			},
		},
		tooltip: {
			trigger: "axis"
		  },
		xAxis: {
			type: "category",
			data: xAxisData
		},
		yAxis: {
			type: "value",
			axisLabel: {
				color: "#BBB",
				fontFamily: "Lato",
				formatter: function (value: number) {
					if (value >= 1_000_000) return `${value / 1_000_000}M`;
					if (value >= 1_000) return `${value / 1_000}k`;
					return value.toString();
				}
			},
		},
		grid: {
			top: "25%"
		},
		series: [
			{
				name: "Samples",
				type: "bar",
				data: seriesData,
				itemStyle: {
					color: "#999999"
				},
				showBackground: true,
				backgroundStyle: {
					color: "#111111"
				}
			}
		]
	};

	return option;
}
