use std::cell::RefCell;
use std::ops::Deref;
use std::ptr;
use std::rc::{Rc, Weak};
use macroquad::color::{BLACK, BLUE, DARKGRAY, GRAY, GREEN, LIGHTGRAY, RED, WHITE};
use macroquad::input::MouseButton;
use macroquad::prelude::{is_mouse_button_pressed, Color};
use nalgebra::Vector2;
use crate::chess::chess_board::ChessBoard;
use crate::rectangle_widget::RectangleWidget;
use crate::widget::{Widget, WidgetData, WidgetVector};
use crate::widget::Side::{Center, Normal};

pub struct ChessSlot{
    original_color: Color,
    pub(crate) board: RefCell<Weak<ChessBoard>>,
    rectangle: RectangleWidget,
    position: Vector2<i32>,
}


impl ChessSlot{
    pub fn get_slot_position(&self) -> Vector2<i32>{
        self.position
    }
    pub fn get_piece_at_slot(&self) -> Option<Rc<dyn Widget>>{
        self.get_children().first().map(|w| w.clone())
    }

    pub fn set_piece_at_slot(self: Rc<Self>, piece: Option<Rc<dyn Widget>>){
        for i in self.get_children().clone(){
            i.set_parent(None);
        }
        piece.map(|x| {


            x.clone().set_parent(Some(self.clone())).expect("hmm");

            x.set_size(self.size());
            x.set_local_position(WidgetVector{
                side: Center,
                ..Default::default()
            })
        });


    }
}

impl ChessSlot{
    fn handle_color(&self) {
        if let Some(gotten)  =self.board.borrow().upgrade(){
            if let Some(gotten_slot) = gotten.get_selected_slot(){
                if ptr::eq(self,gotten_slot.deref()){
                    self.rectangle.color.set(GREEN);
                    return;
                }
            }
        }
        self.rectangle.color.set(self.original_color);
    }
}

impl Widget for ChessSlot{
    fn widget_data(&self) -> &WidgetData {
        self.rectangle.widget_data()
    }

    fn update(self: Rc<Self>) {
        self.handle_color();
        if self.is_hovered_on() && is_mouse_button_pressed(MouseButton::Left){
            *self.board.borrow().upgrade().unwrap().selected_slot.borrow_mut()
            =Some(Rc::downgrade(&self))
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
            side:  Normal} , Vector2::new(SLOT_SIZE as f32, SLOT_SIZE as f32),
            color)
        }
    }
}

