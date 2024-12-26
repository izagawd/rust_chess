use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_pieces::chess_piece::{recursing_direction, ChessColor, ChessPiece, ChessPieceData};
use crate::widget::WidgetData;

pub struct Queen{
    data: ChessPieceData,
    widget_data: WidgetData,
}

impl Queen{
    pub fn new(color: ChessColor) -> Queen{
        Self{
            data: ChessPieceData::new(color),
            widget_data: WidgetData::default()
        }
    }
}

static BLACK_QUEEN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {
    futures::executor::block_on(


        load_texture("pieces-basic-png/black-queen.png")).unwrap()

});
static WHITE_QUEEN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    futures::executor::block_on(    load_texture("pieces-basic-png/white-queen.png")).unwrap()

});
impl crate::widget::Widget for Queen {
    fn widget_data(&self) -> &WidgetData{
        &self.widget_data
    }
    fn render(&self) {
        match self.get_chess_color() {
            ChessColor::Black => {
                self.render_texture(BLACK_QUEEN_IMAGE.deref());
            }
            ChessColor::White => {self.render_texture(WHITE_QUEEN_IMAGE.deref());}
        }
    }
}



impl ChessPiece for Queen {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.data
    }
    fn possible_moves(&self, chess_board: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        let mut forward = recursing_direction(chess_board,self,Vector2::new(0,1)).unwrap().possible_directions;
        let mut backward = recursing_direction(chess_board,self,Vector2::new(0,-1)).unwrap().possible_directions;
        let mut left = recursing_direction(chess_board,self,Vector2::new(-1,0)).unwrap().possible_directions;
        let mut right = recursing_direction(chess_board,self,Vector2::new(1,0)).unwrap().possible_directions;
        let mut right_up = recursing_direction(chess_board, self, Vector2::new(1, 1)).unwrap().possible_directions;
        let mut left_down = recursing_direction(chess_board, self, Vector2::new(-1, -1)).unwrap().possible_directions;
        let mut left_up = recursing_direction(chess_board, self, Vector2::new(-1, 1)).unwrap().possible_directions;
        let mut right_down = recursing_direction(chess_board, self, Vector2::new(1, -1)).unwrap().possible_directions;


        forward.append(&mut backward);
        forward.append(&mut left);
        forward.append(&mut right);
        forward.append(&mut right_up);
        forward.append(&mut left_down);
        forward.append(&mut left_up);
        forward.append(&mut right_down);
        forward
    }
}