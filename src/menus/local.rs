// <RALI - Rali, the Arch Linux Installer>
// Copyright (c) <2021>  <Jacob Stannix>
//
// this program is free software: you can redistribute it and/or modify
// it under the terms of the gnu general public license as published by
// the free software foundation, either version 3 of the license, or
// (at your option) any later version.
//
// this program is distributed in the hope that it will be useful,
// but without any warranty; without even the implied warranty of
// merchantability or fitness for a particular purpose.  see the
// gnu general public license for more details.
//
// you should have received a copy of the gnu general public license
// along with this program.  if not, see <https://www.gnu.org/licenses/>
use crate::menus::{num_list, every_nth};
use std::fs::File;
use std::io::Read;
use crate::ask_for_input;
use regex::Regex;
const MAIN_MENU: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]; 
pub(crate) fn print_menu_thirds() -> usize {
    let mut output = num_list(MAIN_MENU.to_vec().iter().map(|s| s.to_string()).collect());
    let mut second = output.clone();
    let mut thrid = output.clone();
    every_nth(&mut output, -1, 3);
    every_nth(&mut second, -2, 3);
    every_nth(&mut thrid, 0, 3);
    let mut second = second.iter();
    let mut third = thrid.iter();
    let mut menu = Vec::new();

    for i in output {
        let mut string = String::new();
        string.push_str(&i);
        string.push_str("  ");
        match second.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        string.push_str("  ");
        match third.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        menu.push(string);
    }

    println!("Please select a catagory");
    for i in menu {
        println!("{}", i);
    }
    let answer = ask_for_input("").parse::<usize>().unwrap();
    answer - 1
}

fn strip_comments(file: String) -> String {
    let rc = Regex::new(r"(?m)^#").unwrap();
    let stripted = rc.replace_all(&file, "");
    stripted.to_string()
}

fn get_locals() -> String  {
    let mut  locals = File::open("/etc/locale.gen").unwrap();
    let mut contents = String::new();
    locals.read_to_string(&mut contents).unwrap();
    contents
}

pub(crate) fn print_menu(letter: usize) -> Vec<(String, String)> {
    let choice = MAIN_MENU[letter];
    let mut locals = Locals::new(strip_comments(get_locals()));
    locals.activate(choice);
    vec![(locals.header, locals.contents), (locals.active, String::new())]
}

#[derive(Default)]
struct Locals {
    header: String,
    contents: String,
    active: String,
}
impl Locals {
    fn new(file: String) -> Locals {
	let rs = Regex::new(r"(?m)^aa_DJ.UTF-8 UTF-8").unwrap();
	let mut split = rs.split(&file);
	let header = split.next().unwrap().to_string();
	let mut contents = String::from("aa_DJ.UTF-8 UTF-8\n");
	contents.push_str(split.next().unwrap().trim());
	Locals {
	    header,
	    contents,
	    ..Locals::default()
	}
    }
    fn activate(&mut self, letter: &str) {
	let rs = regex_set(letter);
	let n = rs.captures(&self.contents).unwrap().name("letter");
	let mut output = String::new();
	match n {
	    Some(a) => output.push_str(a.as_str()),
	    None => panic!(""),
	}
	self.active = output;
    }
}

fn regex_set(answer: &str) -> Regex {
    match answer {
	"a" => Regex::new(r"(?m)^[a+?].+").unwrap(),
	"b" => Regex::new(r"(?m)b*$").unwrap(),
	"c" => Regex::new(r"(?m)c*$").unwrap(),
	"d" => Regex::new(r"(?m)d*$").unwrap(),
	"e" => Regex::new(r"(?m)e*$").unwrap(),
	"f" => Regex::new(r"(?m)f*$").unwrap(),
	"g" => Regex::new(r"(?m)g*$").unwrap(),
	"h" => Regex::new(r"(?m)h*$").unwrap(),
	"i" => Regex::new(r"(?m)i*$").unwrap(),
	"j" => Regex::new(r"(?m)j*$").unwrap(),
	"k" => Regex::new(r"(?m)k*$").unwrap(),
	"l" => Regex::new(r"(?m)l*$").unwrap(),
	"m" => Regex::new(r"(?m)m*$").unwrap(),
	"n" => Regex::new(r"(?m)n*$").unwrap(),
	"o" => Regex::new(r"(?m)o*$").unwrap(),
	"p" => Regex::new(r"(?m)p*$").unwrap(),
	"q" => Regex::new(r"(?m)q*$").unwrap(),
	"r" => Regex::new(r"(?m)r*$").unwrap(),
	"s" => Regex::new(r"(?m)s*$").unwrap(),
	"t" => Regex::new(r"(?m)t*$").unwrap(),
	"v" => Regex::new(r"(?m)v*$").unwrap(),
	"w" => Regex::new(r"(?m)w*$").unwrap(),
	"x" => Regex::new(r"(?m)x*$").unwrap(),
	"y" => Regex::new(r"(?m)y*$").unwrap(),
	"z" => Regex::new(r"(?m)z*$").unwrap(),
	_ => panic!("unexpected variable"),
    }
}
