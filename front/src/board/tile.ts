import { Role } from "./types";
import Konva from "konva";
import { axial2cartesian } from "./util";

export class Tile {
    private readonly radius = 75;
    private readonly colors = {
        brick: "#d5725b",
        grain: "#e7bf67",
        lumber: "#4a946f",
        ore: "#9d9d9d",
        wool: "#bede6f",
        desert: "#e4ddc0",
        sea: "#337dd4",
    };

    private readonly type: Role;
    private readonly number: number | null;
    private readonly coords: [number, number];

    constructor(
        type: Role,
        number: number | null,
        axialX: number,
        axialY: number
    ) {
        this.type = type;
        this.number = number;
        this.coords = axial2cartesian(axialX, axialY, this.radius);
    }

    public render(group: Konva.Group) {
        const hexagon = new Konva.RegularPolygon({
            x: this.coords[0],
            y: this.coords[1],
            sides: 6,
            radius: this.radius,
            fill: this.colors[this.type],
            strokeWidth: 2,
            stroke: "white",
        });

        group.add(hexagon);
    }
}
