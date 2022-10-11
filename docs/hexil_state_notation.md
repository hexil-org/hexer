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

[B5O2W||G2W] [k2m|k|v2] [0|3|2] 2 1

2

R . . r2
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
[k2m|k|v2]
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
r2
```

# Grammar

```rs
hsn = { map ~ placements ~ distribution ~ turn_number ~ turn }

map = { tiles ~ numbers ~ harbors }
tiles = { "[" ~ tile_row ~ ("/" ~ tile_row)* ~ "]" }
numbers = { "[" ~ number_row ~ ("/" ~ number_row)* ~ "]" }
harbors = { "[" ~ harbor ~ ("," ~ harbor)* ~ "]" }

tile_row = { (skip | tile)* }
skip = @{ positive_integer }
number_row = { (number ~ ("," ~ number)*)? }
number = @{ "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" }
harbor = ${ resource? ~ edge_coordinate }

placements = { city_locations ~ village_locations ~ road_locations ~ robber_location }
city_locations = { "TODO" }
village_locations = { "TODO" }
road_locations = { "TODO" }
robber_location = { "TODO" }

distribution = { resources ~ development_cards ~ number_of_used_knights ~ largest_army_holder ~ longest_road_holder }
resources = { "TODO" }
development_cards = { "TODO" }
number_of_used_knights = { "TODO" }
largest_army_holder = { "TODO" }
longest_road_holder = { "TODO" }

turn_number = { "TODO" }

turn = { rolled ~ must ~ bought ~ placable }
rolled = { "TODO" }
must = { "TODO" }
bought = { "TODO" }
placable = { "TODO" }

tile = @{ "S" | "D" | resource }
resource = @{ "B" | "G" | "L" | "O" | "W" }
positive_integer = @{ '1'..'9' ~ '0'..'9'* }

tile_coordinate   = { q_component ~ r_component }
vertex_coordinate = { q_component ~ r_component ~ corner }
edge_coordinate   = { q_component ~ r_component ~ border }

q_component = @{ 'a'..'z'+ }
r_component = @{ '1'..'9' ~ '0'..'9'* }

corner = { "n" | "s" }
border = { "ne" | "nw" | "w" }


WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
```
