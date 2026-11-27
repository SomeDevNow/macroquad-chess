
// this is a reusable library file for handling textures atlases

use macroquad::prelude::*;

pub struct Quad {
    rect: Rect,
    color: Color,
    texture: Texture2D,
    texture_rect: Rect,
}

impl Quad {
    pub fn new(rect: Rect, color: Color, texture: Texture2D, texture_rect: Rect) -> Self {
        return Self { rect: rect, color: color, texture: texture, texture_rect: texture_rect };
    }

    pub fn draw(&self) {
        draw_texture_ex(&self.texture, self.rect.x, self.rect.y, self.color, DrawTextureParams { 
            source: Some(self.texture_rect),
             ..Default::default() 
            });
    }
}