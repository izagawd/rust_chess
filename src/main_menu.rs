use crate::chess::chess_game::ChessGame;
use crate::rectangle_widget::{ColorHandler, RectangleWidget};

use crate::text_widget::TextWidget;
use crate::widget::Alignment::Center;
use crate::widget::{Widget, WidgetVector};
use macroquad::color::{BLUE, RED, WHITE};
use macroquad::input::{is_mouse_button_down, MouseButton};
use nalgebra::Vector2;
use std::cell::{OnceCell, RefCell};
use std::rc::{Rc, Weak};
use crate::level_utilities::scene::{add_widget, Scene, SceneData};

/**
* scene for the main menu
*/
pub struct MainMenu {
    play_button_widget: OnceCell<Weak<RectangleWidget>>,
    scene_data: SceneData
}

impl MainMenu {
    pub fn new() -> Rc<MainMenu> {
      Rc::new(MainMenu { scene_data:SceneData::new(), play_button_widget: OnceCell::new() })
    }
}


impl Scene for MainMenu {
    fn update(self: Rc<Self>) {
       let za_play = self.play_button_widget.get().and_then(|w| w.upgrade());
        if let Some(za_play) = za_play {
            if za_play.is_hovered_on() && is_mouse_button_down(MouseButton::Left){
                self.get_game().change_scene(Rc::new(ChessGame::new()));
            };
        }

    }
    fn init(self: Rc<Self>) {
        // initializing the play button,
        let rectangle = add_widget(self.clone(), RectangleWidget::new(WidgetVector {
            alignment: Center,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(500.0,500.0),ColorHandler::Value(BLUE)));


        // creating the play button
        let play_button = add_widget(self.clone(), RectangleWidget::new(WidgetVector {
            alignment: Center,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(300.0,60.0),ColorHandler::Value(RED)));

        // creating the play button text
        let play_text = add_widget(self.clone(), TextWidget::new(WidgetVector {
            alignment: Center,
            offset: Vector2::new(0.0, 0.0),
        },50.0,WHITE,String::from("PLAY")));

        play_text.clone().set_parent(Some(play_button.clone())).unwrap();

        self.play_button_widget.set(Rc::downgrade(&play_button)).unwrap();
        play_button.set_parent(Some(rectangle)).unwrap();


    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}