# HAN: Hexil Action Notation

## Resources

| Resource | Code |
| -------- | ---- |
| Brick    | 'B'  |
| Grain    | 'G'  |
| Lumber   | 'L'  |
| Ore      | 'O'  |
| Wool     | 'W'  |

## Formula's

A formula is a combination of resources, written like a chemical formula. _For
programs formatting: The letters must appear in aphabetical order (Mnemonic:
BeeGLOW (think of a bee glowing)._ _For programs parsing: Accept any order._

## Action

-   Quoted: Literal string
-   Comma: ordered tuple
-   Plus: unordered tuple
-   Question mark: may be unknown, then notated with a question mark.

Format: Left to right, separated with spaces. Skip dashes (meaning inferred) or
blanks (meaning not relevant).

| Description               | Subject (who)   | Verb | Direct Object (what)         | Goal                     | Indirect object |
| :------------------------ | :-------------- | :--- | :--------------------------- | :----------------------- | :-------------- |
| Roll                      | -               | 'R'  | Numberpair (Number + Number) |                          |                 |
| Move Robber               | -               | 'M'  | -                            | Destination (Coordinate) |                 |
| Discard                   | Player (Player) | 'D'  | Resources (Formula)          |                          |                 |
| Steal                     | -               | 'S'  | Resource (Formula?)          |                          | Player (Player) |
| Buy a village             | -               | 'B'  | Village ('v')                |                          |                 |
| Buy a city                | -               | 'B'  | City ('c')                   |                          |                 |
| Buy a road                | -               | 'B'  | Road ('r')                   |                          |                 |
| Buy a development card    | -               | 'B'  | Development Card ('d')       |                          |                 |
| Place a village           | -               | 'P'  | Village ('v')                | Location (Coordinate)    |                 |
| Place a city              | -               | 'P'  | City ('c')                   | Location (Coordinate)    |                 |
| Place a road              | -               | 'P'  | Road ('r')                   | Location (Coordinate)    |                 |
| Use a knight card         | -               | 'U'  | Knight Card ('k')            |                          |                 |
| Use a year of plenty card | -               | 'U'  | Year of plenty card ('p')    | Resources (Formula)      | -               |
| Use a monopoly card       | -               | 'U'  | Monopoly Card ('m')          | Resource (Formula)       | -               |
| Use a road card           | -               | 'U'  | Road Card ('o')              |                          |                 |
| Trade                     | -               | 'T'  | (Formula)                    | For (Formula)            | With (Player)   |

A coordinate is an ordered tuple of two or three values.

Round brackets must be used around and may only be used around:

-   Formulas (a4b9)
-   Ordered tuples (a,b)
-   Unordered tuples (a+b). _Formatting programs: the smallest element must go
    first. Parsing programs: accept any order._

### Examples

-   Roll 5

    `R(2+3)`

-   Steal 1 ore from player 2

    `S(O)2`

-   Place a village on (0, -1, S)

    `Pv(0,-1,S)`

-   Trade 1 lumber and 2 ore for 3 brick with player 1

    `T(LO2)(B3)1`
