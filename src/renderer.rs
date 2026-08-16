use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use crate::atlas::SpriteAtlas;
use crate::game::{Game, GameMode, GameState};

pub fn render_game(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    atlas: &SpriteAtlas,
    game: &Game,
) -> Result<(), String> {
    // 1. Background Fill (Dark Sci-Fi Space)
    canvas.set_draw_color(Color::RGB(12, 14, 28));
    canvas.clear();

    // Draw background grid lines with camera offset
    let off_x = game.camera_offset_x as i32;
    let off_y = game.camera_offset_y as i32;

    canvas.set_draw_color(Color::RGBA(30, 40, 70, 80));
    let grid_size = 40;
    for x in (0..game.screen_w as i32).step_by(grid_size) {
        let _ = canvas.draw_line(Point::new(x + off_x, off_y), Point::new(x + off_x, game.screen_h as i32 + off_y));
    }
    for y in (0..game.screen_h as i32).step_by(grid_size) {
        let _ = canvas.draw_line(Point::new(off_x, y + off_y), Point::new(game.screen_w as i32 + off_x, y + off_y));
    }

    // 2. Floor Safety Shield Line
    if game.floor_shield_active {
        canvas.set_draw_color(Color::RGB(0, 200, 255));
        let shield_y = game.screen_h as i32 - 20 + off_y;
        let rect = Rect::new(off_x, shield_y - 4, game.screen_w as u32, 8);
        let _ = canvas.fill_rect(rect);
    }

    // 3. Render Bricks
    for b in &game.bricks {
        if b.is_destroyed {
            continue;
        }

        let dest = Rect::new(b.x as i32 + off_x, b.y as i32 + off_y, b.w as u32, b.h as u32);
        atlas.render_sprite(canvas, texture, b.brick_type.base_sprite_name(), dest, 0.0, None, false, false);

        if let Some(dmg_sprite) = b.damage_overlay() {
            atlas.render_sprite(canvas, texture, dmg_sprite, dest, 0.0, None, false, false);
        }
    }

    // 4. Render Boss Mothership
    if let Some(ref boss) = game.boss {
        if boss.is_active {
            let boss_dest = Rect::new(
                (boss.x - boss.w * 0.5) as i32 + off_x,
                (boss.y - boss.h * 0.5) as i32 + off_y,
                boss.w as u32,
                boss.h as u32,
            );
            atlas.render_sprite(canvas, texture, "hazard_turret.png", boss_dest, 180.0, None, false, false);

            // Boss Shield Ring
            let shield_dest = Rect::new(
                (boss.x - (boss.w + 40.0) * 0.5) as i32 + off_x,
                (boss.y - (boss.h + 40.0) * 0.5) as i32 + off_y,
                (boss.w + 40.0) as u32,
                (boss.h + 40.0) as u32,
            );
            atlas.render_sprite(canvas, texture, "fx_portal_swirl.png", shield_dest, boss.shield_angle, None, false, false);

            // Boss Health Bar
            let bar_w = 400;
            let bar_h = 16;
            let bar_x = (game.screen_w as i32 - bar_w) / 2 + off_x;
            let bar_y = 55 + off_y;

            canvas.set_draw_color(Color::RGB(40, 10, 10));
            let _ = canvas.fill_rect(Rect::new(bar_x, bar_y, bar_w as u32, bar_h as u32));

            let hp_ratio = (boss.hp as f32 / boss.max_hp as f32).clamp(0.0, 1.0);
            let hp_w = (bar_w as f32 * hp_ratio) as u32;

            canvas.set_draw_color(if boss.phase == 2 { Color::RGB(255, 40, 40) } else { Color::RGB(255, 180, 0) });
            let _ = canvas.fill_rect(Rect::new(bar_x, bar_y, hp_w, bar_h as u32));

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = canvas.draw_rect(Rect::new(bar_x, bar_y, bar_w as u32, bar_h as u32));
        }
    }

    // 5. Render Hazards
    for h in &game.hazards {
        let dest = Rect::new(
            (h.x - h.w * 0.5) as i32 + off_x,
            (h.y - h.h * 0.5) as i32 + off_y,
            h.w as u32,
            h.h as u32,
        );
        atlas.render_sprite(canvas, texture, h.hazard_type.sprite_name(), dest, h.angle, None, false, false);
    }

    // 6. Render Projectiles
    for p in &game.projectiles {
        let rect = Rect::new(
            (p.x - p.w * 0.5) as i32 + off_x,
            (p.y - p.h * 0.5) as i32 + off_y,
            p.w as u32,
            p.h as u32,
        );
        if p.from_player {
            canvas.set_draw_color(Color::RGB(100, 220, 255));
        } else {
            canvas.set_draw_color(Color::RGB(255, 50, 50));
        }
        let _ = canvas.fill_rect(rect);
    }

    // 7. Render PowerUp Capsules
    for p in &game.powerups {
        let dest = Rect::new(
            (p.x - p.w * 0.5) as i32 + off_x,
            (p.y - p.h * 0.5) as i32 + off_y,
            p.w as u32,
            p.h as u32,
        );
        atlas.render_sprite(canvas, texture, p.powerup_type.sprite_name(), dest, p.rotation, None, false, false);
    }

    // 8. Render Paddle
    let p_dest = Rect::new(
        (game.paddle.x - game.paddle.w * 0.5) as i32 + off_x,
        (game.paddle.y - game.paddle.h * 0.5) as i32 + off_y,
        game.paddle.w as u32,
        game.paddle.h as u32,
    );
    atlas.render_sprite(canvas, texture, game.paddle.paddle_type.sprite_name(), p_dest, 0.0, None, false, false);

    // 9. Render Balls & Motion Trails
    for ball in &game.balls {
        for (idx, &(tx, ty)) in ball.trail.iter().enumerate() {
            let alpha_scale = (idx as f32 + 1.0) / ball.trail.len() as f32 * 0.5;
            let trail_size = (ball.radius * 2.0 * alpha_scale) as u32;
            let t_dest = Rect::new(
                (tx - trail_size as f32 * 0.5) as i32 + off_x,
                (ty - trail_size as f32 * 0.5) as i32 + off_y,
                trail_size,
                trail_size,
            );
            atlas.render_sprite(canvas, texture, "fx_glow_halo.png", t_dest, 0.0, None, false, false);
        }

        let b_dest = Rect::new(
            (ball.x - ball.radius) as i32 + off_x,
            (ball.y - ball.radius) as i32 + off_y,
            (ball.radius * 2.0) as u32,
            (ball.radius * 2.0) as u32,
        );
        atlas.render_sprite(canvas, texture, ball.ball_type.sprite_name(), b_dest, 0.0, None, false, false);
    }

    // 10. Render Particles
    for part in &game.particles {
        let size = (32.0 * part.scale) as u32;
        let dest = Rect::new(
            (part.x - size as f32 * 0.5) as i32 + off_x,
            (part.y - size as f32 * 0.5) as i32 + off_y,
            size,
            size,
        );
        atlas.render_sprite(canvas, texture, &part.sprite_name, dest, part.rotation, None, false, false);
    }

    // 11. Render CRT Scanlines
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 30));
    for y in (0..game.screen_h as i32).step_by(3) {
        let _ = canvas.draw_line(Point::new(0, y), Point::new(game.screen_w as i32, y));
    }

    // 12. Render HUD Header Bar
    render_hud(canvas, texture, atlas, game)?;

    // 13. Render State Displays (Menu / Pause / Shop / Game Over / Victory / Editor)
    match game.state {
        GameState::Menu => render_banner(canvas, Color::RGB(0, 220, 255))?,
        GameState::Paused => render_banner(canvas, Color::RGB(255, 200, 50))?,
        GameState::GameOver => render_banner(canvas, Color::RGB(255, 50, 50))?,
        GameState::Victory => render_banner(canvas, Color::RGB(100, 255, 100))?,
        GameState::Shop => render_shop_ui(canvas, texture, atlas, game)?,
        GameState::Playing => {}
    }

    if game.mode == GameMode::Editor {
        render_editor_ui(canvas, texture, atlas, game)?;
    }

    canvas.present();
    Ok(())
}

