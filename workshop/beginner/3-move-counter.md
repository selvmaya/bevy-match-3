# Move Counter

**Goal:** Track and display the number of swaps the player has made.

**You'll learn:** the resource-to-UI pattern in Bevy, and how
`resource_changed` keeps text updates cheap by skipping frames where nothing
changed.

## Where to look

- `src/game_logic.rs`: `Score` (line 58-61) is the template to copy.
  `process_swap` (line 80) is where every swap is resolved.
- `src/in_game_ui.rs`: `Score` is rendered in the top-left corner; mirror
  this for the move count in the top-right.

## Steps

1. **Define the resource.** In `game_logic.rs`, next to `Score`:

   ```rust
   #[derive(Resource, Default, Debug)]
   pub struct MoveCount {
       pub value: u32,
   }
   ```

   Add a `Display` impl so it formats nicely for the UI:

   ```rust
   impl fmt::Display for MoveCount {
       fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           write!(f, "Moves: {}", self.value)
       }
   }
   ```

2. **Initialize and reset it.** In `GameLogicPlugin::build`:
   - Call `.init_resource::<MoveCount>()` alongside `Score`.
   - Add a `reset_move_count` system on `OnEnter(ScreenState::InGame)`, copying
     the shape of `reset_score`.

3. **Increment it.** Increment in the `else` branch of the
   `for msg in swap_messages.read()` loop in `process_swap`.

4. **Display it.** In `in_game_ui.rs`:
   - Add a `MoveCountText` marker component, mirroring `ScoreText`.
   - In `setup_in_game_ui`, spawn a second `bsn!` scene mirroring the one for
     `ScoreText`. Change the position so that it's anchored on the top right of
     the screen (`right` instead of `left`).
   - Add a `update_move_count_text` system in `InGameUiPlugin` gated by
     `resource_changed::<MoveCount>`:

     ```rust
     fn update_move_count_text(
         moves: Res<MoveCount>,
         mut text: Single<&mut Text, With<MoveCountText>>,
     ) {
         text.0 = moves.to_string();
     }
     ```

## Stretch

- Show _matches per move_: display the running ratio as an efficiency stat.
