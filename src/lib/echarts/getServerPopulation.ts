import type { ServerPopulation } from "$lib/types";
import type { EChartsOption } from "echarts";
import { grayShade } from "./utils";

export function getServerPopulation(data: ServerPopulation | undefined): EChartsOption | undefined {

	if(!data) {
		return;
	}

	const nawTotal = data.na.naw.reduce((sum, m) => sum + m.value, 0);
	const naeTotal = data.na.nae.reduce((sum, m) => sum + m.value, 0);
	const naTotal = [...data.na.naw, ...data.na.nae].reduce((sum, m) => sum + m.value, 0);
    const euTotal = data.eu.metrics.reduce((sum, m) => sum + m.value, 0);
	const total = naTotal + euTotal;

	const transformed = [
		{
			name: data.na.name,
			itemStyle: {
				color: grayShade(naTotal, total)
			},
			children: [
				{
					name: "NAW",
					children: data.na.naw.map((m) => ({
						name: m.name,
						value: m.value,
						itemStyle: {
							color: grayShade(m.value, naTotal)
						},
						label: {
							color: '#FFF',
							fontSize: 6
						}
					})),
					itemStyle: {
						color: grayShade(nawTotal, total)
					},
				},
				{
					name: "NAE",
					children: data.na.nae.map((m) => ({
						name: m.name,
						value: m.value,
						itemStyle: {
							color: grayShade(m.value, naTotal)
						},
						label: {
                            color: '#FFF',
                            fontSize: 6
                        }
					})),
					itemStyle: {
						color: grayShade(naeTotal, total)
					},
				}
			]
		},
		{
			name: data.eu.name,
			children: data.eu.metrics.map((m) => ({
				name: m.name,
				value: m.value,
				itemStyle: {
					color: grayShade(m.value, total)
				},
				label: {
					color: '#FFF',
					fontSize: 6
				}
			})),
			itemStyle: {
				color: grayShade(euTotal, total)
			},
		}
	];

	const option: EChartsOption = {
		backgroundColor: "#000",
		title: {
			text: "Server population",
			top: 0,
			left: "center",
			textStyle: {
				color: "#BBB",
				fontFamily: "Lato",
				fontWeight: 300
			},
		},
		tooltip: {
            trigger: 'item',
            formatter: '{b}: {c}',
            backgroundColor: 'rgba(50, 50, 50, 0.7)',
            borderColor: '#333',
            borderWidth: 1,
            padding: 10,
            textStyle: {
                color: '#FFF',
                fontFamily: 'Lato'
            }
        },
		series: [
			{
				type: "sunburst",
				radius: [0, '70%'],
				nodeClick: false,
				label: {
                    color: '#FFF',
                    formatter: '{b}'
                },
				data: transformed
			}
		]
	};
  
	return option;
}