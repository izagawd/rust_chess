use crate::widget::{Widget, WidgetData, WidgetVector};
use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use nalgebra::Vector2;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;





/// enum for wheter the color is a function that returns a color, or simply a value of Color
pub enum ColorHandler{
    Value(Color),
    Method(Box<dyn Fn() -> Color>),
}


impl From<&ColorHandler> for Color {
    fn from(value: &ColorHandler) -> Self {
        match value {
            ColorHandler::Value(color) => *color,
            ColorHandler::Method(func) => func(),
        }
    }
}
pub struct RectangleWidget{
    widget_data: WidgetData,
    color: RefCell<ColorHandler>,
}

impl RectangleWidget{

    pub fn set_color(& self, color: ColorHandler){
        *self.color.borrow_mut() = color;
    }
    #[inline]
    pub fn get_color(&self) -> Color{
        Color::from(self.color.borrow().deref())
    }
    pub fn new(position: WidgetVector, size: Vector2<f32>, color_handler: ColorHandler) -> Self{
        let gotten =Self{
            widget_data: WidgetData::new(),
            color: RefCell::new(color_handler),
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
        draw_rectangle(global_pos.x, global_pos.y,size.x,size.y, self.get_color());
    }
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
}