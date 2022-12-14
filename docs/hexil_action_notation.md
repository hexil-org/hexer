# HAN: Hexil Action Notation

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

The Hexil Action Notation describes the notation for game actions. This action
notation is defined in terms of smaller notations.

## Maybe notation

Given some notation N, the maybe notation of N is a notation for either N or
the cover value, notated with `?`.

## Resource notation

| Resource | Code |
| -------- | ---- |
| Brick    | `B`  |
| Grain    | `G`  |
| Lumber   | `L`  |
| Ore      | `O`  |
| Wool     | `W`  |

### Standard order

The standard order of resource codes is the alphabetical order. A mnemonic for
this order is 'BeeGLOW' (think of a bee glowing).

## Development card notation

| Development Card | Code |
| ---------------- | ---- |
| Knight           | `Dk` |
| Monopoly         | `Dm` |
| Road building    | `Dr` |
| Victory point    | `Dv` |
| Year of plenty   | `Dy` |

### Standard order

The standard order of development card codes is the alphabetical order.

## Formula notation

A formula describes a combination of resources. It is similar to a chemical
molecular formula. It consists of pairs with a resource code and a number. The
number '1' SHOULD be omitted. The resources SHOULD appear in the standard order.
Each resource SHOULD only appear once.

A formula MUST start with an open parenthesis and end with a closing
parenthesis.

### Examples

-   2 wool

    `(W2)`

-   1 lumber

    `(L)`

-   3 wool, 4 brick and 1 grain

    `(B4GW3)`

## Die-value notation

A die-value is notated with one of `1`, `2`, `3`, `4`, `5`, `6`.

### Standard order

The standard order of die-values is the numerical order.

## Roll-value notation

A roll-value notation MUST start with an open parenthesis and end with a closing
parenthesis.

The die values MUST be separated with the `+` symbol.

The die values SHOULD appear in the standard order, from left to right.

### Examples

-   3 and 3

    `(3+3)`

-   4 and 5

    `(4+5)`

-   3 and 6

    `(3+6)`

## Player notation

A player is notated with either `0` (the bank), `1`, `2`, `3` or `4`.

## Coordinate notation

Read these first:

-   Axial coordinates: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
-   Hexagon grid relationships: https://www.redblobgames.com/grids/parts/#hexagon-coordinates

Tile-coordinates are notated as axial coordinates. But the first value is
notated in spreadsheet-style (1=a, 2=b, 3=c, ..., 26=z, 27=aa). So the axial
coordinate (1,4) is written as a4.

```
       d2  e2  f2

     c3  d3  e3  f3

   b4  c4  d4  e4  f4

     b5  c5  d5  e5

       b6  c6  d6
```

Vertex-coordinates are notated as the tile-coordinate of the tile it touches,
but with an `s` or `n` appended at the end to indicate if it is the south corner
or the north corner of that tile.

Edge-coordinates are notated as the tile-coordinate of the tile it borders, and
an `ne` or `nw` or `w` to indicate if it is the north-east, north-west or west
edge of that tile.

### Examples

-   Axial (4,4), The center hex in the default game setup

    `d4`

-   The top corner of the center hex in the default game setup

    `d4n`

-   The bottom corner of the center hex in the default game setup

    `d4s`

-   The left edge of the center hex in the default game setup

    `d4w`

## Action notation

To form an action, take one row in the table and concatenate from left to right.
Skip cells with dashes (meaning inferred) or blanks (meaning not relevant).

