use macroquad::color::RED;
use macroquad::prelude::{draw_texture, load_image, load_texture, Image, Texture2D};
use crate::widget::{Widget, WidgetData};

pub struct ImageWidget {
    widget_data: WidgetData,
    texture: &'static Texture2D,
}
impl ImageWidget {
    pub fn new(texture2D: &'static Texture2D) -> ImageWidget {

        Self{
            widget_data: WidgetData::default(),
            texture: texture2D,
        }
    }
}
impl Widget for ImageWidget {

    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
    fn render(&self) {
        draw_texture(self.texture, self.global_position().x, self.global_position().y,
        RED)
    }
}