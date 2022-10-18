use std::collections::VecDeque;

#[derive(Default)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push_back(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn pop_all<F>(&mut self, mut func: F)
    where
        F: FnMut(T),
    {
        while let Some(item) = self.pop() {
            func(item)
        }
    }
}
