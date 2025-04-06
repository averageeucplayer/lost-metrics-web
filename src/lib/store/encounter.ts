import type { EncounterMonitor } from "$lib/types";
import { getContext, setContext } from "svelte";
import type { Writable } from "svelte/store";

export const setEncounterMonitor = (context: Writable<EncounterMonitor>) => {
    setContext("encounter-monitor", context);
}

export const useEncounterMonitor = (): Writable<EncounterMonitor> => {
    return getContext("encounter-monitor");
}