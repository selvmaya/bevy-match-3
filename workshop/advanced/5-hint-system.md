# Hint System

**Goal:** When the player has been idle for a few seconds, highlight one of
the gems involved in a valid swap. The hint should disappear as soon as the
player does anything.

**You'll learn:** how to drive a system from "time since last X" state,
and how to factor a piece of board-search logic so it can be used in two
places without duplication.

## Steps

1. **Pull out the test.** Refactor `has_any_valid_move` so the inner
   adjacent-pair check is its own function returning
   `Option<(GridPos, GridPos)>` (the swap that would create a match).
   `has_any_valid_move` becomes `.is_some()`. Add a new
   `find_a_valid_move` method on `Grid` that returns the first one found.

2. **Track idle time.** A `LastInputAt(f32)` resource (or `IdleSince`)
   updated by a single system that reads input messages and cursor change
   events. Reset it on any input. Read `time.elapsed_secs()` for the
   timestamp; that's simpler than holding a `Timer`.

3. **Drive the hint.** A system that runs in
   `GameState::Playing` and only when `now - last_input > HINT_DELAY`
   (5-10s for example). On entering the hint state, call `find_a_valid_move`,
   pick one of the two cells, and mark it. Use a `Hinted` marker component so a
   separate visual system can handle the actual look.

4. **Visualise it.** Scale the gem's `Transform` with a sine of `Time` so it
   breathes. No colour conflict, easy to read.

5. **Clear it.** On any input, remove `Hinted` and restore the gem's
   normal appearance. The same system that resets `LastInputAt` is the
   natural place to drop the marker.

## Stretch

- Pulse-fade the hint instead of a hard on/off, so the gem fades in
  and out rather than popping.
- Track how often the player needs a hint and use it to surface a
  difficulty hint on the game-over screen.
