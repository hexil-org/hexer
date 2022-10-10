import { Config } from "./config";
import { State } from "./state";
import { parse } from "./hsn";
import Konva from "konva";
import { HSN } from "./types";

export default class Board {
    public board: State;
    public config: Config;

    private layers!: {
        tabletop: Konva.Layer;
        board: Konva.Layer;
    };

    private groups!: {
        tiles: Konva.Group;
    };

    private stage!: Konva.Stage;

    constructor(initial: HSN, config: Config) {
        this.board = parse(initial);
        this.config = config;

        // Do the initial paint
        this.initStage();
        this.redrawAll();
    }

    /**
     * Enables or disables panning of the board.
     *
     * @param v
     */
    public setAllowPan(v: boolean = true): void {
        if (v === this.config.allowPan) return;
        this.config.allowPan = v;
        this.setBoardPanning(v);
    }

    /**
     * Enables or disables zooming of the board.
     *
     * @param v
     */
    public setAllowZoom(v: boolean = true): void {
        this.config.allowZoom = v;
    }

    /**
     * Sets the element to attach the stage to.
     *
     * This function is expensive, as it requires the entire scene to be
     * redrawn.
     *
     * @param v
     */
    public setBoardElement(v: HTMLDivElement | string): void {
        if (v === this.config.boardElement) return;
        this.config.boardElement = v;
        this.redrawAll();
    }

    /**
     * Enables or disables panning of the board.
     *
     * @private
     */
    private setBoardPanning(v: boolean = true): void {
        this.layers.board.draggable(v);
    }

    /**
     * Draws the background (tabletop).
     *
     * @private
     */
    private drawBackground(): void {
        // TODO: Clean this up
        const backgroundImage = new Image();
        backgroundImage.src = "/assets/img/tabletop_wood.png";

        const background = new Konva.Rect({
            width: this.stage.width(),
            height: this.stage.height(),
        });

        backgroundImage.onload = function () {
            background.fillPatternImage(backgroundImage);
        };

        this.layers.tabletop.add(background);
    }

    /**
     * Draw the tiles for the current state.
     *
     * @private
     */
    private drawTiles(): void {
        for (const tile of this.board.tiles) {
            tile.render(this.groups.tiles);
        }

        const rect = this.groups.tiles.getClientRect();
        const center = {
            x: this.layers.tabletop.width() / 2 - rect.width / 2 - rect.x,
            y: this.layers.tabletop.height() / 2 - rect.height / 2 - rect.y,
        };

        this.groups.tiles.setPosition(center);

        console.log(rect);
    }

    /**
     * Redraws the entire scene from the current configuration and state.
     *
     * @private
     */
    private redrawAll(): void {
        this.clearStage();

        this.drawBackground();
        this.drawTiles();

        this.setBoardPanning(this.config.allowPan);
    }

    /**
     * Clears the stage.
     *
     * @private
     */
    private clearStage(): void {
        this.layers.tabletop.removeChildren();
        this.groups.tiles.removeChildren();
    }

    /**
     * Initialises the stage and (sub)layers.
     *
     * @private
     */
    private initStage(): void {
        this.layers = {
            tabletop: new Konva.Layer(),
            board: new Konva.Layer(),
        };

        this.groups = {
            tiles: new Konva.Group(),
        };

        this.stage = new Konva.Stage({
            container: this.config.boardElement,
            width: this.config.canvasWidth,
            height: this.config.canvasHeight,
        });

        this.layers.board.add(this.groups.tiles);
        this.stage.add(this.layers.tabletop, this.layers.board);

        this.initZoomHandler();
    }

    /**
     * Initialises zooming.
     *
     * @private
     */
    private initZoomHandler(): void {
        this.stage.on("wheel", (e) => {
            if (!this.config.allowZoom) return;

            const zoomSpeed = 1.05;

            e.evt.preventDefault();

            const pointerPos = this.stage.getPointerPosition()!;
            const direction = e.evt.ctrlKey ? e.evt.deltaY : -e.evt.deltaY;

            const oldScale = this.layers.board.scaleX();
            const newScale =
                direction >= 0 ? oldScale * zoomSpeed : oldScale / zoomSpeed;

            if (newScale < 0.8 || newScale > 3) {
                // Limit zooming to between 80% and 300%
                return;
            }

            const oldPos = {
                x: (pointerPos.x - this.layers.board.x()) / oldScale,
                y: (pointerPos.y - this.layers.board.y()) / oldScale,
            };

            const newPos = {
                x: pointerPos.x - oldPos.x * newScale,
                y: pointerPos.y - oldPos.y * newScale,
            };

            this.layers.board.scale({ x: newScale, y: newScale });
            this.layers.board.position(newPos);
        });
    }
}
