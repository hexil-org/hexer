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
Once three players have joined, the server will initiate the game by randomly
assigning each player a unique identifier from `0` to `2`. This identifier
will be announced to the player by sending them an **id** message. The player
with identifier `n` will henceforth be called _player n_. For example, _player
2_ is the player with identifier `2`.

## Executing turns

### Order of turns

Player 0 will always get the first turn. After player 0 has ended their turn, it
will be player 1's turn. After player 1 has ended their turn, it will be player
2's turn, and so on. In general, after player _n_ has ended their turn, it will be
player _n + 1 % 3_'s turn.

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

Tells the player what their unique assigned ID is.

Message parameters:

-   opcode: `id` (`string`)
-   id: Their unique assigned ID (`int`)

### `endturn`

Signals that a player has ended their turn.

Message parameters:

-   opcode: `endturn`

### `endgame`

Tells the player the game has ended.

Message parameters:

-   opcode: `endgame`
