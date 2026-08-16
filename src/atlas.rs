#![allow(dead_code)]
use std::collections::HashMap;
use std::path::Path;
use serde::Deserialize;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasFrameRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasSize {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasPivot {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasFrame {
    pub frame: AtlasFrameRect,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(default)]
    pub pivot: Option<AtlasPivot>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasMeta {
    pub image: String,
    pub size: AtlasSize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasData {
    pub frames: HashMap<String, AtlasFrame>,
    pub meta: AtlasMeta,
}

pub struct SpriteAtlas {
    pub data: AtlasData,
}

impl SpriteAtlas {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file_content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let data: AtlasData = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
        Ok(Self { data })
    }

    pub fn get_frame(&self, name: &str) -> Option<&AtlasFrame> {
        self.data.frames.get(name)
    }

    pub fn render_sprite(
        &self,
        canvas: &mut Canvas<Window>,
        texture: &Texture,
        sprite_name: &str,
        dest_rect: Rect,
        angle: f64,
        center: Option<sdl2::rect::Point>,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) {
        if let Some(frame_def) = self.get_frame(sprite_name) {
            let src_rect = Rect::new(
                frame_def.frame.x,
                frame_def.frame.y,
                frame_def.frame.w,
                frame_def.frame.h,
            );
            let _ = canvas.copy_ex(
                texture,
                Some(src_rect),
                Some(dest_rect),
                angle,
                center,
                flip_horizontal,
                flip_vertical,
            );
        }
    }
}
