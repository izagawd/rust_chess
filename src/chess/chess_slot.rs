use std::any::Any;
use crate::chess::chess_board::ChessBoard;
use crate::chess::chess_pieces::chess_piece::ChessPiece;
use crate::rectangle_widget::{ColorHandler, RectangleWidget};
use crate::scene::{remove_widget, Scene};
use crate::widget::Alignment::{Center, TopLeft};
use crate::widget::{Widget, WidgetData, WidgetVector};
use macroquad::color::{BLUE, DARKGRAY, GREEN, LIGHTGRAY, PURPLE, RED, YELLOW};
use macroquad::input::MouseButton;
use macroquad::prelude::{is_mouse_button_pressed, Color};
use nalgebra::Vector2;
use std::cell::{OnceCell, RefCell};
use std::ops::Deref;
use std::ptr;
use std::rc::{Rc, Weak};
use macroquad::ui::KeyCode::V;
use crate::chess::chess_game;
use crate::chess::chess_pieces::king::King;
use crate::rectangle_widget::ColorHandler::Value;
use crate::winner_scene::WinnerScene;

pub struct ChessSlot{
    original_color: Color,
    pub(crate) board: OnceCell<Weak<ChessBoard>>,
    rectangle: RectangleWidget,
    position: Vector2<i32>,
}


impl ChessSlot{
    pub fn get_chess_board(&self) -> Rc<ChessBoard>{
        self.board.get().unwrap(). upgrade().unwrap()
    }
    pub fn get_slot_position(&self) -> Vector2<i32>{
        self.position
    }
    pub fn get_piece_at_slot(&self) -> Option<Rc<dyn ChessPiece>>{
        self.get_children().first().map(|w| w.clone().as_chess_piece())
    }

    /// sets piece at slot without removing it from the scene
    pub fn set_piece_at_slot_ex<const remove_widget: bool>(self: Rc<Self>, piece: Option<Rc<dyn ChessPiece>>){
        let to_clone =self.get_children().clone();
        for i in to_clone{
            i.clone().set_parent(None).unwrap();
            if remove_widget{
                crate::scene::remove_widget(self.get_scene(), i);
            }
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
    pub fn set_piece_at_slot(self: Rc<Self>, piece: Option<Rc<dyn ChessPiece>>){
        self.set_piece_at_slot_ex::<true>(piece);
    }
}



impl Widget for ChessSlot{
    fn init(self: Rc<Self>) {
        let cloned_weak_self =Rc::downgrade(&self);
        let function_to_use = move ||{
            let rcd_self =cloned_weak_self.upgrade().expect("this shouldnt happen");
            let board = rcd_self.board.get().unwrap().upgrade().expect("this shouldnt happen");



                if chess_game::MoveHelper.get() &&  board.selected_slot.borrow().as_ref().and_then(|x| x.upgrade()).is_some()
                    && board.selected_piece_available_moves_cache.borrow().iter()
                        .any(|x| x.get_slot_position() ==rcd_self.get_slot_position()){
                        return BLUE;
                }



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
                    .any(|x| x.get_slot_position() == other_king.get_slot().unwrap().get_slot_position()){
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
        let board= self.board.get().unwrap().upgrade().unwrap();
        if self.is_hovered_on() && is_mouse_button_pressed(MouseButton::Left) &&
            let Some(piece) = self.get_piece_at_slot()
            && piece.get_chess_color() == board.turn_taker.get(){
            *board.selected_slot.borrow_mut()
            =Some(Rc::downgrade(&self));
            *board.selected_piece_available_moves_cache.borrow_mut() = board.clone().available_moves_for_piece(piece.clone());
        } else if self.is_hovered_on() && is_mouse_button_pressed(MouseButton::Left) {
            let to_unwrap_slot = board.selected_slot.borrow().clone().and_then(|x| x.upgrade());
            if let Some(unwrapped_slot) = to_unwrap_slot
                && let piece = unwrapped_slot.get_piece_at_slot().unwrap() &&
                    board.clone().available_moves_for_piece(piece.clone()).into_iter().any(|p| p.get_slot_position() == self.position){
                    self.clone().set_piece_at_slot(Some(piece));
                    board.turn_taker.set(board.turn_taker.get().get_opposite());
                    *board.selected_slot.borrow_mut() =None;
                let is_other_team_checkmated =  board.clone().king_is_checkmated(board.turn_taker.get());
                if is_other_team_checkmated {
                    self.get_scene().get_game().change_scene(Rc::new(WinnerScene::new(board.turn_taker.get().get_opposite())));
                }
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
            board: OnceCell::new(),
            position,
            rectangle: RectangleWidget::new(WidgetVector{
                 offset: Vector2::new(
                     (position.x   * SLOT_SIZE) as f32 ,
                (position.y * SLOT_SIZE) as f32
            ),
            alignment: TopLeft
            }, Vector2::new(SLOT_SIZE as f32, SLOT_SIZE as f32),
                                            Value(color))
        }
    }
}

