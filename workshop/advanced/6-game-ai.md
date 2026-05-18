# Game AI

**Goal:** Add an "autoplay" mode where the game plays itself: find a valid
swap, perform it, wait for the cascade to settle, then pick the next one.
Useful as a demo, a screensaver, or a baseline for evaluating scoring
tweaks.

**You'll learn:** how to drive the game from inside the simulation by
emitting the same messages a human player would, and how to gate work on
the existing `GameState` so the AI doesn't overlap with a running cascade.

## Steps

1. **Pick the trigger.** Add a `AutoplayEnabled(bool)` resource toggled from a
   keypress.

2. **Find a move.** Add a
   `Grid::find_a_valid_move(&self) -> Option<(GridPos, GridPos)>` method (reuse
   the one from the hint exercise). For a smarter AI, score each candidate swap
   by the size of the match it would create and pick the best. A random valid
   move is a fine v1.

3. **Pace it.** Add an `AutoplayCooldown(Timer)` resource that ticks every
   frame and only emits a `SwapMessage` when it finishes. Without a delay
   the AI fires a swap per frame.

## Stretch

- Smart AI: score each candidate by the cleared-cell count of the
  resulting match.
- Greedy AI: score each candidate by the cleared-cell count of the
  resulting match and cascades.
- A speed slider in the settings menu (combine with the **settings**
  intermediate exercise) that drives the cooldown duration.
