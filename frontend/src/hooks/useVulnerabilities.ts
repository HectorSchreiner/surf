import createClient from "openapi-fetch";
import { Accessor } from "solid-js";
import { useQuery } from "@tanstack/solid-query";

import type { paths, components } from "../api.gen";

export type Vulnerability = components["schemas"]["Vulnerability"];

export type VulnerabilitesPage = {
  items: Vulnerability[];
  totalItems: number;
  totalPages: number;
  page: number;
  pageSize: number;
};

const client = createClient<paths>({ baseUrl: "/" });

export const useVulnerabilitiesPageQuery = (page: Accessor<number>, pageSize: Accessor<number>) => {
  return useQuery(() => ({
    queryKey: ["vulnerabilities", pageSize(), page()],
    queryFn: async (): Promise<VulnerabilitesPage> => {
      const { data, error } = await client.GET("/api/v1/vulnerabilities", {
        params: {
          query: {
            pageSize: pageSize(),
            page: page(),
          },
        },
      });

      if (data) {
        return data;
      } else {
        throw new Error(error);
      }
    },
  }));
};
