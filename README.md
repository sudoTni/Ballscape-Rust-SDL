# 🪐 Ballscape - Sci-Fi Action Breakout Deluxe

> A fast-paced, feature-packed **Arkanoid / Breakout action game** written in **Rust** using **SDL2**. Features elemental balls, weaponized laser paddles, explosive chain reactions, real-time procedural audio synthesis, screen shake camera recoil, stage boss battles, an in-game level editor, endless horde mode, and a meta-progression shop.

---

<p align = "center"><img width="800" height="437" alt="screenshot" src="https://github.com/user-attachments/assets/aa983f2e-5e07-46b3-9a68-3a7cb397571c" /></p>

## 🌟 Key Features

### 🏀 Gameplay & Physics Engine
- **Angle-Based Paddle Reflections:** Ball rebound angles vary dynamically based on where the ball strikes relative to the paddle center.
- **5 Elemental Ball Types:**
  - `Normal`: Classic chrome sphere with realistic physics.
  - `Fast`: High-velocity blue plasma orb with motion trails.
  - `Fire`: Fiery penetrating ball that melts straight through bricks without bouncing.
  - `Ice`: Frost orb with floating crystal sparkles.
  - `Electric`: High-voltage purple sphere with energetic lightning arcs.
- **6 Paddle Transformations:**
  - `Normal`: Standard silver/blue sci-fi paddle.
  - `Short` & `Long`: Handicap debuff and expand buff states.
  - `Sticky`: Catches and holds balls until manually launched.
  - `Shielded`: Protective energy barrier crest above paddle.
  - `Laser`: Equips dual plasma cannons firing rapid energy blasts.

### 🧱 Dynamic Bricks & Environmental Hazards
- **16 Brick Variants:**
  - Standard Color Bricks (Red, Orange, Yellow, Green, Blue, Purple)
  - Damage Overlay Overlays (Light, Medium, Heavy fracture overlays)
  - `Armored Dark` & `Armored Gold`: Require 2–3 hits to crack.
  - `Explosive`: Triggers area-of-effect skull blast chain reactions.
  - `Portal Exit`: Quantum warp vortex teleports the ball to exit portals.
  - `Moving`: Horizontal slider defense blocks.
  - `Power-up`: Guaranteed capsule drops.
  - `Unbreakable`: Indestructible steel plate obstacles.
- **Active Hazards:** Floating Space Mines, Pinball Rebound Bumpers, Electric Zap Fences, and Automated Defense Turrets firing plasma bolts.

### 💊 10 Droppable Power-Up Capsules
1. **Multi-Ball:** Splits active ball into 3 high-speed balls.
2. **Extra Life:** Grants +1 player life.
3. **Expand Paddle:** Widens paddle length.
4. **Shrink Paddle:** Compacts paddle size.
5. **Sticky Slime:** Catches ball on contact.
6. **Laser Cannons:** Mounts dual weapon blasters.
7. **Fireball:** Converts balls to penetration mode.
8. **Slo-Mo:** Reduces ball velocity.
9. **Floor Shield:** Deploys safety barrier covering the bottom pit.
10. **2x Multiplier:** Doubles all score points earned.

### 🛸 Stage 5 Omega Mothership Boss Battle
- Multi-phase boss battle featuring a rotating force field, laser barrages, minion mine spawning, and enrage mode phase transitions.

### 🔊 Real-Time Procedural Audio Synthesizer
- Zero external audio file dependency! Uses SDL2 audio callback to synthesize custom retro sound effects (laser blasts, pitch-shifted paddle bounces, noise explosions, power-up pickup arpeggios, and victory jingles).

### 🛠️ In-Game Level Editor & Endless Mode
- **Level Builder:** Place and erase bricks with mouse clicks, cycle palette with mouse wheel, and save/load custom JSON stage files (`custom_level.json`).
- **Endless Horde:** Descending brick waves advancing downwards every 12 seconds with scaling score multipliers.

---

## 🎮 Controls & Shortcuts

| Action | Control / Shortcut |
| :--- | :--- |
| **Move Paddle** | Mouse Movement / Left & Right Arrow Keys / `A` & `D` |
| **Launch Ball / Fire Lasers** | `SPACE` or Left Mouse Click |
| **Start Campaign** | `SPACE` or Left Click (from Menu) |
| **Start Endless Horde Mode** | `H` (from Menu) |
| **Toggle Level Editor** | `E` (Left Click place, Right Click erase, Wheel cycle brick) |
| **Save / Load Custom Level** | `S` / `L` (in Level Editor) |
| **Open Hangar Shop** | `B` (from Menu) |
| **Pause / Resume Game** | `P` |
| **Quit Game** | `ESC` |

---

## 🛠️ Build & Installation

### Prerequisites
- **Rust Toolchain:** `rustc` and `cargo` ($1.70+$)
- **SDL2 System Libraries:** `SDL2` and `SDL2_image`

On **Arch / Manjaro Linux**:
```bash
sudo pacman -S sdl2 sdl2_image
```

On **Ubuntu / Debian**:
```bash
sudo apt install libsdl2-dev libsdl2-image-dev
```

### Compiling & Running

```bash
# Clone or navigate to project directory
cd /home/michael/tmp/ballscape

# Run in Development Mode
cargo run

# Build Optimized Release Executable
cargo build --release

# Run Release Binary directly
./target/release/ballscape
```

---

## 📁 Codebase Architecture

```
ballscape/
├── Cargo.toml                    # Rust dependencies & package metadata
├── ballscape_atlas.json          # TexturePacker JSON atlas (60 sprites)
├── ballscape_sprite_sheet.png    # 32-bit RGBA texture atlas image
├── src/
│   ├── main.rs                   # Entry point, SDL2 window & main loop
│   ├── atlas.rs                  # JSON atlas parser & sprite renderer
│   ├── audio.rs                  # Real-time procedural audio synthesizer
│   ├── editor.rs                 # Level Editor & JSON serializer
│   ├── entities.rs               # Paddle, Ball, Brick, Powerup & Hazard models
│   ├── game.rs                   # State machine, physics & collision logic
│   ├── level.rs                  # Campaign stage level layouts (Stage 1-5)
│   └── renderer.rs               # 2D canvas pipeline, screen shake & CRT FX
```

---

## 📄 License
Designed & Built with ❤️ in Rust and SDL2. Assets provided by OpenAI / Ballscape Atlas.
