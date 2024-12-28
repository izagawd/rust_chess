use std::cell::RefCell;
use crate::chess::chess_board::ChessBoard;
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::prelude::ImageFormat::Png;
use crate::chess::chess_pieces::chess_piece::{ChessColor, ChessPiece, ChessPieceData};
use crate::chess::chess_slot::ChessSlot;
use crate::widget::{Widget, WidgetData};

pub struct King{
    widget_data: WidgetData,
    chess_piece_data: ChessPieceData
}


impl Widget for King {
    fn render(&self) {
        if self.get_chess_color() == ChessColor::Black{
            self.render_texture(BLACK_King_IMAGE.deref())
        } else{
            self.render_texture(WHITE_King_IMAGE.deref())
        }
    }
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
}

static BLACK_King_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/black-king.png"),
                                     Some(Png))

});
static WHITE_King_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {


    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/white-king.png"),
                                     Some(Png))

});
impl King{
    pub fn new(chess_color: ChessColor) -> Self{
        Self{
            widget_data: WidgetData::new(),
            chess_piece_data: ChessPieceData::new(chess_color)
        }
    }
}
impl ChessPiece for King {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.chess_piece_data
    }
    fn possible_moves(&self, chess_board: &Rc<ChessBoard>) -> Vec<Rc<ChessSlot>> {
        let my_slot_pos = self.get_slot().unwrap().get_slot_position();
        let mut move_slots = Vec::new();
        for x in [0,1,-1]{
            for y in [0,1,-1]{
                let gotten_slot =chess_board.get_slots()
                    .iter()

                    .find(|i| i.get_slot_position() == Vector2::new(my_slot_pos.x + x ,my_slot_pos.y + y)

                    && i.get_piece_at_slot().map(|x| x.get_chess_color() != self.get_chess_color()).unwrap_or(true));
                if let Some(gotten_slot) = gotten_slot {
                    move_slots.push(gotten_slot.clone());
                }
            }
        }
        move_slots
    }
}