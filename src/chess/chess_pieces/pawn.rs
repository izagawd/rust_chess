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

pub struct Pawn{
    widget_data: RefCell<WidgetData>,
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
    fn widget_data(&self) -> &RefCell<WidgetData> {
        &self.widget_data
    }
}

static BLACK_PAWN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/black-pawn.png"),
                                     Some(Png))

});
static WHITE_PAWN_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {


    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/white-pawn.png"),
                                     Some(Png))

});
impl Pawn{
    pub fn new(chess_color: ChessColor) -> Self{
        Self{
            widget_data: RefCell::new(WidgetData::new()),
            chess_piece_data: ChessPieceData::new(chess_color)
        }
    }
}
impl ChessPiece for Pawn {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.chess_piece_data
    }
    fn possible_moves(&self, board: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        let mut positions = Vec::new();
        let my_slot_pos = self.get_slot().unwrap().get_slot_position();

        // used to determine whether the pawn can move forward or backward depending on it's color
        let mut y_incr = -1;
        if self.get_chess_color() == ChessColor::Black{
            y_incr = 1;
        }

        let top_left =Vector2::new(my_slot_pos.x -1 , my_slot_pos.y + y_incr);
        let top_right =Vector2::new(my_slot_pos.x + 1 , my_slot_pos.y + y_incr);
        let position_at_top_left = board.get_slots().iter().filter(|x| x.get_slot_position()
        == top_left).last();
        let position_at_top_right = board.get_slots().iter().filter(|x| x.get_slot_position()
            == top_right).last();

        if let Some(position) = position_at_top_left && position.get_piece_at_slot().is_some() {
            positions.push(top_left);
        }
        if let Some(position) = position_at_top_right && position.get_piece_at_slot().is_some() {
            positions.push(top_right);
        }
        let forward =Vector2::new(my_slot_pos.x , my_slot_pos.y + y_incr);
        let position_forward = board.get_slots().iter().filter(|x| x.get_slot_position()
            == forward).last();
        if let Some(position) = position_forward && !position.get_piece_at_slot().is_some() {
            positions.push(forward);
        } else {
            return positions;
        }
        // returning here to stop pawn from being able to move forward twice if pawn is not
        // in it's starting position
        if my_slot_pos.y != 1 && my_slot_pos.y != 6{
            return positions;
        }
        let double_forward =Vector2::new(my_slot_pos.x , my_slot_pos.y + (y_incr *2));
        let position_double_forward =board.get_slots().iter().filter(|x| x.get_slot_position()
            == double_forward).last();
        if let Some(position) = position_double_forward && !position.get_piece_at_slot().is_some() {
            positions.push(double_forward);
        }
        positions
    }
}