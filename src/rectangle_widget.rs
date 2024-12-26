use crate::widget::{Widget, WidgetData, WidgetVector};
use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use nalgebra::Vector2;
use std::cell::Cell;
use std::rc::Rc;


pub struct RectangleWidget{
    widget_data: WidgetData,
    pub color: Cell<Color>
}

impl RectangleWidget{

    pub fn new(position: WidgetVector, size: Vector2<f32>, color: Color) -> Self{
        let gotten =Self{
            widget_data: WidgetData::default(),
            color: Cell::new(color)
        };
        gotten.set_size(size);
        gotten.set_local_position(position);
        gotten
    }
}

impl Widget for RectangleWidget {

    fn update(self: Rc<Self>){

    }
    fn render(&self) {
        let global_pos = self.global_position();
        let size = self.size();
        draw_rectangle(global_pos.x, global_pos.y,size.x,size.y, self.color.get());
    }
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
}