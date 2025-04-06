import type { Metric } from "$lib/types";
import type { EChartsOption } from "echarts";
import { linearGrayShade } from "./utils";

export function getClassPopularity(data: Metric[]): EChartsOption | undefined {
    
    if(!data.length) {
        return;
    }

    const total = data.reduce((sum, m) => sum + m.value, 0);

    data = data.map(pr => ({
        ...pr,
        itemStyle: {
            color: linearGrayShade(pr.value, total),
            shadowBlur: 200,
            shadowColor: "rgba(0, 0, 0, 0.5)"
        },
    }))

    const option: EChartsOption = {
        backgroundColor: "#000",
        title: {
            text: "Class popularity",
            textStyle: {
                color: "#BBB",
                fontFamily: "Lato",
                fontWeight: 300
            },
            top: 40,
            left: "center"
        },
        tooltip: {
            trigger: "item"
        },
        series: [
            {
                name: "Class poulatrity",
                type: "pie",
                radius: "50%",
                roseType: "radius",
                data,
                animationType: "scale",
                animationEasing: "elasticOut",
                animationDelay: function (idx) {
                    return Math.random() * 200;
                }
            }
        ]
    };

    return option;
}
