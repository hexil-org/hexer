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
