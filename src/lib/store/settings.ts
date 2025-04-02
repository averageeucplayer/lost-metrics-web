import type { MeterSettings, SnifferSettings, UpdaterSettings } from '$lib/types';
import { setContext, getContext } from 'svelte';
import type { Writable } from 'svelte/store';

export const setSnifferSettings = (context: Writable<SnifferSettings>) => {
    setContext("sniffer-settings", context);
}

export const useSnifferSettings = (): Writable<SnifferSettings> => {
    return getContext("sniffer-settings");
}

export const setUpdaterSettings = (context: Writable<UpdaterSettings>) => {
    return getContext("updater-settings");
}

export const usetUpdaterSettings = (): Writable<UpdaterSettings> => {
    return getContext("updater-settings");
}

export const setMeterSettings = (context: Writable<MeterSettings>) => {
    return getContext("meter-settings");
}

export const useMeterSettings = (): Writable<MeterSettings> => {
    return getContext("meter-settings");
}