# Gem Sprites

**Goal:** Swap the flat coloured rectangles for sprites, and rework the
"selected" appearance so the player can still tell which gem is picked.

**You'll learn:** how `Sprite::from_image` differs from `Sprite::from_color`,
how `Handle<Image>` is shared across entities for free.

## Steps

1. **Find or draw your art.** The simplest path is seven 64×64 PNGs, one per
   `GemType`. A single sprite sheet plus a `TextureAtlas` is the more
   idiomatic Bevy answer; pick whichever you're comfortable with. Save under
   `assets/images/gems/` and credit in `assets/images/CREDITS.md`.
   https://kenney.nl/assets/puzzle-pack-2 for example.

2. **Load the handles once.** Add a `GemTextures` resource, populated on
   `Startup` from the `AssetServer`. Use it as a lookup keyed by `GemType`.
   Loading per-frame inside `sync_gem_colors` would still work, but the
   first frame would stall on disk I/O.

3. **Rewrite the spawn-time sync.** `sync_gem_colors` should now build a
   `Sprite::from_image(handle)` (or `Sprite::from_atlas_image` for a sheet).
   `GEM_SIZE` is no longer enforced by the sprite, so either size the source
   art correctly or set `Sprite::custom_size`.

4. **Rethink "selected".** A few options, increasing in effort:
   - Tint via `Sprite::color` toward white or yellow.
   - Spawn a child entity with a halo/outline sprite, despawn on deselect.
   - Animate scale: a small pulse driven from `Time`.

## Stretch

- Tween the gem on selection (`Transform.scale` lerping over 0.1s).
