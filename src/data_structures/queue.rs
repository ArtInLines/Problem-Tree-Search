use std::collections::VecDeque;

use crate::Queue;

pub struct VecDeQueue<T>(VecDeque<T>);

impl<T> Queue<T> for VecDeQueue<T> {
    fn new() -> Self {
        VecDeQueue(VecDeque::new())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.0.front()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.front_mut()
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    fn push(&mut self, item: T) {
        self.0.push_back(item)
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}
