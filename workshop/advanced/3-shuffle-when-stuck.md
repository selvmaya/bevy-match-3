# Shuffle When Stuck

**Goal:** When the board has no valid swap left, shuffle the gems in place
instead of ending the run. The player should feel a brief "reset" moment,
not a hard stop.

**You'll learn:** how to reuse `Grid::has_any_valid_move` for something
beyond the game-over check, and the subtle difference between shuffling
entities and shuffling kinds.

## Steps

1. **Add the shuffle helper.** On `Grid`, add a `shuffle` method that
   permutes the contents of all occupied cells. You're moving (entity, kind)
   pairs together: a gem entity keeps its `GemType` but lands at a new
   `GridPos`.

2. **Loop until playable.** A random shuffle can land on another stuck
   layout, or on a layout that already has a match. Wrap the shuffle in a
   loop: shuffle, then reject if `has_any_valid_move` is false **or** if
   `find_all_matches` is non-empty (the board shouldn't cascade after a
   shuffle).

3. **Wire it into the cascade.** Replace `next_state.set(GameState::GameOver)`
   with a call to your shuffle. `sync_gridpos_from_grid` already runs at the
   end of the logic step and inserts `Falling` on any entity whose cell
   changed, so you get the animation for free. Stay in `Animating` for one more
   pass so the falls finish before play resumes.

4. **Tell the player.** A sound cue and a brief on-screen "Shuffle!" text
   give the player something to notice. The audio pattern is the same as
   `SoundEffect::Match`; the text can be a one-shot `Text` entity with a
   short-lived `Lifetime` component, or `DespawnOnExit` keyed to a state
   you transition through.

## Stretch

- Animate the shuffle as a full board-wide swap (fade out, scramble, fade
  in) instead of letting each gem `smooth_nudge` independently.
- Cap the number of shuffles per run, then game-over when the cap is hit.
