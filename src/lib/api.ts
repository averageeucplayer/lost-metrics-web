import { invoke } from '@tauri-apps/api/core';
import type { LoadResult, Settings } from './types';

export const load = (): Promise<LoadResult> => {
    return invoke<LoadResult>("load");
}

export const getSettings = (): Promise<Settings> => {
    return invoke<Settings>("get_settings");
}

export const saveSettings = (settings: Settings): Promise<Settings> => {
    return invoke<Settings>("save_settings", { settings });
}