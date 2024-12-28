use std::cell::RefCell;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_pieces::chess_piece::{ChessColor, ChessPiece, ChessPieceData};
use crate::widget::WidgetData;
use macroquad::prelude::{load_texture, Texture2D};
use nalgebra::Vector2;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;
use macroquad::prelude::ImageFormat::Png;

pub struct Knight{
    data: ChessPieceData,
    widget_data: WidgetData,
}

impl Knight{
    pub fn new(color: ChessColor) -> Knight{
        Self{
            data: ChessPieceData::new(color),
            widget_data: WidgetData::new()
        }
    }
}

static BLACK_KNIGHT_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {


    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/black-knight.png"),
                                     Some(Png))

});
static WHITE_KNIGHT_IMAGE: LazyLock<Texture2D> = LazyLock::new(|| {

    Texture2D::from_file_with_format(include_bytes!("../../../pieces-basic-png/white-knight.png"),
                                     Some(Png))
});
impl crate::widget::Widget for Knight {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
    fn render(&self) {
        match self.get_chess_color() {
            ChessColor::Black => {
                self.render_texture(BLACK_KNIGHT_IMAGE.deref());
            }
            ChessColor::White => {self.render_texture(WHITE_KNIGHT_IMAGE.deref());}
        }
    }
}



impl ChessPiece for Knight {
    fn chess_piece_data(&self) -> &ChessPieceData {
        &self.data
    }
    fn possible_moves(&self, chess_board: &Rc<ChessBoard>) -> Vec<Vector2<i32>> {
        if let Some(my_slot) = self.get_slot(){
            let my_slot_pos = my_slot.get_slot_position();
            let collected = chess_board.get_slots().iter()
                .map(|i| i.get_slot_position())
                .filter(|slot_pos|{

                let mut za_bool = false;
                for x in [1,-1]{
                    for y in [2,-2]{
                        za_bool = za_bool || Vector2::new(slot_pos.x, slot_pos.y) ==
                            Vector2::new(my_slot_pos.x + x, my_slot_pos.y + y)
                    }
                }
                if za_bool{
                    return true;
                }
                for x in [2,-2]{
                    for y in [1,-1]{
                        za_bool = za_bool || Vector2::new(slot_pos.x, slot_pos.y) ==
                            Vector2::new(my_slot_pos.x + x, my_slot_pos.y + y)
                    }
                }
                return za_bool;
            })
                .collect::<Vec<Vector2<i32>>>();
            collected
        }else{
            return vec![];
        }

    }
}