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
use crate::ask_for_input;
use crate::menus::lang_vars as lav;
use crate::menus::{every_nth, num_list};
use regex::Regex;
use std::fs::File;
use std::io::Read;
const MAIN_MENU: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
pub(crate) fn print_main_menu_thirds() -> usize {
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

pub(crate) fn print_menu(letter: usize) -> Vec<String> {
    let menu_match = which_menu(letter.clone());
    let answer = sort_menu(menu_match);
    for i in answer {
        println!("{}", i);
    }
    let answer = parse_answer(ask_for_input(
        "please select you lang variables\ne.g. 1 2 3 ect",
    ));
    let last_one = fetch_lang(answer, letter);

    last_one
}

fn format_vec_l20(menu: Vec<&str>) -> Vec<String> {
    let mut first = num_list(menu.iter().map(|x| x.to_string()).collect());
    let mut second = first.clone();

    every_nth(&mut first, -1, 2);
    every_nth(&mut second, 0, 2);

    let mut second = second.iter();

    let mut menu = Vec::new();
    for i in first {
        let mut string = String::from(&i);
        string.push_str("  ");

        match second.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        menu.push(string)
    }
    menu
}
fn format_vec_l30(menu: Vec<&str>) -> Vec<String> {
    let mut first = num_list(menu.iter().map(|s| s.to_string()).collect());
    let mut second = first.clone();
    let mut third = first.clone();

    every_nth(&mut first, -1, 3);
    every_nth(&mut second, -2, 3);
    every_nth(&mut third, 0, 3);

    let mut second = second.iter();
    let mut third = third.iter();

    let mut menu = Vec::new();
    for i in first {
	let mut string = String::from(&i);
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
    menu
}
fn format_vec_l40(menu: Vec<&str>) -> Vec<String> {
    let mut first = num_list(menu.iter().map(|s| s.to_string()).collect());
    let mut second = first.clone();
    let mut third = first.clone();
    let mut fourth = first.clone();

    every_nth(&mut first, -1, 4);
    every_nth(&mut second, -2, 4);
    every_nth(&mut third, -3, 4);
    every_nth(&mut fourth, 0, 4);

    let mut second = second.iter();
    let mut third = third.iter();
    let mut fourth = fourth.iter();

    let mut menu = Vec::new();
    for i in first {
	let mut string = String::from(&i);
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

	string.push_str("  ");
	match fourth.next() {
	    Some(l) => string.push_str(l),
	    None => string.push(' '),
	}
	menu.push(string);
    }
    menu
}
fn format_vec_o50(menu: Vec<&str>) -> Vec<String> {
    let mut first = num_list(menu.iter().map(|s| s.to_string()).collect());
    let mut second = first.clone();
    let mut third = first.clone();
    let mut fourth = first.clone();
    let mut fifth = first.clone();

    every_nth(&mut first, -1, 5);
    every_nth(&mut second, -2, 5);
    every_nth(&mut third, -3, 5);
    every_nth(&mut fourth, -4, 5);
    every_nth(&mut fifth, 0, 5);

    let mut second = second.iter();
    let mut third = third.iter();
    let mut fourth = fourth.iter();
    let mut fifth = fifth.iter();

    let mut menu = Vec::new();
    for i in first {
	let mut string = String::from(&i);
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

	string.push_str("  ");
	match fourth.next() {
	    Some(l) => string.push_str(l),
	    None => string.push(' '),
	}

	string.push_str("  ");
	match fifth.next() {
	    Some(l) => string.push_str(l),
	    None => string.push(' '),
	}
	menu.push(string);
    }
    menu
}
fn fetch_lang(_input: Vec<usize>, _catagory: usize) -> Vec<String> {
    let mut last_one = Vec::new();
    last_one.push(String::new());
    last_one
}

fn parse_answer(answer: String) -> Vec<usize> {
    let mut vec_string = Vec::new();
    for i in answer.split_whitespace() {
        vec_string.push(i.parse::<usize>().unwrap() - 1);
    }
    vec_string
}

fn which_menu<'a>(index: usize) -> Vec<&'a str> {
    match index {
        0 => lav::LANG_A.to_vec(),
        1 => lav::LANG_B.to_vec(),
        2 => lav::LANG_C.to_vec(),
        3 => lav::LANG_D.to_vec(),
        4 => lav::LANG_E.to_vec(),
        5 => lav::LANG_F.to_vec(),
        6 => lav::LANG_G.to_vec(),
        7 => lav::LANG_H.to_vec(),
        8 => lav::LANG_I.to_vec(),
        9 => lav::LANG_J.to_vec(),
        10 => lav::LANG_K.to_vec(),
        11 => lav::LANG_L.to_vec(),
        12 => lav::LANG_M.to_vec(),
        13 => lav::LANG_N.to_vec(),
        14 => lav::LANG_O.to_vec(),
        15 => lav::LANG_P.to_vec(),
        16 => lav::LANG_Q.to_vec(),
        17 => lav::LANG_R.to_vec(),
        18 => lav::LANG_S.to_vec(),
        19 => lav::LANG_T.to_vec(),
        20 => lav::LANG_U.to_vec(),
        21 => lav::LANG_V.to_vec(),
        22 => lav::LANG_W.to_vec(),
        23 => lav::LANG_X.to_vec(),
        24 => lav::LANG_Y.to_vec(),
        25 => lav::LANG_Z.to_vec(),
        _ => panic!("invalid option"),
    }
}
fn sort_menu(menu: Vec<&str>) -> Vec<String> {
    let num = menu.iter().count();
    let colums = if num <= 20 {
        num_list(menu.iter().map(|s| s.to_string()).collect())
    } else if num >= 21 && num <= 30 {
        format_vec_l20(menu)
    } else if num >= 31 && num <= 40 {
        format_vec_l30(menu)
    } else if num >= 41 && num <= 50 {
        format_vec_l40(menu)
    } else {
        format_vec_o50(menu)
    };
    colums
}
