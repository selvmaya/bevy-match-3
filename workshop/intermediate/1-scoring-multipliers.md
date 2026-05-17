# Scoring Multipliers

**Goal:** Award bonus points for cascade chains. The first clear of a swap
scores normally; the second cascade step scores ×2, the third ×3, and so on.

**You'll learn:** how to carry state across frames in a `Resource`, and how
to use states to handle different actions in a frame.

## Where to look

- `src/game_logic.rs`: `process_cascade` (line 112) runs once per cascade
  step. The transition from `Animating` back to `Playing` (line 130) is the
  end of a chain, so the chain count needs to reset there.
- `src/game_logic.rs`: `process_swap` (line 80) is the other place a chain
  can start: a successful swap kicks off chain step 1.

## Steps

1. **Track the chain.** Add a small `ChainCount` resource alongside `Score`.
   `init_resource` it in `GameLogicPlugin::build` and reset it whenever a
   new game starts.

2. **Increment per cascade step.** In `process_cascade`, bump the counter
   each time `matched` is non-empty. Reset it on the `next_state.set(Playing)`
   branch.

3. **Apply the multiplier.** The current line is:

   ```rust
   matched.len() as u32 * POINTS_PER_CLEARED_GEM
   ```

   Multiply by the current chain count. Decide whether you want linear
   (`×n`), capped (`×min(n, 5)`), or something more exotic.

## Stretch

- Show the current chain count as floating text near the board while a
  cascade is in flight, fading out when the chain ends.
- Combine with the **bonus for large clears** beginner exercise: pick a rule
  for how the two bonuses stack (additive? multiplicative?).
- Play a higher-pitched match sound on each successive chain step. The
  existing `SoundEffect::Match` is a single variant, so you'd need to either
  add new variants or pass a pitch alongside the message.
