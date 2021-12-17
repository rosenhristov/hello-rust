extern crate rand;
extern crate regex;
extern crate reqwest;

use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::io::prelude;
use rand::Rng;
use regex::Regex;
use reqwest::StatusCode;

fn main() {
    println!("Hello, Rust!");

    println!(" ::: Working with suffix annotations :::");
    let x = 5f32 / 0f32; // infinity
    println!("Infinity: {}", x);

    // let y = 5i32 / 0i32; // division by zero not allowed
    // println!("{}", y);

    println!("\n::: Working with vectors :::");
    let mut vec: Vec<i32> = Vec::new();
    vec.push(25);
    vec.push(35);
    vec.push(1);
    vec.remove(1);
    let vec2 = vec![1, 2, 3, 4];
    println!("vec No: {}", vec2[2]);
    for num in vec2.iter() {
        println!("num: {}", num);
    }


    println!("\n::: Working with files :::");
    let mut file: File = File::open("C:/Users/rhristov/Documents/personalProjects/hello-rust/Cargo.toml")
        .expect("Can't open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Couldn't read the file");
    println!("content: \n\n {}", content);


    println!("\n::: Working with command line input :::");
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        println!("{}", arg);
    }

    let mut file_name = String::from("output.txt");
    let mut dir = String::from("src/main/resources/");
    let mut filepath = dir.push_str(&*file_name);
    let mut file_to_create = File::create("src/main/resources/output.txt")
        .expect("Could not create file");
    file_to_create.write_all(b"Welcome to Rust").expect("Cannot write to the file");

    println!("\n ::: Working with structs and traits :::");
    let rosen: Person = Person {
        name: String::from("Rosen"),
        age: 39
    };
    println!("Can {} speak? {}", rosen.name, rosen.can_speak());

    println!("\n::: Working with HashMaps :::");
    let mut marks = HashMap::new();
    marks.insert("Java programming language", 6);
    marks.insert("Rust programming language", 5);
    marks.insert("HTML && CSS", 6);
    marks.insert("SQL", 6);
    println!("How many subjects did you study? {}", marks.len());
    match marks.get("Java programming language") {
        Some(mark) => println!("You have got {} for Java programming", mark),
        None => println!("You did not study Java Programming")
    }
    println!("Did you study Ruby?: {}", marks.contains_key("Ruby Programming"));
    println!("Here is what I have studied:");
    for (subject, mark) in marks {
        println!("Subject: '{}', mark: '{}'", subject, mark);
    }

    println!("\n :::: Working with random numbers :::");
    let mut toto:Vec<i32> = Vec::new();
    while toto.len() <= 6 {
        let rand_num = rand::thread_rng().gen_range(1, 50);
        if !toto.contains(&rand_num) {
            toto.push(rand_num);
        }
    }
    print!("Toto: ");
    for i in toto.iter() {
        if i == toto.get(&toto.len() - 1).unwrap() {
            print!("{} ", i);
        } else {
            print!("{}, ", i);
        }
    }


    let mut bools:Vec<bool> = Vec::new();
    while bools.len() <= 6 {
        let rand_bool:bool = rand::thread_rng().gen_weighted_bool(3);
        bools.push(rand_bool);
    }
    print!("\nBools: ");
    for i in 1..6 {
        if i == bools.len() - 1 {
            print!("{} ", bools.get(i).unwrap());
        } else {
            print!("{}, ", bools.get(i).unwrap());
        }
    }

    println!("\n\n ::: Working with strings :::");
    let str: String = String::from("Rust is fantastic!");
    println!("before replace: {}", &str);
    println!("after replace: {}", str.replace("fantastic", "great"));

    let my_string: String = String::from("The weather\nis nice\noutside, mate!");
    println!("Before lining: {}", &my_string);
    println!("After lining: ");
    for line in my_string.lines() {
        println!("[ {} ]", line);
    }
    let my_str: String = String::from("The weather is nice outside, mate!");
    println!("Before splitting: {}", &my_str);
    let tokens:Vec<&str> = my_str.split(" ").collect();
    println!("After splitting: ");
    for token in tokens {
        println!("[ {} ]", token);
    }

    let index = 4;
    match my_str.chars().nth(4) {
        Some(c) => println!("Charcter: {}", c),
        None => println!("No character at index {}", index)
    }

    println!("\n :::Working with chars :::");
    for c in my_str.chars() {
        print!("{} ", c);
    }

    let c : char = 'R';

    println!("\n\n ::: Working with regex expressions :::");
    let reg = Regex::new(r"([\w]{8})").unwrap();
    let text = String::from("Obektron");
    println!("Is '{}' a match?: {}", text, reg.is_match(&text.as_str()));
    match reg.captures(&text) {
        Some(caps) => println!("Found capture: {}", &caps[0]),
        None => println!("Could not find capture")
    }

    println!("\n\n ::: Working with HTTP requests :::");
    //the short way:
    let response_text =
        reqwest::get("https://www.youtube.com/watch?v=xYoESR1aEQk&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=38")
            .expect("Could not make request")
            .text()
             .expect("Could not read the response text");
    println!("Response ext: {}", response_text);

    //the long way:
    // match reqwest::get("https://www.youtube.com/watch?v=xYoESR1aEQk&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=38") {
    //     Ok(mut response) => {
    //         if response.status() == reqwest::StatusCode::Ok {
    //             match response.text() {
    //                 Ok(text) => println!("Response text: {}", text),
    //                 Err(_) => println!("Could not read the response text")
    //             }
    //         } else {
    //             println!("The response was not 200. It was {}", &response.status());
    //         }
    //     },
    //     Err(_) => println!("Could not make the request")
    // }

    println!("\n::: Working with command line input :::");
    let mut user_input:String = String::new();
    println!("Hey, mate, please say something!:");
    let mut input = io::stdin().read_line(&mut user_input);
    match input {
        Ok(_) => println!("Success! You said: {}", user_input.to_uppercase()),
        Err(e) => println!("Oops, something went wrong {}", e)
    }

}


struct Person {
    name: String,
    age: i8
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        self.age > 1
    }
}