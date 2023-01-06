#![allow(unused)]

use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // let thread1 = thread::spawn(|| {
    //     for i in 1..25 {
    //         println!("Spawned thread : {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..20 {
    //     println!("Main thread : {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // thread1.join().unwrap();

    pub struct Bank {
        balance: f32,
    }
    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //     the_bank.balance -= amt;
    // }
    // let mut bank = Bank { balance: 100.0 };
    // withdraw(&mut bank, 5.00);
    // println!("{}", bank.balance);

    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.00);
    // }

    // // this doesn't work
    // thread::spawn(|| {
    //     customer(&mut bank);
    // })
    // .join()
    // .unwrap();

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance is too low");
        } else {
            bank_ref.balance -= amt;
            println!("{} withdrawn. Balance: {}", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total: {}", bank.lock().unwrap().balance);
}

// Common problems with threads
//  1. threads are accessing data in the wrong order
//  2. threads are blocked from executing because of confusion
//  3. over requirements to proceed with execution
