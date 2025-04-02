import * as charts from 'echarts';

export function echarts(node: any, option: any) {
	const chart = charts.init(node,  'dark', {
        renderer: 'svg'
      });
	chart.setOption(option);
}