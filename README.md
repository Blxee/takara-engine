# Takara Engine

Is a Tak board game engine and CLI game build in rust

## ASCII board idea

```
     a     b     c     d     e
  +-----+-----+-----+-----+-----+
1 | F   |FFF  |     |     |     | 1
  +-----+-----+-----+-----+-----+
2 | W   |FFFW |     |     |     | 2
  +-----+-----+-----+-----+-----+
3 |     |FFC  |     |     |     | 3
  +-----+-----+-----+-----+-----+
4 |     |     |     |     |     | 4
  +-----+-----+-----+-----+-----+
5 |     |     |     |     |     | 5
  +-----+-----+-----+-----+-----+
     a     b     c     d     e
```

## Tak game configurations:

| Board Size   | 3x3 | 4x4 | 5x5 | 6x6 | 7x7 | 8x8 |
|--------------|-----|-----|-----|-----|-----|-----|
| Normal Stones| 10  | 15  | 21  | 30  | 40  | 50  |
| CapStones    | 0   | 0   | 1   | 1   | 2   | 2   |

## Input format:

### The input should enable the user to:
  - provide the cell position as a row number and a column letter (order doesn't matter)
  - either move a stone (or a stack) which is already in the board or put a new one
  - specify the amount of the stack to move
  - specify which stone type to put (either flat, standing or capstone)
  - provide which direction to break the stack towards

**Current format**: `<row><col>[amount]<direction(t,d,l,r)>`
