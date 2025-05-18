import { For, type Component } from 'solid-js';

import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { useVulnerabilities } from './hooks/useVulnerabilities';

const VulnerabilitiesList: Component = () => {
  const vulnerabilities = useVulnerabilities();

  return (
    <For each={vulnerabilities.data}>
      {(vulnerability) => (
        <li>
          <p>{vulnerability.key}</p>
        </li>
      )}
    </For>
  )
}

const App: Component = () => {
  const queryClient = new QueryClient();

  return (
    <QueryClientProvider client={queryClient}>
      <VulnerabilitiesList />
    </QueryClientProvider>

  );
};

export default App;
