# Background

**Goal:** Place a background image behind the game board.

**You'll learn:** Bevy's asset loading, sprite z-ordering, and where in the
plugin lifecycle to spawn long-lived entities.

## Where to look

- `src/board.rs`: `setup_board` (line 315) spawns the gems on `OnEnter(ScreenState::InGame)`.
- `src/cursor.rs`: the cursor is spawned at `z = -0.5` (line ~58). Your
  background needs to sit **behind** that, so anything with `z < -0.5`.

## Steps

1. **Find or make an image.** Anything roughly 640×700 (the window size) or
   larger. Public-domain pixel-art backgrounds work well; itch.io and OpenGameArt
   have free packs. Save it under `assets/images/background.png`.

   Always keep credits: add an entry to a new `assets/images/CREDITS.md` that
   names the source, author, and licence, mirroring the existing
   `assets/audio/CREDITS.md`.

2. **Spawn it.** Add a `spawn_background` system on
   `OnEnter(ScreenState::InGame)` in `BoardPlugin` (or your own new plugin):

   ```rust
   fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
       commands.spawn((
           Sprite::from_image(asset_server.load("images/background.png")),
           Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
           DespawnOnExit(ScreenState::InGame),
       ));
   }
   ```

## Stretch

- Add a different background to the main menu (`OnEnter(ScreenState::MainMenu)`).
- Slow parallax: write a system that reads `BoardCursor` and nudges the
  background's `Transform.translation` slightly in response. A few pixels of
  movement is enough to feel "alive".
- Layer two backgrounds (e.g. sky behind clouds) at different z-depths and
  scroll them at different speeds.
