use std::rc::Rc;
use macroquad::color::{Color, BLACK};
use macroquad::input::{is_key_pressed, KeyCode};
use nalgebra::Vector2;
use crate::chess::chess_game::ChessGame;
use crate::chess::chess_pieces::chess_piece::ChessColor;
use crate::scene::{add_widget, Scene, SceneData};
use crate::text_widget::TextWidget;
use crate::widget::Alignment::Center;
use crate::widget::WidgetVector;

pub struct WinnerScene{
    winner_color: ChessColor,
    scene_data: SceneData
}
impl WinnerScene{
    pub fn new(winner_color: ChessColor)->Self{
        WinnerScene{
            winner_color,
            scene_data: SceneData::new()
        }
    }
}
impl Scene for WinnerScene{
    fn init(self: Rc<Self>) {
        let new_text_widget = add_widget(self.clone(),
        TextWidget::new(WidgetVector{
            offset: Vector2::new(0.0,0.0),
            alignment: Center
        },40.0,
        BLACK,
         format!("{} won!\nPress R to play again",self.winner_color.to_string())));
    }
    fn update(self: Rc<Self>) {
        if is_key_pressed(KeyCode::R){
            self.get_game().change_scene(Rc::new(ChessGame::new()))
        }
    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}