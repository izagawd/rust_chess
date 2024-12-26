use std::cell::RefCell;
use std::rc::{Rc, Weak};
use nalgebra::Vector2;
use crate::chess::chess_pieces::chess_piece::ChessColor;
use crate::chess::chess_pieces::chess_piece::ChessColor::{Black, White};
use crate::chess::chess_pieces::pawn::Pawn;
use crate::chess::chess_slot;
use crate::chess::chess_slot::ChessSlot;
use crate::scene::{add_widget, Scene};
use crate::widget::{Widget, WidgetData};

pub struct ChessBoard{
    pub turn_taker: ChessColor,
    pub selected_slot: RefCell<Option<Weak<ChessSlot>>>,
    chess_slots: Vec<Rc<ChessSlot>>,
    widget_data: WidgetData
}
impl Widget for ChessBoard {


    fn size(&self) -> Vector2<f32> {
        let size = self.chess_slots.len().isqrt() as f32 * chess_slot::SLOT_SIZE as f32;
        Vector2::new(size, size)
    }
    fn widget_data(&self) -> &WidgetData{
        &self.widget_data
    }

    fn render(&self){

    }
}

impl ChessBoard{
    pub fn get_slots(&self) -> &Vec<Rc<ChessSlot>>{
        &self.chess_slots
    }
    pub fn get_selected_slot(&self) -> Option<Rc<ChessSlot>> {
        self.selected_slot.borrow().clone().and_then(|s| s.upgrade())
    }
    pub fn new(current_scene: Rc<dyn Scene>) -> Rc<ChessBoard>{
       let mut created = ChessBoard{turn_taker: White, selected_slot: RefCell::new(None), chess_slots: Vec::new(), widget_data: WidgetData::default()};

        let mut vecs_of_chess : Vec<Rc<ChessSlot>> =  Vec::new();
        for i in 0..8{
            for j in 0..8{
                vecs_of_chess.push(add_widget(current_scene.clone(),ChessSlot::new(Vector2::new(i,j))))
            }
        }
        let cloned_vecs_of_chess = vecs_of_chess.clone();
        created.chess_slots = vecs_of_chess;
        let created = add_widget(current_scene.clone()  ,created);
        for i in created.chess_slots.iter(){
            *i.board.borrow_mut() = Rc::downgrade(&created);
        }
        for i in cloned_vecs_of_chess.iter(){
            i.clone().set_parent(Some(created.clone())).expect("ugh");

        }

        //adding black pawns..
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 0){
            let created_pawn =      add_widget(current_scene.clone(),Pawn::new(Black));
           i.clone().set_piece_at_slot(Some(created_pawn));
        }

        //adding white pawns..
        for i in created.chess_slots.iter().filter(|x| x.get_slot_position().y == 7){
            let created_pawn =      add_widget(current_scene.clone(),Pawn::new(White));
            i.clone().set_piece_at_slot(Some(created_pawn));
        }




        return created;
    }
}