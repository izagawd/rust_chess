use std::any::Any;
use std::rc::Rc;
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_slot::ChessSlot;
use crate::widget::Widget;


pub trait ChessPiece : Widget {
    fn get_current_chess_slot(&self) -> Option<Rc<ChessSlot>>{

        match self.get_parent(){
            Some(p) => match Rc::downcast::<ChessSlot>(p){
                Ok(slot) => return Some(slot.clone()),
                Err(_) => return None
            }
            None => return None
        }

    }
    fn possible_moves(&self,chess_board: &Rc<ChessBoard>) -> Vec<Vector2<i32>>;
}