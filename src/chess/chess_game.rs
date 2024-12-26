use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::scene::{Scene, SceneData};
use crate::widget::{Widget, WidgetVector};
use crate::widget::Side::Center;

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
    fn init(&self) {
        let board = ChessBoard::new(self);
        board.set_local_position(WidgetVector{
            side: Center,
            offset: Vector2::new(0.0,0.0)
        })
    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}