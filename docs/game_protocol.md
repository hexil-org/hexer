# Game protocol definition

Version 1, draft.

This document describes the game protocol for Hexil. This protocol is intended
to transmit game state and actions to the server and between players. This
document describes the functions to be performed by this protocol, and how
these functions should be performed. This document is encoding-agnostic, but
examples will be encoded as JSON. How a connection is established is left out
as an implementation detail.

## Game establishment

A player will join the game when they establish a connection with the server.
As soon as a player joins, they will be assigned a unique identifier by the
server. This identifier will be announced to the player using an **id**
message. Once three players have joined, the server will initiate the game, and
send the order in which players get a turn using an **order** message.

## Executing turns

### Ending a turn

To end a turn, the player who's turn it is must send an **endturn** message. If
the server receives a _valid_ **endturn** message, it will broadcast this
message to the other players, otherwise it will be discarded. An **endturn**
message is considered _valid_ if and only if it originated from the player
who's turn it is.

## Game end

A player will leave the game when they break their connection with the server.
As soon as a player leaves, the server will end the game and send all remaining
players an **endgame** message.

## Messages

### `id`

Tells the player what their unique ID is.

Message parameters:

-   opcode: `id` (`string`)
-   id: Their unique ID (`int`)

### `order`

Tells the players the order in which they get a turn.

Message parameters:

-   opcode: `order`
-   order: The order (list of `int`)

### `endturn`

Signals that a player has ended their turn.

Message parameters:

-   opcode: `endturn`

### `endgame`

Tells the player the game has ended.

Message parameters:

-   opcode: `endgame`
