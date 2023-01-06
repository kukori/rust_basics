#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;

mod restaurant;
use crate::restaurant::order_food;

// STACK: stores values in LIFO format, data on the stack must have a defined fixed size
// HEAP: when putting data on the heap u request a certain amount of space. The os finds
// space available and returns an address that space called a pointer.

// RULES:
// 1. Each value has a variable that's called its owner
// 2. There is only one owner at the time
// 3. When the owner goes out of scope the value disappears

// this doesn't apply to integers floats booleans chars tuples, it applies to strings,
// arrays, vectors etc.

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    return x;
}

fn change_str(name: &mut String) {
    name.push_str(" is happy.");
    println!("Message: {}", name);
}

fn main() {
    // ownership
    // let str1 = String::from("World");
    // // let str2 = str1;
    // // let str2 = str1.clone();
    // // println!("Hello {}", str1);
    // let str3 = print_return_str(str1);
    // println!("{}", str3);

    // let mut str4 = String::from("Attila");
    // change_str(&mut str4);

    // hashmaps
    // let mut heroes = HashMap::new();
    // heroes.insert("superman", "clark kent");
    // heroes.insert("batman", "bruce wayne");
    // heroes.insert("flash", "barry allen");

    // for (k, v) in heroes.iter() {
    //     println!("{} is {}", k, v);
    // }

    // if (heroes.contains_key(&"batman")) {
    //     let batman = heroes.get(&"batman");
    //     match batman {
    //         Some(x) => println!("batman is in the hashmap"),
    //         None => println!("batman is not in the hashmap"),
    //     }
    // }

    // structs & traits
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("333 Main St"),
        balance: 234.50,
    };

    bob.address = String::from("505 Main String");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    };

    let rectngle = Rectangle {
        length: 4,
        height: 10.5,
    };

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    };

    struct Rectangle2 {
        length: f32,
        width: f32,
    };

    struct Circle {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle2 {
        fn new(length: f32, width: f32) -> Rectangle2 {
            return Rectangle2 { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    };

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    };

    let rec: Rectangle2 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec area: {}", rec.area());
    println!("Circle area: {}", circ.area());

    // Crates: modules that produce a library or executable
    // Modules: organize and handle privacy
    // Packages: build test and share crates
    // Paths: A way of naming an item such as struct, function
    order_food();
}
