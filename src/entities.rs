#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddleType {
    Normal,
    Short,
    Long,
    Sticky,
    Shielded,
    Laser,
}

impl PaddleType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            PaddleType::Normal => "paddle_normal.png",
            PaddleType::Short => "paddle_short.png",
            PaddleType::Long => "paddle_long.png",
            PaddleType::Sticky => "paddle_sticky.png",
            PaddleType::Shielded => "paddle_shielded.png",
            PaddleType::Laser => "paddle_laser.png",
        }
    }

    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            PaddleType::Normal => (160.0, 36.0),
            PaddleType::Short => (110.0, 34.0),
            PaddleType::Long => (220.0, 38.0),
            PaddleType::Sticky => (170.0, 42.0),
            PaddleType::Shielded => (170.0, 42.0),
            PaddleType::Laser => (165.0, 48.0),
        }
    }
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
    pub paddle_type: PaddleType,
    pub laser_cooldown: f32,
    pub sticky_active: bool,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        let (w, h) = PaddleType::Normal.dimensions();
        Self {
            x,
            y,
            w,
            h,
            speed: 650.0,
            paddle_type: PaddleType::Normal,
            laser_cooldown: 0.0,
            sticky_active: false,
        }
    }

    pub fn set_type(&mut self, ptype: PaddleType) {
        self.paddle_type = ptype;
        let (w, h) = ptype.dimensions();
        self.w = w;
        self.h = h;
        self.sticky_active = ptype == PaddleType::Sticky;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallType {
    Normal,
    Fast,
    Fire,
    Ice,
    Electric,
}

impl BallType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            BallType::Normal => "ball_normal.png",
            BallType::Fast => "ball_fast.png",
            BallType::Fire => "ball_fire.png",
            BallType::Ice => "ball_ice.png",
            BallType::Electric => "ball_electric.png",
        }
    }

    pub fn speed_multiplier(&self) -> f32 {
        match self {
            BallType::Normal => 1.0,
            BallType::Fast => 1.45,
            BallType::Fire => 1.1,
            BallType::Ice => 0.9,
            BallType::Electric => 1.25,
        }
    }

    pub fn is_piercing(&self) -> bool {
        matches!(self, BallType::Fire)
    }
}

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub ball_type: BallType,
    pub is_stuck: bool,
    pub stuck_offset_x: f32,
    pub trail: Vec<(f32, f32)>,
}

impl Ball {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, ball_type: BallType) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            radius: 14.0,
            ball_type,
            is_stuck: false,
            stuck_offset_x: 0.0,
            trail: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_stuck {
            self.trail.push((self.x, self.y));
            if self.trail.len() > 10 {
                self.trail.remove(0);
            }
            self.x += self.vx * dt;
            self.y += self.vy * dt;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrickType {
    ColorRed,
    ColorOrange,
    ColorYellow,
    ColorGreen,
    ColorBlue,
    ColorPurple,
    ArmoredDark,
    ArmoredGold,
    Unbreakable,
    Explosive,
    Powerup,
    Moving,
    Portal,
}

impl BrickType {
    pub fn max_hp(&self) -> i32 {
        match self {
            BrickType::ArmoredDark => 2,
            BrickType::ArmoredGold => 3,
            BrickType::Unbreakable => 9999,
            _ => 1,
        }
    }

    pub fn base_sprite_name(&self) -> &'static str {
        match self {
            BrickType::ColorRed => "brick_red.png",
            BrickType::ColorOrange => "brick_orange.png",
            BrickType::ColorYellow => "brick_yellow.png",
            BrickType::ColorGreen => "brick_green.png",
            BrickType::ColorBlue => "brick_blue.png",
            BrickType::ColorPurple => "brick_purple.png",
            BrickType::ArmoredDark => "brick_armored_dark.png",
            BrickType::ArmoredGold => "brick_armored_gold.png",
            BrickType::Unbreakable => "brick_unbreakable.png",
            BrickType::Explosive => "brick_explosive.png",
            BrickType::Powerup => "brick_powerup.png",
            BrickType::Moving => "brick_moving.png",
            BrickType::Portal => "brick_portal_exit.png",
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            BrickType::ColorRed => 100,
            BrickType::ColorOrange => 90,
            BrickType::ColorYellow => 80,
            BrickType::ColorGreen => 70,
            BrickType::ColorBlue => 60,
            BrickType::ColorPurple => 50,
            BrickType::ArmoredDark => 250,
            BrickType::ArmoredGold => 500,
            BrickType::Explosive => 200,
            BrickType::Powerup => 150,
            BrickType::Moving => 300,
            BrickType::Portal => 180,
            BrickType::Unbreakable => 0,
        }
    }
}

