# New Font

**Goal:** Replace Bevy's default font with one of your own choosing across
every piece of text in the game.

**You'll learn:** how Bevy loads fonts as assets, how `Handle<Font>` is
shared cheaply, and where every piece of text in this codebase lives.

## Where to look

- `src/menu.rs`: the title, the Play / Quit buttons (line 100), and the
  game-over heading + score + Main Menu button (line 135). Lots of
  `TextFont { font_size, ..default() }` to update.
- `src/in_game_ui.rs`: the score counter (line 30).

## Steps

1. **Find a font.** Pick something free and openly licensed; the SIL Open
   Font License is the safest bet. Google Fonts, Bunny Fonts, and Itch.io
   game-font packs are all good sources. A pixel font like
   _[Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)_ matches
   the game's blocky aesthetic. Avoid anything restrictive: "free for personal
   use" usually isn't compatible with redistribution.

2. **Add it to assets and credit it.** Save the file as `assets/fonts/your_font.ttf`
   The `.ttf` extension matters: Bevy auto-detects format from the path.

3. **Use the handle everywhere text is built.** Find each occurrence of:

   ```rust
   TextFont { font_size: px(X) }
   ```

   and replace it with:

   ```rust
   TextFont {
       font_size: px(X),
       font: FontSourceTemplate::Handle("fonts/PressStart2P-Regular.ttf"),
   }
   ```

## Stretch

- Use **two** fonts: a fancy display font for the title and game-over
  heading, a clean readable one for the score and button text.
- Animate the title text by reading `Time` and modulating `TextColor`;
  satisfying once you have a font you like.
