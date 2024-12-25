use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use nalgebra::Vector2;


#[derive(Clone,Copy,Eq,PartialEq)]
pub enum Side{
    Normal,
    Center,

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


pub trait Widget {
    fn render(&self);

    fn local_position(&self) -> WidgetVector;

    fn set_local_position(&self, value: WidgetVector);
    fn set_size(&self, value: Vector2<f32>);
    fn size(&self) -> Vector2<f32>;
    fn global_position(&self) -> Vector2<f32>;
    fn widget_data(&self) -> &WidgetData;
    fn add_child(self: Rc<Self>,to_add: Rc<dyn Widget>) -> Result<(), &'static str>;
    fn get_children(&self)->Ref<Vec<Rc<dyn Widget>>>;

    fn get_parent(&self)->Option<Rc<dyn Widget>>;
}
default impl<T: 'static> Widget for T {
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
                return Vector2::new(size_to_work_with.x /2.0 - my_size_halved.x
                                    ,size_to_work_with.y / 2.0 - my_size_halved.y);
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
    fn add_child(self: Rc<Self>,to_add: Rc<dyn Widget>) -> Result<(),&'static str> {

        let as_widget : Rc<dyn Widget> = self.clone();
        if Rc::ptr_eq(&as_widget,&to_add) {
            return Err("something went wrong");
        }
        if self.widget_data().children.borrow().iter().any(
            |w| Rc::ptr_eq(&to_add, &w)) {
            return Ok(())
        }
        let mut curr_parent = self.widget_data().parent.borrow().clone();
        while let Some(ref parent) = curr_parent {
            if let Some(parent) = parent.upgrade() {
                if Rc::ptr_eq(&to_add, &parent) {
                    return Err("something went wrong")
                }
            } else{
                break
            }
        }
        self.widget_data().children.borrow_mut().push(to_add.clone());
        *to_add.widget_data().parent.borrow_mut() = Some(Rc::downgrade(&as_widget));
        Ok(())
    }
}