import { Tile } from "./types";

// Draws the given tile at the specified axial coordinates
export function drawTile(
    axialX: number,
    axialY: number,
    role: Tile,
    context: CanvasRenderingContext2D
): void {
    // The edges of a hexagon are 60 degrees
    const angle = (2 * Math.PI) / 6;
    const radius = 200;

    // tan(60) = y / r * cos(30) =>
    // y = r * cos(30) * tan(60) =>
    // y = 1.5r.
    const y = 1.5 * radius * axialY;

    // cos(30) = 0.5x / r =>
    // x = 2r * cos(30).
    const x = 2 * radius * Math.cos(angle / 2) * (axialX + axialY / 2);

    context.lineWidth = 15;
    context.beginPath();

    for (let i = 0; i < 6; i++) {
        context.lineTo(
            x + radius * Math.cos(angle * i + Math.PI / 6),
            y + radius * Math.sin(angle * i + Math.PI / 6)
        );
    }

    context.closePath();
    context.stroke();
}
