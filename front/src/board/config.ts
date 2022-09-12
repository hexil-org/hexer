import * as types from "./types";

export default interface Config {
    hsn?: types.HSN; // The starting configuration
    turnPlayers: types.Player[]; // IDs of players that can interact with the board
    colors?: string[]; // A mapping from player IDs to hex colors
}
