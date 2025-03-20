use std::{ops::{Deref, DerefMut}, rc::Rc};
use uuid::Uuid;

pub struct GameObject<T> {
    pub id: Rc<Uuid>,
    pub name: Option<Rc<str>>,
    pub object: T
}

impl<T> Deref for GameObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T> DerefMut for GameObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

