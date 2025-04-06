import type { Writable } from "svelte/store";

export interface LoadResult {
    appName: string;
    version: string;
}

export interface Settings {
    version: string;   
    sniffer: SnifferSettings;
    updater: UpdaterSettings;
    meter: MeterSettings;
}

export interface SnifferSettings {
    port: number;
    checkInterval: string;
    processName: string;
}

export interface UpdaterSettings {
    
}

export interface MeterSettings {
   
}

export interface GetStatsResult {
    classPopularity: Metric[];
    itemLevelBreakdown: Metric[];
    metrics: Metric[];
}

export interface Metric {
   name: string;
   value: number;
}

export interface ServerPopulation {
    na: NorthAmericaNode;
    eu: EuropeNode;
}

export interface NorthAmericaNode {
    name: string;
    naw: Metric[];
    nae: Metric[];
}

export interface EuropeNode {
    name: string;
    metrics: Metric[];
}

export interface GetPastEncountersCriteria {
    
}

export interface Encounter {
    id: String;
    name: string;
    classes: string;
    duration: {
        mmss: number;
    }
}

export interface EncounterMonitor {
    encounter?: Encounter
}

export interface GetPastEncountersResult {
    pageSize: number;
    items: Encounter[]
}

export interface UpdateCheckResult {
    checkedOn: string;
    state: any;
}

export interface ProcessWatcherResult {
    checkedOn: string;
    state: {
        type: String;
    };
}

export interface ProcessWatcher {
    result: Writable<ProcessWatcherResult | undefined>
}

export interface UpdateManager {
    result: Writable<UpdateCheckResult | undefined>;
    checkForUpdate(): Promise<void>;
    install(): Promise<void>;
}

export interface AppState {
    isLoading: boolean;
    version: string;
    appName: string;
}