| Description               | Subject (who) | Verb | Direct Object (what)       | Second Object                | Indirect object |
| :------------------------ | :------------ | :--- | :------------------------- | :--------------------------- | :-------------- |
| Roll                      | -             | `R`  | Value (Maybe Roll-value)   |                              |                 |
| Move Robber               | -             | `M`  | -                          | Destination (TileCoordinate) |                 |
| Abandon (discard)         | (Player)      | `A`  | Resources (Formula)        |                              |                 |
| Steal                     | -             | `S`  | Resource (Resource)        |                              | From (Player)   |
| Buy a village             | -             | `B`  | Village (`V`)              |                              |                 |
| Buy a city                | -             | `B`  | City (`C`)                 |                              |                 |
| Buy a road                | -             | `B`  | Road (`R`)                 |                              |                 |
| Buy a development card    | -             | `B`  | Development Card (`D?`)    |                              |                 |
| Place a village           | -             | `P`  | Village (`V`)              | Location (VertexCoordinate)  |                 |
| Place a city              | -             | `P`  | City (`C`)                 | Location (VertexCoordinate)  |                 |
| Place a road              | -             | `P`  | Road (`R`)                 | Location (EdgeCoordinate)    |                 |
| Use a knight card         | -             | `U`  | Knight Card (`Dk`)         |                              |                 |
| Use a monopoly card       | -             | `U`  | Monopoly Card (`Dm`)       | Resource (Resource)          | -               |
| Use a road card           | -             | `U`  | Road Card (`Dr`)           |                              |                 |
| Use a year of plenty card | -             | `U`  | Year of plenty card (`Dy`) | Resources (Formula)          | -               |
| Trade                     | -             | `T`  | (Formula)                  | For (Formula)                | With (Player)   |
| End turn                  | -             | `E`  | -                          |                              |                 |

### Examples

-   Roll 5

    `R(2+3)`

-   Steal 1 ore from player 2

    `SO2`

-   Player 2 abandons 2 ore

    `2A(O2)`

-   Steal an unknown resource from player 1

    `S?1`

-   Place a village on d4n

    `PVd4n`

-   Trade 1 lumber and 2 ore for 3 brick with player 1

    `T(LO2)(B3)1`

-   Use a monopoly card to obtain all wool

    `UDmW`

-   Use a knight

    `UDk`

-   Buy a development card of unknown type

    `BD?`

-   You bought a development card of type monopoly

    `BDm`

-   End the turn

    `E`

## Grammar

A Parsing Expression Grammar for [pest](https://pest.rs):

```rust
action = { roll | move_robber | abandon | steal | buy | place_village |
          place_city | place_road | use_card | trade | end_turn }

roll        = { "R" ~ ("?" | roll_value) }
move_robber = { "M" ~ tile_coordinate }
abandon     = { player ~ "A" ~ formula }

steal         = { "S" ~ ("?" | resource) ~ player }
buy           = { "B" ~ ("V" | "C" | "R" | (development | "D?")) }
place_village = { "P" ~ "V" ~ vertex_coordinate }
place_city    = { "P" ~ "C" ~ vertex_coordinate }
place_road    = { "P" ~ "R" ~ edge_coordinate }
use_card      = { "U" ~ ("Dk" | ("Dy" ~ formula) | ("Dm" ~ resource) | "Dr") }
trade         = { "T" ~ formula ~ formula ~ player }
end_turn      = { "E" }

roll_value = { "(" ~ die_value ~ "+" ~ die_value ~ ")" }
die_value  = { '1'..'6' }

tile_coordinate   = { q_component ~ r_component }
vertex_coordinate = { q_component ~ r_component ~ corner }
edge_coordinate   = { q_component ~ r_component ~ border }

q_component = @{ 'a'..'z'+ }
r_component = @{ '1'..'9' ~ '0'..'9'* }

corner = { "n" | "s" }
border = { "ne" | "nw" | "w" }

formula = { "(" ~ (resource ~ amount?)+ ~ ")" }
amount = { ('1'..'9' ~ '0'..'9'*) }

resource = { "B" | "G" | "L" | "O" | "W" }
development = { knight | road | monopoly | victory_point | year_of_plenty }

knight          = { "Dk" }
road            = { "Dr" }
monopoly        = { "Dm" }
victory_point   = { "Dv" }
year_of_plenty  = { "Dy" }

player = { '0'..'6' }
```
