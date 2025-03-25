use std::{cell::RefCell, collections::VecDeque};

pub trait Scene {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);

    fn queue_free(&mut self);
    fn is_queue_free(&self) -> bool;
}

struct SceneHandler {
    pub scenes: RefCell<VecDeque<Box<dyn Scene>>>,
    new_scenes: RefCell<VecDeque<Box<dyn Scene>>>,
}

impl SceneHandler {
    fn new() -> Self {
        Self {
            scenes: RefCell::new(VecDeque::new()),
            new_scenes: RefCell::new(VecDeque::new())
        }
    }

    pub fn queue_scene(&mut self, scene: Box<dyn Scene>) {
        self.new_scenes.borrow_mut().push_front(scene);
    }

    pub fn run(&mut self, dt: f32) {
        self.move_new_scenes(); 

        self.scenes.borrow_mut().retain_mut(|scene| {
            scene.update(dt);
            scene.render(dt);

            scene.is_queue_free()
        });
    }

    fn move_new_scenes(&mut self) {
        let new_scens = std::mem::take(self.new_scenes.get_mut()).into_iter();
        self.scenes.borrow_mut().extend(new_scens);
    }
}

thread_local! {
    static SCENE_HANDLER: RefCell<SceneHandler> = RefCell::new(SceneHandler::new());
}

