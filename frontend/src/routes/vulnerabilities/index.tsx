import { Component, createEffect, createSignal } from "solid-js";
import { createFileRoute } from "@tanstack/solid-router";
import { Chart } from "../../components/ui/Chart";

const initialData = [
  ["Critical", 1],
  ["High", 1],
  ["Medium", 1],
  ["Low", 1],
];

const AnalyticsPage: Component = () => {
  const [data, setData] = createSignal(initialData);

  return (
    <div class="px-6 py-4">
      <div class="flex h-60 w-full flex-col rounded-md border border-gray-200">
        <div class="px-4 py-4">
          <p class="font-medium text-blue-700">Vulnerabilities Severity</p>
          <p class="text-sm font-medium">Last 7 days</p>
        </div>
        <Chart
          options={{
            grid: {
              top: 16,
              bottom: 32,
              left: 64,
              right: 64,
            },
            xAxis: { type: "category" },
            yAxis: { type: "value" },
            series: { type: "bar", data: data() },
            animation: false,
          }}
        />
      </div>
    </div>
  );
};

export const Route = createFileRoute("/vulnerabilities/")({
  component: AnalyticsPage,
});
