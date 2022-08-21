# Game protocol definition

Version 1, draft.

This document describes the game protocol for Hexil. This protocol is intended
to transmit game state and actions to the server and between players. This
document describes the functions to be performed by this protocol, and how
these functions should be performed. This document is encoding-agnostic, but
examples will be encoded as JSON. How a connection is established is left out
as an implementation detail.

## Game establishment

As soon as the player establishes a connection, they will be assigned a unique
identifier, which will be announced to the player using an **id** message. Once
three players have an active connection with the server, the server will
initiate the game, and send the order in which players get a turn using an
**order** message.

## Executing turns

### Ending a turn

To end a turn, the player who's turn it is must send an **endturn** message. If
the server receives an **endturn** message from the player who's turn it is, it
will broadcast this message to the other players. If the server receives an
**endturn** message from a player who's turn it not currently is, the message
will be discarded.

## Game end

When a player disconnects by breaking their connection, the server will end the
game and send all players an **endgame** message.

## Messages

#### `id`

Tells the player what their unique ID is.

Message parameters:

-   opcode: `id` (`string`)
-   id: Their unique ID (`int`)

#### `order`

Tells the players the order in which they get a turn.

Message parameters:

-   opcode: `order`
-   order: The order (ordered tuple of type `int`)

#### `endturn`

When sent to the server, tells the server that a player has ended their turn.
When sent to a player, tells the player that the person who's turn it was has
ended their turn.

Message parameters:

-   opcode: `endturn`

### `endgame`

Tells the player the game has ended.

Message parameters:

-   opcode: `endgame`
