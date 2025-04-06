import { invoke } from '@tauri-apps/api/core';
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { Encounter, GetPastEncountersCriteria, GetPastEncountersResult, GetStatsResult, LoadResult, ProcessWatcherResult, Settings, UpdateCheckResult } from './types';

export const load = (): Promise<LoadResult> => {
    return invoke<LoadResult>("load");
}

export const getSettings = (): Promise<Settings> => {
    return invoke<Settings>("get_settings");
}

export const saveSettings = (settings: Settings): Promise<Settings> => {
    return invoke<Settings>("save_settings", { settings });
}

export const getStats = (): Promise<GetStatsResult> => {
    return invoke<GetStatsResult>("get_stats");
}

export const getPastEncounters = (criteria: GetPastEncountersCriteria): Promise<GetPastEncountersResult> => {
    return invoke<GetPastEncountersResult>("get_past_encounters");
}

export const onEncounterUpdate = (handler: (encounter: Encounter) => void): Promise<UnlistenFn> => {
    return listen<Encounter>("encounter-update", (event) => {
        handler(event.payload);
    });
}

export const onUpdateCheck = (handler: (result: UpdateCheckResult) => void): Promise<UnlistenFn> => {
    return listen<UpdateCheckResult>("updater", (event) => {
        handler(event.payload);
    });
}

export const onProcessUpdate = (handler: (result: ProcessWatcherResult) => void): Promise<UnlistenFn> => {
    return listen<ProcessWatcherResult>("process-check", (event) => {
        handler(event.payload);
    });
}

export const checkForUpdate = (): Promise<void> => {
    return emit("check-update");
}

export const installUpdate = (): Promise<void> => {
    return emit("install-update");
}