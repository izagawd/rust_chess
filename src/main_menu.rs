use std::rc::Rc;
use macroquad::color::{Color, BLUE, RED};
use nalgebra::Vector2;
use crate::rectangle_widget::RectangleWidget;
use crate::scene::{add_widget, Scene, SceneData};
use crate::widget::{Widget, WidgetVector};
use crate::widget::Side::{Center, Normal};

pub struct MainMenu {
    scene_data: SceneData
}

impl MainMenu {
    pub fn new() -> Rc<MainMenu> {
      Rc::new(MainMenu { scene_data: SceneData::new() })
    }
}
impl Scene for MainMenu {
    fn init(&self) {
        let created = add_widget(self, RectangleWidget::new(WidgetVector {
            side: Center,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(500.0,500.0),BLUE));


        let created = add_widget(self, RectangleWidget::new(WidgetVector {
            side: Center,
            offset: Vector2::new(0.0, 0.0),
        },Vector2::new(250.0,250.0),RED));
    }
    fn scene_data(&self) -> &SceneData {
        &self.scene_data
    }
}