use std::rc::Rc;
use macroquad::color::{BLACK, BLUE, DARKGRAY, GRAY, LIGHTGRAY, RED, WHITE};
use nalgebra::Vector2;
use crate::rectangle_widget::RectangleWidget;
use crate::widget::{Widget, WidgetData, WidgetVector};
use crate::widget::Side::Normal;

pub struct ChessSlot{
    rectangle: RectangleWidget,
    position: Vector2<i32>,
}


impl ChessSlot{
    pub fn get_piece_at_slot(&self) -> Option<Rc<dyn Widget>>{
        self.get_children().first().map(|w| w.clone())
    }
}
impl Widget for ChessSlot{
    fn widget_data(&self) -> &WidgetData {
        self.rectangle.widget_data()
    }
    fn render(&self) {
        self.rectangle.render()
    }
}
pub static SLOT_SIZE : i32 = 60;
impl ChessSlot{
    pub fn new(position: Vector2<i32>) -> ChessSlot{

        Self{
            position,
            rectangle: RectangleWidget::new(WidgetVector{
                 offset: Vector2::new(
                     (position.x   * SLOT_SIZE) as f32 ,
                (position.y * SLOT_SIZE) as f32
            ),
            side:  Normal} , Vector2::new(SLOT_SIZE as f32, SLOT_SIZE as f32),
            if (position.x + position.y) % 2 == 0{ DARKGRAY} else{ LIGHTGRAY } )
        }
    }
}

