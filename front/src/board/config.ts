import * as types from "./types";

export default interface Config {
    hsn?: types.HSN; // The starting configuration
    colors?: string[]; // A mapping from player IDs to hex colors
}
