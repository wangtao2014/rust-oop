use crate::List::{Cons, Nil};

pub struct AdverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AdverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub comments: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comment in self.comments.iter() {
            comment.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Self {
        Self {
            width,
            height,
            label,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        );
    }
}

pub struct SeletcBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl SeletcBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        Self {
            width,
            height,
            options,
        }
    }
}

impl Draw for SeletcBox {
    fn draw(&self) {
        println!(
            "SeletcBox width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        );
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // let screen = Screen {
    //     comments: vec![
    //         Box::new(Button::new(100, 100, String::from("Confirm!!!"))),
    //         Box::new(SeletcBox::new(
    //             200,
    //             200,
    //             vec![String::from("hello"), String::from("world")],
    //         )),
    //     ],
    // };

    // screen.run();

    let s1 = gives_ownership();
    let s2 = String::from("nihao");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {:?}", s1);
    println!("s3: {:?}", s3);

    let (length, s4) = calculate_length(s1);
    println!("s4: {:?}, len: {}", s4, length);

    let size = calculate_length1(&s3);
    println!("size: {:?}, s3: {:?}", size, s3);

    let list = Cons(12, Box::new(Cons(13, Box::new(Cons(14, Box::new(Nil))))));
    println!("list: {:?}", list);
}

/// borrowed test

fn gives_ownership() -> String {
    let something = String::from("hello world");
    println!("something: {:?}", something);
    something
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(a_string: String) -> (usize, String) {
    let len = a_string.len();
    (len, a_string)
}

fn calculate_length1(s: &String) -> usize {
    println!("s: {:?}", s.as_ptr());
    s.len()
}
