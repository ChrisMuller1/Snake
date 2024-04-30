use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw::draw_block;
// color of snake
const SNAKE_COLOR: Color = [0.68, 0.80, 0.90, 1.0];
#[derive(Copy, Clone, PartialEq)]
// directions of keys
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// will be necessary to check if the snake is moving in the opposite direction of itself
// later on in the code
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    // method called new with x and y value. This method creates the initial
    // starting state of the snake that will have a horizontal length of 3.**
    pub fn new(x: i32, y: i32) -> Snake {
        // creating a body that is a linked list of blocks
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });

        Snake {
            // the starting direction of the snake will be moving to the right **
            direction: Direction::Right,
            body,
            tail: None,
        }
    }


    // creates new blocks that go to the head of snake that get taken from the back of the snake
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        // push the new block to the front
        self.body.push_front(new_block);
        // pops back of the snake
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    } 
    // Cloning a direction
    pub fn head_direction(&self) -> Direction {
        self.direction
    }
    // this function actually draws the blocks of the snake body
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
    // push cloned tail to back of the body
    // tail doesn't get rendered unless we eat an apple
    // when apple is eaten then the current tail is pushed into the snake
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // determines where the next head of the snake will be depending on the direction
    // that the snake is traveling in. Necessary for game logic of snake's movements 
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();
        // get snake direction
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
     // returns boolean value
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut z = 0;
        // iterate through body
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            z += 1;
            // if the snake overlaps, break
            if z == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}