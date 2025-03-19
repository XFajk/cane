use std::{cell::RefCell, collections::VecDeque};
pub use uuid::Uuid;

pub type SceneQueue = VecDeque<(Uuid, Box<dyn Scene>)>;

pub trait Scene {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);

    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}

pub trait SceneQueueModifier {
    fn modify(&mut self, scene_queue: &mut SceneQueue);
}

pub struct PushSceneMod(Option<Box<dyn Scene>>);

impl SceneQueueModifier for PushSceneMod {
    fn modify(&mut self, scene_queue: &mut SceneQueue) {
        scene_queue.push_front((Uuid::new_v4(), self.0.take().unwrap()));
    }
}

pub struct PopSceneMod;

impl SceneQueueModifier for PopSceneMod {
    fn modify(&mut self, scene_queue: &mut SceneQueue) {
        scene_queue.pop_front();
    }
}

pub struct RemoveSceneMod(Uuid);

impl SceneQueueModifier for RemoveSceneMod {
    fn modify(&mut self, scene_queue: &mut SceneQueue) {
        scene_queue.retain(|scene| scene.0 != self.0);
    }
}

thread_local! {
    static SCENE_MODIFIER_QUEUE: RefCell<VecDeque<Box<dyn SceneQueueModifier>>> = RefCell::new(VecDeque::new());
}

pub fn schedule_scene(new_scene: Box<dyn Scene>) {
    SCENE_MODIFIER_QUEUE.with_borrow_mut(move |modifier_queue| {
        modifier_queue.push_front(Box::new(PushSceneMod(Some(new_scene))));
    });
}

pub fn apply_modifier_to_scenes(scene_queue: &mut SceneQueue) {
    SCENE_MODIFIER_QUEUE.with(|scene_modifier_queue_cell| {
        let scene_modifier_queue = std::mem::replace(
            &mut *scene_modifier_queue_cell.borrow_mut(),
            VecDeque::new(),
        );

        for mut modifier in scene_modifier_queue.into_iter() {
            modifier.modify(scene_queue);
        }
    });
}
