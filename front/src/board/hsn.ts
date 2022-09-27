import * as types from "./types";
import { Tile } from "./tile";
import { HeadlessState } from "./state";

const resources: { [letter: string]: types.Role } = {
    b: "brick",
    g: "grain",
    l: "lumber",
    o: "ore",
    w: "wool",
    d: "desert",
    s: "sea",
};

export function parse(hsn: types.HSN): HeadlessState {
    let state: HeadlessState = { tiles: [] };

    let row = 0,
        col = 0;

    for (let i = 0; i < hsn.length; i++) {
        const c = hsn[i];
        switch (c) {
            case " ":
                return state;
            case "/":
                col = 0;
                row += 1;
                break;
            default:
                if (c >= "0" && c <= "9") {
                    let s = c;
                    while (
                        hsn[i + 1] &&
                        hsn[i + 1] >= "0" &&
                        hsn[i + 1] <= "9"
                    ) {
                        s += hsn[++i];
                    }

                    col += parseInt(s);
                } else if (c in resources) {
                    state.tiles.push(new Tile(resources[c], 1, col, row));

                    col++;
                }

                break;
        }
    }

    return state;
}
