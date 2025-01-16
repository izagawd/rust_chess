use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use macroquad::prelude::clear_background;
use crate::level_utilities::scene::Scene;
use crate::widget::Widget;

/// The main source of this game. It has a scene that can be changed anytime
pub struct Game{
    scene: RefCell<Rc<dyn Scene>>
}


fn recursive_render(gotten: &dyn Widget){
    gotten.render();

    for i in gotten.get_children().iter() {
        recursive_render(i.deref());
    }
}
impl Game{
    pub fn update(self: Rc<Self>){
        let cloned = self.scene.borrow().get_widgets().clone();
        //updating every widget contained in the scene
        for i in cloned{
            i.update();
        }
        let borrowed = self.scene.borrow().clone();
        borrowed.update();
    }

    pub fn render(&self){
        // rendering every widget in the game
        // renders the parent, then it's children
        let scene_to_render = self.scene.borrow().clone();
        clear_background(scene_to_render.background_color());
        for i in scene_to_render.get_widgets().iter()
        {
            // ignoring widgets that have parent, since we are looking to
            // start at root widgets
            if i.get_parent().is_some(){
                continue;
            }
            recursive_render(i.deref());
        }
        self.scene.borrow().render();
    }
    pub fn new(scene: Rc<dyn Scene>) -> Rc<Self>{
       let made =Rc::new( Self { scene: RefCell::new(scene.clone())} );
        made.scene.borrow().scene_data().game.set(Rc::downgrade(&made)).unwrap();
        scene.init();
        made
    }
    /// Changes the scene
    pub fn change_scene(self: Rc<Self>, scene:Rc<dyn Scene>){
        *self.scene.borrow_mut() = scene.clone();
        scene.scene_data().game.set(Rc::downgrade(&self)).unwrap();
        scene.init();
    }
}