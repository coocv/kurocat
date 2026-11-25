# 🐈‍⬛ KuroCat · 2D Custom Branch (`feat/bellblack-2d`)

<p align="center">
  <a href="./README.md">🇨🇳 简体中文</a> &nbsp;|&nbsp;
  <strong>🇺🇸 English</strong> &nbsp;|&nbsp;
  <a href="./README.ja.md">🇯🇵 日本語</a>
</p>

A 2D custom branch of [qianyiai/kurocat](https://github.com/qianyiai/kurocat), version **5.2.0**. Keyboard/mouse following, the transparent always-on-top window, tray, password privacy guard, MCP and the 3D engine all work as upstream ships them; this branch simply keeps an extra cat in the 2D engine.

<p align="center">
  <img src="docs/2d/hero.webp" width="420" alt="Bellblack cat, idle pose" />
</p>

## The cat: Bellblack (`bellblack`)

A round-faced black cat wearing a thin leather cord with a small gold bell.

- **Round-cheek silhouette**: a `BODY_SLICES_ROUND` outline — wide, soft face with full cheeks
- **Fine cord + gold ball bell**: brick-red cord `#6e2f21` tapering to fine points at both ends; a radial-gradient gold bell with a flat oval sound hole
- **Single-stroke paws**: both front legs drawn as one continuous stroke, with full outline and paw-ball arc even on dark fur
- **Keycap-coloured headphones**: white-keycap earcups, no boom mic; while music plays a hand-drawn note floats overhead (single note or beamed double note)
- **Its own menu entry**: listed as "铃铛小黑" in both the tray menu and the in-app appearance menu; selecting it persists `catBreed` in `settings.json` and survives restarts

<table>
  <tr>
    <td align="center" width="50%"><img src="docs/2d/paw.webp" width="330" alt="Pose while the mouse is held" /><br/><sub>Mouse held: one paw braces on the ground, with a solid outline and paw-ball arc</sub></td>
    <td align="center" width="50%"><img src="docs/2d/desktop.webp" width="300" alt="Bellblack on the desktop" /><br/><sub>On the desktop: round cheeks, cord and gold bell, single-stroke paws, 2D keyboard</sub></td>
  </tr>
</table>

## Music groove

When the system plays music the whole cat moves with the beat:

- **Base groove**: head nod `beatSin*0.078 + beatHalf*0.026`, face sway `musicNodGain*beatHalf*3.6`, weight shifting via `musicGroove*sin(phase*0.5)*0.055`, tail swinging at half frequency (amplitude 6.4)
- **Random gestures**: every 2200–5200 ms one of `enjoy / enjoy / ear / slowBlink / look / tail / tilt`, with BPM randomised between 92 and 126 per session
- **Typing wins**: nod gain is multiplied by `(1 - min(1, typingHunch)*0.55)` so typing keeps its own rhythm

<table>
  <tr>
    <td align="center" width="56%"><img src="docs/2d/music.webp" width="360" alt="Listening: keycap headphones + single note" /><br/><sub>Headphones + one note, laid out on two alternating lanes</sub></td>
    <td align="center" width="44%"><img src="docs/2d/groove.webp" width="420" alt="Six-frame groove sequence" /><br/><sub>Six frames: head tilting, weight moving side to side, tail following</sub></td>
  </tr>
</table>

## Eyes-shut bliss

- `IdleFidget.kind = 'enjoy'`: **0.38 s to close, 0.45 s to open**, total 2400–3300 ms
- Past 0.55 closure the eyes become relaxed arcs (both built-in and custom skin eye paths support it), with a 0.042 lean and `targetGazeY -= 0.12*actEnv`
- While shut, the nod amplitude is multiplied by `(1 - enjoyEnv*0.45)` — the cat melts into the music

<p align="center">
  <img src="docs/2d/enjoy.webp" width="620" alt="Open eyes vs. eyes shut" /><br/>
  <sub>Left: normal open eyes. Right: eyes-shut bliss (arc eyes + slight head lean)</sub>
</p>

## Motion details

- **Bracing paw**: while the mouse is held one paw braces the ground, with a solid outline on dark fur
- **Headband follow**: on lateral head turns the band and earcups rotate with `faceTurnX*0.125`, staying on the head
- **Collar follows the pose**: `collarAnchor()` uses `leanDamp = 1 - min(1,|spineLean|)*0.66` so the cord stays close to horizontal even in a strong side lean
- **Menu and version**: `breed_bellblack` in `tray.rs`, the `get_music_active` command and playback polling in `music_detect.rs`; version 5.2.0 in `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`

<p align="center">
  <img src="docs/2d/menu.webp" width="330" alt="Bellblack entry in the menu" /><br/>
  <sub>A standalone entry under cat appearance; selecting it highlights it and shows `KuroCat 铃铛小黑` in the menu header</sub>
</p>

## How it differs from the classic black cat

| | Classic black cat | Bellblack |
|---|---|---|
| Face | standard silhouette | `faceShape: 'round'`, cheeks up to 63.5 wide |
| Neck | no accessory / custom spots | engine-drawn cord + gold ball bell |
| Front legs | segmented strokes | one continuous stroke with a solid paw-ball arc |
| Headphones | dark earcups + glow ring | white-keycap earcups + amber ring, no boom mic |
| Notes | — | hand-drawn single / beamed note, two lanes |
| Music | headphones only | nod, face sway, weight shift, half-rate tail + random gestures |
| Eyes | ordinary blinks | `enjoy` bliss, arc eyes with a slight lean |

## How it is built

Everything is drawn live on Canvas 2D — no texture assets.

- **Skin wiring**: a `bellblack` entry in `DEFAULT_BUILTIN_CATS` (`emoji: '🔔'`, `faceShape: 'round'`, amber eyes `#fbcb45`, pale leg lines and whiskers), mirrored in `src-tauri/src/cats.rs`, plus the `breed_bellblack` tray item
- **Round cheeks**: `BODY_SLICES_ROUND` holds 14 `{t, hw}` samples (t 0.12→1.0, widest 63.5 at t=0.30, belly out to 73). Drawing interpolates the half-width along t and mirrors it about the spine
- **Cord**: no `spots` involved. `drawCollarBand()` uses a quadratic bezier as the centreline (`rx 49.4`, sag `3.2`, ends lifted `2.6`) and sweeps `halfOf(t) = thick*0.5*(0.3+0.7*sin(πt)^0.8)` along it for a 44-segment ribbon: 3.4 wide at the centre, tapering to fine points. Fill is a top-down linear gradient (+16% light → base → 34% dark) over a 1.5px contact shadow
- **Collar anchor**: `collarAnchor()` samples `spinePoint(0.382 / 0.462 / 0.468)`, takes the spine normal via `-atan2(Δx, Δy)`, multiplies by `leanDamp = 1-min(1,|spineLean|)*0.66` and adds `headTilt*0.2`. The band is drawn before the paws and the bell after them, so a raised paw never covers it
- **Bell**: a 6.2-radius sphere — elliptical contact shadow, `createRadialGradient` (highlight → `#edb938` → shade), 0.8px dark-gold rim, a flat oval sound hole on the front face. Sway is `sin(t*5.6)*(0.03 + typingEnergy*0.14 + |spineLean|*0.18) + strikeRock*0.1`
- **Headphones**: two passes. `drawHeadphonesBack()` draws the band — 11.1px light edge, 9.4px charcoal shell, 2.0px satin highlight, 5.4px inner pad, titanium sliders; `drawHeadphonesFront()` draws the cups — `#9fa39c` cushion, white-keycap gradient face, 7.6×14.4 inset driver, 5.4×11.4 amber LED ring. Putting on/taking off is driven by `headsetSpring`; band and cups follow head turns by `faceTurnX*0.125 / 0.25`
- **Notes**: `noteSprite()` bakes the glyph into an offscreen canvas at 3× resolution (so the two ink layers cannot show through each other while fading), alternating two variants. Spawns start at `spinePoint(0.18)` on two lanes (+20 / +45); if another note sits within 23×22 the spawn waits 280 ms. Lifetime 2200 ms, drifting upward
- **Groove and blinks**: state lives in `musicBpm / musicBeatPhase / musicGroove / musicAct / nextMusicActAt`; each frame advances the phase and picks a gesture when due. `fidgetEyeClosure()` produces the lid envelope and `isBlissClosed()` switches to arc eyes above 0.55
- **Music signal**: `music_detect.rs` requires 6 consecutive on-ticks before reporting playback through the `kuro-music-active` event and the `get_music_active` command

## Build and run

Prerequisites: macOS 11+ / Windows 10+, Node.js 18+, a Rust toolchain.

```bash
git clone https://github.com/coocv/kurocat.git -b feat/bellblack-2d
cd kurocat
npm install
npm run tauri:dev      # local dev
npm run tauri:build    # release build
```

On macOS, keeping the Accessibility / Input Monitoring grants across versions needs one fixed self-signed certificate: leave `signingIdentity` out of `tauri.conf.json` and sign after packaging

```bash
codesign --force --deep --sign "your-self-signed-cert" src-tauri/target/release/bundle/macos/KuroCat.app
```

A self-signed certificate cannot replace Apple notarisation, so the first launch still needs approval under Privacy & Security.

## Scope

- **2D only** — the 3D engine (`catEngine3d.ts`) is not part of this branch
- Bellblack is a built-in breed, not a `cats.json` custom skin; making it a pure custom skin would require parameterising the collar and headphone drawing
- Custom skins can layer patterns with `spots`, while the collar and bell are drawn by the engine
- Build artefacts such as `src-tauri/gen/schemas` are gitignored

## Upstream and licence

Upstream project and docs: [qianyiai/kurocat](https://github.com/qianyiai/kurocat), MIT License (see [LICENSE](LICENSE)). This branch only touches 2D rendering and characters; all upstream attribution and licence notices are retained.
