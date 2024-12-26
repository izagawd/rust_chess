use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::color::BLACK;
use macroquad::experimental::scene::Handle;
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;

use crate::chess::chess_pieces::chess_piece::{ChessColor, ChessPiece, ChessPieceData};
use crate::widget::{Widget, WidgetData};

pub struct Pawn{
    widget_data: WidgetData,
    chess_piece_data: ChessPieceData
}


impl Widget for Pawn {
    fn render(&self) {
        if self.get_chess_color() == ChessColor::Black{
            self.render_texture(BLACK_PAWN_IMAGE.deref())
        } else{
            self.render_texture(WHITE_PAWN_IMAGE.deref())
        }

    }

    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
}

static BLACK_PAWN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {
    futures::executor::block_on(


        load_texture("pieces-basic-png/black-pawn.png")).expect("Failed to load texture")

});
static WHITE_PAWN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    futures::executor::block_on(    load_texture("pieces-basic-png/white-pawn.png")).expect("Failed to load texture")

});
impl Pawn{
    pub fn new(chess_color: ChessColor) -> Self{
        Self{
            widget_data: WidgetData::default(),
            chess_piece_data: ChessPieceData::new(chess_color)
        }
    }
}
impl ChessPiece for Pawn {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.chess_piece_data
    }
    fn possible_moves(&self, _: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        Vec::new()
    }
}