import Board from "./board";
import { configure, PartialConfig } from "./config";
import { HSN } from "./types";

export default function board(initial: HSN, config: PartialConfig): Board {
    return new Board(initial, configure(config));
}
