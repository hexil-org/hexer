import Config from "./config";
import Konva from "konva";
import { parse } from "./hsn";

export function board(element: HTMLDivElement, config: Config) {
    const stage = new Konva.Stage({
        container: element,
        width: window.innerWidth,
        height: window.innerHeight,
    });

    const layer = new Konva.Layer();
    const tiles = new Konva.Group();

    layer.add(tiles);
    stage.add(layer);

    const state = parse("3ssss/2slwgs/1sbobws/sdlglgs/sbwwos/sogls/ssss");

    for (const t of state.tiles) {
        t.render(tiles);
    }
}
