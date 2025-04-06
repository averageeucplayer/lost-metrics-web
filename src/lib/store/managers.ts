import type { ProcessWatcher, UpdateManager } from "$lib/types";
import { getContext, setContext } from "svelte";

export const setUpdateManager = (context: UpdateManager) => {
    setContext("update-manager", context);
}

export const useUpdateManager = (): UpdateManager => {
    return getContext("update-manager");
}

export const setProcessWatcher = (context: ProcessWatcher) => {
    setContext("process-watcher", context);
}

export const useProcessWatcher = (): ProcessWatcher => {
    return getContext("process-watcher");
}