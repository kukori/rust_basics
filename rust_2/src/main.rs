#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;

fn get_sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for x in list.iter() {
        sum += x;
    }
    return sum;
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    // println!("What is your name?");
    // let mut name: String = String::new();
    // let greeting: &str = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't receive input");
    // println!("Hello, {}! {}", name.trim_end(), greeting);

    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age: &str = "37";
    // let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} years old", age);

    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f64 : {}", f64::MAX);

    // let is_true: bool = true;
    // let my_grade: char = 'A';

    // let num_1: f32 = 1.111111111111111;
    // println!("f32: {}", num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f64: {}", num_2 + 0.111111111111111);
    // let num_3: u32 = 5;
    // let num_4: u32 = 4;
    // println!("5 + 4: {}", num_3 + num_4);
    // println!("5 - 4: {}", num_3 - num_4);
    // println!("5 * 4: {}", num_3 * num_4);
    // println!("5 / 4: {}", num_3 / num_4);
    // println!("5 % 4: {}", num_3 % num_4);
    // let random_num: i32 = rand::thread_rng().gen_range(1..101);
    // println!("random number: {}", random_num);

    // let age: i32 = 27;
    // if (age >= 1) && (age <= 18) {
    //     println!("important birthday");
    // } else if (age == 21) || (age == 50) {
    //     println!("important birthday");
    // } else if age >= 65 {
    //     println!("important birthday");
    // } else {
    //     println!("not important birthday")
    // }

    // let can_vote: bool = if age >= 18 { true } else { false };
    // println!("can vote: {}", can_vote);

    // let age2: i32 = 27;
    // match age2 {
    //     1..=18 => println!("important birthday"),
    //     21 | 50 => println!("important birthday"),
    //     65..=i32::MAX => println!("important birthday"),
    //     _ => println!("not important birthday"),
    // };

    // let my_age: i32 = 18;
    // let voting_age: i32 = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("can't vote"),
    //     Ordering::Greater => println!("can vote"),
    //     Ordering::Equal => println!("can vote"),
    // }

    let arr_1: [i32; 4] = [1, 2, 3, 4];
    // println!(
    //     "1st {} 2nd: {} 3rd: {} 4th: {}",
    //     arr_1[0], arr_1[1], arr_1[2], arr_1[3]
    // );
    // println!("length {}", arr_1.len());

    let mut index: usize = 0;
    // loop {
    //     if arr_1[index] % 2 == 0 {
    //         index += 1;
    //         continue;
    //     }
    //     println!("Val: {}", arr_1[index]);
    //     index += 1;
    //     if index == 3 {
    //         break;
    //     }
    // }

    // while index < arr_1.len() {
    //     println!("Array: {}", arr_1[index]);
    //     index += 1;
    // }

    // for val in arr_1.iter() {
    //     println!("Value: {}", val);
    // }

    // let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    // println!("Name: {}", my_tuple.1);
    // let (v1, v2, v3) = my_tuple;
    // println!("Name: {}", v1);

    // let mut st1 = String::new();

    // st1.push('A');
    // st1.push_str(" word");

    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }

    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    // let st3 = String::from("x s g b j j s m c");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }
    // let st4: &str = "Randon string";
    // let mut st5: String = st4.to_string();
    // println!("{}", st5);

    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..6];
    // println!("String length: {}", st6.len());
    // st5.clear();

    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7; //This destroys st6 but not 7
    // for char in st8.bytes() {
    //     println!("{}", char);
    // }

    // casting
    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // enums
    // enum Day {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday,
    // }

    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Day::Saturday | Day::Sunday => true,
    //             _ => false,
    //         }
    //     }
    // }

    // let today: Day = Day::Monday;

    // println!("Is today the weekend: {}", today.is_weekend());

    // vectors
    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i32> = vec![1, 2, 3, 4];
    // vec2.push(5);
    // println!("1st {}", vec2[0]);
    // // verify existence
    // let second: &i32 = &vec2[1];
    // match vec2.get(1) {
    //     Some(second) => println!("2nd {}", second),
    //     None => println!("No second value"),
    // }

    // for i in &mut vec2 {
    //     *i *= 2;
    // }
    // for i in &mut vec2 {
    //     println!("{}", i)
    // }
    // println!("Vector length: {}", vec2.len());
    // println!("Pop: {:?}", vec2.pop());

    // println!("{}", get_sum(1, 3));

    // let (val_1, val_2) = get_2(3);
    // println!("value 1: {}, value 2: {}", val_1, val_2);

    // let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // println!("Sum list: {}", sum_list(&numbers));
    println!("{}", get_sum_gen(1, 3));
    println!("{}", get_sum_gen(1.1, 2.2));
}
