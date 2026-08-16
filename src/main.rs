mod atlas;
mod audio;
mod editor;
mod entities;
mod game;
mod level;
mod renderer;

use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::mouse::MouseButton;
use atlas::SpriteAtlas;
use audio::AudioEngine;
use editor::{load_custom_level, save_custom_level};
use entities::{Brick, BrickType};
use game::{Game, GameMode, GameState};
use renderer::render_game;

fn main() -> Result<(), String> {
    // 1. Initialize SDL2 Context & Subsystems
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    // 2. Create Audio Engine & Window
    println!("[Ballscape] Initializing procedural synth audio engine...");
    let mut audio_engine = AudioEngine::new(&audio_subsystem);

    let window = video_subsystem
        .window("Ballscape - Sci-Fi Action Breakout Deluxe", 1280, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_logical_size(1280, 720).map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    // 3. Load Atlas Data & Texture Sheet
    println!("[Ballscape] Loading sprite sheet and texture atlas...");
    let atlas = SpriteAtlas::load_from_file("ballscape_atlas.json")?;
    let texture = texture_creator.load_texture("ballscape_sprite_sheet.png")?;
    println!("[Ballscape] Loaded atlas with 60 sprites successfully!");

    // 4. Initialize Game State
    let mut game = Game::new(1280.0, 720.0);
    let mut event_pump = sdl_context.event_pump()?;

    let target_frame_duration = Duration::from_micros(16_666); // ~60 FPS
    let mut last_instant = Instant::now();

    println!("[Ballscape] All Deluxe Systems Online! Controls:");
    println!("  - SPACE / Left Click: Start Campaign / Launch Ball / Fire Lasers");
    println!("  - H: Start Endless Horde Mode");
    println!("  - E: Toggle Level Editor (Left Click Place, Right Click Erase, Scroll Wheel Change Brick)");
    println!("  - S / L in Editor: Save / Load Custom Level JSON");
    println!("  - B: Open Hangar Shop (Upgrade Paddle, Lasers, Lives & Magnet Beam)");
    println!("  - P: Pause / Resume Game");
    println!("  - ESC: Quit Game");

    let brick_palette = [
        BrickType::ColorRed,
        BrickType::ColorOrange,
        BrickType::ColorYellow,
        BrickType::ColorGreen,
        BrickType::ColorBlue,
        BrickType::ColorPurple,
        BrickType::ArmoredDark,
        BrickType::ArmoredGold,
        BrickType::Unbreakable,
        BrickType::Explosive,
        BrickType::Powerup,
        BrickType::Moving,
        BrickType::Portal,
    ];
    let mut palette_idx = 0;

    'running: loop {
        let now = Instant::now();
        let dt = (now - last_instant).as_secs_f32().min(0.05);
        last_instant = now;

        let mut move_left = false;
        let mut move_right = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    if game.state == GameState::Playing {
                        game.state = GameState::Paused;
                    } else if game.state == GameState::Paused {
                        game.state = GameState::Playing;
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    if game.state == GameState::Menu {
                        game.start_endless_mode();
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    if game.state == GameState::Menu {
                        game.state = GameState::Shop;
                    } else if game.state == GameState::Shop {
                        game.state = GameState::Menu;
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    if game.mode == GameMode::Editor {
                        game.mode = GameMode::Campaign;
                        game.state = GameState::Menu;
                    } else {
                        game.mode = GameMode::Editor;
                        game.state = GameState::Playing;
                        game.bricks.clear();
                        game.hazards.clear();
                        game.powerups.clear();
                        game.boss = None;
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if game.mode == GameMode::Editor {
                        let _ = save_custom_level(&game.bricks, "custom_level.json");
                        println!("[Level Editor] Saved custom level to custom_level.json");
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    if game.mode == GameMode::Editor {
                        if let Ok(loaded_bricks) = load_custom_level("custom_level.json") {
                            game.bricks = loaded_bricks;
                            println!("[Level Editor] Loaded custom level from custom_level.json");
                        }
                    }
                }

                Event::MouseWheel { y, .. } => {
                    if game.mode == GameMode::Editor {
                        if y > 0 {
                            palette_idx = (palette_idx + 1) % brick_palette.len();
                        } else if y < 0 {
                            if palette_idx == 0 {
                                palette_idx = brick_palette.len() - 1;
                            } else {
                                palette_idx -= 1;
                            }
                        }
                        game.editor_selected_brick = brick_palette[palette_idx];
                    }
                }

                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if game.mode == GameMode::Editor {
                        let grid_w = 84.0;
                        let grid_h = 34.0;
                        let snap_x = (x as f32 / (grid_w + 10.0)).floor() * (grid_w + 10.0) + 40.0;
                        let snap_y = (y as f32 / (grid_h + 10.0)).floor() * (grid_h + 10.0) + 60.0;

                        if mouse_btn == MouseButton::Left {
                            let new_id = game.bricks.len();
                            game.bricks.retain(|b| (b.x - snap_x).abs() > 5.0 || (b.y - snap_y).abs() > 5.0);
                            game.bricks.push(Brick::new(new_id, snap_x, snap_y, grid_w, grid_h, game.editor_selected_brick));
                        } else if mouse_btn == MouseButton::Right {
                            game.bricks.retain(|b| (b.x - snap_x).abs() > 5.0 || (b.y - snap_y).abs() > 5.0);
                        }
                    } else {
                        match game.state {
                            GameState::Menu | GameState::GameOver | GameState::Victory => {
                                game.start_game();
                            }
                            GameState::Playing => {
                                game.launch_balls();
                                game.fire_paddle_laser();
                            }
                            _ => {}
                        }
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    match game.state {
                        GameState::Menu | GameState::GameOver | GameState::Victory => {
                            game.start_game();
                        }
                        GameState::Playing => {
                            game.launch_balls();
                            game.fire_paddle_laser();
                        }
                        _ => {}
                    }
                }

                Event::MouseMotion { x, .. } => {
                    if game.state == GameState::Playing {
                        game.paddle.x = x as f32;
                    }
                }

                _ => {}
            }
        }

        // Keyboard Movement Controls
        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left)
            || keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            move_left = true;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right)
            || keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            move_right = true;
        }

        // 5. Update Game World
        game.update(dt, move_left, move_right);

        // 6. Play Audio Queue
        for sfx in game.sound_events.drain(..) {
            audio_engine.play(sfx);
        }

        // 7. Render Frame
        render_game(&mut canvas, &texture, &atlas, &game)?;

        // Frame timing cap
        let elapsed = now.elapsed();
        if elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed);
        }
    }

    println!("[Ballscape] Game shutting down. Thanks for playing!");
    Ok(())
}
