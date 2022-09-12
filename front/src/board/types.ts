export type HSN = string;
export type Player = bigint;
export type Resource = "brick" | "grain" | "lumber" | "ore" | "wool";
export type Role = Resource | "desert" | "sea";
export type Tiles = Tile[][];

export interface Tile {
    type: Role;
    number?: bigint;
    robber?: boolean;
}
