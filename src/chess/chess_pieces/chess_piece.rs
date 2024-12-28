use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_slot::ChessSlot;
use crate::widget::Widget;
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use nalgebra::Vector2;
use std::rc::Rc;

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

pub struct MoveDirectionResult {
    pub collided_piece: Option<Rc<dyn ChessPiece>>,
    pub possible_directions: Vec<Rc<ChessSlot>>
}


/// gets all the locations from a pieces position in a direction, until there's something blocking it
/// (eg rook can go forward until there's a piece in the way)
pub fn recursing_direction(board: &Rc<ChessBoard>,  piece: &(impl ChessPiece + ?Sized), dir: Vector2<i32>) ->Result<MoveDirectionResult,&'static str>{

    if dir == Vector2::new(0,0){
        return Err("direction cannot be (0,0)!!");
    }
    if let Some(gotten_pos) = piece.get_slot(){
        let mut vec_to_use = Vec::new();
        let mut curr_loc = gotten_pos.get_slot_position();
        let mut collided_piece = None;
        while let Some(gotten) = board.get_slots().iter().filter(|x| x.get_slot_position() == curr_loc + dir
         && x.get_piece_at_slot().map(|x| x.get_chess_color() != piece.get_chess_color()).unwrap_or(true)).last(){
            vec_to_use.push(gotten.clone());
            if let Some(collided) = gotten.get_piece_at_slot(){
                collided_piece = Some(collided);
                break;
            }
            curr_loc += dir;

        }
        return Ok(MoveDirectionResult {collided_piece, possible_directions: vec_to_use });
    } else{
        return Ok(MoveDirectionResult {collided_piece: None, possible_directions: Vec::new() });
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
    fn possible_moves(&self, chess_board: &Rc<ChessBoard>) -> Vec<Rc<ChessSlot>>;
}