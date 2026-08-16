#![allow(dead_code)]
use rand::Rng;
use crate::entities::*;
use crate::level::{load_level, Level};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Campaign,
    Endless,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
    Victory,
    Shop,
}

pub struct Upgrades {
    pub paddle_speed_level: u32,
    pub laser_rate_level: u32,
    pub starting_lives_level: u32,
    pub magnet_beam_level: u32,
    pub credits: u32,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            paddle_speed_level: 0,
            laser_rate_level: 0,
            starting_lives_level: 0,
            magnet_beam_level: 0,
            credits: 100,
        }
    }
}

pub struct Boss {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub phase: u32,
    pub shield_angle: f64,
    pub attack_timer: f32,
    pub minion_timer: f32,
    pub is_active: bool,
}

pub struct Game {
    pub state: GameState,
    pub mode: GameMode,
    pub screen_w: f32,
    pub screen_h: f32,
    pub current_level: u32,
    pub max_levels: u32,
    pub level_name: String,
    pub score: u32,
    pub lives: i32,
    pub combo: u32,
    pub combo_timer: f32,
    pub score_multiplier: u32,
    pub multiplier_timer: f32,
    pub floor_shield_active: bool,
    pub floor_shield_timer: f32,

    pub shake_intensity: f32,
    pub shake_timer: f32,
    pub camera_offset_x: f32,
    pub camera_offset_y: f32,

    pub upgrades: Upgrades,

    pub editor_selected_brick: BrickType,
    pub editor_bricks: Vec<Brick>,

    pub endless_wave: u32,
    pub endless_shift_timer: f32,

    pub boss: Option<Boss>,

    pub paddle: Paddle,
    pub balls: Vec<Ball>,
    pub bricks: Vec<Brick>,
    pub powerups: Vec<PowerUp>,
    pub hazards: Vec<Hazard>,
    pub projectiles: Vec<Projectile>,
    pub particles: Vec<Particle>,
    pub floating_texts: Vec<FloatingText>,

    pub sound_events: Vec<SoundEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    BouncePaddle,
    BounceWall,
    BrickDestroy,
    Explosion,
    LaserFire,
    PowerupPickup,
    Warp,
    GameOver,
    Victory,
}

