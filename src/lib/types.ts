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
