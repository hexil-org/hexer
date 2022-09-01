# HAN: Hexil Action Notation

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

The Hexil Action Notation describes the notation for game actions. This action
notation (at the bottom of this document) is defined in terms of smaller
notations.

Each notation has a standard order.

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

### Standard Order

The standard order of resource codes is the alphabetical order. A mnemonic for
this order is 'BeeGLOW' (think of a bee glowing).

## Formula notation

A formula describes a combination of resources. It is similar to a chemical
molecular formula. It consists of pairs with a resource code and a number. The
number '1' SHOULD be omitted. The resources SHOULD appear in the standard order.

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

### Standard order

The standard order is the numerical order.

## Coordinate

Read these first:

-   Axial coordinates: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
-   Hexagon grid relationships: https://www.redblobgames.com/grids/parts/#hexagon-coordinates

A coordinate MUST start with an open parenthesis and end with a closing
parenthesis.

Tile-coordinates are notated as axial coordinates. But the first value is
converted to a letter in spreadsheet-style (1=a, 2=b, 3=c, ..., 26=z, 27=aa).
So the axial coordinate (1,4) is written as a4.

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

    `(d4)`

-   The top corner of the center hex in the default game setup

    `(d4n)`

-   The bottom corner of the center hex in the default game setup

    `(d4s)`

-   The left edge of the center hex in the default game setup

    `(d4w)`

## Action notation

To form an action, take one row in the table and concatenate from left to right.
Skip cells with dashes (meaning inferred) or blanks (meaning not relevant).

| Description               | Subject (who) | Verb | Direct Object (what)      | Goal                         | Indirect object |
| :------------------------ | :------------ | :--- | :------------------------ | :--------------------------- | :-------------- |
| Roll                      | -             | `R`  | Value (Maybe Roll-value)  |                              |                 |
| Move Robber               | -             | `M`  | -                         | Destination (TileCoordinate) |                 |
| Discard                   | (Player)      | `D`  | Resources (Formula)       |                              |                 |
| Steal                     | -             | `S`  | Resource (Maybe Formula)  |                              | From (Player)   |
| Buy a village             | -             | `B`  | Village (`v`)             |                              |                 |
| Buy a city                | -             | `B`  | City (`c`)                |                              |                 |
| Buy a road                | -             | `B`  | Road (`r`)                |                              |                 |
| Buy a development card    | -             | `B`  | Development Card (`d`)    |                              |                 |
| Place a village           | -             | `P`  | Village (`v`)             | Location (VertexCoordinate)  |                 |
| Place a city              | -             | `P`  | City (`c`)                | Location (VertexCoordinate)  |                 |
| Place a road              | -             | `P`  | Road (`r`)                | Location (EdgeCoordinate)    |                 |
| Use a knight card         | -             | `U`  | Knight Card (`k`)         |                              |                 |
| Use a year of plenty card | -             | `U`  | Year of plenty card (`y`) | Resources (Formula)          | -               |
| Use a monopoly card       | -             | `U`  | Monopoly Card (`m`)       | Resource (Formula)           | -               |
| Use a road card           | -             | `U`  | Road Card (`o`)           |                              |                 |
| Trade                     | -             | `T`  | (Formula)                 | For (Formula)                | With (Player)   |
| End turn                  | -             | `E`  | -                         |                              |                 |

### Examples

-   Roll 5

    `R(2+3)`

-   Steal 1 ore from player 2

    `S(O)2`

-   Player 2 discards 2 ore

    `2D(O2)`

-   Steal an unknown resource from player 1

    `S?1`

-   Place a village on (d4n)

    `Pv(d4n)`

-   Trade 1 lumber and 2 ore for 3 brick with player 1

    `T(LO2)(B3)1`

## Syntax

```
action = { roll | move_robber | discard | steal | buy | place_village |
          place_city | place_road | use_card | trade | end_turn }

roll        = { "R" ~ ("?" | roll_value) }
move_robber = { "M" ~ tile_coordinate }
discard     = { player ~ "D" ~ formula }

steal           = { "S" ~ ("?" | formula) ~ player }
buy             = { "B" ~ buyable }
place_village   = { "P" ~ "v" ~ vertex_coordinate }
place_city      = { "P" ~ "c" ~ vertex_coordinate }
place_road      = { "P" ~ "r" ~ edge_coordinate }
use_card        = { "U" ~ ("k" | ("y" ~ formula) | ("m" ~ formula) | "o") }
trade           = { "T" ~ formula ~ formula ~ player }
end_turn        = { "E" }

buyable = { "v" | "c" | "r" | "d" }

roll_value = { "(" ~ die_value ~ "+" ~ die_value ~ ")" }
die_value  = { '1'..'6' }

tile_coordinate     = { "(" ~ q_component ~ r_component ~  ")" }
vertex_coordinate   = { "(" ~ q_component ~ r_component ~ corner ~ ")" }
edge_coordinate     = { "(" ~ q_component ~ r_component ~ border ~ ")" }

q_component = @{ 'a'..'z'+ }
r_component = @{ '1'..'9' ~ '0'..'9'* }

corner = { "n" | "s" }
border = { "ne" | "nw" | "w" }

formula = { "(" ~ (resource ~ integer?)+ ~ ")" }

resource = { "B" | "G" | "L" | "O" | "W" }

player = { '0'..'6' }

integer = { "0" | ('1'..'9' ~ '0'..'9'*) }
```
