use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use macroquad::color::RED;
use macroquad::prelude::clear_background;
use macroquad::prelude::scene::clear;
use crate::game;
use crate::game::Game;
use crate::widget::Widget;

pub trait Scene{
    fn get_game(&self) -> Rc<Game>{
        self.scene_data().game.borrow().clone().and_then(|x| x.upgrade()).expect("worked")
    }
    fn get_widgets(&self) -> Ref<Vec<Rc<dyn Widget>>>{
        self.scene_data().widgets. borrow()
    }
    fn background_color(&self) -> macroquad::color::Color{
        RED
    }
    fn init(&self){}
    fn scene_data(&self) -> &SceneData;
    fn update(&self){

    }
    fn render(&self);
}

pub fn add_widget<T: Widget + 'static>(scene: &dyn Scene, widget: T) -> Rc<T>{

    let created = Rc::new(widget);
    scene.scene_data().widgets.borrow_mut().push(created.clone());
    created
}


#[derive(Default)]
pub struct SceneData{
    widgets: RefCell<Vec<Rc<dyn Widget>>>,
    pub(crate) game: RefCell<Option<Weak<Game>>>
}

impl SceneData{

    pub fn new() -> Self{
        Self::default()
    }
}

fn recursive_render(gotten: &dyn Widget){
    gotten.render();
    let mut sorted = gotten.get_children().clone();
    sorted.sort_by_key(|x| x.get_priority());
    for i in sorted{
        recursive_render(i.deref());
    }
}
default impl<T> Scene for T{
    fn render(&self){
        clear_background(self.background_color());
        let mut sorted = self.scene_data().widgets
            .borrow()
            .clone();

        sorted.sort_by_key(|x| x.get_priority());
        for i in sorted
        {
            recursive_render(i.deref());
        }
    }
}