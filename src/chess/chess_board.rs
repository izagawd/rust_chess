use std::rc::Rc;
use nalgebra::Vector2;
use crate::chess::chess_slot;
use crate::chess::chess_slot::ChessSlot;
use crate::scene::{add_widget, Scene};
use crate::widget::{Widget, WidgetData};

pub struct ChessBoard{
    chess_slots: Vec<Rc<ChessSlot>>,
    widget_data: WidgetData
}
impl Widget for ChessBoard {
    fn size(&self) -> Vector2<f32> {
        let size = self.chess_slots.len().isqrt() as f32 * chess_slot::SLOT_SIZE as f32;
        Vector2::new(size, size)
    }
    fn widget_data(&self) -> &WidgetData{
        &self.widget_data
    }
    fn render(&self){

    }
}

impl ChessBoard{

    pub fn new(current_scene: &dyn Scene) -> Rc<ChessBoard>{
       let mut created = ChessBoard{chess_slots: Vec::new(), widget_data: WidgetData::default()};

        let mut vecs_of_chess : Vec<Rc<ChessSlot>> =  Vec::new();
        for i in 0..8{
            for j in 0..8{
                vecs_of_chess.push(add_widget(current_scene,ChessSlot::new(Vector2::new(i,j))))
            }
        }
        let cloned_vecs_of_chess = vecs_of_chess.clone();
        created.chess_slots = vecs_of_chess;
        let created = add_widget(current_scene,created);
        for i in cloned_vecs_of_chess.iter(){
            created.clone().add_child(i.clone());
        }
        return created;
    }
}