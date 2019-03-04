# Replay

A spiritual successor to the puzzle game [Replay 2: The Sequel](https://jayisgames.com/review/cgdc3-replay-2-the-sequel.php)

## High Level Design
Turn-based gameplay<br/>
Player character is followed by a _shadow_ character at some _n_ turns removed.<br/>
Player character must hold doors for _shadow_, _shadow_ must hold doors for player.<br/>
Enemies in the form of objects that move in a straight line until colliding with a wall, returning along same path.<br/>
Some enemies collide with doors, others do not.<br/>
Switches can toggle objects (initially just doors).
Switches toggleable by player, _shadow_, enemies.<br/>
Teleports will, on entering, teleport a player, _shadow_, or enemy to a specified location.<br/>

## UX ideas
The original had a couple main complaints.<br/>

Input must feel responsive.<br/>
Undo/backup functionality should exist.<br/>

## Engine design
Every move could create a world state, player could go to any previous move state.<br/>
Previous attempts at undo functionality recorded input and simply restarted the level and replayed the input back to the desired undo point. This is a bit heavy.<br/>

## Platform
Windows/Linux<br/>
In browser via _wasm_ would be nice<br/>