pub struct Brick {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub brick_type: BrickType,
    pub hp: i32,
    pub max_hp: i32,
    pub is_destroyed: bool,
    pub move_dir: f32,
    pub min_x: f32,
    pub max_x: f32,
}

impl Brick {
    pub fn new(id: usize, x: f32, y: f32, w: f32, h: f32, brick_type: BrickType) -> Self {
        let max_hp = brick_type.max_hp();
        Self {
            id,
            x,
            y,
            w,
            h,
            brick_type,
            hp: max_hp,
            max_hp,
            is_destroyed: false,
            move_dir: 1.0,
            min_x: x - 100.0,
            max_x: x + 100.0,
        }
    }

    pub fn damage_overlay(&self) -> Option<&'static str> {
        if self.max_hp <= 1 || self.hp >= self.max_hp || self.brick_type == BrickType::Unbreakable {
            return None;
        }
        let ratio = self.hp as f32 / self.max_hp as f32;
        if ratio <= 0.34 {
            Some("brick_damage_3.png")
        } else if ratio <= 0.67 {
            Some("brick_damage_2.png")
        } else {
            Some("brick_damage_1.png")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpType {
    Multiball,
    ExtraLife,
    Expand,
    Shrink,
    Sticky,
    Laser,
    Fireball,
    Slow,
    Shield,
    ScoreMultiplier,
}

impl PowerUpType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            PowerUpType::Multiball => "powerup_multiball.png",
            PowerUpType::ExtraLife => "powerup_extra_life.png",
            PowerUpType::Expand => "powerup_expand.png",
            PowerUpType::Shrink => "powerup_shrink.png",
            PowerUpType::Sticky => "powerup_sticky.png",
            PowerUpType::Laser => "powerup_laser.png",
            PowerUpType::Fireball => "powerup_fireball.png",
            PowerUpType::Slow => "powerup_slow.png",
            PowerUpType::Shield => "powerup_shield.png",
            PowerUpType::ScoreMultiplier => "powerup_score_x.png",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PowerUpType::Multiball => "MULTI-BALL!",
            PowerUpType::ExtraLife => "EXTRA LIFE!",
            PowerUpType::Expand => "EXPAND PADDLE!",
            PowerUpType::Shrink => "SHRINK PADDLE!",
            PowerUpType::Sticky => "STICKY PADDLE!",
            PowerUpType::Laser => "LASER CANNONS!",
            PowerUpType::Fireball => "FIREBALL!",
            PowerUpType::Slow => "SLO-MO BALL!",
            PowerUpType::Shield => "SAFETY SHIELD!",
            PowerUpType::ScoreMultiplier => "SCORE 2X!",
        }
    }
}

pub struct PowerUp {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub w: f32,
    pub h: f32,
    pub powerup_type: PowerUpType,
    pub collected: bool,
    pub rotation: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HazardType {
    Spikes,
    Mine,
    Bumper,
    Electric,
    Turret,
}

impl HazardType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            HazardType::Spikes => "hazard_spikes.png",
            HazardType::Mine => "hazard_mine.png",
            HazardType::Bumper => "hazard_bumper.png",
            HazardType::Electric => "hazard_electric.png",
            HazardType::Turret => "hazard_turret.png",
        }
    }
}

pub struct Hazard {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub hazard_type: HazardType,
    pub angle: f64,
    pub shoot_timer: f32,
}

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub w: f32,
    pub h: f32,
    pub from_player: bool,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub scale: f32,
    pub alpha: f32,
    pub life: f32,
    pub max_life: f32,
    pub sprite_name: String,
    pub rotation: f64,
    pub rot_speed: f64,
}

pub struct FloatingText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub color: (u8, u8, u8),
    pub alpha: f32,
    pub life: f32,
    pub max_life: f32,
}
