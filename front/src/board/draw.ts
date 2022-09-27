import { HeadlessState } from "./state";
import { Tile } from "./tile";
import Konva from "konva";

export function draw(state: HeadlessState): void {
    // TODO

    drawTiles(state.tiles);
}

function drawTiles(tiles: Tile[], tileLayer: Konva.Layer): void {
    for (const tile of tiles) {
        tile.render(tileLayer);
    }
}
