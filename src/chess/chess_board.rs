use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::thread::yield_now;
use nalgebra::Vector2;
use crate::chess::chess_pieces::bishop::Bishop;
use crate::chess::chess_pieces::chess_piece::{ChessColor, ChessPiece};
use crate::chess::chess_pieces::chess_piece::ChessColor::{Black, White};
use crate::chess::chess_pieces::king::King;
use crate::chess::chess_pieces::knight::Knight;
use crate::chess::chess_pieces::pawn::Pawn;
use crate::chess::chess_pieces::queen::Queen;
use crate::chess::chess_pieces::rook::Rook;
use crate::chess::chess_slot;
use crate::chess::chess_slot::ChessSlot;
use crate::scene::{add_widget, Scene};
use crate::widget::{Widget, WidgetData};

pub struct ChessBoard{
    pub turn_taker: Cell<ChessColor>,
    pub selected_slot: RefCell<Option<Weak<ChessSlot>>>,
    chess_slots: Vec<Rc<ChessSlot>>,
    widget_data: WidgetData
}
impl Widget for ChessBoard {
    fn size(&self) -> Vector2<f32> {
        let size = self.chess_slots.len().isqrt() as f32 * chess_slot::SLOT_SIZE as f32;
        Vector2::new(size, size)
    }
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
    fn render(&self){

    }
}

impl ChessBoard{

    pub gen fn king_is_checked(self: Rc<Self>,king: Rc<King>) -> Rc<dyn ChessPiece>{
        let king_loc = king.get_slot().unwrap().get_slot_position();
        let king_col = king.get_chess_color();
        for i in self.chess_slots.clone().into_iter().map(|x| x.get_piece_at_slot())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .filter(move |x| x.get_chess_color() != king_col){
            if i.possible_moves(&self).iter().any(|x| *x == king_loc){
                yield i
            }
        }
    }
    pub fn get_slots(&self) -> &Vec<Rc<ChessSlot>>{
        &self.chess_slots
    }
    pub fn get_selected_slot(&self) -> Option<Rc<ChessSlot>> {
        self.selected_slot.borrow().clone().and_then(|s| s.upgrade())
    }
    pub fn new(current_scene: Rc<dyn Scene>) -> Rc<ChessBoard>{
       let mut created = ChessBoard{turn_taker: Cell::new(White), selected_slot: RefCell::new(None), chess_slots: Vec::new(), widget_data: WidgetData::new()};

        let mut vecs_of_chess : Vec<Rc<ChessSlot>> =  Vec::new();
        for i in 0..8{
            for j in 0..8{
                vecs_of_chess.push(add_widget(current_scene.clone(),ChessSlot::new(Vector2::new(i,j))))
            }
        }

        created.chess_slots = vecs_of_chess;
        let created = add_widget(current_scene.clone()  ,created);
        for i in created.chess_slots.iter(){
            *i.board.borrow_mut() = Rc::downgrade(&created);
        }
        for i in created.chess_slots.iter(){
            i.clone().set_parent(Some(created.clone())).unwrap();

        }

        //adding black pawns..
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 1){
            let created_pawn =      add_widget(current_scene.clone(),Pawn::new(Black));
           i.clone().set_piece_at_slot(Some(created_pawn));
        }

        //adding white pawns..
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 6){
            let created_pawn =      add_widget(current_scene.clone(),Pawn::new(White));
            i.clone().set_piece_at_slot(Some(created_pawn));
        }

        // adding black rooks
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 0 &&
            (x.get_slot_position().x == 0 || x.get_slot_position().x == 7)){
            let created_rook = add_widget(current_scene.clone(),Rook::new(Black));
                i.clone().set_piece_at_slot(Some(created_rook));

        }

        // adding white rooks
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 7 &&
            (x.get_slot_position().x == 0 || x.get_slot_position().x == 7)){
            let created_rook = add_widget(current_scene.clone(),Rook::new(White));
            i.clone().set_piece_at_slot(Some(created_rook));

        }

        // adding black bishops
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 0 &&
            (x.get_slot_position().x == 1 || x.get_slot_position().x == 6)){
            let created_bishop = add_widget(current_scene.clone(),Bishop::new(Black));
            i.clone().set_piece_at_slot(Some(created_bishop));

        }

        // adding white bishops
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 7 &&
            (x.get_slot_position().x == 6 || x.get_slot_position().x == 1)){
            let created_bishop = add_widget(current_scene.clone(),Bishop::new(White));
            i.clone().set_piece_at_slot(Some(created_bishop));

        }
        // adding black knights
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 0 &&
            (x.get_slot_position().x == 5 || x.get_slot_position().x == 2)){
            let created_bishop = add_widget(current_scene.clone(),Knight::new(Black));
            i.clone().set_piece_at_slot(Some(created_bishop));

        }

        // adding white knights
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 7 &&
            (x.get_slot_position().x == 5 || x.get_slot_position().x == 2)){
            let created_bishop = add_widget(current_scene.clone(),Knight::new(White));
            i.clone().set_piece_at_slot(Some(created_bishop));

        }

        // adding black queen
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 0
            && x.get_slot_position().x == 3){
            let created_queen = add_widget(current_scene.clone(),Queen::new(Black));
            i.clone().set_piece_at_slot(Some(created_queen));
        }

        // adding white queen
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 7
            && x.get_slot_position().x == 3){
            let created_queen = add_widget(current_scene.clone(),Queen::new(White));
            i.clone().set_piece_at_slot(Some(created_queen));
        }

        // adding black king
        for i in created.chess_slots
            .iter()
            .filter(|x| x.get_slot_position().y == 0
            && x.get_slot_position().x == 4){
            let created_king = add_widget(current_scene.clone(), King::new(Black));
            i.clone().set_piece_at_slot(Some(created_king));
        }

        // adding white king
        for i in created.chess_slots
            .iter()
            .filter(|x| x.get_slot_position().y == 7
            && x.get_slot_position().x == 4){
            let created_king = add_widget(current_scene.clone(), King::new(White));
            i.clone().set_piece_at_slot(Some(created_king));
        }
        return created;
    }
}