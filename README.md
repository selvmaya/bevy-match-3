# Bevy Match-3 (RustWeek 2026 Workshop)

A bog-standard Match-3 game built with [Bevy](https://bevy.org/) for [RustWeek 2026](https://2026.rustweek.org/).
Polished and playable, but intentionally simple so workshop participants can read, understand, and extend it.

<img width="636" height="724" alt="image" src="https://github.com/user-attachments/assets/e63b9e4a-b6d7-42e4-829e-21f734b164de" />

## Expected Background

- Some programming experience in any language
  - You understand for loops, functions, modules, building and running programs
  - You're comfortable reading compiler errors
  - You have written at least one small project of your own in any language
- Basic Rust exposure
  - You can read function signatures
  - You know what `struct`, `enum`, and simple traits are
- No prior Bevy or game development experience required
- No math or art skills needed

## Understanding this Codebase

For the most part, this project is documented and commented as if it were a real, production game.
As a result, if you're new to Bevy, you should expect to supplement it with other learning materials.

Still, we make a few concessions.
To make this more useful as a learning tool, `src/main.rs` provides a high-level codebase overview,
and `game_logic.rs` contains an ASCII gameplay diagram.
In a few places, key learning points are explicitly called out to explore the implications of critical Bevy patterns.

When you encounter a concept you are not familiar with, look it up!
Check out Bevy's [Book](https://bevy.org/learn/book/intro/)
for background information on the key patterns used here.
To look up information on specific APIs, use `cargo doc`, use your IDE's tooltips, or visit Bevy's [docs.rs page](https://docs.rs/bevy/latest/bevy/).
When you're ready to add new features, the [Bevy examples](https://github.com/bevyengine/bevy/tree/latest/examples)
are very helpful for understanding Bevy's tools in a simplified but working context.

If you are completing this workshop in person,
please do not hesitate to ask for help from the instructors or your peers.
If you are exploring this remotely, your best bet is the [Bevy Discord](https://discord.gg/bevy).
It's full of friendly, helpful people who were in exactly the same position as you not long ago.

Indulge your curiosity: the point is to explore and learn!

## Getting Started

- **Rust** (stable, 1.92 or later), install via [rustup](https://rustup.rs/).
- On Linux you may need extra system libraries for audio/windowing.
  See [linux_dependencies.md](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

1. Create your own fork of this [template repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template).
2. Clone your fork.
3. Open a terminal in your new project's directory and use `cargo run` to try out the game!

The first build compiles Bevy and dependencies, so it can take a few minutes.
Subsequent builds are much faster due to incremental compilation.

### Dev Mode

Run with the `dev-mode` feature to improve diagnostics on errors:

```bash
cargo run --features dev-mode
```

You can add additional dev tools to this feature flag as you need them!

## Reading Guide

The code is commented and meant to be read.
While the initial build compiles, open [`src/main.rs`](src/main.rs) and follow
the reading order in its module doc through the rest of the sources.

### Bevy concepts, with a place to see each

A pointer into the codebase for each core Bevy concept. If a term is new, open
the listed file and read the surrounding comments.

| Concept                                                                                                                                                                                                                                   | Where to look                                                                                                                                                                           |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **[Plugin]**: bundles systems, resources, and messages into a reusable unit                                                                                                                                                               | `audio.rs` (smallest), then `game_logic.rs` and `selection.rs`                                                                                                                          |
| **App setup** ([`App`][App]): registering plugins, states, sets, and schedules                                                                                                                                                            | `main.rs::main`                                                                                                                                                                         |
| **[Entity]**: spawning, inserting components, despawning                                                                                                                                                                                  | `cursor.rs::spawn_cursor`, `game_logic.rs::apply_gravity_and_refill`, `audio.rs::play_audio_cues`                                                                                       |
| **[Component]**: data attached to entities                                                                                                                                                                                                | `board.rs::GridPos`, `gems.rs::GemType`, `cursor.rs::BoardCursor`                                                                                                                       |
| **Marker [component][Component]**: empty struct used as a query filter                                                                                                                                                                    | `gems.rs::Falling` (read `animate_falling_gems` to see how it gets queried and removed)                                                                                                 |
| **Required components** ([`#[require(...)]`][RequiredComponent]): auto-insert sibling components                                                                                                                                          | `gems.rs::GemType` (inserts `Sprite`, `GridPos`, `Pickable`, `DespawnOnExit`); `board.rs::GridPos` (requires `Transform`)                                                               |
| **[Resource]**: global, singleton data                                                                                                                                                                                                    | `board.rs::Grid` (authoritative board state), `game_logic.rs::Score`, `selection.rs::Selection`, `audio.rs::AudioHandles`                                                               |
| **System**: a plain function that takes system parameters                                                                                                                                                                                 | every plugin has a few; `selection.rs::process_selection` is a representative one                                                                                                       |
| **System parameters** ([`Res`][Res], [`ResMut`][ResMut], [`Query`][Query], [`Single`][Single], [`Commands`][Commands], [`Time`][Time], [`MessageReader`][MessageReader], [`MessageWriter`][MessageWriter], [`NextState`][NextState], ...) | `game_logic.rs::process_swap` and `process_cascade` use many at once                                                                                                                    |
| **[Query]**: iterate components on matching entities                                                                                                                                                                                      | `selection.rs::highlight_selected_gem` (`Query<(Entity, &GemType, &mut Sprite)>`)                                                                                                       |
| **Query filters** ([`With`][With], [`Without`][Without], [`Added`][Added], [`Changed`][Changed])                                                                                                                                          | `gems.rs::snap_new_gem_transforms` (`Added<GridPos>, Without<Falling>`); `gems.rs::animate_falling_gems` (`With<GemType>, With<Falling>`)                                               |
| **Change detection** ([`Added`][Added], [`Ref`][Ref]`::is_changed`, [`resource_changed`][resource_changed])                                                                                                                               | `cursor.rs::sync_cursor_transform` (`Ref` + `is_changed`); `in_game_ui.rs` and `selection.rs` (`run_if(resource_changed::<...>)`)                                                       |
| **Singletons** ([`Single<...>`][Single] for "exactly one" entities)                                                                                                                                                                       | `cursor.rs::sync_cursor_transform`, `input.rs::move_cursor_with_keyboard`                                                                                                               |
| **[Messages][Message] / [Events][Event]**: decoupled producer/consumer communication                                                                                                                                                      | `input.rs::SelectAction` -> `selection.rs::process_selection` -> `selection.rs::SwapMessage` -> `game_logic.rs::process_swap`. `audio.rs::SoundEffect` is the simplest complete example |
| **[States] & [SubStates]**: high-level mode switches                                                                                                                                                                                      | `main.rs::ScreenState` (state) and `GameState` (substate of `InGame`)                                                                                                                   |
| **[System sets][SystemSet] & ordering** (`.chain()`, `.after()`, `.in_set(...)`)                                                                                                                                                          | `main.rs` configures `GameSystems::{Input, Logic, AudioVisual}`; `input.rs::InputPlugin` shows finer-grained ordering within a plugin                                                   |
| **Run conditions** (`run_if(`[`in_state`][in_state]`(...))`, [`resource_changed`][resource_changed], custom closures)                                                                                                                     | `game_logic.rs::GameLogicPlugin`; `input.rs::InputPlugin` (includes a closure run condition)                                                                                            |
| **[`Commands`][Commands]**: deferred spawn / insert / despawn                                                                                                                                                                             | `game_logic.rs::process_cascade` (despawn); `apply_gravity_and_refill` (spawn)                                                                                                          |
| **Schedules** ([`Startup`][Startup], [`Update`][Update], [`OnEnter`][OnEnter], [`OnExit`][OnExit])                                                                                                                                        | `audio.rs` (`Startup` for asset load); `cursor.rs` and `in_game_ui.rs` (`OnEnter(ScreenState::InGame)`)                                                                                 |
| **[`DespawnOnExit`][DespawnOnExit]**: automatic cleanup tied to a state                                                                                                                                                                   | `gems.rs::GemType`, `cursor.rs::BoardCursor`, `in_game_ui.rs::setup_in_game_ui`                                                                                                         |
| **Asset loading** ([`AssetServer`][AssetServer], [`Handle<T>`][Handle])                                                                                                                                                                   | `audio.rs::load_audio_handles`                                                                                                                                                          |
| **Picking** ([`Pickable`][Pickable] + [`Pointer`][Pointer]`<Click>` messages)                                                                                                                                                             | `gems.rs::GemType` (opt-in via required components); `input.rs::move_cursor_with_mouse` and `confirm_with_mouse`                                                                        |
| **`[bsn!][bsn]` scene macro**: declarative spawning of UI/scene trees                                                                                                                                                                     | `in_game_ui.rs::setup_in_game_ui`                                                                                                                                                       |

[App]: https://docs.rs/bevy/latest/bevy/prelude/struct.App.html
[Plugin]: https://docs.rs/bevy/latest/bevy/prelude/trait.Plugin.html
[Entity]: https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html
[Component]: https://docs.rs/bevy/latest/bevy/prelude/trait.Component.html
[RequiredComponent]: https://docs.rs/bevy/latest/bevy/prelude/trait.Component.html#required-components
[Resource]: https://docs.rs/bevy/latest/bevy/prelude/trait.Resource.html
[Commands]: https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html
[Query]: https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html
[Single]: https://docs.rs/bevy/latest/bevy/prelude/struct.Single.html
[Res]: https://docs.rs/bevy/latest/bevy/prelude/struct.Res.html
[ResMut]: https://docs.rs/bevy/latest/bevy/prelude/struct.ResMut.html
[Time]: https://docs.rs/bevy/latest/bevy/prelude/struct.Time.html
[Message]: https://docs.rs/bevy/latest/bevy/prelude/trait.Message.html
[MessageReader]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageReader.html
[MessageWriter]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageWriter.html
[Event]: https://docs.rs/bevy/latest/bevy/ecs/event/trait.Event.html
[NextState]: https://docs.rs/bevy/latest/bevy/prelude/enum.NextState.html
[States]: https://docs.rs/bevy/latest/bevy/prelude/trait.States.html
[SubStates]: https://docs.rs/bevy/latest/bevy/prelude/trait.SubStates.html
[OnEnter]: https://docs.rs/bevy/latest/bevy/prelude/struct.OnEnter.html
[OnExit]: https://docs.rs/bevy/latest/bevy/prelude/struct.OnExit.html
[in_state]: https://docs.rs/bevy/latest/bevy/prelude/fn.in_state.html
[DespawnOnExit]: https://docs.rs/bevy/latest/bevy/prelude/struct.DespawnOnExit.html
[With]: https://docs.rs/bevy/latest/bevy/prelude/struct.With.html
[Without]: https://docs.rs/bevy/latest/bevy/prelude/struct.Without.html
[Added]: https://docs.rs/bevy/latest/bevy/prelude/struct.Added.html
[Changed]: https://docs.rs/bevy/latest/bevy/prelude/struct.Changed.html
[Ref]: https://docs.rs/bevy/latest/bevy/prelude/struct.Ref.html
[resource_changed]: https://docs.rs/bevy/latest/bevy/prelude/fn.resource_changed.html
[SystemSet]: https://docs.rs/bevy/latest/bevy/prelude/trait.SystemSet.html
[AssetServer]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html
[Handle]: https://docs.rs/bevy/latest/bevy/prelude/enum.Handle.html
[Pickable]: https://docs.rs/bevy/latest/bevy/prelude/struct.Pickable.html
[Pointer]: https://docs.rs/bevy/latest/bevy/prelude/struct.Pointer.html
[Startup]: https://docs.rs/bevy/latest/bevy/prelude/struct.Startup.html
[Update]: https://docs.rs/bevy/latest/bevy/prelude/struct.Update.html
[bsn]: https://dev-docs.bevy.org/bevy/prelude/macro.bsn.html

## How to Play

Match 3+ gems of the same color in a row or column to clear them.
Cleared gems are replaced by new ones falling from above.
New matches from falling gems cascade automatically until the board stabilizes.
If no valid moves remain, the game ends.

**Score:** 10 points per cleared gem.

## Controls

| Action              | Keyboard                  | Mouse          | Gamepad           |
| ------------------- | ------------------------- | -------------- | ----------------- |
| Move cursor         | Arrow keys / WASD         | —              | D-Pad             |
| Select / confirm    | Space or Enter            | Left click     | South (A / Cross) |
| Deselect            | Select the same gem again | Click same gem | South on same gem |
| Restart (game over) | Enter or Space            | —              | South (A / Cross) |

## Workshop Ideas

Some directions to explore once you're comfortable with the codebase.

### Beginner

- **New gem colour:** add a new colour variant to `GemType`.
- **Bonus points for large clears:** award more points when many gems are cleared in one step.
- **Move counter:** track and display the number of swaps made.
- **High score:** persist the best score to disk across sessions.
- **Background:** find or make a background image, and place it behind the game board.
- **Angry wiggles:** make the gems shake angrily when an invalid move is made.
- **New font:** find a better font, add it to the assets folder, and use it.

### Intermediate

- **Scoring multipliers:** award bonus points for cascade chains.
- **Timer mode:** add a countdown and a game-over screen. For a different flavour, try a limited moves budget instead.
- **Background music:** add a looping music track that plays during a game.
- **Pause menu:** pause the game on Escape and show a menu.
- **Settings:** add a settings menu to control volume, grid size and number of gem colors.

### Advanced

- **Gem sprites:** replace the colored rectangles with sprites of your choice, and update the selection appearance to match.
- **Special gems:** add a bomb or line-clear gem type with a unique clearing effect.
- **Shuffle when stuck:** detect when no valid moves remain and shuffle the board.
- **Particle effects:** spawn short-lived particle entities when gems are cleared.
- **Hint system:** after a few seconds of inactivity, highlight a valid move.
- **Game AI:** add a mode where the game makes matches automatically.
