# New Gem Colour

**Goal:** Add an eighth gem variant to the game.

**You'll learn:** how an `enum` is wired into rendering, random generation, and
matching logic, and how Rust's exhaustive `match` lets the compiler guide you
through the change.

## Where to look

- `src/gems.rs`: `GemType` enum (line 33), the `ALL` const array
  (line 44), and the `color()` method (line 54).

## Steps

1. **Add the variant.** Pick a name and add it to `GemType`, e.g. `Cyan`,
   `Teal`, or `Lime`.

2. **Pick a hue that stands out.** The existing seven gems live at HSL hues
   `0°, 27°, 54°, 133°, 224°, 276°, 348°`. The biggest visual gaps are around
   `90°` (yellow-green) and `190°` (cyan). Pick a hue in one of those gaps so
   players can tell your new gem apart at a glance. Add it to `fn color(self)`.

3. **Register it in `ALL`.** `random()` and `random_no_match()` both draw
   colors for gems from `ALL`.

## Stretch

- Add a _second_ new colour. How easily can your eye tell two greens apart on
  the board? This is where art direction starts mattering. Extra colors could
  be stripped instead fo a flat color, or be handled by a shader.
- Remove colors mode (5 instead of 7) by editing `ALL` and observe how the
  match frequency changes. With fewer colours, cascades become much more
  common, useful intuition for balancing your own design later.
