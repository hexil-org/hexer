# HSN: Hexil State Notation

HSN consists of five parts:

-   Map
-   Placements
-   Distribution
-   Turn number
-   Turn

## Example

Example of a complete HSN:

```
[3SSSS/2SOWLS/1SGBWBS/SGLDLOS/SLOGWS1/SBGWS2/SSSS3]
[/10,2,9/12,6,4,10/9,11,3,9/8,3,4,5/5,6,11]
[d2nw,e2nw,c3w,Of3ne,Bg4w,Wb5w,Le6nw,a7ne,Gc7nw]

[||e3n] [c3n,d5s|d3n,c5n|c4n] [d2w,d6nw|e2w,d4w|f2w,c4ne] d4

[B5O2W||G2W] [Dk2Dm|Dk|Dv2] [0|3|2] 2 1

2

R . . R2
```

## Map

The map, or static part of the board.

### Tile kinds

Axial, row-column based.

```
[3SSSS/2SOWLS/1SGBWBS/SGLDLOS/SLOGWS1/SBGWS2/SSSS3]
```

### Resource numbers

For each B, G, L, O or W in the tile kinds, give the corresponding number.

```
[/10,2,9/12,6,4,10/9,11,3,9/8,3,4,5/5,6,11]
```

### Harbors

Comma separated. Coordinates are sorted row-column. Optionally first a resource
and a dash.

```
[d2nw,e2nw,c3w,Of3ne,Bg4w,Wb5w,Le6nw,a7ne,Gc7nw]
```

## Placements

The placements, or dynamic part of the board.

### City locations

A list of locations, comma separated. Coordinates are sorted row-column. The
pipe symbol `|` separates players.

```
[||e3n]
```

### Village locations

Exactly like the city locations.

```
[c3n,d5s|d3n,c5n|c4n]
```

### Road locations

A list of locations, comma separated. Coordinates are sorted row-column. The
pipe symbol `|` separates players.

```
[d2w,d6nw|e2w,d4w|f2w,c4ne]
```

### Robber location

Just the coordinate.

```
d4
```

## Distribution

### Held resources

The resource formula for each player. Players are separated by the pipe symbol.

```
[B5O2W||G2W]
```

### Development cards

The development cards each player holds, _excluding_ any cards bought in the
current turn. So these are the cards that the player could use in this turn.

Written like a formula, with the card codes.

```
[Dk2Dm|Dk|Dv2]
```

### Number of used knights

The number of used knights per player.

```
[0|3|2]
```

### Largest army

Just the player number. `0` if the bank has it (not awarded).

```
2
```

### Longest road

Just the player number. `0` if the bank has it (not awarded).

```
1
```

## Turn number

Just the turn number. Counting starts at 1.

```
2
```

## Turn

### Rolled

An `R` if the current player already rolled. A dot (`.`) otherwise.

```
R
```

### Must

An `S` if the current player must steal now. An `M` if the current player must
move the robber now. Or a `D` followed by a list of players that must discard
now. Or a `P` followed by a list of items that the player _must_ place now. A
dot (`.`) otherwise.

```
D[1,3]
```

### Bought development cards

A formula of development cards bought this turn. A `.` otherwise.

```
.
```

### Placeable items

A formula of items that the player _may_ place immediately (road building
card). A `.` otherwise.

```
R2
```

## Grammar

A Parsing Expression Grammar for [pest](https://pest.rs):

```rust
state = { map ~ placements ~ distribution ~ turn_number ~ turn }

map          = { tile_kinds ~ resource_numbers ~ harbors }
placements   = { city_locations ~ village_locations ~ road_locations ~ robber_location }
distribution = { held_resources ~ held_development_cards ~ number_of_used_knights ~ largest_army_holder ~ longest_road_holder }
turn_number  = { amount }
turn         = { rolled_flag ~ forced_flag ~ bought_development_cards ~ placeable_items }

tile_kinds           = { "[" ~ (tile_kinds_row ~ ("/" ~ tile_kinds_row)*)? ~ "]" }
tile_kinds_row       = { (skip | tile_kind)* }
skip                 = { amount }
resource_numbers     = { "[" ~ (resource_numbers_row ~ ("/" ~ resource_numbers_row)*)? ~ "]" }
resource_numbers_row = { (amount ~ ("," ~ amount)*)? }
harbors              = { "[" ~ (harbor ~ ("," ~ harbor)*)? ~ "]" }
harbor               = { resource? ~ edge_coordinate }

city_locations    = { per_player_vertex_coordinate_list }
village_locations = { per_player_vertex_coordinate_list }
road_locations    = { per_player_edge_coordinate_list }
robber_location   = { tile_coordinate }

per_player_vertex_coordinate_list = _{ "[" ~ (vertex_coordinate_list ~ ("|" ~ vertex_coordinate_list)*)? ~ "]" }
vertex_coordinate_list            =  { (vertex_coordinate ~ ("," ~ vertex_coordinate)*)? }
per_player_edge_coordinate_list   = _{ "[" ~ (edge_coordinate_list ~ ("|" ~ edge_coordinate_list)*)? ~ "]" }
edge_coordinate_list              =  { (edge_coordinate ~ ("," ~ edge_coordinate)*)? }

held_resources         = { "[" ~ (formula? ~ ("|" ~ formula?)*)? ~ "]" }
held_development_cards = { "[" ~ (development_card_formula? ~ ("|" ~ development_card_formula?)*)? ~ "]" }
number_of_used_knights = { "[" ~ (integer? ~ ("|" ~ integer?)*)? ~ "]" }
largest_army_holder    = { player }
longest_road_holder    = { player }

rolled_flag              = { "R" | "." }
forced_flag              = { "S" | "M" | ("D" ~ "[" ~ (player ~ ("," ~ player)*)? ~ "]") | ("P" ~ "[" ~ (player ~ ("," ~ player)*)? ~ "]") | "." }
bought_development_cards = { development_card_formula | "." }
placeable_items          = { item_formula | "." }

tile_coordinate   =  { q_component ~ r_component }
vertex_coordinate =  { q_component ~ r_component ~ corner }
edge_coordinate   =  { q_component ~ r_component ~ border }
q_component       = @{ 'a'..'z'+ }
r_component       = @{ '1'..'9' ~ '0'..'9'* }
corner            =  { "n" | "s" }
border            =  { "ne" | "nw" | "w" }

item_formula             =  { (item ~ amount?)+ }
development_card_formula =  { (development ~ amount?)+ }
formula                  =  { (resource ~ amount?)+ }
tile_kind                =  { "S" | "D" | resource }
resource                 =  { "B" | "G" | "L" | "O" | "W" }
item                     =  { "V" | "C" | "R" }
development              =  { knight | road | monopoly | victory_point | year_of_plenty }
amount                   = @{ '1'..'9' ~ '0'..'9'* }
integer                  = @{ '0'..'9'+ }
player                   = @{ '0'..'6' }
knight                   =  { "Dk" }
road                     =  { "Dr" }
monopoly                 =  { "Dm" }
victory_point            =  { "Dv" }
year_of_plenty           =  { "Dy" }

WHITESPACE = _{ " " | "\t" | "\n" }
```
