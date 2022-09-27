import { Role } from "./types";
import Konva from "konva";
import { axial2cartesian } from "./util";

export class Tile {
    private readonly colors = {
        brick: "#d5725b",
        grain: "#e7bf67",
        lumber: "#4a946f",
        ore: "#9d9d9d",
        wool: "#bede6f",
        desert: "#e4ddc0",
        sea: "#337dd4",
    };

    private readonly radius = 50;

    private readonly type: Role;
    private readonly number: number | null;
    private readonly axialX: number;
    private readonly axialY: number;

    private hasRobber: boolean = false;

    constructor(
        type: Role,
        number: number | null,
        axialX: number,
        axialY: number
    ) {
        this.type = type;
        this.number = number;
        this.axialX = axialX;
        this.axialY = axialY;
    }

    public setRobber(robber: boolean = true) {
        this.hasRobber = robber;
    }

    public render(group: Konva.Group) {
        const coords = axial2cartesian(this.axialX, this.axialY, this.radius);
        const hexagon = new Konva.RegularPolygon({
            x: coords[0],
            y: coords[1],
            sides: 6,
            radius: this.radius,
            fill: this.colors[this.type],
            strokeWidth: 2,
            stroke: "white",
        });

        group.add(hexagon);
    }
}
