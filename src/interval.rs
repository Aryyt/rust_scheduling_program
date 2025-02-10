use std::i32;

pub struct Interval{
    start: i32,
    finish: i32,
    weight: i32,
}

impl Interval {
    pub fn new(start: i32, finish: i32, weight: i32) -> Self {
        Self { start, finish, weight }
    }

    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn finish(&self) -> i32 {
        self.finish
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }
}