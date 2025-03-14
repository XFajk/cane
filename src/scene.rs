use std::{cell::RefCell, collections::VecDeque};

pub type SceneQueue = VecDeque<Box<dyn Scene>>;

thread_local! {
    pub static CONSTRUCTOR_QUEUE: RefCell<VecDeque<Box<dyn FnMut() -> Box<dyn Scene>>>> = RefCell::new(VecDeque::new());
}

pub fn schedule_scene(scene_constructor: Box<dyn FnMut() -> Box<dyn Scene>>) {
    CONSTRUCTOR_QUEUE
        .with_borrow_mut(move |constructor_queue| constructor_queue.push_back(scene_constructor));
}

pub fn transfer_scheduled_scenes(scenes: &mut SceneQueue) {
    CONSTRUCTOR_QUEUE.with_borrow_mut(|constructor_queue| {
        for constructor in constructor_queue.iter_mut() {
            scenes.push_back(constructor());
        }

        constructor_queue.clear();
    });
}

pub trait Scene {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);

    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}
