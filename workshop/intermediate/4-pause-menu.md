# Pause Menu

**Goal:** Pause the game when the player presses Escape, show an overlay
with Resume and Main Menu buttons, and freeze gameplay while it's open.

**You'll learn:** how to add a new variant to a `States` enum and gate
systems on it, and how `OnEnter` / `DespawnOnExit` make UI overlays
trivial to spawn and clean up.

## Where to look

- `src/main.rs`: add a `Paused` variant to the `GameState` enum.
- `src/input.rs`: `return_to_main_menu` currently grabs Escape and bounces
  straight to the menu. You'll want to replace that behaviour with a pause
  toggle.
- `src/menu.rs`: `setup_game_over_screen` is the closest template for what
  you're building: a full-screen overlay with buttons that emits state
  transitions on `Activate`.

## Steps

1. **Add the state.** Extend `GameState` with a `Paused` variant. The
   compiler's exhaustive `match` checks will point you at every place that
   needs updating.

2. **Toggle on Escape.** Replace `return_to_main_menu` with a toggle: if
   in `Playing`, go to `Paused`; if in `Paused`, go back to `Playing`.
   Keep it inside `GameSystems::Input`.

3. **Gate gameplay.** `process_swap` and `process_cascade` are already
   gated with `run_if(in_state(...))`, so they stop on their own. Double
   check the input systems in `InputPlugin::build`: the confirmation set
   already runs only in `Playing`, but cursor movement currently runs in
   `Playing | Animating`. Decide whether the cursor should be frozen too.

4. **Spawn the overlay.** Add a `setup_pause_menu` system on
   `OnEnter(GameState::Paused)` in `MenuPlugin`. Mirror
   `setup_game_over_screen`: a semi-transparent background, a heading,
   two buttons.
   - "Resume" calls `next_state.set(GameState::Playing)`
   - "Main Menu" calls `next_state.set(ScreenState::MainMenu)`

   Use `DespawnOnExit::<GameState>(GameState::Paused)` so the overlay
   tears itself down when you resume.

## Stretch

- Pause the in-game music (track down the music entity's `AudioSink` and
  call `pause()`); resume it on `OnExit(Paused)`.
- Show the current score on the pause overlay.
- Add a "Restart" button that transitions through the menu and back, so
  the board regenerates.
