# Particle Effects

**Goal:** When gems clear, spawn a few short-lived sprites at each cleared
cell, fading and shrinking until they despawn. The match should feel like
something happened, not just like the gem vanished.

**You'll learn:** the "spawn many small entities, despawn on lifetime
expiry" pattern, and how to keep per-particle systems cheap when there are
hundreds of them at once.

## Steps

1. **Define the component.** A `Particle` component carrying
   `velocity: Vec2`, `lifetime: f32`, and `elapsed: f32` is enough for a first
   pass. Build a plugin (`ParticlePlugin`) that registers the update system in
   `GameSystems::AudioVisual`.

2. **Spawn at the clear site.** In `process_cascade`, for each cleared
   cell, spawn N small `Sprite`s (8 is a good starting number) at
   `pos.to_world()`. Give each a random outward velocity (polar coords are
   easiest: `Vec2::from_angle(theta) * speed`) and a short lifetime
   (0.4-0.8s feels right). Tint with the gem's `color()`.

3. **Update each frame.** One system, marker query `With<Particle>`:
   - Advance `elapsed` by `time.delta_secs()`.
   - Translate by `velocity * dt`. Add a touch of gravity if you want them
     to arc.
   - Optional: scale and alpha as `1.0 - elapsed / lifetime`. Multiply
     `Sprite::color`'s alpha rather than building a separate fade
     component.
   - When `elapsed >= lifetime`, `commands.entity(e).despawn()`.

4. **Watch the count.** A 6×3 match emits 6×8 = 48 particles per step, and
   chains stack. If you ever exceed a few hundred live at once and the
   frame rate dips, that's the first place to look: cap N, or skip the
   alpha fade in favour of a hard despawn.

## Stretch

- A `ParticleBurst` event: any system can write a `(position, color, count)`
  message, and a single spawner consumes them. The cascade becomes one writer
  of many.
- Trail effect: leave a few faded copies of each particle behind by
  spawning child sprites at a lower update rate.
