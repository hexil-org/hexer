/**
 * Converts a numerical tile position into a tile key.
 */
export function pos2key(col: number, row: number): string {
    col--;

    let colName = "";
    while (col >= 0) {
        colName = "abcdefghijklmnopqrstuvwxyz"[col % 26] + colName;
        col = Math.floor(col / 26) - 1;
    }

    return colName + row;
}

/**
 * Converts a pair of axial coordinates into cartesian coordinates.
 */
export function axial2cartesian(
    axialX: number,
    axialY: number,
    radius: number
): [number, number] {
    const x =
        2 * radius * Math.cos(Math.PI / 6) * (axialX + axialY / 2) + radius;
    const y = 1.5 * radius * axialY + radius;

    return [x, y];
}
