use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::experimental::scene::Handle;
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_piece_texture::ChessPieceTexture;
use crate::chess::chess_pieces::chess_piece::ChessPiece;
use crate::widget::{Widget, WidgetData};

pub struct Pawn{
    chess_piece_texture: ChessPieceTexture,
}


impl Widget for Pawn {
    fn render(&self) {
        self.chess_piece_texture.render();
    }

    fn widget_data(&self) -> &WidgetData {
        self.chess_piece_texture.widget_data()
    }
}

static BLACK_PAWN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    futures::executor::block_on(    load_texture("pieces-basic-png/black-pawn.png")).unwrap()

});

impl ChessPiece for Pawn {
    fn possible_moves(&self, _: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        todo!()
    }
}