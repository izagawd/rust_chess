use std::cell::RefCell;
use crate::chess::chess_board::ChessBoard;
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::prelude::ImageFormat::Png;
use crate::chess::chess_pieces::chess_piece::{ChessColor, ChessPiece, ChessPieceData};
use crate::widget::{Widget, WidgetData};

pub struct King{
    widget_data: RefCell<WidgetData>,
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
    fn widget_data(&self) -> &RefCell<WidgetData> {
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
            widget_data: RefCell::new(WidgetData::new()),
            chess_piece_data: ChessPieceData::new(chess_color)
        }
    }
}
impl ChessPiece for King {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.chess_piece_data
    }
    fn possible_moves(&self, board: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        Vec::new()
    }
}