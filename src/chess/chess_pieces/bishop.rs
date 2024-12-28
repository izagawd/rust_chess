use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::prelude::{load_texture, Texture2D};
use macroquad::prelude::ImageFormat::Png;
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_pieces::chess_piece::{recursing_direction, ChessColor, ChessPiece, ChessPieceData};
use crate::chess::chess_slot::ChessSlot;
use crate::widget::WidgetData;

pub struct Bishop{
    data: ChessPieceData,
    widget_data: WidgetData,
}

impl Bishop{
    pub fn new(color: ChessColor) -> Bishop{
        Self{
            data: ChessPieceData::new(color),
            widget_data: WidgetData::new()
        }
    }
}

static BLACK_BISHOP_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/black-bishop.png"),
                                     Some(Png))

});
static WHITE_BISHOP_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {


    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/white-bishop.png"),
                                     Some(Png))

});
impl crate::widget::Widget for Bishop {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
    fn render(&self) {
        match self.get_chess_color() {
            ChessColor::Black => {
                self.render_texture(BLACK_BISHOP_IMAGE.deref());
            }
            ChessColor::White => {self.render_texture(WHITE_BISHOP_IMAGE.deref());}
        }
    }
}



impl ChessPiece for Bishop {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.data
    }
    fn possible_moves(&self, chess_board: &Rc<ChessBoard>) -> Vec<Rc<ChessSlot>> {
        let mut right_up = recursing_direction(chess_board, self, Vector2::new(1, 1)).unwrap().possible_directions;
        let mut left_down = recursing_direction(chess_board, self, Vector2::new(-1, -1)).unwrap().possible_directions;
        let mut left_up = recursing_direction(chess_board, self, Vector2::new(-1, 1)).unwrap().possible_directions;
        let mut right_down = recursing_direction(chess_board, self, Vector2::new(1, -1)).unwrap().possible_directions;
        right_up.append(&mut left_down);
        right_up.append(&mut left_up);
        right_up.append(&mut right_down);
        right_up
    }
}