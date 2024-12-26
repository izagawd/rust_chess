use std::cell::RefCell;
use std::rc::Rc;
use crate::scene::Scene;
use crate::widget::Widget;

pub struct Game{
    scene: RefCell<Rc<dyn Scene>>
}



impl Game{
    pub fn update(self: Rc<Self>){
        for i in self.scene.borrow().get_widgets().clone(){
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
        *made.scene.borrow().scene_data().game.borrow_mut() = Some(Rc::downgrade(&made));
        scene.init();
        made
    }
    pub fn change_scene(& self, scene:Rc<dyn Scene>){
        *self.scene.borrow_mut() = scene.clone();
        scene.init();
    }
}