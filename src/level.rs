use crate::entities::{Brick, BrickType, Hazard, HazardType};

pub struct Level {
    pub name: &'static str,
    pub bricks: Vec<Brick>,
    pub hazards: Vec<Hazard>,
}

pub fn load_level(level_num: u32, screen_w: f32, screen_h: f32) -> Level {
    match level_num {
        1 => level_1(screen_w, screen_h),
        2 => level_2(screen_w, screen_h),
        3 => level_3(screen_w, screen_h),
        4 => level_4(screen_w, screen_h),
        _ => level_5(screen_w, screen_h),
    }
}

fn level_1(screen_w: f32, _screen_h: f32) -> Level {
    let mut bricks = Vec::new();
    let rows = 5;
    let cols = 10;
    let brick_w = 84.0;
    let brick_h = 34.0;
    let spacing_x = 10.0;
    let spacing_y = 10.0;
    let start_x = (screen_w - (cols as f32 * (brick_w + spacing_x) - spacing_x)) / 2.0;
    let start_y = 90.0;

    let types = [
        BrickType::ColorRed,
        BrickType::ColorOrange,
        BrickType::ColorYellow,
        BrickType::ColorGreen,
        BrickType::ColorBlue,
    ];

    let mut id = 0;
    for r in 0..rows {
        for c in 0..cols {
            let x = start_x + c as f32 * (brick_w + spacing_x);
            let y = start_y + r as f32 * (brick_h + spacing_y);
            let mut btype = types[r % types.len()];

            if (r == 1 && c == 2) || (r == 3 && c == 7) {
                btype = BrickType::Powerup;
            } else if (r == 2 && c == 4) || (r == 2 && c == 5) {
                btype = BrickType::ArmoredDark;
            }

            bricks.push(Brick::new(id, x, y, brick_w, brick_h, btype));
            id += 1;
        }
    }

    Level {
        name: "Stage 1: Neon Genesis Grid",
        bricks,
        hazards: Vec::new(),
    }
}

fn level_2(screen_w: f32, _screen_h: f32) -> Level {
    let mut bricks = Vec::new();
    let rows = 6;
    let cols = 10;
    let brick_w = 84.0;
    let brick_h = 34.0;
    let spacing_x = 10.0;
    let spacing_y = 10.0;
    let start_x = (screen_w - (cols as f32 * (brick_w + spacing_x) - spacing_x)) / 2.0;
    let start_y = 90.0;

    let mut id = 0;
    for r in 0..rows {
        for c in 0..cols {
            let x = start_x + c as f32 * (brick_w + spacing_x);
            let y = start_y + r as f32 * (brick_h + spacing_y);

            let btype = if (c == 0 || c == cols - 1) && (r == 0 || r == rows - 1) {
                BrickType::Unbreakable
            } else if (r == 2 && (c == 4 || c == 5)) || (r == 3 && (c == 4 || c == 5)) {
                BrickType::Explosive
            } else if r == 0 {
                BrickType::ArmoredGold
            } else if r == 1 {
                BrickType::ArmoredDark
            } else if (c + r) % 3 == 0 {
                BrickType::Powerup
            } else {
                BrickType::ColorPurple
            };

            bricks.push(Brick::new(id, x, y, brick_w, brick_h, btype));
            id += 1;
        }
    }

    let hazards = vec![
        Hazard {
            x: screen_w * 0.25,
            y: 360.0,
            w: 50.0,
            h: 50.0,
            hazard_type: HazardType::Bumper,
            angle: 0.0,
            shoot_timer: 0.0,
        },
        Hazard {
            x: screen_w * 0.75,
            y: 360.0,
            w: 50.0,
            h: 50.0,
            hazard_type: HazardType::Bumper,
            angle: 0.0,
            shoot_timer: 0.0,
        },
        Hazard {
            x: screen_w * 0.5,
            y: 390.0,
            w: 55.0,
            h: 55.0,
            hazard_type: HazardType::Mine,
            angle: 0.0,
            shoot_timer: 0.0,
        },
    ];

    Level {
        name: "Stage 2: Explosive Citadel",
        bricks,
        hazards,
    }
}

