use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Frame<T> {
    pub value: T,
    pub duration: f32,
}

#[derive(Debug, Clone)]
pub struct Animation<T> {
    frames: Vec<Frame<T>>,
    current_frame: usize,
    time_elapsed: f32,
    finished: bool,
    pub auto_reset: bool,
}

impl<T> Animation<T> {
    pub fn new_single_duration(frame_items: Vec<T>, duration: f32) -> Self {
        let frames: Vec<Frame<T>> = frame_items
            .into_iter()
            .map(|x| Frame {
                value: x,
                duration,
            })
            .collect();

        Self {
            frames,
            current_frame: 0,
            time_elapsed: 0.,
            finished: false,
            auto_reset: false
        }
    }

    pub fn new_multiple_durations(frame_items: Vec<T>, durations: Vec<f32>) -> Self {
        let frames: Vec<Frame<T>> = frame_items
            .into_iter()
            .zip(durations.into_iter())
            .map(|(x, duration)| Frame {
                value: x,
                duration,
            })
            .collect();
        
        Self {
            frames,
            current_frame: 0,
            time_elapsed: 0.,
            finished: false,
            auto_reset: false
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_elapsed += dt;
        self.finished = false;

        if self.time_elapsed >= self.frames[self.current_frame].duration {
            self.time_elapsed = 0.;
            
            if self.current_frame+1 >= self.frames.len() {
                self.finished = true;
                if self.auto_reset {
                    self.reset();
                }
            } else {
                self.current_frame += 1;
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
    }

    pub fn get_frame(&self) -> &T {
        &self.frames[self.current_frame].value
    }
}

#[derive(Debug, Clone)]
pub struct AnimationManager<T> {
    animations: HashMap<Box<str>, Animation<T>>,
    current_animation: Box<str>,
}

impl<T> AnimationManager<T> {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: "".into(),
        }
    }

    pub fn add_animation(&mut self, name: &str, animation: Animation<T>) {
        self.animations.insert(Box::from(name), animation);
    }

    pub fn play(&mut self, name: &str) {
        self.current_animation = Box::from(name);
    }

    pub fn update(&mut self, dt: f32) {
        self.animations
            .get_mut(&self.current_animation)
            .unwrap()
            .update(dt);
    }

    pub fn reset(&mut self) {
        self.animations
            .get_mut(&self.current_animation)
            .unwrap()
            .reset();
    }

    pub fn finished(&self) -> bool {
        self.animations
            .get(&self.current_animation)
            .unwrap()
            .finished
    }

    pub fn get_frame(&self) -> &T {
        self.animations
            .get(&self.current_animation)
            .unwrap()
            .get_frame()
    }
}
