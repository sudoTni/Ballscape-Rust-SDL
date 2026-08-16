use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use crate::entities::{Brick, BrickType};

#[derive(Serialize, Deserialize)]
pub struct CustomLevelBrick {
    pub x: f32,
    pub y: f32,
    pub brick_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomLevel {
    pub name: String,
    pub bricks: Vec<CustomLevelBrick>,
}

pub fn save_custom_level(bricks: &[Brick], filename: &str) -> Result<(), String> {
    let mut custom_bricks = Vec::new();
    for b in bricks {
        if !b.is_destroyed {
            custom_bricks.push(CustomLevelBrick {
                x: b.x,
                y: b.y,
                brick_type: b.brick_type.base_sprite_name().to_string(),
            });
        }
    }

    let level_data = CustomLevel {
        name: "User Custom Stage".to_string(),
        bricks: custom_bricks,
    };

    let json_str = serde_json::to_string_pretty(&level_data).map_err(|e| e.to_string())?;
    let mut file = File::create(filename).map_err(|e| e.to_string())?;
    file.write_all(json_str.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_custom_level(filename: &str) -> Result<Vec<Brick>, String> {
    let mut file = File::open(filename).map_err(|e| e.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| e.to_string())?;

    let level_data: CustomLevel = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let brick_w = 84.0;
    let brick_h = 34.0;
    let mut bricks = Vec::new();

    for (id, cb) in level_data.bricks.iter().enumerate() {
        let btype = match cb.brick_type.as_str() {
            "brick_red.png" => BrickType::ColorRed,
            "brick_orange.png" => BrickType::ColorOrange,
            "brick_yellow.png" => BrickType::ColorYellow,
            "brick_green.png" => BrickType::ColorGreen,
            "brick_blue.png" => BrickType::ColorBlue,
            "brick_purple.png" => BrickType::ColorPurple,
            "brick_armored_dark.png" => BrickType::ArmoredDark,
            "brick_armored_gold.png" => BrickType::ArmoredGold,
            "brick_unbreakable.png" => BrickType::Unbreakable,
            "brick_explosive.png" => BrickType::Explosive,
            "brick_powerup.png" => BrickType::Powerup,
            "brick_moving.png" => BrickType::Moving,
            "brick_portal_exit.png" => BrickType::Portal,
            _ => BrickType::ColorRed,
        };
        bricks.push(Brick::new(id, cb.x, cb.y, brick_w, brick_h, btype));
    }

    Ok(bricks)
}
