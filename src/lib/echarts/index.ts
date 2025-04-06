import type { Metric } from "$lib/types";
import type { EChartsOption } from "echarts";
import * as charts from "echarts";

export function echarts(node: HTMLElement, option: EChartsOption | undefined) {
	
	const chart = charts.init(node,  "dark", {
		renderer: "svg"
	});

	if(!option) {
		chart.showLoading();
	}

	return {
		update(option: EChartsOption) {
			chart.hideLoading();
			chart.setOption(option!);
		},
		destroy() {
			
		}
	}
}

export * from "./getItemLevelbreakdown";
export * from "./getServerPopulation";
export * from "./getClassPopularity";