use macroquad::color::RED;
use macroquad::prelude::clear_background;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::game::Game;
use crate::widget::Widget;

pub trait Scene{
    /// Gets the game the scene resides in
    fn get_game(&self) -> Rc<Game>{
        self.scene_data().borrow().game.as_ref().and_then(|x| x.upgrade())
            .unwrap()
    }

    /// Gets the widgets contained in the scene
    fn get_widgets(&self) -> Ref<Vec<Rc<dyn Widget>>>{
        Ref::map(self.scene_data().borrow(),|x| &x.widgets)
    }
    fn background_color(&self) -> macroquad::color::Color{
        RED
    }
    fn init(self: Rc<Self>){}


    /// The data all scenes must have
    fn scene_data(&self) -> &RefCell<SceneData>;


    /// Called just before render
    fn update(self: Rc<Self>){

    }



    fn render(&self);
}

pub fn add_widget<T: Widget + 'static>(scene: Rc<dyn Scene>, widget: T) -> Rc<T>{

    let created = Rc::new(widget);
    scene.scene_data().borrow_mut().widgets.push(created.clone());
    created.widget_data().borrow_mut().scene = Some(Rc::downgrade(&scene));
    created
}
pub fn remove_widget(scene:Rc<dyn Scene>, widget: Rc<dyn Widget>) {
    widget.clone().set_parent(None).unwrap();
    let pos = scene.scene_data().borrow().widgets.iter().position(|x| Rc::ptr_eq(x, &widget)).unwrap();
    scene.scene_data().borrow_mut().widgets.remove(pos);

}

#[derive(Default)]
pub struct SceneData{
    widgets: Vec<Rc<dyn Widget>>,
    pub(crate) game: Option<Weak<Game>>
}

impl SceneData{

    pub fn new() -> Self{
        Self::default()
    }
}

fn recursive_render(gotten: &dyn Widget){
    gotten.render();

    for i in gotten.get_children().iter() {
        recursive_render(i.deref());
    }
}
default impl<T> Scene for T{
    fn render(&self){
        clear_background(self.background_color());
        for i in self.scene_data().borrow().widgets.iter()
        {
            recursive_render(i.deref());
        }
    }
}