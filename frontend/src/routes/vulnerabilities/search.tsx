import { createFileRoute, Link } from "@tanstack/solid-router";
import { ColumnDef, createSolidTable, flexRender, getCoreRowModel } from "@tanstack/solid-table";

import { Accessor, Component, createEffect, createSignal, For, onMount, Suspense } from "solid-js";
import { useVulnerabilitiesPageQuery, Vulnerability } from "../../hooks/useVulnerabilities";
import { Icon } from "../../components/ui/Icon";

type VulnerabilitiesTableProps = {
  items: Vulnerability[];
};

const VulnerabilitiesList: Component<VulnerabilitiesTableProps> = (props) => {
  return (
    <>
      <ol class="">
        <For each={props.items}>
          {(item) => (
            <li class="border-t border-t-gray-200 py-2">
              <Link to="/" class="text-lg font-medium text-blue-500">
                {item.key}
              </Link>
              <div class="mt-1">
                <p class="text-gray-600">{item.description}</p>
              </div>
            </li>
          )}
        </For>
      </ol>
    </>
  );
};

const SearchPage: Component = () => {
  const [search, navigate] = [Route.useSearch(), Route.useNavigate()] as const;
  const searchPage = () => search().page - 1;
  const searchPageSize = () => search().pageSize;

  const vulnerabilitiesQuery = useVulnerabilitiesPageQuery(searchPage, searchPageSize);

  const [tableWidth, setTableWidth] = createSignal(0);
  const [tableHeight, setTableHeight] = createSignal(0);

  const handlePreviousClick = () => {
    if (search().page > 1) {
      navigate({
        to: "/vulnerabilities",
        search: (prevSearch) => {
          return {
            page: prevSearch.page - 1,
            pageSize: prevSearch.pageSize,
          };
        },
      });
    }
  };

  const handleNextClick = () => {
    navigate({
      to: "/vulnerabilities",
      search: (prevSearch) => {
        return {
          page: prevSearch.page + 1,
          pageSize: prevSearch.pageSize,
        };
      },
    });
  };

  let tableContainerRef: HTMLDivElement | undefined = undefined;

  onMount(() => {
    setTableWidth(tableContainerRef!.clientWidth);
  });

  return (
    <div class="flex h-full flex-col px-6 py-4">
      <h1 class="text-lg font-medium">Vulnerabilities</h1>

      <div class="mt-2 grid grid-cols-3">
        <div class="col-span-2">
          <input
            type="text"
            placeholder="Filter vulnerabilities..."
            class="h-9 w-96 rounded-md border border-gray-200 px-2 text-gray-600"
          />
        </div>
        <div class="col-span-1">
          <div class="mt-6 flex items-center gap-x-2">
            <button
              on:click={handlePreviousClick}
              class="flex h-9 w-9 cursor-pointer items-center justify-center rounded-md bg-blue-700 font-medium text-white hover:bg-blue-800"
            >
              <Icon name="arrow_left_alt" />
            </button>

            <div>{search().page}</div>

            <button
              on:click={handleNextClick}
              class="flex h-9 w-9 cursor-pointer items-center justify-center rounded-md bg-blue-700 font-medium text-white hover:bg-blue-800"
            >
              <Icon name="arrow_right_alt" />
            </button>
          </div>
        </div>
      </div>

      <div ref={tableContainerRef} class="mt-6">
        <Suspense fallback={<p>Loading...</p>}>
          <VulnerabilitiesList items={vulnerabilitiesQuery.data?.items || []} />
        </Suspense>
      </div>
    </div>
  );
};

// const VulnerabilitiesList: Component = () => {
//   const handlePreviousClick = () => {
//     if (search().page > 1) {
//       navigate({
//         to: "/vulnerabilities",
//         search: (prevSearch) => {
//           return {
//             page: prevSearch.page - 1,
//             pageSize: prevSearch.pageSize,
//           };
//         },
//       });
//     }
//   };

//   const handleNextClick = () => {
//     navigate({
//       to: "/vulnerabilities",
//       search: (prevSearch) => {
//         return {
//           page: prevSearch.page + 1,
//           pageSize: prevSearch.pageSize,
//         };
//       },
//     });
//   };

//   return (
//     <div class="px-4 py-4">
//       <h1 class="font-medium text-xl">Vulnerabilities</h1>

//       <Suspense fallback={<p>Loading...</p>}>
//         <p class="mt-4">
//           Showing {search().pageSize} of {vulnerabilities.data?.totalItems}{" "}
//           Vulnerabilities
//         </p>

//         <table>
//           <thead>
//             <tr>
//               <th>CVE</th>
//               <th>Reference</th>
//             </tr>
//           </thead>
//           <tbody>
//             <For each={vulnerabilities.data?.items}>
//               {(vulnerability) => (
//                 <tr class="list-none">
//                   <td>{vulnerability.key}</td>
//                   <td>{JSON.stringify(vulnerability.references[0])}</td>
//                 </tr>
//               )}
//             </For>
//           </tbody>
//         </table>

//         <div class="flex gap-x-2 mt-6 items-center">
//           <button
//             on:click={handlePreviousClick}
//             class="h-9 w-9 flex justify-center items-center text-white rounded-md bg-blue-700 cursor-pointer hover:bg-blue-800 font-medium"
//           >
//             <Icon name="arrow_left_alt" />
//           </button>

//           <div>{search().page}</div>

//           <button
//             on:click={handleNextClick}
//             class="h-9 w-9 flex justify-center items-center text-white rounded-md bg-blue-700 cursor-pointer hover:bg-blue-800 font-medium"
//           >
//             <Icon name="arrow_right_alt" />
//           </button>
//         </div>
//       </Suspense>
//     </div>
//   );
// };

export type VulnerabilitiesSearch = {
  page: number;
  pageSize: number;
};

export const Route = createFileRoute("/vulnerabilities/search")({
  validateSearch: (search): VulnerabilitiesSearch => {
    const page = Number.parseInt((search?.page as string) || "1");
    const pageSize = Number.parseInt((search?.pageSize as string) || "25");

    if (page < 0) {
      throw new Error(`invalid page (got: "${page}", expected: "1..")`);
    }

    if (page < 0) {
      throw new Error(`invalid page size (got: "${page}", expected: "1..=10000")`);
    }

    return {
      page,
      pageSize,
    };
  },
  component: SearchPage,
});