impl Game {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        let paddle = Paddle::new(screen_w / 2.0, screen_h - 60.0);
        let mut game = Self {
            state: GameState::Menu,
            mode: GameMode::Campaign,
            screen_w,
            screen_h,
            current_level: 1,
            max_levels: 5,
            level_name: String::new(),
            score: 0,
            lives: 3,
            combo: 0,
            combo_timer: 0.0,
            score_multiplier: 1,
            multiplier_timer: 0.0,
            floor_shield_active: false,
            floor_shield_timer: 0.0,
            shake_intensity: 0.0,
            shake_timer: 0.0,
            camera_offset_x: 0.0,
            camera_offset_y: 0.0,
            upgrades: Upgrades::default(),
            editor_selected_brick: BrickType::ColorRed,
            editor_bricks: Vec::new(),
            endless_wave: 1,
            endless_shift_timer: 0.0,
            boss: None,
            paddle,
            balls: Vec::new(),
            bricks: Vec::new(),
            powerups: Vec::new(),
            hazards: Vec::new(),
            projectiles: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),
            sound_events: Vec::new(),
        };
        game.init_level(1);
        game
    }

    pub fn trigger_shake(&mut self, intensity: f32, duration: f32) {
        self.shake_intensity = intensity;
        self.shake_timer = duration;
    }

    pub fn play_sfx(&mut self, sfx: SoundEvent) {
        self.sound_events.push(sfx);
    }

    pub fn init_level(&mut self, level_num: u32) {
        self.current_level = level_num;
        let lvl: Level = load_level(level_num, self.screen_w, self.screen_h);
        self.level_name = lvl.name.to_string();
        self.bricks = lvl.bricks;
        self.hazards = lvl.hazards;
        self.powerups.clear();
        self.projectiles.clear();
        self.particles.clear();
        self.boss = None;

        if level_num == 5 {
            self.boss = Some(Boss {
                x: self.screen_w / 2.0,
                y: 120.0,
                w: 220.0,
                h: 110.0,
                hp: 50,
                max_hp: 50,
                phase: 1,
                shield_angle: 0.0,
                attack_timer: 2.0,
                minion_timer: 4.0,
                is_active: true,
            });
        }

        self.reset_paddle_and_balls();
    }

    pub fn reset_paddle_and_balls(&mut self) {
        self.paddle.x = self.screen_w / 2.0;
        self.paddle.y = self.screen_h - 60.0;
        self.paddle.speed = 650.0 + (self.upgrades.paddle_speed_level as f32 * 90.0);
        self.paddle.set_type(PaddleType::Normal);

        self.balls.clear();
        let mut ball = Ball::new(self.paddle.x, self.paddle.y - 25.0, 200.0, -450.0, BallType::Normal);
        ball.is_stuck = true;
        ball.stuck_offset_x = 0.0;
        self.balls.push(ball);
    }

    pub fn start_game(&mut self) {
        self.mode = GameMode::Campaign;
        self.score = 0;
        self.lives = 3 + self.upgrades.starting_lives_level as i32;
        self.combo = 0;
        self.floor_shield_active = false;
        self.score_multiplier = 1;
        self.state = GameState::Playing;
        self.init_level(1);
    }

    pub fn start_endless_mode(&mut self) {
        self.mode = GameMode::Endless;
        self.state = GameState::Playing;
        self.score = 0;
        self.lives = 3 + self.upgrades.starting_lives_level as i32;
        self.endless_wave = 1;
        self.endless_shift_timer = 10.0;
        self.level_name = "Endless Horde Mode".to_string();
        self.bricks.clear();
        self.hazards.clear();
        self.powerups.clear();
        self.boss = None;
        self.generate_endless_wave(0);
        self.reset_paddle_and_balls();
    }

    fn generate_endless_wave(&mut self, row_offset: usize) {
        let cols = 10;
        let brick_w = 84.0;
        let brick_h = 34.0;
        let spacing_x = 10.0;
        let start_x = (self.screen_w - (cols as f32 * (brick_w + spacing_x) - spacing_x)) / 2.0;
        let y = 80.0 + (row_offset as f32 * (brick_h + 10.0));

        let types = [
            BrickType::ColorRed,
            BrickType::ColorOrange,
            BrickType::ColorYellow,
            BrickType::ColorGreen,
            BrickType::ColorBlue,
            BrickType::ArmoredDark,
            BrickType::Explosive,
            BrickType::Powerup,
        ];
        let mut rng = rand::thread_rng();
        let base_id = self.bricks.len();

        for c in 0..cols {
            let x = start_x + c as f32 * (brick_w + spacing_x);
            let btype = types[rng.gen_range(0..types.len())];
            self.bricks.push(Brick::new(base_id + c, x, y, brick_w, brick_h, btype));
        }
    }

    pub fn launch_balls(&mut self) {
        for ball in &mut self.balls {
            if ball.is_stuck {
                ball.is_stuck = false;
                let angle_rad = (-75.0_f32).to_radians();
                let speed = 500.0 * ball.ball_type.speed_multiplier();
                ball.vx = speed * angle_rad.cos();
                ball.vy = speed * angle_rad.sin();
            }
        }
    }

    pub fn fire_paddle_laser(&mut self) {
        if self.paddle.paddle_type == PaddleType::Laser && self.paddle.laser_cooldown <= 0.0 {
            let cooldown = 0.25 - (self.upgrades.laser_rate_level as f32 * 0.05);
            self.paddle.laser_cooldown = cooldown.max(0.1);

            self.projectiles.push(Projectile {
                x: self.paddle.x - self.paddle.w * 0.4,
                y: self.paddle.y - self.paddle.h * 0.5,
                vy: -800.0,
                w: 8.0,
                h: 24.0,
                from_player: true,
            });
            self.projectiles.push(Projectile {
                x: self.paddle.x + self.paddle.w * 0.4,
                y: self.paddle.y - self.paddle.h * 0.5,
                vy: -800.0,
                w: 8.0,
                h: 24.0,
                from_player: true,
            });
            self.trigger_shake(2.0, 0.08);
            self.play_sfx(SoundEvent::LaserFire);
            self.spawn_floating_text("PEW!", self.paddle.x, self.paddle.y - 30.0, (255, 100, 100));
        }
    }

    pub fn update(&mut self, dt: f32, move_left: bool, move_right: bool) {
        if self.state != GameState::Playing {
            return;
        }

        let mut boss_text_events = Vec::new();

        if self.shake_timer > 0.0 {
            self.shake_timer -= dt;
            let mut rng = rand::thread_rng();
            self.camera_offset_x = rng.gen_range(-self.shake_intensity..self.shake_intensity);
            self.camera_offset_y = rng.gen_range(-self.shake_intensity..self.shake_intensity);
        } else {
            self.camera_offset_x = 0.0;
            self.camera_offset_y = 0.0;
        }

        if self.paddle.laser_cooldown > 0.0 {
            self.paddle.laser_cooldown -= dt;
        }
        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 0;
            }
        }
        if self.multiplier_timer > 0.0 {
            self.multiplier_timer -= dt;
            if self.multiplier_timer <= 0.0 {
                self.score_multiplier = 1;
            }
        }
        if self.floor_shield_timer > 0.0 {
            self.floor_shield_timer -= dt;
            if self.floor_shield_timer <= 0.0 {
                self.floor_shield_active = false;
            }
        }

        if self.mode == GameMode::Endless {
            self.endless_shift_timer -= dt;
            if self.endless_shift_timer <= 0.0 {
                self.endless_shift_timer = 12.0;
                self.endless_wave += 1;
                for b in &mut self.bricks {
                    if !b.is_destroyed {
                        b.y += 40.0;
                        if b.y >= self.paddle.y - 40.0 {
                            self.state = GameState::GameOver;
                        }
                    }
                }
                self.generate_endless_wave(0);
                boss_text_events.push((format!("WAVE {}!", self.endless_wave), self.screen_w / 2.0, 200.0, (255, 200, 50)));
            }
        }

        let half_w = self.paddle.w / 2.0;
        if move_left {
            self.paddle.x -= self.paddle.speed * dt;
        }
        if move_right {
            self.paddle.x += self.paddle.speed * dt;
        }
        self.paddle.x = self.paddle.x.clamp(half_w + 10.0, self.screen_w - half_w - 10.0);

        for ball in &mut self.balls {
            if ball.is_stuck {
                ball.x = self.paddle.x + ball.stuck_offset_x;
                ball.y = self.paddle.y - self.paddle.h * 0.5 - ball.radius;
            }
        }

        if self.upgrades.magnet_beam_level > 0 {
            let magnet_speed = 150.0 * self.upgrades.magnet_beam_level as f32;
            for p in &mut self.powerups {
                if !p.collected {
                    if (p.x - self.paddle.x).abs() < 250.0 {
                        let dx = self.paddle.x - p.x;
                        p.x += dx.signum() * magnet_speed * dt;
                    }
                }
            }
        }

        if let Some(ref mut boss) = self.boss {
            if boss.is_active {
                boss.shield_angle += 60.0 * dt as f64;
                boss.x += (boss.shield_angle as f32 * 0.05).sin() * 80.0 * dt;

                boss.attack_timer -= dt;
                if boss.attack_timer <= 0.0 {
                    boss.attack_timer = if boss.phase == 2 { 1.5 } else { 2.5 };
                    self.projectiles.push(Projectile {
                        x: boss.x - 40.0,
                        y: boss.y + 40.0,
                        vy: 400.0,
                        w: 12.0,
                        h: 24.0,
                        from_player: false,
                    });
                    self.projectiles.push(Projectile {
                        x: boss.x + 40.0,
                        y: boss.y + 40.0,
                        vy: 400.0,
                        w: 12.0,
                        h: 24.0,
                        from_player: false,
                    });
                }

                boss.minion_timer -= dt;
                if boss.minion_timer <= 0.0 {
                    boss.minion_timer = 6.0;
                    self.hazards.push(Hazard {
                        x: boss.x,
                        y: boss.y + 60.0,
                        w: 50.0,
                        h: 50.0,
                        hazard_type: HazardType::Mine,
                        angle: 0.0,
                        shoot_timer: 0.0,
                    });
                    boss_text_events.push(("MINION SPAWN!".to_string(), boss.x, boss.y + 80.0, (255, 100, 100)));
                }
            }
        }

        for (txt, x, y, col) in boss_text_events {
            self.spawn_floating_text(&txt, x, y, col);
        }

        for b in &mut self.bricks {
            if !b.is_destroyed && b.brick_type == BrickType::Moving {
                b.x += b.move_dir * 120.0 * dt;
                if b.x <= b.min_x || b.x >= b.max_x {
                    b.move_dir *= -1.0;
                }
            }
        }

        let mut new_turret_shots = Vec::new();
        for h in &mut self.hazards {
            h.angle += 40.0 * dt as f64;
            if h.hazard_type == HazardType::Turret {
                h.shoot_timer -= dt;
                if h.shoot_timer <= 0.0 {
                    h.shoot_timer = 2.5;
                    new_turret_shots.push(Projectile {
                        x: h.x,
                        y: h.y + h.h * 0.5,
                        vy: 350.0,
                        w: 10.0,
                        h: 20.0,
                        from_player: false,
                    });
                }
            }
        }
        self.projectiles.extend(new_turret_shots);

        let mut i = 0;
        let mut text_events = Vec::new();
        let mut explosion_events = Vec::new();
        let mut projectile_destroyed_bricks = Vec::new();

        while i < self.projectiles.len() {
            let p = &mut self.projectiles[i];
            p.y += p.vy * dt;
            let mut removed = false;

            if p.y < 0.0 || p.y > self.screen_h {
                self.projectiles.remove(i);
                continue;
            }

            if p.from_player {
                if let Some(ref mut boss) = self.boss {
                    if boss.is_active
                        && p.x >= boss.x - boss.w * 0.5
                        && p.x <= boss.x + boss.w * 0.5
                        && p.y >= boss.y - boss.h * 0.5
                        && p.y <= boss.y + boss.h * 0.5
                    {
                        boss.hp -= 1;
                        explosion_events.push((p.x, p.y, 8));
                        if boss.hp <= boss.max_hp / 2 && boss.phase == 1 {
                            boss.phase = 2;
                            text_events.push(("BOSS ENRAGED!".to_string(), boss.x, boss.y - 30.0, (255, 50, 50)));
                        }
                        if boss.hp <= 0 {
                            boss.is_active = false;
                            self.score += 5000;
                            text_events.push(("BOSS DESTROYED!".to_string(), boss.x, boss.y, (255, 220, 50)));
                            explosion_events.push((boss.x, boss.y, 60));
                        }
                        removed = true;
                    }
                }

                if !removed {
                    for b in &mut self.bricks {
                        if !b.is_destroyed && b.brick_type != BrickType::Unbreakable {
                            if p.x >= b.x && p.x <= b.x + b.w && p.y >= b.y && p.y <= b.y + b.h {
                                b.hp -= 1;
                                if b.hp <= 0 {
                                    b.is_destroyed = true;
                                    projectile_destroyed_bricks.push((b.x, b.y, b.w, b.h, b.brick_type, b.id));
                                }
                                removed = true;
                                break;
                            }
                        }
                    }
                }
            } else {
                if p.x >= self.paddle.x - self.paddle.w * 0.5
                    && p.x <= self.paddle.x + self.paddle.w * 0.5
                    && p.y >= self.paddle.y - self.paddle.h * 0.5
                    && p.y <= self.paddle.y + self.paddle.h * 0.5
                {
                    explosion_events.push((p.x, p.y, 10));
                    text_events.push(("TURRET HIT!".to_string(), p.x, p.y - 20.0, (255, 50, 50)));
                    self.trigger_shake(6.0, 0.2);
                    removed = true;
                }
            }

            if removed {
                self.projectiles.remove(i);
            } else {
                i += 1;
            }
        }

        for (bx, by, bw, bh, btype, bid) in projectile_destroyed_bricks {
            self.on_brick_destroyed(bx, by, bw, bh, btype, bid);
        }
        for (text, x, y, col) in text_events {
            self.spawn_floating_text(&text, x, y, col);
        }
        for (x, y, cnt) in explosion_events {
            self.spawn_explosion_particles(x, y, cnt);
        }

        let mut lost_balls_indices = Vec::new();
        let floor_y = self.screen_h - 20.0;

        let mut bounce_flashes = Vec::new();
        let mut text_queue = Vec::new();
        let mut explosion_queue = Vec::new();
        let mut sound_queue = Vec::new();

        for (ball_idx, ball) in self.balls.iter_mut().enumerate() {
            if ball.is_stuck {
                continue;
            }

            ball.update(dt);

            if ball.x - ball.radius <= 10.0 {
                ball.x = 10.0 + ball.radius;
                ball.vx = ball.vx.abs();
                bounce_flashes.push((ball.x, ball.y));
                sound_queue.push(SoundEvent::BounceWall);
            } else if ball.x + ball.radius >= self.screen_w - 10.0 {
                ball.x = self.screen_w - 10.0 - ball.radius;
                ball.vx = -ball.vx.abs();
                bounce_flashes.push((ball.x, ball.y));
                sound_queue.push(SoundEvent::BounceWall);
            }

            if ball.y - ball.radius <= 40.0 {
                ball.y = 40.0 + ball.radius;
                ball.vy = ball.vy.abs();
                bounce_flashes.push((ball.x, ball.y));
                sound_queue.push(SoundEvent::BounceWall);
            }

            if ball.y + ball.radius >= floor_y {
                if self.floor_shield_active {
                    ball.y = floor_y - ball.radius;
                    ball.vy = -ball.vy.abs();
                    bounce_flashes.push((ball.x, ball.y));
                    sound_queue.push(SoundEvent::BounceWall);
                    text_queue.push(("SHIELD REBOUND!".to_string(), ball.x, ball.y - 20.0, (100, 200, 255)));
                } else {
                    lost_balls_indices.push(ball_idx);
                    continue;
                }
            }

            let p_left = self.paddle.x - self.paddle.w * 0.5;
            let p_right = self.paddle.x + self.paddle.w * 0.5;
            let p_top = self.paddle.y - self.paddle.h * 0.5;
            let p_bottom = self.paddle.y + self.paddle.h * 0.5;

            if ball.x + ball.radius >= p_left
                && ball.x - ball.radius <= p_right
                && ball.y + ball.radius >= p_top
                && ball.y - ball.radius <= p_bottom
                && ball.vy > 0.0
            {
                sound_queue.push(SoundEvent::BouncePaddle);
                if self.paddle.sticky_active {
                    ball.is_stuck = true;
                    ball.stuck_offset_x = ball.x - self.paddle.x;
                    text_queue.push(("STUCK!".to_string(), ball.x, ball.y - 20.0, (255, 255, 100)));
                } else {
                    let hit_pos = (ball.x - self.paddle.x) / (self.paddle.w * 0.5);
                    let max_angle = 65.0_f32.to_radians();
                    let bounce_angle = hit_pos.clamp(-0.9, 0.9) * max_angle;

                    let speed = (ball.vx * ball.vx + ball.vy * ball.vy).sqrt() * ball.ball_type.speed_multiplier();
                    ball.vx = speed * bounce_angle.sin();
                    ball.vy = -speed * bounce_angle.cos().abs();

                    bounce_flashes.push((ball.x, ball.y));
                }
            }

            if let Some(ref mut boss) = self.boss {
                if boss.is_active {
                    let dx = ball.x - boss.x;
                    let dy = ball.y - boss.y;
                    if dx.abs() <= boss.w * 0.5 + ball.radius && dy.abs() <= boss.h * 0.5 + ball.radius {
                        boss.hp -= 2;
                        ball.vy = -ball.vy;
                        bounce_flashes.push((ball.x, ball.y));
                        explosion_queue.push((ball.x, ball.y, 12));
                        sound_queue.push(SoundEvent::Explosion);
                        if boss.hp <= 0 {
                            boss.is_active = false;
                            text_queue.push(("BOSS SLAIN!".to_string(), boss.x, boss.y, (255, 230, 50)));
                        }
                    }
                }
            }

            for h in &self.hazards {
                let dx = ball.x - h.x;
                let dy = ball.y - h.y;
                let dist_sq = dx * dx + dy * dy;
                let rad_sum = ball.radius + h.w * 0.4;
                if dist_sq <= rad_sum * rad_sum {
                    match h.hazard_type {
                        HazardType::Bumper => {
                            ball.vx = dx * 12.0;
                            ball.vy = dy * 12.0;
                            bounce_flashes.push((ball.x, ball.y));
                            sound_queue.push(SoundEvent::BounceWall);
                            text_queue.push(("BUMPER!".to_string(), ball.x, ball.y - 20.0, (255, 200, 50)));
                        }
                        HazardType::Mine => {
                            explosion_queue.push((h.x, h.y, 25));
                            sound_queue.push(SoundEvent::Explosion);
                            ball.vx = -ball.vx;
                            ball.vy = -ball.vy;
                            text_queue.push(("BOOM!".to_string(), h.x, h.y - 20.0, (255, 50, 50)));
                        }
                        _ => {}
                    }
                }
            }
        }

        for (x, y) in bounce_flashes {
            self.spawn_bounce_flash(x, y);
        }
        for (txt, x, y, col) in text_queue {
            self.spawn_floating_text(&txt, x, y, col);
        }
        for (x, y, cnt) in explosion_queue {
            self.spawn_explosion_particles(x, y, cnt);
        }
        for sfx in sound_queue {
            self.play_sfx(sfx);
        }

        for idx in lost_balls_indices.into_iter().rev() {
            self.balls.remove(idx);
        }

        if self.balls.is_empty() {
            self.lives -= 1;
            if self.lives <= 0 {
                self.state = GameState::GameOver;
                self.play_sfx(SoundEvent::GameOver);
            } else {
                self.reset_paddle_and_balls();
            }
        }

        let mut destroyed_bricks = Vec::new();
        let mut portal_warps = Vec::new();

        for ball in &mut self.balls {
            if ball.is_stuck {
                continue;
            }

            for b in &mut self.bricks {
                if b.is_destroyed {
                    continue;
                }

                let closest_x = ball.x.clamp(b.x, b.x + b.w);
                let closest_y = ball.y.clamp(b.y, b.y + b.h);

                let dx = ball.x - closest_x;
                let dy = ball.y - closest_y;
                let distance_sq = dx * dx + dy * dy;

                if distance_sq <= ball.radius * ball.radius {
                    if b.brick_type == BrickType::Portal {
                        portal_warps.push((b.id, ball.x));
                        continue;
                    }

                    if b.brick_type != BrickType::Unbreakable {
                        b.hp -= 1;
                        if b.hp <= 0 {
                            b.is_destroyed = true;
                            destroyed_bricks.push((b.x, b.y, b.w, b.h, b.brick_type, b.id));
                        }
                    }

                    if !ball.ball_type.is_piercing() {
                        let overlap_x = ball.radius - dx.abs();
                        let overlap_y = ball.radius - dy.abs();

                        if overlap_x < overlap_y {
                            ball.vx = -ball.vx;
                        } else {
                            ball.vy = -ball.vy;
                        }
                    }
                    break;
                }
            }
        }

        for (_portal_id, ball_x) in portal_warps {
            if let Some(other_portal) = self.bricks.iter().find(|b| !b.is_destroyed && b.brick_type == BrickType::Portal) {
                let px = other_portal.x + other_portal.w / 2.0;
                let py = other_portal.y + other_portal.h + 20.0;
                for ball in &mut self.balls {
                    if (ball.x - ball_x).abs() < 30.0 {
                        ball.x = px;
                        ball.y = py;
                        ball.vy = ball.vy.abs();
                    }
                }
                self.play_sfx(SoundEvent::Warp);
                self.spawn_floating_text("WARP!", px, py, (200, 100, 255));
            }
        }

        for (bx, by, bw, bh, btype, bid) in destroyed_bricks {
            self.on_brick_destroyed(bx, by, bw, bh, btype, bid);
        }

        let mut collected_powerups = Vec::new();
        for p in self.powerups.iter_mut() {
            if p.collected {
                continue;
            }
            p.y += p.vy * dt;
            p.rotation += 60.0 * dt as f64;

            let p_left = self.paddle.x - self.paddle.w * 0.5;
            let p_right = self.paddle.x + self.paddle.w * 0.5;
            let p_top = self.paddle.y - self.paddle.h * 0.5;
            let p_bottom = self.paddle.y + self.paddle.h * 0.5;

            if p.x + p.w * 0.5 >= p_left
                && p.x - p.w * 0.5 <= p_right
                && p.y + p.h * 0.5 >= p_top
                && p.y - p.h * 0.5 <= p_bottom
            {
                p.collected = true;
                collected_powerups.push(p.powerup_type);
            }
        }

        self.powerups.retain(|p| !p.collected && p.y <= self.screen_h);

        for ptype in collected_powerups {
            self.apply_powerup(ptype);
        }

        let mut i = 0;
        while i < self.particles.len() {
            let part = &mut self.particles[i];
            part.x += part.vx * dt;
            part.y += part.vy * dt;
            part.life += dt;
            part.alpha = 1.0 - (part.life / part.max_life).clamp(0.0, 1.0);
            part.rotation += part.rot_speed * dt as f64;
            if part.life >= part.max_life {
                self.particles.remove(i);
            } else {
                i += 1;
            }
        }

        let mut i = 0;
        while i < self.floating_texts.len() {
            let txt = &mut self.floating_texts[i];
            txt.y += txt.vy * dt;
            txt.life += dt;
            txt.alpha = 1.0 - (txt.life / txt.max_life).clamp(0.0, 1.0);
            if txt.life >= txt.max_life {
                self.floating_texts.remove(i);
            } else {
                i += 1;
            }
        }

        if self.mode == GameMode::Campaign {
            let remaining_destructible = self.bricks.iter().filter(|b| !b.is_destroyed && b.brick_type != BrickType::Unbreakable).count();
            let boss_cleared = self.boss.as_ref().map_or(true, |b| !b.is_active);

            if remaining_destructible == 0 && boss_cleared {
                if self.current_level >= self.max_levels {
                    self.state = GameState::Victory;
                    self.play_sfx(SoundEvent::Victory);
                } else {
                    self.init_level(self.current_level + 1);
                }
            }
        }
    }

    fn on_brick_destroyed(&mut self, bx: f32, by: f32, bw: f32, bh: f32, btype: BrickType, bid: usize) {
        self.combo += 1;
        self.combo_timer = 2.5;

        let points = btype.points() * self.score_multiplier * (1 + self.combo / 5);
        self.score += points;
        self.upgrades.credits += points / 10;

        self.play_sfx(SoundEvent::BrickDestroy);

        self.spawn_floating_text(
            &format!("+{}", points),
            bx + bw / 2.0,
            by,
            (255, 230, 100),
        );

        self.spawn_explosion_particles(bx + bw / 2.0, by + bh / 2.0, 15);

        if btype == BrickType::Explosive {
            self.trigger_shake(10.0, 0.4);
            self.play_sfx(SoundEvent::Explosion);
            self.spawn_floating_text("BOOM!", bx + bw / 2.0, by, (255, 80, 80));
            let radius = 180.0;
            let mut chain_destroys = Vec::new();

            for b in &mut self.bricks {
                if !b.is_destroyed && b.id != bid && b.brick_type != BrickType::Unbreakable {
                    let dx = (b.x + b.w / 2.0) - (bx + bw / 2.0);
                    let dy = (b.y + b.h / 2.0) - (by + bh / 2.0);
                    if dx * dx + dy * dy <= radius * radius {
                        b.is_destroyed = true;
                        chain_destroys.push((b.x, b.y, b.w, b.h, b.brick_type, b.id));
                    }
                }
            }
            for (cbx, cby, cbw, cbh, cbtype, cbid) in chain_destroys {
                self.on_brick_destroyed(cbx, cby, cbw, cbh, cbtype, cbid);
            }
        }

        let mut rng = rand::thread_rng();
        let drop_chance = if btype == BrickType::Powerup { 1.0 } else { 0.35 };

        if rng.gen_bool(drop_chance) {
            let ptypes = [
                PowerUpType::Multiball,
                PowerUpType::ExtraLife,
                PowerUpType::Expand,
                PowerUpType::Shrink,
                PowerUpType::Sticky,
                PowerUpType::Laser,
                PowerUpType::Fireball,
                PowerUpType::Slow,
                PowerUpType::Shield,
                PowerUpType::ScoreMultiplier,
            ];
            let chosen_type = ptypes[rng.gen_range(0..ptypes.len())];
            self.powerups.push(PowerUp {
                x: bx + bw / 2.0,
                y: by + bh / 2.0,
                vy: 180.0,
                w: 32.0,
                h: 46.0,
                powerup_type: chosen_type,
                collected: false,
                rotation: 0.0,
            });
        }
    }

    pub fn apply_powerup(&mut self, ptype: PowerUpType) {
        self.play_sfx(SoundEvent::PowerupPickup);
        self.spawn_floating_text(
            ptype.label(),
            self.paddle.x,
            self.paddle.y - 50.0,
            (100, 255, 100),
        );

        match ptype {
            PowerUpType::Multiball => {
                let mut new_balls = Vec::new();
                for b in &self.balls {
                    new_balls.push(Ball::new(b.x, b.y, b.vx * 0.8 + 100.0, -b.vy.abs(), BallType::Fast));
                    new_balls.push(Ball::new(b.x, b.y, b.vx * 0.8 - 100.0, -b.vy.abs(), BallType::Electric));
                }
                self.balls.extend(new_balls);
            }
            PowerUpType::ExtraLife => {
                self.lives += 1;
            }
            PowerUpType::Expand => {
                self.paddle.set_type(PaddleType::Long);
            }
            PowerUpType::Shrink => {
                self.paddle.set_type(PaddleType::Short);
            }
            PowerUpType::Sticky => {
                self.paddle.set_type(PaddleType::Sticky);
            }
            PowerUpType::Laser => {
                self.paddle.set_type(PaddleType::Laser);
            }
            PowerUpType::Fireball => {
                for b in &mut self.balls {
                    b.ball_type = BallType::Fire;
                }
            }
            PowerUpType::Slow => {
                for b in &mut self.balls {
                    b.vx *= 0.7;
                    b.vy *= 0.7;
                }
            }
            PowerUpType::Shield => {
                self.floor_shield_active = true;
                self.floor_shield_timer = 15.0;
            }
            PowerUpType::ScoreMultiplier => {
                self.score_multiplier = 2;
                self.multiplier_timer = 20.0;
            }
        }
    }

    fn spawn_bounce_flash(&mut self, x: f32, y: f32) {
        self.particles.push(Particle {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            scale: 0.8,
            alpha: 1.0,
            life: 0.0,
            max_life: 0.2,
            sprite_name: "fx_bounce_flash.png".to_string(),
            rotation: 0.0,
            rot_speed: 0.0,
        });
    }

    fn spawn_explosion_particles(&mut self, x: f32, y: f32, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(50.0..300.0);
            self.particles.push(Particle {
                x,
                y,
                vx: speed * angle.cos(),
                vy: speed * angle.sin(),
                scale: rng.gen_range(0.5..1.2),
                alpha: 1.0,
                life: 0.0,
                max_life: rng.gen_range(0.3..0.6),
                sprite_name: "fx_impact_spark.png".to_string(),
                rotation: rng.gen_range(0.0..360.0),
                rot_speed: rng.gen_range(-180.0..180.0),
            });
        }
    }

    fn spawn_floating_text(&mut self, text: &str, x: f32, y: f32, color: (u8, u8, u8)) {
        self.floating_texts.push(FloatingText {
            text: text.to_string(),
            x,
            y,
            vy: -60.0,
            color,
            alpha: 1.0,
            life: 0.0,
            max_life: 1.0,
        });
    }
}
