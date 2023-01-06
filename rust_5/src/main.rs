#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::thread;


// result has two variants Ok and Err
// enum Result<T, E> {
// Ok(T)
// Err(E) }
// where T represents the data typeof the value returns and E the type of error

fn use_func<T>(a: i32, b: i32, func: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    func(a, b)
}

fn main() {
    // panic!("Unrecoverable error!");
    let lil_arr = [1, 2];
    // println!("{}", lil_arr[10]);
    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("problem creating file: {:?}", error);
        }
    };

    write!(output, "Just some\nrandom words").expect("failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(file) => file,
                Err(error) => {
                    panic!("problem creating file: {:?}", error);
                }
            },
            _other_error => {
                panic!("problem opening file: {:?}", error);
            }
        },
    };

    let mut arr_it = [1, 2, 3, 4];
    // an iterator goes trough the values by borrowing
    for val in arr_it {
        println!("{}", val);
    }

    let mut iter1 = arr_it.iter();
    println!("1st {:?}", iter1.next());

    // closures
    // let var_name = |parameters| -> return_type {BODY}
    // closures unlike functions can access variables outside of their body (borrowing)
    let can_vote = |age: i32| age >= 18;
    println!("Can vote: {}", can_vote(8));
    println!("Can vote: {}", can_vote(18));

    let mut samp1 = 5;
    let print_var = || println!("samp1 = {} ", samp1);
    print_var();
    samp1 = 10;

    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    let sum = |a, b| a + b;
    let multiply = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, multiply));

    // BOX - stores data on the heap - you would request a certain amount of space and the OS would find that space
    // let b_int1 = Box::new(10);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1)
    .left( TreeNode::new(2));
}
