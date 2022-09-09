# HSN: Hexil State Notation

HSN consists of multiple parts. The parts muts be separated by whitespace and
must occur in the same order as in the document.

## Static

### Tile kinds

Axial, row-column based.

```
3SSSS/2SOWLS/1SGBWBS/SGLDLOS/SLOGWS1/SBGWS2/SSSS3
```

### Resource numbers

For each B, G, L, O or W in the tile kinds, give the corresponding number.

```
10,2,9/12,6,4,10/9,11,3,9/8,3,4,5/5,6,11
```

### Harbor locations

Comma separated. Coordinates are sorted row-column. Optionally first a resource
and a dash.

```
d2nw,e2nw,c3w,O-f3ne,B-g4w,W-b5w,L-e6nw,a7ne,G-c7nw
```

## Dynamic

### Village locations

A list of locations, comma separated. Coordinates are sorted row-column. A
slash separates players.

```
c3n,d5s/d3n,c5n/c4n
```

### City locations

Exactly like the village locations.

```
//e3n
```

### Road locations

A list of locations, comma separated. Coordinates are sorted row-column. A
slash separates players.

```
d2w,d6nw/e2w,d4w/f2w,c4ne
```

### Robber location

Just the coordinate.

```
d4
```
