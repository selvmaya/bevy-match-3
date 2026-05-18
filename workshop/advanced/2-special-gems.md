# Special Gems

**Goal:** Introduce a special gem type (a bomb that clears a 3×3 area, or a
line-clear that wipes its row and column) and weave it into the existing
match-and-cascade pipeline.

**You'll learn:** how to extend `Grid` data without breaking match detection,
and where in the cascade loop a "side-effect clear" fits.

## Steps

1. **Decide the spawn rule.** Classic match-3: a clear of 4 produces a
   line-clear; a clear of 5 produces a bomb. In `process_cascade`, after
   you compute `matched` and before you despawn, look at the contiguous
   runs that made up the match (you may need to recompute these, since
   `find_all_matches` returns a flat set) and spawn the special at one of
   the cleared cells, typically the cell the player swapped into.

2. **Expand clears on trigger.** When you despawn matched cells, check each
   one for a `SpecialKind`. For every special caught in the clear, compute
   the extra cells it affects (3×3 neighbourhood, full row+column, etc.)
   and union them into the clear set _before_ despawning. Be careful about
   handling chain reactions.

3. **Score and audio.** Decide what a bomb is worth. Probably more than
   the cells it clears, since the player worked to make it. Add a distinct
   `SoundEffect::Bomb`, or just play a louder `Match`.

4. **Make it visible.** A special gem needs to look different from a normal
   one even before it triggers. The cheapest path is a tint or an extra
   child sprite on top of the base gem.

## Stretch

- Two special types that interact: bomb + line-clear swapped together
  could clear a 3-wide band across the board.
- Special gems that survive a single match and stay on the board until
  manually triggered by a swap.
