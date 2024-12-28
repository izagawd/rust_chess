use std::cell::RefCell;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_pieces::chess_piece::{recursing_direction, ChessColor, ChessPiece, ChessPieceData};
use crate::widget::{Widget, WidgetData};
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::prelude::ImageFormat::Png;

pub struct Rook{
    data: ChessPieceData,
    widget_data: RefCell<WidgetData>,
}

impl Rook{
    pub fn new(color: ChessColor) -> Rook{
        Self{
            data: ChessPieceData::new(color),
            widget_data: RefCell::new(WidgetData::new())
        }
    }
}

static BLACK_ROOK_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

        Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/black-rook.png"),
                                         Some(Png))

});
static WHITE_ROOK_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/white-rook.png"),
                                     Some(Png))

});
impl Widget for Rook {
    fn widget_data(&self) -> &RefCell<WidgetData> {
        &self.widget_data
    }
    fn render(&self) {
        match self.get_chess_color() {
            ChessColor::Black => {
                self.render_texture(BLACK_ROOK_IMAGE.deref());
            }
            ChessColor::White => {self.render_texture(WHITE_ROOK_IMAGE.deref());}
        }
    }
}



impl ChessPiece for Rook {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.data
    }
    fn possible_moves(&self, chess_board: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        let mut forward = recursing_direction(chess_board,self,Vector2::new(0,1)).unwrap().possible_positions;
        let mut backward = recursing_direction(chess_board,self,Vector2::new(0,-1)).unwrap().possible_positions;
        let mut left = recursing_direction(chess_board,self,Vector2::new(-1,0)).unwrap().possible_positions;
        let mut right = recursing_direction(chess_board,self,Vector2::new(1,0)).unwrap().possible_positions;
        forward.append(&mut backward);
        forward.append(&mut left);
        forward.append(&mut right);
        forward
    }
}