use std::fmt::{Debug, Display};

pub fn display_and<T: Display>(v: T) -> T {
    println!("{}", v);
    v
}

pub fn debug_and<T: Debug>(v: T) -> T {
    println!("{:?}", v);
    v
}