fn render_hud(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    atlas: &SpriteAtlas,
    game: &Game,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(18, 22, 40));
    let hud_rect = Rect::new(0, 0, game.screen_w as u32, 45);
    let _ = canvas.fill_rect(hud_rect);

    canvas.set_draw_color(Color::RGB(45, 60, 100));
    let _ = canvas.draw_line(Point::new(0, 45), Point::new(game.screen_w as i32, 45));

    for i in 0..game.lives {
        let dest = Rect::new(15 + i * 32, 8, 28, 28);
        atlas.render_sprite(canvas, texture, "ui_life.png", dest, 0.0, None, false, false);
    }

    let score_dest = Rect::new(200, 8, 28, 28);
    atlas.render_sprite(canvas, texture, "ui_score.png", score_dest, 0.0, None, false, false);

    if game.combo > 1 {
        let combo_dest = Rect::new(450, 8, 28, 28);
        atlas.render_sprite(canvas, texture, "ui_combo.png", combo_dest, 0.0, None, false, false);
    }

    Ok(())
}

fn render_banner(
    canvas: &mut Canvas<Window>,
    color: Color,
) -> Result<(), String> {
    let w = 640;
    let h = 200;
    let x = (1280 - w) / 2;
    let y = (720 - h) / 2;

    canvas.set_draw_color(Color::RGBA(10, 15, 30, 235));
    let rect = Rect::new(x, y, w as u32, h as u32);
    let _ = canvas.fill_rect(rect);

    canvas.set_draw_color(color);
    let _ = canvas.draw_rect(rect);

    Ok(())
}

fn render_shop_ui(
    canvas: &mut Canvas<Window>,
    _texture: &Texture,
    _atlas: &SpriteAtlas,
    _game: &Game,
) -> Result<(), String> {
    let w = 700;
    let h = 450;
    let x = (1280 - w) / 2;
    let y = (720 - h) / 2;

    canvas.set_draw_color(Color::RGBA(15, 20, 45, 240));
    let rect = Rect::new(x, y, w as u32, h as u32);
    let _ = canvas.fill_rect(rect);

    canvas.set_draw_color(Color::RGB(0, 200, 255));
    let _ = canvas.draw_rect(rect);

    Ok(())
}

fn render_editor_ui(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    atlas: &SpriteAtlas,
    game: &Game,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGBA(20, 25, 50, 220));
    let rect = Rect::new(20, 660, 400, 50);
    let _ = canvas.fill_rect(rect);

    canvas.set_draw_color(Color::RGB(0, 220, 255));
    let _ = canvas.draw_rect(rect);

    let preview_dest = Rect::new(30, 668, 70, 32);
    atlas.render_sprite(canvas, texture, game.editor_selected_brick.base_sprite_name(), preview_dest, 0.0, None, false, false);

    Ok(())
}
