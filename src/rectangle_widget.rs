use macroquad::color::Color;
use macroquad::miniquad::KeyCode::W;
use macroquad::shapes::draw_rectangle;
use nalgebra::Vector2;
use crate::widget::{Widget, WidgetData, WidgetVector};

pub struct RectangleWidget{
    widget_data: WidgetData,
    pub color: Color
}

impl RectangleWidget{

    pub fn new(position: WidgetVector, size: Vector2<f32>, color: Color) -> Self{
        let gotten =Self{
            widget_data: WidgetData::default(),
            color
        };
        gotten.set_size(size);
        gotten.set_local_position(position);
        gotten
    }
}

impl Widget for RectangleWidget {
    fn update(&self){

    }
    fn render(&self) {
        let global_pos = self.global_position();
        let size = self.size();
        draw_rectangle(global_pos.x, global_pos.y,size.x,size.y, self.color);
    }
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
}