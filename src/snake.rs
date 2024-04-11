use std::collections::LinkedList;
use piston_window::{Context,G2d};
use piston_window::types::Color;

use draw::draw_block;
// create the color of the snake
const SNAKE_COLOR: Color = [0.50,0.00,0.05,1.0];
// be able to map directions of the keys
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl Direction{
    // makes sure that if the snake for example is going down,
    // it can't go up
    pub fn opposite(self) -> Direction{
        match *self {
            Direction::Up=> Direction::Down,
            Direction::Down=> Direction::Up,
            Direction::Left=> Direction::Right,
            Direction::Right=> Direction::Left,
        }
    }
}