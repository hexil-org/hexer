# HAN: Hexil Action Notation

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

The Hexil Action Notation describes the notation for game actions. This action
notation (at the bottom of this document) is defined in terms of smaller
notations.

Each notation has a standard order.

## Unordered tuple notation

Given some notation N, the unordered tuple of N is a notation for a set of two
or more values in the denotation of N.

An unordered tuple MUST start with an open parenthesis and end with a closing
parenthesis.

The elements of an unordered tuple MUST be separated with the `+` symbol.

The elements of an unordered tuple SHOULD appear in the standard order, from
left to right.

## Ordered tuple notation

Given some notation N, the ordered tuple of N is a notation for a sequence of
two or more values in the denotation of N.

An ordered tuple MUST start with an open parenthesis and end with a closing
parenthesis.

The elements of an unordered tuple MUST be separated with the `,` symbol.

## Hidable notation

Given some notation N, the hidable notation of N is a notation for either N or
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

The roll-value notation is the unordered tuple notation of two die-values.

### Examples

-   3 and 3

    `(3+3)`

-   4 and 5

    `(4+5)`

-   3 and 6

    `(3+6)`

## Player notation

A player is notated with one of `0` (the bank), `1`, `2`, `3` or `4`.

### Standard order

The standard order is the numerical order.

## Coordinate

Read these first:

-   Axial coordinates: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
-   Hexagon grid relationships: https://www.redblobgames.com/grids/parts/#hexagon-coordinates

A tile-coordinate is notated as an axial coordinate. It is an ordered tuple of
the q and r value.

A vertex-coordinate is notated as an ordered tuple with the axial q and r value
of the tile it touches, and an `S` or `N` to indicate if it is the south corner
or the north corner of that tile.

An edge-coordinate is notated as an ordered tuple with the axial q and r value
of the tile it borders, and an `NE` or `NW` or `W` to indicate if it is the
north-east, north-west or west edge of that tile.

## Action notation

To form an action, take one row in the table and concatenate from left to right.
Skip cells with dashes (meaning inferred) or blanks (meaning not relevant).

| Description               | Subject (who) | Verb | Direct Object (what)       | Goal                         | Indirect object |
| :------------------------ | :------------ | :--- | :------------------------- | :--------------------------- | :-------------- |
| Roll                      | -             | `R`  | Value (Roll-value)         |                              |                 |
| Move Robber               | -             | `M`  | -                          | Destination (TileCoordinate) |                 |
| Discard                   | (Player)      | `D`  | Resources (Formula)        |                              |                 |
| Steal                     | -             | `S`  | Resource (Hidable Formula) |                              | From (Player)   |
| Buy a village             | -             | `B`  | Village (`v`)              |                              |                 |
| Buy a city                | -             | `B`  | City (`c`)                 |                              |                 |
| Buy a road                | -             | `B`  | Road (`r`)                 |                              |                 |
| Buy a development card    | -             | `B`  | Development Card (`d`)     |                              |                 |
| Place a village           | -             | `P`  | Village (`v`)              | Location (VertexCoordinate)  |                 |
| Place a city              | -             | `P`  | City (`c`)                 | Location (VertexCoordinate)  |                 |
| Place a road              | -             | `P`  | Road (`r`)                 | Location (EdgeCoordinate)    |                 |
| Use a knight card         | -             | `U`  | Knight Card (`k`)          |                              |                 |
| Use a year of plenty card | -             | `U`  | Year of plenty card (`p`)  | Resources (Formula)          | -               |
| Use a monopoly card       | -             | `U`  | Monopoly Card (`m`)        | Resource (Formula)           | -               |
| Use a road card           | -             | `U`  | Road Card (`o`)            |                              |                 |
| Trade                     | -             | `T`  | (Formula)                  | For (Formula)                | With (Player)   |

### Examples

-   Roll 5

    `R(2+3)`

-   Steal 1 ore from player 2

    `S(O)2`

-   Steal an unknown resource from player 1

    `S?1`

-   Place a village on (0, -1, S)

    `Pv(0,-1,S)`

-   Trade 1 lumber and 2 ore for 3 brick with player 1

    `T(LO2)(B3)1`
