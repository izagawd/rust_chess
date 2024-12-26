
#![feature(trait_upcasting)]
#![feature(specialization)]
#![feature(let_chains)]

use crate::game::Game;
use crate::main_menu::MainMenu;
use macroquad::prelude::*;
use crate::widget::Widget;

mod widget;
mod game;
mod scene;
mod main_menu;
mod rectangle_widget;
mod text_widget;
mod ImageWidget;

mod chess{

    pub mod chess_pieces{
        pub mod chess_piece;
        pub mod pawn;
    }
    pub mod chess_game;
    pub mod chess_slot;
    pub mod chess_board;
}

#[macroquad::main("MyGame")]
async fn main() {
    unsafe {std::env::set_var("debug_refcell", "true")};
    let game = Game::new(MainMenu::new());
    loop {
        game.clone().update();
        game.render();

        next_frame().await
    }
}
