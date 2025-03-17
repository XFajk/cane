use std::{cell::RefCell, collections::VecDeque};

pub type SceneQueue = VecDeque<Box<dyn Scene>>;

thread_local! {
    static NEW_SCENE_QUEUE: RefCell<VecDeque<Box<dyn Scene>>> = RefCell::new(VecDeque::new());
}

pub fn schedule_scene(scheduled_scene: Box<dyn Scene>) {
    NEW_SCENE_QUEUE
        .with_borrow_mut(move |constructor_queue| constructor_queue.push_back(scheduled_scene));
}

pub fn transfer_scheduled_scenes(scenes: &mut SceneQueue) {
    NEW_SCENE_QUEUE.with(|constructor_queue_cell| {
        let constructor_queue = std::mem::replace(&mut *constructor_queue_cell.borrow_mut(), VecDeque::new());
        
        for scene in constructor_queue.into_iter() {
            scenes.push_back(scene);
        }
    });
}

pub trait Scene {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);

    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}
