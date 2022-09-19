import Config from "./config";
import { drawTile } from "./draw";

export function board(element: HTMLCanvasElement, config: Config) {
    element.width = 2160;
    element.height = 2160;
    const drawContext = element.getContext("2d");

    if (drawContext === null) return;

    drawTile(
        0,
        0,
        {
            type: "sea",
        },
        drawContext
    );
    drawTile(
        1,
        0,
        {
            type: "sea",
        },
        drawContext
    );
    drawTile(
        0,
        1,
        {
            type: "sea",
        },
        drawContext
    );
    drawTile(
        1,
        1,
        {
            type: "sea",
        },
        drawContext
    );
    drawTile(
        0,
        2,
        {
            type: "sea",
        },
        drawContext
    );
    drawTile(
        1,
        2,
        {
            type: "sea",
        },
        drawContext
    );
}