fn level_3(screen_w: f32, _screen_h: f32) -> Level {
    let mut bricks = Vec::new();
    let rows = 6;
    let cols = 11;
    let brick_w = 78.0;
    let brick_h = 32.0;
    let spacing_x = 8.0;
    let spacing_y = 8.0;
    let start_x = (screen_w - (cols as f32 * (brick_w + spacing_x) - spacing_x)) / 2.0;
    let start_y = 85.0;

    let mut id = 0;
    for r in 0..rows {
        for c in 0..cols {
            let x = start_x + c as f32 * (brick_w + spacing_x);
            let y = start_y + r as f32 * (brick_h + spacing_y);

            let btype = if r == 2 && (c == 2 || c == cols - 3) {
                BrickType::Portal
            } else if r == 1 && (c % 2 == 1) {
                BrickType::Moving
            } else if r == 0 {
                BrickType::ArmoredGold
            } else if r == 3 {
                BrickType::ColorOrange
            } else if r == 4 {
                BrickType::ColorGreen
            } else {
                BrickType::ColorBlue
            };

            bricks.push(Brick::new(id, x, y, brick_w, brick_h, btype));
            id += 1;
        }
    }

    let hazards = vec![
        Hazard {
            x: screen_w * 0.5,
            y: 350.0,
            w: 120.0,
            h: 40.0,
            hazard_type: HazardType::Electric,
            angle: 0.0,
            shoot_timer: 0.0,
        },
    ];

    Level {
        name: "Stage 3: Quantum Warp Core",
        bricks,
        hazards,
    }
}

fn level_4(screen_w: f32, _screen_h: f32) -> Level {
    let mut bricks = Vec::new();
    let rows = 7;
    let cols = 11;
    let brick_w = 78.0;
    let brick_h = 32.0;
    let spacing_x = 8.0;
    let spacing_y = 8.0;
    let start_x = (screen_w - (cols as f32 * (brick_w + spacing_x) - spacing_x)) / 2.0;
    let start_y = 80.0;

    let mut id = 0;
    for r in 0..rows {
        for c in 0..cols {
            let x = start_x + c as f32 * (brick_w + spacing_x);
            let y = start_y + r as f32 * (brick_h + spacing_y);

            let btype = if (c == 5) && (r == 1 || r == 3 || r == 5) {
                BrickType::Explosive
            } else if (r == 0) || (r == rows - 1) {
                BrickType::ArmoredDark
            } else if c == 0 || c == cols - 1 {
                BrickType::Unbreakable
            } else if (r + c) % 4 == 0 {
                BrickType::Powerup
            } else {
                BrickType::ColorRed
            };

            bricks.push(Brick::new(id, x, y, brick_w, brick_h, btype));
            id += 1;
        }
    }

    let hazards = vec![
        Hazard {
            x: 150.0,
            y: 380.0,
            w: 60.0,
            h: 60.0,
            hazard_type: HazardType::Turret,
            angle: 0.0,
            shoot_timer: 1.5,
        },
        Hazard {
            x: screen_w - 150.0,
            y: 380.0,
            w: 60.0,
            h: 60.0,
            hazard_type: HazardType::Turret,
            angle: 0.0,
            shoot_timer: 3.0,
        },
    ];

    Level {
        name: "Stage 4: Turret Fortress",
        bricks,
        hazards,
    }
}

fn level_5(screen_w: f32, _screen_h: f32) -> Level {
    let mut bricks = Vec::new();
    let rows = 7;
    let cols = 12;
    let brick_w = 72.0;
    let brick_h = 30.0;
    let spacing_x = 8.0;
    let spacing_y = 8.0;
    let start_x = (screen_w - (cols as f32 * (brick_w + spacing_x) - spacing_x)) / 2.0;
    let start_y = 80.0;

    let mut id = 0;
    for r in 0..rows {
        for c in 0..cols {
            let x = start_x + c as f32 * (brick_w + spacing_x);
            let y = start_y + r as f32 * (brick_h + spacing_y);

            let btype = if (c == 2 || c == cols - 3) && (r == 2 || r == 4) {
                BrickType::Portal
            } else if (c == 3 || c == cols - 4) && (r == 3) {
                BrickType::Explosive
            } else if r == 0 || r == 1 {
                BrickType::ArmoredGold
            } else if (r + c) % 3 == 0 {
                BrickType::Powerup
            } else {
                BrickType::ColorPurple
            };

            bricks.push(Brick::new(id, x, y, brick_w, brick_h, btype));
            id += 1;
        }
    }

    let hazards = vec![
        Hazard {
            x: screen_w * 0.3,
            y: 370.0,
            w: 55.0,
            h: 55.0,
            hazard_type: HazardType::Mine,
            angle: 0.0,
            shoot_timer: 0.0,
        },
        Hazard {
            x: screen_w * 0.7,
            y: 370.0,
            w: 55.0,
            h: 55.0,
            hazard_type: HazardType::Mine,
            angle: 0.0,
            shoot_timer: 0.0,
        },
        Hazard {
            x: screen_w * 0.5,
            y: 400.0,
            w: 60.0,
            h: 60.0,
            hazard_type: HazardType::Turret,
            angle: 0.0,
            shoot_timer: 2.0,
        },
    ];

    Level {
        name: "Stage 5: Master Ballscape Omega",
        bricks,
        hazards,
    }
}
