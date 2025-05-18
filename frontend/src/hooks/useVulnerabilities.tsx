import { useQuery } from "@tanstack/solid-query";
import type {paths, components } from "../schema";

import createClient from "openapi-fetch"; 

export type Vulnerability = components["schemas"]["Vulnerability"];

const client = createClient<paths>({baseUrl: "/"})

export const useVulnerabilities = () => {
    return useQuery(() => ({
        queryKey: ["vulnerabilities"],
        queryFn: async (): Promise<Vulnerability[]> => {
            const {data, error} = await client.GET("/api/v1/vulnerabilities");

            if (data) {
                return data;
            } else {
                throw new Error(error);
            }
        }
    }))
}