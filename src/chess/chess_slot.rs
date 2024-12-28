use std::any::Any;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_pieces::chess_piece::ChessPiece;
use crate::rectangle_widget::{ColorHandler, RectangleWidget};
use crate::scene::remove_widget;
use crate::widget::Alignment::{Center, Normal};
use crate::widget::{Widget, WidgetData, WidgetVector};
use macroquad::color::{DARKGRAY, GREEN, LIGHTGRAY, RED, YELLOW};
use macroquad::input::MouseButton;
use macroquad::prelude::{is_mouse_button_pressed, Color};
use nalgebra::Vector2;
use std::cell::RefCell;
use std::ops::Deref;
use std::ptr;
use std::rc::{Rc, Weak};
use macroquad::ui::KeyCode::V;
use crate::chess::chess_pieces::king::King;
use crate::rectangle_widget::ColorHandler::Value;

pub struct ChessSlot{
    original_color: Color,
    pub(crate) board: RefCell<Weak<ChessBoard>>,
    rectangle: RectangleWidget,
    position: Vector2<i32>,
}


impl ChessSlot{
    pub fn get_chess_board(&self) -> Rc<ChessBoard>{
        self.board.borrow().upgrade().unwrap()
    }
    pub fn get_slot_position(&self) -> Vector2<i32>{
        self.position
    }
    pub fn get_piece_at_slot(&self) -> Option<Rc<dyn ChessPiece>>{
        self.get_children().first().map(|w| w.clone().as_chess_piece())
    }

    pub fn set_piece_at_slot(self: Rc<Self>, piece: Option<Rc<dyn ChessPiece>>){
        let to_clone =self.get_children().clone();
        for i in to_clone{
            i.clone().set_parent(None).unwrap();
            remove_widget(self.get_scene(),i);
        }
        piece.map(|x| {


            x.clone().set_parent(Some(self.clone())).unwrap();

            x.set_size(self.size());
            x.set_local_position(WidgetVector{
                alignment: Center,
                ..Default::default()
            })
        });


    }
}



impl Widget for ChessSlot{
    fn init(self: Rc<Self>) {
        let cloned_weak_self =Rc::downgrade(&self);
        let function_to_use = move ||{
            let rcd_self =cloned_weak_self.upgrade().expect("this shouldnt happen");
            let board = rcd_self.board.borrow().upgrade().expect("this shouldnt happen");
            if let Some(gotten_slot) = board.get_selected_slot(){

                if ptr::eq(rcd_self.deref(),gotten_slot.deref()){
                    return GREEN
                }
            }
            if let Some(gotten_piece) = rcd_self.get_piece_at_slot(){
                if let Ok(gotten_piece) = Rc::downcast::<King>(gotten_piece.clone()){
                    let mut iteratorss =  board.clone().king_is_checked(gotten_piece);
                    if iteratorss.any(|_|true){
                        return RED;
                    }
                }
                let other_king = board.clone().get_slots().iter()
                    .map(|x| x.get_piece_at_slot())
                    .filter(|x| {
                        if let Some(gotten) = x.as_ref()
                            && gotten.get_chess_color() != gotten_piece.get_chess_color()
                        &&
                        (gotten.clone() as Rc<dyn Any>).is::<King>(){
                            return true;
                        }
                        return false;
                    })
                    .last().flatten().unwrap();
                if gotten_piece.possible_moves(&board).iter()
                    .any(|x| *x == other_king.get_slot().unwrap().get_slot_position()){
                    return YELLOW;
                }
            }
            return rcd_self.original_color;


        };
        self.rectangle.set_color(ColorHandler::Method(Box::new(function_to_use)));
    }
    fn widget_data(&self) -> &WidgetData {
        self.rectangle.widget_data()
    }

    fn update(self: Rc<Self>) {
        let board= self.board.borrow().upgrade().unwrap();
        if self.is_hovered_on() && is_mouse_button_pressed(MouseButton::Left) &&
            let Some(piece) = self.get_piece_at_slot()
            && piece.get_chess_color() == self.board.borrow().upgrade().unwrap().turn_taker.get(){
            *self.board.borrow().upgrade().unwrap().selected_slot.borrow_mut()
            =Some(Rc::downgrade(&self))
        } else if self.is_hovered_on() && is_mouse_button_pressed(MouseButton::Left) {
            let to_unwrap_slot = board.selected_slot.borrow().clone().and_then(|x| x.upgrade());
            if let Some(unwrapped_slot) = to_unwrap_slot
                && let piece = unwrapped_slot.get_piece_at_slot().unwrap() &&
                    piece.possible_moves(&board).iter().any(|p| *p == self.position){
                    self.set_piece_at_slot(Some(piece));
                    board.turn_taker.set(board.turn_taker.get().get_opposite());
                    *board.selected_slot.borrow_mut() =None;
            }
        }


    }

    fn render(&self) {
        self.rectangle.render()
    }
}
pub static SLOT_SIZE : i32 = 60;
impl ChessSlot{
    pub fn new(position: Vector2<i32>) -> ChessSlot{

        let color =if (position.x + position.y) % 2 == 0{ DARKGRAY} else{ LIGHTGRAY };
        Self{
            original_color: color,
            board: RefCell::new(Weak::new()),
            position,
            rectangle: RectangleWidget::new(WidgetVector{
                 offset: Vector2::new(
                     (position.x   * SLOT_SIZE) as f32 ,
                (position.y * SLOT_SIZE) as f32
            ),
            alignment:  Normal}, Vector2::new(SLOT_SIZE as f32, SLOT_SIZE as f32),
                                            Value(color))
        }
    }
}

