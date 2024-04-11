use piston_window::{rectangle, Context, G2d}; // ability to draw 2d content, G2d actually draws it (graphics)
use piston_window::types::{Color, Width}; // allow us to color 

const BLOCK_SIZE: f64 = 25.0; // blocks will scale up 25 pixles
// game coordinate and multiply by our block size, pub keyword makes the function public to our entire program
pub fn to_coord(game_coord: i32) -> f64{
    (game_coord as f64) * BLOCK_SIZE
}
// imputting a color, x and y, graphics 2d buffer
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d){
    // factor the coordinates by block size, a lot of technical graphics stuff
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    // pass in parameters for the rectangle (width, height, xval, yval, graphics)
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn  draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d
){
    let x = to_coord(x);
    let y = to_coord(y);
    // allows us to control the size of the board
    rectangle(
        color,
        [   
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        
        ],
        con.transform,
        g,

    );
}

