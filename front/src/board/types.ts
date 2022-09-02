export type Coordinate = [number, number, number];
export type HSN = string;
export type Player = bigint;
export type Resource =
    | "B" // brick
    | "G" // grain
    | "L" // lumber
    | "O" // ore
    | "W" // wool
    | "S" // sea
    | null; // 'air'
export type Role = "road" | "settlement"; // TODO: Add support for cities

// Use axial coordinates in a 2-dimensional array
export type TileSet = Tile[][];

export interface Piece {
    role: Role;
    player: Player;
}

export interface Tile {
    type: Resource;
    number: bigint | null;
    robber: boolean;
}
