import { setContext, getContext } from 'svelte';
import type { Writable } from 'svelte/store';

export interface AppState {
    isLoading: boolean;
    version: string;
    appName: string;
}

export const setAppState = (context: Writable<AppState>) => {
    setContext("app-state", context);
}

export const useAppState = (): Writable<AppState> => {
    return getContext("app-state");
}