use macroquad::color::RED;
use macroquad::prelude::clear_background;
use std::cell::{OnceCell, Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::game::Game;
use crate::widget::Widget;


/// A scene can be seen as the level the game is currently in
pub trait Scene{
    /// Gets the game the scene resides in
    fn get_game(&self) -> Rc<Game>;

    /// Gets the widgets contained in the scene
    fn get_widgets(&self) -> Ref<Vec<Rc<dyn Widget>>>;
    fn background_color(&self) -> macroquad::color::Color;
    fn init(self: Rc<Self>);


    /// The data all scenes must have
    fn scene_data(&self) -> &SceneData;


    /// Called just before render
    fn update(self: Rc<Self>);

    fn render(&self);
}

pub fn add_widget<T: Widget>(scene: Rc<dyn Scene>, widget: T) -> Rc<T>{

    let created = Rc::new(widget);
    scene.scene_data().widgets.borrow_mut().push(created.clone());
    created.widget_data().widget_data_inner.borrow_mut().scene = Some(Rc::downgrade(&scene));
    created.clone().init();
    created
}
pub fn remove_widget(scene:Rc<dyn Scene>, widget: Rc<dyn Widget>) {
    widget.clone().set_parent(None).unwrap();
    let pos = scene.scene_data().widgets.borrow().iter().position(|x| Rc::ptr_eq(x, &widget)).unwrap();
    scene.scene_data().widgets.borrow_mut().remove(pos);

}

#[derive(Default)]
pub struct SceneData{
    widgets: RefCell<Vec<Rc<dyn Widget>>>,
    pub(crate) game: OnceCell<Weak<Game>>
}

impl SceneData{

    pub fn new() -> Self{
        Self::default()
    }
}


default impl<T> Scene for T{
    fn get_game(&self) -> Rc<Game>{
        self.scene_data().game.get().as_ref().and_then(|x| x.upgrade())
            .unwrap()
    }
    fn background_color(&self) -> macroquad::color::Color{
        RED
    }
    fn get_widgets(&self) -> Ref<Vec<Rc<dyn Widget>>> {
        self.scene_data().widgets. borrow()
    }
    fn init(self: Rc<Self>){}
    fn update(self: Rc<Self>) {
        
    }
    fn render(&self){

    }
}