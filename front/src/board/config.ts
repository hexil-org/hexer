import * as types from "./types";

export interface PartialConfig {
    boardElement: HTMLDivElement | string; // The element to attach the stage to
    turnPlayers?: types.Player[]; // IDs of players that can interact with the board (default: [])
    colors?: string[]; // A mapping from player IDs to hex colors (default: [])
    allowPan?: boolean; // Whether to allow the user to pan around the board (default: true)
    allowZoom?: boolean; // Whether to allow the user to zoom the board in/out (default: true)
    canvasHeight?: number; // The height of the canvas (default: 1000)
    canvasWidth?: number; // The width of the canvas (default: 1000)
}

export interface Config {
    boardElement: HTMLDivElement | string;
    turnPlayers: types.Player[];
    colors: string[];
    allowPan: boolean;
    allowZoom: boolean;
    canvasHeight: number;
    canvasWidth: number;
}

/**
 * Amends the given partial config with the defaults.
 *
 * @param config
 */
export function configure(config: PartialConfig): Config {
    return { ...defaults(), ...config };
}

export function defaults() {
    return {
        turnPlayers: [],
        colors: [],
        allowPan: true,
        allowZoom: true,
        canvasHeight: 1000,
        canvasWidth: 1000,
    };
}
