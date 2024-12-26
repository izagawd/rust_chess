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
impl ChessColor {
    pub fn get_opposite(self) -> Self{
        match self {
            ChessColor::Black => ChessColor::White,
            ChessColor::White => ChessColor::Black
        }
    }
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
    /// Gets the current slot the chess piece is on
    fn get_slot(&self) -> Option<Rc<ChessSlot>>{
        self.get_parent().and_then(|p| match Rc::downcast::<ChessSlot>(p)  {
            Ok(gotten) => { Some(gotten)},
            Err(_) => {None}
        } )
    }
    /// Returns either black or white depending on the chess piece
    fn get_chess_color(&self) -> ChessColor{
        self.chess_piece_data().chess_color
    }
    ///common data every chess piece should have
    fn chess_piece_data(&self) -> &ChessPieceData;


    /// code to easily render a chess piece with less stress
    fn render_texture(&self,texture: &Texture2D){

        draw_texture_ex(texture, self.global_position().x,
                     self.global_position().y,
                     WHITE,DrawTextureParams{
                dest_size: Some(Vec2::new(self.size().x,self.size().y)),
                ..Default::default()
            })
    }


    /// Returns the possible moves this piece can do based on the state of the game
    fn possible_moves(&self,chess_board: &Rc<ChessBoard>) -> Vec<Vector2<i32>>;
}