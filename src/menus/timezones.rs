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
use std::fs;
fn num_list(items: Vec<String>) -> Vec<String> {
    let mut num = 0;
    let items = items
        .iter()
        .map(|s| {
            num += 1;
            format!("{}) {}", num, s)
        })
        .collect();
    items
}
fn list_fils_as_vec(path: &str) -> Vec<String> {
    
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                files.push(entry.file_name().into_string().unwrap());
            }
        }
    };
    files
}
pub(crate) fn print_menu(items: &str) -> Vec<String> {
    let mut files = list_fils_as_vec(items);
    files.sort();
    let file_vec = files.clone();

    let output = num_list(files);
    for i in output {
	println!("{}", i);
    };
    file_vec
}
pub(crate) fn print_menu_thirds(items: &str) -> std::io::Result<Vec<String>> {
    let mut files = list_fils_as_vec(items);
    files.sort();

    let file_vec = files.clone();

    let mut output = num_list(files);

    let mut second = output.clone();
    let mut third = output.clone();

    every_nth(&mut output, -1, 3);
    every_nth(&mut second, -2, 3);
    every_nth(&mut third, 0, 3);

    let mut n2 = second.iter();
    let mut n3 = third.iter();
    let mut menu = Vec::new();

    for i in output {
        let mut string = String::new();
        string.push_str(&i);
        string.push_str("  ");
        match n2.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        string.push_str("  ");
        match n3.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        menu.push(string);
    }

    for i in menu {
        println!("{}", i);
    }
    Ok(file_vec)
}
fn every_nth(values: &mut Vec<String>, start_value: i32, incriment: i32) {
    let mut c = start_value;
    values.retain(|_| {
        c += 1;
        return c % incriment == 0;
    });
}
