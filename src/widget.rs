use std::any::Any;
use std::cell::{Ref, RefCell};

use crate::chess::chess_pieces::chess_piece::ChessPiece;
use crate::scene::Scene;
use macroquad::input::mouse_position;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use nalgebra::Vector2;
use std::rc::{Rc, Weak};


#[derive(Clone,Copy,Eq,PartialEq)]
pub enum Alignment {
    Normal,
    Center,
    TopCenter,

}
#[derive(Clone,Copy,PartialEq)]
pub struct WidgetVector{
    pub(crate) offset: Vector2<f32>,
    pub(crate) alignment: Alignment,
}
impl Default for WidgetVector{
    fn default()->Self{
        Self{offset:Vector2::zeros(), alignment: Alignment::Normal}
    }
}
#[derive(Default)]
pub struct WidgetData{
    pub scene: RefCell<Option<Weak<dyn Scene>>>,
    size: RefCell<Vector2<f32>>,

    widget_position: RefCell<WidgetVector>,
    parent: RefCell<Option<Weak<dyn Widget>>>,
    children: RefCell<Vec<Rc<dyn Widget>>>
}
impl WidgetData{
    fn new() -> Self{
        Self::default()
    }
}



/// A widget is any object that is in a scene that can be rendered
pub trait Widget : Any{

    /// Gets the scene the widget resides in
    fn get_scene(&self) -> Rc<dyn Scene>{
        self.widget_data().scene.borrow().as_ref().unwrap().upgrade().unwrap()
    }
    /// Gets the widget as a chess piece. If it is not a chess piece an error is thrown
    fn as_chess_piece(self: Rc<Self>) -> Rc<dyn ChessPiece>;

    /// Returns whether or not the mouse is hovered on the widget
    fn is_hovered_on(&self) -> bool{
        let mouse_pos = mouse_position();
        let glob_pos = self.global_position();
        let size = self.size();
        return mouse_pos.0 >= glob_pos.x  && mouse_pos.0 <= glob_pos.x + size.x &&
            mouse_pos.1 >= glob_pos.y && mouse_pos.1 <= glob_pos.y + size.y;
    }
    /// Runs just before update
    fn update(self: Rc<Self>){}
    /// widgets with a higher priority are rendered on top of those with lower


    /// Implementation on how the widget is rendered. Will be called every frame
    fn render(&self);

    /// Gets The position of the widget, based on its parent
    fn local_position(&self) -> WidgetVector;

    /// Sets The position of the widget, based on its parent
    fn set_local_position(&self, value: WidgetVector);
    fn set_size(&self, value: Vector2<f32>);
    fn size(&self) -> Vector2<f32>;


    /// Gets the position of the widget based on the screen
    fn global_position(&self) -> Vector2<f32>;
    /// Data all widgets must have
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
        let size_to_work_with : Vector2<f32>;

        let za_parent = self.get_parent();
        if let Some(ref parent) = za_parent {
            size_to_work_with = parent.size();
            position_to_work_with = parent.global_position()
        } else{
            size_to_work_with = Vector2::new(screen_width(),screen_height());
        }
        let widget_pos = self.widget_data().widget_position.borrow().clone();

        match widget_pos.alignment {
            Alignment::Normal => {
                return position_to_work_with + widget_pos.offset;
            }
            Alignment::Center => {
                let my_size_halved = self.size() / 2.0;
                return Vector2::new((size_to_work_with.x /2.0 - my_size_halved.x) +
                                        widget_pos.offset.x + position_to_work_with.x
                                    ,(size_to_work_with.y / 2.0 - my_size_halved.y) + widget_pos.offset.y +
                position_to_work_with.y);
            }
            Alignment::TopCenter => {
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
            let prev_parent = self.widget_data().parent.borrow().clone().and_then(|x| x.upgrade());
            if let Some(prev_parent) = prev_parent {
                let index = prev_parent.widget_data().children.borrow().iter().position(|x|
                Rc::ptr_eq(&as_widget,x));
                if let Some(index) = index {
                    prev_parent.widget_data().children.borrow_mut().remove(index);
                }

            }

            *self.widget_data().parent.borrow_mut() = Some(Rc::downgrade(&to_become_parent));
            Ok(())
        } else{
            let curr_parent = self.widget_data().parent.borrow().clone().and_then(|x| x.upgrade());

            if let Some(true_one) = curr_parent {

                let gotten = true_one.widget_data().children.borrow().iter()
                    .position(|x| Rc::ptr_eq(x,&as_widget));
                if let Some(gotten_index)= gotten {

                    println!("{}","yo");
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