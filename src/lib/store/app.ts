import type { AppState } from '$lib/types';
import { setContext, getContext } from 'svelte';
import type { Writable } from 'svelte/store';

export const setAppState = (context: Writable<AppState>) => {
    setContext("app-state", context);
}

export const useAppState = (): Writable<AppState> => {
    return getContext("app-state");
}