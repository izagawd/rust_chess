
#![feature(trait_upcasting)]
#![feature(specialization)]
#![feature(let_chains)]
#![feature(gen_blocks)]

use crate::game::Game;
use crate::main_menu::MainMenu;
use macroquad::prelude::*;
use crate::widget::WidgetData;

mod widget;
mod game;
mod scene;
mod main_menu;
mod rectangle_widget;
mod text_widget;


mod chess{
    pub mod chess_pieces{
        pub mod king;
        pub mod queen;
        pub mod knight;
        pub mod bishop;
        pub mod rook;
        pub mod chess_piece;
        pub mod pawn;
    }
    pub mod chess_game;
    pub mod chess_slot;
    pub mod chess_board;
}

#[macroquad::main("Legendary Chess")]
async fn main() {
    println!("{}",size_of::<WidgetData>());
    let game = Game::new(MainMenu::new());
    loop {
        game.clone().update();
        game.render();

        next_frame().await
    }
}
