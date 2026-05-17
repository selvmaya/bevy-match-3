# Timer Mode

**Goal:** Add a countdown timer (or a moves budget) that ends the game when
it hits zero, sending the player to the existing game-over screen.

**You'll learn:** how `Time` and `Timer` work in Bevy, and how to drive a
state transition from a system that isn't tied to player input.

## Where to look

- `src/main.rs`: `GameState::GameOver` is the state to push
  to when the timer expires.
- `src/game_logic.rs`: `reset_score` is the template for a
  per-run reset system on `OnEnter(ScreenState::InGame)`.
- `src/in_game_ui.rs`: `ScoreText` is the template for displaying
  a value that updates each frame.
- `src/menu.rs`: `setup_game_over_screen` already renders the
  end-of-run screen, so you don't need a new one.

## Steps

1. **Decide the rule.** A 60-second countdown, a 30-move budget, or both.
   The shape of the resource changes accordingly, so pick one before writing
   code.

2. **Resource + reset.** Add a `TimeRemaining` (or `MovesLeft`) resource,
   `init_resource` it in `GameLogicPlugin`, and reset it on
   `OnEnter(ScreenState::InGame)`.

3. **Tick it.** For the timer flavour, decrement by `time.delta_secs()`
   every frame while `GameState::Playing` (and probably also `Animating`,
   so cascades don't pause the clock; try both and see which you prefer).
   For the moves flavour, decrement in `process_swap` when a swap
   succeeds.

4. **End the run.** When the value hits zero, `next_state.set(GameState::GameOver)`.
   The existing game-over screen will appear automatically.

5. **Display it.** Mirror the `ScoreText` setup in `in_game_ui.rs`. Anchor
   it somewhere that doesn't fight the score (top centre, or top right).
   Use `resource_changed::<TimeRemaining>` to keep the text update cheap.

## Stretch

- Add a few seconds back to the clock per cleared gem, so good play
  extends the run.
- Flash the timer red below 10 seconds.
- A "best time to N points" mode: timer counts up, run ends when the
  player crosses a score threshold.
- Show the final time on the game-over screen by extending
  `setup_game_over_screen` in `menu.rs`.
