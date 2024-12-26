use std::any::Any;
use std::rc::Rc;
use macroquad::color::{RED, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_texture, draw_texture_ex, DrawTextureParams, Texture2D};
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_slot::ChessSlot;
use crate::widget::Widget;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ChessColor{
    Black,White
}
pub struct ChessPieceData{
    chess_color: ChessColor
}
impl ChessPieceData{
    pub fn new(chess_color: ChessColor) -> Self{
        Self{chess_color}
    }
}
default impl<T: ChessPiece> Widget for T{

    fn as_chess_piece(self: Rc<Self>) -> Rc<dyn ChessPiece> {
        self
    }
}
pub trait ChessPiece : Widget {
    fn get_slot(&self) -> Option<Rc<ChessSlot>>{
        self.get_parent().and_then(|p| match Rc::downcast::<ChessSlot>(p)  {
            Ok(gotten) => { Some(gotten)},
            Err(_) => {None}
        } )
    }
    fn get_chess_color(&self) -> ChessColor{
        self.chess_piece_data().chess_color
    }
    fn chess_piece_data(&self) -> &ChessPieceData;
    fn render_texture(&self,texture: &Texture2D){

        draw_texture_ex(texture, self.global_position().x,
                     self.global_position().y,
                     WHITE,DrawTextureParams{
                dest_size: Some(Vec2::new(self.size().x,self.size().y)),
                ..Default::default()
            })
    }
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