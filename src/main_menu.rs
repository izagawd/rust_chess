use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::RwLock;
use macroquad::color::{Color, BLUE, RED, WHITE};
use macroquad::input::{is_mouse_button_down, MouseButton};
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_game::ChessGame;
use crate::rectangle_widget::RectangleWidget;
use crate::scene::{add_widget, Scene, SceneData};
use crate::text_widget::TextWidget;
use crate::widget::{Widget, WidgetVector};
use crate::widget::Side::{Center, Normal, TopCenter};

/**
* scene for the main menu
*/
pub struct MainMenu {
    play_button_widget: OnceCell<Weak<RectangleWidget>>,
    scene_data: SceneData
}

impl MainMenu {
    pub fn new() -> Rc<MainMenu> {
      Rc::new(MainMenu { scene_data: SceneData::new(), play_button_widget: OnceCell::new() })
    }
}

static HEY : RwLock<i32> = RwLock::new(0);
impl Scene for MainMenu {
    fn update(&self) {
       let za_play = self.play_button_widget.get().and_then(|w| w.upgrade());
        if let Some(za_play) = za_play {
            if za_play.is_hovered_on() && is_mouse_button_down(MouseButton::Left){
                self.get_game().change_scene(Rc::new(ChessGame::new()));
            };
        }

    }
    fn init(&self) {
        // initializing the play button,
        let rectangle = add_widget(self, RectangleWidget::new(WidgetVector {
            side: Center,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(500.0,500.0),BLUE));


        let play_button = add_widget(self, RectangleWidget::new(WidgetVector {
            side: Center,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(300.0,60.0),RED));


        let play_text = add_widget(self, TextWidget::new(WidgetVector {
            side: Center,
            offset: Vector2::new(0.0, 0.0),
        },50.0,WHITE,String::from("PLAY")));

        play_button.clone().add_child(play_text.clone()).expect("ugh");
        self.play_button_widget.set(Rc::downgrade(&play_button));
        rectangle.add_child(play_button).expect("ugh");

    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}