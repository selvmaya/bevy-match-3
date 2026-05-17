# Bonus Points for Large Clears

**Goal:** Reward the player when many gems clear in a single step.

**You'll learn:** how to extend a small piece of pure game logic.

## Where to look

- `src/game_logic.rs`: `process_cascade`, line 143. The scoring line
  is:

  ```rust
  score.value = score.value.saturating_add(matched.len() as u32 * POINTS_PER_CLEARED_GEM);
  ```

  `matched` is a `HashSet<GridPos>` containing **every** cell cleared in this
  cascade step, across all simultaneous matches. A single move that clears
  one 3-run gives `len == 3`; a swap that triggers two separate 3-runs gives
  `len == 6`.

## Steps

1. **Decide what "large clear" means.** A few options, in order of effort:
   - **Flat bonus:** if `matched.len() >= 5`, add a fixed bonus.
   - **Tiered:** 3 cells = base, 4 = ×1.5, 5 = ×2, 6+ = ×3.
   - **Curved:** scale points super-linearly, e.g. `n * n * POINTS_PER_CLEARED_GEM / 3`.

   Pick one. You can always change it later.

2. **Extract a helper.** Replace the scoring line with a call to a small
   function. Keep the unit test boundary clean:

   ```rust
   fn points_for_clear(cell_count: usize) -> u32 {
       // your formula here
   }
   ```

   Keeping the formula in its own function makes it easy to write a test for,
   and easy to tune.

## Stretch

- Spawn a short-lived `Text` entity at the centre of the cleared region
  saying "+200!".
- Play a different sounds based on the number of points
