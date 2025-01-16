use std::cell::{Cell, OnceCell, RefCell};
use std::iter::Once;
use std::rc::Rc;
use macroquad::color::{BLACK, GRAY, RED, WHITE};
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton};
use macroquad::prelude::is_key_down;
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::level_utilities::scene::{add_widget, Scene, SceneData};
use crate::rectangle_widget::{ColorHandler, RectangleWidget};

use crate::text_widget::TextWidget;
use crate::widget::{Widget, WidgetVector};
use crate::widget::Alignment::{Center, TopLeft, TopCenter, TopRight};

pub struct ChessGame {
    scene_data: SceneData,
    restart_button: OnceCell<Rc<RectangleWidget>>,
    move_helper_toggle_button: OnceCell<Rc<RectangleWidget>>,
}

impl ChessGame {

    pub fn new() -> ChessGame {
        Self{
            restart_button: OnceCell::new(),
            scene_data: SceneData::new(),
            move_helper_toggle_button: OnceCell::new(),
        }
    }
}
/// Whether or not the move helper is on
thread_local! {

    pub static  MOVE_HELPER : Cell<bool> = Cell::new(true);
}
impl Scene for ChessGame {
    fn update(self: Rc<Self>) {
        if self.restart_button.get().unwrap().is_hovered_on() && is_mouse_button_pressed(MouseButton::Left){
            self.get_game().change_scene(Rc::new(ChessGame::new()));
        }
        if self.move_helper_toggle_button.get().unwrap().is_hovered_on() && is_mouse_button_pressed(MouseButton::Left){
            MOVE_HELPER.set(!MOVE_HELPER.get());
        }
    }
    fn init(self: Rc<Self>) {
        let board = ChessBoard::new(self.clone());
        board.set_local_position(WidgetVector{
            alignment: Center,
            offset: Vector2::new(0.0,0.0)
        });
        let restart_button = add_widget(self.clone(), RectangleWidget::new(WidgetVector {
            alignment: TopLeft,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(250.0,40.0),ColorHandler::Value(GRAY)));

        // creating the play button text
        let restart_text = add_widget(self.clone(), TextWidget::new(WidgetVector {
            alignment: Center,
            offset: Vector2::new(0.0, 0.0),
        },50.0,BLACK,String::from("RESTART")));

        restart_text.set_parent(Some(restart_button.clone())).unwrap();

        let toggle_move_helper_button = add_widget(self.clone(), RectangleWidget::new(WidgetVector {
            alignment: TopRight,
            offset: Vector2::new(0.0, 0.0),
        }, Vector2::new(300.0,40.0), ColorHandler::Value(GRAY)));

        // creating the play button text
        let toggle_move_helper_text = add_widget(self.clone(), TextWidget::new(WidgetVector {
            alignment: Center,
            offset: Vector2::new(0.0, 0.0),
        }, 30.0, BLACK, String::from("Toggle Move Helper")));

        toggle_move_helper_text.set_parent(Some(toggle_move_helper_button.clone())).unwrap();
        if let Err(_) = self.restart_button.set(restart_button){
            panic!("OnceCell error occured while trying to cache the restart button, which usually shouldn't happen")
        };
        if let Err(_) = self.move_helper_toggle_button.set(toggle_move_helper_button){
            panic!("OnceCell error occured while trying to cache the move helper toggle button, which usually shouldn't happen")
        }
    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}