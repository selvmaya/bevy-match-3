# High Score

**Goal:** Remember the best score the player has achieved, across runs and
across sessions.

**You'll learn:** why blocking file I/O is the wrong tool inside a Bevy
system, and how to use Bevy's built-in `IoTaskPool` to do disk work off the
frame thread.

## Why not just call `std::fs::write` from a system?

Bevy systems run on the frame loop. If a system calls `std::fs::write` or
`std::fs::read_to_string`, the whole schedule waits for the disk before the
next system runs. Even a "fast" SSD write can stall for tens of milliseconds
when something else has the disk busy, and that's a visible hitch.

The fix is Bevy's [`IoTaskPool`](https://docs.rs/bevy/latest/bevy/tasks/struct.IoTaskPool.html):
a thread pool designed exactly for blocking I/O. The system spawns a small
async task onto the pool and returns immediately; the actual `std::fs::write`
runs on a worker thread that's allowed to block.

For reads at startup, we cheat in the opposite direction: do the read in
`fn main` _before_ `App::new()`. That's plain Rust code, not a system, so a
one-shot blocking read there is fine.

## Where to look

- `src/main.rs`: `fn main` is where you'll do the startup read.
- `src/game_logic.rs`: `Score` (line ~58) is the live value. Add a
  one-shot save system on `OnEnter(GameState::GameOver)`.

## Steps

1. **Define the resource.**

   ```rust
   #[derive(Resource, Default, Debug)]
   pub struct HighScore(pub u32);
   ```

2. **Load at boot, in `fn main`.** This is just regular Rust, no system, no
   frame loop:

   ```rust
   fn main() {
       let initial = std::fs::read_to_string("highscore.txt")
           .ok()
           .and_then(|s| s.trim().parse::<u32>().ok())
           .unwrap_or(0);

       App::new()
           // ... existing setup ...
           .insert_resource(HighScore(initial))
           .run();
   }
   ```

   A missing or malformed file is fine: `unwrap_or(0)` handles both. First
   run starts at zero.

3. **Save on game over, on the task pool.** Add this system on
   `OnEnter(GameState::GameOver)`:

   ```rust
   use bevy::tasks::IoTaskPool;

   fn save_high_score(
       score: Res<Score>,
       mut high_score: ResMut<HighScore>,
   ) {
       if score.value <= high_score.0 {
           return;
       }
       high_score.0 = score.value;

       // Move a copy into the task; the system itself returns immediately.
       let to_write = score.value;
       IoTaskPool::get()
           .spawn(async move {
               if let Err(e) = std::fs::write("highscore.txt", to_write.to_string()) {
                   warn!("failed to save high score: {e}");
               }
           })
           .detach();
   }
   ```

   `.detach()` tells the pool "fire and forget": we don't care when the
   write finishes, only that it eventually does. The blocking `std::fs::write`
   is now running on a worker thread, not the frame thread.

## Stretch

- A "New high score!" flash on the game-over screen when the player beat
  their previous best.
- Show the current high score in the game menu
