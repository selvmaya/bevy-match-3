# Settings Menu

**Goal:** Add a settings screen reachable from the main menu, with controls
for master volume, grid size, and the number of gem colours in play.

**You'll learn:** how to fan a single `Settings` resource out to multiple
subsystems, and which "constants" in this codebase are actually fine to
turn into runtime values.

## Where to look

- `src/board.rs`: `GRID_COLS` and `GRID_ROWS` are `const`.
  Read every use site of these to understand what becomes dynamic if you
  make them runtime values: `to_world`, the arrays in `Grid`, the loops
  in `setup_board`, `apply_gravity_and_refill`, and so on.
- `src/gems.rs`: `GemType::ALL` is what `random()` and `random_no_match()`
  draw from. Cap the slice at runtime instead of changing the array.
- `src/audio.rs`: `play_audio_cues` is the single funnel for every sound
  effect.
- `src/menu.rs`: `setup_menu` is the template for a new screen.

## Steps

1. **Define the resource.** A `Settings` resource with `volume: f32`,
   `grid_size: usize`, `gem_color_count: usize`. Insert it at startup
   with sensible defaults.

2. **Wire volume into audio.** In `play_audio_cues`, use
   `PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.volume))`.
   Do the same for the music if you also did the background-music exercise.

3. **Wire gem colour count.** Change `GemType::random()` /
   `random_no_match()` to take a `count: usize` and draw from
   `&Self::ALL[..count]`. Update the callers in `apply_gravity_and_refill`
   and `setup_board` to read from the resource. Clamp the count to
   `3..=ALL.len()`.

4. **Grid size.** Turn `GRID_COLS`/`GRID_ROWS` into a runtime value. Swap the
   fixed-size `[[T; COLS]; ROWS]` arrays in `Grid` for `Vec<Vec<T>>`. This
   touches every method on `Grid`.

5. **The screen.** Add a `ScreenState::Settings` variant and a new screen
   spawned on `OnEnter`. Each setting is a row with a label and either
   `+`/`-` buttons. The `menu_button` helper in `menu.rs` is directly reusable.
   Add a "Back" button that returns to `ScreenState::MainMenu`.

## Stretch

- A "difficulty" preset that bundles grid size + colour count + (if you
  did **timer mode**) starting time.
- Two volume sliders (music, SFX) instead of a single master.
- Save settings to a file and load it on start.
