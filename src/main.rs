#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    timestamp: usize,
    event_type: String,
    user_id: String,
    content_id: String
}

fn main() {
    let file = File::open("./events.json").unwrap();
    let reader = BufReader::new(file);
    let mut users = HashSet::new();

    for line in reader.lines() {
        let de: Event = serde_json::from_str(&line.unwrap()).unwrap();
        &users.insert(de.user_id);
    }
    println!("{}", users.len());

    /*
    let mut numbers = Vec::new();
    let mut children = Vec::new();
    for i in 1..101 {
        numbers.push(i);
    }

    for i in 0..4 {
        let slice = &numbers[i*25..(i+1)*25];

        let owned = slice.to_vec();

        println!("thread spawned");
        children.push(thread::spawn(move || {
            let mut sum = 0;
            for j in owned {
                sum += j;
            }
            sum
        }));
    }
    let mut full_sum = 0;
    for c in children {
        full_sum += c.join().unwrap();
    }
    println!("{}", full_sum);
    */
}
