import { createFileRoute } from "@tanstack/solid-router";

export const Route = createFileRoute("/")({
  component: () => <p>index</p>,
});
