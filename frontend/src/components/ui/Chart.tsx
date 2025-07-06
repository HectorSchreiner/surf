import { createEffect, createSignal, on, onCleanup, onMount, type Component } from "solid-js";
import { createElementSize } from "@solid-primitives/resize-observer";
import * as echarts from "echarts";

export type ChartProps = {
  options: echarts.EChartsOption;
};

export const Chart: Component<ChartProps> = (props) => {
  let chart: echarts.ECharts | undefined = undefined;
  let [chartRef, setChartRef] = createSignal<HTMLDivElement | undefined>();
  const chartRefSize = createElementSize(chartRef);

  onMount(() => {
    chart = echarts.init(chartRef()!);
  });

  onCleanup(() => {
    if (chart !== undefined) {
      echarts.dispose(chart);
    }
  });

  createEffect(
    on(
      () => props.options,
      () => {
        chart?.setOption(props.options);
      },
    ),
  );

  createEffect(
    on(
      () => [chartRefSize.width, chartRefSize.height],
      () => {
        chart?.resize();
      },
    ),
  );

  return <div class="h-full w-full" ref={setChartRef}></div>;
};
