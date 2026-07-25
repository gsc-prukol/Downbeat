use macroquad::prelude::*;

#[macroquad::main("Downbeat")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}