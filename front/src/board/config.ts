import * as types from "./types";

export default interface Config {
    position?: types.Position; // The initial board position/starting configuration
    colors?: string[]; // A mapping from player IDs to hex colors
}
