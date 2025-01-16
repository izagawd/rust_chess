use std::cell::OnceCell;
use std::rc::Rc;
use macroquad::color::{Color, BLACK};
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton};
use nalgebra::Vector2;
use crate::chess::chess_game::ChessGame;
use crate::chess::chess_pieces::chess_piece::ChessColor;
use crate::level_utilities::scene::{add_widget, Scene, SceneData};
use crate::text_widget::TextWidget;
use crate::widget::Alignment::Center;
use crate::widget::WidgetVector;

pub struct WinnerScene{
    ///used, so that I can wait for a bit of delay after the scene is created to detect input
    time_since_start: OnceCell<f64>,
    winner_color: ChessColor,
    scene_data: SceneData
}
impl WinnerScene{
    pub fn new(winner_color: ChessColor)->Self{
        WinnerScene{
            time_since_start: OnceCell::new(),
            winner_color,
            scene_data: SceneData::new()
        }
    }
}
impl Scene for WinnerScene{
    fn init(self: Rc<Self>) {
        self.time_since_start.set(macroquad::time::get_time()).unwrap();
        add_widget(self.clone(),
        TextWidget::new(WidgetVector{
            offset: Vector2::new(0.0,0.0),
            alignment: Center
        },40.0,
        BLACK,
         format!("{} won!\nTap on screen to play again!",self.winner_color.to_string())));
    }
    fn update(self: Rc<Self>) {
        // adding delay before listening for input
        if is_mouse_button_pressed(MouseButton::Left) && macroquad::time::get_time() -
            self.time_since_start.get().unwrap() > 0.3{
            self.get_game().change_scene(Rc::new(ChessGame::new()))
        }
    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}