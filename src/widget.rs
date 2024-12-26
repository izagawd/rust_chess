use std::any::{type_name, Any, TypeId};
use std::cell::{Cell, Ref, RefCell};

use std::rc::{Rc, Weak};
use macroquad::input::{mouse_delta_position, mouse_position};
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use nalgebra::Vector2;
use crate::chess::chess_pieces::chess_piece::ChessPiece;
use crate::chess::chess_slot::ChessSlot;
use crate::scene::Scene;

#[derive(Clone,Copy,Eq,PartialEq)]
pub enum Side{
    Normal,
    Center,
    TopCenter,

}
#[derive(Clone,Copy,PartialEq)]
pub struct WidgetVector{
    pub(crate) offset: Vector2<f32>,
    pub(crate) side: Side,
}
impl Default for WidgetVector{
    fn default()->Self{
        Self{offset:Vector2::zeros(), side:Side::Normal}
    }
}
#[derive(Default)]
pub struct WidgetData{
    pub scene: RefCell<Option<Weak<dyn Scene>>>,
    size: RefCell<Vector2<f32>>,
    priority: Cell<i32>,
    widget_position: RefCell<WidgetVector>,
    parent: RefCell<Option<Weak<dyn Widget>>>,
    children: RefCell<Vec<Rc<dyn Widget>>>
}
impl WidgetData{
    fn new() -> Self{
        Self::default()
    }
}




pub trait Widget : Any{

    fn get_scene(&self) -> Rc<dyn Scene>{
        self.widget_data().scene.borrow().as_ref().unwrap().upgrade().unwrap()
    }
    fn as_chess_piece(self: Rc<Self>) -> Rc<dyn ChessPiece>;
    fn is_hovered_on(&self) -> bool{
        let mouse_pos = mouse_position();
        let glob_pos = self.global_position();
        let size = self.size();
        return mouse_pos.0 >= glob_pos.x  && mouse_pos.0 <= glob_pos.x + size.x &&
            mouse_pos.1 >= glob_pos.y && mouse_pos.1 <= glob_pos.y + size.y;
    }
    fn update(self: Rc<Self>){}
    fn set_priority(&self,priority:i32){
        self.widget_data().priority.set(priority);
    }
    fn get_priority(&self) -> i32{
        self.widget_data().priority.get()
    }
    fn render(&self);

    fn local_position(&self) -> WidgetVector;


    fn set_local_position(&self, value: WidgetVector);
    fn set_size(&self, value: Vector2<f32>);
    fn size(&self) -> Vector2<f32>;
    fn global_position(&self) -> Vector2<f32>;
    fn widget_data(&self) -> &WidgetData;
    fn set_parent(self: Rc<Self>, parent: Option<Rc<dyn Widget>>) -> Result<(), &'static str>;
    fn get_children(&self)->Ref<Vec<Rc<dyn Widget>>>;

    fn get_parent(&self)->Option<Rc<dyn Widget>>;
}
default impl<T: 'static> Widget for T {
    fn as_chess_piece(self: Rc<Self>) -> Rc<dyn ChessPiece> {
        panic!()
    }
    fn size(&self) -> Vector2<f32>{
        self.widget_data().size.borrow().clone()
    }
    fn set_local_position(&self, value: WidgetVector) {
        *self.widget_data().widget_position.borrow_mut() = value;
    }
    fn local_position(&self) -> WidgetVector{
        self.widget_data().widget_position.borrow().clone()
    }
    fn set_size(&self, value: Vector2<f32>){
        *self.widget_data().size.borrow_mut() = value;
    }
    fn global_position(&self) -> Vector2<f32>{
        let mut position_to_work_with = Vector2::new(0.0, 0.0);
        let mut size_to_work_with : Vector2<f32>;

        let za_parent = self.get_parent();
        if let Some(ref parent) = za_parent {
            size_to_work_with = parent.size();
            position_to_work_with = parent.global_position()
        } else{
            size_to_work_with = Vector2::new(screen_width(),screen_height());
        }
        let widget_pos = self.widget_data().widget_position.borrow().clone();

        match widget_pos.side {
            Side::Normal => {
                return position_to_work_with + widget_pos.offset;
            }
            Side::Center => {
                let my_size_halved = self.size() / 2.0;
                return Vector2::new((size_to_work_with.x /2.0 - my_size_halved.x) +
                                        widget_pos.offset.x + position_to_work_with.x
                                    ,(size_to_work_with.y / 2.0 - my_size_halved.y) + widget_pos.offset.y +
                position_to_work_with.y);
            }
            Side::TopCenter => {
                let my_size_halved = self.size() / 2.0;
                return Vector2::new((size_to_work_with.x /2.0 - my_size_halved.x) +
                                        widget_pos.offset.x + position_to_work_with.x
                                    ,position_to_work_with.y + widget_pos.offset.y + my_size_halved.y
                + position_to_work_with.y);

            }
        }
    }
    fn get_parent(&self)->Option<Rc<dyn Widget>>{
        let kk =self.widget_data().parent.borrow().clone();
        if let Some(k) = kk {
            return k.upgrade();
        }
        return None;
    }
    fn get_children(&self)->Ref<Vec<Rc<dyn Widget>>>{
        self.widget_data().children.borrow()
    }
    fn set_parent(self: Rc<Self>,parent: Option<Rc<dyn Widget>>) -> Result<(),&'static str> {



        let as_widget : Rc<dyn Widget> =self.clone();
        if let Some(to_become_parent) = parent{

            if Rc::ptr_eq(&as_widget,&to_become_parent) {
                return Err("something went wrong");
            }

            if to_become_parent.widget_data().children.borrow().iter().any(
                |w| Rc::ptr_eq(&as_widget, &w)) {
                return Ok(())
            }
            let mut curr_parent = to_become_parent.widget_data().parent.borrow().clone();
            while let Some(ref parent) = curr_parent {
                if let Some(parent) = parent.upgrade() {
                    if Rc::ptr_eq(&as_widget, &parent) {
                        return Err("something went wrong")
                    }
                    curr_parent = parent.get_parent().and_then(|x| Some(Rc::downgrade(&x)))
                } else{
                    break
                }
            }
            to_become_parent.widget_data().children.borrow_mut().push(self.clone());
            *self.widget_data().parent.borrow_mut() = Some(Rc::downgrade(&to_become_parent));
            Ok(())
        } else{
            let curr_parent = self.widget_data().parent.borrow().clone().and_then(|x| x.upgrade());

            if let Some(true_one) = curr_parent {

                let gotten = true_one.widget_data().children.borrow().iter()
                    .position(|x| Rc::ptr_eq(x,&as_widget));
                if let Some(gotten_index)= gotten {

                    true_one.widget_data().children.borrow_mut().remove(
                        gotten_index
                    );
                }
            }
            *self.widget_data().parent.borrow_mut() = None;
            return Ok(());

        }

    }
}