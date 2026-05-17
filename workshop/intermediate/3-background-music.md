# Background Music

**Goal:** Play a looping music track during gameplay, and stop it when the
player leaves the game screen.

**You'll learn:** the difference between a one-shot sound entity and a
long-lived audio entity, and how `PlaybackSettings` controls looping and
cleanup.

## Where to look

- `src/audio.rs`: `play_audio_cues` shows the one-shot pattern: spawn an
  `AudioPlayer` with `PlaybackSettings::DESPAWN`. Background music is the
  opposite: spawn it once, keep it around, despawn explicitly when the run
  ends.

## Steps

1. **Find a track.** Something looping, low-key, openly licensed. Incompetech,
   Free Music Archive, and OpenGameArt are good sources. Save it as
   `assets/audio/music.ogg` (convert it if needed) and add it to `CREDITS.md`.

2. **Spawn on entering gameplay.** Add a `start_music` system on
   `OnEnter(ScreenState::InGame)` in `AudioPlugin`. Use:

   ```rust
   PlaybackSettings::LOOP
   ```

   Attach `DespawnOnExit::<ScreenState>(ScreenState::InGame)` so the music
   stops automatically when the player exits to the main menu.

3. **(Optional) Pre-load the handle.** Mirror `load_audio_handles` for the
   music asset so the first play doesn't stall on disk I/O.

## Stretch

- Duck the music volume briefly when a match clears (a classic juice trick).
- A different track on the main menu, started on `OnEnter(ScreenState::MainMenu)`.
- Crossfade between menu and gameplay tracks instead of a hard cut. Two
  `AudioPlayer` entities, each with `PlaybackSettings::LOOP`, and a system
  that adjusts each one's `AudioSink::set_volume` over a short window.
