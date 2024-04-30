extern crate piston_window;
extern crate rand;
// ties our other files to the main file
mod draw;
mod snake;
mod game;
// piston window is a game engine inside of rust, it actually creates the window that allows
// us to play the snake game
use piston_window::*;
use piston_window::types::Color;
use piston_window::Button;
use crate::game::Game;
use crate::draw::to_coord_u32;
// background color
const BACK_COLOR: Color = [0.0, 0.5, 0.5, 0.5];



fn main() {
    // we have to initialize what the actual width and height of the game will be 
    // in reference to the game windown(through the piston window framework)
    let (width, height) = (30, 30);
    // this actually creates the piston window(how to initialize it in accordance with our game coordinates)
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    // here we use the height and width we declared earlier to actually create the new game with
    // this height and weight
    let mut game = Game::new(width, height);
    // this is the game loop that allows the game to stay running
    while let Some(event) = window.next() {
        // this takes in keyboard input whenever a key is pressed and potentially
        // maps it to an action in the game, such as the arrow keys controlling where the snake
        // moves
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        // draws the game on the window 
        window.draw_2d(&event, |c, g, _| {
            // utilizes the background color we established
            clear(BACK_COLOR, g);
            // actually draws the different elements of the game we've specified in other classes
            game.draw(&c, g);
        });
        // updates the state of game. Required for smooth running of the game
        event.update(|arg| {
            game.update(arg.dt);
        });
}
}