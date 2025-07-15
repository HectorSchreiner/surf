import { createRootRoute, Outlet } from "@tanstack/solid-router";
import { TanStackRouterDevtools } from "@tanstack/solid-router-devtools";
import { SolidQueryDevtools as TanStackQueryDevtools } from "@tanstack/solid-query-devtools";
import { Sidebar } from "../components/Sidebar";

export const Route = createRootRoute({
  component: () => (
    <>
      <div class="grid grid-cols-[auto_1fr]">
        <Sidebar />
        <main class="h-screen overflow-y-scroll">
          <Outlet />
        </main>
      </div>
      <TanStackRouterDevtools />
      <TanStackQueryDevtools />
    </>
  ),
  notFoundComponent: () => <p>Page doesn't exist</p>,
});
