export type Coordinate = [number, number, number];
export type Player = bigint;
export type Position = {};

// TODO: Add support for cities
export type Role = "road" | "settlement";

// A piece can be identified by its role and the player it belongs to (its 'color')
export interface Piece {
    role: Role;
    player: Player;
}
