
use crate::utils::quad::*;
use macroquad::prelude::*;

const GRID_SPACE: Vec2 = Vec2::new(0.0,0.0);
const GRID_OFFSET: Vec2 = Vec2::new(192.0, 52.0);
const GRID_SIZE: f32 = 32.0;
const GRID_SCALE: f32 = 1.0;
const GRID_COLOR_1: Color = Color::from_hex(0xe0d3c8);
const GRID_COLOR_2: Color = Color::from_hex(0x452b3f);

#[derive(Default)]
pub struct GameHandler { 

}

impl GameHandler {
    pub fn new() -> Self {
        return Self { 

        };
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&self) {
        //draw grid
        for  i in 0..8 {
            for j in 0..8 {
                draw_rectangle(
                    GRID_OFFSET.x + GRID_SCALE*((GRID_SPACE.x+GRID_SIZE)*i as f32), 
                    GRID_OFFSET.y + GRID_SCALE*((GRID_SPACE.y+GRID_SIZE)*j as f32), 
                    GRID_SCALE*GRID_SIZE, GRID_SCALE*GRID_SIZE, 
                    if (i+j)%2 == 0 {GRID_COLOR_1} else {GRID_COLOR_2}
                );
            }
        }
    }
}