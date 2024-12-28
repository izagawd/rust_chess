use std::cell::RefCell;
use std::rc::Rc;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::prelude::is_key_down;
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::scene::{Scene, SceneData};
use crate::widget::{Widget, WidgetVector};
use crate::widget::Alignment::Center;

pub struct ChessGame {
    scene_data: SceneData,

}

impl ChessGame {

    pub fn new() -> ChessGame {
        Self{
            scene_data: SceneData::new(),
        }
    }
}
impl Scene for ChessGame {
    fn update(self: Rc<Self>) {
        if is_key_pressed(KeyCode::R){
            self.get_game().change_scene(Rc::new(ChessGame::new()));
        }
    }
    fn init(self: Rc<Self>) {
        let board = ChessBoard::new(self);
        board.set_local_position(WidgetVector{
            alignment: Center,
            offset: Vector2::new(0.0,0.0)
        })
    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}