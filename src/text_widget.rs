use crate::widget::{Widget, WidgetData, WidgetVector};
use macroquad::color::Color;
use macroquad::text::{draw_text, measure_text};
use nalgebra::Vector2;


pub struct TextWidget{
    widget_data: WidgetData,
    color: Color,
    text: String,
    font_size: f32,
}

impl TextWidget{
    pub fn new(position: WidgetVector, font_size: f32, color: Color, text: String) -> TextWidget{
        let created =Self{
            widget_data: WidgetData::default(),
            color,
            text,
            font_size
        };
        created.set_local_position(position);
        return created;
    }
}
impl Widget for TextWidget{
    fn render(&self) {

        let global_pos = self.global_position();
        draw_text(self.text.as_str(),global_pos.x,global_pos.y + self.size().y,self.font_size, self.color);

    }

    fn size(&self) -> Vector2<f32>{

        let measured =measure_text(self.text.as_str(),None,self.font_size as u16,1.0);
        Vector2::new(measured.width, measured.height)
    }
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
}