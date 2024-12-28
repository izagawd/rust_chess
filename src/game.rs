use std::cell::RefCell;
use std::rc::Rc;
use crate::scene::Scene;


/// The main source of this game. It has a scene that can be changed anytime
pub struct Game{
    scene: RefCell<Rc<dyn Scene>>
}



impl Game{
    pub fn update(self: Rc<Self>){
        let cloned = self.scene.borrow().get_widgets().clone();
        for i in cloned{
            i.update();
        }
        let borrowed = self.scene.borrow().clone();
        borrowed.update();
    }
    pub fn render(&self){
        self.scene.borrow().render();
    }
    pub fn new(scene: Rc<dyn Scene>) -> Rc<Self>{
       let made =Rc::new(Self{scene:RefCell::new(scene.clone())});
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