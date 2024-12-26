use macroquad::prelude::Texture2D;
use nalgebra::Vector2;
use crate::chess::chess_slot;
use crate::chess::chess_slot::ChessSlot;
use crate::ImageWidget::ImageWidget;
use crate::scene::Scene;
use crate::widget::{Widget, WidgetData};

pub struct ChessPieceTexture{
    image_widget: ImageWidget
}

impl Widget for ChessPieceTexture{
    fn render(&self) {
        self.image_widget.render()
    }
    fn widget_data(&self) -> &WidgetData {
        self.image_widget.widget_data()
    }

}
impl ChessPieceTexture{
    pub fn new (texture2d: &'static Texture2D) -> ChessPieceTexture{
        let mut image_Widget = ImageWidget::new(texture2d);
        image_Widget.set_size(Vector2::new(chess_slot::SLOT_SIZE as f32, chess_slot::SLOT_SIZE as f32));
        Self{
            image_widget: image_Widget
        }

    }
}