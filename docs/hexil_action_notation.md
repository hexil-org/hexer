# Resources

| Resource | Code |
| -------- | ---- |
| Brick    | 'B'  |
| Grain    | 'G'  |
| Lumber   | 'L'  |
| Ore      | 'O'  |
| Wool     | 'W'  |

# Formula's

A formula is a combination of resources, written like a chemical formula. _For
programs formatting: The letters must appear in aphabetical order (Mnemonic:
BeeGLOW (think of a bee glowing)._ _For programs parsing: Accept any order._

# Action

-   Quoted: Literal string
-   Comma: ordered tuple
-   Plus: unordered tuple
-   Question mark: may be unknown, then notated with a question mark.

Format: Left to right, separated with spaces. Skip dashes (meaning inferred) or
blanks (meaning not relevant).

| Description               | Subject (who)         | Verb | Direct Object (what)         | Goal                        | Indirect object       |
| :------------------------ | :-------------------- | :--- | :--------------------------- | :-------------------------- | :-------------------- |
| Roll                      | -                     | 'R'  | Numberpair (Number + Number) |                             |                       |
| Move Robber               | -                     | 'M'  | -                            | Destination (HexCoordinate) |                       |
| Discard                   | Player (PlayerNumber) | 'D'  | Resources (ResourceFormula)  |                             |                       |
| Steal                     | -                     | 'S'  | Resource (ResourceFormula?)  |                             | Player (PlayerNumber) |
| Buy a village             | -                     | 'B'  | Village ('v')                |                             |                       |
| Buy a city                | -                     | 'B'  | City ('c')                   |                             |                       |
| Buy a road                | -                     | 'B'  | Road ('r')                   |                             |                       |
| Buy a development card    | -                     | 'B'  | Development Card ('d')       |                             |                       |
| Place a village           | -                     | 'P'  | Village ('v')                | Location (HexCoordinate)    |                       |
| Place a city              | -                     | 'P'  | City ('c')                   | Location (HexCoordinate)    |                       |
| Place a road              | -                     | 'P'  | Road ('r')                   | Location (HexCoordinate)    |                       |
| Use a knight card         | -                     | 'U'  | Knight Card ('k')            |                             |                       |
| Use a year of plenty card | -                     | 'U'  | Year of plenty card ('p')    | Resources (ResourceFormula) | -                     |
| Use a monopoly card       | -                     | 'U'  | Monopoly Card ('m')          | Resource (ResourceCode)     | -                     |
| Use a road card           | -                     | 'U'  | Road Card ('o')              |                             |                       |
| Trade                     | -                     | 'T'  | (ResourceFormula)            | For (ResourceFormula)       | With (PlayerNumber)   |

A hexcoordinate is an ordered tuple with the three hexcoordinate values.

Round brackets must be used around and may only be used around:

-   Formulas (a4b9)
-   Ordered tuples (a,b)
-   Unordered tuples (a+b). _Formatting programs: the smallest element must go
    first. Parsing programs: accept any order._
